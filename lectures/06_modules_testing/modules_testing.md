---
marp: true
paginate: true
theme: rust
class: invert
---


<style>
@import url('https://fonts.googleapis.com/css2?family=Noto+Sans+Mono:wght@100..900&family=Noto+Sans:ital,wght@0,100..900;1,100..900&display=swap');
section {
    font-family: "Noto Sans";
}
code {
    font-family: "Noto Sans Mono";
}
</style>

<!-- _class: communism invert  -->

## Intro to Rust Lang

# Modules and Testing


---


# Today: Modules and Testing


* Modules, Packages, and Crates
    * The `use` keyword
    * Module Paths and the File System
* Testing
    * Unit Testing
    * Integration Testing
* Code Review


---


# Large Programs

As your programs get larger, the organization of the code becomes increasingly important. It is generally good practice to:

* Split code into multiple folders and files
* Group related functionality
* Separate code with distinct features
* _Encapsulate_ implementation details
* _Modularize_ your program


---


# Module System

Rust implements a number of organizational features, collectively referred to as the _module system_.

* **Packages**: A Cargo feature that lets you build, test, and share crates
* **Crates**: A tree of modules that produces a library or executable
* **Modules**: Lets you control the organization, scope, and privacy of paths
* **Paths**: A way of naming an item, such as a struct, function, or module


<!--
Paths are what we've been calling namespaces this whole time basically
-->

---


# **Packages and Crates**


---


# Crate

A _crate_ is the smallest amount of code that the Rust compiler considers at a time.

* The equivalent in C/C++ is a _compilation unit_
* Running `rustc` on a single file also builds a crate
* Crates contain modules
    * Modules can be defined in other files
    * Paths allow modules to refer to other modules

<!--
You've probably never heard of compilation units---
Think of it as adding a .o to your makefile. When you add it,
the preprocessor will logically pull in all of the headers. The
source c/cxx/cpp file and all of its dependents is one compilation unit.

Explicitly state that in C/C++ each file is treated as a compilation unit, and that is NOT the case in Rust
-->


---



# Crate

There are two types of crates: **binary** crates and **library** crates.

* A binary crate can be compiled to an executable
    * Contains a `main` function
    * Examples include command-line utilities or servers
* A library crate has no `main` function, and does not compile to an executable
    * Defines functionality intended to be shared with multiple projects
* Each crate also has a file referred to as the _crate root_
    * _The Rust compiler looks at this file first, and it is also the root module of the crate (more on modules later!)_


---


# Package

A package is a bundle of one or more crates.

* A package is defined by a `Cargo.toml` file at the root of your directory
    * `Cargo.toml` describes how to build all of the crates
* A package can contain **any number of** binary crates, but **at most one** library crate


---


# `cargo new`

Let's walk through what happens when we create a package with `cargo new`.

```sh
$ cargo new my-project
     Created binary (application) `my-project` package

$ ls my-project
Cargo.toml
src

$ ls my-project/src
main.rs
```

* Creates a new package called `my-project`
* Creates a `src/main.rs` file that prints `"Hello, world!"`
* Creates a `Cargo.toml` in the root directory

<!--
Some stuff has been elided for slide real estate...
-->


---


# `Cargo.toml`

Let's take a look inside the `Cargo.toml`.

```toml
[package]
name = "my-project"
version = "0.1.0"
edition = "2024"

[dependencies]
```

* File written in `toml`, a file format for configuration files
* Notice how there is no explicit mention of `src/main.rs`
* Cargo follows the convention that `src/main.rs` is the root of a _binary_ crate
* Similarly, a `src/lib.rs` file is the root of a _library_ crate

<!--
When we say root, we mean root _modules_

You _can_ have both lib.rs and main.rs
-->


---


# Example: `cargo`

Cargo is itself a Rust package that ships with installations of Rust!

* Contains the binary crate that compiles to the executable `cargo`
* Contains a library crate that the `cargo` binary depends on

<!--
It is typical for binary executables in Rust to be thin wrappers around library crates as that makes
testing the program easier
-->


