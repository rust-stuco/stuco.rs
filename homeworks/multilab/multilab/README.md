### 98-008: Intro to the Rust Programming Language

# Multi Lab

The goal of this homework is to make sure you are comfortable with some of the generic collection
types in the [`std::collections`] module.

You will implement two data structures that track multiplicity (the number of times elements appear)
on top of their normal operations.

The first will be a [`MultiSet`](crate::multiset::MultiSet), which is a set that can have
duplicate entries. The second will be a [`MultiMap`](crate::multimap::MultiMap), which is a map that
maps keys to any number of values.

For [`MultiSet`](crate::multiset::MultiSet), we have not provided what types the fields should be.
_We recommend that you use one of the collection types provided in the [`std::collections`] module_
_to help you build these new types._

We would like you to think a bit more about your own code than previous homeworks, so we will be
providing less detail than before on implementation details. Instead, you should read the
documentation and implement the data structure and methods so that they follow the documentation and
pass all the test cases.

# Part 1: `MultiSet`

Click this hyperlink to go to the documentation for [`MultiSet`](crate::multiset::MultiSet)!

# Part 2: `MultiMap`

Click this hyperlink to go to the documentation for [`MultiMap`](crate::multimap::MultiMap)!

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
