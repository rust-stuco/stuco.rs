use std::fmt::Write;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(super) struct StaffMember {
    pub(super) name: &'static str,
    pub(super) email: Option<&'static str>,
}

impl StaffMember {
    const fn new(name: &'static str, email: Option<&'static str>) -> Self {
        Self { name, email }
    }
}

#[derive(Debug, PartialEq, Eq)]
pub(super) struct Semester {
    pub(super) name: &'static str,
    pub(super) instructors: &'static [StaffMember],
    pub(super) tas: &'static [StaffMember],
    pub(super) link: &'static str,
}

pub(super) fn format_staff_names(staff: &[StaffMember]) -> String {
    match staff {
        [] => String::new(),
        [member] => member.name.to_owned(),
        [first, second] => format!("{} and {}", first.name, second.name),
        [leading @ .., last] => {
            let mut names = leading
                .iter()
                .map(|member| member.name)
                .collect::<Vec<_>>()
                .join(", ");
            write!(names, ", and {}", last.name).expect("writing to a String cannot fail");
            names
        }
    }
}

pub(super) const CURRENT_SEMESTER: Semester = Semester {
    name: "Fall 2026",
    instructors: &[
        StaffMember::new("Anish Pallati", Some("apallati@andrew.cmu.edu")),
        StaffMember::new("Max Wen", Some("maxwen@andrew.cmu.edu")),
    ],
    tas: &[StaffMember::new("Bright Zheng", Some("brightz@andrew.cmu.edu"))],
    link: "https://stuco.rs",
};

pub(super) const PREVIOUS_SEMESTERS: &[Semester] = &[
    Semester {
        name: "S26",
        instructors: &[
            StaffMember::new("Stephen Mao", Some("stmao@andrew.cmu.edu")),
            StaffMember::new("Hugo Latendresse", Some("hlatendr@andrew.cmu.edu")),
            StaffMember::new("Anish Pallati", Some("apallati@andrew.cmu.edu")),
        ],
        tas: &[StaffMember::new("Max Wen", Some("maxwen@andrew.cmu.edu"))],
        link: "https://stuco.rs",
    },
    Semester {
        name: "F25",
        instructors: &[
            StaffMember::new("Fiona Fisher", None),
            StaffMember::new("Terrance Chen", None),
        ],
        tas: &[StaffMember::new("Stephen Mao", None)],
        link: "https://rust-stuco.github.io/",
    },
    Semester {
        name: "S25",
        instructors: &[
            StaffMember::new("Connor Tsui", None),
            StaffMember::new("Jessica Ruan", None),
        ],
        tas: &[
            StaffMember::new("Fiona Fisher", None),
            StaffMember::new("Terrance Chen", None),
        ],
        link: "https://rust-stuco.github.io/old/s25/",
    },
    Semester {
        name: "F24",
        instructors: &[
            StaffMember::new("Benjamin Owad", None),
            StaffMember::new("Connor Tsui", None),
            StaffMember::new("David Rudo", None),
        ],
        tas: &[],
        link: "https://rust-stuco.github.io/old/f24/",
    },
    Semester {
        name: "S24",
        instructors: &[
            StaffMember::new("Benjamin Owad", None),
            StaffMember::new("Connor Tsui", None),
            StaffMember::new("David Rudo", None),
        ],
        tas: &[],
        link: "https://rust-stuco.github.io/old/s24/",
    },
    Semester {
        name: "S22, F22, and S23",
        instructors: &[
            StaffMember::new("Jack Duvall", None),
            StaffMember::new("Cooper Pierce", None),
        ],
        tas: &[],
        link: "https://old-rust-stuco.duvallj.pw/",
    },
];
