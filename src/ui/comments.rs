use crate::Comment;
use dioxus::prelude::*;

use super::CommentsState;

#[component]
pub fn Comments() -> Element {
    let comments_state = use_context::<Signal<CommentsState>>();

    match comments_state() {
        CommentsState::Unset => rsx! {
            div {
                class: "mt-6",
                p { "Select a sotry to view comments" }
            }
        },
        CommentsState::Loading => rsx! {
            div {
                class: "mt-6",
                p { "Loading comments..." }
            }
        },
        CommentsState::Loaded(data) => rsx! {
            ul {
                for comment in data.comments {
                    StoryComment { comment }
                }
            }
        },
    }
}

#[component]
pub fn StoryComment(comment: Comment) -> Element {
    rsx! {
        section {
            article {
                class: "mt-4 text-gray-500 leading-7 tracking-wider text-gray-500 border-b border-gray-200",
                span {
                    "{comment.by} {comment.time} | next [-]"
                }
                div {
                    dangerous_inner_html: comment.text
                }
            }
        }
    }
}
