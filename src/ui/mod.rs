mod comments;
mod stories;
use crate::StoryData;
use dioxus::prelude::*;

#[derive(Debug, Clone)]
pub enum CommentsState {
    Unset,
    Loading,
    Loaded(StoryData),
}

pub fn App() -> Element {
    use_context_provider(|| Signal::new(CommentsState::Unset));
    rsx! {
        // Router::<Route> {}
        main { class: "flex w-full h-full shadow-lg rounded-3xl",
        // 左侧文章列表
        section { class: "flex flex-col pt-3 w-4/12 bg-gray-50 h-full overflow-y-scroll",
            stories::Stories {}
        }
        section { class: "w-8/12 px-4 flex flex-col bg-white rounded-r-3xl",
            comments::Comments {}
        }
    }
    }
}
