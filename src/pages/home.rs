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
        div { class: "mx-auto px-8 pt-8 flex flex-col md:flex-row gap-4",
            div { class: "flex-1 p-4",
                Link {
                    to: "https://rust-lang.org/",
                    class: "text-secondary hover:underline",
                    "Rust"
                }
                " is a systems programming language renowned for its memory-safe performance, rich type system, and developer-first tooling. Backed by rising usage in the tech industry, Rust has been consistently voted as the "
                Link {
                    to: "https://blog.rust-lang.org/2025/12/19/what-do-people-love-about-rust/",
                    class: "text-secondary hover:underline",
                    "most-loved language"
                }
                " among developers globally. This course provides a low-stress introduction to Rust, starting with its type system and unique ownership model. Students will then explore advanced Rust features such as lifetimes, closures, and smart pointers, before moving onto advanced Rust patterns such as parallelism, concurrency, and unsafe. "
                Link {
                    to: "https://www.cs.cmu.edu/~15122/",
                    class: "text-secondary hover:underline",
                    "15-122"
                }
                " or equivalent low-level programming experience is expected."
            }
            div { class: "flex-1 p-4",
                h2 { class: "text-3xl font-bold text-primary mb-4", "Course Info" }
                table { class: "w-full border-collapse text-left",
                    tbody {
                        tr { class: "border-b border-white/10",
                            td { class: "py-2 pr-4 font-bold text-primary", "Instructors" }
                            td { class: "py-2", "{format_staff_names(CURRENT_SEMESTER.instructors)}" }
                        }
                        tr { class: "border-b border-white/10",
                            td { class: "py-2 pr-4 font-bold text-primary", "TA" }
                            td { class: "py-2", "{format_staff_names(CURRENT_SEMESTER.tas)}" }
                        }
                        tr { class: "border-b border-white/10",
                            td { class: "py-2 pr-4 font-bold text-primary", "Lectures" }
                            td { class: "py-2",
                                "Thursdays, 7-7:50 PM in "
                                Link {
                                    to: "https://maps.scottylabs.org/GHC-4307",
                                    class: "text-secondary hover:underline",
                                    "GHC 4307"
                                }
                            }
                        }
                        tr {
                            td { class: "py-2 pr-4 font-bold text-primary", "Office Hours" }
                            td { class: "py-2", "TBD" }
                        }
                    }
                }
            }
        }
    }
}
