//! wasm bindings for viz.js
//! viz.js is loaded int he main app as a Script tag, and exposes a global function `viz_instance`
#![allow(dead_code)]
use wasm_bindgen::prelude::*;
use wasm_bindgen_futures::JsFuture;
use web_sys::{js_sys, Element};

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
    RenderError(String),
}

impl From<VizError> for ui::Error {
    fn from(e: VizError) -> Self {
        ui::Error::DotRenderError(format!("{:?}", e))
    }
}

impl GViz {
    pub async fn new() -> Result<Self, VizError> {
        let promise = viz_instance();
        let js_instance = JsFuture::from(promise).await.map_err(|e| {
            VizError::RenderError(format!("Failed to create Viz instance: {:?}", e))
        })?;
        let instance: Viz = js_instance.into();
        Ok(Self { instance })
    }

    pub fn render_dot(&self, dot: &str) -> Result<String, VizError> {
        let element = self.instance.render_svg_element(dot);
        element
            .map(|el| el.outer_html())
            .map_err(|e| VizError::RenderError(format!("Failed to render DOT: {:?}", e)))
    }
}

impl ui::GraphVizable for GViz {
    type Error = ui::Error;
    fn render_dot(&self, dot: &str) -> Result<String, Self::Error> {
        self.render_dot(dot).map_err(ui::Error::from)
    }
}
