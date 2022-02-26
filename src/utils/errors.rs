use serde::Serialize;

/**
Default structure to respond to an errored request.

# Fields
- `error`: The error message.
*/
#[derive(Debug, Serialize)]
pub struct ErrorHandling {
    pub error: String,
}

impl ErrorHandling {
    pub fn new(text: String) -> Self {
        Self { error: text }
    }
}
