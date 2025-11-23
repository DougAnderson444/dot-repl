use dioxus::prelude::*;

const BLOG_CSS: Asset = asset!("/assets/styling/blog.css");

#[component]
pub fn BlogView<R>(id: i32, prev: R, next: R) -> Element
where
    R: Routable + Clone + PartialEq,
{
    rsx! {
        document::Link { rel: "stylesheet", href: BLOG_CSS}

        div {
            id: "blog",

            // Content
            h1 { "This is blog #{id}!" }
            p { "In blog #{id}, we show how the Dioxus router works and how URL parameters can be passed as props to our route components." }

            // Navigation links
            Link {
                to: prev,
                "Previous"
            }
            span { " <---> " }
            Link {
                to: next,
                "Next"
            }
        }
    }
}
