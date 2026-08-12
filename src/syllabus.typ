// Course details
#let course-location = "GHC-4307"
#let meeting-time = "Thursdays 7:00-7:50pm"

#let long-semester = "Fall 2026"
#let short-semester = "F26"

#let instructors = (
  ("Anish Pallati", "apallati@andrew.cmu.edu"),
  ("Max Wen", "maxwen@andrew.cmu.edu"),
)

#let tas = (
  ("Bright Zheng", "brightz@andrew.cmu.edu"),
)

// Document setup
#set document(
  title: "98-008: Student Taught Courses (StuCo): Intro to Rust Lang",
  author: instructors.map(((name, email)) => name),
)

#set page(
  margin: 1.2in,
  numbering: "1",
  header: context {
    if counter(page).get().first() > 1 [
      #instructors.map(((name, email)) => name.split(" ").last()).join(", ")
      #h(1fr)
      98-008 #short-semester
    ]
  },
)

#let rust(body) = raw(body, lang: "rust")

// Use this for bare links (i.e. when the URL is also the text)
#let styled-link(url, body) = {
  link(url)[#text(fill: blue, font: "DejaVu Sans Mono", size: 0.8em)[#body]]
}

// Use this when links have custom text
#let unstyled-link(url, body) = {
  link(url)[#text(fill: blue)[#body]]
}

