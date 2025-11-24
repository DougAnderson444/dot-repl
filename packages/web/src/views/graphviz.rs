// Graphviz
use crate::{Route, GVIZ_CONTEXT};
use base64::{prelude::BASE64_STANDARD, Engine as _};
use dioxus::prelude::*;
use ui::views::GraphView;

#[component]
pub fn GraphvizView(encoded_dot: String) -> Element {
    let dot = BASE64_STANDARD
        .decode(&encoded_dot)
        .unwrap_or_else(|_| b"digraph { error -> decoding; }".to_vec())
        .into_iter()
        .map(|b| b as char)
        .collect::<String>();
    if let Some(gviz) = GVIZ_CONTEXT.signal().read().as_ref() {
        let svg = gviz.render_dot(&dot);
        rsx! {
            GraphView {
                svg: svg,
            }
        }
    } else {
        return rsx! {
            div {
                class: "text-red-500 p-4 text-center",
                "Graphviz context unavailable."
            }
        };
    }
}
