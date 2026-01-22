//! This module contains the component that looks up the data by its key
//! and displays the graph. This view is a reference implementation of how to
//! use the components from this library with routing.
use crate::{
    components::{CodeEditor, DotDisplay, ErrorOverlay},
    hooks::use_graph_editor_logic,
    platform, StorageProvider,
};
use dioxus::prelude::*;

static KITCHEN_SINK: &str = include_str!("../../assets/dot/kitchen_sink.dot");
const TAILWIND_CSS: Asset = asset!("../../assets/tailwind.css");

#[component]
pub fn GraphView<R>(route: R, key_path: String, rough_enabled: Signal<bool>) -> Element
where
    R: Routable + Clone + PartialEq,
{
    let mut dot_input = use_signal(String::new);
    let storage = use_context::<StorageProvider>();
    let mut editor = use_graph_editor_logic();

    let decoded = url_escape::decode(&key_path).to_string();

    let storage_clone = storage.clone();
    let decoded_clone = decoded.clone();
    use_effect(move || {
        // triggers when key_path changes
        let _ = use_route::<R>();
        let dot = storage_clone
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
            });

        dot_input.set(dot);
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
        document::Link { rel: "stylesheet", href: TAILWIND_CSS }
        div {
            class: "flex h-full overflow-hidden",

            // Left panel: collapsible
            if !(editor.collapsed)() {
                div {
                    class: "flex flex-col flex-1 bg-gray-50 border-r border-gray-200 overflow-none w-1/2 max-w-[800px]",
                    h2 {
                        class: "text-xl font-bold text-gray-800 p-2 border-b border-gray-200 flex justify-between items-center",
                        "DOT Source"
                        button {
                            class: "ml-2 px-2 py-1 text-xs bg-gray-200 hover:bg-gray-300 rounded",
                            onclick: move |_| editor.collapsed.set(true),
                            "⟨⟨⟨⟨"
                        }
                    }
                    CodeEditor {
                        value: dot_input(),
                        oninput: move |new_value: String| dot_input.set(new_value),
                        error_lines: (editor.error_lines)(),
                        placeholder: "Enter your DOT graph here...".to_string()
                    }
                }
            } else {
                // Collapsed: show expand button
                div {
                    class: "flex flex-col text-xl font-bold bg-gray-50 border-r border-gray-200 w-[32px]",
                    button {
                        class: "px-2 py-1 text-xs bg-gray-200 hover:bg-gray-300 rounded mt-4 relative",
                        onclick: move |_| editor.collapsed.set(false),
                        "⟩⟩⟩⟩"
                    }
                }
            }

            // Right panel: Preview
            div {
                class: "flex flex-col bg-white overflow-auto flex-1 relative",
                ErrorOverlay {
                    errors: editor.render_errors
                }
                div {
                    class: "flex-1 bg-white overflow-auto",
                    DotDisplay {
                        dot: dot_input(),
                        error_signal: editor.render_errors,
                        rough: rough_enabled(),
                    }
                }
            }
        }
    }
}
