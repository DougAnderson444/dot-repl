//! A desktop application built with Dioxus that features routing and a navbar.
use dioxus::prelude::*;
use dot_repl_desktop::DesktopApp;
use dot_repl_ui::Navbar;
use views::{Blog, GraphVizDesktopView, Home};
mod views;

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
    // Global rough_enabled state that persists across navigation
    let rough_enabled = use_signal(|| false);
    use_context_provider(|| rough_enabled);

    rsx! {
        document::Link { rel: "stylesheet", href: TAILWIND_CSS }
        DesktopApp {
            path: "dot_files".to_string(),
            div {
                class: "h-screen flex flex-col",
                Router::<Route> {}
            }
        }
    }
}

/// A desktop-specific Router around the shared `Navbar` component
/// which allows us to use the desktop-specific `Route` enum.
#[component]
fn DesktopNavbar() -> Element {
    let navigator = use_navigator();
    let route = use_route::<Route>();
    let mut rough_enabled = use_context::<Signal<bool>>();

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
            button {
                class: "font-sans px-4 py-1 text-neutral-200 bg-sky-600 hover:bg-sky-700 rounded-md",
                onclick: move |_| navigator.go_back(),
                "←"
            }
            button {
                class: "font-sans px-4 py-1 text-neutral-200 bg-sky-600 hover:bg-sky-700 rounded-md",
                onclick: move |_| navigator.go_forward(),
                "→"
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
            if let Route::GraphVizDesktopView { key_path } = route {
                div {
                    class: "m-4 font-mono text-xs text-gray-500",
                    "/{key_path}"
                }
            }
        }

        Outlet::<Route> {}
    }
}
