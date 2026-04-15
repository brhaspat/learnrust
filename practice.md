# Rust Practice Guide

Companion to `src/practice.rs`. Every section below maps to a test module you can run right now.

```bash
cargo test            # run all 97 tests
cargo test s10        # run just one section
cargo test -- --list  # see all test names
```

---

## §3 · Variables & Mutability
**Module:** `s03_variables`

The biggest daily-driver difference from Go: variables are **immutable by default**.

| Go | Rust |
|---|---|
| `x := 10` (mutable) | `let x = 10` (immutable) |
| `var x int = 10` (mutable) | `let mut x = 10` (mutable) |
| `const Y = 30` | `const Y: u32 = 30` (type required) |
| — | `static ADDR: &str = "..."` (fixed address) |

**Shadowing** is a Rust superpower that Go doesn't have. `let x = x + 1` creates a *new* binding — it doesn't mutate the old one. The key difference from `mut`: shadowing can change the **type**:

```rust
let input = "42";               // &str
let input: i32 = input.parse().unwrap(); // now i32 — same name, new binding
```

### Tests
| Test | What it proves |
|---|---|
| `immutable_vs_mutable` | `let` is immutable; `let mut` opts in |
| `shadowing_same_type` | same-type shadow: 5 → 6 → 12 |
| `shadowing_changes_type` | shadow can change type; `mut` cannot |
| `const_and_static` | `const` needs type annotation; `static` has a fixed address |

### Try this
In `immutable_vs_mutable`, add `x = 20;` after `let x = 10;` and run `cargo build`. Read the compiler error — it tells you exactly what to do.

---

## §4 · Types & Type System
**Module:** `s04_types`

Rust **never** does implicit numeric conversion. Every cast is written out.

| Operation | Rust | Notes |
|---|---|---|
| Widen | `x as i64` | always safe |
| Truncate | `x as u8` | **silent** on overflow — use carefully |
| Safe convert | `x.try_into()` | returns `Result` — panics if you `.unwrap()` on overflow |

