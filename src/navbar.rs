use crate::{links, pages::Route};
use dioxus::prelude::*;

const FERRIS: Asset = asset!("/assets/ferris.png");

pub(crate) const SHORT_LINKS: &[(&str, &str)] = &[
    ("/excusals", "Excusals"),
    ("/gradescope", "Gradescope"),
    ("/github", "GitHub"),
    ("/discord", "Discord"),
];

#[component]
pub(crate) fn Navbar() -> Element {
    let mut menu_open = use_signal(|| false);

    rsx! {
        div { class: "min-h-svh",
            div {
                id: "navbar",
                class: "sticky top-0 z-50 flex flex-col lg:flex-row lg:items-center justify-between px-6 py-4 text-primary",
                style: "background-color: var(--color-background);",

                div { class: "flex items-center justify-between w-full lg:w-auto shrink-0",
                    Link {
                        to: Route::Home {},
                        class: "flex items-center gap-3 text-lg font-black tracking-wider whitespace-nowrap",
                        img {
                            src: FERRIS,
                            alt: "Home",
                            class: "size-8 shrink-0 rounded-sm transition-transform hover:scale-110 active:scale-95",
                        }
                        span { "Intro to Rust Lang" }
                    }

                    button {
                        class: "lg:hidden text-2xl cursor-pointer",
                        onclick: move |_| menu_open.toggle(),
                        if menu_open() {
                            "✕"
                        } else {
                            "☰"
                        }
                    }
                }

                div {
                    class: "w-full text-sm tracking-wider whitespace-nowrap flex-col gap-4 pt-4 lg:pt-0 lg:w-auto lg:flex lg:flex-row lg:gap-6 lg:ml-auto",
                    class: if menu_open() { "flex" } else { "hidden" },

                    Link {
                        to: Route::Faq {},
                        onclick: move |_| menu_open.set(false),
                        "FAQ"
                    }
                    Link {
                        to: Route::Schedule {},
                        onclick: move |_| menu_open.set(false),
                        "Schedule"
                    }
                    Link { to: "/syllabus.pdf", new_tab: true, "Syllabus" }
                    for (path , label) in SHORT_LINKS {
                        Link {
                            key: "{path}",
                            to: links::destination(path),
                            new_tab: true,
                            "{label}"
                        }
                    }
                }
            }
            div { class: "pt-16", Outlet::<Route> {} }
            footer { class: "text-xs text-tertiary text-center py-8", "© 2022-2026 Rust StuCo" }
        }
    }
}
