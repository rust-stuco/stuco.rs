# Rust StuCo Homeworks

_All homeworks are due the week after they are assigned, unless you use late days._
_You have 7 late days to use whenever you need._

## Homeworks

We've' tried to calibrate each of the homeworks to take around an hour.
If you find yourself spending much more than an hour on these, please let us know!
This is the first time these assignments have been handed out,
and we are very open to feedback regarding the length and difficulty.

## Setup

Make sure you have Rust installed. You should have installed Rust through `rustup`. To confirm, run:

```sh
$ rustup --version
rustup 1.28.2 (e4f3ad6f8 2025-04-28)
$ cargo --version
cargo 1.92.0 (344c4567c 2025-10-21)
```

Make sure that both of those commands execute successfully, and that they have relatively recent
dates. If the dates are not recent, you can update `rustup` by running `rustup update`.

If you want a local version of this generated writeup, you can generate it with `cargo doc`. Once
you have `cargo` installed, run `cargo doc --open` in this directory to generate documentation for
this homework.

```sh
$ cargo doc --open
Documenting primerlab v0.1.0 (<path>/primerlab)
   Finished dev [unoptimized + debuginfo] target(s) in 0.11s
    Opening <path>/primerlab/target/doc/primerlab/index.html
```

Either way, a version of this writeup will be up on our website!

## Submission

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

## Collaboration

In general, feel free to discuss homeworks with other students! As long as you do not copy someone
else's work, any communication is fair game.

All formal questions should be asked on Piazza. Try to discuss on Piazza so that other students can
see your questions and answers as well!

You can also discuss on Discord, but try to keep any technical questions on Piazza.

## Feedback

We would like to reiterate that you should let us know if you spent anywhere in significant excess
of an hour on this homework.

In addition, Rust has a notoriously steep learning curve, so if you find yourself not understanding
the concepts, you should reach out to us and let us know as well --- chances are, you're not the
only one!
