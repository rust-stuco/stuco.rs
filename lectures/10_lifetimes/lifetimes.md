---
theme: default
title: Lifetimes
author: Rust StuCo
info: |
  Week 10 of Rust StuCo: Lifetimes.
colorSchema: auto
aspectRatio: 16/9
canvasWidth: 1280
fonts:
  sans: Noto Sans Variable
  mono: Noto Sans Mono Variable
  provider: none
lineNumbers: false
monaco: false
drawings:
  enabled: true
  persist: false
  presenterOnly: true
exportFilename: lifetimes
export:
  timeout: 60000
  withToc: true
layout: default
class: communism
---

## Intro to Rust Lang

# Lifetimes

<br>

---

# Today: Lifetimes

We've used the term "lifetime" a few times before, and today we're going to explore what exactly it means.

* What is `'a` lifetime?
* How to think about lifetimes
* Other perspectives...

---

# Lifetimes

Lifetimes are all about _borrows_.

* Informal definition:
**Lifetimes provide a way for Rust to validate references at compile time**

* Formal definition:
**Lifetimes are named regions of code that a reference must be valid for**

* Recall from the Ownership lecture:
  * "The scope of a reference **ends at the last point it is used**"
  * "The specific term for reference scopes are _lifetimes_"

---

# Why Do We Need Lifetimes?

Recall from Error Handling and Traits that Rust's type system is _affine_:

```rust
fn dup<T>(x: T) -> (T, T) {
    (x, x) // error: use of moved value: `x`
}
```

* A value can be used at most once, so ownership alone prevents double frees
* But borrowing with `&x` doesn't count as a use, and `&T` is `Copy`
  * We can copy a reference as many times as we want...
* Something must stop those copies from outliving the value they point to
  * That something is lifetimes!

---

# Lifetimes vs. Generics

Lifetimes are a kind of generic parameter, similar to the `T` in `Vec<T>`.

* A type parameter `T` lets one function work with many types
* A lifetime parameter `'a` lets one function work with references of many lifetimes
* Recall `fn same<T>(a: T, b: T)`: one `T` forces both arguments to be the same type
* Similarly, one `'a` on two references ties their lifetimes together
* Unlike `T`, lifetimes are **not** monomorphized
  * They only exist at compile time, and are erased afterward
  * No extra copies of functions, and no runtime cost!

---
class: image-right image-width-25
---

# Validating References

<img class="slide-image" style="--image-size: 0.75;" src="../images/ferris_does_not_compile.svg">

The main goal of lifetimes is to prevent _dangling references_.

```rust
fn main() {
    let r;

    {
        let x = 5;
        r = &x;
    }

    println!("r: {}", r);
}
```

* What is the issue with this code?

---

# Validating References

```ansi
[1m[91merror[E0597][0m[1m: `x` does not live long enough[0m
 [1m[94m--> [0msrc/main.rs:6:13
  [1m[94m|[0m
[1m[94m5[0m [1m[94m|[0m         let x = 5;
  [1m[94m|[0m             [1m[94m-[0m [1m[94mbinding `x` declared here[0m
[1m[94m6[0m [1m[94m|[0m         r = &x;
  [1m[94m|[0m             [1m[91m^^[0m [1m[91mborrowed value does not live long enough[0m
[1m[94m7[0m [1m[94m|[0m     }
  [1m[94m|[0m     [1m[94m-[0m [1m[94m`x` dropped here while still borrowed[0m
[1m[94m8[0m [1m[94m|[0m
[1m[94m9[0m [1m[94m|[0m     println!("r: {}", r);
  [1m[94m|[0m                       [1m[94m-[0m [1m[94mborrow later used here[0m
```

* `x` is dropped at the end of the inner scope...
* ...but `r` still refers to `x` when we print it

---
class: image-right image-width-25
---

# The Borrow Checker

<img class="slide-image" style="--image-size: 0.75;" src="../images/ferris_does_not_compile.svg">

The Rust compiler's borrow checker compares how long each borrow is used against how long the borrowed value lives.

Here is the same code, but with a lifetime diagram:

