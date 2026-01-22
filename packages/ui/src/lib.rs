//! This crate contains all shared UI for the workspace.
mod hero;
pub use hero::Hero;

mod navbar;
pub use navbar::Navbar;

pub mod hooks;

// mod echo;
// pub use echo::Echo;

pub mod views;

pub mod components;

mod storage;
pub use storage::{PlatformStorage, StorageProvider};

pub mod error;
pub use error::Error;

mod gviz;
pub use gviz::{GVizProvider, GraphVizable};

/// Platform specific utilities
mod platform;
