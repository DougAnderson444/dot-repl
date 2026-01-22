//! Web-specific entry point
mod error;
pub use error::Error;

mod bindgen;
use bindgen::GViz;

mod storage;

use dioxus::prelude::*;

use dot_repl_ui::{GVizProvider, StorageProvider};

use gloo_timers::future::sleep;
use std::time::Duration;
use wasm_bindgen::JsValue;
use web_sys::js_sys::Reflect;

const FAVICON: Asset = asset!("/assets/favicon.ico");

#[component]
pub fn WebApp(children: Element) -> Element {
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
