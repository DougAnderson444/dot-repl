use crate::Route;
use dioxus::prelude::*;
use dot_repl_ui::views::GraphView;

/// GraphView is generic over R, but we need to set R to route here as it's only
/// used in the top level.
#[component]
pub fn GraphVizWebView(key_path: String) -> Element {
    let route = use_route::<Route>();

    rsx! {
        GraphView { route, key_path }
    }
}
