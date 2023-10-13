# Rust Stuco S24 Outline

</br>

# Course Materials:
- Main Source: [The Rust Programming Langauge (The Rust Book)](https://doc.rust-lang.org/book/)
- The Rust Lang Book [in video format](https://www.youtube.com/playlist?list=PLai5B987bZ9CoVR-QEIN9foz4QCJ0H2Y8) on YouTube
- [Rust By Example](https://doc.rust-lang.org/rust-by-example/index.html)
- [The Rustonomicon](https://doc.rust-lang.org/nomicon/)
- [The Rust Reference](https://doc.rust-lang.org/reference/index.html)
- Rust for Rustaceans - PDF available on O'Reilly for CMU students
- [Exercism](https://exercism.org/tracks/rust)
- [`rustlings`](https://github.com/rust-lang/rustlings)

</br>

# Schedule

## Week 1: Introduction
- Why Rust?
- Cargo basics
- Rust Syntax
- Installing Rust


## Week 2: Ownership
- Ownership
- `Vec` and `String` basics
- References and Borrowing
- Slices


## Week 3: Structs and Enums
- Ownership review
- Structs
- Enums


## Week 4: Standard Collections and Generics
- Vectors
- Strings
- Maps
- Generics


## Week 5: Error handling and Traits
- Error Handling
- Traits


## Week 6: Cargo, Modules, Crates, and Testing
- Crates and Modules file system layout
- Unit Testing and Integration Testing

</br>
</br>

# TODO

## Week 7: Lifetimes
Chapter 10.3 + probably need way more

- Very important, need to get this right


## Week 8: Iterators, Closures, and Advanced Functional
Chapter 13


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


## Week 10 - 13: Misc Advanced Rust
- Parallelism & Concurrency
    - Locking
    - Message Passing
    - Async/Await
- Essential Rust Crates
    - `no_std`
        - `rand`
        - `time`
    - `std`
        - `log`
        - `tracing`
        - `anyhow`
        - `clap`
        - `rayon`
    - Frameworks
        - `serde`
        - `criterion`
        - `tokio`
- Macros
- Unsafe
    - FFI



