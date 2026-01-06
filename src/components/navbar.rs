use crate::Route;
use dioxus::prelude::*;

const FERRIS: Asset = asset!("/assets/ferris.png");

#[component]
pub fn Navbar() -> Element {
    let mut menu_open = use_signal(|| false);

    rsx! {
        div { class: "min-h-svh",
            div {
                id: "navbar",
                class: "absolute top-0 left-0 right-0 flex flex-wrap items-center gap-x-12 gap-y-4 px-8 py-4 bg-black/25 text-primary",
                Link { to: Route::Home {},
                    img {
                        src: FERRIS,
                        alt: "Home",
                        class: "size-8 rounded-sm transition-transform hover:scale-110 active:scale-95",
                    }
                }
                button {
                    class: "sm:hidden ml-auto text-2xl",
                    onclick: move |_| menu_open.toggle(),
                    if menu_open() {
                        "✕"
                    } else {
                        "☰"
                    }
                }
                div {
                    class: "w-full text-sm font-bold flex-col gap-4 sm:w-auto sm:flex sm:flex-row sm:gap-12 sm:ml-0",
                    class: if menu_open() { "flex" } else { "hidden" },

                    Link {
                        to: Route::About {},
                        onclick: move |_| menu_open.set(false),
                        "About"
                    }
                    Link {
                        to: Route::Resources {},
                        onclick: move |_| menu_open.set(false),
                        "Resources"
                    }
                    Link {
                        to: Route::Schedule {},
                        onclick: move |_| menu_open.set(false),
                        "Schedule"
                    }

                    Link { to: "/syllabus.pdf", new_tab: true, "Syllabus" }
                    Link { to: "https://github.com/rust-stuco", new_tab: true, "GitHub" }
                    Link { to: "https://discord.gg/styjqeEdBG", new_tab: true, "Discord" }
                }
            }
            div { class: "pt-16", Outlet::<Route> {} }
            footer { class: "text-xs text-tertiary text-center py-8", "© 2023-2026 Rust StuCo" }
        }
    }
}
