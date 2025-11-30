//! Error overlay component for displaying render errors
use crate::error::{ErrorLevel, RenderError};
use dioxus::prelude::*;

#[component]
pub fn ErrorOverlay(errors: ReadSignal<Option<RenderError>>) -> Element {
    if let Some(render_error) = errors() {
        rsx! {
            div {
                class: "absolute top-0 right-0 m-4 max-w-md z-10",
                for (idx, error_info) in render_error.errors.iter().enumerate() {
                    div {
                        key: "{idx}",
                        class: match error_info.level {
                            ErrorLevel::Error => "bg-red-100 border-l-4 border-red-500 text-red-700 p-3 mb-2 rounded shadow-lg",
                            ErrorLevel::Warning => "bg-yellow-100 border-l-4 border-yellow-500 text-yellow-700 p-3 mb-2 rounded shadow-lg",
                            ErrorLevel::Info => "bg-blue-100 border-l-4 border-blue-500 text-blue-700 p-3 mb-2 rounded shadow-lg",
                        },
                        div {
                            class: "font-bold mb-1",
                            {match error_info.level {
                                ErrorLevel::Error => "Error",
                                ErrorLevel::Warning => "Warning",
                                ErrorLevel::Info => "Info",
                            }}
                            if let Some(line) = error_info.line {
                                span { class: "ml-2 font-normal", "at line {line}" }
                            }
                        }
                        div { class: "text-sm", "{error_info.message}" }
                    }
                }
            }
        }
    } else {
        rsx! {}
    }
}
