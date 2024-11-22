mod story_comment;
mod story_item;
use dioxus::prelude::*;
use story_item::StoryItem;

use crate::api::get_top_stories;
pub fn App() -> Element {
    rsx! {
        // Router::<Route> {}
        main { class: "flex w-full h-full shadow-lg rounded-3xl",
        // 左侧文章列表
        section { class: "flex flex-col pt-3 w-4/12 bg-gray-50 h-full overflow-y-scroll",
            Stories {}
        }
        section { class: "w-8/12 px-4 flex flex-col bg-white rounded-r-3xl",
            ul {}
        }
    }
    }
}

#[component]
fn Stories() -> Element {
    let stories = use_resource(move || get_top_stories(20));
    match &*stories.read_unchecked() {
        Some(Ok(stories)) => rsx! {
            ul {
                // class: "mt-6",
                for story in stories {
                    StoryItem { story: story.clone()}
                }
            }
        },
        Some(Err(err)) => rsx! {
            div {
                class: "mt-6 text-red-500",
                p { "Failed to fetch stories" }
                p { "{err}" }
            }
        },
        None => rsx! {
            div {
                class: "mt-6",
                p { "Loading stories..." }
            }
        },
    }
}
