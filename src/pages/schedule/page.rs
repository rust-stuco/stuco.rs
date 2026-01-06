use crate::pages::schedule::data::{Homework, Week, load_weeks};
use dioxus::prelude::*;
use std::sync::LazyLock;

static WEEKS: LazyLock<Vec<Week>> = LazyLock::new(load_weeks);

#[component]
pub fn Schedule() -> Element {
    rsx! {
        document::Title { "Schedule - Rust StuCo" }
        div { class: "max-w-4xl mx-auto px-8 pt-16",
            h1 { class: "text-3xl font-bold italic text-primary mb-6 text-center",
                "Schedule"
            }
            table { class: "w-full border-collapse",
                thead {
                    tr { class: "border-b border-tertiary",
                        th { class: "text-left p-2", "Week" }
                        th { class: "text-left p-2", "Topics" }
                        th { class: "text-left p-2", "Slides" }
                        th { class: "text-left p-2", "Homework" }
                    }
                }
                tbody {
                    for (i , week) in WEEKS.iter().enumerate() {
                        tr { class: "border-b border-tertiary/50",
                            td { class: "p-2 align-top", "{i + 1}" }
                            td { class: "p-2 align-top",
                                span { class: "font-semibold", "{week.title}" }
                            }
                            td { class: "p-2 align-top",
                                SlideLinks { slides: &week.slides }
                            }
                            td { class: "p-2 align-top",
                                if let Some(hw) = &week.homework {
                                    HomeworkLinks { homework: hw }
                                }
                                if let Some(hw_ec) = &week.homework_ec {
                                    div { class: "mt-1",
                                        span { class: "text-secondary text-sm", "EC: " }
                                        HomeworkLinks { homework: hw_ec }
                                    }
                                }
                            }
                        }
                    }
                }
            }
        }
    }
}

#[component]
fn SlideLinks(slides: &'static str) -> Element {
    let base = "https://raw.githubusercontent.com/rust-stuco/lectures/main";
    let name = slides.split('_').skip(1).collect::<Vec<_>>().join("_");

    rsx! {
        div { class: "flex gap-2 text-sm",
            a {
                class: "text-primary hover:underline",
                href: "{base}/{slides}/{name}-light.pdf",
                "light"
            }
            span { class: "text-secondary", "/" }
            a {
                class: "text-primary hover:underline",
                href: "{base}/{slides}/{name}-dark.pdf",
                "dark"
            }
        }
    }
}

#[component]
fn HomeworkLinks(homework: &'static Homework) -> Element {
    if let Some(slug) = &homework.slug {
        rsx! {
            span { "{homework.name} " }
            span { class: "text-sm",
                "("
                a {
                    class: "text-primary hover:underline",
                    href: "/hw/{slug}/{slug}.zip",
                    "handout"
                }
                " / "
                a {
                    class: "text-primary hover:underline",
                    href: "/hw/{slug}/doc/{slug}/index.html",
                    "writeup"
                }
                ")"
            }
        }
    } else {
        rsx! {
            span { "{homework.name}" }
        }
    }
}
