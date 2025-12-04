//! A simplified version of `DotDisplay` for standalone, non-editable graph rendering.
use dioxus::prelude::*;

use crate::components::dot_display::{GraphvizSvg, SvgBuildConfig};
use crate::error::Error as UiError;
use crate::GVizProvider;

/// Renders a DOT string into a self-contained, interactive SVG.
///
/// This is a lean component for displaying a graph visualization outside of an
/// editor, such as in a thumbnail preview or an embedded diagram in a post.
/// It reuses `GraphvizSvg` to ensure interactivity like clickable nodes is preserved.
#[component]
pub fn StandaloneDotDisplay(
    /// The DOT graph string to render.
    dot: String,
    /// Apply a "hand-drawn" aesthetic to the SVG. Defaults to `false`.
    #[props(default = false)]
    rough_style: bool,
    /// Make the SVG scale to fit its container. Defaults to `false`.
    #[props(default = false)]
    scale_to_fit: bool,
    /// Optional CSS classes to apply to the container `div`.
    #[props(default)]
    class: String,
) -> Element {
    let container_class = if class.is_empty() {
        "w-full h-full overflow-auto".to_string()
    } else {
        class
    };

    let gviz_signal = use_context::<Signal<Option<GVizProvider>>>();

    // Memoize the expensive rendering process. The closure captures the `gviz_signal`
    // and `dot` string, both of which have a `'static` lifetime, satisfying
    // the hook's requirements. The memo will re-run if `gviz_signal` changes.
    let svg_result = use_memo(move || {
        if let Some(gviz) = gviz_signal.read().as_ref() {
            gviz.render_dot(&dot)
        } else {
            Err(UiError::GvizNotInitialized)
        }
    });

    match svg_result() {
        Ok(svg) => {
            let config = SvgBuildConfig {
                rough_style,
                scale_to_fit,
                ..Default::default()
            };
            rsx! {
                div {
                    class: "{container_class}",
                    GraphvizSvg {
                        svg_text: svg,
                        config: config
                    }
                }
            }
        }
        Err(UiError::GvizNotInitialized) => {
            // Graphviz engine is not yet available.
            rsx! {
                div {
                    class: "{container_class} flex items-center justify-center",
                    div {
                        class: "text-gray-400 p-2 text-center text-xs",
                        "Loading..."
                    }
                }
            }
        }
        Err(_) => {
            // Handle cases where the DOT string is invalid.
            rsx! {
                div {
                    class: "{container_class} flex items-center justify-center",
                    div {
                        class: "text-red-500 p-2 text-center text-xs",
                        "Render Error"
                    }
                }
            }
        }
    }
}
