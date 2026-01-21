//! Web-specific entry point

use dioxus::prelude::*;
use dot_repl_web::WebApp;
use ui::components::fonts::ARCHITECTS_DAUGHTER_FAMILY;
use ui::Navbar;

mod views;
use views::{Blog, GraphVizWebView, Home};

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
    rsx! {
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
            }
        }

        Outlet::<Route> {}
    }
}
