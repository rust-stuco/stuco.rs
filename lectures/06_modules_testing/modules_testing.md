---
theme: default
title: Modules and Testing
author: Rust StuCo
info: |
  Week 6 of Rust StuCo: Modules and Testing.
colorSchema: auto
aspectRatio: 16/9
canvasWidth: 1280
fonts:
  sans: Noto Sans Variable
  mono: Noto Sans Mono Variable
  provider: none
lineNumbers: false
monaco: false
drawings:
  enabled: true
  persist: false
  presenterOnly: true
exportFilename: modules_testing
export:
  timeout: 60000
  withToc: true
layout: default
class: communism
---

## Intro to Rust Lang

# Modules and Testing

---

# Review: Traits

```rust
trait Shape {
    fn area(&self) -> f32;
    fn print(&self) { println!("My area is {}", self.area()); } // Default
}

impl Shape for Rectangle {
    fn area(&self) -> f32 { self.height * self.width }
}
```

* A _trait_ defines behavior that types can implement, but it is not a type itself
* `trait Student: Person` means implementing `Student` also requires `Person`
* `#[derive]` writes the obvious implementation, if every field implements the trait too

---

# Review: Trait Bounds

```rust
fn notify<T: Summary>(item: &T) { ... }
fn notify(item: &impl Summary) { ... } // Same idea, shorter

fn make() -> impl Display { 5 } // Caller only knows it's `Display`
```

* A generic function can only use what its bounds allow
  * `mystery<T>` had no bounds, so all it could do was return `x`
* `impl Trait` as an argument: the _caller_ picks the type
* `impl Trait` as a return type: the _function_ picks one type and hides it

---

# Why Doesn't `largest` Compile?

```rust
fn largest<T>(list: &[T]) -> &T {
    let mut largest = &list[0];
    for item in list {
        if item > largest {
            largest = item;
        }
    }
    largest
}
```

* `T` could be any type, so Rust can't assume that `>` works on it
* Fix: add a bound, `fn largest<T: PartialOrd>(list: &[T]) -> &T`
* `PartialOrd` rather than `Ord`, so `f64` works too (`NaN` makes floats only partially ordered)

---

# Today: Modules and Testing

* Modules, Packages, and Crates
  * The `use` keyword
  * Module Paths and the File System
* Testing
  * Unit Testing
  * Integration Testing

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

* **Paths**: A way of naming an item, such as a struct, function, or module
* **Modules**: Lets you control the organization, scope, and privacy of paths
* **Crates**: A tree of modules that produces a library or executable
* **Packages**: A Cargo feature that lets you build, test, and share crates

<!--
Paths are what we've been calling namespaces this whole time basically
-->

---
layout: section
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
* A package can _define_ **any number of** binary crates, but **at most one** library crate.
  * A package can _depend_ on any number of library crates.

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

# `cargo new`

Let's walk through what happens when we create a package with `cargo new`.

```ansi
$ cargo new my-project
[1m[92m    Creating[0m binary (application) `my-project` package
[1m[92mnote[0m: see more `Cargo.toml` keys and their definitions at <-- snip -->

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
layout: section
---

# **Modules**

---

# Modules

_Modules_ let us organize code within a crate for readability and easy reuse.

* Modules are collections of _items_
  * Items are functions, structs, traits, etc.
* Similar to C++ namespaces
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
cargo new restaurant
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
mod kitchen {
    // `cook` is defined in the module `kitchen`
    fn cook() {
        println!("I'm cooking");
    }
}

fn main() {
    println!("Hello, World!");
}

```

---

# Using Modules

By default, all module items are private in Rust!

To use items outside of a module, we must declare them as `pub`.

###### src/main.rs

```rust
mod kitchen {
    pub fn cook() { println!("I'm cooking"); }

    // Only items internal to the `kitchen` should be able to access this
    fn examine_ingredients() {}
}

fn main() {
    kitchen::cook();
}
```

<!--
In fact, generally everything is private by default in Rust
Private by default is very very good
-->

---

# Declaring Submodules

We can declare submodules inside of other modules.

###### src/main.rs

```rust
mod kitchen {
    pub mod stove {
        pub fn cook() { println!("I'm cooking"); }
    }
}

fn main() {
    kitchen::stove::cook();
}
```

* Submodules also have to be declared as `pub mod` to be accessible
* The module system is a tree, just like a file system

---

# Modules as Files

In addition to declaring modules _within_ files, we can move a module's contents into its own file named `module_name.rs`.

```sh
src
├── module_name.rs
└── main.rs
```

* Allows us to represent our module structure in the file system
* The parent module must still declare it with `mod module_name;`, or the file is ignored
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
├── kitchen
│  ├── dish_washer.rs
│  ├── oven.rs
│  └── stove.rs
├── bathroom.rs
├── dining_room.rs
├── kitchen.rs
└── lib.rs
```

---

# File Structure Comparison

Consistency with surrounding codebase is _**always**_ most important!

See discussions:

* [Rust Users Forum](https://users.rust-lang.org/t/module-mod-rs-or-module-rs/122653)
* [Rust Internals Forum](https://internals.rust-lang.org/t/the-module-scheme-module-rs-file-module-folder-instead-of-just-module-mod-rs-introduced-by-the-2018-edition-maybe-a-little-bit-more-confusing/21977/17?u=zirconium-n)
* [Reddit](https://www.reddit.com/r/rust/comments/18pytwt/noob_question_foomodrs_vs_foors_foo_for_module/)

<!--
You don't really need to explain the discussions in here, leave it for interested students to find out on their own
-->

---

# The Module Tree, Visualized

Even with our file system changes, the module tree stays the same!

```text
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

