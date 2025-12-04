//! The REPL (Read-Eval-Print Loop) component for the UI.
use dioxus::prelude::*;

use crate::components::{CodeEditor, DotDisplay, ErrorOverlay};
use crate::error::RenderError;

// Example usage component with live editing
#[component]
pub fn GraphEditor(dot_input: Signal<String>) -> Element {
    let mut collapsed = use_signal(|| false);
    // Use global rough_enabled state from context instead of local state
    let mut rough_enabled = use_context::<Signal<bool>>();
    let mut chat_input = use_signal(String::new);
    let render_errors = use_signal(|| None::<RenderError>);

    // Calculate which lines have errors
    let error_lines = use_memo(move || {
        render_errors()
            .map(|err| {
                err.errors
                    .iter()
                    .filter_map(|e| e.line)
                    .collect::<Vec<u32>>()
            })
            .unwrap_or_default()
    });

    rsx! {
            div {
                class: "flex h-full overflow-hidden",

                // Left panel: collapsible
                if !collapsed() {
                    div {
                        class: "flex flex-col flex-1 bg-gray-50 border-r border-gray-200 overflow-auto w-1/2 max-w-[800px]",
                        h2 {
                            class: "text-xl font-bold text-gray-800 p-2 border-b border-gray-200 flex justify-between items-center",
                            "DOT Source"
                            button {
                                class: "ml-2 px-2 py-1 text-xs bg-gray-200 hover:bg-gray-300 rounded",
                                onclick: move |_| collapsed.set(true),
                                "⟨⟨⟨⟨"
                            }
                        }
                        CodeEditor {
                            value: dot_input(),
                            oninput: move |new_value: String| dot_input.set(new_value),
                            error_lines: error_lines(),
                            placeholder: "Enter your DOT graph here...".to_string()
                        }
                    }
                } else {
                    // Collapsed: show expand button
                    div {
                        // The div is more narrow when collapsed, so we need a way to show this button
                        // overtop of the right panel
                        class: "flex flex-col text-xl font-bold bg-gray-50 border-r border-gray-200 w-[32px]",
                        button {
                            class: "px-2 py-1 text-xs bg-gray-200 hover:bg-gray-300 rounded mt-4 relative",
                            onclick: move |_| collapsed.set(false),
                            "⟩⟩⟩⟩"
                        }
                    }
                }

    // Right panel: Preview + Chat
    div {
        class: "flex flex-col bg-white overflow-auto flex-1 relative",
        div {
            class: "flex items-center justify-between px-4 py-2 bg-gray-50 border-b",
            span { class: "text-sm font-medium", "Preview" }
            label {
                class: "flex items-center gap-2 text-sm",
                input {
                    r#type: "checkbox",
                    checked: rough_enabled(),
                    onchange: move |_| {
                        rough_enabled.toggle()
                    },
                }
                "Rough Style"
            }
        }
        // Error overlay
        ErrorOverlay {
            errors: render_errors
        }

        div {
            class: "flex-1 bg-white overflow-auto",
            DotDisplay {
                dot: dot_input(),
                error_signal: render_errors,
                rough_enabled: rough_enabled,
            }
        }
        // Chat panel sits at the bottom, not absolute
        div {
            class: "w-full bg-gray-50 border-t border-gray-200 p-2 flex items-center",
            input {
                class: "flex-1 px-2 py-1 text-sm rounded border border-gray-300 bg-white",
                r#type: "text",
                value: "{chat_input}",
                oninput: move |e| chat_input.set(e.value()),
                placeholder: "Request an edit...",
            }
            button {
                class: "ml-2 px-2 py-1 text-xs bg-blue-500 hover:bg-blue-600 text-white rounded",
                onclick: move |_| {
                    chat_input.set(String::new());
                },
                "Send"
            }
        }
    }
            }
        }
}
