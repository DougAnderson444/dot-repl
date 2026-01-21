//! wasm bindings for viz.js
//! viz.js is loaded int he main app as a Script tag, and exposes a global function `viz_instance`
#![allow(dead_code)]
use wasm_bindgen::prelude::*;
use wasm_bindgen_futures::JsFuture;
use web_sys::{js_sys, Element};

use dot_repl_ui as ui;

#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(js_namespace = window, js_name = "viz_instance")]
    fn viz_instance() -> js_sys::Promise;
}

#[wasm_bindgen]
extern "C" {
    type Viz;
    #[wasm_bindgen(method, js_name = renderSVGElement, catch)]
    fn render_svg_element(this: &Viz, dot: &str) -> Result<Element, JsValue>;
}

pub struct GViz {
    instance: Viz,
}

#[derive(Debug)]
pub enum VizError {
    RenderError { message: String, line: Option<u32> },
}

impl From<VizError> for dot_repl_ui::Error {
    fn from(e: VizError) -> Self {
        match e {
            VizError::RenderError { message, line } => {
                dot_repl_ui::Error::DotRenderError(dot_repl_ui::error::RenderError {
                    errors: vec![dot_repl_ui::error::ErrorInfo {
                        level: dot_repl_ui::error::ErrorLevel::Error,
                        message,
                        line,
                    }],
                })
            }
        }
    }
}

impl GViz {
    pub async fn new() -> Result<Self, VizError> {
        let promise = viz_instance();
        let js_instance = JsFuture::from(promise)
            .await
            .map_err(|e| VizError::RenderError {
                message: format!("Failed to create Viz instance: {:?}", e),
                line: None,
            })?;
        let instance: Viz = js_instance.into();
        Ok(Self { instance })
    }

    pub fn render_dot(&self, dot: &str) -> Result<String, VizError> {
        let element = self.instance.render_svg_element(dot);
        element.map(|el| el.outer_html()).map_err(|e| {
            // Extract clean error message from JavaScript Error object
            let message = if let Some(err) = e.dyn_ref::<js_sys::Error>() {
                err.message()
                    .as_string()
                    .unwrap_or_else(|| format!("{:?}", e))
            } else {
                format!("{:?}", e)
            };

            // Extract line number from message (e.g., "syntax error in line 3")
            let line = extract_line_number(&message);

            VizError::RenderError { message, line }
        })
    }
}

/// Extract line number from error message
fn extract_line_number(msg: &str) -> Option<u32> {
    // Look for "line" followed by digits
    msg.find("line").and_then(|pos| {
        let after = &msg[pos + 4..];
        after
            .chars()
            .skip_while(|c| !c.is_ascii_digit())
            .take_while(|c| c.is_ascii_digit())
            .collect::<String>()
            .parse::<u32>()
            .ok()
    })
}

impl ui::GraphVizable for GViz {
    type Error = ui::Error;
    fn render_dot(&self, dot: &str) -> Result<String, Self::Error> {
        self.render_dot(dot).map_err(ui::Error::from)
    }
}
