use crate::{Echo, Hero};
use dioxus::prelude::*;

#[component]
pub fn Home() -> Element {
    rsx! {
        Hero {}
        Echo {}
    }
}
