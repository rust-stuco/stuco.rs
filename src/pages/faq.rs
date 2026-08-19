use super::semesters::{PREVIOUS_SEMESTERS, format_staff_names};
use dioxus::prelude::*;

#[component]
pub(super) fn Faq() -> Element {
    let items = [
        (
            "Who can take this course?",
            rsx! {
                "Any Carnegie Mellon student who has low-level programming experience at the level of "
                a {
                    href: "https://www.cs.cmu.edu/~15122/",
                    class: "text-secondary",
                    "15-122"
                }
                " can take this course. If you are not a CMU student, unfortunately you cannot register for this course, but you are welcome to follow along using our lecture slides and homework publicly posted on this website!"
            },
        ),
        (
            "I'm waitlisted! What should I do?",
            rsx! { "There is a lot of movement in the first week of the semester, so chances are you can get in eventually! If you are waitlisted, talk to us after lecture and we can bump you up." },
        ),
        (
            "Adding this course would exceed my unit cap. Can I still enroll?",
            rsx! { "Yes! StuCos do not count toward your unit cap." },
        ),
        (
            "What is the time commitment for this course?",
            rsx! { "We anticipate that students will spend 2-3 hours per week on this course: 1 hour for lecture, plus 1-2 hours for homework and/or review. If you find yourself spending more time than this, please let us know!" },
        ),
        (
            "What is the attendance policy?",
            rsx! {
                "Attendance is required! It is StuCo policy that students who have more than 2 unexcused absences will automatically fail the course. Additionally, every 3 excused absences count as 1 unexcused absence. Please read the "
                a { href: "/syllabus.pdf", class: "text-secondary", "syllabus" }
                " for more information."
            },
        ),
        (
            "What is the homework policy?",
            rsx! {
                "Homework will be released on the Schedule page and submitted on Gradescope. Students must attain at least 1000 points through homework submissions, with up to 4 allowed late submissions. Please read the "
                a { href: "/syllabus.pdf", class: "text-secondary", "syllabus" }
                " for more information."
            },
        ),
        (
            "What was this course like in past semesters?",
            rsx! {
                "Intro to Rust Lang was first taught in 2022! Check out our past offerings:"
                ul { class: "list-disc ml-8 mt-2 marker:text-foreground/50",
                    for semester in PREVIOUS_SEMESTERS {
                        {
                            let instructors = format_staff_names(semester.instructors);
                            let ta_label = if semester.tas.len() == 1 { "TA" } else { "TAs" };
                            let tas = format_staff_names(semester.tas);

                            rsx! {
                                li {
                                    a {
                                        class: "text-secondary",
                                        href: "{semester.link}",
                                        target: "_blank",
                                        rel: "noopener noreferrer",
                                        "{semester.name}"
                                    }
                                    ": {instructors}"
                                    if !semester.tas.is_empty() {
                                        "; {ta_label}: {tas}"
                                    }
                                }
                            }
                        }
                    }
                }
            },
        ),
    ];

    rsx! {
        document::Title { "FAQ - Rust StuCo" }
        div { class: "max-w-4xl mx-auto px-8",
            h1 { class: "text-6xl font-bold text-primary mb-12 text-center", "FAQ" }

            div { class: "space-y-8",
                for (i , (question , answer)) in items.into_iter().enumerate() {
                    div { key: "{question}", class: "space-y-2",
                        h2 { class: "text-2xl font-semibold", "{i + 1}. {question}" }
                        p { class: "leading-relaxed", {answer} }
                    }
                }
            }
        }
    }
}
