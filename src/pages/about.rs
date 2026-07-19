use super::semesters::{
    CURRENT_SEMESTER, PREVIOUS_SEMESTERS, Semester, StaffMember, format_staff_names,
};
use dioxus::prelude::*;

#[component]
pub(super) fn About() -> Element {
    rsx! {
        document::Title { "About - Rust StuCo" }
        div { class: "max-w-prose mx-auto px-8 pt-16",
            h1 { class: "text-3xl font-bold italic text-primary mb-6 text-center",
                "About"
            }
            p { class: "mb-6",
                "Intro to Rust Lang (98-008) is intended to provide an approachable introduction to the Rust programming language. We don't expect any previous exposure to the language, but we expect some previous background in C, equivalent to completing the course 15-122."
            }
            p { class: "mb-6",
                "There will be weekly homeworks that are intended to take around an hour each week so that you have a chance to practice what we have taught in lecture. See the "
                a { href: "/syllabus.pdf", class: "text-secondary", "syllabus" }
                " for more information!"
            }
            p { class: "mb-12",
                "Please contact us with any questions or concerns, either through Piazza or directly via email. Thank you for your interest in our course!"
            }

            h2 { class: "text-2xl font-bold mb-4 text-white", "Staff ({CURRENT_SEMESTER.name})" }
            StaffList { semester: &CURRENT_SEMESTER }

            h2 { class: "text-2xl font-bold mb-4 mt-12 text-white", "Previous Iterations:" }
            ul { class: "list-disc ml-8 marker:text-foreground/50",
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
        }
    }
}

#[component]
fn StaffList(semester: &'static Semester) -> Element {
    let ta_label = if semester.tas.len() == 1 { "TA" } else { "TAs" };

    rsx! {
        StaffGroup { title: "Instructors", members: semester.instructors }
        StaffGroup { title: ta_label, members: semester.tas }
    }
}

#[component]
fn StaffGroup(title: &'static str, members: &'static [StaffMember]) -> Element {
    rsx! {
        if !members.is_empty() {
            h3 { class: "font-bold mb-2 text-white", "{title}" }
            ul { class: "list-disc ml-8 mb-4 marker:text-foreground/50",
                for member in members {
                    li {
                        "{member.name}"
                        if let Some(email) = member.email {
                            " ("
                            a {
                                href: "mailto:{email}",
                                class: "text-secondary",
                                "{email}"
                            }
                            ")"
                        }
                    }
                }
            }
        }
    }
}
