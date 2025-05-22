use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize)]
pub struct TransformOutput {
    pub code: String,
    pub map: Option<String>,
}
