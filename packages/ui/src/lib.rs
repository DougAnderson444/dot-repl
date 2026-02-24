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

/// A signal provided in context by `WebApp` (or any platform host) that flips
/// to `true` once all static DOT assets have been preloaded into storage.
/// Components that read storage on mount (e.g. `GraphView`) subscribe to this
/// so they automatically re-try after the preload completes.
pub type PreloadComplete = dioxus::prelude::Signal<bool>;
