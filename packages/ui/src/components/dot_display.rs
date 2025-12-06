//! Component to render DOT graphs using Graphvizm and Dioxus
pub mod fonts;
use fonts::{ARCHITECTS_DAUGHTER_CSS, ARCHITECTS_DAUGHTER_FAMILY};

mod render;
pub use render::{GraphvizSvg, SvgBuildConfig};

use dioxus::prelude::*;

use crate::error::RenderError;
use crate::GVizProvider;

#[component]
pub fn DotDisplay(
    dot: String,
    error_signal: Signal<Option<RenderError>>,
    rough_enabled: Signal<bool>,
) -> Element {
    let mut svg_signal = use_signal(|| None::<String>);
    let gviz_signal = use_context::<Signal<Option<GVizProvider>>>();
    let maybe_gviz = gviz_signal.read();

    match maybe_gviz.as_ref() {
        // Case 1: No gviz provider yet
        None => {
            rsx! {
                div {
                    class: "text-grey-500 p-4 text-center",
                    "Graphviz context loading..."
                }
            }
        }
        // Cases 2-4: We have gviz
        Some(gviz) => {
            // Case 2: Empty dot string
            if dot.is_empty() {
                // Clear error when input is empty
                if error_signal.read().is_some() {
                    error_signal.set(None);
                }
                return rsx! {
                    div {
                        class: "text-grey-500 p-4 text-center",
                        "Enter DOT string"
                    }
                };
            }

            // Case 3 & 4: Try to render, update signal if valid and different
            match gviz.render_dot(&dot) {
                Ok(rendered_svg) => {
                    // Clear any previous errors
                    if error_signal.read().is_some() {
                        error_signal.set(None);
                    }

                    // Case 4: Valid render - update signal if different
                    if svg_signal.read().as_ref() != Some(&rendered_svg) {
                        svg_signal.set(Some(rendered_svg));
                    }
                }
                Err(e) => {
                    // Update error signal with render error
                    match e {
                        crate::Error::DotRenderError(render_error) => {
                            error_signal.set(Some(render_error));
                        }
                        _ => {
                            // For other error types, we don't have line info
                            // but we could still clear the error signal
                        }
                    }
                }
            }

            // Display current SVG if we have one
            if let Some(svg) = svg_signal.read().as_ref() {
                let rough = rough_enabled();
                let config = SvgBuildConfig {
                    rough_style: rough,
                    ..Default::default()
                };

                rsx! {
                    div {
                        class: "w-full h-full overflow-auto",
                        GraphvizSvg {
                            key: key,
                            svg_text: svg,
                            config: config
                        }
                    }
                }
            } else {
                // No SVG yet (first render with invalid dot)
                rsx! {
                    div {
                        class: "text-grey-500 p-4 text-center",
                        "Invalid Graphviz syntax"
                    }
                }
            }
        }
    }
}

//
// // Simple display-only component
// #[component]
// pub fn SimpleGraph() -> Element {
//     let dot = use_signal(|| {
//         r#"digraph Example {
//     rankdir=LR;
//     node [shape=box, style=filled, fillcolor=lightblue];
//
//     Start -> Process;
//     Process -> Decision;
//     Decision -> End [label="yes"];
//     Decision -> Process [label="no"];
// }"#
//         .to_string()
//     });
//
//     rsx! {
//         div {
//             class: "container mx-auto p-8",
//
//             h1 {
//                 class: "text-3xl font-bold mb-6 text-gray-900",
//                 "My Graph"
//             }
//
//             div {
//                 class: "bg-white rounded-lg shadow-lg p-6 border border-gray-200",
//                 DotDisplay {
//                     dot_source: dot
//                 }
//             }
//         }
//     }
// }
//
// // Dark mode variant of the editor
// #[component]
// pub fn DarkGraphEditor() -> Element {
//     let mut dot_input = use_signal(|| {
//         r#"digraph G {
//     A -> B;
//     B -> C;
//     C -> A;
// }"#
//         .to_string()
//     });
//
//     rsx! {
//         div {
//             class: "grid grid-cols-1 md:grid-cols-2 gap-4 h-screen p-4 bg-gray-900",
//
//             div {
//                 class: "flex flex-col space-y-4",
//
//                 h2 {
//                     class: "text-2xl font-bold text-gray-100",
//                     "DOT Source"
//                 }
//
//                 textarea {
//                     class: "flex-1 font-mono text-sm p-3 border border-gray-700 rounded-lg
//                             focus:outline-none focus:ring-2 focus:ring-blue-400 focus:border-transparent
//                             resize-none bg-gray-800 text-gray-100 shadow-lg",
//                     rows: 10,
//                     value: "{dot_input}",
//                     oninput: move |e| dot_input.set(e.value()),
//                     placeholder: "Enter your DOT graph here..."
//                 }
//             }
//
//             div {
//                 class: "flex flex-col space-y-4",
//
//                 h2 {
//                     class: "text-2xl font-bold text-gray-100",
//                     "Preview"
//                 }
//
//                 div {
//                     class: "flex-1 border border-gray-700 rounded-lg overflow-hidden bg-gray-800 shadow-lg",
//                     DotDisplay {
//                         dot_source: dot_input
//                     }
//                 }
//             }
//         }
//     }
// }
//
// // Compact inline graph display with ReadOnlySignal
// #[component]
// pub fn InlineGraph(dot_source: ReadSignal<String>, title: Option<String>) -> Element {
//     rsx! {
//         div {
//             class: "my-4",
//
//             if let Some(title) = title {
//                 h3 {
//                     class: "text-lg font-semibold text-gray-700 mb-2",
//                     "{title}"
//                 }
//             }
//
//             div {
//                 class: "bg-gray-50 rounded-md p-4 border border-gray-200",
//                 DotDisplay {
//                     dot_source
//                 }
//             }
//         }
//     }
// }
//
// // Example: Dynamic graph that updates based on user interaction
// #[component]
// pub fn DynamicGraphExample() -> Element {
//     let mut node_count = use_signal(|| 3);
//
//     // Compute the DOT string based on node_count
//     let dot_source = use_memo(move || {
//         let count = node_count();
//         let mut edges = String::new();
//         for i in 0..count {
//             if i < count - 1 {
//                 edges.push_str(&format!("    Node{} -> Node{};\n", i, i + 1));
//             }
//         }
//         format!("digraph Dynamic {{\n    rankdir=LR;\n{}}}", edges)
//     });
//
//     rsx! {
//         div {
//             class: "container mx-auto p-8",
//
//             div {
//                 class: "mb-4 flex items-center gap-4",
//
//                 label {
//                     class: "text-lg font-semibold text-gray-700",
//                     "Node Count: {node_count}"
//                 }
//
//                 input {
//                     r#type: "range",
//                     class: "w-64",
//                     min: "2",
//                     max: "10",
//                     value: "{node_count}",
//                     oninput: move |e| {
//                         if let Ok(val) = e.value().parse::<i32>() {
//                             node_count.set(val);
//                         }
//                     }
//                 }
//             }
//
//             div {
//                 class: "bg-white rounded-lg shadow-lg p-6 border border-gray-200",
//                 DotDisplay {
//                     dot_source: Signal::new(dot_source())
//                 }
//             }
//         }
//     }
// }
