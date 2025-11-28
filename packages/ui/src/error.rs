//! UI Errors
use thiserror::Error;

#[derive(Debug, Clone, Error)]
pub enum Error {
    /// Storage-related failure
    #[error("Storage failure: {0}")]
    StorageFailure(&'static str),

    /// Generic I/O error
    #[error("I/O error: {0}")]
    Io(String),

    /// DOT Render error
    #[error("DOT Render error: {0}")]
    DotRenderError(String),
}
