### 98-008: Intro to the Rust Programming Language

# Grep Lab

The goal of this homework is to make sure you know how to create a command line tool in Rust, as
well as how to write unit tests for your programs.

This homework is going to be _very_ different from other homeworks. There is no autograder, and we
will be **manually grading** your submission for correctness. You may also opt in to be graded on
code quality and robustness through a code review.

**For this homework, you will follow
[Chapter 12](https://doc.rust-lang.org/book/ch12-00-an-io-project.html) of the Rust Book and create
a project called `minigrep`**. Follow the hyperlink and go through all of
[Chapter 12](https://doc.rust-lang.org/book/ch12-00-an-io-project.html), building `minigrep` as you
go along.

There will be no handout for this homework. Instead, run `cargo new minigrep` in your terminal to
get started!

We encourage you to not skim through the book and blindly copy and paste any code you see. On top of
asking you to write robust code, we will require one "extra" thing from your submission, which will
check if you actually understand what you are pasting into your submission.

# Requirements

You must add at least 1 additional feature to `minigrep` on top of the functionality described in
the Book. Given this requirement, it will be more efficient for you to read the (relatively short)
[Chapter 12](https://doc.rust-lang.org/book/ch12-00-an-io-project.html) in its entirety, rather than
copy and paste everything and then reverse engineer. See the [Extra Features](#extra-features)
section below for some potential features you could add. You must document this feature as a
documentation comment (we will run `cargo doc` on your submission when we grade it, so the
documentation should be in the root of your library crate).

Another soft requirement is that you add unit tests. This won't count towards the first 100 points,
but will definitely contribute a non-significant amount to your code review score. We care more
about quality over quantity, but you should have at least 2 types of test cases (general
functionality, edge cases, potentially even performance benchmarking if you want to go above and
beyond with [criterion](https://bheisler.github.io/criterion.rs/book/)).

# Extra Features

The extra feature you want to add is **up to you** once you've finished the base `minigrep`.

- A very basic feature that you could add is a "count" flag through `-c` or `--count`, which changes
  the output to show how many lines a pattern is in, rather than printing out all of the lines.
- Another feature you could implement is searching directories as well as specific files.
- You can add regex support by using the [`regex`](https://docs.rs/regex/latest/regex/) crate.
- You can integrate a third-party CLI library such as [`clap`](https://docs.rs/clap/latest/clap/) to
  make the command line interface more user-friendly.
- You can integrate a third-party error-handling library such as
  [`anyhow`](https://docs.rs/anyhow/latest/anyhow/) or
  [`thiserror`](https://docs.rs/thiserror/latest/thiserror/) into your error handling code.

If you would like to do this with a different third-party crate, please ask us for permission first!

There are many things that you could do here, so try and be creative if you have time!
A good source of inspiration would be the man page for
[`grep`](https://man7.org/linux/man-pages/man1/grep.1.html). If you have more than one additional
feature, we will probably give you extra credit points in your code review.

Whatever you choose, make sure you document it by indicating what your feature is in your
documentation comments. The very first thing we will do when grading your submission will be running
`cargo doc --open`, so make sure it is obvious!

You should also make an effort to include a help message in your binary (triggered by `-h` or
`--help` flags). Make sure to include a description of each flag and its behavior. The help message
should also be printed if the user gives invalid command line arguments.

# Code Review and Grading

Your submission will be manually graded.

You may opt in to a code review by letting us know on Piazza. (More information will be posted there.)
If you simply follow along the tutorial and add a simple additional feature, you will get a full 100
points on this assignment. The optional code review will give you _up to_ 100 extra credit points.

We are mainly looking at the correctness of your feature (does your feature work how you describe in
your documentation and does it run without modifications), the robustness of your code (architecture
and design), the quality of tests you add, and general style (documentation and comments).

If you simply follow along the tutorial and add a simple additional feature, you will not score
very well on the code review. A high-quality submission should look _very_ different from the code
presented in the tutorial.

The way we grade this assignment will be different from almost every Computer Science course here at
CMU, where grading is mainly based on an autograder, and maybe a handful of points deductions for
style. Instead, we will be looking at your code wholistically, judging it based on how people expect
you to write code _outside_ of school.

However, we understand that this is still a StuCo, which is why this is all for extra credit.
Nevertheless, we still believe that this is an important exercise to go through as you will likely
need to go through this process in the future.

We are going to be really harsh! This practice is taken from CMU's Operating Systems course,
[15-410/605](https://www.cs.cmu.edu/~410/). Please don't worry if you receive a low code review
score, and remember that this is all extra credit!

# Submission

This time, you will submit the entire `minigrep` crate to Gradescope (instead of just the `src/`
directory)!

Please do not include the `target/` subdirectory when you zip the crate's root folder, either
manually or with `cargo clean`. You can always regenerate it with `cargo build` and `cargo test`.

Make sure that your code is fully formatted and linted with the following commands:

```sh
cargo clippy && cargo fmt --all -- --check
```

By following [Rust's style guidelines](https://doc.rust-lang.org/stable/style-guide/), you ensure
that anybody reading your code (who is familiar with Rust) will be able to easily navigate your
code. This can help with diving into an unfamiliar code base, and it also eliminates the need for
debate with others over style rules, saving time and energy.

See the official [guidelines](https://doc.rust-lang.org/stable/style-guide/) for more information.

# Collaboration

In general, feel free to discuss homeworks with other students! As long as you do not copy someone
else's work, any communication is fair game.

All formal questions should be asked on Piazza. Try to discuss on Piazza so that other students can
see your questions and answers as well!

You can also discuss on Discord, but try to keep any technical questions on Piazza.

# Feedback

We would like to reiterate that you should let us know if you spent anywhere in significant excess
of an hour on this homework.

In addition, Rust has a notoriously steep learning curve, so if you find yourself not understanding
the concepts, you should reach out to us and let us know as well --- chances are, you're not the
only one!
