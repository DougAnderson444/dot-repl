//! A code editor component with line highlighting support
use dioxus::prelude::*;

/// A code editor that supports highlighting specific lines
/// Uses CSS background-image with linear-gradient for line highlighting
#[component]
pub fn CodeEditor(
    value: String,
    oninput: EventHandler<String>,
    error_lines: Vec<u32>,
    placeholder: String,
) -> Element {
    // Build a CSS background-image that highlights error lines
    let background_style = if error_lines.is_empty() {
        "background: white;".to_string()
    } else {
        // Calculate how many lines we need (based on content + some buffer)
        let line_count = value.lines().count().max(20) + 10;
        let line_height = 1.5; // em
        
        // Build gradient stops for each line
        let mut gradient_parts = vec![];
        
        for line_num in 1..=line_count {
            let line_u32 = line_num as u32;
            let start = (line_num - 1) as f32 * line_height;
            let end = line_num as f32 * line_height;
            
            if error_lines.contains(&line_u32) {
                // Highlighted line - yellow with opacity
                gradient_parts.push(format!("rgba(250, 204, 21, 0.3) {}em", start));
                gradient_parts.push(format!("rgba(250, 204, 21, 0.3) {}em", end));
            } else {
                // Transparent line
                gradient_parts.push(format!("transparent {}em", start));
                gradient_parts.push(format!("transparent {}em", end));
            }
        }
        
        format!(
            "background: linear-gradient(to bottom, {});",
            gradient_parts.join(", ")
        )
    };

    rsx! {
        div {
            class: "flex-1 relative",
            
            // Error indicators in gutter
            if !error_lines.is_empty() {
                div {
                    class: "absolute left-0 top-0 bottom-0 w-8 bg-gray-100 border-r border-gray-300 pointer-events-none z-10",
                    for line_num in error_lines.iter() {
                        div {
                            key: "{line_num}",
                            class: "absolute left-1 w-2 h-2 bg-red-500 rounded-full",
                            style: "top: calc(({line_num} - 1) * 1.5em + 1rem);",
                            title: "Error on line {line_num}"
                        }
                    }
                }
            }
            
            // Textarea with line highlighting
            textarea {
                class: "w-full h-full font-mono text-sm p-4 pl-12 border-none outline-none resize-none overflow-auto",
                style: "{background_style} line-height: 1.5em; tab-size: 4;",
                value: "{value}",
                placeholder: "{placeholder}",
                spellcheck: false,
                
                oninput: move |e| {
                    oninput.call(e.value());
                },
            }
        }
    }
}
