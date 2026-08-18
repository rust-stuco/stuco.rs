use super::Route;
use super::semesters::{CURRENT_SEMESTER, format_staff_names};
use dioxus::prelude::*;

const BACKGROUND: Asset = asset!("/assets/elixir-rust-cover.webp");

#[component]
pub(super) fn Home() -> Element {
    rsx! {
        document::Title { "Intro to Rust Lang" }
        div {
            class: "-mt-16 h-[50vh] w-full bg-cover bg-center flex items-center p-12 shadow-2xl",
            style: format!("background-image: url('{}')", BACKGROUND),

            div { class: "bg-black/60 text-primary px-12 py-10 max-w-2xl rounded-sm backdrop-blur-xs",
                h1 { class: "text-4xl sm:text-6xl font-bold tracking-wide border-b border-white/20 pb-4",
                    "Intro to Rust Lang"
                }
                p { class: "text-xl sm:text-2xl mt-4 font-semibold text-white/90",
                    "CMU 98-008 - {CURRENT_SEMESTER.name}"
                }
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
