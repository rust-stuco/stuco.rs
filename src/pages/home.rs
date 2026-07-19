use super::Route;
use super::semesters::{CURRENT_SEMESTER, format_staff_names};
use dioxus::prelude::*;

const BACKGROUND: Asset = asset!("/assets/elixir-rust-cover.webp");

#[component]
pub(super) fn Home() -> Element {
    rsx! {
        document::Title { "Intro to Rust Lang" }
        div {
            class: "-mt-16 h-[90vh] w-full bg-cover bg-center flex items-center justify-center shadow-2xl",
            style: format!("background-image: url('{}')", BACKGROUND),
            div { class: "bg-black/25 text-primary text-center px-8 py-6",
                h1 { class: "text-4xl sm:text-6xl font-bold", "Intro to Rust Lang" }
                p { class: "text-xl sm:text-2xl mt-2", "{CURRENT_SEMESTER.name}" }
            }
        }
        div { class: "max-w-prose mx-auto px-8 pt-16",
            h2 { class: "text-3xl text-center font-bold italic text-primary mb-6",
                "Intro to Rust Lang"
            }
            p {
                {
                    let instructors = format_staff_names(CURRENT_SEMESTER.instructors);
                    let semester = CURRENT_SEMESTER.name;

                    rsx! {
                        "Welcome to Intro to Rust Lang (98-008). The course will be offered in {semester} by {instructors} at Carnegie Mellon University. Please see the "
                        Link { to: Route::About {}, class: "text-secondary", "about page" }
                        " for more!"
                    }
                }
            }
        }
    }
}
