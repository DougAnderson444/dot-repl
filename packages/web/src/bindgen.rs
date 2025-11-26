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
    #[wasm_bindgen(method, js_name = renderSVGElement)]
    fn render_svg_element(this: &Viz, dot: &str) -> Element;
}

pub struct GViz {
    instance: Viz,
}

#[derive(Debug)]
pub enum VizError {
    RenderError(String),
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

    pub fn render_dot(&self, dot: &str) -> String {
        let element = self.instance.render_svg_element(dot);
        element.outer_html()
    }
}

impl ui::GraphVizable for GViz {
    fn render_dot(&self, dot: &str) -> String {
        self.render_dot(dot)
    }
}
