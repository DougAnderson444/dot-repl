use crate::Route;
use dioxus::prelude::*;
use dot_repl_ui::views::BlogView;

#[component]
pub fn Blog(id: i32) -> Element {
    rsx! {
        BlogView {
            id: id,
            prev: Route::Blog { id: id - 1 },
            next: Route::Blog { id: id + 1 },
        }
    }
}
