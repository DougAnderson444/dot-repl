//! This crate contains all shared UI for the workspace.
use std::future::Future;

mod hero;
pub use hero::Hero;

mod navbar;
pub use navbar::Navbar;

mod echo;
pub use echo::Echo;

pub mod views;
