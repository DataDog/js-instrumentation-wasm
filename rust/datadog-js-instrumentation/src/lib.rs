use wasm_bindgen::prelude::*;

use js_instrumentation_shared::{InstrumentationInput, InstrumentationOptions};

#[wasm_bindgen]
pub fn transform(input: JsValue, options: JsValue) -> Result<JsValue, JsError> {
    let input: InstrumentationInput = serde_wasm_bindgen::from_value(input)?;
    let options: InstrumentationOptions = serde_wasm_bindgen::from_value(options)?;
    let transform_output = js_instrumentation_transform::apply_transform(&input, &options);
    let js_result = transform_output.map_err(|e| JsError::from(&*e))?;
    Ok(serde_wasm_bindgen::to_value(&js_result)?)
}
