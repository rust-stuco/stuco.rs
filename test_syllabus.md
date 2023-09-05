# Rust Stuco S24 Initial Plan

Spring 2024 tentative syllabus

# Materials:
- Main Source: [The Rust Programming Langauge (The Rust Book)](https://doc.rust-lang.org/book/)
- [Rust By Example](https://doc.rust-lang.org/rust-by-example/index.html)
- [The Rustonomicon](https://doc.rust-lang.org/nomicon/)
- [The Rust Reference](https://doc.rust-lang.org/reference/index.html)
- Rust for Rustaceans - pdf on oreilly??
- Exercism??


# Schedule

## Week 1: Introduction
- Why Rust? (15 min)
- Cargo Basics (5 min)
- Syntax (30 min)


## Week 2: Ownership
- Ownership (28 min)
- References and Borrowing (14 min)
- Slices (8 min)


## Week 3: Structs and Enums
- Structs (10 min)
- `impl` Structs and Methods (10 min)
- Enum basics (30 min)


## Week 4: Standard Collections and Generics
- Vectors (8 min)
- Strings (10 min)
- Maps (15 min)
- Generics basics (17 min)


## Week 5: Error handling and Traits
- Error Handling (20 min)
- Trait basics (15 min)
- Derivable Traits (10 min)
- OOP discussion (Chapter 17)??? (remaining time) TODO


## Week 6: Cargo, Modules, Crates, and Testing
- File system (20 min)
- Testing (30 min)


# TODO

## Week 7: Lifetimes
Chapter 10.3 + probably need more

- Very important, need to get this right


## Week 8: Iterators, Closures, and Advanced Functional
Chapter 13


# Remaining topics???
- Trait Objects
- Smart Pointers
- Fearless Concurrency
    - Locking
    - Message Passing
- Async/Await
- Macros
- Unsafe
    - FFI
- API design / Advanced Types?
- Essential Rust Crates
    - `anyhow`
    - `serde`
    - `chrono`
    - `tokio`
    - `log`
    - `tracing`


## Week 9: Smart Pointers and Trait Objects
Chapter 15

- `Box<T>`
- `drop()`
- `Rc<T>`
- `RefCell<T>`
    - `UnsafeCell<T>`
- Memory leaks
- Trait Objects
    - Wide pointers
    - Dynamic dispatch with `dyn`
    - Vtable
    - Probably need more
- Dynamically Sized Types
- Object safety


## Week 10: Fearless Concurrency and Async/Await
Chapter 16


## Week 11: Macros and Unsafe
Chapters 17-19

- Macros
- Unsafe
    - FFI


## Week 12: Final Project
- Essential Rust Crates?
- API design
