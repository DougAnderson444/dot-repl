//! Wrapper around Graphvizm so we can implement the ui::GraphVizable trait
use graphvizm::Graphvizm;

pub struct GraphvizmWrapper {
    inner: Graphvizm,
}

impl GraphvizmWrapper {
    pub fn new(gviz: Graphvizm) -> Self {
        Self { inner: gviz }
    }
}

impl From<Graphvizm> for GraphvizmWrapper {
    fn from(gviz: Graphvizm) -> Self {
        Self::new(gviz)
    }
}

impl ui::GraphVizable for GraphvizmWrapper {
    fn render_dot(&self, dot: &str) -> String {
        match self.inner.render_dot(dot) {
            Ok(svg) => svg,
            Err(err) => format!("<svg><text x='10' y='20'>Error: {:?}</text></svg>", err),
        }
    }
}
