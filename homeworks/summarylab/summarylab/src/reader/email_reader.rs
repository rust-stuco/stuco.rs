use crate::{Reader, Summary};
use std::io;

/// A struct that represents an email message.
///
/// The [`EmailReader`] struct will model an email message.
/// [`EmailReader`] should have the attributes similar to these:
///  - `subject: String`
///  - `sender: String`
///  - `receiver: String`
///  - `message: String`
///
/// All of these fields should be _private_ (not accessible outside of the struct).
///
/// ---
///
/// The file / [`String`] that the [`EmailReader`] reads should be in the following format:
///
/// ```text
/// Subject: {subject}
/// From: {sender}
/// To: {receiver}
/// {message}
/// ```
///
/// The message can span multiple lines. If no message is found, assume the message is `""`.
/// The subject, sender, and receiver, however, cannot be empty strings.
///
/// ---
///
/// Once you've added the fields to the struct, implement the [`Reader`] trait
/// that has been defined for you in `src/lib.rs` by implementing the
/// [`parse`](Reader::parse) method. Refer to the documentation for more details.
///
/// We also want [`EmailReader`]s to be able to summarize themselves.
/// We'll want to implement the [`Summary`] trait, which is also defined
/// in `src/lib.rs`, which has the [`msg_len`](Summary::msg_len),
/// [`summarize`](Summary::summarize), and [`get_info`](Summary::get_info) methods.
#[derive(Debug)]
pub struct EmailReader {
    _replace_me: (),
}

impl Reader for EmailReader {
    /// Creates a new [`EmailReader`] from a [`String`] of data.
    ///
    /// This method takes in a `file_str`, which is a [`String`] containing the same data
    /// as a file. If the string is not in the correct format, returns an IO error
    /// with the message `"File is not in the correct format"`.
    ///
    /// ---
    ///
    /// This function returns a [`Result<EmailReader, std::io::Error>`].
    /// Here are the cases we need to handle when reading and parsing data from a file:
    /// - The file does not exist or we cannot read it
    /// - The file is not in the correct format
    ///
    /// We've implemented [`Reader::new`] for you to deal with opening a file and
    /// returning a [`std::io::Error`] if something goes wrong. Take a look at the
    /// default implementation and make sure you understand what it is doing.
    /// At a minimum, you should understand what the `?` operator is there for.
    ///
    /// So we are able to leverage the standard library to handle opening and reading files,
    /// and it will tell us if something goes wrong with the IO when we call `new`.
    /// The standard library does not, however, know if the file contains the correct format
    /// of an email we are expecting.
    ///
    /// You will have to create your own error with an error kind
    /// [`InvalidData`](std::io::ErrorKind::InvalidData) and the message
    /// `"File is not in the correct format"`.
    /// See [`std::io::Error::new`] for more details on how to do this.
    fn parse(file_str: String) -> Result<EmailReader, io::Error> {
        // lines is a vector of all the newline-separated lines as &str from the file_str
        let lines: Vec<&str> = file_str.lines().collect();

        todo!()
    }
}

impl Summary for EmailReader {
    /// Returns the length of the message in the email (not the full email).
    fn msg_len(&self) -> usize {
        todo!()
    }

    /// Returns a [`String`] of the form `"{sender}: {message}"`. Maximum 280 characters (truncate).
    fn summarize(&self) -> String {
        todo!()
    }

    /// Returns a [`String`] of the form `"{subject}\nFrom: {sender}, To: {receiver}"`.
    fn get_info(&self) -> String {
        todo!()
    }
}
