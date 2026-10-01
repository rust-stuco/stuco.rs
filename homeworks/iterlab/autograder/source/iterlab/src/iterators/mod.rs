// Would you have preferred this to be in src/iterators.rs or src/iterators/mod.rs?

//! This module contains 4 iterators that you will implement!
//!
//! Implement this module _before_ the [`hofs`](self) module.
//!
//! You don't _have_ to implement these in the order that we state, but we would recommend
//! following our suggestions.
//!
//!
//! # Fibonacci
//!
//! Implement the [`Fibonacci`](fibonacci::Fibonacci) sequence as an iterator!
//!
//! This is mainly a warmup for the next 3 parts.
//!
//!
//! # Cycle
//!
//! Implement an iterator that takes as input another iterator, and instead of exhausting the input
//! iterator when it reaches its last element, cycle back to the first element.
//!
//! You'll need to figure out what information we need to keep track of to implement this iterator,
//! and store it in [`Cycle`](cycle::Cycle).
//!
//! Don't worry too much about performance here, just make sure it works!
//!
//!
//! # Interleave
//!
//! Given two input iterators, create an iterator that interleaves the outputs of the two iterators.
//! Note that we require the two input iterators to have the same associated `Item` type.
//!
//! Similarly to [`Cycle`](cycle::Cycle), you will need to figure out what to store in the struct
//! [`Interleave`](interleave::Interleave).
//!
//!
//! # Double
//!
//! Given a single input iterator, double every single output!
//!
//! Hint: What is the _easiest_ way to implement this using code you have already written?

/// The Fibonacci sequence! Do this **first**.
pub mod fibonacci;

/// A cycling iterator! Do this **second**.
pub mod cycle;

/// An interleaved iterator! Do this **third**.
pub mod interleave;

/// A doubled iterator! Do this **fourth**.
pub mod double;
