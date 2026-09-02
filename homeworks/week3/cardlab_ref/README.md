### 98-008: Intro to the Rust Programming Language

# Card Lab

The goal of this homework is to make sure that you understand Rust's Structs and Enums, semantically
known as Algebraic Data Types.

In this homework, you will be modeling and implementing a [`Card`] type, which represents a card in
a standard deck of 52 playing cards (excluding Jokers).

In the first part of this homework, you will model a [`Card`] by defining the fields and enums
needed to represent a playing card. In the second part, you will then implement some comparison
methods on [`Card`] that somebody using your [`Card`] might find useful (that "somebody" could be
you in the future!).

**This writeup is relatively long. Please make sure to read the entire writeup, as it contains
useful information and guidance that will make completing this homework easier!**

The reference solution can be found on our GitHub, but we ask that you first give a solid attempt at
finishing this homework before looking at the solution. If you find yourself stuck, please do not
hesitate to reach out to us for help!

Finally, in a few weeks, we will ask you to use the [`Card`] type that you created in this homework
to help with a future homework that models a game of Poker. So make sure to spend some time
understanding what you have written, as you will need to use it in the future!

# Part 1: Modeling

In the file `src/card.rs`, you will see a unit struct [`Card`] type, as well as several
implementation blocks. In this first part, you will model the structure of this [`Card`] type.

There are many "correct" ways to do this, some more favorable than others. For example, you _could_
model a `Card` like this:

```rust,ignore
pub struct Card {
    name: String,
}

let three_of_spades = Card::new(String::from("three of spades"));
let king_of_spades = Card::new(String::from("king of spades"));
assert!(three_of_spades < king_of_spades);
```

If we were to adopt this model, how would we implement the logic saying that "three of spades" is
less than "king of spades"? It is certainly possible, but it is probably tedious and inefficient.
And what happens if someone writes `Card::new(String::from("fourteen of gold"))`? Now you also have
to handle these edge cases.

What if we instead assigned a unique integer to each card? Since we only have 52 cards, this should
be very easy.

```rust,ignore
pub struct Card {
    value: u8,
}
```

However, how would we assign integers to specific cards? If we say that the integer 2 represents the
two of diamonds, and the integer 3 represents the three of diamonds, what should represent the two
of hearts? Of clubs? And even if you did figure out how to represent them, if someone using your
card type chooses any value other than the 52 you choose for your `Card` type, you have now have to
handle those errors.

Again, using a single integer is possible, but it might be painful to implement and error prone.

## Suggested Modeling

For this lab, we will show you a model for [`Card`] that _eliminates the possibility of incorrect
state_. More specifically, we will model the [`Card`] in such a way that it is impossible to create
an invalid [`Card`].

This model is purposefully inefficient (prioritizing correctness over performance), so if you would
like to try using a model different from the one we suggest, go for it!

Here is the model for [`Card`] that we suggest:

```rust,ignore
pub struct Card {
    rank: Rank,
    suit: Suit,
}
```

Since playing cards are uniquely identified by the pair of `(Rank, Suit)`, we can model our code in
the same way. `Rank` and `Suit` can also be enums:

```rust,ignore
enum Suit {
    /// The Diamond suit, typically represented by a red ♦ symbol.
    Diamond,
    /// The Club suit, typically represented by a black ♣ symbol.
    Club,
    /// The Heart suit, typically represented by a red ♥ symbol.
    Heart,
    /// The Spade suit, typically represented by a black ♠ symbol.
    Spade,
}

enum Rank {
    /// A number card (2-10).
    Number(Number),
    /// A face card (Ace, King, Queen, Jack).
    Face(Face),
}
```

Modeling `Suit` is rather straightforward and obvious, but modeling `Rank` is a little more
complicated. Because a card can have both a number as its value and as well as a face, we choose to
represent the `Rank` type as an enum (sum type) between a `Number` type and a `Face` type.

It is up to you how you want to model `Rank`. If you choose to follow our model, you will need to
define a `Number` and `Face` type. Additionally, it is likely that you will have to write a lot more
boilerplate code (as the tradeoff for being more correct).

Once you have finished modeling [`Card`], you can move on to implementation!

# Part 2: Implementation

You will need to implement 5 public methods on [`Card`] in `src/card.rs`:

1. [`Card::new`]
2. [`Card::suit_name`]
3. [`Card::rank_value`]
4. [`Card::eq`]
5. [`Card::cmp`]

Click on the links above for more information on the function specification.

Note that the last two methods (`eq` and `cmp`) are methods on standard library _traits_. We will
talk more about traits in a few weeks, but for now just treat these methods as normal implementation
methods. As you can probably guess, `eq` implements the `==` operator and `cmp` implements the
remaining comparison operators (`<`, `<=`, `>=`, `>`).

If you go with our modeling above (with the `Suit` and `Rank` types), you will find that you want to
compare two `Rank` types together. You can copy and paste the `impl <Trait> for Card` blocks and
replace the `Card` for `Rank` / `Suit` and implement those methods to allow comparison for the types
you have defined.

In other words, you'll probably want to implement `impl PartialEq for Rank`, `impl Ord for Rank`,
and a few more things. Ask on Piazza or consult the reference solution if you are confused!

_We are aware that there is a very easy way to implement the comparison traits with derived traits.
You technically do not have to manually implement all of the comparison traits yourself._

_However, we believe that it is instructive to manually write out the implementations so that you
**fully understand the code you are writing**. If you **do** know how to use derived traits, you are
free to use them, but we ask that you make sure you understand **exactly** what those macros are
doing._

Once you finish implementing these 5 methods (and pass all of the unit tests in `src/tests.rs`), you
are done!

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
