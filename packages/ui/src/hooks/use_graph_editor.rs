//! A "headless" hook containing the logic for the GraphEditor component.
use dioxus::prelude::*;
use crate::error::RenderError;

/// A headless hook containing the logic for the GraphEditor component.
///
/// This hook encapsulates the state management for the graph editor,
/// allowing the consumer to build a custom UI around it.
#[must_use]
pub fn use_graph_editor_logic() -> GraphEditorLogic {
    let collapsed = use_signal(|| false);
    let render_errors = use_signal(|| None::<crate::error::RenderError>);

    // Calculate which lines have errors
    let error_lines = use_memo(move || {
        render_errors()
            .map(|err| {
                err.errors
                    .iter()
                    .filter_map(|e| e.line)
                    .collect::<Vec<u32>>()
            })
            .unwrap_or_default()
    });

    GraphEditorLogic {
        collapsed,
        render_errors,
        error_lines,
    }
}

/// The state and signals returned by the `use_graph_editor_logic` hook.
pub struct GraphEditorLogic {
    /// A signal that determines whether the editor panel is collapsed.
    pub collapsed: Signal<bool>,
    /// A signal that holds any rendering errors.
    pub render_errors: Signal<Option<RenderError>>,
    /// A memoized list of lines that have errors.
    pub error_lines: Memo<Vec<u32>>,
}
