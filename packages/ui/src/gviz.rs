//! This module defines the trait details for managing data.
use std::sync::Arc;

pub trait GraphVizable {
    type Error;
    fn render_dot(&self, dot: &str) -> Result<String, Self::Error>;
}

// A storage provider context that wraps any storage implementation
#[derive(Clone)]
pub struct GVizProvider {
    inner: Arc<dyn GraphVizable<Error = crate::Error>>,
}

impl GVizProvider {
    pub fn new<G: GraphVizable<Error = crate::Error> + 'static>(graphviz: G) -> Self {
        Self {
            inner: Arc::new(graphviz),
        }
    }

    pub fn render_dot(&self, dot: &str) -> Result<String, crate::Error> {
        self.inner.render_dot(dot)
    }
}
