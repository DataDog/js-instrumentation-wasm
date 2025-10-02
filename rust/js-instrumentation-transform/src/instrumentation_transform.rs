use anyhow::Result;
use js_instrumentation_rewrite::rewrite::Rewrite;
use js_instrumentation_rewrite::rewrite_plan::build_rewrite_plan;
use js_instrumentation_shared::instrumentation_options::HelperFunctionSource;
use js_instrumentation_shared::{
    build_parser, debug_log, module_kind_for, InputFile, InstrumentationInput,
    InstrumentationOptions, InstrumentationOutput,
};
use swc_common::comments::SingleThreadedComments;
use swc_common::source_map::SmallPos;
use swc_common::BytePos;
use swc_ecma_ast::Program;

use crate::comments::process_comments;
use crate::dictionary::{
    DictionaryTracker, OptimizedDictionary, DEFAULT_ADD_TO_DICTIONARY_FUNCTION,
    DEFAULT_DICTIONARY_IDENTIFIER,
};
use crate::features::FeatureTracker;
use crate::identifiers::IdentifierTracker;
use crate::rewrite::{
    build_dictionary_declaration, build_helper_declaration, delete_source_map_comment,
    RewriteTracker, TemplateParameters,
};
use crate::source_maps::{
    chain_source_map_if_needed, serialize_source_map, source_map_comment_span_to_delete,
};
use crate::visitor::visit;

pub fn apply_transform(
    input: &InstrumentationInput,
    options: &InstrumentationOptions,
) -> Result<InstrumentationOutput> {
    let mut input_file = InputFile::new(&input.id, &input.code);
    let comments: SingleThreadedComments = Default::default();
    let mut parser = build_parser(&input_file, &comments, options);
    let program: Program = match parser.parse_program() {
        Ok(program) => program,
        Err(err) => {
            return Err(anyhow::anyhow!("Parsing failed: {:?}", err));
        }
    };

    let (directive_set, source_map_comment) = process_comments(&input_file, &comments);

    let default_add_to_dictionary_helper = get_default_add_to_dictionary_helper(&options);

    let mut dictionary_tracker = DictionaryTracker::new(directive_set);
    let mut feature_tracker = FeatureTracker::new();
    let mut identifier_tracker = IdentifierTracker::new(vec![
        default_add_to_dictionary_helper,
        DEFAULT_DICTIONARY_IDENTIFIER,
    ]);
    let mut rewrite_tracker = RewriteTracker::new();

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
    let privacy_dictionary_size = dictionary.strings.len();

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

    let (mut rewrites, token_positions) = rewrite_tracker.take();
    if let Some(span_to_delete) = source_map_comment_span_to_delete(&source_map_comment) {
        rewrites.push(delete_source_map_comment(span_to_delete));
    }

    let start_of_first_line = compute_start_of_first_line(&input_file, &program);
    let header_rewrites = build_helper_declaration(start_of_first_line, &template_parameters)
        .into_iter()
        .chain(build_dictionary_declaration(start_of_first_line, &template_parameters).into_iter());

    let body_rewrites = rewrites
        .into_iter()
        .flat_map(|rewrite| {
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
        });

    let rewrite_plan = build_rewrite_plan(header_rewrites, body_rewrites);

    let (mut instrumented_code, transform_map) = rewrite_plan.apply(
        &mut input_file,
        token_positions,
        options.output.embed_code_in_source_map,
    );

    let source_map = chain_source_map_if_needed(&source_map_comment, &input.map, transform_map)?;

    if options.output.inline_source_map {
        if let Some(ref source_map) = source_map {
            instrumented_code += "//# sourceMappingURL=";
            instrumented_code += &source_map.to_data_url()?;
        }
    }

    let serialized_source_map = match source_map {
        Some(source_map) => Some(serialize_source_map(source_map)?),
        None => None,
    };

    Ok(InstrumentationOutput {
        id: input.id.clone(),
        code: instrumented_code,
        map: serialized_source_map,
        privacy_dictionary_size,
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

fn compute_start_of_first_line(input_file: &InputFile, program: &Program) -> BytePos {
    let has_shebang = match program {
        Program::Module(ref module) => module.shebang.is_some(),
        Program::Script(ref script) => script.shebang.is_some(),
    };
    if has_shebang {
        input_file.next_line_start(input_file.start_pos)
    } else {
        input_file.start_pos
    }
}
