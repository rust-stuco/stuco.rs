### 98-008: Intro to the Rust Programming Language

# Get Owned Lab

The goal of this homework is to familiarize yourself with Rust's system of Ownership.

Lecture was heavily based on
[Chapter 4 of the Rust Book](https://doc.rust-lang.org/book/ch04-00-understanding-ownership.html).
We **_strongly_** suggest that you make sure you understand the rules of Ownership before attempting
this homework, and this text is a great place to reference!

This homework is a bit shorter than the previous one, mainly so you can have extra time to read
through the book and stew with the concepts a bit. If you have more time, you are welcome to solve the bonus problem, but note that it is completely optional!

If you find yourself getting stuck, it might be good to take a small break and go over the borrow
checker's [rules](https://doc.rust-lang.org/book/ch04-01-what-is-ownership.html#ownership-rules).

Remember to always read the error messages that the compiler gives you, and as always, we encourage
you to reach out for help!

# Part 1: Slices

Like the previous homework, you will modify 3 files that don't compile by adding, removing, or
reorganizing code to make them compile! The process should be identical to the previous homework,
except the exercises are now specific to slices.

When you are done, you should be able to run `cargo test` with all of the modules (`pub mod`s) in
`src/slices/mod.rs` uncommented.

# Part 2: Move Semantics

Like the previous section, you will modify 5 files to make them compile. The first 3 exercises
involve the `Vec` type, which is an _owned_ type.

For this homework, all you need to know is the `vec![]` macro and the `push` method on vectors.
Recall the `vec![]` creates a new vector with elements, and `push` appends an element onto the end
of a vector.

# Part 3: Strings

In this section, you'll be working with `String` and `&str` instead of `Vec`. Recall that `String`
is an _owned_ type and `&str` is _borrowed_ type.

The first three exercises in `src/strings` are just like the previous section, where you need to
make some changes for the functions to compile.

A useful thing to know here is that a `&String` can be _deref coerced_ into a `&str` when passed as
a function parameter (you can imagine it turning into a different type as it enters the function).

Other than that, just remember to always read what the compiler tells you (it may or may not give
away the answer)!

**For `strings4.rs`, you will need to look at some real documentation.**

You _could_ just google the answer, but it would be good to familiarize yourself with official Rust
documentation! Linked here is the documentation for
[`str`](https://doc.rust-lang.org/std/primitive.str.html), which might be useful.

There's a search bar at the top of the documentation that you should make use of. Try searching for
a method called `trim` there!

# Bonus: Rice's Theorem

_This bonus problem is worth 50 extra credit points. Feel free to skip it!_

## Motivation

We've talked a lot about Rust's safety guarantees and its remarkable borrow checking system. By writing safe Rust and following ownership rules, we can eliminate an entire class of memory bugs.

But Rust is not perfect. In fact, it is impossible for any compiler to determine if a program is memory safe or not. **[Rice's theorem](https://en.wikipedia.org/wiki/Rice%27s_theorem)** states that all non-trivial semantic properties of programs are undecidable. This applies to the property of memory safety as well.

To get around this, the creators of Rust had to make a compromise. Rust is _sound_, meaning that if safe Rust code compiles, then it is guaranteed to be memory safe. However, Rust is not _complete_, meaning that if a program is memory safe, then it might not necessarily compile.

In other words, the borrow checker is designed to be _conservative_. It prefers to reject a safe program rather than risk letting an unsafe one slip through. You'll demonstrate this corollary of Rice's theorem in this bonus problem.

## Task

**Show that the Rust borrow checker is not complete.**

More specifically, in `src/bonus.md`, write a Rust program that the borrow checker rejects but is actually memory safe. Copy the error message you get when compiling the program. Then, prove the safety of your program.

**Constraints:** Do not use types, data structures, or features that we have not taught yet. You may only use programming concepts that we have covered in Weeks 1 and 2. Please don't search up this problem or use AI either!

When we say _prove_, we don't require anything extremely formal. You may write paragraphs or even 15-122-style point-to proofs - whatever method you prefer, as long as you convince us!

We highly recommend brainstorming a solution with another student! You could make a new friend along the way :D

# Submission

### Formatting and Style

The autograder will run these two commands on your code:

```sh
cargo clippy && cargo fmt --all -- --check
```

**If the autograder detects any errors from the command above, you will not be able to receive**
**any points.** This may seem strict, but we have decided to follow standard best practices for
Rust.

By following [Rust's style guidelines](https://doc.rust-lang.org/stable/style-guide/), you ensure
that anybody reading your code (who is familiar with Rust) will be able to easily navigate your
code. This can help with diving into an unfamiliar code base, and it also eliminates the need for
debate with others over style rules, saving time and energy.

See the official [guidelines](https://doc.rust-lang.org/stable/style-guide/) for more information.

### Unix

If you are on a unix system, we will try to create a `handin.zip` automatically for you,
**but you will need to have `zip` already installed**.

If you _do not_ have `zip` installed on your system, install `zip` on your machine or use the CMU
Linux SSH machines. If you need help with this, please reach out to us!

Once you have `zip` installed, we will create the `handin.zip` automatically for you (_take a peek_
_into `build.rs` file if you're interested in how this works!_).

Once you have the `handin.zip` file, submit it (and only the zip) to Gradescope.

### Windows

If you are on a windows system, you can zip the `src/` folder manually and upload that to
Gradescope.

Note that you don't _need_ to name it `handin.zip`, you can name it whatever you'd like.

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
