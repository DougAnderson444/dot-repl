//! Web-specific entry point
mod error;
pub use error::Error;

mod bindgen;
use bindgen::GViz;

mod storage;

use dioxus::prelude::*;

use ui::{GVizProvider, Navbar, StorageProvider};
use views::{Blog, GraphVizWebView, Home};

mod views;

use gloo_timers::future::sleep;
use std::time::Duration;
use wasm_bindgen::JsValue;
use web_sys::js_sys::Reflect;

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

const FAVICON: Asset = asset!("/assets/favicon.ico");
const TAILWIND_CSS: Asset = asset!("/assets/tailwind.css");

fn main() {
    dioxus::launch(App);
}

#[component]
fn App() -> Element {
    // Build cool things ✌️
    let storage = storage::WebStorage::new();
    let storage_provider = StorageProvider::new(storage);

    // provide storgae in context for all child elements
    use_context_provider(|| storage_provider);

    // signal that will be saved to the context as None, until GViz is loaded
    let gviz_signal = use_signal::<Option<GVizProvider>>(|| None);
    let mut gviz_signal = use_context_provider(|| gviz_signal);

    // Global rough_enabled state that persists across navigation
    let rough_enabled = use_signal(|| false);
    use_context_provider(|| rough_enabled);

    spawn(async move {
        // Wait for the viz_instance_promise to be loaded
        loop {
            if let Ok(val) = Reflect::get(
                &web_sys::window().unwrap(),
                &JsValue::from_str("viz_instance"),
            ) {
                if !val.is_undefined() {
                    break;
                }
            }
            sleep(Duration::from_millis(50)).await;
        }

        let gviz = GViz::new()
            .await
            .map_err(|e| {
                panic!("Failed to create GViz instance: {:?}", e);
            })
            .unwrap();

        let gviz_provider = GVizProvider::new(gviz);
        gviz_signal.set(Some(gviz_provider));
    });

    rsx! {
        // Global app resources
        document::Link { rel: "icon", href: FAVICON }
        document::Link { rel: "stylesheet", href: TAILWIND_CSS }
        document::Script {
            r#type: "module",
            r#"
            import {{ instance }} from 'https://cdn.jsdelivr.net/npm/@viz-js/viz@3.21.0/dist/viz.js';
            window.viz_instance = instance;
            "#
        }
        div {
            class: "h-screen flex flex-col",
            Router::<Route> {}
        }
    }
}

/// A web-specific Router around the shared `Navbar` component
/// which allows us to use the web-specific `Route` enum.
#[component]
fn WebNavbar() -> Element {
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
