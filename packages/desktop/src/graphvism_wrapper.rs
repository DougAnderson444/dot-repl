//! Wrapper around Graphvizm so we can implement the ui::GraphVizable trait
use graphvizm::{Graphvizm, GraphvizmError};

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
        self.inner.render_dot(dot).map_err(|e| match e {
            GraphvizmError::Render(render_error) => {
                let render_e = ui::error::RenderError {
                    errors: render_error
                        .errors
                        .into_iter()
                        .map(|err_info| ui::error::ErrorInfo {
                            level: match err_info.level {
                                graphvizm::ErrorLevel::Info => ui::error::ErrorLevel::Info,
                                graphvizm::ErrorLevel::Warning => ui::error::ErrorLevel::Warning,
                                graphvizm::ErrorLevel::Error => ui::error::ErrorLevel::Error,
                            },
                            message: err_info.message,
                            line: err_info.line,
                        })
                        .collect(),
                };
                ui::Error::DotRenderError(render_e)
            }
            _ => unreachable!(),
        })
    }
}
