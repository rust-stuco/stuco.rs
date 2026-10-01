#![allow(unused_variables, unused_imports)]

use crate::{Reader, Summary};
use std::io;

/// A struct that represents a tweet.
///
/// The [`TweetReader`] struct will model a tweet.
/// [`TweetReader`] should have _at minimum_ the following attributes:
/// - `username: String`
/// - `content: String`
///
/// **A Tweet can also either be a retweet or a reply, on top of being a normal tweet.**
///
/// It is up to you how you want to represent this type of state. You could have two booleans
/// that tell you if it is a retweet or a reply, but make sure that they aren't both true.
/// If only there was a way to _enumerate_ the types of states we could be in...
///
/// _If you choose to create some new type, make sure to annotate it with `#[derive(Debug)]`_
/// _so you can easily debug it later._
///
/// Again, all of these fields should be _private_ (not accessible outside of the struct).
///
/// ---
///
/// The file / [`String`] should be in the following format:
///
/// ```text
/// @{username}
/// "{content}"
/// {"" | "reply" | "retweet"}
/// ```
///
/// Note that content can span multiple lines, and will always be surrounded by double quotes.
/// If there is no content, there will just be a line containing `""`.
///
/// If the tweet is a reply, the word `"reply"` will be on a new line after the content,
/// and if instead the tweet is a retweet, the word `"retweet"`
/// will be on a new line after the content.
///
/// For this assignment, a Tweet can only be one of a normal tweet, a reply, or a retweet.
///
/// ---
///
/// Just like with [`EmailReader`](crate::reader::email_reader::EmailReader),
/// we'll want to implement both [`Reader`] and [`Summary`](
/// for this type as well, both located in `src/lib.rs`.
/// That means we want to implement the [`parse`](Reader::parse),
/// [`msg_len`](Summary::msg_len), [`summarize`](Summary::summarize),
/// and [`get_info`](Summary::get_info) methods.
#[derive(Debug)]
pub struct TweetReader {
    _replace_me: (),
}

impl Reader for TweetReader {
    /// Creates a new [`TweetReader`] from a [`String`] of data.
    ///
    /// This method takes in a `file_str`, which is a [`String`] containing the same data
    /// as a file.
    ///
    /// You're going to want to do the exact same thing as you did for
    /// [`EmailReader`](crate::reader::email_reader::EmailReader) with how you handle errors.
    /// Refer to the documentation for
    /// [`EmailReader`](crate::reader::email_reader::EmailReader)'s version of
    /// [`parse`](crate::reader::email_reader::EmailReader::parse).
    fn parse(file_str: String) -> Result<TweetReader, std::io::Error> {
        // lines is a vector of all the newline-separated lines as &str from the file_str
        let lines: Vec<&str> = file_str.lines().collect();

        todo!()
    }
}

impl Summary for TweetReader {
    /// Returns the length of the content in the tweet (not the full tweet).
    fn msg_len(&self) -> usize {
        todo!()
    }

    /// Returns a string equivalent to `"@{username}: {content}"`. No maximum length.
    fn summarize(&self) -> String {
        todo!()
    }

    /// Returns a string equivalent to
    /// `"Tweet from @{username}"`. with (reply) or (retweet) appended if applicable.
    ///
    /// For example, if `@ferris` made a retweet,
    /// this would return `"Tweet from @ferris (retweet)"`.
    fn get_info(&self) -> String {
        todo!()
    }
}
