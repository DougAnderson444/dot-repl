//! Graphviz
use base64::{engine::general_purpose::URL_SAFE, Engine as _};
use dioxus::prelude::*;
use graphvizm::Graphvizm;
use ui::components::{GraphvizSvg, SvgBuildConfig};

use crate::GVIZ_CONTEXT;

#[component]
pub fn GraphvizView(encoded_dot: String) -> Element {
    // Reactively render SVG whenever dot_source signal changes
    let svg_result = use_memo(move || {
        let dot = URL_SAFE
            .decode(&encoded_dot)
            .unwrap_or_else(|_| b"digraph { error -> decoding; }".to_vec())
            .into_iter()
            .map(|b| b as char)
            .collect::<String>();
        GVIZ_CONTEXT
            .signal()
            .read()
            .as_ref()
            .and_then(|graphviz| graphviz.render_dot(&dot).ok())
    });

    let svg_build_config = SvgBuildConfig::default();

    rsx! {
        div {
            class: "w-full h-full overflow-auto",

            match svg_result() {
                Some(svg) => rsx! {
                    div {
                        class: "p-2",
                        GraphvizSvg {
                            svg_text: &svg,
                            config: svg_build_config
                        }
                    }
                },
                None => rsx! {
                    div {
                        class: "text-red-500 p-4 text-center",
                        "No graph to render."
                    }
                }
            }
        }
    }
}