```rust
fn main() {
    let r;
    {
        let x = 5;        // ─┐ x
        r = &x;           //  │ ─┐ 'a
    }                     // ─┘  │
    println!("r: {}", r); //    ─┘
}
```

---
class: image-right image-width-20
---

# The Borrow Checker

<img class="slide-image" style="--image-size: 0.75;" src="../images/ferris_does_not_compile.svg">

The borrow checker compares two different regions:

```rust
fn main() {
    let r;
    {
        let x = 5;        // ─┐ x
        r = &x;           //  │ ─┐ 'a
    }                     // ─┘  │
    println!("r: {}", r); //    ─┘
}
```

* `x`'s scope: ends when `x` is dropped at the inner `}`
* `'a`: the borrow `&x`, from its creation to the last use of `r`
* Rejects because `x` does not _outlive_ `'a`

---
class: image-right image-width-25
---

# Placating the Borrow Checker

<img class="slide-image" style="--image-size: 0.75;" src="../images/ferris_happy.svg">

We can fix this code by removing the scope.

```rust
fn main() {
    let r;
    let x = 5;            // ─┐ x
    r = &x;               //  │ ─┐ 'a
    println!("r: {}", r); //  │ ─┘
}                         // ─┘
```

* `x` now outlives `'a`, so `r` can reference `x`

---
class: image-right image-width-25
---

# Lifetimes vs. Scope

<img class="slide-image" style="--image-size: 0.75;" src="../images/ferris_happy.svg">

We could also fix it by using `r` earlier.

```rust
fn main() {
    let r;
    {
        let x = 5;            // ─┐ x
        r = &x;               //  │ ─┐ 'a
        println!("r: {}", r); //  │ ─┘
    }                         // ─┘
}
```

* `r` is in scope until the end of `main`, but `'a` ends at the last use of `r`
* Recall from Closures and Iterators: a borrow ends at its **last use**, not at the end of the scope

---

# Story Time

_Long ago, in the dark ages of 2018, a borrow lasted until the end of its scope._

```rust
let mut v = vec![1, 2, 3];
let first = &v[0];
println!("{}", first);
v.push(4); // Error before Rust 2018!
```

* Rust used to have _lexical lifetimes_: a borrow lasted until the `}` of the scope it was declared in
  * Programmers added extra `{ }` blocks just to end borrows early
* Rust 2018 introduced _non-lexical lifetimes_ (NLL)
  * The compiler tracks where each reference is last used instead
  * This is the "last use" rule we've relied on since the Ownership lecture

<!--
NLL shipped with the 2018 edition in Rust 1.31, and was enabled for the 2015 edition in Rust 1.36.
-->

---

# Generic Lifetimes

Let's try to write some string functions.

```rust
fn main() {
    let string1 = String::from("abcd");
    let string2 = "xyz";

    let result = longest(string1.as_str(), string2);
    println!("The longest string is {}", result);
}
```

We want this output:

```
The longest string is abcd
```

* Let's implement `longest`!

---
class: image-right image-width-25
---

# `longest`

<img class="slide-image" style="--image-size: 0.75;" src="../images/ferris_does_not_compile.svg">

Here is a first attempt:

```rust
fn longest(x: &str, y: &str) -> &str {
    if x.len() > y.len() {
        x
    } else {
        y
    }
}
```

* _We don't want to take ownership, so we take `&str` inputs_

---

# `longest` Error

Unfortunately, our attempt will not compile:

```ansi
[1m[91merror[E0106][0m[1m: missing lifetime specifier[0m
 [1m[94m--> [0msrc/main.rs:9:33
  [1m[94m|[0m
[1m[94m9[0m [1m[94m|[0m fn longest(x: &str, y: &str) -> &str {
  [1m[94m|[0m               [1m[94m----[0m     [1m[94m----[0m     [1m[91m^[0m [1m[91mexpected named lifetime parameter[0m
  [1m[94m|[0m
  [1m[94m= [0m[1mhelp[0m: this function's return type contains a borrowed value,
    but the signature does not say whether it is borrowed from `x` or `y`
[1m[96mhelp[0m: consider introducing a named lifetime parameter
  [1m[94m|[0m
[1m[94m9[0m [1m[94m| [0mfn longest[92m<'a>[0m(x: &[92m'a [0mstr, y: &[92m'a [0mstr) -> &[92m'a [0mstr {
  [1m[94m|[0m           [92m++++[0m     [92m++[0m          [92m++[0m          [92m++[0m
```

