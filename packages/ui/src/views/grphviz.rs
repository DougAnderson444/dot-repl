//! This module contains the component that looks up the data by it's key,
//! and displays the graph.
//!
//! We have a system which allows user to take starter dot files and cusotmize
//! them, which are saved to their storage system.
//!
//! So when we look up a key,the following stepsneed to happen in this order:
//! 1. Check if the key exists in storage, if so, load it.
//! 2. If not available from local storage, try the static default template, if name exists.
//! 3. If the default static key and data doesn't exists, create
//!    the data in dynamic storage. This allows users to creates a link to a
//!    new file in their system and build out their hyperlinks.
use dioxus::prelude::*;

use crate::{components::GraphEditor, StorageProvider};

static KITCHEN_SINK: &str = include_str!("../../assets/dot/kitchen_sink.dot");

#[component]
pub fn GraphView<R>(route: R, key_path: String) -> Element
where
    R: Routable + Clone + PartialEq,
{
    let mut dot_input = use_signal(String::new);
    let storage = use_context::<StorageProvider>();

    let p = key_path.to_string();
    let decoded = url_escape::decode(&p).to_string();

    info!("GraphView rendering for key_path: {}", decoded);

    use_effect(move || {
        // triggers when key_path changes
        let _ = use_route::<R>();
        let dot = storage
            .load(&decoded)
            .map(|data| String::from_utf8_lossy(&data).to_string())
            .unwrap_or_else(|_| {
                // THis file doesn't exist in storage, so we either load the static default
                // or create a new file with starter content.
                let d = if decoded == "kitchen_sink.dot" {
                    KITCHEN_SINK.to_string()
                } else {
                    "digraph { creating -> new_file; }".to_string()
                };
                if let Err(e) = storage.save(&decoded, d.as_bytes()) {
                    error!("Failed to save new file to storage: {}", e);
                }
                d
            });

        info!("Loaded DOT data: {}", dot);
        dot_input.set(dot.clone());
    });

    rsx! {
        GraphEditor { dot_input }
    }
}
