use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize)]
pub struct InstrumentationInput {
    /// The filename or id of the code to instrument.
    pub id: String,
    /// The source code to instrument.
    pub code: String,
    /// The source map for the code to instrument, if any.
    pub map: Option<String>,
}