`usize` is pointer-sized (like Go's `int`). Always use it for indices and lengths.

Tuples are first-class in Rust. Go uses multiple returns instead; Rust has both.

### Tests
| Test | What it proves |
|---|---|
| `explicit_numeric_cast` | widening cast with `as` |
| `as_truncates_silently` | `300 as u8 == 44` — truncation, no panic |
| `try_into_is_safe` | `300.try_into::<u8>()` → `Err`, `200` → `Ok(200)` |
| `tuples` | create, destructure, index a `(i32, &str)` tuple |
| `usize_for_indexing` | why `v[i]` requires `usize` |

### Try this
Change `let big: i32 = 300;` to `301` in `as_truncates_silently` and calculate the expected result: `301 % 256 = 45`. Run `cargo test s04` and verify.

---

## §5 · Functions & Methods
**Module:** `s05_functions`

The last expression in a block **without a semicolon** is the return value. Use `return` only for early exits.

```rust
fn add(a: i32, b: i32) -> i32 {
    a + b   // implicit return — no semicolon
}

fn divide(a: f64, b: f64) -> f64 {
    if b == 0.0 { return f64::INFINITY; }  // early return
    a / b
}
```

Rust has **no variadic functions** (`...args` in Go). Use a slice `&[T]` parameter instead — a `Vec` can be passed as `&v`.

### Tests
| Test | What it proves |
|---|---|
| `test_add` | implicit return (no semicolon) |
| `test_divide` | early `return` for the special case |
| `test_swap` | returning a tuple = Go's multiple returns |
| `test_sum_slice` | `&[i32]` accepts both array literals and `&vec` |

### Try this
Add a semicolon after `a + b` in `add` — i.e., `a + b;` — and run `cargo build`. The compiler will say the function returns `()` instead of `i32`. This is how you accidentally return the unit type in Rust.

---

## §6 · Control Flow
**Module:** `s06_control_flow`

`if` is an **expression** — assign it directly. Rust has no ternary operator:

```rust
// Rust
let label = if x > 0 { "positive" } else { "non-positive" };

// Go equivalent
var label string
if x > 0 { label = "positive" } else { label = "non-positive" }
```

`loop` can **return a value** via `break`:
```rust
let result = loop {
    counter += 1;
    if counter == 10 { break counter * 2; }
};
```

`match` is Go's `switch` but exhaustive — the compiler errors if any case is missing.

### Tests
| Test | What it proves |
|---|---|
| `if_as_expression` | `if` as a value (no ternary needed) |
| `for_exclusive_range` | `0..5` = 0,1,2,3,4 |
| `for_inclusive_range` | `1..=5` = 1,2,3,4,5 |
| `for_enumerate` | `iter().enumerate()` = Go's `for i, v := range` |
| `loop_returns_value` | `break counter * 2` returns from the loop |
| `match_is_exhaustive` | OR `\|`, range `3..=9`, guard `if n < 0`, wildcard `_` |
| `if_let_shorter_than_match` | `if let Some(v) = x` — cleaner than full match |
| `while_let_pops_until_empty` | loop until pattern stops matching |

### Try this
In `match_is_exhaustive`, remove the `_ => ...` wildcard arm. `cargo build` will error: "non-exhaustive patterns". Add a case for `10` and see if `_` is still required.

---

## §7 · Structs & Impl
**Module:** `s07_structs`

`impl` blocks attach three kinds of functions to a type:

| Receiver | Syntax | Meaning |
|---|---|---|
| None | `fn new(...) -> Self` | Associated function (constructor) — called as `Type::new()` |
| `&self` | `fn foo(&self)` | Immutable method — reads but can't change |
| `&mut self` | `fn foo(&mut self)` | Mutable method — can modify fields |
| `self` | `fn foo(self)` | Consuming method — takes ownership; struct is dropped after |

`#[derive(Debug)]` auto-generates `{:?}` formatting — equivalent to Go's `%+v`.

Struct update syntax `..s` copies remaining fields — like `s2 := s; s2.Port = 443` in Go.

### Tests
| Test | What it proves |
|---|---|
| `constructor_and_method` | `Server::new(...)` + `&self` method |
| `mutable_method` | `&mut self` modifies a field |
| `consuming_method_moves_struct` | `self` by value — struct unusable after |
| `struct_update_syntax` | `Server { port: 443, ..s1 }` |
| `tuple_struct` | `Rgb(255, 0, 0)` — fields by `.0`, `.1`, `.2` |
| `debug_derive` | `{:?}` works via `#[derive(Debug)]` |

### Try this
In `consuming_method_moves_struct`, add `println!("{}", s.host)` **after** `s.into_address()`. Run `cargo build` — the compiler will say `s` was moved. This is the ownership system catching a use-after-free at compile time.

---

## §8 · Traits vs Interfaces
**Module:** `s08_traits`

| | Go interfaces | Rust traits |
|---|---|---|
| Satisfaction | Implicit (duck typing) | Explicit `impl Trait for Type` |
| Default methods | No | Yes |
| Dispatch | Always dynamic (vtable) | Static by default; `dyn` for dynamic |

**Static dispatch** (`impl Trait` or generics) — compiler generates a separate copy per type. Zero runtime overhead.

**Dynamic dispatch** (`dyn Trait`) — single copy, vtable at runtime. Same as a Go interface variable. Enables `Vec<Box<dyn Trait>>`.

### Tests
| Test | What it proves |
|---|---|
| `static_dispatch` | `&impl Greet` — compiler resolves at compile time |
| `dynamic_dispatch_heterogeneous_vec` | `Vec<Box<dyn Greet>>` holds mixed types |
| `default_vs_overridden_method` | default used vs overridden |
| `multiple_trait_bounds` | `T: Greet + fmt::Debug` — both required |

### Try this
Add a `struct German;` that implements `Greet`. Add it to `greeters` in `dynamic_dispatch_heterogeneous_vec`. You only need to write `impl Greet for German { fn greet(&self) -> String { "Hallo".to_string() } }` — no interface registration needed.

---

## §9 · Enums & Pattern Matching
**Module:** `s09_enums`

Go enums are just typed integers (`iota`). Rust enums are **algebraic data types** — each variant can carry different data:

```rust
enum Message {
    Quit,                       // no data
    Move { x: i32, y: i32 },   // named fields
    Write(String),              // single value
    Color(u8, u8, u8),          // tuple
}
```

`match` on an enum extracts the data from each variant. The compiler ensures every variant is handled.

### Tests
| Test | What it proves |
|---|---|
| `match_each_variant` | extract data from each variant type |
| `enum_method` | `impl` on an enum; `matches!` macro |
| `heterogeneous_vec_via_enum` | no interface needed — enum IS the type |

### Try this
Add a new `variant Color2(u8, u8, u8)` to `Message` and run `cargo test s09`. The `describe` function will fail to compile because `match` is no longer exhaustive — the compiler tells you exactly which variant is missing.

---

## §10 · Ownership — The Core Concept
**Module:** `s10_ownership`

The three rules:
1. Each value has exactly **one owner**
2. When the owner goes out of scope, the value is **dropped** (memory freed)
3. Ownership can be **moved** or **borrowed** — not both at once

```
let s1 = String::from("hello");
let s2 = s1;   // ownership MOVED — s1 is now invalid
```

**Copy types** (stack-only: `i32`, `bool`, `f64`, `char`, `&T`) are copied on assignment — the original stays valid.

To keep both alive for heap types, use `.clone()` (explicit deep copy).

### Tests
| Test | What it proves |
|---|---|
| `move_invalidates_original` | `let s2 = s1` moves — s1 unusable |
| `clone_keeps_both_alive` | `.clone()` makes an independent copy |
| `copy_types_are_not_moved` | `i32`, `bool` copy implicitly |
| `ownership_through_functions` | move into fn vs borrow into fn vs Copy |

### Try this
In `move_invalidates_original`, uncomment `// println!("{}", s1);` and run `cargo build`. The error message is Rust's clearest explanation of ownership: `value borrowed here after move`.

---

## §11 · Borrowing & References
**Module:** `s11_borrowing`

A reference lets you use a value without taking ownership — like passing a pointer in Go, but the compiler verifies safety.

```
&T      — shared borrow: many readers, no writers
&mut T  — exclusive borrow: one writer, no readers
```

**The rules (compile-time enforced):**
- Rule 1: any number of `&T` OR exactly one `&mut T` — never both simultaneously
- Rule 2: a reference can never outlive the data it points to (no dangling pointers)

**NLL (Non-Lexical Lifetimes):** a borrow ends at its last *use*, not at the end of the block. So you can take a `&mut` in the same block as a `&` as long as they don't overlap in usage.

### Tests
| Test | What it proves |
|---|---|
| `immutable_borrow_keeps_original_alive` | `&s` — s still alive after the call |
| `mutable_borrow_allows_modification` | `&mut s` — function can modify |
| `multiple_immutable_borrows_are_fine` | two `&s` at the same time — OK |
| `borrow_ends_at_last_use_nll` | `&` ends at last use; `&mut` safe after |
| `string_slice_borrow` | `&str` = a borrowed slice into a `String` |

### Try this
In `borrow_ends_at_last_use_nll`, move `let r2 = &mut s;` to *before* `let _ = r1.len();` and run `cargo build`. The error explains that `r1` (an immutable borrow) and `r2` (a mutable borrow) cannot coexist.

---

## §12 · Lifetimes
**Module:** `s12_lifetimes`

Lifetimes prevent use-after-free for references. The compiler infers them 95% of the time (lifetime elision). You only write explicit annotations when the compiler can't determine how long a returned reference must stay valid.

```rust
fn longest<'a>(a: &'a str, b: &'a str) -> &'a str {
    // 'a means: the output lives as long as the SHORTER of a and b
    if a.len() >= b.len() { a } else { b }
}
```

A struct that holds a reference must declare a lifetime — it can never outlive the data it points to.

**Beginner tip:** if lifetime annotations feel hard, just own the data (`String` instead of `&str`, `Vec` instead of `&[T]`). That's always correct and usually fast enough.

### Tests
| Test | What it proves |
|---|---|
| `longest_string` | explicit `'a` annotation on a function |
| `struct_with_lifetime` | `Excerpt<'a>` cannot outlive the source string |
| `static_lifetime_for_string_literals` | `'static` = lives for the whole program |

---

## §13 · Error Handling: Result & Option
**Module:** `s13_errors`

| Go | Rust |
|---|---|
| `*T` nullable pointer | `Option<T>` — `Some(v)` or `None` |
| `(T, error)` | `Result<T, E>` — `Ok(v)` or `Err(e)` |
| `if err != nil { return ..., err }` | `value?` — early return on Err |
| `if user == nil { ... }` | `match opt { Some(u) => ..., None => ... }` |

**The `?` operator** is Rust's version of Go's `if err != nil { return err }`. It works on both `Result` and `Option`, and it chains — each `?` propagates the error upward.

### Tests
| Test | What it proves |
|---|---|
| `option_some_and_none` | `Some` when found, `None` when not |
| `option_methods` | `unwrap`, `unwrap_or`, `map`, `is_some`, `is_none` |
| `result_ok_and_err` | `Ok` on success, `Err` on parse failure or range error |
| `result_methods` | `unwrap`, `unwrap_or`, `map`, `is_ok`, `is_err` |
| `question_mark_chains_errors` | `?` propagates error through two functions |

### Try this
Change `parse_age("25")` to `parse_age("25abc")` in `result_ok_and_err` and run `cargo test s13 -- --nocapture`. The `.parse()` will fail and `?` will return early with an `Err`.

---

## §14 · Collections: Vec, HashMap, HashSet
**Module:** `s14_collections`

| Go | Rust |
|---|---|
| `[]T` slice / `make([]T, 0, cap)` | `Vec<T>` |
| `map[K]V` | `HashMap<K, V>` |
| no built-in set | `HashSet<T>` |
| `append(s, x)` | `v.push(x)` |
| `if _, ok := m[k]; !ok { m[k] = 0 }` | `m.entry(k).or_insert(0)` |
| `v, ok := m[k]` | `m.get(&k)` → `Option<&V>` |

The **entry API** is especially useful for incrementing counts:
```rust
let count = m.entry(word).or_insert(0);  // &mut value, inserted as 0 if absent
*count += 1;                              // dereference to mutate
```

### Tests
| Test | What it proves |
|---|---|
| `vec_push_access_slice` | push, index, `.get()` (safe), slice `&v[1..3]` |
| `vec_sort_dedup_retain` | sort → dedup removes consecutive dupes → retain filters |
| `vec_iterate` | `.iter().sum()`, `&mut v` mutation loop |
| `hashmap_insert_get_remove` | insert, `m["key"]` (panics), `.get()` (safe), remove |
| `hashmap_entry_word_count` | entry API — idiomatic word-count pattern |
| `hashset_unique_and_operations` | union, intersection, difference |

---

## §15 · Strings: `&str` vs `String`
**Module:** `s15_strings`

This is the most common confusion for Go developers. Go has one string type. Rust has two.

| Type | What it is | Go equivalent |
|---|---|---|
| `&str` | borrowed slice — pointer + length into existing data, immutable | `string` (Go strings are always immutable references) |
| `String` | owned, heap-allocated, growable | `strings.Builder` / `[]byte` that became a string |

**Rule of thumb:**
- Function **parameters** → `&str` (accepts both literals and `&owned_string` via deref coercion)
- Struct **fields** / return values you own → `String`

Rust strings are **UTF-8**. `.len()` returns bytes, not characters. Use `.chars().count()` for the character count.

### Tests
| Test | What it proves |
|---|---|
| `literal_is_str` | `"hello"` has type `&str`, lives in the binary |
| `various_ways_to_make_string` | `String::from`, `.to_string()`, `format!` |
| `coerce_string_to_str` | `&String` auto-derefs to `&str` |
| `convert_between_types` | `&str` → `String` allocates; `&String` → `&str` is free |
| `string_mutation` | `.push()` char, `.push_str()` for `&str` |
| `common_string_operations` | trim, contains, to_lowercase, replace |
| `split_and_collect` | `split(',').collect::<Vec<&str>>()` |
| `byte_len_vs_char_count` | `"hello 🦀".len() == 10` vs `.chars().count() == 7` |

---

## §16 · Closures & Iterators
**Module:** `s16_closures_iterators`

**Closures** are anonymous functions that capture their environment:
```rust
let factor = 10;
let scale = |x: i32| x * factor;  // captures `factor` by reference

let s = String::from("hi");
let f = move || println!("{}", s); // MOVE s into closure — required for threads
```

**Iterator adapters** are lazy (nothing runs until `.collect()` / `.sum()` / etc.) and compile to the same code as a hand-written for loop — zero overhead.

```
.map(|x| ...)          transform each element
.filter(|x| ...)       keep matching elements
.filter_map(|x| ...)   filter + transform (keep Some values)
.fold(init, |acc, x|)  reduce to a single value
.flat_map(|x| ...)     map then flatten
.take(n) / .skip(n)    limit / offset
.enumerate()           pair with index
.zip(other)            pair elements from two iterators
.chain(other)          concatenate two iterators
.any() / .all()        short-circuit boolean check
.find()                first matching element
```

### Tests
| Test | What it proves |
|---|---|
| `basic_closure` | `\|a, b\| a + b` — types inferred |
| `closure_captures_by_reference` | captures `factor` from outer scope |
| `move_closure_owns_captured_data` | `move \|\|` — takes ownership |
| `map_transform_each_element` | double each number |
| `filter_keep_matching` | keep only even numbers |
| `filter_map_filter_and_transform` | parse only valid numbers from strings |
| `fold_is_manual_accumulation` | product of all elements |
| `any_all_find` | short-circuit checks |
| `chain_and_zip` | combine / pair two iterators |
| `take_skip_enumerate` | limit, offset, add index |
| `flat_map_and_flatten` | split each sentence into words |
| `chained_pipeline` | full pipeline: filter → map → collect |

### Try this
Rewrite `chained_pipeline` using a manual `for` loop in Go style. Then compare. The iterator version compiles to identical machine code — but it reads as a pipeline and the compiler can optimize each step.

---

## §17 · Generics
**Module:** `s17_generics`

Same idea as Go 1.18+ generics, but Rust uses **trait bounds** to describe requirements:

| Go | Rust |
|---|---|
| `func Max[T constraints.Ordered](a, b T) T` | `fn max<T: PartialOrd>(a: T, b: T) -> T` |
| `[T interface{ Stringer; Closer }]` | `<T: fmt::Display + io::Write>` |
| `type Stack[T any] struct { ... }` | `struct Stack<T> { items: Vec<T> }` |

The `where` clause is just a formatting preference — use it when bounds get long.

All generics are **monomorphized** at compile time: the compiler generates a separate version for each concrete type used. Zero runtime overhead.

### Tests
| Test | What it proves |
|---|---|
| `generic_max_works_for_multiple_types` | same fn works for `i32`, `f64`, `&str` |
| `generic_stack_with_integers` | `Stack<i32>` push/pop/peek |
| `generic_stack_with_strings` | same `Stack<T>` works for `String` |
| `where_clause` | `where T: Display, U: Debug` readability |
| `print_and_return_generic` | multiple trait bounds `T: Display + Clone` |

---

## §18 · Concurrency: Threads & Channels
**Module:** `s18_concurrency`

Rust threads are OS threads — heavier than Go goroutines. For lightweight concurrency, use Tokio's async tasks (§19).

| Go | Rust |
|---|---|
| `go func() { }()` | `thread::spawn(move \|\| { })` |
| `wg.Wait()` | `handle.join().unwrap()` |
| `ch <- x` | `tx.send(x).unwrap()` |
| `v := <-ch` | `rx.recv().unwrap()` |
| `sync.Mutex` | `Mutex<T>` — wraps the data it protects |
| `sync.Map` | `Arc<Mutex<HashMap<K,V>>>` |

**The Rust difference:** the borrow checker prevents data races at **compile time**. `Arc<Mutex<T>>` is the pattern for shared mutable state — `Arc` for shared ownership, `Mutex` for exclusive access.

`move` on thread closures is required because threads can outlive the current scope — all data must be owned by the thread.

### Tests
| Test | What it proves |
|---|---|
| `spawn_and_join` | spawn + join, thread returns a value |
| `move_data_into_thread` | `move \|\|` transfers ownership |
| `channel_producer_consumer` | mpsc channel, `rx.iter()` blocks until sender drops |
| `multiple_producers_single_consumer` | `tx.clone()` for two senders, one receiver |
| `arc_mutex_shared_counter` | 10 threads increment a shared counter |

### Try this
In `move_data_into_thread`, remove `move` from `move ||` and run `cargo build`. The error will say the closure may outlive the current function — this is the compiler preventing a dangling reference.

---

## §19 · Async / Await with Tokio
**Module:** `s19_async`

| Go | Rust |
|---|---|
| Goroutine (green thread, always runs) | `Future` (state machine, runs only when `.await`ed) |
| `go func() { }()` | `tokio::spawn(async { })` |
| `wg.Wait()` / `<-done` | `handle.await.unwrap()` |
| `errgroup` (concurrent, wait for all) | `tokio::join!(a(), b())` |
| `select { }` (first branch wins) | `tokio::select! { }` |
| `context.Context` cancellation | `tokio::select!` with a cancellation future |

**Key mental model:** `async fn` returns a `Future` object — it does absolutely nothing until someone calls `.await` on it. The Tokio runtime is the executor that polls futures and drives them to completion.

`#[tokio::test]` is the async equivalent of `#[test]` — it starts a Tokio runtime for the test.

### Tests
| Test | What it proves |
|---|---|
| `basic_await` | `async fn` + `.await` |
| `spawn_concurrent_task` | `tokio::spawn` = goroutine |
| `join_runs_both_concurrently` | `tokio::join!` — wait for both |
| `select_first_wins` | `tokio::select!` — first to finish wins |
| `async_error_propagation` | `?` works inside `async fn` |

---

## §21 · Common Patterns & Idioms
**Module:** `s21_patterns`

### Builder Pattern
Go uses functional options or large config structs. Rust APIs commonly use a builder:
```rust
let config = ConfigBuilder::new()
    .host("0.0.0.0")
    .port(443)
    .timeout(60)
    .build();
```
Each setter takes `self` by value and returns `Self` — enabling method chaining. The `build()` call consumes the builder and produces the final value.

### Newtype Pattern
Wrap a primitive to create a distinct type. Prevents mixing up `UserId(42)` and `OrderId(42)` — they're both `u64` underneath but the compiler treats them as different types.

### Derive Macros
`#[derive(...)]` auto-generates boilerplate:
```rust
#[derive(Debug, Clone, PartialEq, Default)]
struct Config { host: String, port: u16 }
// You now get: {:?}, .clone(), ==, Config::default() — for free
```

### Tests
| Test | What it proves |
|---|---|
| `builder_pattern` | chained setters, consuming `build()` |
| `builder_defaults` | unset fields use defaults |
| `newtype_prevents_type_mixups` | `UserId` ≠ `OrderId` at compile time |
| `derive_macros` | `Default`, `Clone`, `PartialEq`, `Debug` all from one line |
| `question_mark_error_chain` | `?` propagates through two functions |

---

## Quick Commands Reference

```bash
# Run all tests
cargo test

# Run one section
cargo test s03
cargo test s10
cargo test s16

# Run a single test
cargo test s11_borrowing::mutable_borrow_allows_modification

# See println! output (suppressed by default in tests)
cargo test s16 -- --nocapture

# List all test names without running
cargo test -- --list

# Check code without running tests (fast)
cargo check
```

## Learning Path (recommended order)

1. **§3** Variables — the `mut` / immutable split
2. **§5** Functions — implicit returns
3. **§6** Control flow — `match`, `if let`
4. **§7** Structs — `&self` / `&mut self` / `self`
5. **§10** Ownership — the key mental model
6. **§11** Borrowing — `&T` vs `&mut T`
7. **§13** Error handling — `Result`, `Option`, `?`
8. **§9** Enums — algebraic data types
9. **§16** Iterators — the functional pipeline style
10. **§8** Traits — after you understand ownership
11. **§18** Concurrency — threads, channels, `Arc<Mutex<T>>`
12. **§19** Async — Tokio, `join!`, `select!`
