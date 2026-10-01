### 98-008: Intro to the Rust Programming Language



# Summary Lab

The goal of this homework is to make sure you understand error handling and the `?` operator,
as well as traits and how to implement them. While these are the main topics for this week,
this homework will also heavily test your fluency with Rust's [`String`] and its methods.

You will be implementing two traits on two different types. The two traits are
[`Reader`](crate::Reader) and [`Summary`](crate::Summary), and the two types
are [`EmailReader`](crate::reader::email_reader::EmailReader) and
[`TweetReader`](crate::reader::tweet_reader::TweetReader).

Since string parsing is a bit tricky (especially in Rust), you will get `100` points for completing
the implementation for [`EmailReader`](crate::reader::email_reader::EmailReader)
and `50` points for completing the implementation for
[`TweetReader`](crate::reader::tweet_reader::TweetReader).
This means you only need to do the first part for full credit, and the second part is
extra credit!

If you find yourself spending more than an hour on the first part of the assignment,
please let us know!
This assignment is harder than the previous homeworks,
so there is no shame if you can't complete it in under an hour.
We may give partial credit to honest attempts at the end of the semester,
even if you don't pass any test cases.

That being said, if you have the time,
we still encourage you to try and pass all of the test cases for practice!



# Part 1: Emails

Click this hyperlink to go to the writeup for
[`EmailReader`](crate::reader::email_reader::EmailReader)!



# Part 2: Tweets (Bonus)

Click this hyperlink to go to the writeup for
[`TweetReader`](crate::reader::tweet_reader::TweetReader)!



# Challenge: Command Line Tool (Bonus)

As a sneak peek, for the next homework
we're going to ask you to implement the same command line tool as the one explained in the book
[Chapter 12](https://doc.rust-lang.org/book/ch12-00-an-io-project.html).
Since the book walks you through exactly how to do it,
it will be much more straightforward compared to this homework.

**_If you have ample time to spare_, and you want to get better at Rust,**
**here is a challenge you can do now instead of next week's homework:**

Instead of just running preset tests, you can turn this crate into a binary that _actually_
reads from the file system.

Right now, the `new` functions on all of the `Reader`s never actually get called because
there is no `main` function to call them. If you're up to the challenge (and you have time),
create a command line tool that allows you to read in a tweet or an email from the `data` folder
(either through a pipe or specifying a file) that outputs a summary of what it reads.

You can decide what flags to pass in for specific outputs. And if you'd like, you can
take a look at the [clap](https://docs.rs/clap/latest/clap/index.html) third-party crate
to help you!

We don't have an autograder set up for this, so you'll mostly be on your own.
We will still expect you to write your own test cases and have generally good documentation.

If you do decide to do this challenge, let us know that you did and we'll
manually grade it ourselves to determine how many points you will receive
(the maximum points we would probably be willing to give out is triple that of a normal homework).

To reiterate: Only do this if you have ample time to spare and you want to get better at Rust!
**This is not a trivial task.**

**If you decide to do this challenge, you can skip next week’s homework**, or rather,
you can turn this in next week in place of the official next homework.
It would still be a good idea to read through
[Chapter 12](https://doc.rust-lang.org/book/ch12-00-an-io-project.html)
since you should know how to write your own test cases and parse command line arguments.



# Submission


### Unix

If you are on a unix system, we will try to create a `handin.zip` automatically for you,
**but you will need to have `zip` already installed**.

If you _do not_ have `zip` installed on your system,
install `zip` on your machine or use the CMU Linux SSH machines.
If you need help with this, please reach out to us!

Once you have `zip` installed, we will create the `handin.zip` automatically for you
(_take a peek into `build.rs` if you're interested in how this works!_).

Once you have the `handin.zip` file, submit it to Gradescope.


### Windows

If you are on a windows system, you can zip the `src/` folder manually
and upload that to Gradescope.

Note that you don't _need_ to name it `handin.zip`, you can name it whatever you'd like.



# Collaboration

In general, feel free to discuss homeworks with other students!
As long as you do not copy someone else's work, any communication is fair game.

All formal questions should be asked on Piazza. Try to discuss on Piazza so that
other students can see your questions and answers as well!

You can also discuss on Discord, but try to keep any technical questions on Piazza.



# Feedback

We would like to reiterate that you should let us know if you spent
anywhere in significant excess of an hour on this homework.

In addition, Rust has a notoriously steep learning curve,
so if you find yourself not understanding the concepts,
you should reach out to us and let us know as well ---
chances are, you're not the only one!
