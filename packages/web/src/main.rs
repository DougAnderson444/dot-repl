//! Web-specific entry point

use dioxus::prelude::*;
use dot_repl_ui::components::fonts::ARCHITECTS_DAUGHTER_FAMILY;
use dot_repl_ui::Navbar;
use dot_repl_web::WebApp;

mod views;
use views::{Blog, GraphVizWebView, Home};

const TAILWIND_CSS: Asset = asset!("/assets/tailwind.css");
const FAVICON: Asset = asset!("/assets/favicon.ico");

#[derive(Debug, Clone, Routable, PartialEq)]
#[rustfmt::skip]
enum Route {
    #[layout(WebNavbar)]
    #[route("/")]
    Home {},
    #[route("/blog/:id")]
    Blog { id: i32 },
    /// Graphviz Route 
    #[route("/:key_path")]
    GraphVizWebView { key_path: String },
}

fn main() {
    dioxus::launch(App);
}

#[component]
fn App() -> Element {
    let rough_enabled = use_signal(|| false);
    use_context_provider(|| rough_enabled);

    rsx! {
        document::Link { rel: "stylesheet", href: TAILWIND_CSS }
        document::Link { rel: "icon", href: FAVICON }
        WebApp {
            div {
                class: "h-screen flex flex-col",
                Router::<Route> {}
            }
        }
    }
}

/// A web-specific Router around the shared `Navbar` component
/// which allows us to use the web-specific `Route` enum.
#[component]
fn WebNavbar() -> Element {
    let mut rough_enabled = use_context::<Signal<bool>>();
    rsx! {
        div {
            style: "font-family: {ARCHITECTS_DAUGHTER_FAMILY}",
            class: "font-bold",
            Navbar {
                Link {
                    to: Route::Home {},
                    "Drawn Systems"
                }
                Link {
                    to: Route::Blog { id: 1 },
                    "Blog"
                }
                label {
                    class: "flex items-center gap-2 text-sm text-neutral-200",
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
        }

        Outlet::<Route> {}
    }
}
