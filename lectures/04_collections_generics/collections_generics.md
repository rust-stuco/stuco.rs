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

# Standard Collections and Generics


---


# Today: Standard Collections and Generics

* Rust's `std::collection` Types
    - `Vec<T>`
    - `String`
    - `HashMap<K, V>`
* Generics


---


# Standard Collections


Rust's standard library contains a number of useful data structures called _collections_.

* Most other data types represent a single value, but collections contain multiple values
* Values in collections are (_almost always_) stored on the **heap**
    * The size of each collection does not need to be known at compile time
    * _For more information on other standard library collections, refer to the [documentation](https://doc.rust-lang.org/std/collections/index.html) of the `std::collections` module_

<!--
There might be small string optimizations or small vector optimizations
-->


---


# **Vectors**


---


# Review: Vectors

Vectors are contiguous growable array types.

* Defined as `Vec<T>`
* Short for "Vector"
* Implemented as a dynamic array
    * Similar to Python lists


---


# Review: Vectors

You can create a vector with `new`, and add elements with `push`.

```rust
let mut v = Vec::new();

v.push(5);
v.push(6);
v.push(7);
v.push(8);

println!("{:?}", v);
```


---


# Review: `vec!` Macro

Rust provides a _macro_ to create vectors easily in your programs.

```rust
let v = vec![1, 2, 3];

println!("{:?}", v);
```

```
[1, 2, 3]
```


---


# Review: Indexing

You can index into a vector to retrieve a reference to an element.

```rust
let v = vec![1, 2, 3, 4, 5];

let third_ref: &i32 = &v[2];
println!("The third element is {}", third_ref);

let third: i32 = v[2]; // This is only possible because `i32` is `Copy`
println!("The third element is {}", third);
```

<!--
Might be a good idea to quickly check that students remember what `Copy` means.
-->


---


# `Vec::get()`

You can also use the `get` method to access an optional reference.

```rust
let v = vec![1, 2, 3, 4, 5];

let third: Option<&i32> = v.get(2);
match third {
    Some(third) => println!("The third element is {}", third),
    None => println!("There is no third element."),
}
```

* Using `get` returns `None` if the index is out of bounds instead of panicking


---


# `Vec` and References

![bg right:25% 75%](../images/ferris_does_not_compile.svg)

Recall the rules for immutable and mutable references.

```rust
let mut v = vec![1, 2, 3, 4, 5];

let vec_ref = &v;

v.push(6); // `push` takes a mutable reference!

println!("The vector is: {:?}", vec_ref);
```

* What is wrong with this code?


---


# `Vec` and References

```
error[E0502]: cannot borrow `v` as mutable because it is also borrowed as immutable
 --> src/main.rs:6:5
  |
4 |     let vec_ref = &v;
  |                   -- immutable borrow occurs here
5 |
6 |     v.push(6);
  |     ^^^^^^^^^ mutable borrow occurs here
7 |
8 |     println!("The vector is: {:?}", vec_ref);
  |                                     ------- immutable borrow later used here
```


---


# `Vec` and References

![bg right:25% 75%](../images/ferris_does_not_compile.svg)

What about this new code snippet?

```rust
let mut v = vec![1, 2, 3, 4, 5];

//            vvv
let first = &v[0];

v.push(6);

println!("The first element is: {}", first);
```

<!--
Spell out the fact that we are technically no longer taking a reference to `v` but
something _inside_ `v`.
-->


---


# `Vec` and References

If we try to run this:

```rust
let mut v = vec![1, 2, 3, 4, 5];
let first = &v[0];
v.push(6);
println!("The first element is: {}", first);
```

```
error[E0502]: cannot borrow `v` as mutable because it is also borrowed as immutable
 --> src/main.rs:4:5
  |
3 |     let first = &v[0];
  |                  - immutable borrow occurs here
4 |     v.push(6);
  |     ^^^^^^^^^ mutable borrow occurs here
5 |     println!("The first element is: {}", first);
  |                                          ----- immutable borrow later used here
```


---


# `Vec` and References

```
error[E0502]: cannot borrow `v` as mutable because it is also borrowed as immutable
 --> src/main.rs:4:5
  |
3 |     let first = &v[0];
  |                  - immutable borrow occurs here
4 |     v.push(6);
  |     ^^^^^^^^^ mutable borrow occurs here
5 |     println!("The first element is: {}", first);
  |                                          ----- immutable borrow later used here
```

* You cannot mutate a vector while references to its elements exist
* Appending might resize and reallocate the vector and change its location in memory!


---


# Iterating over a Vector

To access each element in order, we can iterate through the elements with a `for` loop directly, rather than using indices.


```rust
let v = vec![100, 32, 57];

for elem in &v { // `elem` is a reference to an `i32` (aka `&i32`)
    println!("{}", elem);
}
```

```
100
32
57
```

<!--
Advanced comment: note that `IntoIterator` is directly implemented for `&Vec<T>` (where `Target = &T`),
IN ADDITION to implementing `Deref<Target = [T]>` which means we can call `.iter()` for the same effect.
-->


---


# Mutable Iteration over a Vector

We can also iterate over mutable references to each element to make changes to each element.

```rust
let mut v = vec![100, 32, 57];

for elem in &mut v {  // `elem` is a mutable reference to an `i32`
    *elem += 50;
}

println!("{:?}", v);
```

```
[150, 82, 107]
```

* We only have a single mutable reference into the vector at a time
    * We pass the borrow checker's rules!

<!--
Note: This doesn't allow us to remove elements from the vector, since we'd need a mutable reference to the entire vector to call the remove method.
-->


---


# `Vec` and Mutation

![bg right:25% 75%](../images/ferris_does_not_compile.svg)

Say that you wanted to mutate the vector by appending a new element while you were iterating over it.

```rust
let mut v = vec![100, 32, 57];

for elem in &v {
    if *elem == 100 {
        // Insert `42` into the beginning of `v`.
        v.insert(0, 42);
    }
}
```

* What's the problem here?


---


# `Vec` and Mutation

```
error[E0502]: cannot borrow `v` as mutable because it is also borrowed as immutable
 --> src/main.rs:7:13
  |
4 |     for elem in &v {
  |                 --
  |                 |
  |                 immutable borrow occurs here
  |                 immutable borrow later used here
...
7 |             v.insert(0, 42);
  |             ^^^^^^^^^^^^^^^ mutable borrow occurs here
```

* Again, you are not allowed to mutate vectors while holding a reference!
    * There could be iteration invariants broken
    * Reallocation is still a problem

<!--
When iterating over the vector, what is supposed to happen when all of the elements shift?

Also ask: what happens if you do this in Python? We don't know either!
-->


---


# For Loop Sugar

You can also _consume_ the vector when you want to loop over it.

```rust
let v = vec![100, 32, 57];

for i in v {
    println!("{}", i);
}

// println!("{:?}", v); <-- Can't do this anymore!
```

* We'll talk more about this in a future week!

<!--
It calls `into_iter` instead of `iter`, "consuming" the vec

Note that this ^ is slightly inaccurate (see speaker note from a few slides ago)
-->


---


# Deref Coercion to `&[T]`

Instead of a function taking a `&Vec<T>` as a parameter, we can take a `&[T]`.

```rust
fn largest(list: &Vec<i32>) -> &i32
```
```rust
fn largest(list: &[i32]) -> &i32
```

* The second is more general and preferred
* We can do this because of _deref coercion_
    * This basically means you can turn a `&Vec<T>` into a `&[T]`
* We'll talk more about this in a future week!

<!--
Don't worry about our notation of `T` here, it'll make more sense soon.
-->


---


# Use Enums to Store Multiple Types

Vectors can only store values of the same type, so we can use enums to store values of different types (variants).

```rust
enum SpreadsheetCell {
    Int(i32),
    Float(f64),
    Text(String),
}

let row = vec![
    SpreadsheetCell::Int(3),
    SpreadsheetCell::Text(String::from("blue")),
    SpreadsheetCell::Float(10.12),
];
```


---


# Vectors and Ownership

Vectors own all of their contained elements.

```rust
let mut v = vec![String::from("rust"), String::from("is")];

let s = String::from("great!");

v.push(s); // move `s` into `v`

// `s` is no longer usable here!
```

* To insert an owned value, it must be _moved_ into the vector
    * In other words, ownership must be transferred to the vector


---


# Dropping a Vector

Like any other struct, a vector is dropped when it goes out of scope.

```rust
{
    let v = vec!["rust".to_string(), "is".to_string(), "great!".to_string()];

    stuff(&v); // do stuff with `v`

} // <- `v` goes out of scope and everything it contains is freed
```

* When the vector gets dropped, all of its contents are also dropped
* The borrow checker will ensure that references into the vector cannot be used after it has been dropped


---


# **String**


---


# What is a String?

* A `String` is a `Vec` of bytes (`u8`) interpreted as UTF-8 text
* We've seen `String`s before, but now we'll look at them in more depth
* New Rust programmers may be confused by:
    * `String`'s internal UTF-8 encoding
    * Rust's prevention of possible logical errors from the encoding
    * `String`s are not as simple as they may initially seem


---


# `str` vs `String`

Rust "only" has one string type in the core language, `str`.

* We always* see it in its borrowed form, `&str`
* String slices are `&str`
* String literals are _also_ `&str`
    * They reference data stored in the program's binary
* On the other hand, `String` is a growable, mutable, owned, UTF-8 encoded string type

<!--
When we say "core" language, we mean not in the standard library.

A raw `str` type always has to be on the heap, behind a ref, Box, Rc, raw ptr, or similar.

Slices and literals are both actual references to existing data!

Main idea: `&str` is a primitive more akin to a C array, while String is a managed alternative
(std::string or Java String).

Generally the only other times you will see `str` not in its borrowed form is when using `Arc<str>` or `Rc<str>`, or in dynamically-sized types.
-->


---


# Creating a `String`

You can create a `String` with the methods `new`, `to_string`, or `from`.

```rust
let mut s = String::new(); // empty mutable string

let data = "initial contents"; // string literal
let s = data.to_string(); // string literal into `String`

let s = "initial contents".to_string(); // method directly on a literal

let s = String::from("initial contents"); // string literal into `String`
```


---


# Strings are UTF-8 Encoded

We can represent any properly encoded data in `String`.

Here are some greetings in different languages!

```rust
let hello = String::from("السلام عليكم");
let hello = String::from("Dobrý den");
let hello = String::from("Hallo");
let hello = String::from("שָׁלוֹם");
let hello = String::from("नमस्ते");
let hello = String::from("Olá");
let hello = String::from("Привет");
let hello = String::from("Hola");
```


---


# Updating a `String`

We can grow a `String` by using the `push` or `push_str` methods.

```rust
let mut s = String::from("foo");

s.push('b'); // push a `char`
s.push_str("ar"); // push a `&str`

println!("{}", s);
```

```
foobar
```

<!--
Unless you are "building" strings from an empty string, usage of these methods is pretty rare.

Generally you want to use the `format!()` macro if you already know the different parts of your string.
-->


---


# Updating a `String`

The `push_str` method takes a string slice because we don't want to take ownership of the string passed in.

```rust
let mut s1 = String::from("foo");

let s2 = String::from("bar");

s1.push_str(&s2);

println!("s2 is {}", s2); // `s2` is still valid!
```

```
s2 is bar
```


---


# Concatenating Strings

You can combine two strings with `+`:

```rust
let s1 = String::from("Hello, ");
let s2 = String::from("world!");
let s3 = s1 + &s2; // note `s1` has been moved here (can no longer be used)
```

This is syntactic sugar for a function whose signature looks something like this:
```rust
fn add(self, s: &str) -> String
```

* Notice that `add` takes full ownership of `self`
* Also notice `add` takes `&str` as its second parameter and not `&String`
    * This is the same _deref coercion_ as with `&Vec<T>` to `&[T]`!


---


# Concatenating Multiple Strings

```rust
let s1 = String::from("tic");
let s2 = String::from("tac");
let s3 = String::from("toe");
```

To combine multiple strings, you can use multiple `+`'s:

```rust
let s = s1 + "-" + &s2 + "-" + &s3;
```

Or you can use the `format!` macro:

```rust
let s = format!("{}-{}-{}", s1, s2, s3);

let s = format!("{s1}-{s2}-{s3}"); // now considered more idiomatic
```

<!--
When we first created this lecture, the last example was very new. Now, it is considered more idiomatic to place things that implement `Display` directly in the format string!
-->


---


# Indexing into Strings

![bg right:25% 75%](../images/ferris_does_not_compile.svg)

This code might seem reasonable from any other programming language like Python or C.

```rust
let s1 = String::from("hello");
let h = s1[0];
```

* Accessing individual characters in a string by indexing is common in many languages
* However, if you try to access a `String` using an index, you will get an error...



---


# Indexing into Strings

```rust
let s1 = String::from("hello");
let h = s1[0];
```

```
error[E0277]: the type `String` cannot be indexed by `{integer}`
 --> src/main.rs:3:13
  |
3 |     let h = s1[0];
  |             ^^^^^ `String` cannot be indexed by `{integer}`
  |
  = help: the trait `Index<{integer}>` is not implemented for `String`
```

* Why won't Rust allow indexing into `String`?


---


# Internal Representation of Strings

A `String` is really a wrapper over `Vec<u8>`, or a vector of bytes.

```rust
let hello = String::from("Hola");
```

* How long is this string?
    * The length of the string is 4
    * The internal vector storing the string `"Hola"` is 4 bytes long
* Simple enough, right?


---


# Internal Representation: Cyrillic

Now suppose we wanted to say "Hello", but in Russian.

```rust
let hello = String::from("Привет");
```

* How long is this string?
    * There are 6 distinct characters
    * However, the string's `len` is 12, the number of bytes needed in the internal vector


---


# Internal Representation: UTF-8

![bg right:25% 75%](../images/ferris_does_not_compile.svg)

Let's revisit some invalid Rust code again.

```rust
let hello = "Привет";
let answer = &hello[0];
```

* What _should_ `answer` be?
    * It can't be `П`, internally it is represented by 2 bytes: `[208, 159]`
    * Do we return `208` instead?
* There isn't any obvious expected behavior here...


---


# Internal Representation: UTF-8

```rust
let hello = "Привет";
let answer = &hello[0]; // BAD!
```

* Anything we can return here might not be an "expected" result
* The philosophy of Rust is to _not compile this code at all_
    * Prevents misunderstandings early in the development process
* Further reading on UTF-8: [Rust Book Chapter 8.2](https://doc.rust-lang.org/book/ch08-02-strings.html#bytes-and-scalar-values-and-grapheme-clusters-oh-my)


---


# Slicing Strings

Instead of indexing with a single number, you can use `[]` with a _range_ to create a string slice containing specific bytes.

```rust
let hello = "Привет";

let s = &hello[0..4]; // `s` == "Пр"
```

<!--
Make sure to note that р != p :D
-->


---


# Slicing Strings

However, if we try to slice only a part of a character's bytes, Rust panics at runtime.

```rust
let hello = "Привет";

let s = &hello[0..1];
```

```
thread 'main' panicked at 'byte index 1 is not a char boundary;
it is inside 'П' (bytes 0..2) of `Привет`'
```

* This happens in the same way that an invalid index causes a panic!

<!--
Still preventing weird cases!
-->


---


# Iterating Over Strings

Normally, we want to iterate over individual Unicode scalar values, and we can use the `chars` method.

```rust
for c in "Пр".chars() {
    println!("{c}");
}
```

```
П
р
```

<!--
May want to mention this:
Be careful when using this with `.nth()` as that is a linear time operation, not constant time!
-->


---


# Iterating Over Strings

Alternatively, if you want the actual raw bytes, you can use the `bytes` method.

```rust
for b in "Пр".bytes() {
    println!("{b}");
}
```

```
208
159
209
128
```


---


# Recap: Strings

* Rust chooses to use UTF-8 strings as the default (for both `String` and `&str`)
    * Programmers have to think about handling unicode upfront
    * The complexity brought on by encodings is more apparent in Rust
    * However, this prevents having to deal with non-ASCII characters later!
* The standard library offers many methods for [`String`](https://doc.rust-lang.org/std/string/struct.String.html) and [`&str`](https://doc.rust-lang.org/std/primitive.str.html) types to help handle these complex situations correctly


---


# **HashMap**


---


# `HashMap<K, V>`

The type `HashMap<K, V>` stores keys with type `K` mapped to values with type `V`.

* Many languages support this kind of data structure, even if they use a different name:
    - Hash
    - Map
    - Object
    - Hash Table
    - Dictionary
    - Associative Array


---


# Creating a `HashMap`

We can create a new hash map with `new` and insert entries with `insert`.

```rust
use std::collections::HashMap;

let mut scores = HashMap::new();

scores.insert(String::from("Blue"), 10);
scores.insert(String::from("Yellow"), 50);
```

* Note that we need to import `HashMap` from the standard library's collections module with `use`
* We'll talk more about `use` in an upcoming week!

<!-- Mention that because Vec and String are used so frequently, they are automatically "imported" -->


---


# Accessing Values in a `HashMap`

We can use the `get` method to get the value associated with a key.

```rust
let mut scores = HashMap::new();
scores.insert(String::from("Blue"), 10);
scores.insert(String::from("Yellow"), 50);

let team_name = String::from("Blue");
let score = scores.get(&team_name).unwrap_or(&0);
```

* The `get` method returns an `Option<&V>`, similar to `Vec::get()`
* We use `unwrap_or(&0)` on the result
    * If it returns `Some(&x)`, we unwrap and get `&x`
    * If it returns `None`, we go to the default case `&0`


---


# Iterating over a `HashMap`

We can iterate over each key/value pair using a `for` loop, similar to vectors.

```rust
let mut scores = HashMap::new();

scores.insert(String::from("Blue"), 10);
scores.insert(String::from("Yellow"), 50);

for (key, value) in scores {
    println!("{key}: {value}");
}
```

```
Yellow: 50
Blue: 10
```

* _Note that the order is non-deterministic_


---


# `HashMap`s and Ownership

Hash maps own their contained data, just like vectors.

```rust
let field_name = String::from("Favorite color");
let field_value = String::from("Blue");

let mut map = HashMap::new();
map.insert(field_name, field_value);

// `field_name` and `field_value` are invalid at this point!
```


---


# Updating a `HashMap`

Hash maps only contain one value for each distinct key, so to update we can just insert twice.

```rust
let mut scores = HashMap::new();

scores.insert(String::from("Blue"), 10);
scores.insert(String::from("Blue"), 25);

println!("{:?}", scores);
```

```
{"Blue": 25}
```

* Inserting twice overwrites the existing value for the given key


---


# Accessing a `HashMap` with Defaults

* A common pattern when accessing a `HashMap` is:
    * If the key exists, we want to access the value
    * If the key does not exist, insert a default value and then access it
* `HashMap` has a special API called `Entry`


---


# `HashMap` Entries

To insert a value if the key does not already exist, you can use the `Entry` API and the method `or_insert`.

```rust
let mut scores = HashMap::new();
scores.insert(String::from("Blue"), 10);

scores.entry(String::from("Yellow")).or_insert(50);
scores.entry(String::from("Blue")).or_insert(50);

println!("{:?}", scores);
```

```
{"Yellow": 50, "Blue": 10}
```


---


# `HashMap` Entries

If you want to update a value, or provide a default if it doesn't yet exist, you can do something similar:


```rust
let text = "hello world wonderful world";

let mut map = HashMap::new();

for word in text.split_whitespace() {
    let count: &mut usize = map.entry(word).or_insert(0);
    *count += 1;
}

println!("{:?}", map);
```

```
{"world": 2, "hello": 1, "wonderful": 1}
```

<!--
Typically, you would just do:
*map.entry(word).or_insert(0) += 1
-->


---


# `hash_map::Entry::or_insert`

The method `or_insert` has the following signature:

```rust
fn or_insert(self, default: V) -> &mut V
```

* Given the `Entry`, it gives out a mutable reference to the value
    * We need to provide a default since this data might not exist yet
    * That reference is then guaranteed to point to valid data
* Shorter and more readable than separate conditionals

<!--
If the data might not exist, we would use a...OPTION TYPE
-->

---


# Recap:

* We covered [The Rust Book Chapter 8](https://doc.rust-lang.org/book/ch08-00-common-collections.html)
* Always refer to the documentation!
    * `Vec<T>` [documentation](https://doc.rust-lang.org/std/vec/struct.Vec.html)
    * `String` [documentation](https://doc.rust-lang.org/std/string/struct.String.html)
    * `HashMap<K, V>` [documentation](https://doc.rust-lang.org/stable/std/collections/hash_map/struct.HashMap.html)


---


# **Generics**


---


# Generics

So what was the deal with the `T` in `Vec<T>`, and `K, V` in `HashMap<K, V>`?

* We refer to these as _generic_ types
* Think of it as being able to fill in any type you want in place of `T`
* Generics allow us to replace specific types with a placeholder that represents multiple types
    * Removes code duplication

---


# Removing Code Duplication

Let's say we want to find the largest number in a list.

```rust
let number_list = vec![34, 50, 25, 100, 65];

let mut largest = &number_list[0];

// Pretend the list always has at least 1 element.
for number in &number_list {
    if number > largest {
        largest = number;
    }
}

println!("The largest number is {}", largest);
```


---


# Removing Code Duplication

What if we have multiple lists? We then have to do multiple searches.

```rust
let number_list = vec![34, 50, 25, 100, 65];
let mut largest = &number_list[0];
for number in &number_list {
    if number > largest { largest = number; }
}
println!("The largest number is {}", largest);

let number_list = vec![102, 34, 6000, 89, 54, 2, 43, 8];
let mut largest = &number_list[0];
for number in &number_list {
    if number > largest { largest = number; }
}
println!("The largest number is {}", largest); // I is good programr :D
```

<!--
Remember to say that this is BAD code quality :D
-->


---


# Removing Code Duplication

Instead, we can make a function called `largest`.

```rust
// Assume that the `list` is non-empty :D
fn largest(list: &[i32]) -> &i32 {
    let mut largest = &list[0];

    for item in list {
        if item > largest { largest = item; }
    }

    largest
}

let number_list = vec![34, 50, 25, 100, 65];
println!("The largest number is {}", largest(&number_list));
```

<!--
For now we'll ignore the fact that this requires there to be at least 1 element in the vector
-->


---


# Remove Function Duplication

What if we also wanted to find the largest character in a slice?

```rust
//        +++++         ++++       ++++
fn largest_char(list: &[char]) -> &char {
    let mut largest = &list[0];
    for item in list {
        if item > largest { largest = item; }
    }
    largest
}
```

* This is the same as finding the largest number in a list
* We would still need to write a new function in addition to `largest`
* Can we deduplicate a _function_?


---


# Generic Functions

We can define a function as generic over some type `T` with a tag `<T>`:

```rust
fn largest<T>(list: &[T]) -> &T
```

* This function is generic over `T`
* This function takes in a slice of `T` as input
* This function returns a reference to `T`
* `T` can be _any_* type!
    * _Not actually **any** type_...


---


# Generic Functions

Generic types can have any name, not just `<T>`:

```rust
fn largest<T>(list: &[T]) -> &T
```

```rust
fn largest<Key>(list: &[Key]) -> &Key
```

```rust
fn largest<Smile>(list: &[Smile]) -> &Smile
```

* All of these essentially mean the same thing!

<!--
Last one is _frowned_ upon since it might seem like a struct or enum

There are always exceptions to the rule, especially if you have multiple generic type parameters.
-->


---


# Generic Functions

Let's try to modify our old function directly:

![bg right:25% 75%](../images/ferris_does_not_compile.svg)

```rust
fn largest<T>(list: &[T]) -> &T {
    let mut largest = &list[0];
    for item in list {
        if item > largest { largest = item; }
    }
    largest
}

println!("The largest number is {}",
            largest(&[34, 50, 25, 100, 65]));
println!("The largest char is {}",
            largest(&['y', 'm', 'a', 'q']));
```

---

# Generic Functions

We get an error:

```
error[E0369]: binary operation `>` cannot be applied to type `&T`
 --> src/main.rs:4:17
  |
4 |         if item > largest {
  |            ---- ^ ------- &T
  |            |
  |            &T
  |
help: consider restricting type parameter `T`
  |
1 | fn largest<T: std::cmp::PartialOrd>(list: &[T]) -> &T {
  |             ++++++++++++++++++++++
```


---


```
error[E0369]: binary operation `>` cannot be applied to type `&T`
 --> src/main.rs:4:17
  |
4 |         if item > largest {
  |            ---- ^ ------- &T
  |            |
  |            &T
  |
help: consider restricting type parameter `T`
  |
1 | fn largest<T: std::cmp::PartialOrd>(list: &[T]) -> &T {
  |             ++++++++++++++++++++++
```

* We cannot just compare any two `&T` to each other
* We've stated that `T` can be _any_ type, regardless of if `T` is a type that cannot actually be compared
* Let's just follow the compiler's advice for now!


---


# Generic Functions

Once we make that change, this works!

```rust
//            ++++++++++++++++++++
fn largest<T: std::cmp::PartialOrd>(list: &[T]) -> &T {
    let mut largest = &list[0];

    for item in list {
        if item > largest { largest = item; }
    }

    largest
}
```

```
The largest number is 100
The largest char is y
```


---


# Sneak Peek: Traits

```rust
fn largest<T: std::cmp::PartialOrd>(list: &[T]) -> &T {
    let mut largest = &list[0];

    for item in list {
        if item > largest { largest = item; }
    }

    largest
}
```

* We'll talk about type restrictions with _traits_ next week!
* For now, all you need to know is that we need the `PartialOrd` trait to enable comparisons between values (establish an `Ord`ering)


---


# Generic Structs

We can define structs to contain a generic type using the `<T>` syntax as well!

```rust
struct Point<T> {
    x: T,
    y: T,
}

fn main() {
    let integer = Point { x: 5, y: 10 };
    let float = Point { x: 1.0, y: 4.0 };
}
```


---


# Generic Structs

![bg right:25% 75%](../images/ferris_does_not_compile.svg)

Observe that this declaration defines both the `x` field and the `y` field to be of the same type.

```rust
struct Point<T> {
    x: T,
    y: T,
}

fn main() {
    let wont_work = Point { x: 5, y: 4.0 };
}
```


---


# Generic Structs

If we try to compile this, we get an error

```
error[E0308]: mismatched types
 --> src/main.rs:7:38
  |
7 |     let wont_work = Point { x: 5, y: 4.0 };
  |                                      ^^^ expected integer,
                                             found floating-point number
```


---


# Generic Structs

If we want a struct that allows different generic fields to have different types, we need to define another generic type.

```rust
struct Point<T, U> {
    x: T,
    y: U,
}

fn main() {
    let both_integer = Point { x: 5, y: 10 };
    let both_float = Point { x: 1.0, y: 4.0 };
    let integer_and_float = Point { x: 5, y: 4.0 };
}
```

* Note that they can still be the same!


---


# Generic Enums

Recall the `Option<T>` type:

```rust
enum Option<T> {
    Some(T),
    None,
}
```

* This is a generic enum over `T`!


---


# Generic Enums

Enums can be generic over multiple types, just like structs.

```rust
enum Result<T, E> {
    Ok(T),
    Err(E),
}
```

* This enum is generic over `T` and `E`, with each stored in a variant
* `Result<T, E>` is a very common type in the standard library that we will talk about soon!


---


# Generic Methods

Methods on structs can also be generic.

```rust
struct Point<T> {
    x: T,
    y: T,
}

impl<T> Point<T> {
    fn x(&self) -> &T {
        &self.x
    }
}

let p = Point { x: 5, y: 10 };
println!("p.x = {}", p.x());
```


---


# Generic Methods

```rust
impl<T> Point<T> {
    fn x(&self) -> &T {
        &self.x
    }
}
```

* Observe that we have to declare `T` after the `impl` as well as after `Point`
* This is to specify that we're implementing methods on the _type_ `Point<T>`
* This is different from implementing methods on the _type_ `Point<f32>`


---


# Generic `impl`

We could have made an implementation specific to `Point<f32>`:

```rust
impl Point<f32> {
    fn distance_from_origin(&self) -> f32 {
        (self.x.powi(2) + self.y.powi(2)).sqrt()
    }
}
```

* This code means that `Point<f32>` will have an additional `distance_from_origin` method on top of the methods defined for `Point<T>`

<!--
Specialization still not implemented:
https://rust-lang.github.io/rfcs/1210-impl-specialization.html

This means that the blanket `impl<T>` cannot also have a `distance_from_origin` method if the one on `Point<f32>` exists.
-->


---


# Generic `impl`

Going back to the `Point<T, U>` example:

```rust
struct Point<T, U> {
    x: T,
    y: U,
}
```

We could implement methods for when `x` is `i32`, but with no restrictions on `y`.

```rust
impl<U> Point<i32, U> {
    fn get_sum_x(&self, other: Point<i32, U>) -> i32 {
        self.x + other.x
    }
}
```


---


# Generic `impl`

However, this actually restricts the type of `other` to have the same generic type parameters `<i32, U>`.

```rust
impl<U> Point<i32, U> {
    fn get_sum_x(&self, other: Point<i32, U>) -> i32 {
        self.x + other.x
    }
}

fn main() {
    let p1 = Point { x: 5, y: 3.2 }; // y is f64
    let p2 = Point { x: 5, y: 4.4 }; // y is also f64
    println!("{}", p1.get_sum_x(p2))
}
```

* Note that `U` has to be the same in both `self` and `other`


---


# Generic `impl`

To solve this, we can make the method generic over another type:

```rust
impl<U> Point<i32, U> {
    fn get_sum_x<V>(&self, other: Point<i32, V>) -> i32 {
        self.x + other.x
    }
}

fn main() {
    let p1 = Point { x: 5, y: 3.2 };           // y is f64
    let p2 = Point { x: 5, y: String::new() }; // y is String
    println!("{}", p1.get_sum_x(p2))
}
```



---


Here's another example of a generic `impl`:

```rust
struct Point<X1, Y1> {
    x: X1,
    y: Y1,
}

impl<X1, Y1> Point<X1, Y1> {
    fn mixup<X2, Y2>(self, other: Point<X2, Y2>) -> Point<X1, Y2> {
        Point { x: self.x, y: other.y }
    }
}

fn main() {
    let p1 = Point { x: 5, y: 10.4 };
    let p2 = Point { x: "Hello", y: 'c' };

    let p3 = p1.mixup(p2);
    println!("p3.x = {}, p3.y = {}", p3.x, p3.y);
}
```


---


# Performance of Generics

* The good news is that there is _zero_ runtime overhead to using generics!
    * The work is done at compile-time instead of runtime
* Rust accomplishes this with _monomorphization_


---


# Monomorphization

Let's look at how this works by using the standard library's generic `Option<T>`:

```rust
let integer = Some(5);
let float = Some(5.0);
```

* The compiler will identify which types `T` can take on by find all instances of `Option<T>`, in this case `i32` and `f64`
* It creates _monomorphized_ versions of `Option` specific to those types


---


# Monomorphization

The compiler will generate something similar to the following:

```rust
enum Option_i32 {
    Some(i32),
    None,
}

enum Option_f64 {
    Some(f64),
    None,
}

let integer = Option_i32::Some(5);
let float = Option_f64::Some(5.0);
```

* All extra work is performed at compile-time!


---


# Recap: Generics

* Generics allow us to reduce code duplication
* Monomorphization means we do not incur any runtime cost!


---


# Homework 4

* You'll be implementing two collection data structures:
    * `MultiSet`
        * A collection that stores unordered values and tracks multiplicity
    * `MultiMap`
        * A collection that maps keys to any number of values
* Make sure you are familiar with the API for [`HashMap`](https://doc.rust-lang.org/std/collections/struct.HashMap.html) and [`Entry`](https://doc.rust-lang.org/std/collections/hash_map/enum.Entry.html)!


---


# **Next Lecture: Errors and Traits**

![bg right:30% 80%](../images/ferris_happy.svg)

Thanks for coming!

<br>

_Slides created by:_
Connor Tsui, Benjamin Owad, David Rudo,
Jessica Ruan, Fiona Fisher, Terrance Chen
