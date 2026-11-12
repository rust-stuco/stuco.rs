### 98-008: Intro to the Rust Programming Language

# Row Lab

This is the final assignment of the semester! Congrats on making it here.

The goal here is to get familiar with **parallelism** in Rust, as well as put together everything you've learned over the past semester to write a program that has practical application in the real world!

# Overview

Your final task is to implement something similar to
[The One Billion Row Challenge](https://www.morling.dev/blog/one-billion-row-challenge/)! Given a billion weather station measurements, find an efficient way to calculate the min, mean, and max temperature values of each station.

There are two differences between this assignment and the real challenge: A) we are not writing Java (obviously!) and B) since Gradescope does not support more than 6 GB of memory per autograder (1 billion rows is approximately 14 GB), we generate the random data via an iterator instead of reading the data from a file.

# Starter Code

**First, implement the `todo!()`s in `aggregation.rs`**, which contains our recommended helper structs and methods for aggregating the data. You are allowed to rewrite anything, as long as the tests pass! We encourage you to change the fields of `AggregationResults`.

**Next, implement the `aggregate` function in `lib.rs`** to solve the core challenge. We have
provided you with a naive single-threaded version of this challenge, but it times out on Gradescope. From here, it is up to you to make things faster! Feel free to use any third-party crates like `rayon`. See the [Optimization Advice](#optimization-advice) section for some hints.

You are allowed to modify `main.rs` and `measurements.rs` on your own computer, but the Gradescope autograder will be using the starter code for those two files.

# Testing

There are integration tests located in `tests/mock.rs` to assess correctness. If you make any changes to struct definitions or function signatures, make sure that you can still compile everything with `cargo test`!

# Benchmarking

We have set up benchmarking via [Criterion](https://bheisler.github.io/criterion.rs/book/) for you.

You can run `cargo bench` to see how long (on average) your `aggregate` function takes to aggregate
1 billion rows. Note that the minimum number of samples it will run is 10, so if your code is
**very** slow, you may just want to run the small timing program in `main.rs` via `cargo run`.

The autograder will benchmark your solution using 4.0 CPU and 6.0 GB RAM, with up to 8 available cores. Pass all the tests without exceeding the memory limit to earn full credit and claim your spot on the **Gradescope leaderboard**!

### Bonus: Can you beat Ferris?

_This bonus problem is worth many extra credit points. Feel free to skip it!_

Ferris has implemented a reference solution that requires 30 seconds of runtime, according to `cargo bench`.

Beat this time to earn extra credit! The faster your solution, the higher your bonus and ranking on the leaderboard.

### Optimization Advice

There are many ways to speed up a program like this one. In fact, there
is a whole field dedicated to such optimizations: when you have a `GROUP BY` clause in
SQL, the relational database executing the SQL query is almost doing this exact aggregation! If you
are interested in this, we recommend CMU's
[Database Systems](https://15445.courses.cs.cmu.edu/) course.

Since the original challenge involved reading from a file (interacting with I/O), not
everything online will be applicable to this assignment. Your code will likely be bottlenecked by the input iterator, which is expected. Still, there's a lot of cool things on the
Internet that you _can_ make use of.

We won't go into detail here, but you are encouraged to go online and look at all of the techniques
other people have used for this challenge. You can also read the
[Rust Performance Book](https://nnethercote.github.io/perf-book/). Third-party crates like `rayon` can be very useful if used correctly! Be careful when trying to use SIMD, as you will be graded on the Gradescope Docker containers.

After making a good-faith first attempt, you can also look at our reference solution, but note that it is purposefully not very well optimized.

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
Gradescope. For this lab, you also need to add the `Cargo.toml` file to that zip folder. Please
reach out to us if you are unsure how to do this!

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
