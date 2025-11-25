use base64::{engine::general_purpose::URL_SAFE, Engine as _};
use dioxus::prelude::*;

const HERO_CSS: Asset = asset!("/assets/styling/hero.css");
const HEADER_SVG: Asset = asset!("/assets/header.svg");

#[component]
pub fn Hero() -> Element {
    let next = r#"digraph { rankdir=TB; 
    Douglas [label="Douglas / Doug", URL="https://drawn.systems/"];
    Douglas -> Next; Next -> Cool; Cool -> Douglas; }"#;
    let dot = format!(
        r#"digraph {{ rankdir=LR; Apple [URL="/blog/69"]; Apple -> B; B -> C; C -> Apple; D -> Apple; D -> B; D -> C; }}"#,
    );
    // Encode the DOT string for safe URL usage
    let encoded_dot = URL_SAFE.encode(&dot);

    rsx! {
        document::Link { rel: "stylesheet", href: HERO_CSS }


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
