use base64::{engine::general_purpose::URL_SAFE, Engine as _};
use dioxus::prelude::*;

use crate::{
    components::{GraphvizSvg, SvgBuildConfig},
    GVizProvider, StorageProvider,
};

#[component]
pub fn GraphView(key_path: String) -> Element {
    let gviz_signal = use_context::<Signal<Option<GVizProvider>>>();
    let storage = use_context::<StorageProvider>();

    let dot_key = URL_SAFE
        .decode(&key_path)
        .map(|data| String::from_utf8_lossy(&data).to_string())
        .unwrap_or_else(|_| "file_not_found".to_string());

    trace!("GraphView rendering for key_path: {}", dot_key);

    let dot = storage
        .load(&dot_key)
        .map(|data| String::from_utf8_lossy(&data).to_string())
        .unwrap_or_else(|_| "digraph { file -> not_found; }".to_string());

    let maybe_gviz = gviz_signal.read();
    if let Some(gviz) = maybe_gviz.as_ref() {
        let svg = gviz.render_dot(&dot);
        let svg_build_config = SvgBuildConfig {
            // TODO: Toggle this
            rough_style: true,
            ..Default::default()
        };

        rsx! {
            div {
                class: "w-full h-full overflow-auto",
                GraphvizSvg {
                    svg_text: &svg,
                    config: svg_build_config
                }
            }
        }
    } else {
        return rsx! {
            div {
                class: "text-grey-500 p-4 text-center",
                "Graphviz context loading..."
            }
        };
    }
    // Show key_path for now
    // rsx! {
    //         div {
    //             class: "p-4 text-center",
    //             "Storage, GraphVizSignal, key_path: {key_path} = {dot_key:?}"
    //         }
    // }
}
