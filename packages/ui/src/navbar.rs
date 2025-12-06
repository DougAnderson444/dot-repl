use dioxus::prelude::*;

// const NAVBAR_CSS: Asset = asset!("/assets/styling/navbar.css");

#[component]
pub fn Navbar(children: Element) -> Element {
    rsx! {
        // document::Link { rel: "stylesheet", href: NAVBAR_CSS }

        div {
            id: "navbar",
            class: "w-full h-12 bg-sky-500 text-neutral-200 flex items-center px-4 space-x-4",
            {children}
        }
    }
}
