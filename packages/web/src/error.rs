use dot_repl_ui as ui;

#[derive(Debug, thiserror::Error, Clone)]
pub enum Error {
    /// Failed to initialize the storage
    #[error("Failed to initialize storage: {0}")]
    StorageFailure(String),

    /// From gloo-storage errors
    #[error("Storage error: {0}")]
    StorageError(String),

    /// Decoding error
    #[error("Decoding error: {0}")]
    DecodeError(#[from] base64::DecodeError),

    /// from ui::Error
    #[error("UI error: {0}")]
    UIError(#[from] ui::Error),
}
