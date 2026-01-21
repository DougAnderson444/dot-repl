//! A desktop application built with Dioxus that features routing and a navbar.
use dioxus::prelude::*;
use dot_repl_desktop::DesktopApp;
use ui::Navbar;
use views::{Blog, GraphVizDesktopView, Home};
mod views;

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
    rsx! {
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

        }

        Outlet::<Route> {}
    }
}
