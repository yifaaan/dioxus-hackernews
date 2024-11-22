use crate::{StoryData, StoryItem};
use dioxus::prelude::*;

#[component]
pub fn StoryComment(data: StoryItem) -> Element {
    rsx! {
        section {
            h1 { class: "font-bold text-2xl", "We need UI/UX designer" }
            article { class: "mt-8 text-gray-500 leading-7 tracking-wider",
                p { "Hi Akhil," }
                p {
                    "Design and develop enterprise-facing UI and consumer-facing UI as well as\n            REST API\n            backends.Work with\n            Product Managers and User Experience designers to create an appealing user experience for\n            desktop\n            web and\n            mobile web."
                }
                footer { class: "mt-12",
                    p { "Thanks & Regards," }
                    p { "Alexandar" }
                }
            }
        }
    }
}
