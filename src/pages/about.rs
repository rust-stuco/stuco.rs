use dioxus::prelude::*;

#[derive(PartialEq)]
pub struct Semester {
    pub name: &'static str,
    pub instructors: &'static [(&'static str, Option<&'static str>)], // (name, optional email)
    pub tas: &'static [(&'static str, Option<&'static str>)],
}

pub const CURRENT_SEMESTER: Semester = Semester {
    name: "Spring 2026",
    instructors: &[
        ("Stephen Mao", Some("stmao@andrew.cmu.edu")),
        ("Hugo Latendresse", Some("hlatendr@andrew.cmu.edu")),
        ("Anish Pallati", Some("apallati@andrew.cmu.edu")),
    ],
    tas: &[("Max Wen", Some("maxwen@andrew.cmu.edu"))],
};

const PREVIOUS_SEMESTERS: &[Semester] = &[
    Semester {
        name: "F25",
        instructors: &[("Fiona Fisher", None), ("Terrance Chen", None)],
        tas: &[("Stephen Mao", None)],
    },
    Semester {
        name: "S25",
        instructors: &[("Connor Tsui", None), ("Jessica Ruan", None)],
        tas: &[("Fiona Fisher", None), ("Terrance Chen", None)],
    },
    Semester {
        name: "F24",
        instructors: &[
            ("Benjamin Owad", None),
            ("Connor Tsui", None),
            ("David Rudo", None),
        ],
        tas: &[],
    },
    Semester {
        name: "S24",
        instructors: &[
            ("Benjamin Owad", None),
            ("Connor Tsui", None),
            ("David Rudo", None),
        ],
        tas: &[],
    },
    Semester {
        name: "S22, F22, and S23",
        instructors: &[("Jack Duvall", None), ("Cooper Pierce", None)],
        tas: &[],
    },
];

#[component]
pub fn About() -> Element {
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
                        let instructors = semester
                            .instructors
                            .iter()
                            .map(|(name, _)| *name)
                            .collect::<Vec<_>>()
                            .join(", ");
                        let ta_label = if semester.tas.len() == 1 { "TA" } else { "TAs" };
                        let tas = semester
                            .tas
                            .iter()
                            .map(|(name, _)| *name)
                            .collect::<Vec<_>>()
                            .join(", ");

                        rsx! {
                            li {
                                span { class: "text-primary", "{semester.name}" }
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
        if !semester.instructors.is_empty() {
            h3 { class: "font-bold mb-2 text-white", "Instructors" }
            ul { class: "list-disc ml-8 mb-4 marker:text-foreground/50",
                for (name , email) in semester.instructors {
                    li {
                        "{name}"
                        if let Some(email) = email {
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
        if !semester.tas.is_empty() {
            h3 { class: "font-bold mb-2 text-white", "{ta_label}" }
            ul { class: "list-disc ml-8 mb-4 marker:text-foreground/50",
                for (name , email) in semester.tas {
                    li {
                        "{name}"
                        if let Some(email) = email {
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
