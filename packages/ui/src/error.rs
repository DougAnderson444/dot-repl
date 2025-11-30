//! UI Errors
use std::fmt;

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
    #[error(transparent)]
    DotRenderError(#[from] RenderError),
}

#[derive(Debug, Clone)]
pub struct RenderError {
    pub errors: Vec<ErrorInfo>,
}

#[derive(Debug, Clone)]
pub struct ErrorInfo {
    pub level: ErrorLevel,
    pub message: String,
    pub line: Option<u32>,
}

#[derive(Debug, Clone)]
pub enum ErrorLevel {
    Info,
    Warning,
    Error,
}

impl std::error::Error for RenderError {}

impl fmt::Display for RenderError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        if self.errors.is_empty() {
            return write!(f, "Render failed: unknown error");
        }

        if self.errors.len() == 1 {
            let error = &self.errors[0];
            if let Some(line) = error.line {
                write!(f, "Render failed at line {}: {}", line, error.message)
            } else {
                write!(f, "Render failed: {}", error.message)
            }
        } else {
            writeln!(f, "Render failed with {} error(s):", self.errors.len())?;
            for (i, error) in self.errors.iter().enumerate() {
                let level_str = match error.level {
                    ErrorLevel::Error => "ERROR",
                    ErrorLevel::Warning => "WARNING",
                    ErrorLevel::Info => "INFO",
                };

                if let Some(line) = error.line {
                    writeln!(
                        f,
                        "  {}. [{}] Line {}: {}",
                        i + 1,
                        level_str,
                        line,
                        error.message
                    )?;
                } else {
                    writeln!(f, "  {}. [{}] {}", i + 1, level_str, error.message)?;
                }
            }
            Ok(())
        }
    }
}
