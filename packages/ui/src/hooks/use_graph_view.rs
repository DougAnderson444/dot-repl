//! A "headless" hook containing the logic for the GraphView component.
use dioxus::prelude::*;
use crate::{platform, StorageProvider};

static KITCHEN_SINK: &str = include_str!("../../assets/dot/kitchen_sink.dot");

/// A headless hook containing the logic for the GraphView component.
///
/// This hook encapsulates the data loading and saving logic for a graph,
/// allowing the consumer to build a custom UI around it.
#[must_use]
pub fn use_graph_view_logic(key_path: String) -> Signal<String> {
    let storage = use_context::<StorageProvider>();

    // Decode the key_path
    let decoded = url_escape::decode(&key_path).to_string();

    // Load the file content - this re-runs whenever key_path prop changes
    let initial_content = {
        info!("use_graph_view_logic: Loading file for key_path: {}", key_path);
        let storage_clone = storage.clone();
        let decoded_clone = decoded.clone();
        storage_clone
            .load(&decoded_clone)
            .map(|data| String::from_utf8_lossy(&data).to_string())
            .unwrap_or_else(|_| {
                let d = if decoded_clone == "kitchen_sink.dot" {
                    KITCHEN_SINK.to_string()
                } else {
                    "digraph { creating -> new_file; }".to_string()
                };
                if let Err(e) = storage_clone.save(&decoded_clone, d.as_bytes()) {
                    error!("Failed to save new file to storage: {}", e);
                }
                d
            })
    };

    // Create signal with the loaded content
    // IMPORTANT: use_signal is called every render, but it only initializes once
    // We need to UPDATE the signal when key_path changes
    let mut dot_input = use_signal(|| initial_content.clone());
    
    // Update the signal content when key_path changes
    use_effect(move || {
        info!("use_graph_view_logic effect: Updating dot_input signal with new content (len: {})", initial_content.len());
        dot_input.set(initial_content.clone());
    });

    // Effect for auto-saving with debouncing
    use_effect(move || {
        let current_dot = dot_input();
        if current_dot.is_empty() {
            return;
        }

        let decoded_clone = decoded.clone();
        let storage_clone = storage.clone();
        spawn(async move {
            platform::sleep(std::time::Duration::from_millis(500)).await;

            if let Err(e) = storage_clone.save(&decoded_clone, current_dot.as_bytes()) {
                error!("Failed to auto-save changes: {}", e);
            } else {
                info!("Auto-saved changes to {}", decoded_clone);
            }
        });
    });

    dot_input
}
