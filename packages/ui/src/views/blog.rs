use dioxus::prelude::*;

#[component]
pub fn BlogView<R>(id: i32, prev: R, next: R) -> Element
where
    R: Routable + Clone + PartialEq,
{
    rsx! {

        div {
            id: "blog",

            // Content
            h1 { "This is blog #{id}!" }
            p { "In blog #{id}, we show how the Dioxus router works and how URL parameters can be passed as props to our route components." }

            // Navigation links
            span {
                Link {
                    to: prev,
                    class: "mr-4 bg-green-200 rounded-md px-3 py-1 hover:bg-sky-300",
                    "Previous"
                }
            }
            span { " <---> " }
            span {
                class: "mr-4 bg-sky-300 rounded-md px-3 py-1 hover:bg-sky-500",
                Link {
                    to: next,
                    "Next"
                }
            }
        }
    }
}
