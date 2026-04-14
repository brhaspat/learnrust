# Rust for Go Developers

A practical guide to learning Rust if you already know Go.
The biggest shift: Go trusts the GC to manage memory. Rust trusts *you* — but gives you a compiler that catches every mistake.

---

## Table of Contents

1. [The Mental Model Shift](#1-the-mental-model-shift)
2. [Toolchain: Cargo vs go tool](#2-toolchain-cargo-vs-go-tool)
3. [Variables & Mutability](#3-variables--mutability)
4. [Types & Type System](#4-types--type-system)
5. [Functions & Methods](#5-functions--methods)
6. [Control Flow](#6-control-flow)
7. [Structs & Impl](#7-structs--impl)
8. [Traits vs Interfaces](#8-traits-vs-interfaces)
9. [Enums & Pattern Matching](#9-enums--pattern-matching)
10. [Ownership — The Core Concept](#10-ownership--the-core-concept)
11. [Borrowing & References](#11-borrowing--references)
12. [Lifetimes](#12-lifetimes)
13. [Error Handling: Result & Option](#13-error-handling-result--option)
14. [Collections: Vec, HashMap, HashSet](#14-collections-vec-hashmap-hashset)
15. [Strings: &str vs String](#15-strings-str-vs-string)
16. [Closures & Iterators](#16-closures--iterators)
17. [Generics](#17-generics)
18. [Concurrency: Threads & Channels](#18-concurrency-threads--channels)
19. [Async / Await with Tokio](#19-async--await-with-tokio)
20. [Web Development: Axum](#20-web-development-axum)
21. [Common Patterns & Idioms](#21-common-patterns--idioms)
22. [Cheat Sheet: Go → Rust](#22-cheat-sheet-go--rust)

---

## 1. The Mental Model Shift

| Concept | Go | Rust |
|---|---|---|
| Memory | Garbage collected | Ownership system (compile-time) |
| Null | `nil` everywhere | No null — `Option<T>` |
| Errors | Return `error` values | `Result<T, E>` enforced by type system |
| Concurrency model | Goroutines (M:N scheduling) | OS threads + `async`/`await` |
| Zero values | All types have a zero value | No zero values — you must initialize |
| Interfaces | Implicit (duck typing) | Explicit `impl Trait for Type` |
| Mutability | Mutable by default | Immutable by default |
| Inheritance | Struct embedding | Trait composition |

### What Rust gives you that Go doesn't
- **No data races** — enforced by the borrow checker at compile time
- **No null pointer dereferences** — ever
- **Zero-cost abstractions** — iterators, closures, generics compile to the same code as hand-written loops
- **Predictable performance** — no GC pauses, no stop-the-world

### What Go gives you that Rust doesn't
- Faster compile times
- Simpler concurrency model (goroutines)
- Easier onboarding
- A smaller mental model

---

## 2. Toolchain: Cargo vs go tool

```
# Go                          # Rust equivalent
go mod init myapp             cargo new myapp
go build                      cargo build
go run main.go                cargo run
go test ./...                 cargo test
go get github.com/foo/bar     # add to Cargo.toml, then:
                              cargo add foo_bar
go vet ./...                  cargo clippy
gofmt -w .                    cargo fmt
go build -race                # borrow checker covers this at compile time
```

**Cargo.toml** is the equivalent of **go.mod + go.sum**:

```toml
[package]
name    = "proj1"
version = "0.1.0"
edition = "2021"

[dependencies]
axum    = "0.7"
tokio   = { version = "1", features = ["full"] }
serde   = { version = "1", features = ["derive"] }

[dev-dependencies]
# test-only deps go here
```

Key difference: Go's module proxy fetches source. Cargo resolves, downloads, and **compiles** all dependencies — so the first `cargo build` is slow. Subsequent builds use an incremental cache.

---

## 3. Variables & Mutability

```go
// Go
x := 10         // mutable
x = 20          // fine
const Y = 30    // immutable
```

```rust
// Rust
let x = 10;      // immutable — cannot reassign
let mut y = 10;  // mutable
y = 20;          // fine

const MAX: u32 = 30;          // compile-time constant, requires type
static ADDR: &str = "0.0.0.0"; // static lifetime, fixed memory address

// Shadowing — re-bind the same name with `let`
let x = x + 1;   // new x = 11, old x dropped
let x = x * 2;   // new x = 22
```

**Shadowing** lets you reuse a name with a different type, which is common in Rust:

```rust
let input = "42";        // &str
let input: i32 = input.parse().unwrap();  // i32 — same name, new binding
```

---

## 4. Types & Type System

### Primitive Types

| Category | Go | Rust |
|---|---|---|
| Signed integers | `int`, `int8/16/32/64` | `i8`, `i16`, `i32`, `i64`, `i128`, `isize` |
| Unsigned integers | `uint`, `uint8/16/32/64` | `u8`, `u16`, `u32`, `u64`, `u128`, `usize` |
| Floats | `float32`, `float64` | `f32`, `f64` |
| Boolean | `bool` | `bool` |
| String | `string` | `&str`, `String` (see section 15) |
| Byte | `byte` (alias uint8) | `u8` |
| Rune | `rune` (alias int32) | `char` (always 4 bytes, Unicode scalar) |

**`isize`/`usize`** are pointer-sized (like Go's `int`). Use `usize` for indexing and lengths.

### Type Conversion

Go allows implicit numeric widening in some cases. Rust **never** does implicit conversion — always explicit:

```rust
let x: i32 = 42;
let y: i64 = x as i64;   // explicit cast
let z: f64 = x as f64;

// Careful: `as` truncates, doesn't panic
let big: i32 = 300;
let small: u8 = big as u8;  // 44 (truncated, not an error)

// Safe conversion with error handling
let small: u8 = big.try_into().expect("value too large for u8");
```

### Tuples

Go doesn't have tuples natively (it uses multiple returns). Rust has first-class tuples:

```rust
let pair: (i32, &str) = (42, "hello");
let (num, text) = pair;       // destructure
println!("{}", pair.0);       // index access
println!("{}", pair.1);
```

---

## 5. Functions & Methods

```go
// Go
func add(a, b int) int {
    return a + b
}
func swap(a, b int) (int, int) {
    return b, a
}
```

```rust
// Rust
fn add(a: i32, b: i32) -> i32 {
    a + b  // last expression without `;` is the return value
}

// Use `return` for early returns
fn divide(a: f64, b: f64) -> f64 {
    if b == 0.0 {
        return f64::INFINITY;  // early return
    }
    a / b  // implicit return
}

// Multiple returns via tuple
fn swap(a: i32, b: i32) -> (i32, i32) {
    (b, a)
}

let (x, y) = swap(1, 2);
```

**No function overloading** in Rust (same as Go). Use different names or traits.

**Variadic functions** don't exist in Rust (unlike Go's `...args`). Use a slice parameter instead:

```rust
fn sum(nums: &[i32]) -> i32 {
    nums.iter().sum()
}

sum(&[1, 2, 3, 4, 5]);
```

---

## 6. Control Flow

### For Loops

```rust
// Range (exclusive)
for i in 0..5 { }        // 0,1,2,3,4

// Range (inclusive)
for i in 0..=5 { }       // 0,1,2,3,4,5

// Iterate a collection
for item in &vec { }     // borrow each element
for item in vec { }      // consume (move) each element
for item in &mut vec { } // mutably borrow each element

// With index (Go's `for i, v := range slice`)
for (i, item) in vec.iter().enumerate() { }

// Iterate map (Go's `for k, v := range m`)
for (key, val) in &map { }

// `loop` = infinite loop (Go's `for { }`)
loop {
    if condition { break; }
    if other { continue; }
}

// loop can return a value
let result = loop {
    counter += 1;
    if counter == 10 { break counter * 2; }
};

// while
while n < 100 { n *= 2; }
```

### If / Else

```rust
// if is an expression — use it like the ternary operator
let label = if x > 0 { "positive" } else { "non-positive" };
```

### Match

`match` is Go's `switch` — but exhaustive and much more powerful:

```rust
match value {
    0 => println!("zero"),
    1 | 2 => println!("one or two"),         // OR
    3..=9 => println!("three to nine"),      // range
    n if n < 0 => println!("negative: {}", n),  // guard
    _ => println!("other"),                  // wildcard (required if not exhaustive)
}

// Match on struct fields
match point {
    Point { x: 0, y } => println!("on y-axis at {}", y),
    Point { x, y: 0 } => println!("on x-axis at {}", x),
    Point { x, y } => println!("at ({}, {})", x, y),
}
```

### if let / while let

```rust
// Cleaner than a full match when you only care about one variant
if let Some(value) = map.get("key") {
    println!("{}", value);
}

// Loop while the pattern matches
while let Some(top) = stack.pop() {
    println!("{}", top);
}
```

---

## 7. Structs & Impl

```go
// Go
type Server struct {
    Host string
    Port int
}
func NewServer(host string, port int) *Server {
    return &Server{Host: host, Port: port}
}
func (s *Server) Address() string {
    return fmt.Sprintf("%s:%d", s.Host, s.Port)
}
```

```rust
// Rust
struct Server {
    host: String,  // lowercase field names are idiomatic
    port: u16,
}

impl Server {
    // Associated function (no `self`) — like a static method or constructor
    fn new(host: String, port: u16) -> Self {
        Server { host, port }  // field shorthand when variable names match
    }

    // Method — takes &self (immutable borrow)
    fn address(&self) -> String {
        format!("{}:{}", self.host, self.port)
    }

    // Mutable method — takes &mut self
    fn set_port(&mut self, port: u16) {
        self.port = port;
    }

    // Consuming method — takes self (moves ownership)
    fn into_address(self) -> String {
        format!("{}:{}", self.host, self.port)
    }  // `self` is dropped here
}

let mut s = Server::new("localhost".to_string(), 8080);
println!("{}", s.address());
s.set_port(9090);
```

### Struct Update Syntax

```rust
let s2 = Server {
    port: 443,
    ..s  // copy remaining fields from `s` (like Go's s2 := s; s2.Port = 443)
};
```

### Tuple Structs

```rust
struct Rgb(u8, u8, u8);

let red = Rgb(255, 0, 0);
println!("{}", red.0);  // access by index
```

### Debug & Display

```go
// Go: implements fmt.Stringer
func (s Server) String() string { return s.Address() }
fmt.Println(s)
```

```rust
// Rust: derive Debug for free (like %+v in Go)
#[derive(Debug)]
struct Server { host: String, port: u16 }
println!("{:?}", s);   // debug format
println!("{:#?}", s);  // pretty debug

// Implement Display for custom formatting (like Go's Stringer)
use std::fmt;
impl fmt::Display for Server {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}:{}", self.host, self.port)
    }
}
println!("{}", s);
```

---

## 8. Traits vs Interfaces

Go interfaces are **implicit** — if a type has the methods, it satisfies the interface.
Rust traits are **explicit** — you must write `impl Trait for Type`.

```go
// Go
type Stringer interface {
    String() string
}
// Any type with a String() method automatically satisfies Stringer
```

```rust
// Rust
trait Summary {
    fn summarize(&self) -> String;

    // Default implementation (Go interfaces can't do this)
    fn preview(&self) -> String {
        format!("{}...", &self.summarize()[..50])
    }
}

struct Article { title: String, body: String }

impl Summary for Article {
    fn summarize(&self) -> String {
        format!("{}: {}", self.title, self.body)
    }
}

// You can require multiple traits (like embedding interfaces in Go)
fn notify(item: &(impl Summary + fmt::Display)) { }

// Or with generic syntax (equivalent)
fn notify<T: Summary + fmt::Display>(item: &T) { }

// Dynamic dispatch (like Go interface variable) — uses a vtable
fn notify_dyn(item: &dyn Summary) { }
let items: Vec<Box<dyn Summary>> = vec![...];
```

### Common Standard Traits

| Trait | Go equivalent | Purpose |
|---|---|---|
| `fmt::Display` | `fmt.Stringer` | Human-readable formatting |
| `fmt::Debug` | `%+v` | Developer/debug formatting |
| `Clone` | — | Explicit deep copy |
| `Copy` | — | Implicit bitwise copy (primitives) |
| `Iterator` | — | for-loop compatibility |
| `From`/`Into` | — | Type conversion |
| `Default` | zero values | Provides a default value |
| `PartialEq`/`Eq` | `==` operator | Equality comparison |
| `PartialOrd`/`Ord` | `<`, `>` | Ordering |
| `Hash` | — | HashMap key compatibility |
| `Send`/`Sync` | — | Thread safety markers |

Most are derivable:

```rust
#[derive(Debug, Clone, PartialEq, Eq, Hash, Default)]
struct UserId(u64);
```

---

## 9. Enums & Pattern Matching

Go's enums are `iota` constants. Rust enums are **algebraic data types** — each variant can carry different data.

```go
// Go
type Direction int
const (
    North Direction = iota
    South
    East
    West
)
```

```rust
// Rust — simple enum
enum Direction { North, South, East, West }

// Enums with data (no Go equivalent — this is closer to a tagged union)
enum Message {
    Quit,                          // no data
    Move { x: i32, y: i32 },      // named fields (like a struct)
    Write(String),                 // single value
    Color(u8, u8, u8),             // tuple
}

// Pattern match extracts the data
fn process(msg: Message) {
    match msg {
        Message::Quit => println!("quit"),
        Message::Move { x, y } => println!("move to {},{}", x, y),
        Message::Write(text) => println!("write: {}", text),
        Message::Color(r, g, b) => println!("color: {},{},{}", r, g, b),
    }
}

// Enums can have methods too
impl Message {
    fn is_quit(&self) -> bool {
        matches!(self, Message::Quit)
    }
}
```

---

## 10. Ownership — The Core Concept

This is what makes Rust different from every other language. There is no GC and no manual `free()`.

**The three rules:**
1. Each value has exactly one **owner**
2. When the owner goes out of scope, the value is **dropped** (memory freed)
3. Ownership can be **moved** or **borrowed**, never both simultaneously

```rust
fn main() {
    // s1 owns the String
    let s1 = String::from("hello");

    // Ownership MOVES to s2 — s1 is now invalid
    let s2 = s1;
    // println!("{}", s1);  // compile error: value moved

    // To keep both, clone explicitly (deep copy)
    let s3 = String::from("world");
    let s4 = s3.clone();
    println!("{} {}", s3, s4);  // both valid
}
```

**Copy types** (stack-allocated primitives) are copied implicitly — no move:

```rust
let x: i32 = 5;
let y = x;          // copied, not moved
println!("{}", x);  // still valid
```

Types that implement `Copy`: all integers, floats, `bool`, `char`, tuples of `Copy` types, `&T` references.

**Ownership through functions:**

```rust
fn takes_ownership(s: String) {  // s moves in
    println!("{}", s);
}                                // s is dropped here

fn makes_copy(n: i32) {          // i32 is Copy, so n is copied in
    println!("{}", n);
}

fn gives_ownership() -> String {
    String::from("hello")        // ownership moves to caller
}

fn main() {
    let s = String::from("hello");
    takes_ownership(s);
    // println!("{}", s);  // error: s was moved into the function

    let x = 5;
    makes_copy(x);
    println!("{}", x);  // fine — x was copied
}
```

**Go comparison:** In Go, you decide between value and pointer semantics manually. In Rust, the compiler enforces that ownership rules are followed — you can't accidentally share mutable state.

---

## 11. Borrowing & References

Instead of transferring ownership, you can **borrow** a value with `&`. This is like passing a pointer in Go, but the compiler verifies safety.

```go
// Go — passing a pointer to avoid copying
func printLen(s *string) {
    fmt.Println(len(*s))
}
s := "hello"
printLen(&s)
```

```rust
// Rust — borrowing (immutable reference)
fn print_len(s: &String) {     // borrows, does not take ownership
    println!("{}", s.len());
}  // borrow ends here, s is not dropped

let s = String::from("hello");
print_len(&s);
println!("{}", s);  // s still valid — we only lent it
```

### Mutable References

```rust
fn push_world(s: &mut String) {
    s.push_str(", world");
}

let mut s = String::from("hello");
push_world(&mut s);
println!("{}", s);  // "hello, world"
```

### The Borrowing Rules (enforced at compile time)

**Rule 1:** You can have any number of immutable references `&T` OR exactly one mutable reference `&mut T` — never both at the same time.

```rust
let mut s = String::from("hello");

let r1 = &s;       // immutable borrow
let r2 = &s;       // another immutable borrow — fine
// let r3 = &mut s; // ERROR: cannot borrow as mutable while immutable borrows exist

println!("{} {}", r1, r2);  // r1 and r2 used here, borrows end after this line

let r3 = &mut s;   // now fine — previous borrows ended
println!("{}", r3);
```

**Rule 2:** References must always be valid (no dangling pointers):

```rust
// This won't compile — returning a reference to a local variable
fn dangle() -> &String {     // error
    let s = String::from("hello");
    &s  // s is dropped when function returns, reference would dangle
}

// Fix: return the owned value
fn no_dangle() -> String {
    String::from("hello")
}
```

---

## 12. Lifetimes

Lifetimes tell the compiler how long references must be valid. Most of the time they're **inferred** and you don't write them. You only need explicit lifetime annotations when the compiler can't figure it out.

```rust
// The compiler needs to know: does the return reference live as long as `a` or `b`?
// We say: the output lives at least as long as the shorter of `a` and `b`
fn longest<'a>(a: &'a str, b: &'a str) -> &'a str {
    if a.len() >= b.len() { a } else { b }
}

let s1 = String::from("long string");
let result;
{
    let s2 = String::from("xy");
    result = longest(s1.as_str(), s2.as_str());
    println!("{}", result);  // fine — both s1 and s2 alive here
}
// println!("{}", result);  // would be an error — s2 dropped above
```

**Lifetimes in structs** — when a struct holds a reference:

```rust
// This struct cannot outlive the string slice it holds
struct Excerpt<'a> {
    text: &'a str,
}

let novel = String::from("Call me Ishmael. Some years ago...");
let first_sentence = novel.split('.').next().unwrap();
let excerpt = Excerpt { text: first_sentence };
// excerpt can't outlive `novel`
```

**Tip for beginners:** If the compiler complains about lifetimes, the usual fix is to own the data (`String` instead of `&str`) rather than trying to annotate lifetimes until you understand them deeply.

---

## 13. Error Handling: Result & Option

### Option<T> — replaces nil checks

```go
// Go
func findUser(id int) *User {
    // returns nil if not found
}
user := findUser(42)
if user == nil { ... }
```

```rust
// Rust — None is explicit in the type, impossible to forget to check
fn find_user(id: u32) -> Option<User> {
    // returns None if not found
}

match find_user(42) {
    Some(user) => println!("{}", user.name),
    None => println!("not found"),
}

// Shorthand patterns
let user = find_user(42)?;           // return None from current fn if None
let user = find_user(42).unwrap();   // panic if None (use only in tests/examples)
let user = find_user(42).expect("user 42 must exist");  // panic with message
let name = find_user(42).map(|u| u.name);               // Option<String>
let name = find_user(42).unwrap_or_default();            // default value
let name = find_user(42).unwrap_or_else(|| User::guest()); // lazy default
```

### Result<T, E> — replaces (value, error) returns

```go
// Go
func readFile(path string) ([]byte, error) { ... }
data, err := readFile("foo.txt")
if err != nil {
    return nil, fmt.Errorf("reading file: %w", err)
}
```

```rust
// Rust
fn read_file(path: &str) -> Result<String, std::io::Error> { ... }

match read_file("foo.txt") {
    Ok(data) => process(data),
    Err(e) => eprintln!("error: {}", e),
}

// The `?` operator — equivalent to Go's `if err != nil { return ..., err }`
fn process_file(path: &str) -> Result<(), Box<dyn std::error::Error>> {
    let data = read_file(path)?;         // returns Err early if Err
    let parsed: Config = serde_json::from_str(&data)?;  // chains errors
    println!("{:?}", parsed);
    Ok(())
}
```

### Custom Errors

```rust
use thiserror::Error;  // popular crate

#[derive(Error, Debug)]
enum AppError {
    #[error("user {0} not found")]
    UserNotFound(u32),

    #[error("database error: {0}")]
    Database(#[from] sqlx::Error),  // automatic From conversion

    #[error("invalid input: {0}")]
    InvalidInput(String),
}

fn get_user(id: u32) -> Result<User, AppError> {
    if id == 0 {
        return Err(AppError::InvalidInput("id cannot be 0".to_string()));
    }
    // ...
}
```

### The `?` operator with conversions

The `?` operator also calls `From::from` on the error type, so you can use it across different error types as long as conversions are defined — similar to `%w` wrapping in Go.

---

## 14. Collections: Vec, HashMap, HashSet

### Vec<T> — Go's slice

```rust
// Create
let mut v: Vec<i32> = Vec::new();
let v = vec![1, 2, 3];            // macro shorthand
let v = vec![0; 100];             // 100 zeros

// Append / modify
v.push(4);
v.extend([5, 6, 7]);
v.insert(0, 99);   // insert at index
v.remove(0);       // remove by index

// Access
let first = &v[0];               // panics if OOB
let first = v.get(0);            // returns Option<&i32>

// Slice
let slice = &v[1..3];            // &[i32]

// Iterate
for x in &v { }                  // borrow
for x in &mut v { *x *= 2; }    // mutably borrow
for x in v { }                   // consume

// Common operations
v.len()
v.is_empty()
v.contains(&42)
v.sort()
v.sort_by(|a, b| a.cmp(b))
v.dedup()                        // remove consecutive duplicates
v.retain(|x| *x > 0)            // keep elements matching predicate
let sum: i32 = v.iter().sum();
```

### HashMap<K, V> — Go's map

```rust
use std::collections::HashMap;

// Create
let mut m: HashMap<String, i32> = HashMap::new();

// Insert
m.insert("alice".to_string(), 100);

// Read
let score = m["alice"];          // panics if missing
let score = m.get("alice");      // Option<&i32>

// Check existence (Go: _, ok := m[key])
if let Some(s) = m.get("alice") { println!("{}", s); }
if m.contains_key("alice") { }

// Insert only if absent (Go: if _, ok := m[k]; !ok { m[k] = v })
m.entry("bob".to_string()).or_insert(0);

// Update based on existing value
let count = m.entry("word".to_string()).or_insert(0);
*count += 1;

// Delete
m.remove("alice");

// Iterate
for (key, val) in &m { }

// Collect from iterator
let m: HashMap<&str, i32> = vec![("a", 1), ("b", 2)].into_iter().collect();
```

### HashSet<T>

```rust
use std::collections::HashSet;

let mut s: HashSet<i32> = HashSet::new();
s.insert(1);
s.insert(2);
s.contains(&1);     // true
s.remove(&1);

// Set operations
let a: HashSet<i32> = [1,2,3].iter().cloned().collect();
let b: HashSet<i32> = [2,3,4].iter().cloned().collect();

let union: HashSet<_> = a.union(&b).collect();
let intersection: HashSet<_> = a.intersection(&b).collect();
let difference: HashSet<_> = a.difference(&b).collect();
```

---

## 15. Strings: &str vs String

This is the most confusing part for Go developers. Go has one string type. Rust has two.

| Type | Description | Go equivalent |
|---|---|---|
| `&str` | Borrowed string slice — a pointer + length into some string data. Immutable. | `string` (it's always a reference in Go) |
| `String` | Owned, heap-allocated, growable string | `strings.Builder` or `[]byte` that's been stringified |

```rust
// &str — string literals are always &str
let s: &str = "hello";           // points into the binary, static lifetime

// String — heap-allocated
let s: String = String::from("hello");
let s: String = "hello".to_string();
let s: String = format!("{}:{}", host, port);  // like fmt.Sprintf

// Convert between them
let owned: String = s.to_string();     // &str -> String (allocates)
let borrowed: &str = &owned;           // String -> &str (free)
let borrowed: &str = owned.as_str();   // explicit

// Functions should accept &str when they don't need ownership
fn greet(name: &str) { println!("Hello, {}", name); }

greet("alice");                    // &str literal — fine
greet(&my_string);                 // String deref-coerces to &str
```

### String Operations

```rust
let mut s = String::from("hello");

s.push(' ');                       // append char
s.push_str("world");               // append &str
let s2 = s + " again";             // consumes s, allocates new String
let s3 = format!("{} {}", a, b);   // like Sprintf, doesn't consume

s.len()                            // byte length (not char count)
s.is_empty()
s.contains("ell")
s.starts_with("hel")
s.ends_with("rld")
s.replace("hello", "goodbye")
s.to_uppercase()
s.trim()                           // like strings.TrimSpace

// Split and collect
let words: Vec<&str> = s.split_whitespace().collect();
let parts: Vec<&str> = s.split(',').collect();

// Iterate chars (Go's `for _, r := range s`)
for ch in s.chars() { }
for byte in s.bytes() { }         // iterate bytes
```

---

## 16. Closures & Iterators

### Closures

```go
// Go
add := func(a, b int) int { return a + b }
```

```rust
// Rust — type inference in closures
let add = |a: i32, b: i32| a + b;
let add = |a, b| a + b;    // types inferred from usage

// Closures capture their environment
let factor = 10;
let scale = |x| x * factor;   // captures `factor` by reference

// Move ownership into the closure (required for threads)
let s = String::from("hello");
let print_s = move || println!("{}", s);  // s moved into closure
print_s();
```

### Iterator Adapters (Rust's killer feature)

These compile to the same machine code as hand-written loops — zero overhead.

```rust
let numbers = vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10];

// map — transform each element
let doubled: Vec<i32> = numbers.iter().map(|x| x * 2).collect();

// filter — keep matching elements
let evens: Vec<&i32> = numbers.iter().filter(|x| *x % 2 == 0).collect();

// filter_map — filter + transform in one step
let parsed: Vec<i32> = vec!["1", "two", "3"]
    .iter()
    .filter_map(|s| s.parse().ok())
    .collect();

// fold — like Go's manual accumulator loop (reduce)
let sum = numbers.iter().fold(0, |acc, x| acc + x);
let sum: i32 = numbers.iter().sum();   // shorthand

// any / all — short-circuit checks
let has_even = numbers.iter().any(|x| x % 2 == 0);
let all_pos = numbers.iter().all(|x| *x > 0);

// find — first matching element
let first_even = numbers.iter().find(|x| *x % 2 == 0);

// chain — combine iterators
let combined: Vec<i32> = [1,2,3].iter().chain([4,5,6].iter()).cloned().collect();

// zip — pair elements from two iterators
let pairs: Vec<_> = keys.iter().zip(values.iter()).collect();

// flat_map — map then flatten
let words: Vec<&str> = sentences.iter().flat_map(|s| s.split_whitespace()).collect();

// take / skip
let first_three: Vec<_> = numbers.iter().take(3).collect();
let after_two: Vec<_> = numbers.iter().skip(2).collect();

// Chain them
let result: Vec<String> = users
    .iter()
    .filter(|u| u.active)
    .map(|u| u.name.to_uppercase())
    .take(10)
    .collect();
```

---

## 17. Generics

```go
// Go 1.18+ generics
func Max[T constraints.Ordered](a, b T) T {
    if a > b { return a }
    return b
}
```

```rust
// Rust generics — trait bounds specify requirements
fn max<T: PartialOrd>(a: T, b: T) -> T {
    if a > b { a } else { b }
}

// Multiple bounds with `+`
fn print_and_return<T: fmt::Display + Clone>(val: T) -> T {
    println!("{}", val);
    val.clone()
}

// `where` clause — cleaner for complex bounds
fn complex<T, U>(t: T, u: U) -> String
where
    T: fmt::Display + Clone,
    U: fmt::Debug,
{
    format!("{} {:?}", t, u)
}

// Generic struct
struct Stack<T> {
    items: Vec<T>,
}

impl<T> Stack<T> {
    fn new() -> Self { Stack { items: Vec::new() } }
    fn push(&mut self, item: T) { self.items.push(item); }
    fn pop(&mut self) -> Option<T> { self.items.pop() }
    fn is_empty(&self) -> bool { self.items.is_empty() }
}

let mut s: Stack<i32> = Stack::new();
s.push(1);
```

---

## 18. Concurrency: Threads & Channels

### Threads

```go
// Go goroutine
go func() {
    fmt.Println("hello from goroutine")
}()
```

```rust
use std::thread;

// Rust thread — heavier than a goroutine (OS thread)
let handle = thread::spawn(|| {
    println!("hello from thread");
});
handle.join().unwrap();  // wait for completion — like WaitGroup.Wait()

// Pass data — must `move` to transfer ownership
let data = vec![1, 2, 3];
let handle = thread::spawn(move || {
    println!("{:?}", data);  // data moved into thread
});
handle.join().unwrap();
```

### Channels

```go
// Go
ch := make(chan int, 10)
go func() { ch <- 42 }()
val := <-ch
```

```rust
use std::sync::mpsc;

// mpsc = multi-producer, single-consumer
let (tx, rx) = mpsc::channel();

let tx2 = tx.clone();  // clone sender for multiple producers

thread::spawn(move || { tx.send(1).unwrap(); });
thread::spawn(move || { tx2.send(2).unwrap(); });

// Receive in a loop (blocks until all senders dropped)
for val in rx {
    println!("{}", val);
}

// Buffered channel
let (tx, rx) = mpsc::sync_channel(10);  // buffer size 10
```

### Shared State: Arc + Mutex

```go
// Go
var mu sync.Mutex
var counter int
mu.Lock()
counter++
mu.Unlock()
```

```rust
use std::sync::{Arc, Mutex};

// Arc = atomic reference counting (like Go's implicit sharing)
// Mutex = mutual exclusion guard
let counter = Arc::new(Mutex::new(0));
let mut handles = vec![];

for _ in 0..10 {
    let c = Arc::clone(&counter);
    handles.push(thread::spawn(move || {
        let mut num = c.lock().unwrap();  // blocks until lock acquired
        *num += 1;
    }));  // lock released when `num` drops
}

for h in handles { h.join().unwrap(); }
println!("{}", *counter.lock().unwrap());  // 10
```

### Send + Sync Markers

- `Send`: safe to transfer ownership across threads (most types)
- `Sync`: safe to share references across threads (`Arc<T>` where `T: Sync`)
- `Mutex<T>` is `Send + Sync` even if `T` is not `Sync`
- `Rc<T>` is neither `Send` nor `Sync` — use `Arc<T>` for multi-threading

The compiler enforces these — you cannot accidentally share a non-thread-safe type across threads.

---

## 19. Async / Await with Tokio

Go's goroutines are M:N green threads managed by the runtime. Rust's async is a state machine that requires an **executor** (runtime) to drive it. The most popular runtime is **Tokio**.

```go
// Go — goroutine
go func() {
    resp, err := http.Get("http://example.com")
}()
```

```rust
// Rust async/await
use tokio;

#[tokio::main]
async fn main() {
    let result = fetch_url("http://example.com").await;
}

async fn fetch_url(url: &str) -> Result<String, reqwest::Error> {
    let body = reqwest::get(url).await?.text().await?;
    Ok(body)
}
```

### Spawning Async Tasks (like goroutines)

```rust
// tokio::spawn is the goroutine equivalent
let handle = tokio::spawn(async {
    // runs concurrently
    expensive_computation().await
});

let result = handle.await.unwrap();

// Run multiple tasks concurrently (like errgroup in Go)
let (r1, r2) = tokio::join!(task_one(), task_two());

// Race — return first to finish (like select in Go)
tokio::select! {
    result = task_one() => println!("task one: {:?}", result),
    result = task_two() => println!("task two: {:?}", result),
}
```

### Async in Traits

Async functions in traits require the `async-trait` crate until Rust stabilizes this natively:

```rust
use async_trait::async_trait;

#[async_trait]
trait Fetcher {
    async fn fetch(&self, url: &str) -> Result<String, reqwest::Error>;
}
```

### Key async rules
- `async fn` returns a `Future` — it doesn't execute until `.await`ed
- Futures are lazy — nothing happens until polled by an executor
- You can only `.await` inside an `async fn` or `async` block
- `tokio::spawn` requires `Send` — the future must be safe to move across threads

---

## 20. Web Development: Axum

You're already using Axum in `proj1/src/main.rs`. Here's how it maps to Go concepts.

```go
// Go (net/http or gin)
http.HandleFunc("/users", getUsers)
http.ListenAndServe(":8080", nil)
```

```rust
// Rust (axum)
use axum::{routing::get, Router};
use tokio::net::TcpListener;

#[tokio::main]
async fn main() {
    let app = Router::new()
        .route("/users", get(get_users));

    let listener = TcpListener::bind("0.0.0.0:3000").await.unwrap();
    axum::serve(listener, app).await.unwrap();
}
```

### Extractors (like Gin's binding)

```rust
use axum::{
    extract::{Path, Query, State, Json},
    http::StatusCode,
};
use serde::{Deserialize, Serialize};

#[derive(Deserialize)]
struct Params { page: u32, limit: u32 }

#[derive(Serialize)]
struct User { id: u32, name: String }

// Path parameter
async fn get_user(Path(id): Path<u32>) -> Json<User> {
    Json(User { id, name: "alice".to_string() })
}

// Query parameters (?page=1&limit=10)
async fn list_users(Query(params): Query<Params>) -> Json<Vec<User>> {
    Json(vec![])
}

// JSON body
async fn create_user(Json(body): Json<User>) -> (StatusCode, Json<User>) {
    (StatusCode::CREATED, Json(body))
}

// Shared state (dependency injection)
#[derive(Clone)]
struct AppState { db: Arc<Database> }

async fn handler_with_state(State(state): State<AppState>) -> String {
    state.db.query().await
}
```

### Middleware

```rust
use tower::ServiceBuilder;
use tower_http::{trace::TraceLayer, cors::CorsLayer};

let app = Router::new()
    .route("/", get(handler))
    .layer(
        ServiceBuilder::new()
            .layer(TraceLayer::new_for_http())
            .layer(CorsLayer::permissive())
    );
```

### Cargo.toml for a typical Axum API

```toml
[dependencies]
axum       = "0.7"
tokio      = { version = "1", features = ["full"] }
serde      = { version = "1", features = ["derive"] }
serde_json = "1"
tower      = "0.4"
tower-http = { version = "0.5", features = ["trace", "cors"] }
tracing    = "0.1"
tracing-subscriber = "0.3"
thiserror  = "1"
```

---

## 21. Common Patterns & Idioms

### The Builder Pattern

Common in Rust where Go would use functional options or config structs:

```rust
struct ServerConfig {
    host: String,
    port: u16,
    timeout_secs: u64,
}

struct ServerConfigBuilder {
    host: String,
    port: u16,
    timeout_secs: u64,
}

impl ServerConfigBuilder {
    fn new() -> Self {
        Self { host: "localhost".into(), port: 8080, timeout_secs: 30 }
    }
    fn host(mut self, h: &str) -> Self { self.host = h.into(); self }
    fn port(mut self, p: u16) -> Self { self.port = p; self }
    fn timeout(mut self, t: u64) -> Self { self.timeout_secs = t; self }
    fn build(self) -> ServerConfig {
        ServerConfig { host: self.host, port: self.port, timeout_secs: self.timeout_secs }
    }
}

let config = ServerConfigBuilder::new()
    .host("0.0.0.0")
    .port(443)
    .timeout(60)
    .build();
```

### Newtype Pattern

Wrap a primitive to get type safety:

```rust
struct UserId(u64);
struct OrderId(u64);

// Now you can't accidentally pass an OrderId where a UserId is expected
fn get_user(id: UserId) -> Option<User> { ... }
```

### ? in main

```rust
fn main() -> Result<(), Box<dyn std::error::Error>> {
    let data = std::fs::read_to_string("config.json")?;
    let config: Config = serde_json::from_str(&data)?;
    Ok(())
}
```

### Logging

```rust
use tracing::{info, warn, error, debug};

// setup (usually in main)
tracing_subscriber::fmt::init();

// usage
info!("server starting on port {}", port);
warn!("retrying after error: {}", e);
error!("fatal: {}", e);
debug!("request: {:?}", req);
```

### derive macros for free functionality

```rust
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize, Default)]
struct Config {
    host: String,
    port: u16,
    debug: bool,
}

// Now you get:
// {:?} formatting, .clone(), ==, HashMap key, JSON ser/de, Config::default()
```

---

## 22. Cheat Sheet: Go → Rust

```
Go                              Rust
─────────────────────────────────────────────────────────
var x int = 5                   let x: i32 = 5;
x := 5                          let x = 5;
var x int                       // no zero values — must initialize
x = 5 (mutable)                 let mut x = 5; x = 10;

const X = 5                     const X: i32 = 5;

func f(a, b int) int {}         fn f(a: i32, b: i32) -> i32 {}
func f() (int, error) {}        fn f() -> Result<i32, Error> {}
return x, nil                   Ok(x)
return 0, err                   Err(e)
if err != nil { return ..., err } val?

fmt.Println(x)                  println!("{}", x);
fmt.Sprintf("x=%d", x)         format!("x={}", x)
fmt.Errorf("msg: %w", err)     format!("msg: {}", err)  // or thiserror

type S struct { X int }         struct S { x: i32 }
s := S{X: 1}                    let s = S { x: 1 };
s.X                             s.x
func (s *S) M() {}              impl S { fn m(&mut self) {} }

type I interface { M() }        trait I { fn m(&self); }
                                impl I for S { fn m(&self) {} }

s, ok := x.(Type)               if let Some(v) = x.downcast_ref::<Type>(){}

go func() { }()                 tokio::spawn(async { });
ch := make(chan int)             let (tx, rx) = mpsc::channel();
ch <- x                         tx.send(x).unwrap();
x := <-ch                       let x = rx.recv().unwrap();
select { case v := <-ch: }     tokio::select! { v = rx.recv() => {} }

sync.Mutex                      std::sync::Mutex<T>
sync.WaitGroup                  Vec<JoinHandle> + join()
sync.Map                        Arc<Mutex<HashMap<K,V>>>
atomic.AddInt64                 std::sync::atomic::AtomicI64

make([]int, 0, cap)             Vec::with_capacity(cap)
append(s, x)                    v.push(x)
s[i:j]                          &v[i..j]
len(s)                          v.len()
copy(dst, src)                  dst.copy_from_slice(&src)

make(map[K]V)                   HashMap::new()
m[k] = v                        m.insert(k, v);
v, ok := m[k]                   m.get(&k)  -> Option<&V>
delete(m, k)                    m.remove(&k);

nil                             None
*T (nullable pointer)           Option<T>
*T (non-null pointer)           &T or Box<T>
[]byte                          Vec<u8> or &[u8]
string                          &str or String
error                           Result<T, E> or Box<dyn Error>

for i := 0; i < n; i++ {}      for i in 0..n {}
for i, v := range slice {}      for (i, v) in slice.iter().enumerate() {}
for k, v := range m {}          for (k, v) in &m {}
for { }                         loop { }

defer f()                       // RAII: implement Drop, or use scopeguard crate
panic("msg")                    panic!("msg")
recover()                       std::panic::catch_unwind(...)
```

---

## Next Steps

1. **Read** [The Rust Book](https://doc.rust-lang.org/book/) — free online, the best resource
2. **Practice** with [Rustlings](https://github.com/rust-lang/rustlings) — small exercises
3. **Browse** [Rust by Example](https://doc.rust-lang.org/rust-by-example/) — annotated code
4. **Explore** your existing `proj1` — it already uses `axum`, `tokio`, `serde`, `Arc<Mutex<T>>`
5. **Key crates to know:**
   - `serde` / `serde_json` — JSON (equivalent to `encoding/json`)
   - `tokio` — async runtime (equivalent to Go's runtime scheduler)
   - `axum` / `actix-web` — HTTP (equivalent to net/http + gin)
   - `sqlx` / `diesel` — database (equivalent to `database/sql`)
   - `reqwest` — HTTP client (equivalent to `net/http` client)
   - `thiserror` / `anyhow` — error handling ergonomics
   - `tracing` — structured logging (equivalent to `log/slog`)
   - `clap` — CLI args (equivalent to `flag`)
