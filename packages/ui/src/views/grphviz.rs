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

use crate::{components::GraphEditor, platform, StorageProvider};

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

    let storage_clone = storage.clone();
    let decoded_clone = decoded.clone();
    use_effect(move || {
        // triggers when key_path changes
        let _ = use_route::<R>();
        let dot = storage_clone
            .load(&decoded_clone)
            .map(|data| String::from_utf8_lossy(&data).to_string())
            .unwrap_or_else(|_| {
                // THis file doesn't exist in storage, so we either load the static default
                // or create a new file with starter content.
                let d = if decoded_clone == "kitchen_sink.dot" {
                    KITCHEN_SINK.to_string()
                } else {
                    "digraph { creating -> new_file; }".to_string()
                };
                if let Err(e) = storage_clone.save(&decoded_clone, d.as_bytes()) {
                    error!("Failed to save new file to storage: {}", e);
                }
                d
            });

        dot_input.set(dot.clone());
    });

    // Add auto-save effect with debouncing
    use_effect(move || {
        let current_dot = dot_input();

        // Skip saving on initial load or empty content
        if current_dot.is_empty() {
            return;
        }

        let storage_clone = storage.clone();
        let decoded_clone = decoded.clone();
        // Debounce: wait 500ms after last edit before saving
        spawn(async move {
            platform::sleep(std::time::Duration::from_millis(500)).await;

            if let Err(e) = storage_clone.save(&decoded_clone, current_dot.as_bytes()) {
                error!("Failed to auto-save changes: {}", e);
            } else {
                info!("Auto-saved changes to {}", decoded_clone);
            }
        });
    });

    rsx! {
        GraphEditor { dot_input }
    }
}
