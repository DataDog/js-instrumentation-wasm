use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize)]
pub struct InstrumentationOutput {
    /// The filename or id of the code that was instrumented.
    pub id: String,
    /// The instrumented source code.
    pub code: String,
    /// The source map for the instrumented code. If an input source map was specified,
    /// this map will be the combination of the input source map and the instrumentation
    /// source map -- in other words, the two source maps will be chained.
    pub map: String,
}
