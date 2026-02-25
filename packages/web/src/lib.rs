//! Web-specific entry point
mod error;
pub use error::Error;

mod bindgen;
use bindgen::GViz;

mod storage;
pub use storage::WebStorage;
pub mod asset_loader;

use dioxus::logger::tracing;
use dioxus::prelude::*;

use dot_repl_ui::{GVizProvider, PreloadComplete, StorageProvider};

use crate::asset_loader::preload_dot_files;
use gloo_timers::future::sleep;
use std::time::Duration;
use wasm_bindgen::JsValue;
use web_sys::js_sys::Reflect;

#[component]
pub fn WebApp(children: Element) -> Element {
    // Build cool things ✌️
    let storage = storage::WebStorage::new();
    let storage_provider = StorageProvider::new(storage.clone());

    // provide storgae in context for all child elements
    use_context_provider(|| storage_provider);

    // signal that will be saved to the context as None, until GViz is loaded
    let gviz_signal = use_signal::<Option<GVizProvider>>(|| None);
    let mut gviz_signal = use_context_provider(|| gviz_signal);

    // Global rough_enabled state that persists across navigation
    let rough_enabled = use_signal(|| false);
    use_context_provider(|| rough_enabled);

    // Flips to true once the host app finishes preloading static DOT assets.
    // GraphView subscribes to this so it re-reads storage after preload completes.
    let mut preload_complete: PreloadComplete = use_signal(|| false);
    use_context_provider(|| preload_complete);

    // Preload DOT assets from the server on every cold page load.
    // Uses a server-hash sentinel in LocalStorage to decide whether to
    // overwrite existing data: if the server has a new version of a file it
    // gets written; if not, the user's local copy (possibly hand-edited) is
    // kept intact.  In-session edits are always safe — they live in signals.
    let preload_storage = storage.clone();
    spawn(async move {
        match preload_dot_files(&preload_storage, "/assets/dots").await {
            Ok(n) => tracing::info!("Preloaded {} DOT file(s) from server", n),
            Err(e) => tracing::warn!("DOT preload failed: {}", e),
        }
        // Signal to GraphView (and any other subscriber) that storage is ready.
        preload_complete.set(true);
    });

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
        document::Link {
            rel: "stylesheet",
            href: "https://fonts.googleapis.com/css2?family=Architects+Daughter&display=swap"
        }
        document::Link {
            rel: "stylesheet",
            href: "https://fonts.googleapis.com/css2?family=Noto+Sans+Symbols+2&display=swap"
        }
        document::Script {
            r#type: "module",
            r#"
            import {{ instance }} from 'https://cdn.jsdelivr.net/npm/@viz-js/viz@3.21.0/dist/viz.js';
            window.viz_instance = instance;
            "#
        }
        {children}
    }
}