* An _absolute path_ is the full path starting from the crate root, beginning with `crate` (or the name of an external crate)
* A _relative path_ starts from the current module and uses `self`, `super`, or an identifier in the current module
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
* The equivalent absolute path is `crate::kitchen::stove::cook()`

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

We can also import items from other crates. One common example is the Rust standard library (`std`).

```rust
use std::collections::HashMap;
use std::io::Bytes;
use std::io::Write;
```

* `std` is a crate
* `collections` and `io` are modules
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

# Binary and Library Crate Paths

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

# Binary and Library Crate Paths

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

We treat our library crate as an _external_ crate, named after our package (with any `-` replaced by `_`).

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

```text
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

```text
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
  * Child modules can see private items in their ancestors, but parents cannot see private items in their children

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
layout: section
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
pub fn add(left: u64, right: u64) -> u64 {
    left + right
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let result = add(2, 2);
        assert_eq!(result, 4);
    }
}
```

---

# Writing Tests

Let's break down the test that `cargo new adder --lib` generated.

```rust
#[test]
fn it_works() {
    let result = add(2, 2);
    assert_eq!(result, 4);
}
```

* The `#[test]` attribute indicates that this is a test function
* We set up the value `result` by calling `add(2, 2)`
* We use the `assert_eq!` macro to assert that `result` is correct
* We don't need to return anything, since not panicking _is_ the test!

---

# Why `use super::*`?

```rust
mod tests {
    use super::*; // Bring the parent module's items into scope

    #[test]
    fn it_works() {
        assert_eq!(add(2, 2), 4);
    }
}
```

* `tests` is a child module, so `add` is not in its scope automatically
  * `super` is the parent module, and `*` is the glob import from earlier
* Glob imports are usually discouraged, but `use super::*` in test modules is idiomatic

---

# Running Tests

We run tests with `cargo test`.

```ansi
$ cargo test
[1m[92m   Compiling[0m adder v0.1.0 (/projects/adder)
[1m[92m    Finished[0m `test` profile [unoptimized + debuginfo] target(s) in 0.30s
[1m[92m     Running[0m unittests src/lib.rs (target/debug/deps/adder-ff710a55681752b1)

running 1 test
test tests::it_works ... [32mok[0;10m

test result: [32mok[0;10m. 1 passed; 0 failed; 0 ignored; 0 measured; <-- snip -->

[1m[92m   Doc-tests[0m adder

running 0 tests

test result: [32mok[0;10m. 0 passed; 0 failed; 0 ignored; 0 measured; <-- snip -->
```

---

# Running Tests

Let's break down the output of `cargo test`.

```ansi
running 1 test
test tests::it_works ... [32mok[0;10m

test result: [32mok[0;10m. 1 passed; 0 failed; 0 ignored; 0 measured; <-- snip -->
```

* We see `test result: ok`, meaning we have passed all the tests
* In this case, only 1 test has run, and it has passed

<!--
Disregard the "0 measured", that is for nightly benchmarking
-->

---

# Documentation Tests

You may have seen something similar to this in your homework:

```ansi
[1m[92m   Doc-tests[0m adder

running 0 tests

test result: [32mok[0;10m. 0 passed; 0 failed; 0 ignored; 0 measured; <-- snip -->
```

* Rust code examples in a library's documentation comments are run as tests!
  * Except blocks marked `ignore` or written in another language, like `text`
* This is useful for keeping your docs and code in sync

---

# Writing Documentation Tests

Documentation comments start with `///`, and code blocks inside them are doc tests.

````rust
/// Adds two numbers together.
///
/// ```
/// let sum = adder::add(2, 2);
/// assert_eq!(sum, 4);
/// ```
pub fn add(left: u64, right: u64) -> u64 {
    left + right
}
````

* Doc tests use your library from the outside, so they need the full path `adder::add`
* `cargo test --doc` runs only the doc tests

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

```ansi
$ cargo test

running 2 tests
test tests::exploration ... [32mok[0;10m
test tests::another ... [31mFAILED[0;10m

failures:
<-- snip -->

test result: [31mFAILED[0;10m. 1 passed; 1 failed; <-- snip -->

[1m[91merror[0m: test failed, to rerun pass `--lib`
```

---

# Failing Tests

```ansi
failures:

---- tests::another stdout ----

thread 'tests::another' (26397132) panicked at src/lib.rs:10:9:
Make this test fail
note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace


failures:
    tests::another

test result: [31mFAILED[0;10m. 1 passed; 1 failed; <-- snip -->

[1m[91merror[0m: test failed, to rerun pass `--lib`
```

