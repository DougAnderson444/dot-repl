use crate::Route;
use dioxus::prelude::*;
use ui::{components::GraphEditor, StorageProvider};

#[component]
pub fn GraphVizWebView(key_path: String) -> Element {
    let mut dot_input = use_signal(String::new);
    let storage = use_context::<StorageProvider>();

    let p = key_path.to_string();
    let decoded = url_escape::decode(&p).to_string();

    info!("GraphView rendering for key_path: {}", decoded);

    use_effect(move || {
        // calling use_route makes use_effect reactive to Route changes when key_path changes
        let _ = use_route::<Route>();
        let dot = storage
            .load(&decoded)
            .map(|data| String::from_utf8_lossy(&data).to_string())
            .unwrap_or_else(|_| "digraph { file -> not_found; }".to_string());

        info!("Loaded DOT data: {}", dot);
        dot_input.set(dot.clone());
    });

    rsx! {
        GraphEditor { dot_input }
    }
}
