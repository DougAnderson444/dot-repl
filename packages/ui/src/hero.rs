use base64::{prelude::BASE64_STANDARD, Engine as _};
use dioxus::prelude::*;

const HERO_CSS: Asset = asset!("/assets/styling/hero.css");
const HEADER_SVG: Asset = asset!("/assets/header.svg");

#[component]
pub fn Hero() -> Element {
    // let dot = r#"digraph { A -> B; B -> C; C -> A; }"#;
    let next = r#"digraph { rankdir=LR; A -> B; B -> C; C -> A; D -> A; D -> B; D -> C; }"#;
    let dot = format!(
        r#"digraph {{ rankdir=LR; A [URL="/graphviz/{next}"]; A -> B; B -> C; C -> A; D -> A; D -> B; D -> C; }}"#,
        next = BASE64_STANDARD.encode(next)
    );
    // Encode the DOT string for safe URL usage
    let encoded_dot = BASE64_STANDARD.encode(&dot);

    rsx! {
        document::Link { rel: "stylesheet", href: HERO_CSS }


        div {
            id: "hero",
            img { src: HEADER_SVG, id: "header" }
            div { id: "links",
                a { href: "https://dioxuslabs.com/learn/0.7/", "📚 Learn Dioxus" }
                // link to /graphviz route
                a { href: "/graphviz/{encoded_dot}", "🖼️ Graphviz Demo" }
            }
        }
    }
}
