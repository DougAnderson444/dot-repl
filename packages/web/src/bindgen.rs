use wasm_bindgen::prelude::*;
use web_sys::{js_sys, Element};

#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(js_namespace = ["window", "Viz"])]
    fn instance() -> js_sys::Promise;
}

#[wasm_bindgen]
extern "C" {
    type Viz;

    #[wasm_bindgen(method, js_name = renderSVGElement)]
    fn render_svg_element(this: &Viz, dot: &str) -> Element;
}
