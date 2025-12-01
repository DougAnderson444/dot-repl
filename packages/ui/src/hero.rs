use crate::storage::KITCHEN_SINK_STORAGE_KEY;
use dioxus::prelude::*;

#[component]
pub fn Hero() -> Element {
    let encoded_dot = url_escape::encode_fragment(KITCHEN_SINK_STORAGE_KEY);

    rsx! {
        div {
            id: "hero",
            // img { src: HEADER_SVG, id: "header" }
            div { id: "links",
                Link {
                    to: "/graphviz/{encoded_dot}",
                    "🖼️ The Kitchen Sink Graph"
                }
            }
        }
    }
}
