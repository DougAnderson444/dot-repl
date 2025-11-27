//! The REPL (Read-Eval-Print Loop) component for the UI.
use dioxus::prelude::*;

use crate::components::DotDisplay;

// Example usage component with live editing
#[component]
pub fn GraphEditor(dot_initial: String) -> Element {
    let mut dot_input = use_signal(|| dot_initial);
    let mut collapsed = use_signal(|| false);

    // Chat panel state
    let mut chat_collapsed = use_signal(|| false);
    let mut chat_input = use_signal(|| String::new());

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
                        textarea {
                            class: "flex-1 font-mono text-sm p-4 border-none outline-none resize-none bg-white overflow-auto",
                            value: "{dot_input}",
                            oninput: move |e| dot_input.set(e.value()),
                            placeholder: "Enter your DOT graph here..."
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
        class: "flex flex-col bg-white overflow-auto flex-1", // removed 'relative'
        div {
            class: "flex-1 bg-white overflow-scroll",
            DotDisplay {
                dot: dot_input()
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
