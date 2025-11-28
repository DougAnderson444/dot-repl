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
    type Error = ui::Error;
    fn render_dot(&self, dot: &str) -> Result<String, Self::Error> {
        self.inner
            .render_dot(dot)
            .map_err(|e| ui::Error::DotRenderError(e.to_string()))
    }
}
