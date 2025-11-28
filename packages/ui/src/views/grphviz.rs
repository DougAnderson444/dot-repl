use base64::{engine::general_purpose::URL_SAFE, Engine as _};
use dioxus::prelude::*;

use crate::{components::GraphEditor, StorageProvider};

#[component]
pub fn GraphView(key_path: String) -> Element {
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

    rsx! {
        GraphEditor { dot_initial: dot }
    }
}
