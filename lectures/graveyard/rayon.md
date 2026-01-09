


# **Libraries and Frameworks**


---


# Libraries and Frameworks

Libraries can come in all different shapes and sizes, but there are several libraries that are so integral to the Rust that they are basically their own sub-ecosystems.

- **`serde`**
- `rayon`
- `tokio`
- `reqwest`
- `tracing`

<!--
Listed in no particular order. Well also only talk about the first 2
-->


---


# `rayon`

Write parallel code as easily as sequential code

---


# `rayon`: The Problem

Your sequential code:

```rust
let data = (1..1_000_000).collect::<Vec<i32>>();
let sum: i32 = data.iter().sum();
```

* Say we want to parallelize this
    * "Divide the vector into eight regions, spawn eight threads, give each thread a region, and combine their results..."


---


# `rayon`: The Problem

Question: do we really want to type all of this?

```rust
// ... some logic to divide the vector ...
let handle1 = thread::spawn(|| { /* work */ });
...
let handle8 = thread::spawn(|| { /* work */ });
handle1.join().unwrap();
...
handle8.join().unwrap();
// ... then we have to combine results...
```

* "Gee, `data.iter().sum()` was so clean!"
    * "If only we could type the equivalent of `data.iter().sum()`, and it'd be parallelized for us..."
    * We can!

---

# `rayon`: The Solution

Add `rayon`:

```toml
[dependencies]
rayon = "1.10"
```

---


# `rayon`: The Solution

Then change `iter().sum()`...

```rust
let data = (1..1_000_000).collect::<Vec<i32>>();
let sum: i32 = data.iter().sum();
```

---


# `rayon`: The Solution

...to `par_iter().sum()`!

```rust
let data = (1..1_000_000).collect::<Vec<i32>>();
let sum: i32 = data.par_iter().sum(); // done!
```

* No threading needed
* Automatically parallelized


---


# `rayon`: Chained Operations

Recall that we can chain operations for iterators:

```rust
let result = numbers.iter()
    .filter(|&&x| x % 2 == 0)
    .map(|x| x * x)
    .collect();
```

* We can parallelize these operations as well!

---


# `rayon`: Chained Operations

When we use `par_iter()`, subsequent operations are automatically parallelized:

```rust
let result = numbers.par_iter()
    .filter(|&&x| x % 2 == 0)
    .map(|x| x * x)
    .collect();
```

* `filter`, `map`, and `collect` are now parallelized via Rayon's implementation
* Yet it still reads like sequential code!


---


# `rayon`: Parallelized Sort

Need to sort a massive collection?

```rust
let mut data = (1..1_000_000).collect::<Vec<i32>>();
data.sort();
```

---


# `rayon`: Parallelized Sort

`rayon` to the rescue:

```rust
let mut data = (1..1_000_000).collect::<Vec<i32>>();
data.par_sort();
```

* Just change `sort()` to `par_sort()`


---

# How `rayon` Works

* Suspiciously simple
    * All we have to do is add `par_`?
* How does Rayon work its magic?

---


# How `rayon` Works: Thread Pool Basics

Rayon creates a global thread pool:
* One pool per program, initialized on first use
* Number of threads matches your CPU cores (e.g., 8)
* Threads are ready, waiting for work


---


# How `rayon` Works: Splitting the Work

When you call `par_iter()`:
* Data (e.g., vector of `numbers`) is split into chunks
* Each thread grabs a chunk to process (e.g., filter, map)
* No manual division needed!


---


# How `rayon` Works: Work-Stealing

Threads don't sit idle:
* If a thread finishes early, it _steals_ work from others
    * Balances load dynamically


---


# `rayon`: Key Idea

Rayon creates the illusion that you are writing sequential code
* Divides the labor and monitors utilization rates so you don't have to
* _Dynamically_ manages the load balancing for high efficiency


---
