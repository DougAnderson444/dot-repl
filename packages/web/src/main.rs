mod bindgen;
use bindgen::GViz;

use dioxus::{prelude::*, router::Navigator};

use ui::Navbar;
use views::{Blog, GraphvizView, Home};

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
    #[route("/graphviz/:encoded_dot")]
    GraphvizView { encoded_dot: String },
}

const FAVICON: Asset = asset!("/assets/favicon.ico");
const TAILWIND_CSS: Asset = asset!("/assets/tailwind.css");

// global signal for the GViz context
static GVIZ_CONTEXT: GlobalSignal<Option<GViz>> = Signal::global(|| None);

fn main() {
    dioxus::launch(App);
}

#[component]
fn App() -> Element {
    // Build cool things ✌️
    spawn(async {
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
        *GVIZ_CONTEXT.write() = Some(gviz);
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
        Router::<Route> {}
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
