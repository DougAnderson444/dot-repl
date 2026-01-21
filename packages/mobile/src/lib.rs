use dioxus::prelude::*;

use ui::Navbar;


const MAIN_CSS: Asset = asset!("/assets/main.css");


#[component]
pub fn MobileApp(children: Element) -> Element {
    // Build cool things ✌️

    rsx! {
        // Global app resources
        document::Link { rel: "stylesheet", href: MAIN_CSS }

        {children}
    }
}

