use dot_repl_ui as ui;

#[derive(Debug, Clone, thiserror::Error)]
pub enum Error {
    /// Failed to initialize the storage
    #[error("Failed to initialize storage: {0}")]
    StorageFailure(String),

    /// From<std::io::Error>
    #[error("I/O error: {0}")]
    Io(String),

    /// Fro ui::Error
    #[error("UI error: {0}")]
    Ui(#[from] ui::Error),
}
