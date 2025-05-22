use anyhow::Result;

use js_instrumentation_shared::transform_options::TransformOptions;
use js_instrumentation_shared::transform_output::TransformOutput;
use js_instrumentation_shared::{log::debug_log, module_kind::module_kind_for};

use crate::dictionary::{DictionaryTracker, OptimizedDictionary, DEFAULT_DICTIONARY_IDENTIFIER};
use crate::features::FeatureTracker;
use crate::identifiers::IdentifierTracker;
use crate::rewrite::RewriteTracker;
use crate::{
    emit::{emit, EmitContext},
    input::{build_parser, InputFile},
    visitor::visit,
};

pub fn apply_transform(
    filename: &str,
    code: &str,
    options: &TransformOptions,
) -> Result<TransformOutput> {
    let input_file = InputFile::new(filename, code);
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
    let mut rewrite_tracker = RewriteTracker::new();

    visit(
        program,
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

    let emit_context = EmitContext::new(
        dictionary,
        dictionary_identifier,
        &options.helpers_module,
        helper_identifier,
        module_kind,
    );

    let rewrites = rewrite_tracker.rewrites;

    let (instrumented_code, emit_errors) = emit(emit_context, &input_file, &rewrites);
    if !emit_errors.is_empty() {
        debug_log(&format!(
            "Serialization failed: {:?}",
            emit_errors
                .into_iter()
                .map(|e| format!("{}", e))
                .collect::<Vec<String>>()
                .join(", ")
        ));
    }

    let output = TransformOutput {
        code: instrumented_code,
        map: None,
    };

    Ok(output)
}
