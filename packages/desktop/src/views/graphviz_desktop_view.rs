use crate::Route;
use dioxus::prelude::*;
use dot_repl_ui::views::GraphView;

/// Reference implementation showing how to use the dot-repl GraphView component
#[component]
pub fn GraphVizDesktopView(key_path: String) -> Element {
    let route = use_route::<Route>();
    let rough_enabled = use_context::<Signal<bool>>();

    rsx! {
        GraphView { route, key_path, rough_enabled }
    }
}
