### 98-008: Intro to the Rust Programming Language

# Split Lab

The goal of this homework is to give you a better understanding of **lifetimes** in Rust.

So far, we have mostly dealt with owned values. However, there is overhead to dealing with owned
values everywhere, as holding multiply clones of data can be inefficient and expensive. For
performance-critical applications, it is often necessary to use references instead of owned values.
When you use references, you are also using lifetimes, even if you can't see them directly.

In this homework, you will implement functionality similar to that of the [`split`] method on
[`str`] in the Rust standard library. You will need to construct a `Split` struct that implements
the [`Iterator`] trait, where the items it yields are slices of the original string (the
`haystack`), delimited by some `delimiter`.

A correct solution is likely going to contain very few lines of code (~20). However, the interaction
between the lifetimes and your code will probably be tricky to implement on your own.

Instead of trying to figure this out on your own, we ask that you watch the following livestream
called ["Crust of Rust: Lifetime Annotations"](https://youtu.be/rAl-9HwD858?si=VTQfI8Re7DvrtDqy).
This is a live-coding stream on YouTube where Jon Gjengset (a very notable educator in the Rust
community) explains exactly how to implement this iterator.

[`split`]: https://doc.rust-lang.org/std/primitive.str.html#method.split
[`str`]: https://doc.rust-lang.org/std/primitive.str.html

# Notes

Please don't spend too much time on this! The goal here is to understand how lifetimes work, not how
to specifically implement something that is already in the standard library.

**Also, the starter code we have given you is slightly different to how Jon implements `Split`, so
be aware of the difference. It is not so different that you will need to rewrite everything from
scratch.** You will, however, need to make some changes to the starter code in order to get things
to compile (mainly removing the placeholder `'static` lifetimes).

_Note that Jon does not reveal how to make those compile until the end of the livestream (the
"Multiple lifetimes" sections). After that section, you only need to keep watching if you want to
complete the extra credit in `src/split_pattern.rs`._

**Finally, it is probably in your interest to watch the livestream _before_ writing any code, as you
will not need to write more than 20 lines of code, and understanding the concepts is far more
important.** If you try to finesse how much of the livestream you watch and how much code you write
in between the sections, you will likely end up spending **more** time than if you just watched the
entire livestream and wrote your own solution.

## Livestream

The livestream is 90 minutes, but you can skip several sections. We've listed the sections you
_should_ watch below (you can jump around via the chapters on YouTube). Of course, we would
recommend watching the entire livestream if you have time, as Jon is an excellent teacher! It is
also in general a good exercise to watch how an experienced developer writes code.

What's even better is that you watch the entire livestream _without_ touching your code editor, and
after you finish it, you can try to implement the iterator yourself!

### Livestream Sections

Below are the sections. If they are ~~struck out~~, they are not absolutely necessary to watch in
order to finish this assignment, but they will definitely help you understand the concepts better.
The **bolded** sections are ones that we think are **very** important.

#### Naive Implementation

In these sections, Jon walks you through a naive implementation of a split iterator.

- ~~0:00:00 Introduction~~
- ~~0:03:36 Start a rust project~~
- **0:05:20 Struct and method definitions for StrSplit and first test**
- ~~0:09:32 How you decide between a library and a binary~~
- **0:10:58 Start implementing StrSplit**
- ~~0:16:15 When to use match vs if let some~~

#### Compile Error: Lifetime Specifiers

After writing the naive implementation, you will realize that there are compile-time errors. How do
we fix them?

- **0:17:10 Doesn't compile! missing lifetime specifier**
- 0:20:33 Can I be wrong by specifying lifetimes?
- 0:21:25 Anonymous lifetime '\_
- ~~0:23:10 Order lifetimes based on how long they are~~
- ~~0:25:18 Anonymous lifetime '\_ (with multiple lifetimes)~~
- **0:26:52 Compile error: lifetime of reference outlives lifetime of borrowed content**
- 0:34:45 Static lifetime

#### Bugs

The compile-time errors have been fixed, but there are still a few logical bugs.

The bugs in this section don't get fixed until the end of this group of sections. If you would like
you can skip everything and then pause to read the updated code.

_Note that this code is not exactly idiomatic to 2024 Rust, as there were certain features not
stabilized back in 2020. In other words, don't worry too much about the `ref` keyword. You can look
at the reference solution on our GitHub that does not use `ref`._

- 0:41:27 Bug when a delimiter tails a string
- 0:48:07 What is the ref keyword and why not &
- 0:51:36 What's the \* on the left of remainder
- 0:52:46 What is take() doing
- 0:54:48 Mutable references are one level deep
- 0:55:39 Solving a hang with as_mut()

#### Multiple Lifetimes

The code is working, but it will not work for very specific scenarios. What happens if the haystack
and the delimiter have different lifetimes? Or in other words, what happens if they come from
different places?

- **0:57:49 Multiple lifetimes, implementing until_char**
- ~~1:03:19 Difference between a str and a String~~
- **1:08:15 Multiple lifetimes (continued)**

#### Generic Delimiters (Extra Credit):

We can observe that our delimiters only need to have one operation: finding themselves inside a
string! You can implement the `split_pattern` file which exposes a `Pattern` trait. This is a
simplified version of the real `Pattern` trait in the standard library.

- 1:15:24 Generic delimiter (Delimiter trait)
- 1:23:14 char length utf8
- 1:25:30 Standard library split
- 1:27:39 Q&A

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
