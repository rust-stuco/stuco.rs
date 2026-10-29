### 98-008: Intro to the Rust Programming Language

# Split Lab

The goal of this homework is to give you a better understanding of **lifetimes** in Rust.

So far, we have mostly dealt with owned values. However, there is overhead to dealing with owned
values everywhere, as holding multiply clones of data can be inefficient and expensive. For
performance-critical applications, it is often necessary to use references instead of owned values.
When you use references, you are also using lifetimes, even if you can't see them directly.

# Task

You will implement functionality similar to that of the [`split`] method on [`str`] in the Rust standard library!

**In `split_str.rs`, construct a `Split` struct** that implements the [`Iterator`] trait, where the items it yields are slices of the original string (the `haystack`), delimited by some `delimiter`.

[`split`]: https://doc.rust-lang.org/std/primitive.str.html#method.split
[`str`]: https://doc.rust-lang.org/std/primitive.str.html
[`Iterator`]: https://doc.rust-lang.org/std/iter/trait.Iterator.html

Instead of trying to figure this out on your own, we ask that you watch ["Crust of Rust: Lifetime Annotations"](https://www.youtube.com/watch?v=rAl-9HwD858) where Jon Gjengset (a very notable educator in the Rust community) explains exactly how to implement this iterator.

**Note:** The starter code we have given you is slightly different to how Jon implements `Split`, so
be aware of the difference. It is not so different that you will need to rewrite everything from
scratch. You will, however, need to make some changes to the starter code in order to get things
to compile (mainly removing the placeholder `'static` lifetimes).

# Livestream

The full livestream is 90 minutes, but you can skip many sections!
If they are ~~struck out~~ below, they are not necessary to watch in
order to finish this assignment, but they will help you understand the concepts better. The **bolded** sections are ones that we think are **very** important.

We recommend watching the livestream _before_ writing any code. You
will not need to write more than 20 lines of code, and understanding the concepts is far more important!

## Naive Implementation

To begin, Jon walks you through a naive implementation of a split iterator.

- ~~0:00:00 Introduction~~
- ~~0:03:36 Start a rust project~~
- **0:05:20 Struct and method definitions for StrSplit and first test**
- ~~0:09:32 How you decide between a library and a binary~~
- **0:10:58 Start implementing StrSplit**
- ~~0:16:15 When to use match vs if let some~~

## Compile Error: Lifetime Specifiers

After writing the naive implementation, you will realize that there are compile-time errors. How do
we fix them?

- **0:17:10 Doesn't compile! missing lifetime specifier**
- 0:20:33 Can I be wrong by specifying lifetimes?
- 0:21:25 Anonymous lifetime '\_
- ~~0:23:10 Order lifetimes based on how long they are~~
- ~~0:25:18 Anonymous lifetime '\_ (with multiple lifetimes)~~
- **0:26:52 Compile error: lifetime of reference outlives lifetime of borrowed content**
- 0:34:45 Static lifetime

## Bugs

The compile-time errors have been fixed, but there are still a few logical bugs.

The bugs in this section don't get fixed until the end of this group of sections. If you would like,
you can skip everything and then pause to read the updated code.

**Note:** This code is not exactly idiomatic to 2024 Rust, as there were certain features not
stabilized back in 2020. In other words, don't worry too much about the `ref` keyword. You can look
at the reference solution on our GitHub that does not use `ref`.

- 0:41:27 Bug when a delimiter tails a string
- 0:48:07 What is the ref keyword and why not &
- 0:51:36 What's the \* on the left of remainder
- 0:52:46 What is take() doing
- 0:54:48 Mutable references are one level deep
- 0:55:39 Solving a hang with as_mut()

## Multiple Lifetimes

The code is working, but it will not work for very specific scenarios. What happens if the haystack
and the delimiter have different lifetimes? Or in other words, what happens if they come from
different places?

- **0:57:49 Multiple lifetimes, implementing until_char**
- ~~1:03:19 Difference between a str and a String~~
- **1:08:15 Multiple lifetimes (continued)**

## Bonus: Generic Delimiters

_This bonus problem is worth 50 extra credit points. Feel free to skip it!_

We can observe that our delimiters only need to have one operation: finding themselves inside a
string!

In `split_pattern.rs`, implement the `split_pattern` file which exposes a `Pattern` trait. This is a simplified version of the real `Pattern` trait in the standard library.

- 1:15:24 Generic delimiter (Delimiter trait)
- ~~1:23:14 char length utf8~~
- ~~1:25:30 Standard library split~~
- ~~1:27:39 Q&A~~

# Bonus: Lifetime Elision

_This bonus problem is worth 50 extra credit points. Feel free to skip it!_

Complete the Lifetime Elision assignment on [Gradescope](https://stuco.rs/gradescope) for even more bonus points!

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

# Feedback

We would like to reiterate that you should let us know if you spent anywhere in significant excess
of an hour on this homework.

In addition, Rust has a notoriously steep learning curve, so if you find yourself not understanding
the concepts, you should reach out to us and let us know as well --- chances are, you're not the
only one!
