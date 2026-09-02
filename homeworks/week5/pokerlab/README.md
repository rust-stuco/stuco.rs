### 98-008: Intro to the Rust Programming Language

# Poker Lab

This goal of this homework is to further expose you to working with structs and enums, as well as
auto-implementing the traits found in the standard library.

You will be modeling [Poker](https://en.wikipedia.org/wiki/Poker) by implementing comparison between
sets of cards, or a "hand" of cards.

This lab is a continuation of an earlier lab (Card Lab), in which you modeled a playing card in a
standard 52-card deck. In that lab, you implemented the comparison traits for individual cards. In
this lab, you will now model a comparison among multiple Cards, specifically a [`PokerHand`] type
that represents a "hand" of 5 cards.

We have provided our own definition and implementation of [`Card`] in `src/card.rs`. Take some time
to read over the documentation and code for [`Card`], as it is slightly different than the one we
asked you to implement in the previous lab (the differences will hopefully make implementing
[`PokerHand`] easier).

_You should compare and contrast the `card.rs` file in this assignment to the one from your own Card
Lab. You'll notice that we are no longer manually implementing any of the comparison traits, and
instead we are using the `derive` macro to auto-implement their behavior. Make sure you understand
how the derived traits work!_

## Implementation

You will need to fill in the implementation of 8 methods on the [`Hand`] type (located in
`src/hand/rs`), which is essentially a wrapper struct for an array of 5 [`Card`]s. We use these
methods on [`Hand`] to implement the [`solve`] method on [`PokerHand`]. We have provided the
definition and implementation of [`PokerHand`] to you, and you can read more about this type in its
documentation.

There are 30 test cases, all located inside `tests/poker_tests.rs`. Each test is worth 5 points, for
a total of up to 150 points. By implementing more methods on [`Hand`], you will pass more test
cases. Remember that you can test your code by running `cargo test`!

We would recommend going bottom up: start with implementing the method for detecting a single pair,
then two pairs, etc. You could get 80/150 points just by implementing one method (though we would
encourage you to try and implement all of them for extra credit)! You also do not need to go in
order, and if you want to skip any of the methods that is totally fine.

You are allowed to change any of the fields of the structs defined in `src/hand.rs` if you feel that
it would be easier for you to use different fields. As long as you pass the test cases, mostly
everything is fair game (other than plagiarism). If you have any questions about this, please do not
hesitate to ask us!

[`Card`]: crate::card::Card
[`Hand`]: crate::hand::Hand
[`PokerHand`]: crate::hand::PokerHand
[`solve`]: crate::hand::PokerHand::solve

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
