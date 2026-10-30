### 98-008: Intro to the Rust Programming Language

# Bomb Lab

The goal of this homework is to practice **smart pointers and trait objects** in Rust.

This lab consists of 3 phases, each worth 50 points. It is **more conceptual** than other labs you have done in the course, so you won't end up writing much code here. To that end, we ask that you turn off any AI tools now, as it will spoil you the fun (and may even give you a wonky, incorrect answer). If you get stuck, please read the hints below or ask us!

# The Mission

Dr. Evil has gotten tired of terrorizing students with bombs in 15/18-213. Fed up with students easily disassembling code in GDB, Dr. Evil has upgraded from assembly to Rust. Now, there is word that 98-008 will be under attack soon. ~~Lives~~ Points are at stake!

But Dr. Evil can't simply plant a bomb in person. The bomb needs someone who can walk into Gates without raising suspicion... someone like *you*. Sources say Dr. Evil is hiring an intern to do just that.

Armed with your knowledge from this course, you are tasked to go undercover and thwart Dr. Evil's plans!

# Phase 1: Get Hired

You find Dr. Evil's secret job listing on Handshake.

*Henchperson Intern, The Lair (Fall 2026)*

Like every tech company nowadays, The Lair screens applicants with an online assessment. Luckily, Dr. Evil hasn't heard of LeetCode or HackerRank yet, so this step should be quick and easy!

**Complete Dr. Evil's OA on [Gradescope](https://stuco.rs/gradescope)**, then report back here when you are done!

# Phase 2: A Debug Vector

You're hired! On day 1, Dr. Evil hands you your first task. Completing this will earn Dr. Evil's trust and grant you full access to The Lair.

After being defeated by 213, Dr. Evil hates GDB with a burning passion and now attaches `#[derive(Debug)]` to everything. The original plan was to store a status report of every system in The Lair for debugging purposes. But a [`Vec`] can only hold one type, and printing everything one at a time is driving Dr. Evil insane. "The previous intern tried converting everything to [`String`] first. They didn't last long here. I want to see *the real thing*."

In `debug_vec.rs`, **implement [`DebugVec`]**, a vector that holds values of any type that implements [`Debug`] and can print them all at once. Your solution shouldn't take more than a few lines of code.

To test this phase only, run
```sh
cargo test debug
```

**Hint:** What was mentioned in last few questions of phase 1 might be useful here.

**Strong hint (only look if you are *completely* stuck):** Read [this](https://doc.rust-lang.org/book/ch18-02-trait-objects.html#listing-18-4).

# Phase 3: Defuse the Bomb

Impressed with your work, Dr. Evil grants you full access to The Lair and decides that you are ready for the ultimate task: planting the bomb. "Take this to the 98-008 lecture hall, and whatever you do, don't drop it on the way!"

You receive the [`Bomb`]. (Take a look at its code in `bomb.rs`; it is remarkably simple!)

Then, you notice the detail Dr. Evil left out of the job. *The bomb acts like a dead man's switch.* It explodes the instant it is `drop`ped, so there is no safe place to let go of it. Now you know what really happened to the last intern. You are left with no choice but to defuse the bomb now. Dr. Evil chuckles smugly behind you...

In `defuser.rs`, **implement [`defuse`](defuser::defuse) so any bomb that is passed in never explodes.**

**Constraint:** You may NOT use [`Box::leak`] or any functions we haven't mentioned in our lecture slides. No credit will be awarded for solutions that use such functions or circumvent the learning goal of this homework. *In particular, AI often suggests code that violates this constraint. You must solve this one yourself!*

To test this phase only, run
```sh
cargo test defuse
```

**Hint:** What was mentioned in last few questions of phase 1 might be useful here.

**Strong hint (only look if you are *completely* stuck):** Read [this](https://doc.rust-lang.org/book/ch15-06-reference-cycles.html).

# Conclusion

The next lecture goes on without a bang. Everyone is saved! Dr. Evil is defeated once again and vows to target a more error-prone language (*cough cough* [PHP](https://en.wikipedia.org/wiki/PHP2Shell)).

BombLab is new this semester, so we would love to hear your feedback! Let us know on Discord what you thought about this one and if it was the right difficulty for you.

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
