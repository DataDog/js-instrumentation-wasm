use anyhow::Result;
use js_instrumentation_rewrite::rewrite::Rewrite;
use js_instrumentation_rewrite::rewrite_plan::build_rewrite_plan;
use js_instrumentation_shared::{
    build_parser, debug_log, module_kind_for, InputFile, TransformOptions, TransformOutput,
};
use swc_common::source_map::SmallPos;

use crate::dictionary::{DictionaryTracker, OptimizedDictionary, DEFAULT_DICTIONARY_IDENTIFIER};
use crate::features::FeatureTracker;
use crate::identifiers::IdentifierTracker;
use crate::rewrite::{
    insert_dictionary_declaration, insert_helper_import, RewriteTracker, TemplateParameters,
};
use crate::visitor::visit;

pub fn apply_transform(
    filename: &str,
    code: &str,
    options: &TransformOptions,
) -> Result<TransformOutput> {
    let mut input_file = InputFile::new(filename, code);
    let mut parser = build_parser(&input_file, options);
    let program = match parser.parse_program() {
        Ok(program) => program,
        Err(err) => {
            return Err(anyhow::anyhow!("Parsing failed: {:?}", err));
        }
    };

    let mut dictionary_tracker = DictionaryTracker::new();
    let mut feature_tracker = FeatureTracker::new();
    let mut identifier_tracker = IdentifierTracker::new(vec![
        &options.add_to_dictionary_helper,
        DEFAULT_DICTIONARY_IDENTIFIER,
    ]);
    let mut rewrite_tracker = RewriteTracker::new(vec![
        insert_helper_import(input_file.start_pos),
        insert_dictionary_declaration(input_file.start_pos),
    ]);

    visit(
        program,
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
        identifier_tracker.new_unused_identifier(&options.add_to_dictionary_helper);

    let module_kind = module_kind_for(
        filename,
        options,
        Some(feature_tracker.module_keyword_usage),
    );

    let template_parameters = TemplateParameters::new(
        dictionary,
        dictionary_identifier,
        &options.helpers_module,
        helper_identifier,
        module_kind,
    );

    let rewrite_plan = build_rewrite_plan(
        rewrite_tracker
            .take()
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

    let instrumented_code = rewrite_plan.apply(&mut input_file);

    let output = TransformOutput {
        code: instrumented_code,
        map: None,
    };

    Ok(output)
}
