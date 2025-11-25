//! A desktop application built with Dioxus that features routing and a navbar.
mod storage;

mod error;
pub use error::Error;

use dioxus::prelude::*;

use ui::{Navbar, StorageProvider};
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

// global signal for the GViz context
static GVIZ_CONTEXT: GlobalSignal<Option<Graphvizm>> = Signal::global(|| None);

fn main() {
    dioxus::launch(App);
}

#[component]
fn App() -> Element {
    // Build cool things ✌️
    let storage = storage::DesktopStorage::new().unwrap();
    let storage_provider = StorageProvider::new(storage.clone());

    // provide storage in context for all child elements
    use_context_provider(|| storage_provider);

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