// Title page content
#align(center)[
  #block(width: 85%)[
    #set par(justify: false)
    #text(size: 20pt)[98-008: Student Taught Courses (StuCo): Intro to Rust Lang]

    #text(size: 10pt, style: "italic")[Updated #datetime.today().display("[month repr:long] [day], [year]")]

    #table(
      columns: 4,
      align: (right, center, left, left),
      stroke: none,
      inset: 3pt,

      table.cell(rowspan: instructors.len(), align: right + horizon)[#text(weight: "bold")[Instructors]],
      table.cell(rowspan: instructors.len(), align: center + horizon)[
        $lr(brace.l, size: #(instructors.len() * 1em))$
      ],
      ..instructors
        .map(((name, email)) => (
          [#name],
          [#styled-link("mailto:" + email)[#email]],
        ))
        .flatten(),

      table.cell(rowspan: tas.len(), align: right + horizon)[#text(weight: "bold")[TAs]],
      table.cell(rowspan: tas.len(), align: center + horizon)[
        $lr(brace.l, size: #(tas.len() * 1em))$
      ],
      ..tas
        .map(((name, email)) => (
          [#name],
          [#styled-link("mailto:" + email)[#email]],
        ))
        .flatten(),
    )

    *#long-semester*
  ]
]

#v(0.5em)

#table(
  columns: (1.2fr, 1fr),
  stroke: none,
  table.hline(stroke: 1pt),
  [Office Hours: after lecture, other times TBD], [Web: #styled-link("https://stuco.rs")[https://stuco.rs]],
  [Time: #meeting-time],
  [Location: #unstyled-link("https://maps.scottylabs.org/" + course-location)[#course-location]],
  table.hline(stroke: 1pt),
)

#v(1em)

= Course Description

This course is an introduction to the Rust programming language, a memory-safe systems programming language. We will cover a range of topics, from the Rust borrow checker to parallelism and the #rust("unsafe") keyword. Outside of lectures, there will be programming exercises (homework assignments) that students will need to complete in order to pass this course.

This course is taught in Rust and you are not expected to have prior exposure to Rust. However, this course assumes knowledge of topics covered in *15-122* or equivalent experience with the C programming language. Knowledge of functional programming idioms taught in *15-150*, as well as knowledge of systems programming taught in *15-213* is a bonus, but not strictly required.

= Learning Objectives

By the end of the semester, students should be able to read, write, and reason about Rust code. This includes:

- Core Rust syntax such as #rust("struct"), #rust("enum"), #rust("impl") blocks, pattern matching, generics, and #rust("trait")s
- Common Rust types such as #rust("Vec<T>"), #rust("String"), #rust("&str"), #rust("Option<T>"), and #rust("Result<T, E>")
- Use of the borrow checker through ownership and #rust("move") semantics
- Advanced Rust features such as iterators, lifetimes, closures, and smart pointers
- Advanced Rust patterns such as parallelism, concurrency, and #rust("unsafe")

= Additional Materials

Access to a computer capable of running `rustc` (the Rust compiler) and `cargo` (the Rust package manager) is required for this course. If you are using stable versions of MacOS, Windows, Nintendo Switch, or Linux, you should be fine.

You can follow the steps from the official Rust #unstyled-link("https://www.rust-lang.org/tools/install", "website"). If you need any assistance, please do not hesitate to ask us!

#pagebreak()

= Course Schedule

#align(center)[
  #block(width: 70%)[
    #table(
      columns: (auto, 1fr),
      align: (right, left),
      stroke: none,
      table.header([Week], [Topics #text(style: "italic")[(subject to change)]]),
      table.hline(stroke: 1pt),
      [Week 1], [Introduction],
      [Week 2], [Ownership],
      [Week 3], [Structs & Enums],
      [Week 4], [Collections and Generics],
      [Week 5], [Error Handling and Traits],
      [Week 6], [Modules and Testing],
      [Week 7], [The Rust Ecosystem],
      [Week 8], [Closures and Iterators],
      [Week 9], [Lifetimes],
      [Week 10], [Smart Pointers & Trait Objects],
      [Week 11], [Unsafe],
      [Week 12], [Parallelism],
      [Week 13], [Concurrency],
      table.hline(stroke: 1pt),
    )
  ]
]

= Attendance Policy

#text(fill: red, weight: "bold")[
  Please read this entire section, as not reading it may cause you to fail this course!
]

Attendance is required. According to StuCo guidelines, *if you have more than 2 unexcused absences, then you will automatically fail the course.*

== Excused Absences

If you are going to miss a lecture, please fill out the form on our #unstyled-link("https://stuco.rs", "course website") at least 2 hours in advance, and we can grant you an excused absence. Please do not email us for excusals.

Every 3 excused absences convert into 1 unexcused absence. Therefore, if you have 0 unexcused absences, you are allowed a maximum of 8 excused absences.

== Participation

Attendance will be taken via a survey or activity in lecture.

If you need to miss a lecture, we will determine an appropriate alternative so you don't miss out on that week's content. Most of the time, this is simply reading over the lecture and finishing the weekly programming assignment.

= Grading

Grading is determined solely by homework assignments submitted on Gradescope. Most of these assignments are autograded, but some are manually graded.

In order to pass this course (grade letter P or S), you must accumulate 1000 points. Homework assignments are usually worth 100-150 points each. However, many of the homework assignments have extra credit points. There will also be extra credit opportunities through submissions of feedback forms to us.

#align(center)[
  #table(
    columns: 2,
    align: (left, left),
    stroke: none,
    table.header([Grade], [Points]),
    table.hline(stroke: 1pt),
    [NP], [0-999],
    [P (or S)], [1000+],
    table.hline(stroke: 1pt),
  )
]

== Homework

There will be weekly homework assignments where you will write Rust code applying topics discussed in lecture. They are designed to take on average an hour per week, so if they take significantly longer than this, please let us know!

Homework deadlines are indicated on Gradescope. Usually, these are due the week after they are released, but we offer some #unstyled-link(<flexibility>)[flexibility with late submissions].

== Solutions

We believe that the best way to learn a programming language is to _actually write code_. You can only learn so much from watching someone else use a language, and this is especially true for Rust.

However, we understand that you are all busy students and we do not want this course to be a burden. *Therefore, we publish most homework solutions on our #unstyled-link("https://github.com/rust-stuco", "GitHub repository").*

The most obvious consequence of this decision is that you will have access to a correct solution before you even start. We strongly encourage students to avoid looking at these solutions before making a good-faith attempt. You will not learn much by copying and pasting our solutions as your own (and we will be able to detect if you do this).

*Referencing these solutions should be a _last resort_ if you run out of time or cannot reach us for help.* After completing an assignment, we encourage you to look at the reference solution to see our approach!

== AI

We recommend turning off any AI code assistants or agentic editors for this course. Learning Rust may require you to rewire parts of your thought process, and that rewiring is hindered when something else does your work for you. Remember that the goal here is not to pass the autograder, it is to learn a new programming language!

== Late work <flexibility>

Students have 7 late days to be used for any assignment. *However, if you let us know before an assignment's deadline, you can request additional late days.*

= Time Commitment

This is a 3-unit StuCo course, which means that this course should require around 3 hours of your time a week, including the 50-minute lecture.

If you find yourself spending more than 3 hours, please let us know immediately so that we can either help you or adjust something in the course itself.

= Passing

In the case that a student is unable to reach the passing boundary for any reason, they can reach out to us and we will make sure to work with them and provide alternate paths towards passing (usually a student-defined final project in Rust).

In other words, there are only two ways to fail this course. The first is accumulating three unexcused absences, which is due to StuCo guidelines. The second is the combination of not completing homework assignments _and_ not reaching out to us (you will have to reach out to us if you want to do a final project).

_Please_ always make sure to reach out to us on Piazza if you have _any_ concerns!

#pagebreak()

= Course Notes

== Academic Integrity

Discussion of homework problems, written or verbal, is not only permitted, but encouraged.

That said, *verbatim or near-verbatim copying of answers over any medium is expressly forbidden*. This *includes copying and pasting our solutions*. *If we find that you have plagiarized, we are required to report you to the university.*

If you find yourself needing to do this, talk to us! It might be the case that a change needs to happen on our end, not yours. It is _much_ easier for us to give you an extension than to go through the process of academic integrity violations (which we have learned by experience).

== Accommodations for students with disabilities

If you have a disability and have an accommodations letter from the Disability Resources office, we encourage you to discuss your accommodations and needs with us as early in the semester as possible. We will work with you to ensure that accommodations are provided as appropriate. If you suspect that you may have a disability and would benefit from accommodations but are not yet registered with the Office of Disability Resources, we encourage you to contact them at #styled-link("mailto:access@andrew.cmu.edu", "access@andrew.cmu.edu").

== Statement on student wellness

Take care of yourself. Do your best to maintain a healthy lifestyle this semester by eating well, exercising, avoiding drugs and alcohol, getting enough sleep and taking some time to relax. This will help you achieve your goals and cope with stress.

All of us benefit from support during times of struggle. There are many helpful resources available on campus and an important part of the college experience is learning how to ask for help. Asking for support sooner rather than later is almost always helpful.

If you or anyone you know experiences any academic stress, difficult life events, or feelings like anxiety or depression, we strongly encourage you to seek support. Counseling and Psychological Services (CaPS) is here to help: call #styled-link("tel:+4122682922", "(412) 268-2922") and visit their website at #styled-link("http://www.cmu.edu/counseling/", "http://www.cmu.edu/counseling/"). Consider reaching out to a friend, faculty or family member you trust for help getting connected to the support that can help.

== Statement on Diversity, Equity and Inclusion

We must treat every individual with respect. We are diverse in many ways, and this diversity is fundamental to building and maintaining an equitable and inclusive campus community. Diversity can refer to multiple ways that we identify ourselves, including but not limited to race, color, national origin, language, sex, disability, age, sexual orientation, gender identity, religion, creed, ancestry, belief, veteran status, or genetic information. Each of these diverse identities, along with many others not mentioned here, shape the perspectives our students, faculty, and staff bring to our campus. We, at CMU, will work to promote diversity, equity and inclusion not only because diversity fuels excellence and innovation, but because we want to pursue justice. We acknowledge our imperfections while we also fully commit to the work, inside and outside of our classrooms, of building and sustaining a campus community that increasingly embraces these core values.

Each of us is responsible for creating a safer, more inclusive environment.

Unfortunately, incidents of bias or discrimination do occur, whether intentional or unintentional. They contribute to creating an unwelcoming environment for individuals and groups at the university. Therefore, the university encourages anyone who experiences or observes unfair or hostile treatment on the basis of identity to speak out for justice and support, within the moment of the incident or after the incident has passed. Anyone can share these experiences using the following resources:

- Center for Student Diversity and Inclusion: #styled-link("mailto:csdi@andrew.cmu.edu", "csdi@andrew.cmu.edu"), #styled-link("tel:+4122682150", "(412) 268-2150")
- Report-It online anonymous reporting platform: #styled-link("https://reportit.net", "https://reportit.net")
  - username: `tartans`
  - password: `plaid`

All reports will be documented and deliberated to determine if there should be any following actions. Regardless of incident type, the university will use all shared experiences to transform our campus climate to be more equitable and just.