---

# `longest` Error

The help text from the compiler error reveals some useful information:

```ansi
  [1m[94m= [0m[1mhelp[0m: this function's return type contains a borrowed value,
    but the signature does not say whether it is borrowed from `x` or `y`
```

* Rust can't figure out if the reference returned refers to `x` or `y`
* In fact, neither do we!

---
class: image-right image-width-20
---

# `longest` Error

<img class="slide-image" style="--image-size: 0.9;" src="../images/ferris_does_not_compile.svg">

```rust
fn longest(x: &str, y: &str) -> &str {
    if x.len() > y.len() {
        x
    } else {
        y
    }
}
```

* We don't know which execution path this code will take
* We also don't know the lifetimes of the input references
* Thus we cannot determine the lifetime we return!
* We will need to _annotate_ these references

---
class: image-right image-width-20
---

# What Could Go Wrong?

<img class="slide-image" style="--image-size: 0.9;" src="../images/ferris_does_not_compile.svg">

Suppose Rust guessed that the result always borrows from `x`:

```rust
let s1 = String::from("short");
let result;
{
    let s2 = String::from("much longer string");
    result = longest(&s1, &s2); // Actually returns `s2`!
}                               // `s2` is dropped and freed
println!("{result}");           // Use after free!
```

* The borrow checker thinks `result` only borrows `s1`, so it allows this
* Guessing `y` also fails, depending on the string lengths at runtime

<!--
References never free anything, so dangling references cause use-after-frees, never double frees.
-->

---

# Lifetime Annotation Syntax

We can annotate lifetimes with generic parameters that start with a `'`, like `'a`.

```rust
&i32          // a reference
&'a i32       // a reference with an explicit annotated lifetime
&'a mut i32   // a mutable reference with an explicit lifetime

&'hello usize // annotations can be any word or character,
&'world bool  // as long as it starts with a tick (')
```

* Annotations don't change how long references live, they only describe _relationships_ between lifetimes
* `x: &'a i32`: `x` can only be used during `'a`, and the `i32` it refers to must be valid for all of `'a`
* The borrow checker checks that borrowed values outlive their borrows

---
class: image-right image-width-20
---

# `longest` Lifetimes

<img class="slide-image" style="--image-size: 0.9;" src="../images/ferris_does_not_compile.svg">

Let's return back to our `longest` function.

```rust
fn longest(x: &str, y: &str) -> &str {
    if x.len() > y.len() {
        x
    } else {
        y
    }
}
```

* What do we want the function signature to express?
* What should the relationship be between the lifetimes of the references?

---

# `longest` Lifetimes

What exactly are we returning?

```rust
if x.len() > y.len() {
    x
} else {
    y
}
```

* We return either `x` or `y`, which each have their own lifetimes
* The returned reference should only be valid while _both_ `x` and `y` are valid
* So we want lifetimes of `x` and `y` to _outlive_ the returned lifetime

<!--
In other words, we do not want the thing we return to outlive `x` or `y`
-->

---
class: image-right image-width-25
---

# `longest` Lifetimes

<img class="slide-image" style="--image-size: 0.75;" src="../images/ferris_happy.svg">

Since lifetimes are a kind of generic parameter, we must declare them like normal generic type parameters.

```rust
fn longest<'a>(x: &'a str, y: &'a str) -> &'a str {
    if x.len() > y.len() {
        x
    } else {
        y
    }
}
```

* This will compile now!
* _Note that these lifetime annotations don't **change** any lifetimes, they just **restrict**_

<!--
They just tell the borrow checker to reject any values that don't adhere to these constraints/invariants
-->

---

# Lifetime Annotations in Functions

We can extrapolate a lot from a function's signature, even without the body.

```rust
fn longest<'a>(x: &'a str, y: &'a str) -> &'a str;
```

