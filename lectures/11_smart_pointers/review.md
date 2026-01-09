---
marp: true
theme: rust
class: invert

paginate: true
---


<!-- _class: communism communism2 invert  -->

### Spring 2024

## Intro to Rust Lang

# Quiz Review

<br>

Benjamin Owad, David Rudo, and Connor Tsui

<!-- ![bg right:35% 65%](../images/ferris.svg) -->


---


# Quiz Answers and Review

* For any of the "does this compile" questions, try to compile it yourself and see what happens!
* Generally, the error messages are very helpful
* Use [Rust Playground](https://play.rust-lang.org/?version=stable&mode=debug&edition=2021) if you want to do it online!


---


# Today

* We will go over questions 4, 6, 8, and 9
* If you want to ask about other questions please ask on Piazza!


---


# Question 2

Determine whether the program will pass the compiler.

```rust
fn add_suffix(mut s: String) -> String {
    s.push_str(" world");
    s
}

fn main() {
    let s = String::from("hello");
    let s2 = add_suffix(s);
    println!("{}", s2);
}
```


---


# Question 2 Answer

```rust
fn add_suffix(mut s: String) -> String {
    s.push_str(" world");
    s
}

fn main() {
    let s = String::from("hello");
    let s2 = add_suffix(s);
    println!("{}", s2);
}
```

* Answer:
    * `hello world`
* Reason:
    * Just a normal program (warmup question)


---


# Question 3

Determine whether the program will pass the compiler.

```rust
fn main() {
    let s = String::from("hello");
    let s2;
    let b = false;
    if b {
        s2 = s;
    }
    println!("{}", s);
}
```


---


# Question 3 Answer

```rust
fn main() {
    let s = String::from("hello");
    let s2;
    let b = false;
    if b {
        s2 = s;
    }
    println!("{}", s);
}
```

* Answer:
    * `does not compile`
* Reason:
    * Try compiling it yourself! The error message is pretty good
    * Remember, even if the programmer can guarantee something, the compiler might not know about that guarantee!


---


# Question 3 Brown Explanation

Because `s` could be moved inside of the if-statement, it is illegal to use it on line 8.

While the if-statement will never execute in this program because `b` is always `false`, Rust does not in general try to determine whether if-statements will or won't execute.

Rust just assumes that it *might* be executed, and therefore `s` *might* be moved.


---


# Question 4

_This question is tricky._

Say we have a function that moves a box, like this:

```rust
fn move_a_box(b: Box<i32>) {
    // This space intentionally left blank
}
```

* Note that a `Box` is just like any owned type we've dealt with
* Notably, the `Box` is moved into the function and then dropped at the end
    * This means that the data the `Box` owns on the heap is _freed_
*   _It's on us if you didn't know that, but if you got it wrong before because of that, now is the time to think about this question more!_


---


# Question 4 (A)

Imagine if Rust allowed this snippet to compile and run.

Would this snippet cause undefined behavior?

```rust
fn move_a_box(b: Box<i32>) {
    // This space intentionally left blank
}
```

```rust
// A
let b = Box::new(0);
let b2 = b;
move_a_box(b);
```


---


# Question 4 Answer (A)

```rust
// A
let b = Box::new(0);
let b2 = b;
move_a_box(b);
```

* Answer:
    * Yes, this would cause undefined behavior
* Reason:
    * If `b2 = b` is executed, and we don't invalidate `b`, then we have 2 pointers to the same place on the heap
    * `move_a_box(b)` frees the memory on the heap
    * At the end of the outer scope, `b2` gets freed
    * Double free!


---


# Question 4 (B)

Imagine if Rust allowed this snippet to compile and run.

Would this snippet cause undefined behavior?

```rust
fn move_a_box(b: Box<i32>) {
    // This space intentionally left blank
}
```

```rust
// B
let b = Box::new(0);
let b2 = b;
println!("{}", b);
move_a_box(b2);
```


---


# Question 4 Answer (B)

```rust
// B
let b = Box::new(0);
let b2 = b;
println!("{}", b);
move_a_box(b2);
```

* Answer:
    * No, this would not cause undefined behavior
* Reason:
    * Running `b2 = b` doesn't change the data on the heap, so we could still access the memory that `b` points to
    * Also, since we "moved" `b` into `b2`, Rust will not call `drop` on `b`, so we don't get a double free


---


# Question 4 (C)

Imagine if Rust allowed this snippet to compile and run.

Would this snippet cause undefined behavior?

```rust
fn move_a_box(b: Box<i32>) {
    // This space intentionally left blank
}
```

```rust
// C
let b = Box::new(0);
move_a_box(b);
let b2 = b;
```


---


# Question 4 Answer (C)

```rust
// C
let b = Box::new(0);
move_a_box(b);
let b2 = b;
```

* Answer:
    * Yes, this is undefined behavior
* Reason:
    * We free the memory that `b` points to, and then set `b2` to point to freed memory
    * Double free!


---


# Question 4 (D)

Imagine if Rust allowed this snippet to compile and run.

Would this snippet cause undefined behavior?

```rust
fn move_a_box(b: Box<i32>) {
    // This space intentionally left blank
}
```

```rust
// D
let b = Box::new(0);
move_a_box(b);
println!("{}", b);
```


---


# Question 4 Answer (D)

```rust
// D
let b = Box::new(0);
move_a_box(b);
println!("{}", b);
```

* Answer:
    * Yes, this would cause undefined behavior
* Reason:
    * We are trying to read freed memory


---


# Question 4 Brown Explanation

The key idea is that when a box is passed to `move_a_box`, its memory is freed after `move_a_box` ends.

Therefore:
* Reading `b` via `println` after `move_a_box` is undefined behavior, as it reads freed memory.
* Giving `b` a second owner is undefined behavior, as it would cause Rust to free the box a second time on behalf of `b2`. It doesn't matter whether the `let b2 = b` binding happens before or after `move_a_box`.

However, doing `let b2 = b` and then `println` is not undefined behavior. Although `b` is moved, its data is not freed until `move_a_box` is called at the end. Therefore this program is technically safe, although still rejected by Rust.


---


# Question 5

Determine whether the program will pass the compiler.

```rust
fn main() {
    let mut s = String::from("hello");
    let s2 = &s;
    let s3 = &mut s;
    s3.push_str(" world");
    println!("{s2}");
}
```


---


# Question 5 Answer

```rust
fn main() {
    let mut s = String::from("hello");
    let s2 = &s;
    let s3 = &mut s;
    s3.push_str(" world");
    println!("{s2}");
}
```

* Answer:
    * `does not compile`


---


# Question 6

_This question is tricky._

Consider this Rust function that pushes a number onto the end of a vector, and then removes and returns the number from the front of the vector:

```rust
fn give_and_take(v: &Vec<i32>, n: i32) -> i32 {
    v.push(n);
    v.remove(0)
}
```


---


# Question 6

Normally, if you try to compile this function, the compiler returns the following error:

```
error[E0596]: cannot borrow `*v` as mutable, as it is behind a `&` reference
 --> test.rs:2:5
  |
1 | fn give_and_take(v: &Vec<i32>, n: i32) -> i32 {
  |                     --------- help: consider changing this to be a
                                        mutable reference: `&mut Vec<i32>`
2 |     v.push(n);
  |     ^^^^^^^^^ `v` is a `&` reference,
        so the data it refers to cannot be borrowed as mutable
```


---


# Question 6 (A)

Assume the compiler did NOT reject this function.

Would this snippet cause undefined behavior?

```rust
fn give_and_take(v: &Vec<i32>, n: i32) -> i32 {
    v.push(n);
    v.remove(0)
}
```

```rust
// A
let v = vec![1, 2, 3];
let n = &v[0];
give_and_take(&v, 4);
println!("{}", n);
```


---


# Question 6 Answer (A)

```rust
fn give_and_take(v: &Vec<i32>, n: i32) -> i32 {
    v.push(n);
    v.remove(0)
}

// A
let v = vec![1, 2, 3];
let n = &v[0];
give_and_take(&v, 4);
println!("{}", n);
```

* Answer:
    * Yes, this would cause undefined behavior
* Reason:
    * `v.push` might reallocate the `Vec`
    * `n` would point to invalid memory


---


# Question 6 (B)

Assume the compiler did NOT reject this function.

Would this snippet cause undefined behavior?

```rust
fn give_and_take(v: &Vec<i32>, n: i32) -> i32 {
    v.push(n);
    v.remove(0)
}
```

```rust
// B
let v = vec![1, 2, 3];
let v2 = &v;
give_and_take(&v, 4);
println!("{}", v2[0]);
```


---


# Question 6 Answer (B)

```rust
// B
let v = vec![1, 2, 3];
let v2 = &v;
give_and_take(&v, 4);
println!("{}", v2[0]);
```

* Answer:
    * No this would not cause undefined behavior
* Reason:
    * Observe that `v2` is a reference to `v`, NOT a reference to heap memory
    * So even if `v` resizes, `v2` still points to `v`, which points to valid memory


---


# Question 6 (C)


Assume the compiler did NOT reject this function.

Would this snippet cause undefined behavior?

```rust
fn give_and_take(v: &Vec<i32>, n: i32) -> i32 {
    v.push(n);
    v.remove(0)
}
```

```rust
// C
let v = vec![1, 2, 3];
let n = &v[0];
let k = give_and_take(&v, 4);
println!("{}", k);
```

---


# Question 6 Answer (C)

```rust
// C
let v = vec![1, 2, 3];
let n = &v[0];
let k = give_and_take(&v, 4);
println!("{}", k);
```


* Answer:
    * No this would not cause undefined behavior
* Reason:
    * We never use `n` after creating it, so this is perfectly safe


---


# Question 6 Brown Explanation

As we saw earlier in the section, `v.push(n)` can cause `v` to reallocate its internal contents, invalidating any references to the elements of `v` on the heap.

Therefore calling `give_and_take(&v, 4)` will cause previously-created element references to point to invalid memory. The two programs that bind `let n = &v[0]` before `give_and_take` are candidates for undefined behavior. `let v2 = &v` is not a candidate because a reference to the container `v` is not actually invalidated by mutating `v`.

The program that does `println!("{}", n)` will cause undefined behavior by reading the invalid memory. The program that does `println!("{}", k)` will not cause undefined behavior, because it does not use the invalidated reference.


---


# Question 7

Let's say a programmer tried writing the following function:

```rust
/// Returns a person's name with "Ph.D." added as a title
fn award_phd(name: &String) -> String {
    let mut name = *name;
    name.push_str(", Ph.D.");
    name
}
```

The Rust compiler rejects their code with the following error:

```text
error[E0507]: cannot move out of `*name` which is behind a shared reference
 --> test.rs:3:20
  |
3 |     let mut name = *name;
  |                    ^^^^^
  |                    |
  |                    move occurs because `*name` has type `String`,
                       which does not implement the `Copy` trait
  |                    help: consider borrowing here: `&*name`
```


---


# Question 7 Answer

What is the most idiomatic fix to the program? Remember, we want to return a _new_ name, and the old name should not be modified.

Answer:

```rust
// A
fn award_phd(name: &String) -> String {
    let mut name = name.clone();
    name.push_str(", Ph.D.");
    name
}
```


---


# Question 7 Incorrect Answer (B)

Option B doesn't compile:

```rust
// B
fn award_phd(name: &String) -> String {
    let mut name = &*name;
    name.push_str(", Ph.D.");
    name
}
```

```
   Compiling playground v0.0.1 (/playground)
error[E0308]: mismatched types
 --> src/main.rs:8:5
  |
5 | fn award_phd(name: &String) -> String {
  |                                ------ expected `String` because of return type
...
8 |     name
  |     ^^^^- help: try using a conversion method: `.to_string()`
  |     |
  |     expected `String`, found `&String`
```


---


# Question 7 Incorrect Answer (C & D)

Options C and D both mutate the original `String`, instead of creating a new one:

```rust
// C
fn award_phd(mut name: String) -> String {
    name.push_str(", Ph.D.");
    name
}
```

```rust
// D
fn award_phd(name: &mut String) {
    name.push_str(", Ph.D.");
}
```


---


# Question 8

_This question is tricky._

The following program does not compile:

```rust
fn copy_to_prev(v: &mut Vec<i32>, i: usize) {
    let n = &mut v[i];
    *n = v[i - 1];
}

fn main() {
    let mut v = vec![1, 2, 3];
    copy_to_prev(&mut v, 1);
}
```


---


# Question 8

```
   Compiling playground v0.0.1 (/playground)
error[E0502]: cannot borrow `*v` as immutable because it is also borrowed as mutable
 --> src/main.rs:3:10
  |
2 |     let n = &mut v[i];
  |                  - mutable borrow occurs here
3 |     *n = v[i - 1];
  |     -----^-------
  |     |    |
  |     |    immutable borrow occurs here
  |     mutable borrow later used here
  |
help: try adding a local storing this...
 --> src/main.rs:3:11
  |
3 |     *n = v[i - 1];
  |           ^^^^^^^
help: ...and then using that local here
 --> src/main.rs:3:5
  |
3 |     *n = v[i - 1];
  |     ^^^^^^^^^^^^^
```




---


# Question 8

```rust
fn copy_to_prev(v: &mut Vec<i32>, i: usize) {
    let n = &mut v[i];
    *n = v[i - 1];
}
fn main() {
    let mut v = vec![1, 2, 3];
    copy_to_prev(&mut v, 1);
}
```

Which of the following best describes the undefined behavior that could occur if this program were allowed to execute?

* The assignment `*n` is a use of freed memory
* The borrow `&mut v[i]` creates a pointer to freed memory
* The read of `v[i - 1]` is a use of freed memory
* There is no undefined behavior in this program


---


# Question 8 Answer

```rust
fn copy_to_prev(v: &mut Vec<i32>, i: usize) {
    let n = &mut v[i];
    *n = v[i - 1];
}

fn main() {
    let mut v = vec![1, 2, 3];
    copy_to_prev(&mut v, 1);
}
```

* Answer:
    * There is no undefined behavior in this program
* Reason:
    * The borrow checker doesn't know that `v[i]` and `v[i - 1]` point to completely different things


---


# Question 8 Brown Explanation

This program is safe. No undefined behavior could occur if it were executed. (If `i` was outside the bounds of `v`, then Rust will panic at runtime rather than cause undefined behavior.)

The issue is that Rust doesn't know for sure that `v[i]` and `v[i - 1]` are referring to different elements.


---


# Question 9

Which statement is true about the sizes of `s2` and `s3`? Remember, we only care about the amount of data on the _stack_.

```rust
fn main() {
    let s = String::from("hello");
    let s2: &String = &s;
    let s3: &str = &s[..];
}
```

 * `s3` has fewer bytes than `s2`
 * `s3` has more bytes than `s2`
 * `s3` has the same number of bytes as `s2`


---


# Question 9 Answer

```rust
fn main() {
    let s = String::from("hello");
    let s2: &String = &s;
    let s3: &str = &s[..];
}
```

* Answer:
    * `s3` has more bytes than `s2`
* Reason:
    * `s3` is a string slice, so it stores both a 64-bit pointer and a 64-bit length of the slice
    * `s2` is just a normal 64-bit pointer


---


# Question 10

Determine whether the program will pass the compiler.

```rust
struct Point {
    x: i32,
    y: i32
}

impl Point {
    fn get_x(&mut self) -> &mut i32 {
      &mut self.x
    }
}
```


---


# Question 10

```rust
struct Point {
    x: i32,
    y: i32
}

impl Point {
    fn get_x(&mut self) -> &mut i32 {
      &mut self.x
    }
}

fn main() {
    let mut p = Point { x: 1, y: 2 };
    let x = p.get_x();
    *x += 1;
    println!("{} {}", *x, p.y);
}
```


---


# Question 10 Answer

```rust
fn main() {
    let mut p = Point { x: 1, y: 2 };
    let x = p.get_x();
    *x += 1;
    println!("{} {}", *x, p.y);
}
```

* Answer:
    * `does not compile`


---


# Question 11

```rust
/// Removes all the zeros in-place from a vector of integers.
fn remove_zeros(v: &mut Vec<i32>) {
    for (i, t) in v.iter().enumerate().rev() {
        if *t == 0 {
            v.remove(i);
            v.shrink_to_fit();
        }
    }
}
```

If you tried to compile this function, which of the following best describes the compiler error you would get?

 * `v` does not live long enough to call `v.remove(i)`
 * `t` cannot be dereferenced while `i` is live
 * `v.remove(i)` cannot borrow `v` as mutable
 * `v.iter()` cannot be called on a mutable reference


---


# Question 11 Answer

```rust
/// Removes all the zeros in-place from a vector of integers.
fn remove_zeros(v: &mut Vec<i32>) {
    for (i, t) in v.iter().enumerate().rev() {
        if *t == 0 {
            v.remove(i);
            v.shrink_to_fit();
        }
    }
}
```

* Answer:
    * `v.remove(i)` cannot borrow `v` as mutable
* Reason:
    * `remove` requires a mutable reference
    * `iter` requires an immutable reference
    * Do the math!


---


# Question 12

Consider the following un-annotated function signature.

```rust
struct Foo<'a> {
    bar: &'a i32
}

fn baz(f: Foo) -> &i32 { /* ... */ }
```

Will Rust accept this function signature? If so, what lifetimes will it infer?


---


# Question 12 Answer

```rust
struct Foo<'a> {
    bar: &'a i32
}

fn baz(f: Foo) -> &i32 { /* ... */ }
```

If we follow the lifetime elision rules, we get this:

```rust
// D
fn baz<'a>(f: Foo<'a>) -> &'a i32
```


---


# Question 13

Consider the following un-annotated function signature.

```rust
struct Foo<'a> {
    bar: &'a i32
}

// Foo changed to &Foo
fn baz(f: &Foo) -> &i32 { /* ... */ }
```

Will Rust accept this function signature? If so, what lifetimes will it infer?


---


# Question 13 Answer

```rust
struct Foo<'a> {
    bar: &'a i32
}

// Foo changed to &Foo
fn baz(f: &Foo) -> &i32 { /* ... */ }
```

* Answer:
    * Rust will reject this function signature
* Reason:
    * Every reference needs a lifetime, including `Foo`'s inner reference
    * Since there are two input lifetimes, Rust cannot figure out the lifetime of the return type


---


# Question 14

Determine whether the program will pass the compiler.

If it passes, write the expected output of the program if it were executed. If it does not pass, write "does not compile" below.

```rust
fn main() {
    let v = vec![1, 2, 3, 4];
    let a: Vec<_> = v.iter().filter(|x: &&i32| *x % 2 == 0).map(|x: &i32| x * 2).collect();
    let b: Vec<_> = v.iter().map(|x: &i32| x * 2).filter(|x: &i32| x % 2 == 0).collect();
    println!("{} {}", a[0], b[0]);
}
```


---


# Question 14 Answer

```rust
fn main() {
    let v = vec![1, 2, 3, 4];
    let a: Vec<_> = v.iter().filter(|x: &&i32| *x % 2 == 0).map(|x: &i32| x * 2).collect();
    let b: Vec<_> = v.iter().map(|x: &i32| x * 2).filter(|x: &i32| x % 2 == 0).collect();
    println!("{} {}", a[0], b[0]);
}
```

* Answer:
    * `4 2`
* Reason:
    * Code trace this out!


---


# Question 15

Some setup code:

```rust
struct TestResult {
    /// Student's scores on a test
    scores: Vec<usize>,

    /// A possible value to curve all scores
    curve: Option<usize>
}
```


---


# Question 15

```rust
impl TestResult {
    pub fn get_curve(&self) -> &Option<usize> {
        &self.curve
    }

    /// If there is a curve, then increments all
    /// scores by the curve
    pub fn apply_curve(&mut self) {
        if let Some(curve) = self.get_curve() {
            for score in self.scores.iter_mut() {
                *score += *curve;
            }
        }
    }
}
```




---


# Question 15

```rust
/// If there is a curve, then increments all
/// scores by the curve
pub fn apply_curve(&mut self) {
    if let Some(curve) = self.get_curve() {
        for score in self.scores.iter_mut() {
            *score += *curve;
        }
    }
}
```

If you tried to compile this program, which of the following best describes the compiler error you would get?

 * in `get_curve`, cannot return a reference to a local variable `self.curve`
 * in `apply_curve`, cannot borrow `self.scores` as mutable for `iter_mut`
 * in `apply_curve`, cannot borrow `self` as immutable for `get_curve`
 * in `apply_curve`, `*score` cannot be mutated


---


# Question 15 Answer

* Answer:
    * in `apply_curve`, cannot borrow `self.scores` as mutable for `iter_mut`
* Reason:
    * First, try and figure out what the type of `curve` is
    * Second, reason about why the call to `iter_mut` won't compile


---


# Review Summary

* We didn't teach you how to answer those "undefined behavior" problems
* It is important to understand that Rust has all these rules for a good reason
* So what happens when we know our code is safe, but doesn't compile?
    * _If only we had a way to tell the compiler that we know something it doesn't..._


---


# Next Lecture: Smart Pointers and `unsafe`

![bg right:30% 80%](../images/ferris_happy.svg)

* Thanks for coming!
