use dioxus::prelude::*;

use crate::components::SvgDisplay;

#[component]
pub fn GraphView(svg: String) -> Element {
    rsx! {
        SvgDisplay {
            svg: svg,
        }
    }
}
