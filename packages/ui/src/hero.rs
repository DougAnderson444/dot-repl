use crate::components::StandaloneDotDisplay;
use crate::storage::KITCHEN_SINK_STORAGE_KEY;
use dioxus::prelude::*;

static KITCHEN_SINK_DOT: &str = include_str!("../assets/dot/kitchen_sink.dot");

#[component]
pub fn Hero() -> Element {
    let encoded_dot = url_escape::encode_fragment(KITCHEN_SINK_STORAGE_KEY);

    rsx! {
        div {
            id: "hero",
            // img { src: HEADER_SVG, id: "header" }
            Link {
                to: "/{encoded_dot}",
                div {
                    class: "mt-8 m-2 p-4 border border-gray-200 rounded-lg shadow-md bg-white
                            w-full max-w-sm h-auto transition-transform transform hover:scale-105 cursor-pointer",
                    h2 {
                        class: "text-lg font-semibold text-gray-700 mb-2 text-center",
                        "🖼️ The Kitchen Sink Graph"
                    },
                    div {
                        class: "w-full h-48", // Fixed size for the thumbnail container
                        StandaloneDotDisplay {
                            dot: KITCHEN_SINK_DOT.to_string(),
                            scale_to_fit: true,
                        }
                    }
                }
            }
        }
    }
}