* This function takes two string slices (`x` and `y`) that are both valid for at least the lifetime `'a`
* The string slice returned (the longer of `x` or `y`) is also valid for `'a`, and cannot be used after `'a` ends

---

# Lifetime Annotations in Functions

```rust
fn longest<'a>(x: &'a str, y: &'a str) -> &'a str;
```

* When calling `longest`, the borrow checker picks a region for `'a` where both `x` and `y` are valid (the overlap of their lifetimes)
* In practice, this means the lifetime returned by `longest` is the same as the **smaller** of the two input lifetimes

<!--
Be clear that functions do not "return" lifetimes, this is just for slide space.

Lifetimes are not totally ordered: two lifetimes can partially overlap, with neither containing the other.
That's why `'a` is really "the overlap", and only "the smaller one" when one lifetime contains the other.

Just like the caller picks `T` for a generic function, the caller picks `'a`.
`longest` doesn't need to know exactly how long x and y will live, only that some region can be substituted for 'a that satisfies this signature.
-->

---

# Borrow Checker Example 1

Let's look at some examples where the borrow checker is and isn't happy.

```rust
let string1 = String::from("long string is long");

{
    let string2 = String::from("xyz");

    let result = longest(string1.as_str(), string2.as_str());
    println!("The longest string is {}", result);
}
```

* `string1` is valid in the outer scope, `string2` is valid in the inner scope
* `result` should only be valid in the smaller scope (by our lifetime annotations)
  * Since `println!` is in the smaller (inner) scope, this works!

---
class: image-right image-width-20
---

# Borrow Checker Example 2

<img class="slide-image" style="--image-size: 0.9;" src="../images/ferris_does_not_compile.svg">

Let's reorder some things around.

```rust
let result;
let string1 = String::from("xyz");

{
    let string2 = String::from("long string is long");
    result = longest(string1.as_str(), string2.as_str());
}

println!("The longest string is {}", result);
```

* `result` should only be valid in the smaller (inner) scope, but we try to reference it in the outer scope

<!--
`result` doesn't need to be `mut` because we haven't assigned it anything yet
-->

---

# Borrow Checker Example 2

Sure enough, this does not compile! Rust gives us this error:

```ansi
[1m[91merror[E0597][0m[1m: `string2` does not live long enough[0m
  [1m[94m--> [0msrc/main.rs:7:44
   [1m[94m|[0m
[1m[94m 6[0m [1m[94m|[0m         let string2 = String::from("long string is long");
   [1m[94m|[0m             [1m[94m-------[0m [1m[94mbinding `string2` declared here[0m
[1m[94m 7[0m [1m[94m|[0m         result = longest(string1.as_str(), string2.as_str());
   [1m[94m|[0m                                            [1m[91m^^^^^^^[0m [1m[91mborrowed value does
                                                        not live long enough[0m
[1m[94m 8[0m [1m[94m|[0m     }
   [1m[94m|[0m     [1m[94m-[0m [1m[94m`string2` dropped here while still borrowed[0m
[1m[94m 9[0m [1m[94m|[0m
[1m[94m10[0m [1m[94m|[0m     println!("The longest string is {}", result);
   [1m[94m|[0m                                          [1m[94m------[0m [1m[94mborrow later used here[0m
```

---
class: image-right image-width-20
---

# Borrow Checker Example 3

<img class="slide-image" style="--image-size: 0.9;" src="../images/ferris_does_not_compile.svg">

What if we knew (as the programmer) that `string1` is always longer than `string2`?

Let's switch the strings around:

```rust
let result;
let string1 = String::from("long string is long"); // Longer!

{
    let string2 = String::from("xyz"); // Shorter!
    result = longest(string1.as_str(), string2.as_str());
}

println!("The longest string is {}", result);
```

---
class: image-right image-width-20
---

# Borrow Checker Example 3

<img class="slide-image" style="--image-size: 0.9;" src="../images/ferris_does_not_compile.svg">

```rust
let result;
let string1 = String::from("long string is long"); // Longer!

{
    let string2 = String::from("xyz"); // Shorter!
    result = longest(string1.as_str(), string2.as_str());
}

println!("The longest string is {}", result);
```

