use crate::storage::STORAGE_KEY;
use base64::{engine::general_purpose::URL_SAFE, Engine as _};
use dioxus::prelude::*;

// const HERO_CSS: Asset = asset!("/assets/styling/hero.css");
const HEADER_SVG: Asset = asset!("/assets/header.svg");
// const KITCHEN_SINK_DOT: &str = include_str!("../assets/dot/kitchen_sink.dot");

#[component]
pub fn Hero() -> Element {
    // Encode the DOT string for safe URL usage
    let encoded_dot = URL_SAFE.encode(STORAGE_KEY);

    rsx! {
        // document::Link { rel: "stylesheet", href: HERO_CSS }


        div {
            id: "hero",
            img { src: HEADER_SVG, id: "header" }
            div { id: "links",
                a { href: "https://dioxuslabs.com/learn/0.7/", "📚 Learn Dioxus" }
                // link to /graphviz route
                // a { href: "/graphviz/{encoded_dot}", "🖼️ Graphviz Demo" }
                Link {
                    to: "/graphviz/{encoded_dot}",
                    "🖼️ The Graphviz Demo"
                }
            }
        }
    }
}
