#[derive(Debug, thiserror::Error)]
pub enum Error {
    /// Failed to initialize the storage
    #[error("Failed to initialize storage: {0}")]
    StorageFailure(&'static str),

    /// From<std::io::Error>
    #[error("I/O error: {0}")]
    Io(#[from] std::io::Error),
}