* Even though we know (as humans) that the reference will be valid, the borrow checker rejects it
* At the call site, the borrow checker only looks at `longest`'s _signature_, not its body

<!--
* We even told the compiler that the returned lifetime would be the same as the smaller of the input lifetimes!
-->

---

# Signatures Are Contracts

The borrow checker checks lifetimes one function at a time.

```rust
fn longest<'a>(x: &'a str, y: &'a str) -> &'a str;
```

* Inside `longest`: the body must return something valid for `'a`, and both `x` and `y` qualify
* At the call site: the result may borrow from _either_ argument, so both must stay valid while it's used
* The compiler doesn't run our code, so it can't know which branch will be taken
* Recall trait bounds from Error Handling and Traits: a generic function can only rely on what its bounds promise, and callers only need to satisfy those bounds

---

# Annotating Only What We Need

Suppose we wanted to always return the first input, `x`.

```rust
fn first<'a>(x: &'a str, y: &str) -> &'a str {
    x
}
```

* We don't need to annotate `y` with `'a`, because the return value never borrows from `y`
* `y` still has a lifetime, we just didn't write it down
  * We'll see how Rust fills it in soon!

---
class: image-right image-width-20
---

# Lifetimes of Return Values

<img class="slide-image" style="--image-size: 0.9;" src="../images/ferris_does_not_compile.svg">

A returned reference must borrow from one of the inputs.

```rust
fn dangling<'a>(x: &str, y: &str) -> &'a str {
    let result = String::from("really long string");
    result.as_str() // What lifetime would this have?
}
```

* `result` is dropped when `dangling` returns, so this would dangle
* Exception: data that lives forever, like string literals (`'static`, later!)

<!--
In other words, all valid references must be _derived_ from other valid references, or from data that is never freed.
`fn f<'a>(x: &str, y: &str) -> &'a str { "literal" }` compiles!
-->

---

# Lifetimes of Return Values

Recall `dangle` from the Ownership lecture:

```rust
fn dangle() -> &String { // error[E0106]: missing lifetime specifier
    let s = String::from("hello");
    &s
}
```

* There are no inputs, so there is nothing for the return value to borrow from
* The compiler suggests `'static`, but `s` is still dropped when `dangle` returns
* The real fix is to return an owned `String`

---

# Lifetime Elision

All references must have a lifetime. But we've seen many references without lifetime annotations...

```rust
fn first_word(s: &str) -> &str {
    let bytes = s.as_bytes();

    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return &s[0..i];
        }
    }

    &s[..]
}
```

* There are no lifetime annotations here!

<!--
We saw `first_word`'s signature in the Ownership lecture when we introduced slices
-->

---

# More Story Time

_In the dark ages before Rust 1.0, every returned reference needed an explicit lifetime._

```rust
fn first_word<'a>(s: &'a str) -> &'a str {
```

