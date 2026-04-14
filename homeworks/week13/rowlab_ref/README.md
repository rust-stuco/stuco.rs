### 98-008: Intro to the Rust Programming Language

# Row Lab

This is the final assignment of the semester! Congrats on making it here.

Your final task is to implement something similar to
[The One Billion Row Challenge](https://www.morling.dev/blog/one-billion-row-challenge/). The goal
here is to get familiar with parallelism in Rust, as well as put together everything you've learned
over the past semester to write a program that has practical application in the real world!

We are not going to give that much guidance here, since at this point you should be familiar enough
with Rust as a language that you can figure out everything on your own. Of course, we'll explain
enough to get you started.

**The description of the original challenge can be found
[here](https://www.morling.dev/blog/one-billion-row-challenge/), so give it a quick read!**

The main difference between this assignment and the real challenge is A) we are not writing Java,
and B) instead of reading the data from a file / disk, we computationally generate the random data
(in-memory) via an iterator.

_The second difference is mainly because Gradescope does not support more than 6 GB of memory per
autograder (1 billion rows is approximately 14 GB), which means the complete data cannot fit in
memory. Asking you to interact with I/O while also dealing with parallelism seemed a bit too cruel
for this assignment, so we modified the challenge slightly. That being said, we encourage you to
take the code you write for this lab and try the real challenge yourself!_

**For this lab, you are allowed to use third-party crates!** This means you will have to also submit
your `Cargo.toml` file. See the [Submission](#submission) section for more information.

# Starter Code

We have provided quite a lot of starter code for you to use! The two files that you should be
modifying are `aggregation.rs` and `lib.rs`. You are allowed to modify `main.rs` and
`measurements.rs` locally on your own computer, but the Gradescope autograder will be using the
starter code for those two files. The other two files you should know about are `tests/mock.rs` as
well as `benches/brc.rs`, which are explained in the next two sections.

`aggregation.rs` contains our recommended helper structs and methods for aggregating the data. You
are allowed to completely rewrite everything except the function signature of `aggregate` and the
struct definitions for `WeatherStations` and `AggregationResults` (but you are allowed to and
encouraged to change the fields of `AggregationResults`).

Once you have implemented the `todo!()`s in `aggregation.rs`, you can move on to `lib.rs`. We have
provided you with a naive single-threaded version of this challenge. From here, it is up to you to
make things faster! See the [Benchmarking](#benchmarking-and-leaderboard) section for some hints 🦀.

# Testing

There are 3 integration tests located in `tests/mock.rs`. We will manually check your code for
parallelism, and as long as you have integrated parallelism in some non-trivial manner, you will
receive full credit if you pass the 3 tests.

If you make any changes to struct definitions or function signatures, make sure that you can still
compile everything with `cargo test`!

# Benchmarking and Leaderboard

We have set up benchmarking via [Criterion](https://bheisler.github.io/criterion.rs/book/) for you.
You can run `cargo bench` to see how long (on average) your `aggregate` function takes to aggregate
1 billion rows. Note that the minimum number of samples it will run is 10, so if your code is
**very** slow, you might just want to run the small timing program in `main.rs` via `cargo run`.

There will also be a leaderboard on Gradescope! Compete to please Ferris with the fastest time. We
will give you quite a lot of extra credit if you can beat Ferris (our reference solution) by some
non-trivial amount. The top leaderboard finishers might get a huge amount of points 🦀🦀🦀🦀🦀

### Optimizations

There are many, many ways to speed up a program like the one you need to implement. In fact, there
is a whole field dedicated to speeding up this kind of program: when you have a `GROUP BY` clause in
SQL, the relational database executing the SQL query is doing almost this exact aggregation! If you
are interested in this, you should take CMU's
[Databse Systems](https://15445.courses.cs.cmu.edu/spring2025/) course.

We won't go into detail here, but you are allowed to go online and look at all of the techniques
other people have used for this challenge. You can also read the
[Rust Performance Book](https://nnethercote.github.io/perf-book/) online. Just make sure not to copy
and paste anyone else's code without citing them first!

For this assignment, we would actually encourage you to look at the reference solution after giving
a good-faith attempt at designing an algorithm yourself. Our reference solution is purposefully not
very well optimized, but it does show the syntax for using parallelism in Rust. We encourage you to
play around with the code!

Note that because the original challenge involved reading from a file (interacting with I/O), not
everything online will be applicable to this assignment. Still, there's a lot of cool things on the
internet that you _can_ make use of. Also, be careful when trying to use SIMD, as you will be graded
on the Gradescope Docker containers.

_That being said, if you really want to play around with I/O and perhaps some `unsafe`ty with system
calls (like `mmap`), reach out to us! We might give permission for you to submit the real challenge
if we think you are capable of it._

# Submission

For this lab, you are allowed to use third-party crates! **This means that you must also submit your
`Cargo.toml` file** (otherwise we wouldn't be able to compile your code). The `build.rs` build
script will handle that for you

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
Gradescope. For this lab, you also need to add the `Cargo.toml` file to that zip folder. Please
reach out to us if you are unsure how to do this!

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
