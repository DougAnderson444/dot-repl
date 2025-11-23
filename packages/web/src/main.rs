mod bindgen;
use bindgen::GViz;

use dioxus::prelude::*;

use ui::Navbar;
use views::{Blog, Home};

mod views;

#[derive(Debug, Clone, Routable, PartialEq)]
#[rustfmt::skip]
enum Route {
    #[layout(WebNavbar)]
    #[route("/")]
    Home {},
    #[route("/blog/:id")]
    Blog { id: i32 },
}

const FAVICON: Asset = asset!("/assets/favicon.ico");
const MAIN_CSS: Asset = asset!("/assets/main.css");

// global signal for the GViz context
static GVIZ_CONTEXT: GlobalSignal<Option<GViz>> = Signal::global(|| None);

fn main() {
    dioxus::launch(App);
}

#[component]
fn App() -> Element {
    // Build cool things ✌️

    // set the context for graph visualization implementation
    // we need to call GViz::new().await then set the context
    // so we'll need a use_future then set the signal in the context
    // when it's ready
    use gloo_timers::future::sleep;
    use std::time::Duration;
    use wasm_bindgen::JsValue;
    use web_sys::js_sys::Reflect;
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
        document::Link { rel: "stylesheet", href: MAIN_CSS }
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