---


# Aside: Package vs Project vs Program

* "Package" is the only term of these three with a formal definition in Rust
* "Project" is a very overloaded term
    * More meaningful in the context of an _IDE_
* "Program"
    * Ask the mathematicians ¯\\_(ツ)_/¯


---


# **Modules**


---


# Modules

_Modules_ let us organize code within a crate for readability and easy reuse.

* Modules are collections of _items_
    * Items are functions, structs, traits, etc.
* Allows us to control the privacy of items
* Mitigates namespace collisions
* Here is a [cheat sheet](https://doc.rust-lang.org/book/ch07-02-defining-modules-to-control-scope-and-privacy.html) from the Rust Book!


<!--
Generally, a mechanism for encapsulation
-->

---


# Root Module

The root module is in our `main.rs` (for a binary crate) or `lib.rs` (for a library crate).

```sh
$ cargo new restaurant
```

###### src/main.rs
```rust
fn main() {
    println!("Hello, world!");
}
```

<!--
Root module is implicit here, no `mod` keyword
-->

---


# Declaring Modules

We can declare a new module with the keyword `mod`.

###### src/main.rs
```rust
fn main() {
    println!("Hello, World!");
}

mod kitchen {
    // `cook` is defined in the module `kitchen`
    fn cook() {
        println!("I'm cooking");
    }
}
```


---


# Using Modules

To use items outside of a module, we must declare them as `pub`.

###### src/main.rs
```rust
fn main() {
    kitchen::cook();
}

mod kitchen {
    pub fn cook() { println!("I'm cooking"); }

    // Only items internal to the `kitchen` should be able to access this
    fn examine_ingredients() {}
}
```

* By default, all module items are private in Rust

<!--
In fact, generally everything is private by default in Rust
Private by default is very very good
-->


---


# Declaring Submodules

We can declare submodules inside of other modules.

###### src/main.rs
```rust
fn main() {
    kitchen::stove::cook();
}

mod kitchen {
    pub mod stove {
        pub fn cook() { println!("I'm cooking"); }
    }
}
```

* Submodules also have to be declared as `pub mod` to be accessible
* The module system is a tree, just like a file system


---


# Modules as Files

In addition to declaring modules _within_ files, creating a file named `module_name.rs` declares a corresponding module named `module_name`.

```sh
src
├── module_name.rs
└── main.rs
```

* Allows us to represent our module structure in the file system
* Let's try moving the `kitchen` module to its own file!


---


# Modules as Files

###### src/main.rs
```rust
mod kitchen; // The compiler will look for `kitchen.rs`

fn main() {
    kitchen::stove::cook();
}
```

###### src/kitchen.rs
```rust
pub mod stove {
    pub fn cook() { println!("I'm cooking"); }
}

fn examine_ingredients() {}
```

* What about moving the `stove` submodule to its own file as well?


---


# Submodules as Files

We can move the `stove` submodule into a file  `src/kitchen/stove.rs` to indicate that `stove` is a submodule of `kitchen`.

###### src/kitchen.rs
```rust
pub mod stove; // note this still has to be `pub`

fn examine_ingredients() {}
```

###### src/kitchen/stove.rs
```rust
pub fn cook() {
    println!("I'm cooking");
}
```

* `main.rs` is unchanged (omitted for slide real estate)


---


# Alternate Submodule File Naming

We could also replace `src/kitchen.rs` with `src/kitchen/mod.rs`.

###### src/kitchen/mod.rs
```rust
pub mod stove;

fn examine_ingredients() {}
```

###### src/kitchen/stove.rs
```rust
pub fn cook() {
    println!("I'm cooking");
}
```

* The only difference is in which file the `kitchen` module is defined


---


# Alternate Submodule File Naming

In terms of Rust's module system, these two file trees are (essentially) identical.

```sh
src
├── kitchen
│  └── stove.rs
├── kitchen.rs
└── main.rs
```

```sh
src
├── kitchen
│  ├── mod.rs
│  └── stove.rs
└── main.rs
```

* This is a stylistic choice that each instructor has a very strong opinion on
    * Ask at your own peril...

<!--
Connor and also Terrance prefers `mod.rs`
Ben prefers named modules files
David is partial to both based on the project

Connor also believes that `mod.rs` files should only have docs, `use` statements, and type definitions.
-->


---


# File Structure Comparison: Choice 1

```sh
src
├── bathroom
│  ├── mod.rs
│  ├── sink.rs
│  └── toilet.rs
├── dining_room
│  ├── guests.rs
│  ├── mod.rs
│  ├── seats.rs
│  └── tables.rs
├── garden
│  ├── dirt.rs
│  ├── mod.rs
│  ├── plants.rs
│  └── water.rs
├── kitchen
│  ├── dish_washer.rs
│  ├── mod.rs
│  ├── oven.rs
│  └── stove.rs
└── lib.rs
```


---


# File Structure Comparison: Choice 2

```sh
src
├── bathroom
│  ├── sink.rs
│  └── toilet.rs
├── dining_room
│  ├── guests.rs
│  ├── seats.rs
│  └── tables.rs
├── garden
│  ├── dirt.rs
│  ├── plants.rs
│  └── water.rs
├── kitchen
│  ├── dish_washer.rs
│  ├── oven.rs
│  └── stove.rs
├── bathroom.rs
├── dining_room.rs
├── kitchen.rs
├── garden.rs
└── lib.rs
```


---


# File Structure Comparison

Consistency with surrounding codebase is _**always**_ most important!

See discussions:

- [Rust Users Forum](https://users.rust-lang.org/t/module-mod-rs-or-module-rs/122653)
- [Rust Internals Forum](https://internals.rust-lang.org/t/the-module-scheme-module-rs-file-module-folder-instead-of-just-module-mod-rs-introduced-by-the-2018-edition-maybe-a-little-bit-more-confusing/21977/17?u=zirconium-n)
- [Reddit](https://www.reddit.com/r/rust/comments/18pytwt/noob_question_foomodrs_vs_foors_foo_for_module/)

<!--
You don't really need to explain the discussions in here, leave it for interested students to find out on their own
-->


---


# The Module Tree, Visualized

Even with our file system changes, the module tree stays the same!

```rust
crate restaurant
├── mod kitchen: pub(crate)
│   ├── fn examine_ingredients: pub(self)
│   └── mod stove: pub
│       └── fn cook: pub
└── fn main: pub(crate)
```

* We can customize our file structure without changing any behavior!

<!--
Emphasize that the above module tree can be represented by many different file structures
-->


---


# Module Paths

To use any item in a module, we need to know its _path_, just like a filesystem.

There are two types of paths:

* An _absolute path_ is the full path starting from the crate root
* A _relative path_ starts from the current module and use `self`, `super`, or an identifier in the current module
* Components of paths are separated by double colons (`::`)


---


# Paths for Referring to Modules

You may have noticed a path from the previous sequence:

```rust
kitchen::stove::cook();
```

This is saying:
* In the module `kitchen`
    * In the submodule `stove`
        * Call the function `cook`
* This is a path relative to the current module (in this case, the root)


---


# Using Paths

###### src/main.rs
```rust
mod kitchen;

fn main() {
    kitchen::stove::cook();
}
```

* Not too hard to write...


---


# Using Verbose Paths

What if we had a deeper module tree?

###### src/main.rs
```rust
fn main() {
    kitchen::stove::stovetop::burner::gas::gasknob::pot::cook();
    kitchen::stove::stovetop::burner::gas::gasknob::pot::cook();
    kitchen::stove::stovetop::burner::gas::gasknob::pot::cook();
}
```

* A lot more verbose...
    * Especially if we need to write this multiple times


---


# The `use` Keyword

We can bring paths into scope with the `use` keyword.

###### src/main.rs
```rust
mod kitchen;

use kitchen::stove::stovetop::burner::gas::gasknob::pot;

fn main() {
    pot::cook();
    pot::cook();
    pot::cook();
}
```

* It is idiomatic to `use` up to the _parent_ of a function, rather than the function item itself


<!--
It is idiomatic to do it this way, because it makes it clear that the item is not locally defined
-->


---


# More `use` Syntax

We can also import items from the Rust standard library (`std`).

```rust
use std::collections::HashMap;
use std::io::Bytes;
use std::io::Write;
```

* `HashMap` and `Bytes` are structs, and `Write` is a trait
* It is idiomatic to import structs, enums, traits, etc. directly

<!--
The reason this is idiomatic is because you generally shouldn't have multiple different types that
are named exactly the same, and types are usually defined elsewhere anyways.
If you do have types that have identical names, you need to bring in the types relative to their
parent modules anyways.
-->


---


# More `use` Syntax

We can combine those 2 `std::io` imports into one statement:

```rust
use std::collections::HashMap;
use std::io::{Bytes, Write};

use std::io::*; // Also possible!
```

* You could also write `use std::io::*` to bring in everything from the `std::io` module (including `Bytes` and `Write`)
    * Called the "glob operator"
    * Generally not recommended since it clutters the namespace

<!--
The one case where glob is idiomatic is with the `use some_crate::prelude::*` pattern.

It does not actually increase compilation cost, it just makes dealing with namespace collisions
annoying, so it is good practice to only bring in what you actually need.
-->


---


# Aside: Binary and Library Crate Paths

In the past examples, we were using a binary crate (`src/main.rs`). All the same principles apply to using a library crate.

However, if you use _both_ a binary _and_ a library crate, things are slightly different.

```sh
src
├── kitchen
│  ├── mod.rs
│  └── stove.rs
├── lib.rs <- What happens when we add this?
└── main.rs
```


---


# Aside: Binary and Library Crate Paths

```
src
├── kitchen
│  ├── mod.rs
│  └── stove.rs
├── lib.rs
└── main.rs (wants to call functions from lib.rs)
```

Typically when you have both a binary and library crate in the same package, you want to use functions and types defined in `lib.rs` from `main.rs`.

* If you have both a `main.rs` file and a `lib.rs` file, _both_ are crate roots
* So how can we get items from a separate module tree?

<!--
Since both are crate roots, there are technically 2 separate module trees
-->


---


# Accessing Library from Binary

Let's try to refactor our previous example:

###### restaurant/src/lib.rs
```rust
pub mod kitchen; // Now marked `pub`!
```

###### restaurant/src/main.rs
```rust
fn main() {
    ???::kitchen::stove::cook();
}
```

* All files in `src/kitchen` remain unchanged
* What do we put in `???`?

<!--
Make sure the mention that the paths are now relative to outside package directory now
-->


---


# Accessing Library from Binary

We treat our library crate as an _external_ crate, with the same name as our package.

###### restaurant/src/main.rs
```rust
fn main() {
    restaurant::kitchen::stove::cook();
}
```

* Similar to how you would treat `std` as an external crate
* We'll talk about external crates more next week!


---


# The `super` Keyword

We can also construct relative paths that begin in the parent module with `super`.

```rust
crate restaurant
├── mod kitchen: pub(crate)
│   ├── fn examine_ingredients: pub(self)
│   └── mod stove: pub
│       └── fn cook: pub
└── fn main: pub(crate)
```

###### src/kitchen/stove.rs
```rust
pub fn cook() {
    super::examine_ingredients(); // Make sure you do this before cooking!
    println!("I'm cooking");
}
```


---


# Privacy

```rust
mod kitchen: pub(crate)
├── fn examine_ingredients: pub(self)
└── mod stove: pub
    └── fn cook: pub
```
###### src/kitchen/stove.rs
```rust
pub fn cook() {
    super::examine_ingredients(); // Make sure you do this before cooking!
    println!("I'm cooking");
}
```

* `examine_ingredients` does not need to be public in this case
* `stove` can access anything in its parent module `kitchen`
    * Note that privacy only applies upwards, not downwards

<!--
Child modules can access anything the parent module has access to, but not the other way around.
This means child modules can also access any public item in a sibling module.
-->


---


# Privacy of Types

We can also use `pub` to designate structs and enums as public.

```rust
pub struct Breakfast {
    pub toast: String,
    seasonal_fruit: String,
}

pub enum Appetizer {
    Soup,
    Salad,
}
```

* We can mark specific fields of structs public, allowing direct access
* If an enum is public, so are its variants!


---


# Recap: Modules

* You can split a package into crates
    * Crates into modules
        * Modules into items
* You can refer to items defined in other modules with paths
* All module components are private by default, unless you mark them as `pub`


---


# **Testing**


---


# Testing

> Program testing can be a very effective way to show the presence of bugs, but it is hopelessly inadequate for showing their absence.

* Edsger W. Dijkstra, _The Humble Programmer_


---


# Testing

Correctness of a program is complex and not easy to prove.

* Rust's type system helps with this, but it certainly cannot catch everything
* Rust includes a testing framework for this reason!


---


# What is a Test?

Generally we want to perform at least 3 actions when running a test:

1) Set up needed data or state
2) Run the evaluated code
3) Determine if the results are as expected


---


# Writing Tests

In Rust, a test is a function annotated with the `#[test]` attribute.

###### src/lib.rs
```rust
#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        let result = 2 + 2;
        assert_eq!(result, 4);
    }
}
```

* After running `cargo new adder --lib`, this code will be in `src/lib.rs`


---


# Writing Tests

Let's break this down.

```rust
#[test]
fn it_works() {
    let result = 2 + 2;
    assert_eq!(result, 4);
}
```

* The `#[test]` attribute indicates that this is a test function
* We set up the value `result` by adding `2 + 2`
* We use the `assert_eq!` macro to assert that `result` is correct
* We don't need to return anything, since not panicking _is_ the test!


---


# Running Tests

We run tests with `cargo test`.

```
$ cargo test
   Compiling adder v0.1.0 (file:///projects/adder)
    Finished test [unoptimized + debuginfo] target(s) in 0.57s
     Running unittests src/lib.rs (target/debug/deps/adder-92948b65e88960b4)

running 1 test
test tests::it_works ... ok

test result: ok. 1 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.00s

   Doc-tests adder

running 0 tests

test result: ok. 0 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.00s
```


---


# Running Tests

Let's break down the output of `cargo test`.

```
running 1 test
test tests::it_works ... ok

test result: ok. 1 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.00s
```

* We see `test result: ok`, meaning we have passed all the tests
* In this case, only 1 test has run, and it has passed

<!--
Disregard the "0 measured", that is for nightly benchmarking
-->


---


# Documentation Tests

You may have seen something similar to this in your homework:

```
   Doc-tests adder

running 0 tests

test result: ok. 0 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.00s
```

* All of the code examples in documentation comments are treated as tests!
* This is useful for keeping your docs and code in sync


---


# `#[cfg(test)]`

You may have also noticed this `#[cfg(test)]` attribute in your homework:

```rust
#[cfg(test)]
mod tests {
    // <-- snip -->
}
```

* This tells the compiler that this entire module should _only_ be used for testing
* The compiler ignores this module when compiling with `cargo build`


---


# Writing Better Tests

Let's try and be more creative with our tests.

```rust
#[cfg(test)]
mod tests {
    #[test]
    fn exploration() {
        assert_eq!(2 + 2, 4);
    }

    #[test]
    fn another() {
        panic!("Make this test fail");
    }
}
```


---


# Failing Tests

Let's see what we get:

```
$ cargo test

running 2 tests
test tests::another ... FAILED
test tests::exploration ... ok

failures:

---- tests::another stdout ----
thread 'tests::another' panicked at 'Make this test fail', src/lib.rs:10:9
note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace

failures:
    tests::another

test result: FAILED. 1 passed; 1 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.00s

error: test failed, to rerun pass `--lib`
```


---


# Failing Tests


```
failures:

---- tests::another stdout ----
thread 'tests::another' panicked at 'Make this test fail', src/lib.rs:10:9
note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace


failures:
    tests::another

test result: FAILED. 1 passed; 1 failed; <-- snip -->

error: test failed, to rerun pass `--lib`
```

* Instead of `ok`, we get that the result of `tests:another` is `FAILED`


---


# Checking Results

We can use the `assert!` macro to ensure that something is `true`.

```rust
#[test]
fn larger_can_hold_smaller() {
    let larger = Rectangle {
        width: 8,
        height: 7,
    };
    let smaller = Rectangle {
        width: 5,
        height: 1,
    };

    assert!(larger.can_hold(&smaller));
}
```

<!--
Say in lecture that `assert!` will give you a nicer error message
than if you did an if check and panic manually
-->


---


# Testing Equality

Rust also provides a way to check equality between two values with `assert_eq!`.

```rust
#[test]
fn it_adds_two() {
    assert_eq!(4, add_two(2));
}
```


---


# Testing Equality

If `add_two(2)` somehow evaluated to `5`, we would get this output:

```
---- tests::it_adds_two stdout ----
thread 'tests::it_adds_two' panicked at 'assertion failed: `(left == right)`
  left: `4`,
 right: `5`', src/lib.rs:11:9
note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace
```

* You get a nicer error message from `assert_eq!` versus using
`assert!(left == right)`


---


# Custom Error Messages

We can also write our own custom error messages in `assert!`

```rust
#[test]
fn greeting_contains_name() {
    let result = greeting("Carol");
    assert!(
        result.contains("Carol"),
        "Greeting did not contain name, value was `{}`",
        result
    );
}
```


---


# `#[should_panic]`

If you want test that your code (correctly) panics, you can use `#[should_panic]`:

```rust
#[test]
#[should_panic(expected = "not less than 100")]
fn greater_than_100() {
    this_better_be_less_than_100(200);
}
```

* The `#[should_panic]` attribute says that this test expects a panic!
* Adding the `expected = "..."` means we want a specific panic message

<!--
We don't actually use this anymore... But it is helpful to know nonetheless
-->


<!--
---


# Using `Result<T, E>` in Tests

We can also use `Result` in our tests.

```rust
#[test]
fn it_works() -> Result<(), String> {
    if 2 + 2 == 4 {
        Ok(())
    } else {
        Err(String::from("two plus two does not equal four"))
    }
}
```

* The test will now fail if it returns `Err`
* Allows convenient usage of `?` in tests
* Note that you can't use `#[should_panic]` on tests that return a `Result`
-->


---


# Controlling Test Behavior

`cargo test` compiles your code in test mode and runs the resulting test binary.

* By default, it will run all tests in parallel and will not print any test output
* Other testing configurations are available
* _Note that you can run `cargo test --help`, and `cargo test -- --help` for help_

<!--
Formally, "capturing" means that is won't display any `println!`s or error messages.

Parallel stuff leads into next slide...
-->


---


# Running Tests in Parallel

* Suppose each of your tests all write to some shared file on disk
    * All tests write to a file `output.txt`
* They later assert that the file still contains that data they wrote
* You probably don't want all of them to run at the same time!


---


# Test Threads


By default, Rust will run all tests in parallel, on different threads.

You can use `--test-threads` flag to control the number of threads.

```
$ cargo test -- --test-threads=1
```

* Only use this when you actually need to, otherwise the benefits of running tests in parallel are lost

<!--
Take 15-445 if you want to do this safely without losing parallelism!
-->


---


# Showing Output

If you want to prevent the capturing of output, you can use `--no-capture`.

```
$ cargo test -- --no-capture
$ cargo test -- --show-output
```

* `--no-capture` will print the full output of every test that is run
* Using `--show-output` will only show the output of passed tests
* With 1000 tests, this might become verbose!
* If only we could only run a subset of the tests...

<!--
Note that `cargo test` will show the print output of failed tests
-->


---


# Running Tests by Name

Let's say we have 1000 tests, but only one is named `one_hundred`. We can run `cargo test one_hundred` to only run  the `one_hundred` test.

```
$ cargo test one_hundred

running 1 test
test tests::one_hundred ... ok

test result: ok. 1 passed; 0 failed; 0 ignored; 0 measured; 999 filtered out; finished in 0.00s
```

* Notice how there are now `999 filtered out` tests, these were the tests that didn't match the name `one_hundred`


---


# Multiple Tests by Name

`cargo` will actually find _any_ test that matches the name you passed in.

```
$ cargo test add

running 2 tests
test tests::add_three_and_two ... ok
test tests::add_two_and_two ... ok

test result: ok. 2 passed; 0 failed; 0 ignored; 0 measured; 998 filtered out; finished in 0.00s
```

* If you want an exact name, use `cargo test {name} -- --exact`


---


# Ignoring Tests

We can ignore some tests by using the `#[ignore]` attribute.

```rust
#[test]
fn it_works() {
    assert_eq!(2 + 2, 4);
}

#[test]
#[ignore]
fn expensive_test() {
    // code that takes an hour to run
}
```

* If we want to run only ignored tests, we can run `cargo test -- --ignored`
* If we want to run all tests, we can run `cargo test -- --include-ignored`

<!--
This can be useful if you have super specialized tests that need to run by themselves
-->


---


# Test Organization

The Rust community thinks about tests in terms of two main categories: unit tests and integration tests.

* Unit tests test each unit of code in isolation
* Integration tests are external to your library, testing the entire system


---


# Unit Tests

Unit tests are almost always contained within the `src` directory.

* The convention is to create submodules named `tests` for every module you want to test
    * Make sure to add the attribute `#[cfg(test)]`!
* Prevents deploying extra code in production that is only used for testing

<!--
Make sure to briefly re-explain what `#[cfg(test)]` is just in case
-->


---


# Testing Private Functions

You can unit test private functions as long as the module the test lives in has access to it.

```rust
pub fn add_two(a: i32) -> i32 { internal_adder(a, 2) }
fn internal_adder(a: i32, b: i32) -> i32 { a + b }

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn internal() {
        assert_eq!(4, internal_adder(2, 2));
    }
}
```

<!--
In terms of privacy, unit tests are treated just as any other function.

Excerpt from the Rust Book:

There's debate within the testing community about whether or not private functions should be tested directly, and other languages make it difficult or impossible to test private functions.

If you don't think private functions should be tested, there's nothing in Rust that will compel you to do so.
-->


---


# Integration Tests

Integration Tests use your library in the same way any other code would.

* They can only call functions that are part of your library's public API
* Useful for testing if many parts of your library work together correctly

<!--
By "any other code" we mean code that was written by some other developer using
your library crate.

The main difference here is that integration tests don't have any access to private items
-->


---


# Integration Tests

To create integration tests, we need a `tests` directory.

```
adder
├── Cargo.lock
├── Cargo.toml
├── src
│   └── lib.rs
└── tests
    └── integration_test.rs
```

* Notice how `tests` is _outside_ of `src`


---


# Integration Tests

Since we are now external to our own library, we must import everything as if it were a 3rd-party crate.

###### adder/tests/integration_test.rs
```rust
use adder;

#[test]
fn it_adds_two() {
    assert_eq!(4, adder::add_two(2));
}
```

* Note that we don't need to annotate anything with `#[cfg(tests)]`
* We run test files using the name of the integration test file, like
`cargo test --test integration_test`


---


<!--
# Submodules in Integration Tests

As you add more integration tests, you might want to make more files in the `tests` directory to help organize them.

* You can use submodules in the `tests` directory just like in the `src` directory


// Comment this below:
You can treat the `tests` directory almost exactly the same as the `src` directory, and you can
also use the alternate module file naming that we talked about earlier in the lecture.



---


# Submodules in Integration Tests

Using the alternate naming convention with `common/mod.rs` tells Rust not to treat the `common` module as an integration test file.

```
├── Cargo.lock
├── Cargo.toml
├── src
│   └── lib.rs
└── tests
    ├── common
    │   └── mod.rs
    └── integration_test.rs
```


---


# Submodules in Integration Tests

Here is an example of using `common` in an integration test:

```
└── tests
    ├── common
    │   └── mod.rs
    └── integration_test.rs
```

```rust
use adder;

mod common;

#[test]
fn it_adds_two() {
    common::setup();
    assert_eq!(4, adder::add_two(2));
}
```


---
-->


# Integration Tests for Binary Crates

We cannot create integration tests for a binary crate.

* Binary crates do not expose their functions
* This is why most binary crates will be paired with a library crate, even if they don't _need_ to expose any functions


---


# Recap: Testing

* Unit tests examine parts of a library in isolation and can test private implementation details
* Integration tests check that many parts of the library work together correctly
* Even though Rust can prevent some kinds of bugs, tests are still extremely important to reduce logical bugs!


---


# Homework 6

Homework 6 is going to be very different from the previous 5 homeworks!

* You will be following a tutorial from the [official Rust Book](https://doc.rust-lang.org/book/ch12-00-an-io-project.html) (our textbook)
* Your task is to implement a miniature version of the popular CLI tool `grep`
* We are not giving you _any_ starter code
* Your assignment will be manually graded on both correctness and robustness


---


# **Code Review**


---


# Code?

* What is code?
* Why do we write code?


---


# Typical Assignment Workflow

You are probably very used to [this workflow](https://www.cs.cmu.edu/~410/lectures/L01c_Boot.pdf):

* Get assignment handout and code outline
* "Fill in the blanks"
* Debug a few things
* Graded by autograder
* **All done!**
    * Never use this code again
    * Delete it at the end of the semester

<!--
This slide is inspired very directly from 15-410's Boot Camp Slides, page 45
-->


---


# Real World Workflow

In the real world, the workflow is almost the complete opposite:

* Jump into a _massive_ codebase
* Figure out where the "blanks" are to add new features
* Read and debug someone else's code
* Run tests that may not be exhaustive
* **Not done!**
    * Your users and your team are constantly "grading" your work


---


# Homework 6 Grading

We will be manually grading for both correctness and _robustness_.

* By following the tutorial, you will easily get 100% on this assignment
* We will give you up to 100 extra credit points for style, quality, documentation, and robustness
* If you simply copy and paste everything from the tutorial, _you may get around 15/100 extra credit points_
    * We are going to be super strict!
        * Adopting the 15-410 (CMU Operating Systems) mindset

<!--
A perfect 100 is basically the highest quality open source code.
That is basically impossible to get in an hour...
-->


---


# Style

You can follow the [Rust style guide](https://doc.rust-lang.org/nightly/style-guide/) for advice.

Key things:

* Format your code correctly with `cargo fmt`
* Run the `cargo clippy` linter
* Make sure comments are styled correctly
* Use descriptive naming


---


# Documentation

Most of the extra credit grade will come from your documentation.

* Documentation should be [descriptive and succinct](https://rust-lang.github.io/api-guidelines/documentation.html)
    * For this assignment, explain the features of your executable!
* For fellow developers, explain:
    * Design
    * Architecture / structure of your code
    * _Why_ does this function need to exist?


---


# Errors

There are [3 types of errors](https://www.cs.cmu.edu/~410/lectures/L10a_Errors.pdf):

* Hmm...
    * Try to _resolve_
* That's not right...
    * Try to _report_
* Uh-oh
    * Try to help the developer find the _fatal_ problem faster

<!--
Taken directly from 15-410's Error's lecture, page 37
We're being purposefully ambiguous here since there is not time to do an entire lecture on error handling

Note that for the purposes of this assignment, the only type of error that should really happen is reportable errors,
unless someone is adding a VERY interesting feature...
-->


---


# Testing

Write good tests!

* 1000 tests testing the same thing?
* 5 tests testing edge cases?


---


# Next Lecture: Crates, Closures, and Iterators

![bg right:30% 80%](../images/ferris_happy.svg)

Thanks for coming!

<br>

_Slides created by:_
Connor Tsui, Benjamin Owad, David Rudo,
Jessica Ruan, Fiona Fisher, Terrance Chen
