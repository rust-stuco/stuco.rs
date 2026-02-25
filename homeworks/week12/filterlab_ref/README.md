### 98-008: Intro to the Rust Programming Language

# Filter Lab

The goal of this homework is to tie together several of the things we have talked about over the
past few weeks. This includes code-related topics (such as structs, standard collections, and
generics), but we also want to make sure you understand some of the development tools that Rust
provides (such as crates, modules, and libraries).

For this lab, we will only give you testing and benchmarking code. You should see a `lib.rs` file in
the `src/` directory. We want you to structure your code however you would like! If you are unsure,
you can write all of your code in the `lib.rs` file and then move things out into different files as
necessary. If you have trouble setting things up, you can always reference the reference solution on
our [GitHub](https://github.com/rust-stuco/homeworks).

We will additionally give you access to the `Cargo.toml` file for the first time! There are many
ways to abuse this. For example, you could import someone else's implementation of a bloom filter,
or just import someone else's implementation of a bit vector.

**You are _not allowed_ to import crates that complete this entire lab with minimal effort on your**
**part. You _are_ allowed to import crates that augment your own implementation in some way.**
This includes crates that have "better" hashing functions or even crates with SIMD support.

We will be checking these manually. If you have any questions about this, or if you are wondering
if a crate is allowed, please let us know!

# Bloom Filters

In this lab, you will implement a bloom filter! Bloom filters are arguably one of the most important
probabilistic data structures in all of Computer Systems (and perhaps even Computer Science).

If you are unfamiliar with bloom filters, there are great resources online explaining how they work.
Here are three of many good explanations:

-   [YouTube Video](https://youtu.be/kfFacplFY4Y?si=3gYtD9HBE3_mWOh1)
-   [System Design Article](https://systemdesign.one/bloom-filters-explained/)
-   [Brilliant Article](https://brilliant.org/wiki/bloom-filter/)

TLDR; Bloom filters comprise of 2 main components:

-   The bit vector that stores the set bits
-   The hashing scheme, which is a combination of:
    -   The "class" of hash functions
    -   The number of times you need to hash each element

## Bit Vector

Since we require 15-122, we will assume you understand how to implement a bit vector. We would
recommend implementing the bit vector with a vector of bytes, or a `Vec<u8>`.

If you are unfamiliar with bit vectors, you can read [this](https://en.wikipedia.org/wiki/Bit_array)
for a high-level summary. Please let us know if you need help with this part! You can always
reference the reference solution on our [GitHub](https://github.com/rust-stuco/homeworks) (as long
as you are not copying and pasting it verbatim).

As a reminder, you are _not_ allowed to import someone else's implementation of a bit vector.

## Bloom Filter

Once you have implemented a bit vector, you must create a `BloomFilter` type that has the following
methods:

```rust,ignore
impl<T: Hash> BloomFilter<T> {
    /// Creates a new `BloomFilter` given the maximum number of elements that will be inserted into
    /// the filter and a bound on the size of the `BloomFilter`'s bitvector.
    pub fn new(num_bits: usize, num_hashes: usize) -> Self { ... }

    /// Inserts an element into the bloom filter.
    pub fn insert(&mut self, elem: &T)  { ... }

    /// Checks if an element might have been previously inserted into the bloom filter.
    pub fn contains(&self, elem: &T) -> bool  { ... }
}
```

Notice that the implementation is generic over `T`. The structure of your `BloomFilter` should
probably look like this:

```rust,ignore
/// An approximate-membership query / probabilistic data structure that supports point lookups.
pub struct BloomFilter<T> {
    <-- other fields go here -->

    /// A type marker used to express that this `BloomFilter` is generic over a single type.
    ///
    /// Note that this means this `BloomFilter` is _not_ allowed to "store" elements of different
    /// types. Each `BloomFilter` instance is specific to a single type (Monomorphization), but the
    /// type itself is generic.
    ///
    /// We could have gone with the approach of making only hashing generic, but for simplicity we
    /// will only allow the `BloomFilter` to track elements of a single type.
    phantom: PhantomData<T>,
}
```

Your bloom filter must be generic over _any_ type `T`, only restricting the _methods_ to `T: Hash`.
If you are confused by what this means, please review Week 5's lecture on Traits, or ask us on
Piazza!

## Where To Start...

You'll definitely want to start by implementing the bit vector. You know how to write unit tests
now, so throw in a few of those to make sure it is correct!

Once you finish the bit vector, you may want to write some pseudo code for `insert` and `contains`
so that you know what fields your `BloomFilter` struct needs.

Finally, you will have to decide how you want to hash your elements. There are many, many ways to do
this: some are more efficient, some are easier to implement. Good luck!

We are purposefully not giving much guidance for this homework. We want this lab to be an exercise
of writing Rust in the "real world." This means thinking about how to solve the problem yourself,
reading documentation, and using online resources to help you program. You will likely either have
to look up how to hash a value in plain Rust (through the standard library), or you will have to
look up third-party crates that have similar functionality.

With that being said, we also **strongly recommend** trying to limit your use of LLMs to help you in
this lab. We obviously cannot prevent you from using them, but you limit your ability to learn by
relying on something else to write code for you.

# Grading

There are 4 integration tests located in the `tests` directory, and 2 benchmarks located in the
`benches` directory. You should aim to pass all 4 integration tests and perform roughly the same or
better than our reference solution.

On Gradescope, the write benchmark takes approximately 7 microseconds, and the read benchmark takes
approximately 19 microseconds (as of 11/6/24).

Ideally, your implementation should be faster since our reference solution is purposefully slow.
However, we will be lenient with grading this, and as long as you made an effort to try and make it
faster, you will likely get full credit.

We will be manually grading your submission. We will be looking at he robustness of your code (does
your code run without modifications), correctness of your feature, the quality of your tests,
and general style (documentation and comments).

The way we grade this assignment will be different from almost every Computer Science course here at
CMU, where grading is mainly based on an autograder, and maybe a handful of points deductions for
style. Instead, we will be looking at your code wholistically, judging it based on how people expect
you to write code _outside_ of school.

This practice is taken from CMU's Operating Systems course,
[15-410/615](https://www.cs.cmu.edu/~410/).

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
