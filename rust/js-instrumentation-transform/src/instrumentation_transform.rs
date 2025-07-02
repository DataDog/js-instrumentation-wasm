use std::io::BufWriter;

use anyhow::Result;
use js_instrumentation_rewrite::rewrite::Rewrite;
use js_instrumentation_rewrite::rewrite_plan::build_rewrite_plan;
use js_instrumentation_shared::instrumentation_options::HelperFunctionSource;
use js_instrumentation_shared::{
    build_parser, debug_log, module_kind_for, InputFile, InstrumentationInput,
    InstrumentationOptions, InstrumentationOutput,
};
use swc_common::source_map::SmallPos;
use swc_core::base::sourcemap::SourceMap;

use crate::dictionary::{
    DictionaryTracker, OptimizedDictionary, DEFAULT_ADD_TO_DICTIONARY_FUNCTION,
    DEFAULT_DICTIONARY_IDENTIFIER,
};
use crate::features::FeatureTracker;
use crate::identifiers::IdentifierTracker;
use crate::rewrite::{
    insert_dictionary_declaration, insert_helper_declaration, RewriteTracker, TemplateParameters,
};
use crate::visitor::visit;

pub fn apply_transform(
    input: &InstrumentationInput,
    options: &InstrumentationOptions,
) -> Result<InstrumentationOutput> {
    let mut input_file = InputFile::new(&input.id, &input.code);
    let mut parser = build_parser(&input_file, options);
    let program = match parser.parse_program() {
        Ok(program) => program,
        Err(err) => {
            return Err(anyhow::anyhow!("Parsing failed: {:?}", err));
        }
    };

    let default_add_to_dictionary_helper = get_default_add_to_dictionary_helper(&options);

    let mut dictionary_tracker = DictionaryTracker::new();
    let mut feature_tracker = FeatureTracker::new();
    let mut identifier_tracker = IdentifierTracker::new(vec![
        default_add_to_dictionary_helper,
        DEFAULT_DICTIONARY_IDENTIFIER,
    ]);
    let mut rewrite_tracker = RewriteTracker::new(vec![
        insert_helper_declaration(input_file.start_pos),
        insert_dictionary_declaration(input_file.start_pos),
    ]);

    visit(
        &program,
        &mut input_file,
        &mut dictionary_tracker,
        &mut feature_tracker,
        &mut identifier_tracker,
        &mut rewrite_tracker,
    );

    let dictionary_identifier =
        identifier_tracker.new_unused_identifier(DEFAULT_DICTIONARY_IDENTIFIER);
    let dictionary = OptimizedDictionary::build(&dictionary_identifier, dictionary_tracker.strings);

    let helper_identifier =
        identifier_tracker.new_unused_identifier(default_add_to_dictionary_helper);

    let module_kind = module_kind_for(
        &input.id,
        options,
        Some(feature_tracker.module_keyword_usage),
    );

    let template_parameters = TemplateParameters::new(
        dictionary,
        dictionary_identifier,
        &options.privacy.add_to_dictionary_helper,
        helper_identifier,
        module_kind,
    );

    let (rewrites, token_positions) = rewrite_tracker.take();
    let rewrite_plan = build_rewrite_plan(
        rewrites
            .into_iter()
            .filter_map(|rewrite| {
                rewrite.filter_map_content(|template| {
                    // Evaluate the rewrite templates by substituting in template parameters (e.g.
                    // the dictionary identifier).
                    match template.evaluate(&template_parameters) {
                        Ok(content) => Some(content),
                        Err(err) => {
                            debug_log(&format!("Error evaluating rewrite templates: {}", err));
                            None
                        }
                    }
                })
            })
            .filter(|rewrite| match rewrite {
                // Some optional rewrites are only beneficial if they produce smaller output than
                // the original source code. Filter out these rewrites when they'll provide no
                // benefit.
                Rewrite::Replace { content, span } if content.should_only_replace_if_smaller() => {
                    content.len() < (span.hi.to_usize() - span.lo.to_usize())
                }
                _ => true,
            }),
    );

    let (mut instrumented_code, transform_map) = rewrite_plan.apply(
        &mut input_file,
        token_positions,
        options.output.embed_code_in_source_map,
    );

    let source_map = match &input.map {
        // An input source map was specified; chain it with the source map we generated to produce
        // the output source map.
        Some(unparsed_map) => {
            let mut map = parse_source_map(&unparsed_map)?;
            map.adjust_mappings(&transform_map);
            map
        }
        // No input source map was specified, so the source map we generated can be used as the
        // output source map directly.
        None => transform_map,
    };

    if options.output.inline_source_map {
        instrumented_code += "//# sourceMappingURL=";
        instrumented_code += &source_map.to_data_url()?;
    }

    let serialized_source_map = serialize_source_map(source_map)?;

    Ok(InstrumentationOutput {
        id: input.id.clone(),
        code: instrumented_code,
        map: serialized_source_map,
    })
}

fn get_default_add_to_dictionary_helper<'a>(options: &'a InstrumentationOptions) -> &'a str {
    match options.privacy.add_to_dictionary_helper {
        HelperFunctionSource::Expression { .. } => DEFAULT_ADD_TO_DICTIONARY_FUNCTION,
        HelperFunctionSource::Import { ref func, .. } => {
            // Default to the imported function name if it's one character long. If it's longer,
            // renaming it on import will be more space-efficient, so use the standard default
            // name, which is only one character long.
            if func.len() == 1 {
                &func
            } else {
                DEFAULT_ADD_TO_DICTIONARY_FUNCTION
            }
        }
    }
}

fn parse_source_map(unparsed_map: &str) -> Result<SourceMap> {
    SourceMap::from_reader(unparsed_map.as_bytes())
        .map_err(|err| anyhow::anyhow!("Parsing input source map failed: {}", err))
}

fn serialize_source_map(map: SourceMap) -> Result<String> {
    let mut source_map_buffer = BufWriter::new(Vec::new());
    map.to_writer(&mut source_map_buffer)
        .map_err(|err| anyhow::anyhow!("Serializing output source map failed: {}", err))?;
    let source_map_writer = source_map_buffer
        .into_inner()
        .map_err(|err| anyhow::anyhow!("Unwrapping output source map failed: {}", err))?;
    String::from_utf8(source_map_writer)
        .map_err(|err| anyhow::anyhow!("Converting output source map to string failed: {}", err))
}
