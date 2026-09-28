//! A simplified version of `DotDisplay` for standalone, non-editable graph rendering.
use dioxus::prelude::*;

use crate::components::dot_display::{GraphvizSvg, SvgBuildConfig};
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
    // Track the last rendered DOT so prop updates (e.g. router drills that keep
    // this component mounted) actually re-render. A `use_memo` that only reads
    // `gviz_signal` will keep the first SVG forever.
    let mut last_dot = use_signal(String::new);
    let mut svg_signal = use_signal(|| None::<String>);
    let mut had_error = use_signal(|| false);

    let dot_changed = last_dot.read().as_str() != dot.as_str();
    let gviz_just_ready =
        gviz_signal.read().is_some() && svg_signal.read().is_none() && !dot.is_empty();

    if dot_changed || gviz_just_ready {
        if dot_changed {
            last_dot.set(dot.clone());
        }
        if let Some(gviz) = gviz_signal.read().as_ref() {
            if dot.is_empty() {
                svg_signal.set(None);
                had_error.set(false);
            } else {
                match gviz.render_dot(&dot) {
                    Ok(svg) => {
                        had_error.set(false);
                        svg_signal.set(Some(svg));
                    }
                    Err(_) => {
                        had_error.set(true);
                        svg_signal.set(None);
                    }
                }
            }
        }
    }

    if gviz_signal.read().is_none() {
        return rsx! {
            div {
                class: "{container_class} flex items-center justify-center",
                div {
                    class: "text-gray-400 p-2 text-center text-xs",
                    "Loading..."
                }
            }
        };
    }

    if *had_error.read() {
        return rsx! {
            div {
                class: "{container_class} flex items-center justify-center",
                div {
                    class: "text-red-500 p-2 text-center text-xs",
                    "Render Error"
                }
            }
        };
    }

    let svg = svg_signal.read().clone();
    if let Some(svg) = svg {
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
    } else {
        rsx! {
            div {
                class: "{container_class} flex items-center justify-center",
                div {
                    class: "text-gray-400 p-2 text-center text-xs",
                    "Rendering..."
                }
            }
        }
    }
}
