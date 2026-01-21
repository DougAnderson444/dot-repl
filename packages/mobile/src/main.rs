use dioxus::prelude::*;
use dot_repl_mobile::MobileApp;
use ui::Navbar;

mod views;
use views::{Blog, Home};

#[derive(Debug, Clone, Routable, PartialEq)]
#[rustfmt::skip]
enum Route {
    #[layout(MobileNavbar)]
    #[route("/")]
    Home {},
    #[route("/blog/:id")]
    Blog { id: i32 },
}

fn main() {
    dioxus::launch(App);
}

#[component]
fn App() -> Element {
    rsx! {
        MobileApp {
            Router::<Route> {}
        }
    }
}

/// A mobile-specific Router around the shared `Navbar` component
/// which allows us to use the mobile-specific `Route` enum.
#[component]
fn MobileNavbar() -> Element {
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
