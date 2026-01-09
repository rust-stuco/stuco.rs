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

# Ownership (Part 1)


---


# Welcome back!

* Homework 1 due in 2 weeks
* You can use 7 late days over the whole semester
* If you spent over an hour on the assignment, please let us know!


---

# Today: Ownership

- Ownership
  - Motivation: Strings
  - Moving, `clone`, and `copy`
- References and Borrowing
- Slices and Owned Types


---


# **Ownership**


---


# Ownership

From the official Rust Lang [book](https://doc.rust-lang.org/book/ch04-00-understanding-ownership.html):

> Ownership is Rust's most unique feature and has deep implications for the rest of the language. It enables Rust to make memory safety guarantees without needing a garbage collector, so it's important to understand how ownership works.

* Today we'll introduce _Ownership_, as well as several related features


---


# What is Ownership?

_Ownership_ is a set of rules that govern how a Rust program manages memory.

* Some languages have garbage collection to manage memory
* Other languages require you to explicitly allocate and free memory
* Rust has a third approach:
  * Manage resources via a set of *rules*

<!-- None of the features of ownership will slow down your program when it's running -->


---


# Ownership Rules

* Each value in Rust has an _owner_
* There can only be one owner at a time
* When the owner goes out of scope, the value will be _dropped_


---


# Ownership Rules

For now, assume the following:

* Variables **own** values
* Variables can store their values in two places:
  * Stored on the stack
  * Stored somewhere else...

<!-- Speaker note:
  "Value on stack" => Last week's data types
  "Value lives elsewhere" => Today's focus
-->


---


# Motivation: Strings

* Last week: Scalar Data Types
  * Values live on the stack
* This week: `String`s
  * Values live somewhere else...

<!-- Speaker note:
Stack-based types have simple memory rules, but strings introduce complexities that motivate our ownership rules
-->


---


# String Literals

Every time you see a text like `"Hello, World!"` surrounded by double quotes, that is a _string literal_.

```rust
fn main() {
    println!("Hello, world!");        // Print a string literal

    let s = "Ferris is our friend";   // Store another string literal
}
```

* String literals are stored in a read-only section of the program binary
  * In other words, these strings literals are known at _compile-time_


---


# Python Strings

Suppose we wanted store a user's input. Python can do this easily!

```python
username1 = input("Enter a short name: ")     # Could be "Bob"
username2 = input("Enter a long name: ")      # Could be "Bartholomew"

# Python strings can be any length!
assert len(username1) == 3 and len(username2) == 11
```

* In Python, we don't have to know the length of the string beforehand
* How would we do this in Rust?


---


# Problem: String Literals are Immutable

String literals in Rust must be known at compile-time, so we cannot use them for this type of program.

```rust
fn main() {
    let username1: <???> = ask_for_user_input();
    let username2: <???> = ask_for_user_input();
}
```

* We don't know size of `username` at compile time
* These strings must be _dynamically sized_


---


# The `String` Type

In addition to string literals, Rust has another string type called `String`.

* `String` manages data allocated on the _heap_
* We use `String` for managing string data where we do not know the size of the string at compile-time

<!-- Rust has a lot of string types! -->


---


# `String` Example

You can create a `String` from a string literal using `String::from()`.

```rust
let s = String::from("hello");
```


---

# `String` Example

This kind of string _can_ be mutated:

```rust
let mut s = String::from("hello");

s.push_str(", world!"); // push_str() appends a literal to a String

println!("{}", s); // This will print `hello, world!`
```

---

# Problem: When is `String` valid?

* String literals are hardcoded into the executable
  * Always valid ✅
* On the other hand, `String`s are dynamically allocated on the _heap_
    * Must request memory from the allocator at _runtime_
    * Must return the memory when we're done using it
    * Who's responsible for this?

<!--
String literals are **literally** hardcoded into the executable

As for `Strings`
- Must return memory, can't hog memory
- We cannot place a blob of memory into the binary for each piece of text whose size is unknown at
  compile time, and whose size might change while running the program.
- If we had a garbage collector, it would do this for us
- If we had to manually do this, we _will_ make a mistake
-->


---


# Python and Java's Proposal: Garbage Collection

In Python and Java, the _runtime_ is responsible for managing memory.

* The runtime is a system that provides services while the program runs
  * Among these services is the garbage collector
    * The garbage collector finds unused memory and cleans it up
  * Runtime processes can be inefficient!


---


# C's Proposal: Manual Memory Management

In C, the _programmer_ is responsible for returning memory.

- `malloc`: request memory from allocator
- `free`: return memory to allocator

```c
int main() {
    char *s = (char *)malloc(sizeof("hello"));  // Allocate memory for `s`
    strcpy(s, "hello");                         // Set `s` to "hello"
    free(s);                                    // Done with `s`, free it!
}
```

---


# C's Proposal: Manual Memory Management

However, the programmer must pair exactly one `malloc` with exactly one `free`.

* If we forget to `free`, we leak memory
* If we `free` too early, we have an invalid variable
* If we `free` twice, that's a "double free" bug
* Undefined behavior!!! ☠️


---


# C's Proposal: Memory Footgun

Using `malloc` and `free` can lead to all sorts of undefined behavior.

* Unless you are the perfect developer...
* Who _never_ writes a bug...
* You're bound to shoot yourself in the foot!

<!-- Because developers who never write bugs definitely exist -_- -->


---


# Rust's Proposal: Compile-Time Memory Safety

In Rust, the _compiler_ is responsible for returning memory.

* Compiler marks places to return memory
* It would be great if the compiler knew:
  * When the variable needs memory
  * When the memory is no longer needed
* Idea: **What if we tied heap allocations to the scope of a variable?**


---


# Rust's Proposal: Compile-Time Memory Safety

Every variable has a _scope_.

```rust
{
    let s = String::from("hello"); // s comes into scope
} // s goes out of scope
```

* There are two important points here:
    * When `s` comes _into_ scope, it is valid
    * When `s` goes _out_ of scope, it is invalid


---


# Rust's Proposal: Compile-Time Memory Safety

Memory is returned once the variable that owns it goes out of scope.

```rust
{
    let s = String::from("hello"); // s comes into scope
} // s goes out of scope
```

* When `s` comes into scope, it gets memory from the allocator
* When `s` goes out of scope, its memory is freed
    * Rust automatically calls the function `drop` on `s` when we reach the closing bracket

<!-- This is the RAII pattern in C++ -->
<!-- This might seem simple, but it has profound implication on the way we write code in Rust -->


---


# Example: `String` "copying"

```rust
let s1 = String::from("hello");
let s2 = s1;
```

What is this code doing?

* Bind the `String` containing `"hello"` to `s1`
* Now what?
    * Do we make a copy of the `String`?
    * What does a copy actually mean in this case?

<!--
What does a copy even mean?
Does it mean an alias or a deep copy?
The String might be massive, do we really want to make a deep copy?
Before we make this decision, we should understand what the String looks like in memory
-->


---


# `String` Data Layout

![bg right:50% 90%](../images/String_layout.svg)

```rust
let s1 = String::from("hello");
```

* A `String` is made up of 3 fields:
    * A pointer to the characters somewhere in memory
    * A length
    * A capacity
* Left diagram is on the stack
* Right diagram is on the heap


---


# Pointer Aliasing 😨

![bg right:50% 85%](../images/String_alias.svg)

```rust
let s1 = String::from("hello");
let s2 = s1;
```

One way to handle this case is:

* When we assign `s1` to `s2`, only the stack data is copied
* We do _not_ create a copy of the contents of the `String`
* Also known as a "shallow copy" in some languages

<!--
Shallow copy gets away from the problem of having to recreate the entire string
-->


---


# Pointer Aliasing ☠️

![bg right:50% 85%](../images/String_alias.svg)

```rust
let s1 = String::from("hello");
let s2 = s1;
```

Suppose Rust handled this case with a shallow copy.
* Following Rust's scope rules, what would happen if we tried to drop both `s1` and `s2`?
    * Double free! 🪦
* How can this be prevented?


---


# Enforcing Single Ownership

![bg right:50% 85%](../images/String_move.svg)

To ensure memory safety, after the second line, `s1` is no longer valid.

```rust
let s1 = String::from("hello");
let s2 = s1; // s1 is no longer valid
```

* _Grayed out portion is no longer accessible to the program_


---


What happens if we try to use `s1` after it is invalid?

```rust
let s1 = String::from("hello");
let s2 = s1;
println!("{}, world!", s1);
```

```
error[E0382]: borrow of moved value: `s1`
  |
2 |     let s1 = String::from("hello");
  |         -- move occurs because `s1` has type `String`,
               which does not implement the `Copy` trait
3 |     let s2 = s1;
  |              -- value moved here
4 |
5 |     println!("{}, world!", s1);
  |                            ^^ value borrowed here after move
  |
```


---


# Move Semantics

```rust
let s1 = String::from("hello");   // Create `s1`
let s2 = s1;                      // Move `s1` into `s2`
// println!("{}, world!", s1);    // `s1` is now invalid!
```

* Rust calls this shallow copy plus invalidation a _move_
* We _moved_ `s1` into `s2`
  * Hence `s1` can no longer be accessed


---

# Moving vs Cloning

```rust
let s1 = String::from("hello");
let s2 = s1;
```

* What if we _wanted_ to copy all of the data?
  * Known as deep copying or cloning
* Making Rust implicitly deep copy all data would solve the problem
  * But it would get expensive, quickly


---


# `Clone`

If we do want to deep copy, we can use a method called `clone`.

```rust
let s1 = String::from("hello");
let s2 = s1.clone();

println!("s1 = {}, s2 = {}", s1, s2);
```

```
s1 = hello, s2 = hello
```

* This copies _all_ of the data contained in `s1`, both on the heap and the stack


---


# `Clone`

```rust
let s1 = String::from("hello");
let s2 = s1.clone();

println!("s1 = {}, s2 = {}", s1, s2);
```

```
s1 = hello, s2 = hello
```

* In Rust, cloning must be explicitly performed by the programmer
  * This is very intentional, to avoid accidental performance overhead
* We'll talk more about methods next week!


---


# What About Integers?

Based on the ownership rules we have stated, this code should not work.

```rust
let x = 5;
let y = x;

println!("x = {}, y = {}", x, y);
```

```
x = 5, y = 5
```

* `x` is still valid, but it looks like we moved it into `y`
* Didn't we just say that this wasn't allowed?!



---


# `Copy`

```rust
let x = 5;
let y = x;
```

* Types such as integers have a size known at compile time
* Data is stored either in registers or on the stack
* Copies of integers are very quick to make
* There is no difference between a shallow copy and a deep copy here
  * So why not clone implicitly?

<!---
Copies of integers are quick to make => register copy
-->


---


# `Copy`

Certain types are annotated with a `Copy` trait, which allows implicit copying instead of moving.

Types that are `Copy`:
* All numeric types, including integers (`i32`) and floating points (`f64`)
* The boolean type (`bool`)
* The character type (`char`)
* Tuples, if they only contain types that are `Copy`
    - `(i32, i32)` is `Copy`, but `(i32, String)` is not

<!--
Don't explain traits yet!
-->


---


# Ownership and Functions

Passing a variable to a function behaves just as assignment does.

Passing a `String`:

```rust
fn main() {
    let s = String::from("hello");
    takes_ownership(s);
} // Because `s`'s value was moved, `s` is not dropped

               // `some_string` comes into scope
fn takes_ownership(some_string: String) {
    println!("{} is mine now!", some_string);
} // `some_string` goes out of scope and `drop` is called.
  // The backing memory is freed.
```

---

# Ownership and Functions

What if we tried to use a value after a function takes ownership of it?

```rust
let s = String::from("hello");
takes_ownership(s);
println!("{} is invalid now!", s);
```

```
error[E0382]: borrow of moved value: `s`
 --> src/main.rs:4:36
  |
2 |     let s = String::from("hello");
  |         - move occurs because `s` has type `String`,
              which does not implement the `Copy` trait
3 |     takes_ownership(s);
  |                     - value moved here
4 |     println!("{} is invalid now!", s);
  |                                    ^ value borrowed here after move
```

---


# Ownership and Functions

`Copy` are copied directly into the function parameter:

```rust
fn main() {
    let x = 5;
    makes_copy(x);
    println!("Here is {} again!", x); // x is still valid!
}

fn makes_copy(some_integer: i32) {
    println!("{} just got copied", some_integer);
}
```

```
5 just got copied
Here is 5 again!
```


---


# Return Values and Scope

Returning values can also transfer ownership back to the caller.

```rust
fn main() {
    let s1 = gives_ownership();
    println!("{}", s1); // s1 is valid, we have taken ownership!
}

fn gives_ownership() -> String {
    let some_string = String::from("inside `gives_ownership`");

    some_string // `some_string` is returned and is moved out to the
                // calling function
}
```

```
inside `gives_ownership`
```


---


# Return Values and Scope

Here is another example where a function takes ownership and gives it back:

```rust
fn main() {
    let s2 = String::from("hello");
    let s3 = takes_and_gives_back(s2);
    println!("{}", s3);
} // Here, `s3` goes out of scope and is dropped.
  // `s2` was moved, so nothing happens to `s2`.

fn takes_and_gives_back(a_string: String) -> String {
    a_string  // a_string is returned and
              // moves out to the calling function
}
```


---


# Recap: Ownership

* Ownership rules:
  * Each value in Rust has an _owner_
  * There can only be one owner at a time
  * When the owner goes out of scope, the value will be _dropped_
* With just ownership, we can either move, copy, or clone
  * Moving and copying has no overhead
  * Cloning is expensive


---


# Moving is somewhat tedious

```rust
fn main() {
    let s1 = String::from("hello");
    let (s2, len) = calculate_length(s1);
    println!("The length of '{}' is {}.", s2, len);
}
fn calculate_length(s: String) -> (String, usize) {
    let length = s.len();
    (s, length)
}
```

* If we want to give a function some data, it seems we need to _move_ the data into the function
* To get it back, it seems we need to also return the data back every time
* _What if we want to let a function use a value but not take ownership?_


---


# **References and Borrowing**


---


# References

* Moving into and returning data from a function is a lot of work
* Rust has a feature specifically for using a value without transferring ownership called _references_
* We can share memory using these _references_

<!-- Especially because it is a common concept in programming -->


---


# Reference with `&`

Instead of moving a value into a function, we can provide a _reference_ to the value. We use `&` to define a reference to a value.

```rust
fn main() {
    let s1 = String::from("hello");
    let len = calculate_length(&s1);
    println!("The length of '{}' is {}.", s1, len);
}
fn calculate_length(borrowed: &String) -> usize {
    borrowed.len()
}
```

* The `&s1` syntax lets us create a variable that _refers_ to the value of `s1`
* `&String` means the type of the argument is a reference to a `String`

<!--
Make sure to highlight the `&s1` and the `&String` with a laser pointer.
-->


---


# References as Function Arguments

```rust
                // `borrowed` is a reference to a String
fn calculate_length(borrowed: &String) -> usize {
    borrowed.len()
} // Here, `borrowed` goes out of scope
```

* `borrowed` is a reference to `s1` (i.e. `&s1`)
* We _do not own_ `s1` with just a reference to it
* This means `s1` will _not_ be dropped when `borrowed` goes out of scope
* We call holding a reference _borrowing_


---


# Mutating a Reference

![bg right:25% 75%](../images/ferris_does_not_compile.svg)

What if we want to modify the value of something we've borrowed through a reference?

```rust
fn main() {
    let s = String::from("hello");

    change(&s);
}

fn change(some_string: &String) {
    some_string.push_str(", world");
}
```

---


# Modifying a Reference

We get an error if we try to modify a reference.

```
error[E0596]: cannot borrow `*some_string` as mutable,
              as it is behind a `&` reference

 --> src/main.rs:8:5
  |
7 | fn change(some_string: &String) {
  |                        ------- help: consider changing this
                                   to be a mutable reference: `&mut String`
8 |     some_string.push_str(", world");
  |     ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ `some_string` is a `&` reference,
                                         so the data it refers to cannot
                                         be borrowed as mutable
```

* Just like variables, references are immutable by default


---


# Mutable References

If we want to modify the value that we've borrowed, we must use a mutable reference, denoted `&mut val`.

```rust
fn main() {
    let mut s = String::from("hello");

    change(&mut s);
}

fn change(some_string: &mut String) {
    some_string.push_str(", world");
}
```


---


# Reference Data Layout

![bg right:55% 85%](../images/String_reference.svg)

* In memory, references are just like pointers
* In practice, they have a couple of constraints that make them safer


---


# Reference Constraints

* Mutable references must be exclusive
  * There can only be one mutable reference to a value at a time
* There can never be dangling references

<!--
Outline for the next few slides
-->


---

# Constraint: Mutable References are Exclusive

![bg right:25% 75%](../images/ferris_does_not_compile.svg)

If you have a mutable reference to a value, you can have no other references to that value.

```rust
let mut s = String::from("hello");

let r1 = &mut s;
let r2 = &mut s;

println!("{}, {}", r1, r2);
```


---


# Constraint: Mutable References are Exclusive

```rust
let mut s = String::from("hello");
let r1 = &mut s;
let r2 = &mut s;
println!("{}, {}", r1, r2);
```

```
error[E0499]: cannot borrow `s` as mutable more than once at a time
 --> src/main.rs:5:14
  |
4 |     let r1 = &mut s;
  |              ------ first mutable borrow occurs here
5 |     let r2 = &mut s;
  |              ^^^^^^ second mutable borrow occurs here
6 |
7 |     println!("{}, {}", r1, r2);
  |                        -- first borrow later used here
```


---


# Constraint: Mutable References are Exclusive

* Most languages will let you mutate anything, whenever you want
* If data can be written to from multiple places, the value can become unpredictable
* Making mutable references exclusive can prevent data invalidation and data races at compile time!

<!--
Sometimes people will refer to mutable references as exclusive references, and normal references as
shared references.

Other languages let you mutate values, pointers, variables, etc.

The data races happen when we introduce concurrency, which we'll talk about in the future!

IMPORTANT: The purpose of having 1 mutable reference at a time is not specifically for preventing
data races, since that would only be a problem in multi-threaded code. The main purpose is to make
sure that data isn't invalidated while there are read-only references to it.

See:

- Our example in the next lecture where we show that mutating a `Vec` will cause the memory to be
reallocated, meaning any other references to it would be invalidated
- https://stackoverflow.com/questions/58364807/why-does-rust-prevent-multiple-mutable-references-even-without-multi-threading
-->


---


# Multiple Mutable References

![bg right:25% 80%](../images/ferris_happy.svg)

We are allowed to hold multiple mutable references, just not _simultaneously_.

```rust
let mut s = String::from("hello");

{
    let r1 = &mut s;
} // r1 goes out of scope here, so we can make
  // a new mutable reference with no problems

let r2 = &mut s;
```

* Notice that the scopes of these mutable references do not overlap


---


# Mutable and Immutable References

![bg right:25% 75%](../images/ferris_does_not_compile.svg)

We cannot have both an immutable and mutable reference to the same value.

```rust
let mut s = String::from("hello");

let r1 = &s; // no problem
let r2 = &s; // no problem
let r3 = &mut s; // BIG PROBLEM

println!("{}, {}, and {}", r1, r2, r3);
```

---


# Mutable and Immutable References

```
error[E0502]: cannot borrow `s` as mutable because
              it is also borrowed as immutable
 --> src/main.rs:6:14
  |
4 |     let r1 = &s; // no problem
  |              -- immutable borrow occurs here
5 |     let r2 = &s; // no problem
6 |     let r3 = &mut s; // BIG PROBLEM
  |              ^^^^^^ mutable borrow occurs here
7 |
8 |     println!("{}, {}, and {}", r1, r2, r3);
  |                                -- immutable borrow later used here
```


---


# Mutable and Immutable References

Note that exclusivity rules only apply for references whose scopes overlap.

```rust
let mut s = String::from("hello");

let r1 = &s; // no problem
let r2 = &s; // no problem
println!("{} and {}", r1, r2);
// variables r1 and r2 will not be used after this point

let r3 = &mut s; // no problem
println!("{}", r3);
```

---


# Mutable and Immutable References

```rust
let mut s = String::from("hello");
let r1 = &s; // no problem
let r2 = &s; // no problem
println!("{} and {}", r1, r2);

let r3 = &mut s; // no problem
println!("{}", r3);
```

* The scope of a reference starts when it is initialized
* The scope of a reference **ends at the last point it is used**
* The specific term for reference scopes are _lifetimes_
    * We'll talk about lifetimes in a future week!


---


# Constraint: No Dangling References

![bg right:25% 75%](../images/ferris_does_not_compile.svg)

The Rust compiler guarantees that references will never be invalid, which means it will not allow dangling references.

```rust
fn main() {
    let reference_to_nothing = dangle();
}

fn dangle() -> &String {
    let s = String::from("hello");

    &s
}
```


---


# Constraint: No Dangling References

```
error[E0106]: missing lifetime specifier
 --> src/main.rs:5:16
  |
5 | fn dangle() -> &String {
  |                ^ expected named lifetime parameter
  |
  = help: this function's return type contains a borrowed value,
    but there is no value for it to be borrowed from
help: consider using the `'static` lifetime
<-- snip -->
```

Focus on this line:
> help: this function's return type contains a borrowed value, but there is no value for it to be borrowed from

<!--
Implemented using lifetimes.

Make sure to point out the 'static part, which is actually a bad error message!
-->


---


# Reference Constraints

* Mutable references are exclusive:
  * At any given time, you can have either one mutable reference or any number of immutable references
      * A book being read by multiple people is fine
      * If multiple people write, they may overwrite each other's work
      * References are similar to Reader-Writer locks
* There can be no dangling references, references must always be valid

<!-- Make sure to point out in lecture that a reference is an explicit TYPE -->


---


# The Borrow Checker

The _Borrow Checker_ enforces the ownership and borrowing rules by checking:

* That all variables are initialized before they are used
* That you can't move the same value twice
* That you can't move a value while it is borrowed
* That you can't access a place while it is mutably borrowed (except through the mutable reference)
* That you can't mutate a place while it is immutably borrowed
* and more...


---


# **Slices**


---


# Slices

- _Slices_ let you reference a contiguous sequence of elements in a collection rather than the whole collection
* A slice is similar to a reference, so it does not have ownership


---


# Slices


Suppose we want to write this function:

```rust
fn first_word(s: &String) -> ?
```

* Find the first space and return all the characters before it
* What type should we return?

<!--
Could return an index, but that is boring,
and what would happen if we wanted to return the second word? Two indices?
-->


---


# String Slices

A _string slice_ is sometimes a reference to part of a `String`, and it looks like this:

```rust
let s = String::from("hello world");

let hello = &s[0..5];
let world = &s[6..11];
```

* `hello` contains the first 5 characters of `s`
* `world` contains the 5 characters starting at the 6th index of `s`



---


# String Slices

![bg right:50% 80%](../images/str_slice.svg)

```rust
let s = String::from("hello world");

let hello = &s[0..5];
let world = &s[6..11];
```

* A string slice stores a pointer to memory and a length

<!--
Be clear that the `hello` variable here is not shown, just `s` and `world`.
-->


---


# String Slices

You can shorthand ranges with the `..` syntax.

```rust
let s = String::from("hello");

let slice = &s[0..2];
let slice = &s[..2];

let len = s.len();
let slice = &s[3..len];
let slice = &s[3..];

let slice = &s[0..len];
let slice = &s[..];
```

<!--
Same rules as loops
-->

---


# String Literals are Slices

Recall that we talked about string literals being stored inside the binary.

```rust
let s: &str = "Hello, world!";
```

* The type of `s` here is `&str`: it's a slice pointing to that specific point of the binary with type `str`
* String literals are immutable
  * Their `&str` immutable reference type reflects that

<!--
The above is technically incorrect, it should be `&'static str`, but we're going to ignore that for now.
-->


---


# Owned Types

* String slices and string literals are immutable because they are a special type of immutable reference
* String is an owned type
  * i.e. a type that has an owner
* Another owned type is a _vector_


---


# Vectors

A _vector_ allows you to store a collection of values contiguously in memory.

You can create a vector with the method `new`:

```rust
let v: Vec<i32> = Vec::new();
```

* Internally, a `Vec` is a dynamically sized array stored on the heap
* All values in the `Vec` must be of the same type
* The `<i32>` just means that the vector stores `i32` values
  * We'll talk more about this `<>` syntax in week 4!


---


# Updating a `Vec`

To add elements to a `Vec`, we can use the `push` method.

```rust
let mut v = Vec::new();

v.push(5);
v.push(6);
v.push(7);
v.push(8);

println!("{:?}", v);
```

```
[5, 6, 7, 8]
```

---


# `vec!` Macro

Rust provides a _macro_ to create vectors easily in your programs.

```rust
let v = vec![1, 2, 3];

println!("{:?}", v);
```

```
[1, 2, 3]
```

* Briefly: Macros are a special type of function
  * They can take in a variable number of arguments


---


# Reading Elements of Vectors

You can index into a vector to retrieve a reference to an element.

```rust
let v = vec![1, 2, 3, 4, 5];

let third: &i32 = &v[2];
println!("The third element is {}", third);
```

* Note that Rust will panic if you try to index out of the bounds of the `Vec`

<!--
There is also a get method, but we will talk about that more in week 4
Also note that we don't technically need the & here because i32 is Copy
-->


---


# More `Vec<T>` to come...

We will talk more about `String` and `Vec<T>` in week 4!


---


# Homework 2

* The second homework consists of 12 small ownership puzzles
  * Refer to the `README.md` for further instructions
  * Always follow the compiler's advice!
* We **_highly_** recommend reading the Rust Book chapter on [ownership](https://doc.rust-lang.org/book/ch04-01-what-is-ownership.html)
  * Ownership is a _very tricky concept_ that affects almost every aspect of Rust, so understanding it is key to writing more complex Rust code
* Try your best to understand Ownership _before_ attempting the homework


---


# Next Lecture: Structs and Enums

![bg right:30% 80%](../images/ferris_happy.svg)

Thanks for coming!

<br>

_Slides created by:_
Connor Tsui, Benjamin Owad, David Rudo,
Jessica Ruan, Fiona Fisher, Terrance Chen