* Instead of `ok`, we get that the result of `tests::another` is `FAILED`

---

# Checking Results

We can use the `assert!` macro to ensure that something is `true`.

```rust
#[test]
fn larger_can_hold_smaller() {
    let larger = Rectangle { width: 8, height: 7 };
    let smaller = Rectangle { width: 5, height: 1 };

    assert!(larger.can_hold(&smaller));
}
```

* `Rectangle`, `add_two`, and the other helpers in the next few examples come from [Chapter 11 of the Rust Book](https://doc.rust-lang.org/book/ch11-01-writing-tests.html)

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

thread 'tests::it_adds_two' panicked at src/lib.rs:11:9:
assertion `left == right` failed
  left: 4
 right: 5
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

If you want to test that your code (correctly) panics, you can use `#[should_panic]`:

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

* By default, it runs all tests in parallel and captures their output, only showing the output of failed tests
* Other testing configurations are available
* _Note that you can run `cargo test --help`, and `cargo test -- --help` for help_

<!--
Formally, "capturing" means that it won't display any `println!`s or error messages.

Parallel stuff leads into next slide...
-->

---

# Running Tests in Parallel

* Suppose each of your tests all write to some shared file on disk
  * All tests write to a file `output.txt`
* They later assert that the file still contains the data they wrote
* You probably don't want all of them to run at the same time!

---

# Test Threads

By default, Rust will run all tests in parallel, on different threads.

You can use `--test-threads` flag to control the number of threads.

```
cargo test -- --test-threads=1
```

* Only use this when you actually need to, otherwise the benefits of running tests in parallel are lost

<!--
Take 15-445 if you want to do this safely without losing parallelism!
-->

---

# Showing Output

If you want to prevent the capturing of output, you can use `--no-capture`.

```
cargo test -- --no-capture
cargo test -- --show-output
```

* `--no-capture` prints each test's output live, as it runs
* `--show-output` also shows the captured output of passed tests once they finish (failed tests' output is always shown)
* With 1000 tests, this might become verbose!
* If only we could only run a subset of the tests...

<!--
Note that `cargo test` will show the print output of failed tests
-->

---

# Running Tests by Name

Let's say we have 1000 tests, but only one is named `one_hundred`. We can run `cargo test one_hundred` to only run the `one_hundred` test.

```ansi
$ cargo test one_hundred

running 1 test
test tests::one_hundred ... [32mok[0;10m

test result: [32mok[0;10m. 1 passed; <-- snip --> 999 filtered out; finished in 0.00s
```

* Notice how there are now `999 filtered out` tests, these were the tests that didn't match the name `one_hundred`

---

# Multiple Tests by Name

`cargo` will actually find _any_ test that matches the name you passed in.

```ansi
$ cargo test add

running 2 tests
test tests::add_three_and_two ... [32mok[0;10m
test tests::add_two_and_two ... [32mok[0;10m

test result: [32mok[0;10m. 2 passed; <-- snip --> 998 filtered out; finished in 0.00s
```

* If you want an exact match, pass the full path: `cargo test tests::one_hundred -- --exact`

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

Each file in `tests/` is compiled as its own crate, so we must import our library as if it were a 3rd-party crate.

###### adder/tests/integration_test.rs

```rust
use adder::add_two;

#[test]
fn it_adds_two() {
    assert_eq!(4, add_two(2));
}
```

* Note that we don't need to annotate anything with `#[cfg(test)]`
* `cargo test` runs every integration test; to run a single file, use
`cargo test --test integration_test`

---

# Sharing Code Between Integration Tests

What if several test files need the same helper functions? Let's try putting them in `tests/common.rs`:

```ansi
$ cargo test
<-- snip -->
[1m[92m     Running[0m tests/common.rs (target/debug/deps/common-4697aa33ac8d80b0)

running 0 tests
```

* Every file directly inside `tests/` is compiled as its own crate
* So `common` shows up as a test crate, even though it has no tests!

---

# Sharing Code Between Integration Tests

Cargo only compiles files directly inside `tests/` as test crates, so we can use `tests/common/mod.rs` instead.

<div class="columns">
<div>

```text
tests
├── common
│   └── mod.rs
└── integration_test.rs
```

</div>
<div>

###### tests/integration_test.rs

```rust
mod common;

#[test]
fn it_adds() {
    common::setup();
    assert_eq!(adder::add(2, 2), 4);
}
```

</div>
</div>

* Unlike in `src/`, `common.rs` and `common/mod.rs` are **not** interchangeable here!

---

# Integration Tests for Binary Crates

Integration tests cannot `use` items from a binary crate.

* Only library crates expose items that other crates can import
* Integration tests can still run the binary itself: Cargo sets the `CARGO_BIN_EXE_<name>` environment variable to its path
* This is why most binary crates keep their logic in a library crate, with a thin `main.rs`

---

# Recap: Testing

* Unit tests examine parts of a library in isolation and can test private implementation details
* Integration tests check that many parts of the library work together correctly
* Even though Rust can prevent some kinds of bugs, tests are still extremely important to reduce logical bugs!

---
layout: none
---

<EndingSlide next-lecture="The Rust Ecosystem" />