* Lifetimes on inputs could already be left out, but not lifetimes on return values
* This became incredibly repetitive, so in 2014 [RFC 141](https://rust-lang.github.io/rfcs/0141-lifetime-elision.html) added rules that fill in the most common patterns
* These patterns are called the _lifetime elision rules_

<!--
RFC 141 surveyed the standard library: the new rules removed 87% of the lifetime annotations that were required at the time.
-->

---

# Lifetime Elision

* Lifetime elision is not inference: the compiler fills in the signature using fixed rules, and gives an error if they don't determine every output lifetime
* Lifetimes on function or method arguments are called **input lifetimes**, and lifetimes on return values are called **output lifetimes**
* There are only 3 lifetime elision rules, the first for input lifetimes, the last two for output lifetimes

---

# Lifetime Elision Rule 1

The first rule is that the compiler assigns a different lifetime parameter to each lifetime left out of the inputs.

```rust
fn foo(x: &i32);
fn foo<'a>(x: &'a i32);

fn bar(x: &i32, y: &i32, z: i32);
fn bar<'a, 'b>(x: &'a i32, y: &'b i32, z: i32);

fn baz(x: &&i32);
fn baz<'a, 'b>(x: &'a &'b i32);
```

* A reference to a reference has two lifetimes!

---

# Lifetime Elision Rule 2

The second rule is that if there is only 1 input lifetime parameter, then it is assigned to all output lifetimes.

```rust
fn foo(x: &i32) -> &i32;
fn foo<'a>(x: &'a i32) -> &'a i32;

fn bar(arr: &[i32]) -> (&i32, &i32);
fn bar<'a>(arr: &'a [i32]) -> (&'a i32, &'a i32);
```

---

# Lifetime Elision Rule 3

If there are multiple input lifetime parameters, but the first parameter is `&self` or `&mut self`, the lifetime of `&self` is assigned to all output lifetimes.

* This only applies to methods
* Makes writing methods much nicer!
* _Examples to come later..._

---

# Lifetime Elision Example 1

Let's pretend we are the compiler, and let's attempt to apply the lifetime elision rules to `first_word`.

```rust
fn first_word(s: &str) -> &str;
```

---

# Lifetime Elision Example 1

We apply the first rule, which specifies that each parameter gets its own lifetime.

```rust
fn first_word<'a>(s: &'a str) -> &str;
```

---

# Lifetime Elision Example 1

The second rule specifies that the lifetime of the single input parameter gets assigned to all output lifetimes, so the signature becomes this:

```rust
fn first_word<'a>(s: &'a str) -> &'a str;
```

* Since all references have lifetime annotations, we're done!

---

# Lifetime Elision Example 2

So why didn't elision work with `longest`? Let's trace it out!

We start with this signature without annotations:

```rust
fn longest(x: &str, y: &str) -> &str;
```

---

# Lifetime Elision Example 2

Let's apply the first rule and get annotations for all inputs.

```rust
fn longest<'a, 'b>(x: &'a str, y: &'b str) -> &str;
```

* What now?

---

# Lifetime Elision Example 2

```rust
fn longest<'a, 'b>(x: &'a str, y: &'b str) -> &str;
```

* The second rule doesn't apply here, because there is more than 1 input lifetime (`'a` and `'b`)

* The third rule doesn't apply either, because there is no `&self`

* Since Rust cannot figure out what to do, it gives a compiler error to the programmer so they can write the annotations themselves

---

# Lifetimes in Structs

So far, all of the `struct`s we've looked at have held _owned_ type fields.

If we want a `struct` to hold a reference, we need to annotate them.

```rust
struct ImportantExcerpt<'a> {
    part: &'a str,
    importance: i32,
}

let novel = String::from("Call me Ishmael. Some years ago...");
let first_sentence: &str = novel.split('.').next().expect("No periods");

let important = ImportantExcerpt {
    part: first_sentence,
    importance: 42,
};
```

---

# Lifetimes in Structs

```rust
struct ImportantExcerpt<'a> {
    part: &'a str,
    importance: i32,
}
```

* As with generic data types, we declare the name of the generic lifetime parameter inside angle brackets

* This annotation means an instance of `ImportantExcerpt` can’t outlive the reference it holds in its `part` field

---

# Lifetimes in `impl` Blocks

Similarly, we need to annotate `impl` blocks with lifetime parameters.

```rust
impl<'a> ImportantExcerpt<'a> {
    fn importance(&self) -> i32 {
        self.importance
    }
}
```

---

# Lifetimes in Methods

Here is an example where the third elision rule is applied:

```rust
impl<'a> ImportantExcerpt<'a> {
    fn announce_and_return_part(&self, announcement: &str) -> &str {
        println!("Attention please: {}", announcement);
        self.part
    }
}
```

* The first rule gives both `&self` and `announcement` their own lifetimes
* The third rule gives the return lifetime the lifetime of `&self`

---

# Putting it all together

Let’s briefly look at the syntax of specifying generic type parameters, trait bounds, and lifetimes all in one function!

```rust
fn longest_with_an_announcement<'a, T>(x: &'a str, y: &'a str, ann: T) -> &'a str
where
    T: Display,
{
    println!("Announcement! {}", ann);

    if x.len() > y.len() {
        x
    } else {
        y
    }
}
```

---

# Lifetime Bounds

Lifetimes can be bounds, just like traits.

```rust
#[derive(Debug)]
struct Ref<'a, T: 'a>(&'a T);
```

* `Ref` contains a reference, with a lifetime of `'a`, to a generic type `T`
* The syntax `&'a T` specifies that the argument of type `T` has a lifetime of `'a`.
* `'a` is determined by the lifetime of the argument passed by the caller.
* The bound `T: 'a` specifies that any reference _in_ `T` must outlive `'a`.

<!--
Note that these lifetime bound slides are REALLY not that important.
-->

---

# Lifetime Bounds

Here is a similar example, but with a function instead of a `struct`.

```rust
fn print_ref<'a, T>(t: &'a T)
where
    T: Debug + 'a,
{
    println!("print_ref(t) is {:?}", t);
}
```

* The syntax `&'a T` specifies that the argument of type `T` has a lifetime of `'a`.
* `'a` is determined when the struct is instantiated.
* The bound `T: Debug + 'a` specifies that any reference _in_ `T` must outlive `'a` (and that `T` must implement `Debug`).

---

# Lifetime Bounds

Putting the `Ref` and `print_ref` together:

```rust
#[derive(Debug)]
struct Ref<'a, T: 'a>(&'a T);

fn print_ref<'a, T>(t: &'a T) where T: Debug + 'a {
    println!("print_ref(t) is {:?}", t);
}

let x = vec![9, 8, 0, 0, 8];
let ref_x = Ref(&x);
print_ref(&ref_x);
```

```
print_ref(t) is Ref([9, 8, 0, 0, 8])
```

<!--
This is quite difficult to understand, don't spend too much time here.
-->

---

# Lifetime-bounded Lifetimes

We can have lifetimes that are bounded by other lifetimes.

```rust
// Takes in a `&'a i32` and return a `&'b i32` as a result of coercion
fn choose_first<'a: 'b, 'b>(first: &'a i32, _: &'b i32) -> &'b i32 {
    first
}

fn main() {
    let first = 2; // Longer lifetime
    {
        let second = 3; // Shorter lifetime
        println!("{} is the first", choose_first(&first, &second));
    }
}
```

* `'a: 'b` reads as "lifetime `'a` outlives `'b`"
    * This is needed if we pass an argument to another argument.

<!--
'a outlives 'b == 'a is at least as long as 'b
-->

---

# The `'static` Lifetime

There is a special lifetime called `'static`.

```rust
let s: &'static str = "I have a static lifetime";
```

* `'static` implies that the reference will live until the end of the program (it is valid until the program stops running)
* Here, `s` is stored in the program binary, so it will always be valid!

---

# `'static` Error Messages

You may see suggestions to use the `'static` lifetime in error messages.

```rust
fn foo() -> &i32 {
    let x = 5;
    &x
}
```

```ansi
[1m[96mhelp[0m: consider using the `'static` lifetime, but this is uncommon unless you're
      returning a borrowed value from a `const` or a `static`
  [1m[94m|[0m
[1m[94m2[0m [1m[94m| [0mfn foo() -> &[92m'static [0mi32 {
  [1m[94m|[0m              [92m+++++++[0m
```

---

# `'static` Error Messages

```ansi
[1m[96mhelp[0m: consider using the `'static` lifetime, but this is uncommon unless you're
      returning a borrowed value from a `const` or a `static`
  [1m[94m|[0m
[1m[94m2[0m [1m[94m| [0mfn foo() -> &[92m'static [0mi32 {
  [1m[94m|[0m              [92m+++++++[0m
```

* Before making a change, think about if your reference will _really_ live until the end of the program
* You may actually be trying to create a dangling reference!

---

# `'static` vs `static`

There are two common ways to make a variable with a `'static` lifetime.

1) Make a string literal with has type `&'static str`
2) Make a constant with the `static` declaration

---

# `'static` vs `static` Example

```rust
static NUM: i32 = 42;
static NUM_REF: &'static i32 = &NUM;

fn main() {
    let msg: &'static str = "Hello World";
    println!("{msg} {NUM_REF}!");
}
```

```
Hello World 42!
```

<!--
If time allows, explain the difference between `const` and `static`.
-->

---

# The `'static` Bound

`'static` can also be used as a type bound. However...

* There is a subtle difference between the `'static` lifetime and the `'static` bound.
* The `'static` bound means that the type does not contain any non-static references (including itself).
* This means that all owned data implicitly has a `'static` bound, since owned data holds no references.

<!--
Does not imply contrapositive
-->

---
class: image-right image-width-20
---

# `'static` Bound Example 1

<img class="slide-image" style="--image-size: 0.9;" src="../images/ferris_does_not_compile.svg">

Here's an example of using a `'static` bound.

```rust
fn print_it(input: impl Debug + 'static) {
    println!("'static value passed in is: {:?}", input);
}

fn main() {
    let i = 5;
    print_it(i);
    print_it(&i);
}
```

---

# `'static` Bound Example 1

We get a compiler error:

```ansi
[1m[91merror[E0597][0m[1m: `i` does not live long enough[0m
  [1m[94m--> [0msrc/main.rs:15:14
<-- snip -->
[1m[94m15[0m [1m[94m|[0m     print_it(&i);
   [1m[94m|[0m     [1m[94m---------[0m[1m[91m^^[0m[1m[94m-[0m
   [1m[94m|[0m     [1m[94m|[0m        [1m[91m|[0m
   [1m[94m|[0m     [1m[94m|[0m        [1m[91mborrowed value does not live long enough[0m
   [1m[94m|[0m     [1m[94margument requires that `i` is borrowed for `'static`[0m
[1m[94m16[0m [1m[94m|[0m }
   [1m[94m|[0m [1m[94m-[0m [1m[94m`i` dropped here while still borrowed[0m
[1m[92mnote[0m: requirement that the value outlives `'static` introduced here
  [1m[94m--> [0msrc/main.rs:3:33
   [1m[94m|[0m
[1m[94m 3[0m [1m[94m|[0m fn print_it(input: impl Debug + 'static) {
   [1m[94m|[0m                                 [1m[92m^^^^^^^[0m
```


---
class: image-right image-width-20
---

# `'static` Bound Example 2

<img class="slide-image" style="--image-size: 0.9;" src="../images/ferris_does_not_compile.svg">

```rust
fn print_with_static_lifetime(v: &'static Vec<i32>) {
    println!("{:?}", v);
}

fn main() {
    let v: Vec<i32> = vec![1,2,3];
    print_with_static_lifetime(&v);
}
```

---

# `'static` Bound Example 2


```rust
fn print_with_static_bound(v: impl Debug + 'static) {
    println!("{:?}", v);
}

fn main() {
    let v: Vec<i32> = vec![1,2,3];
    print_with_static_bound(v);
}
```

---
class: image-right image-width-20
---

# `'static` Bound Example 2

<img class="slide-image" style="--image-size: 0.9;" src="../images/ferris_does_not_compile.svg">

```rust
fn print_with_static_bound(v: impl Debug + 'static) {
    println!("{:?}", v);
}

fn main() {
    let v: Vec<i32> = vec![1,2,3];
    print_with_static_bound(&v);
}
```

---

# Another Perspective

* Thinking about lifetimes as regions of code can be confusing
* Instead, can we think about lifetimes as regions of valid memory?
  * This is another valid interpretation!

---

# Another Perspective

```rs
fn longest<'a>(s1: &'a str, s2: &'a str) -> &'a str;

fn main() {
    let x: &'x str = "A";
    let y: &'y str = "B";
    let z: &'z str = "C";

    let l1: &'l1 str = longest(x, y);
    let l2: &'l2 str = longest(l1, z);
}
```

---

# Another Perspective

How about this one?

```rs
fn longest<'s1, 's2, 'out>(s1: &'s1 str, s2: &'s2 str) -> &'out str;
```

---

# Recap

* Rust has lifetimes to prevent dangling references
* The borrow checker will ensure that lifetimes are always valid
* Rust will allow you elide lifetime annotations in some situations

---
layout: none
---

<EndingSlide next-lecture="Smart Pointers and Trait Objects" />
