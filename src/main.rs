#![allow(non_snake_case)]

use dioxus::prelude::*;
use dioxus_logger::tracing::{info, Level};

#[derive(Clone, Routable, Debug, PartialEq)]
enum Route {
    #[route("/")]
    Home {},
    #[route("/blog/:id")]
    Blog { id: i32 },
    #[route("/:path")]
    NotFound { path: String },
}

fn main() {
    // Init logger
    dioxus_logger::init(Level::INFO).expect("failed to init logger");
    info!("starting app");
    launch(App);
}

fn App() -> Element {
    rsx! {
        Router::<Route> {}
    }
}

#[component]
fn Blog(id: i32) -> Element {
    rsx! {
        Link { to: Route::Home {}, "Go to counter" }
        div {
            class: "flex flex-col items-center justify-center h-screen",
            h1 {
                class: "text-3xl p-4",
                "Blog post {id}"
            }
        }
    }
}

#[component]
fn Home() -> Element {
    let mut count = use_signal(|| 0);

    rsx! {
        Link {
            to: Route::Blog {
                id: count()
            },
            "Go to blog"
        }
        div {
            class: "flex flex-col items-center justify-center h-screen",
            h1 {
                class: "text-3xl font-bold mb-4",
                "High-Five counter: {count}" }
            button {
                class: "py-2.5 px-5 me-2 mb-2 text-sm font-medium text-gray-900 focus:outline-none bg-white rounded-lg border border-gray-200 hover:bg-gray-100 hover:text-blue-700 focus:z-10 focus:ring-4 focus:ring-gray-100 dark:focus:ring-gray-700 dark:bg-gray-800 dark:text-gray-400 dark:border-gray-600 dark:hover:text-white dark:hover:bg-gray-700",
                onclick: move |_| count += 1, "Up high!" }
            button {
                class: "focus:outline-none text-white bg-yellow-400 hover:bg-yellow-500 focus:ring-4 focus:ring-yellow-300 font-medium rounded-lg text-sm px-5 py-2.5 me-2 mb-2 dark:focus:ring-yellow-900",
                onclick: move |_| count -= 1, "Down low!" }
        }
    }
}

#[component]
fn NotFound(path: String) -> Element {
    rsx! {
        div {
            class: "flex flex-col items-center justify-center h-screen",
            h1 {
                class: "text-3xl p-4",
                "404 Not Found"
            }
        }
    }
}
