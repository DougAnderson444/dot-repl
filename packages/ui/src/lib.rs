//! This crate contains all shared UI for the workspace.
use std::future::Future;

mod hero;
pub use hero::Hero;

mod navbar;
pub use navbar::Navbar;

mod echo;
pub use echo::Echo;

pub mod views;

// Graphvizable trait with new() and render_dot(&self, dot: &st) -> Result<String, Self::Error>
pub trait Graphvizable: Sized {
    type Error;

    fn new() -> impl Future<Output = Result<Self, Self::Error>> + CondSend;

    fn render_dot(&self, dot: &str) -> String;
}

// CondSend is unti is wasm-bindgen, real Send if not wasm target
#[cfg(target_arch = "wasm32")]
pub trait CondSend {}
#[cfg(not(target_arch = "wasm32"))]
pub trait CondSend: Send {}

// Blanket impl for all types
#[cfg(target_arch = "wasm32")]
impl<T> CondSend for T {}
#[cfg(not(target_arch = "wasm32"))]
impl<T: Send> CondSend for T {}
