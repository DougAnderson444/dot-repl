//! A desktop application built with Dioxus that features routing and a navbar.
mod graphvism_wrapper;
mod storage;

mod error;
pub use error::Error;

use dioxus::prelude::*;

use ui::{GVizProvider, Navbar, StorageProvider};
use views::{Blog, GraphVizDesktopView, Home};

mod views;

use graphvizm::Graphvizm;

const TAILWIND_CSS: Asset = asset!("/assets/tailwind.css");

#[derive(Debug, Clone, Routable, PartialEq)]
#[rustfmt::skip]
enum Route {
    #[layout(DesktopNavbar)]
    #[route("/")]
    Home {},
    #[route("/blog/:id")]
    Blog { id: i32 },
    /// Graphviz Route 
    #[route("/:key_path")]
    GraphVizDesktopView { key_path: String },
}

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

    // signal that will be saved to the context as None, until GViz is loaded
    let gviz_signal = use_signal::<Option<GVizProvider>>(|| None);
    let mut gviz_signal = use_context_provider(|| gviz_signal);

    // Create the Graphvizm instance once
    use_hook(|| {
        if let Ok(gviz) = Graphvizm::new() {
            // set the signal
            gviz_signal.set(Some(GVizProvider::new(
                graphvism_wrapper::GraphvizmWrapper::from(gviz),
            )));
        }
    });

    rsx! {
        // Global app resources
        document::Link { rel: "stylesheet", href: TAILWIND_CSS }

        div {
            class: "h-screen flex flex-col",
            Router::<Route> {}
        }
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
