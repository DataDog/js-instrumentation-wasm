use wasm_bindgen::prelude::*;

use js_instrumentation_shared::transform_options::TransformOptions;

#[wasm_bindgen]
pub fn transform(filename: &str, code: &str, options: JsValue) -> Result<JsValue, JsError> {
    let options: TransformOptions = serde_wasm_bindgen::from_value(options)?;
    let transform_output = js_instrumentation_transform::apply_transform(filename, code, &options);
    let js_result = transform_output.map_err(|e| JsError::from(&*e))?;
    Ok(serde_wasm_bindgen::to_value(&js_result)?)
}
