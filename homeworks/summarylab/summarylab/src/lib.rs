#![doc = include_str!("../README.md")]

use std::fs::File;
use std::{
    io,
    io::{BufReader, Read},
};

/// For now, ignore the `Self: Sized` annotation on this trait.
/// If you are interested, you can take a look at
/// [this](https://github.com/pretzelhammer/rust-blog/blob/master/posts/sizedness-in-rust.md#sized-trait)
/// blog.
/// The high-level reasoning is that Rust must be able to determine the size of
/// a return type at compile time. We'll talk more about this when we talk about
/// Trait Objects in week 9!
pub trait Reader
where
    Self: Sized,
{
    /// Creates a new [`Reader`] given a path to a file.
    ///
    /// Internally, reads a file to a [`String`],
    /// and then calls [`Reader::parse`] on that [`String`].
    fn new(file_path: String) -> Result<Self, io::Error> {
        let file = File::open(file_path)?;
        let mut reader = BufReader::new(file);
        let mut file_str = String::new();
        reader.read_to_string(&mut file_str)?;
        Self::parse(file_str)
    }

    /// Creates a new [`Reader`] from a [`String`] of data.
    fn parse(file_str: String) -> Result<Self, io::Error>;
}

/// A trait that shares methods of summarizing text like emails and tweets.
pub trait Summary {
    fn msg_len(&self) -> usize;

    fn summarize(&self) -> String;

    fn get_info(&self) -> String;
}

pub mod reader;
