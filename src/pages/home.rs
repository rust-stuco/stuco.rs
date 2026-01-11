use crate::Route;
use crate::pages::about::CURRENT_SEMESTER;
use dioxus::prelude::*;

const BACKGROUND: Asset = asset!("/assets/elixir-rust-cover.webp");

#[component]
pub fn Home() -> Element {
    rsx! {
        document::Title { "Intro to Rust Lang" }
        div {
            class: "-mt-16 h-[90vh] w-full bg-cover bg-center flex items-center justify-center shadow-2xl",
            style: format!("background-image: url('{}')", BACKGROUND),
            div { class: "bg-black/25 text-primary text-center px-8 py-6",
                h1 { class: "text-4xl sm:text-6xl font-bold", "Intro to Rust Lang" }
                p { class: "text-xl sm:text-2xl mt-2", "Spring 2026" }
            }
        }
        div { class: "max-w-prose mx-auto px-8 pt-16",
            h2 { class: "text-3xl text-center font-bold italic text-primary mb-6",
                "Intro to Rust Lang"
            }
            p {
                {
                    let names: Vec<_> = CURRENT_SEMESTER
                        .instructors
                        .iter()
                        .map(|(n, _)| *n)
                        .collect();
                    let instructors = if names.len() <= 1 {
                        names.join("")
                    } else {
                        format!(
                            "{}, and {}",
                            names[..names.len() - 1].join(", "),
                            names.last().unwrap(),
                        )
                    };

                    rsx! {
                        "Welcome to Intro to Rust Lang (98-008). The course will be offered in Spring 2026 by {instructors} at Carnegie Mellon University. Please see the "
                        Link { to: Route::About {}, class: "text-secondary", "about page" }
                        " for more!"
                    }
                }
            }
        }
    }
}
