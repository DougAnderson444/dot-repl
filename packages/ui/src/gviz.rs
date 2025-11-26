//! This module defines the trait details for managing data.
use std::sync::Arc;

pub trait GraphVizable {
    fn render_dot(&self, dot: &str) -> String;
}

// A storage provider context that wraps any storage implementation
#[derive(Clone)]
pub struct GVizProvider {
    inner: Arc<dyn GraphVizable>,
}

impl GVizProvider {
    pub fn new<G: GraphVizable + 'static>(graphviz: G) -> Self {
        Self {
            inner: Arc::new(graphviz),
        }
    }

    pub fn render_dot(&self, dot: &str) -> String {
        self.inner.render_dot(dot)
    }
}
