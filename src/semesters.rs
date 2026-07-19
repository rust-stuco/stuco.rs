pub(crate) type StaffMember = (&'static str, Option<&'static str>);

#[derive(PartialEq)]
pub(crate) struct Semester {
    pub(crate) name: &'static str,
    pub(crate) instructors: &'static [StaffMember],
    pub(crate) tas: &'static [StaffMember],
    pub(crate) link: &'static str,
}

pub(crate) const CURRENT_SEMESTER: Semester = Semester {
    name: "Spring 2026",
    instructors: &[
        ("Stephen Mao", Some("stmao@andrew.cmu.edu")),
        ("Hugo Latendresse", Some("hlatendr@andrew.cmu.edu")),
        ("Anish Pallati", Some("apallati@andrew.cmu.edu")),
    ],
    tas: &[("Max Wen", Some("maxwen@andrew.cmu.edu"))],
    link: "https://stuco.rs",
};

pub(crate) const PREVIOUS_SEMESTERS: &[Semester] = &[
    Semester {
        name: "F25",
        instructors: &[("Fiona Fisher", None), ("Terrance Chen", None)],
        tas: &[("Stephen Mao", None)],
        link: "https://rust-stuco.github.io/",
    },
    Semester {
        name: "S25",
        instructors: &[("Connor Tsui", None), ("Jessica Ruan", None)],
        tas: &[("Fiona Fisher", None), ("Terrance Chen", None)],
        link: "https://rust-stuco.github.io/old/s25/",
    },
    Semester {
        name: "F24",
        instructors: &[
            ("Benjamin Owad", None),
            ("Connor Tsui", None),
            ("David Rudo", None),
        ],
        tas: &[],
        link: "https://rust-stuco.github.io/old/f24/",
    },
    Semester {
        name: "S24",
        instructors: &[
            ("Benjamin Owad", None),
            ("Connor Tsui", None),
            ("David Rudo", None),
        ],
        tas: &[],
        link: "https://rust-stuco.github.io/old/s24/",
    },
    Semester {
        name: "S22, F22, and S23",
        instructors: &[("Jack Duvall", None), ("Cooper Pierce", None)],
        tas: &[],
        link: "https://old-rust-stuco.duvallj.pw/",
    },
];
