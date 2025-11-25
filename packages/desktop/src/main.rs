//! A desktop application built with Dioxus that features routing and a navbar.
use dioxus::prelude::*;

use ui::Navbar;
use views::{Blog, GraphvizView, Home};

mod views;

use graphvizm::Graphvizm;

#[derive(Debug, Clone, Routable, PartialEq)]
#[rustfmt::skip]
enum Route {
    #[layout(DesktopNavbar)]
    #[route("/")]
    Home {},
    #[route("/blog/:id")]
    Blog { id: i32 },
    /// Graphviz Route 
    #[route("/graphviz/:encoded_dot")]
    GraphvizView { encoded_dot: String },
}

const MAIN_CSS: Asset = asset!("/assets/main.css");

// global signal for the GViz context
static GVIZ_CONTEXT: GlobalSignal<Option<Graphvizm>> = Signal::global(|| None);

fn main() {
    dioxus::launch(App);
}

#[component]
fn App() -> Element {
    // Build cool things ✌️
    // Create the Graphvizm instance once
    use_hook(|| {
        if let Ok(gviz) = Graphvizm::new() {
            GVIZ_CONTEXT.signal().write().replace(gviz);
        }
    });

    rsx! {
        // Global app resources
        // document::Link { rel: "stylesheet", href: MAIN_CSS }

        Router::<Route> {}
    }
}

/// A desktop-specific Router around the shared `Navbar` component
/// which allows us to use the desktop-specific `Route` enum.
#[component]
fn DesktopNavbar() -> Element {
    rsx! {
        Navbar {
            Link {
                to: Route::Home {},
                "Home"
            }
            Link {
                to: Route::Blog { id: 1 },
                "Blog"
            }
        }

        Outlet::<Route> {}
    }
}
