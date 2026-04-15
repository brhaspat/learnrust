// practice.rs — runnable tests for every section of rust_beginner.md
//
// Run ALL tests:        cargo test
// Run one section:      cargo test s03
// See println output:   cargo test -- --nocapture
// List test names:      cargo test -- --list

// ─────────────────────────────────────────────────────────────────────────────
// §3 · Variables & Mutability
// ─────────────────────────────────────────────────────────────────────────────
//
// Go:   all vars are mutable by default.
// Rust: vars are IMMUTABLE by default. You opt into mutability with `mut`.
//
// Key concepts:
//   • `let x = 5`      — immutable, can never be reassigned
//   • `let mut x = 5`  — mutable, can be reassigned
//   • Shadowing (`let x = x + 1`) — creates a NEW binding with the same name.
//     Unlike `mut`, shadowing can change the TYPE.
//   • `const` — compile-time constant, type annotation required
//   • `static` — fixed memory address, lives for the whole program
#[cfg(test)]
mod s03_variables {
    #[test]
    fn immutable_vs_mutable() {
        let x = 10;     // immutable — uncommenting `x = 20` would fail to compile
        let mut y = 10; // mutable
        y = 20;
        assert_eq!(x, 10);
        assert_eq!(y, 20);
    }

    #[test]
    fn shadowing_same_type() {
        let x = 5;
        let x = x + 1;  // new binding — old x is dropped
        let x = x * 2;  // another new binding
        assert_eq!(x, 12);
    }

    #[test]
    fn shadowing_changes_type() {
        // `mut` cannot change the type of a variable.
        // Shadowing can — common when converting user input.
        let input = "42";              // type: &str
        let input: i32 = input.parse().unwrap(); // type: i32, same name
        assert_eq!(input, 42);
    }

    #[test]
    fn const_and_static() {
        const MAX_SCORE: u32 = 100_000; // underscores allowed for readability
        static APP_NAME: &str = "proj1";
        assert_eq!(MAX_SCORE, 100_000);
        assert_eq!(APP_NAME, "proj1");
    }
}

// ─────────────────────────────────────────────────────────────────────────────
// §4 · Types & Type System
// ─────────────────────────────────────────────────────────────────────────────
//
// Rust NEVER does implicit numeric conversion — you always cast explicitly.
//   • `as`          — truncates on overflow, never panics (be careful!)
//   • `.try_into()` — returns Result, safe
//
// `usize` is the pointer-sized integer — always use it for indices/lengths.
// Tuples are first-class; Go doesn't have them (uses multiple returns instead).
#[cfg(test)]
mod s04_types {
    use std::convert::TryInto;

    #[test]
    fn explicit_numeric_cast() {
        let x: i32 = 42;
        let y: i64 = x as i64; // widen
        let z: f64 = x as f64; // int to float
        assert_eq!(y, 42_i64);
        assert_eq!(z, 42.0_f64);
    }

    #[test]
    fn as_truncates_silently() {
        // `as` wraps around — it does NOT panic.
        // 300 in binary is 0b1_0010_1100; keeping only 8 bits = 0b0010_1100 = 44
        let big: i32 = 300;
        let small: u8 = big as u8;
        assert_eq!(small, 44);
    }

    #[test]
    fn try_into_is_safe() {
        let big: i32 = 300;
        let result: Result<u8, _> = big.try_into();
        assert!(result.is_err(), "300 doesn't fit in u8");

        let small: i32 = 200;
        let result: Result<u8, _> = small.try_into();
        assert_eq!(result.unwrap(), 200);
    }

    #[test]
    fn tuples() {
        let pair: (i32, &str) = (42, "hello");
        let (num, text) = pair; // destructure
        assert_eq!(num, 42);
        assert_eq!(text, "hello");
        assert_eq!(pair.0, 42); // index access
        assert_eq!(pair.1, "hello");
    }

    #[test]
    fn usize_for_indexing() {
        let v = vec![10, 20, 30];
        let i: usize = 1; // index must be usize (pointer-sized)
        assert_eq!(v[i], 20);
        assert_eq!(v.len(), 3); // len() returns usize
    }
}

// ─────────────────────────────────────────────────────────────────────────────
// §5 · Functions & Methods
// ─────────────────────────────────────────────────────────────────────────────
//
// The last expression in a block (no `;`) is the implicit return value.
// Use `return` only for early exits.
//
// No function overloading in Rust (same as Go) — use different names or traits.
// No variadic args — use a slice `&[T]` parameter instead.
#[cfg(test)]
mod s05_functions {
    fn add(a: i32, b: i32) -> i32 {
        a + b // no semicolon = implicit return
    }

    fn divide(a: f64, b: f64) -> f64 {
        if b == 0.0 {
            return f64::INFINITY; // early return — uses `return` keyword
        }
        a / b // implicit return for the normal path
    }

    fn swap(a: i32, b: i32) -> (i32, i32) {
        (b, a) // tuple as a poor-man's multiple return (like Go)
    }

    // Rust has no variadic args — accept a slice instead
    fn sum(nums: &[i32]) -> i32 {
        nums.iter().sum()
    }

    #[test]
    fn test_add() {
        assert_eq!(add(2, 3), 5);
    }

    #[test]
    fn test_divide() {
        assert_eq!(divide(10.0, 2.0), 5.0);
        assert_eq!(divide(1.0, 0.0), f64::INFINITY);
    }

    #[test]
    fn test_swap() {
        let (x, y) = swap(1, 2);
        assert_eq!(x, 2);
        assert_eq!(y, 1);
    }

    #[test]
    fn test_sum_slice() {
        assert_eq!(sum(&[1, 2, 3, 4, 5]), 15);
        assert_eq!(sum(&[]), 0);
        // You can also pass a Vec by taking a slice of it:
        let v = vec![10, 20];
        assert_eq!(sum(&v), 30);
    }
}

// ─────────────────────────────────────────────────────────────────────────────
// §6 · Control Flow
// ─────────────────────────────────────────────────────────────────────────────
//
// `if` is an EXPRESSION — assign its result to a variable (replaces ternary).
// `match` is exhaustive — the compiler errors if you miss a case.
// `loop { break value }` — a loop that returns a value.
// `if let` / `while let` — pattern match without a full `match` block.
#[cfg(test)]
mod s06_control_flow {
    #[test]
    fn if_as_expression() {
        let x = 7;
        // Rust has no ternary operator — use `if` as an expression instead
        let label = if x > 0 { "positive" } else { "non-positive" };
        assert_eq!(label, "positive");
    }

    #[test]
    fn for_exclusive_range() {
        let mut sum = 0;
        for i in 0..5 { // 0,1,2,3,4 (exclusive upper bound)
            sum += i;
        }
        assert_eq!(sum, 10);
    }

    #[test]
    fn for_inclusive_range() {
        let mut sum = 0;
        for i in 1..=5 { // 1,2,3,4,5 (inclusive upper bound)
            sum += i;
        }
        assert_eq!(sum, 15);
    }

    #[test]
    fn for_enumerate() {
        // Go: for i, v := range slice {}
        let words = vec!["a", "b", "c"];
        let mut result = vec![];
        for (i, w) in words.iter().enumerate() {
            result.push(format!("{}:{}", i, w));
        }
        assert_eq!(result, vec!["0:a", "1:b", "2:c"]);
    }

    #[test]
    fn loop_returns_value() {
        let mut counter = 0;
        let result = loop {
            counter += 1;
            if counter == 10 {
                break counter * 2; // loop evaluates to this value
            }
        };
        assert_eq!(result, 20);
    }

    #[test]
    fn match_is_exhaustive() {
        let value = 5;
        let msg = match value {
            0 => "zero",
            1 | 2 => "one or two",    // OR pattern
            3..=9 => "three to nine", // inclusive range pattern
            n if n < 0 => "negative", // match guard (condition)
            _ => "ten or more",       // wildcard — required if not all cases covered
        };
        assert_eq!(msg, "three to nine");
    }

    #[test]
    fn if_let_shorter_than_match() {
        // Use `if let` when you only care about ONE variant
        let maybe: Option<i32> = Some(42);
        if let Some(value) = maybe {
            assert_eq!(value, 42);
        }
        // Equivalent full match:
        // match maybe { Some(v) => assert_eq!(v, 42), None => {} }
    }

    #[test]
    fn while_let_pops_until_empty() {
        let mut stack = vec![1, 2, 3];
        let mut popped = vec![];
        while let Some(top) = stack.pop() { // loop ends when pop() returns None
            popped.push(top);
        }
        assert_eq!(popped, vec![3, 2, 1]);
        assert!(stack.is_empty());
    }
}

// ─────────────────────────────────────────────────────────────────────────────
// §7 · Structs & Impl
// ─────────────────────────────────────────────────────────────────────────────
//
// `impl` blocks attach methods and associated functions to a type.
//   • `fn new(...) -> Self`   — associated function (no `self`), acts as constructor
//   • `fn foo(&self)`         — immutable method (read-only access)
//   • `fn foo(&mut self)`     — mutable method (can modify the struct)
//   • `fn foo(self)`          — consuming method (takes ownership, struct dropped after)
//
// Struct update syntax: `Server { port: 443, ..s }` copies remaining fields from `s`.
// `#[derive(Debug)]` gives you free `{:?}` formatting (like Go's `%+v`).
#[cfg(test)]
mod s07_structs {
    #[derive(Debug, PartialEq, Clone)]
    struct Server {
        host: String,
        port: u16,
    }

    impl Server {
        // Associated function — no `self`, called as Server::new(...)
        fn new(host: &str, port: u16) -> Self {
            Server { host: host.to_string(), port }
        }

        // Immutable method — reads but never changes the struct
        fn address(&self) -> String {
            format!("{}:{}", self.host, self.port)
        }

        // Mutable method — can modify fields
        fn set_port(&mut self, port: u16) {
            self.port = port;
        }

        // Consuming method — takes ownership, caller can't use `self` afterward
        fn into_address(self) -> String {
            format!("{}:{}", self.host, self.port)
            // `self` is dropped here
        }
    }

    #[test]
    fn constructor_and_method() {
        let s = Server::new("localhost", 8080);
        assert_eq!(s.address(), "localhost:8080");
    }

    #[test]
    fn mutable_method() {
        let mut s = Server::new("localhost", 8080);
        s.set_port(9090);
        assert_eq!(s.port, 9090);
    }

    #[test]
    fn consuming_method_moves_struct() {
        let s = Server::new("localhost", 443);
        let addr = s.into_address(); // s is MOVED — using s after this is a compile error
        assert_eq!(addr, "localhost:443");
    }

    #[test]
    fn struct_update_syntax() {
        let s1 = Server::new("localhost", 8080);
        let s2 = Server {
            port: 443,
            ..s1 // copy remaining fields from s1 (s1.host is moved here)
        };
        assert_eq!(s2.host, "localhost");
        assert_eq!(s2.port, 443);
    }

    #[test]
    fn tuple_struct() {
        struct Rgb(u8, u8, u8); // fields accessed by index: .0, .1, .2
        let red = Rgb(255, 0, 0);
        assert_eq!(red.0, 255);
        assert_eq!(red.1, 0);
    }

    #[test]
    fn debug_derive() {
        let s = Server::new("localhost", 8080);
        let debug = format!("{:?}", s);  // works because of #[derive(Debug)]
        assert!(debug.contains("localhost"));
        assert!(debug.contains("8080"));
    }
}

// ─────────────────────────────────────────────────────────────────────────────
// §8 · Traits vs Interfaces
// ─────────────────────────────────────────────────────────────────────────────
//
// Go: implicit — if a type has the right methods it satisfies the interface.
// Rust: explicit — you write `impl Trait for Type`.
//
// Traits CAN have default method implementations (Go interfaces cannot).
// Static dispatch (`impl Trait` / generics) = zero-cost, monomorphized.
// Dynamic dispatch (`dyn Trait`) = runtime vtable, like a Go interface variable.
#[cfg(test)]
mod s08_traits {
    use std::fmt;

    trait Greet {
        fn greet(&self) -> String; // required — every implementor must define this

        // Default implementation — implementors can override or keep it
        fn greet_loud(&self) -> String {
            self.greet().to_uppercase()
        }
    }

    struct English;
    struct Spanish;

    impl Greet for English {
        fn greet(&self) -> String { "Hello".to_string() }
        // uses default greet_loud: "HELLO"
    }

    impl Greet for Spanish {
        fn greet(&self) -> String { "Hola".to_string() }
        // override the default
        fn greet_loud(&self) -> String {
            format!("¡{}!", self.greet().to_uppercase())
        }
    }

    // Static dispatch — compiler generates a separate version for English and Spanish.
    // Zero runtime overhead.
    fn say(g: &impl Greet) -> String { g.greet() }

    // Dynamic dispatch — single version, uses vtable at runtime.
    // Equivalent to a Go interface variable.
    fn say_dyn(g: &dyn Greet) -> String { g.greet() }

    #[test]
    fn static_dispatch() {
        assert_eq!(say(&English), "Hello");
        assert_eq!(say(&Spanish), "Hola");
    }

    #[test]
    fn dynamic_dispatch_heterogeneous_vec() {
        // Vec<Box<dyn Greet>> can hold different types — just like []Stringer in Go
        let greeters: Vec<Box<dyn Greet>> = vec![Box::new(English), Box::new(Spanish)];
        let greetings: Vec<String> = greeters.iter().map(|g| g.greet()).collect();
        assert_eq!(greetings, vec!["Hello", "Hola"]);
    }

    #[test]
    fn default_vs_overridden_method() {
        assert_eq!(English.greet_loud(), "HELLO");  // default used
        assert_eq!(Spanish.greet_loud(), "¡HOLA!"); // override used
    }

    #[test]
    fn multiple_trait_bounds() {
        // Require BOTH Display and Greet — like `interface { Stringer; Greeter }` in Go
        fn describe<T: Greet + fmt::Debug>(g: &T) -> String { g.greet() }

        #[derive(Debug)]
        struct French;
        impl Greet for French { fn greet(&self) -> String { "Bonjour".to_string() } }

        assert_eq!(describe(&French), "Bonjour");
    }
}

// ─────────────────────────────────────────────────────────────────────────────
// §9 · Enums & Pattern Matching
// ─────────────────────────────────────────────────────────────────────────────
//
// Go enums = typed ints (iota). Rust enums = algebraic data types.
// Each variant can carry DIFFERENT data — like a type-safe tagged union.
// `match` on an enum is exhaustive; the compiler errors if you miss a variant.
#[cfg(test)]
mod s09_enums {
    #[derive(Debug, PartialEq)]
    enum Message {
        Quit,                       // no data
        Move { x: i32, y: i32 },   // named fields (like a struct variant)
        Write(String),              // single value
        Color(u8, u8, u8),          // tuple variant
    }

    // Extract data from each variant via pattern matching
    fn describe(msg: &Message) -> String {
        match msg {
            Message::Quit => "quit".to_string(),
            Message::Move { x, y } => format!("move to {},{}", x, y),
            Message::Write(text) => format!("write: {}", text),
            Message::Color(r, g, b) => format!("color: {},{},{}", r, g, b),
        }
    }

    impl Message {
        fn is_quit(&self) -> bool {
            matches!(self, Message::Quit) // `matches!` is a handy macro for single-variant checks
        }
    }

    #[test]
    fn match_each_variant() {
        assert_eq!(describe(&Message::Quit), "quit");
        assert_eq!(describe(&Message::Move { x: 3, y: 4 }), "move to 3,4");
        assert_eq!(describe(&Message::Write("hi".to_string())), "write: hi");
        assert_eq!(describe(&Message::Color(255, 0, 0)), "color: 255,0,0");
    }

    #[test]
    fn enum_method() {
        assert!(Message::Quit.is_quit());
        assert!(!Message::Write("x".to_string()).is_quit());
    }

    #[test]
    fn heterogeneous_vec_via_enum() {
        // Unlike Go, you don't need an interface — the enum IS the type
        let msgs = vec![
            Message::Move { x: 1, y: 2 },
            Message::Color(0, 0, 0),
            Message::Quit,
        ];
        assert_eq!(msgs.len(), 3);
        assert_eq!(describe(&msgs[2]), "quit");
    }
}

// ─────────────────────────────────────────────────────────────────────────────
// §10 · Ownership — The Core Concept
// ─────────────────────────────────────────────────────────────────────────────
//
// THE rule: each value has exactly ONE owner. When the owner goes out of scope,
// the memory is freed — no GC, no manual free().
//
// Assignment of heap types (String, Vec, etc.) MOVES ownership:
//   let s2 = s1;   // s1 is now invalid — compiler error if you use it
//
// Stack-only types that implement `Copy` (i32, bool, f64, char, &T, ...) are
// COPIED instead of moved, so the original stays valid.
//
// To keep both variables alive for a heap type, use `.clone()` (explicit deep copy).
#[cfg(test)]
mod s10_ownership {
    #[test]
    fn move_invalidates_original() {
        let s1 = String::from("hello");
        let s2 = s1; // s1's ownership MOVED to s2
        // Uncomment the next line to see the compile error:
        // println!("{}", s1);  // error: value borrowed here after move
        assert_eq!(s2, "hello");
    }

    #[test]
    fn clone_keeps_both_alive() {
        let s1 = String::from("hello");
        let s2 = s1.clone(); // explicit deep copy — allocates new memory
        assert_eq!(s1, "hello"); // s1 still valid
        assert_eq!(s2, "hello");
    }

    #[test]
    fn copy_types_are_not_moved() {
        // i32 implements Copy — assignment copies the bits, original stays valid
        let x: i32 = 5;
        let y = x; // COPY, not move
        assert_eq!(x, 5); // still valid!
        assert_eq!(y, 5);

        // Same for bool, f64, char, tuples of Copy types, and &T references
        let b = true;
        let b2 = b;
        assert_eq!(b, b2);
    }

    fn takes_ownership(s: String) -> usize {
        s.len()
    } // s is dropped here — memory freed

    fn borrows_only(s: &String) -> usize {
        s.len()
    } // borrow ends, s is NOT dropped

    fn makes_copy(n: i32) -> i32 {
        n + 1
    } // n is copied in, caller's copy unaffected

    #[test]
    fn ownership_through_functions() {
        let s = String::from("hello");
        let len = takes_ownership(s); // s moved into function — can't use s after
        assert_eq!(len, 5);

        let s2 = String::from("world");
        let len2 = borrows_only(&s2); // s2 still alive after the call
        assert_eq!(s2, "world"); // fine
        assert_eq!(len2, 5);

        let x = 5;
        let _ = makes_copy(x); // x is i32 (Copy) — x still valid
        assert_eq!(x, 5);
    }
}

// ─────────────────────────────────────────────────────────────────────────────
// §11 · Borrowing & References
// ─────────────────────────────────────────────────────────────────────────────
//
// A reference lets you USE a value without taking ownership — you're "borrowing" it.
//   &T       — shared (immutable) borrow — many at once, nobody can mutate
//   &mut T   — exclusive (mutable) borrow — exactly ONE, no other borrows at same time
//
// The compiler enforces these rules at compile time:
//   Rule 1: any number of &T  OR  exactly one &mut T — never both simultaneously
//   Rule 2: a reference can never outlive the data it points to (no dangling pointers)
//
// Go comparison: passing `*string` to avoid copying, but without any safety guarantees.
#[cfg(test)]
mod s11_borrowing {
    fn length(s: &String) -> usize {
        s.len()
    } // borrow ends here — `s` is NOT freed

    fn append_world(s: &mut String) {
        s.push_str(", world");
    }

    fn first_word(s: &str) -> &str {
        // Returns a slice into the original string — no allocation
        match s.find(' ') {
            Some(i) => &s[..i],
            None => s,
        }
    }

    #[test]
    fn immutable_borrow_keeps_original_alive() {
        let s = String::from("hello");
        let len = length(&s); // lend s, don't give it away
        assert_eq!(len, 5);
        assert_eq!(s, "hello"); // s is still alive — we only borrowed it
    }

    #[test]
    fn mutable_borrow_allows_modification() {
        let mut s = String::from("hello");
        append_world(&mut s);
        assert_eq!(s, "hello, world");
    }

    #[test]
    fn multiple_immutable_borrows_are_fine() {
        let s = String::from("hello");
        let r1 = &s;
        let r2 = &s; // two readers at the same time — allowed
        assert_eq!(r1.len(), r2.len());
        // r1 and r2 borrows end here (last use)
    }

    #[test]
    fn borrow_ends_at_last_use_nll() {
        // Non-Lexical Lifetimes (NLL): borrows end when last USED, not at end of block
        let mut s = String::from("hello");
        let r1 = &s;
        let _ = r1.len(); // r1's last use — immutable borrow ends here
        // Now safe to take a mutable borrow:
        let r2 = &mut s;
        r2.push('!');
        assert_eq!(s, "hello!");
    }

    #[test]
    fn string_slice_borrow() {
        let s = String::from("hello world");
        let word = first_word(&s); // borrows s, returns a slice into it
        assert_eq!(word, "hello");
        // s still alive — we only borrowed it
        assert!(s.len() > 5);
    }
}

// ─────────────────────────────────────────────────────────────────────────────
// §12 · Lifetimes
// ─────────────────────────────────────────────────────────────────────────────
//
// Lifetimes prevent use-after-free. Usually the compiler infers them (elision).
// You only write explicit lifetime annotations when the compiler can't figure out
// how long a returned reference must be valid.
//
// Syntax: `'a` is a lifetime parameter. `&'a str` means "a reference that lives
// at least as long as `'a`".
//
// Beginner tip: if lifetime annotations feel painful, own the data
// (return `String` instead of `&str`) rather than fighting the borrow checker.
#[cfg(test)]
mod s12_lifetimes {
    // The compiler needs to know: does the returned reference point into `a` or `b`?
    // `'a` says: "the output lives as long as the SHORTER of a and b"
    fn longest<'a>(a: &'a str, b: &'a str) -> &'a str {
        if a.len() >= b.len() { a } else { b }
    }

    // A struct that holds a reference must declare a lifetime:
    // "An Excerpt cannot outlive the string data it points to."
    struct Excerpt<'a> {
        text: &'a str,
    }

    impl<'a> Excerpt<'a> {
        fn content(&self) -> &str {
            self.text
        }
    }

    #[test]
    fn longest_string() {
        let s1 = String::from("long string");
        let s2 = String::from("xy");
        // Both s1 and s2 are alive inside this scope, so longest is safe
        let result = longest(s1.as_str(), s2.as_str());
        assert_eq!(result, "long string");
    }

    #[test]
    fn struct_with_lifetime() {
        let novel = String::from("Call me Ishmael. Some years ago...");
        let first = novel.split('.').next().unwrap(); // &str into novel
        let excerpt = Excerpt { text: first };
        assert_eq!(excerpt.content(), "Call me Ishmael");
        // `excerpt` cannot be used after `novel` is dropped
    }

    #[test]
    fn static_lifetime_for_string_literals() {
        // 'static = lives for the entire program. All string literals have this.
        let s: &'static str = "I am baked into the binary";
        assert_eq!(s.len(), 26);
    }
}

// ─────────────────────────────────────────────────────────────────────────────
// §13 · Error Handling: Result & Option
// ─────────────────────────────────────────────────────────────────────────────
//
// Option<T>  — replaces nil. `Some(value)` or `None`.
// Result<T,E>— replaces (value, error). `Ok(value)` or `Err(e)`.
//
// The `?` operator:
//   — On Result: if Err, returns Err from the current function (like `if err != nil`)
//   — On Option: if None, returns None from the current function
//
// Common methods:
//   .unwrap()          — panics on None/Err (use in tests/prototypes only)
//   .expect("msg")     — panics with a message
//   .unwrap_or(x)      — returns x on None/Err
//   .map(|v| ...)      — transform the inner value, leave None/Err unchanged
//   .is_some()/.is_ok() — check without extracting
#[cfg(test)]
mod s13_errors {
    use std::num::ParseIntError;

    // ── Option ───────────────────────────────────────────────────────────────

    fn find_first_even(nums: &[i32]) -> Option<i32> {
        nums.iter().find(|&&x| x % 2 == 0).copied()
    }

    #[test]
    fn option_some_and_none() {
        assert_eq!(find_first_even(&[1, 3, 4, 5]), Some(4));
        assert_eq!(find_first_even(&[1, 3, 5]), None);
    }

    #[test]
    fn option_methods() {
        let v: Option<i32> = Some(5);
        let n: Option<i32> = None;

        assert_eq!(v.unwrap(), 5);
        assert_eq!(v.unwrap_or(0), 5);
        assert_eq!(n.unwrap_or(0), 0);
        assert_eq!(v.map(|x| x * 2), Some(10));
        assert_eq!(n.map(|x| x * 2), None);
        assert!(v.is_some());
        assert!(n.is_none());
    }

    // ── Result ───────────────────────────────────────────────────────────────

    fn parse_age(s: &str) -> Result<u8, String> {
        let n: i32 = s.parse().map_err(|e: ParseIntError| e.to_string())?; // ? = early return on Err
        if n < 0 || n > 150 {
            return Err(format!("age {} out of range [0,150]", n));
        }
        Ok(n as u8)
    }

    #[test]
    fn result_ok_and_err() {
        assert_eq!(parse_age("25"), Ok(25));
        assert!(parse_age("abc").is_err()); // parse failed
        assert!(parse_age("200").is_err()); // out of range
        assert!(parse_age("-5").is_err());  // out of range
    }

    #[test]
    fn result_methods() {
        let ok: Result<i32, &str> = Ok(42);
        let err: Result<i32, &str> = Err("oops");

        assert_eq!(ok.unwrap(), 42);
        assert_eq!(ok.unwrap_or(0), 42);
        assert_eq!(err.unwrap_or(0), 0);
        assert_eq!(ok.map(|x| x + 1), Ok(43));
        assert!(ok.is_ok());
        assert!(err.is_err());
    }

    // ── Chaining with ? ───────────────────────────────────────────────────────

    // `?` automatically propagates the error upward — equivalent to:
    //   if err != nil { return Err(err) }
    fn double_age(s: &str) -> Result<u16, String> {
        let age = parse_age(s)?; // returns Err early if parse_age fails
        Ok(age as u16 * 2)
    }

    #[test]
    fn question_mark_chains_errors() {
        assert_eq!(double_age("20"), Ok(40));
        assert!(double_age("bad").is_err());
        assert!(double_age("200").is_err());
    }
}

// ─────────────────────────────────────────────────────────────────────────────
// §14 · Collections: Vec, HashMap, HashSet
// ─────────────────────────────────────────────────────────────────────────────
//
// Vec<T>           — Go slice. Growable, heap-allocated sequence.
// HashMap<K, V>    — Go map. Unordered key-value store.
// HashSet<T>       — no direct Go equivalent. A set of unique values.
//
// HashMap entry API:
//   .entry(key).or_insert(v)   — insert v if key absent, return &mut value
//   *count += 1                — increment the returned mutable reference
#[cfg(test)]
mod s14_collections {
    use std::collections::{HashMap, HashSet};

    #[test]
    fn vec_push_access_slice() {
        let mut v = vec![1, 2, 3];
        v.push(4);
        assert_eq!(v.len(), 4);
        assert_eq!(v[0], 1);                       // panics if OOB
        assert_eq!(v.get(10), None);               // safe access via Option
        assert_eq!(&v[1..3], &[2, 3]);            // slice
    }

    #[test]
    fn vec_sort_dedup_retain() {
        let mut v = vec![3, 1, 4, 1, 5, 9, 2, 6, 5];
        v.sort();
        assert_eq!(v[0], 1);
        v.dedup();   // remove consecutive duplicates — only useful after sort
        assert_eq!(v, vec![1, 2, 3, 4, 5, 6, 9]);

        v.retain(|&x| x % 2 == 0); // keep only even numbers
        assert_eq!(v, vec![2, 4, 6]);
    }

    #[test]
    fn vec_iterate() {
        let v = vec![1, 2, 3];
        let sum: i32 = v.iter().sum();
        assert_eq!(sum, 6);

        // Mutably iterate
        let mut v2 = vec![1, 2, 3];
        for x in &mut v2 {
            *x *= 10;
        }
        assert_eq!(v2, vec![10, 20, 30]);
    }

    #[test]
    fn hashmap_insert_get_remove() {
        let mut m: HashMap<&str, i32> = HashMap::new();
        m.insert("alice", 100);
        m.insert("bob", 90);

        assert_eq!(m["alice"], 100);     // panics if missing
        assert_eq!(m.get("alice"), Some(&100)); // safe access
        assert_eq!(m.get("charlie"), None);

        m.remove("bob");
        assert!(!m.contains_key("bob"));
    }

    #[test]
    fn hashmap_entry_word_count() {
        // Classic pattern: count occurrences. Same as Go's `if _, ok := m[k]; !ok`
        let text = "hello world hello rust world hello";
        let mut counts: HashMap<&str, u32> = HashMap::new();

        for word in text.split_whitespace() {
            let count = counts.entry(word).or_insert(0); // insert 0 if absent
            *count += 1;                                 // dereference to increment
        }

        assert_eq!(counts["hello"], 3);
        assert_eq!(counts["world"], 2);
        assert_eq!(counts["rust"], 1);
    }

    #[test]
    fn hashset_unique_and_operations() {
        let a: HashSet<i32> = [1, 2, 3, 4].iter().cloned().collect();
        let b: HashSet<i32> = [3, 4, 5, 6].iter().cloned().collect();

        // Set semantics — duplicates automatically ignored
        let mut s = a.clone();
        s.insert(3); // already there
        assert_eq!(s.len(), 4); // still 4

        let union: HashSet<_> = a.union(&b).collect();
        assert_eq!(union.len(), 6);

        let intersection: HashSet<_> = a.intersection(&b).collect();
        assert_eq!(intersection.len(), 2); // {3, 4}

        let only_in_a: HashSet<_> = a.difference(&b).collect();
        assert_eq!(only_in_a.len(), 2); // {1, 2}
    }
}

// ─────────────────────────────────────────────────────────────────────────────
// §15 · Strings: &str vs String
// ─────────────────────────────────────────────────────────────────────────────
//
// &str  — borrowed string slice: pointer + length into some existing string data.
//         Immutable. String literals have type &'static str.
// String — owned, heap-allocated, growable string.
//
// Rule of thumb:
//   • Function parameters: prefer `&str` — works for both literals and Strings
//   • Return value / struct field: use `String` if you need to own/grow it
//
// Deref coercion: `&String` automatically converts to `&str` when needed.
#[cfg(test)]
mod s15_strings {
    fn greet(name: &str) -> String { // &str parameter accepts both "literal" and &owned_string
        format!("Hello, {}!", name)
    }

    #[test]
    fn literal_is_str() {
        let s: &str = "hello"; // lives in the binary, 'static lifetime
        assert_eq!(s.len(), 5); // byte length
    }

    #[test]
    fn various_ways_to_make_string() {
        let a = String::from("hello");
        let b = "hello".to_string();
        let c = format!("{}", "hello");
        assert_eq!(a, b);
        assert_eq!(b, c);
    }

    #[test]
    fn coerce_string_to_str() {
        let owned = String::from("alice");
        // &owned: &String, which auto-derefs to &str
        assert_eq!(greet(&owned), "Hello, alice!");
        assert_eq!(greet("bob"), "Hello, bob!"); // &str literal works directly
    }

    #[test]
    fn convert_between_types() {
        let borrowed: &str = "hello";
        let owned: String = borrowed.to_string(); // allocates
        let back: &str = &owned;                  // free — just a borrow
        assert_eq!(borrowed, back);
    }

    #[test]
    fn string_mutation() {
        let mut s = String::from("hello");
        s.push(' ');         // append a char
        s.push_str("world"); // append &str
        assert_eq!(s, "hello world");
    }

    #[test]
    fn common_string_operations() {
        let s = String::from("  Hello, World!  ");
        assert_eq!(s.trim(), "Hello, World!"); // like strings.TrimSpace
        assert!(s.contains("World"));
        assert_eq!(s.trim().to_lowercase(), "hello, world!");
        assert_eq!(s.trim().replace("World", "Rust"), "Hello, Rust!");
    }

    #[test]
    fn split_and_collect() {
        let csv = "one,two,three";
        let parts: Vec<&str> = csv.split(',').collect();
        assert_eq!(parts, vec!["one", "two", "three"]);
        assert_eq!(parts.join(" | "), "one | two | three");
    }

    #[test]
    fn byte_len_vs_char_count() {
        // Rust strings are UTF-8. Non-ASCII chars can be multiple bytes.
        let emoji = "hello 🦀"; // crab emoji = 4 bytes
        assert_eq!(emoji.len(), 10);          // byte length (not char count!)
        assert_eq!(emoji.chars().count(), 7); // actual character count
    }
}

// ─────────────────────────────────────────────────────────────────────────────
// §16 · Closures & Iterators
// ─────────────────────────────────────────────────────────────────────────────
//
// Closures: anonymous functions that can capture variables from the enclosing scope.
//   |args| expression          — immutable capture by reference
//   move |args| expression     — MOVE captured variables into the closure (required for threads)
//
// Iterator adapters (.map, .filter, .fold, ...):
//   — LAZY: nothing happens until you call a "consuming" adapter (.collect, .sum, .for_each)
//   — ZERO-COST: compiles to the same machine code as a hand-written for loop
//   — COMPOSABLE: chain them like Unix pipes
#[cfg(test)]
mod s16_closures_iterators {
    #[test]
    fn basic_closure() {
        let add = |a: i32, b: i32| a + b;
        assert_eq!(add(2, 3), 5);
    }

    #[test]
    fn closure_captures_by_reference() {
        let factor = 10;
        let scale = |x: i32| x * factor; // borrows `factor` from outer scope
        assert_eq!(scale(5), 50);
        assert_eq!(scale(3), 30);
    }

    #[test]
    fn move_closure_owns_captured_data() {
        let s = String::from("hello");
        let get_s = move || s.clone(); // s is MOVED into the closure
        // `s` can't be used here anymore
        assert_eq!(get_s(), "hello");
        assert_eq!(get_s(), "hello"); // can call multiple times (it clones internally)
    }

    #[test]
    fn map_transform_each_element() {
        let nums = vec![1, 2, 3, 4, 5];
        let doubled: Vec<i32> = nums.iter().map(|&x| x * 2).collect();
        assert_eq!(doubled, vec![2, 4, 6, 8, 10]);
    }

    #[test]
    fn filter_keep_matching() {
        let nums = vec![1, 2, 3, 4, 5, 6];
        let evens: Vec<i32> = nums.iter().filter(|&&x| x % 2 == 0).cloned().collect();
        assert_eq!(evens, vec![2, 4, 6]);
    }

    #[test]
    fn filter_map_filter_and_transform() {
        // filter_map: keep elements for which the closure returns Some(value)
        let strings = vec!["1", "two", "3", "four", "5"];
        let nums: Vec<i32> = strings.iter().filter_map(|s| s.parse().ok()).collect();
        assert_eq!(nums, vec![1, 3, 5]);
    }

    #[test]
    fn fold_is_manual_accumulation() {
        let nums = vec![1, 2, 3, 4, 5];
        // fold(initial, |accumulator, element| new_accumulator)
        let product = nums.iter().fold(1, |acc, &x| acc * x);
        assert_eq!(product, 120);

        let sum: i32 = nums.iter().sum(); // shorthand for fold to sum
        assert_eq!(sum, 15);
    }

    #[test]
    fn any_all_find() {
        let nums = vec![1, 3, 5, 7, 8];
        assert!(nums.iter().any(|&x| x % 2 == 0));    // at least one even
        assert!(!nums.iter().all(|&x| x % 2 == 0));   // not ALL even
        assert_eq!(nums.iter().find(|&&x| x > 5), Some(&7)); // first > 5
    }

    #[test]
    fn chain_and_zip() {
        let a = vec![1, 2, 3];
        let b = vec![4, 5, 6];
        let all: Vec<i32> = a.iter().chain(b.iter()).cloned().collect();
        assert_eq!(all, vec![1, 2, 3, 4, 5, 6]);

        let keys = vec!["x", "y"];
        let vals = vec![10, 20];
        let pairs: Vec<_> = keys.iter().zip(vals.iter()).collect();
        assert_eq!(*pairs[0].0, "x");
        assert_eq!(*pairs[0].1, 10);
    }

    #[test]
    fn take_skip_enumerate() {
        let every_other: Vec<usize> = (0..10).skip(2).take(4).collect();
        assert_eq!(every_other, vec![2, 3, 4, 5]);

        let labeled: Vec<String> = ["a", "b", "c"]
            .iter()
            .enumerate()
            .map(|(i, s)| format!("{}: {}", i, s))
            .collect();
        assert_eq!(labeled, vec!["0: a", "1: b", "2: c"]);
    }

    #[test]
    fn flat_map_and_flatten() {
        let sentences = vec!["hello world", "foo bar baz"];
        let words: Vec<&str> = sentences.iter().flat_map(|s| s.split_whitespace()).collect();
        assert_eq!(words, vec!["hello", "world", "foo", "bar", "baz"]);
    }

    #[test]
    fn chained_pipeline() {
        // Like Go:
        // var result []string
        // for _, u := range users { if u.Active && u.Score > 50 { result = append(result, strings.ToUpper(u.Name)) } }
        struct User { name: &'static str, active: bool, score: i32 }
        let users = vec![
            User { name: "alice", active: true, score: 90 },
            User { name: "bob", active: false, score: 80 },
            User { name: "carol", active: true, score: 40 },
            User { name: "dave", active: true, score: 75 },
        ];

        let result: Vec<String> = users.iter()
            .filter(|u| u.active && u.score > 50)
            .map(|u| u.name.to_uppercase())
            .collect();

        assert_eq!(result, vec!["ALICE", "DAVE"]);
    }
}

// ─────────────────────────────────────────────────────────────────────────────
// §17 · Generics
// ─────────────────────────────────────────────────────────────────────────────
//
// Like Go 1.18+ generics, but Rust uses TRAIT BOUNDS instead of type constraints.
// All generics are monomorphized at compile time — zero runtime overhead.
//
//   fn max<T: PartialOrd>(a: T, b: T) -> T  — single bound
//   fn foo<T: Display + Clone>(...)          — multiple bounds with `+`
//   where T: Display, U: Debug              — `where` clause for cleaner formatting
#[cfg(test)]
mod s17_generics {
    use std::fmt;

    fn max<T: PartialOrd>(a: T, b: T) -> T {
        if a > b { a } else { b }
    }

    fn print_and_return<T: fmt::Display + Clone>(val: T) -> T {
        let _ = format!("{}", val); // would normally println! here
        val.clone()
    }

    // `where` clause — same as `T: Display + Clone` but easier to read for complex bounds
    fn describe<T, U>(t: &T, u: &U) -> String
    where
        T: fmt::Display,
        U: fmt::Debug,
    {
        format!("display={}, debug={:?}", t, u)
    }

    // Generic struct — Stack<T> works for any T
    struct Stack<T> {
        items: Vec<T>,
    }

    impl<T> Stack<T> {
        fn new() -> Self { Stack { items: Vec::new() } }
        fn push(&mut self, item: T) { self.items.push(item); }
        fn pop(&mut self) -> Option<T> { self.items.pop() }
        fn peek(&self) -> Option<&T> { self.items.last() }
        fn is_empty(&self) -> bool { self.items.is_empty() }
        fn len(&self) -> usize { self.items.len() }
    }

    #[test]
    fn generic_max_works_for_multiple_types() {
        assert_eq!(max(3, 7), 7);
        assert_eq!(max(3.14_f64, 2.71), 3.14);
        assert_eq!(max("apple", "banana"), "banana"); // lexicographic order
    }

    #[test]
    fn generic_stack_with_integers() {
        let mut s: Stack<i32> = Stack::new();
        assert!(s.is_empty());
        s.push(1);
        s.push(2);
        s.push(3);
        assert_eq!(s.len(), 3);
        assert_eq!(s.peek(), Some(&3)); // peek doesn't remove
        assert_eq!(s.pop(), Some(3));
        assert_eq!(s.pop(), Some(2));
        assert_eq!(s.len(), 1);
    }

    #[test]
    fn generic_stack_with_strings() {
        let mut s: Stack<String> = Stack::new();
        s.push("hello".to_string());
        s.push("world".to_string());
        assert_eq!(s.pop(), Some("world".to_string()));
        assert_eq!(s.pop(), Some("hello".to_string()));
        assert_eq!(s.pop(), None); // empty
    }

    #[test]
    fn where_clause() {
        let result = describe(&42, &vec![1, 2, 3]);
        assert!(result.contains("42"));
        assert!(result.contains("[1, 2, 3]"));
    }

    #[test]
    fn print_and_return_generic() {
        let returned = print_and_return(99_i32);
        assert_eq!(returned, 99);
        let returned = print_and_return("hello");
        assert_eq!(returned, "hello");
    }
}

// ─────────────────────────────────────────────────────────────────────────────
// §18 · Concurrency: Threads & Channels
// ─────────────────────────────────────────────────────────────────────────────
//
// Rust threads are OS threads (heavier than Go goroutines).
// The borrow checker prevents data races at compile time — unlike Go, which
// has `-race` as a runtime detector.
//
// Channels (mpsc = multi-producer, single-consumer):
//   tx.send(v)   — like `ch <- v`
//   rx.recv()    — like `v := <-ch`
//   tx.clone()   — create an additional sender (no equivalent in Go channels)
//
// Shared state: Arc<Mutex<T>>
//   Arc  — atomic reference counting (like Go's implicit sharing via goroutines)
//   Mutex— ensures only one thread holds a &mut T at a time
#[cfg(test)]
mod s18_concurrency {
    use std::sync::{Arc, Mutex};
    use std::sync::mpsc;
    use std::thread;

    #[test]
    fn spawn_and_join() {
        // thread::spawn returns a JoinHandle — call .join() to wait (like WaitGroup)
        let handle = thread::spawn(|| 42);
        let result = handle.join().unwrap();
        assert_eq!(result, 42);
    }

    #[test]
    fn move_data_into_thread() {
        let data = vec![1, 2, 3];
        // `move` is required to transfer ownership into the thread closure
        let handle = thread::spawn(move || data.iter().sum::<i32>());
        assert_eq!(handle.join().unwrap(), 6);
    }

    #[test]
    fn channel_producer_consumer() {
        let (tx, rx) = mpsc::channel::<i32>();

        // Producer thread
        thread::spawn(move || {
            for i in 1..=3 {
                tx.send(i).unwrap(); // like ch <- i
            }
            // tx dropped here — rx loop will end
        });

        // Consumer: rx.iter() blocks until all senders are dropped
        let received: Vec<i32> = rx.iter().collect();
        assert_eq!(received, vec![1, 2, 3]);
    }

    #[test]
    fn multiple_producers_single_consumer() {
        let (tx, rx) = mpsc::channel::<i32>();
        let tx2 = tx.clone(); // clone sender for second producer

        let h1 = thread::spawn(move || tx.send(10).unwrap());
        let h2 = thread::spawn(move || tx2.send(20).unwrap());
        h1.join().unwrap();
        h2.join().unwrap();

        let mut vals: Vec<i32> = rx.iter().collect();
        vals.sort(); // order is non-deterministic between threads
        assert_eq!(vals, vec![10, 20]);
    }

    #[test]
    fn arc_mutex_shared_counter() {
        // Arc = atomic ref count (shared ownership across threads)
        // Mutex = guard that gives exclusive mutable access
        let counter = Arc::new(Mutex::new(0_i32));
        let mut handles = vec![];

        for _ in 0..10 {
            let c = Arc::clone(&counter); // cheap: just increments a ref count
            handles.push(thread::spawn(move || {
                let mut num = c.lock().unwrap(); // blocks until lock available
                *num += 1;
            })); // MutexGuard drops here — lock released automatically
        }

        for h in handles { h.join().unwrap(); }
        assert_eq!(*counter.lock().unwrap(), 10);
    }
}

// ─────────────────────────────────────────────────────────────────────────────
// §19 · Async / Await with Tokio
// ─────────────────────────────────────────────────────────────────────────────
//
// Go goroutines = M:N green threads managed by the runtime (always running).
// Rust async = state machine that does NOTHING until polled by an executor.
//
// Executor = Tokio (the runtime that drives futures — like Go's runtime scheduler).
// `async fn` returns a Future; it starts executing only when you `.await` it.
//
// Parallels:
//   tokio::spawn(async { })   ≈  go func() { }()
//   tokio::join!(a, b)        ≈  errgroup — wait for both, run concurrently
//   tokio::select! { }        ≈  select { } — first to complete wins
//   handle.await              ≈  wg.Wait() / <-done
#[cfg(test)]
mod s19_async {
    async fn add_async(a: i32, b: i32) -> i32 {
        // In real code this might be a DB query or HTTP call
        a + b
    }

    async fn fetch_simulated(id: u32) -> String {
        format!("result-{}", id)
    }

    #[tokio::test]
    async fn basic_await() {
        let result = add_async(2, 3).await; // .await drives the future to completion
        assert_eq!(result, 5);
    }

    #[tokio::test]
    async fn spawn_concurrent_task() {
        // tokio::spawn = goroutine equivalent. Returns a JoinHandle.
        let handle = tokio::spawn(async { add_async(10, 20).await });
        let result = handle.await.unwrap(); // await the JoinHandle
        assert_eq!(result, 30);
    }

    #[tokio::test]
    async fn join_runs_both_concurrently() {
        // tokio::join! starts both at once and waits for BOTH to finish
        // (like Go's errgroup.Wait after spawning two goroutines)
        let (r1, r2) = tokio::join!(
            fetch_simulated(1),
            fetch_simulated(2),
        );
        assert_eq!(r1, "result-1");
        assert_eq!(r2, "result-2");
    }

    #[tokio::test]
    async fn select_first_wins() {
        // tokio::select! returns as soon as EITHER branch completes
        // (like Go's select { case v := <-ch1: ... case v := <-ch2: ... })
        let result = tokio::select! {
            r = fetch_simulated(1) => r,
            r = fetch_simulated(2) => r,
        };
        assert!(result.starts_with("result-"));
    }

    #[tokio::test]
    async fn async_error_propagation() {
        async fn might_fail(ok: bool) -> Result<i32, String> {
            if ok { Ok(42) } else { Err("failed".to_string()) }
        }

        // `?` works in async functions just like in sync ones
        async fn caller() -> Result<i32, String> {
            let v = might_fail(true).await?; // returns Err early if Err
            Ok(v * 2)
        }

        assert_eq!(caller().await, Ok(84));
    }
}

// ─────────────────────────────────────────────────────────────────────────────
// §21 · Common Patterns & Idioms
// ─────────────────────────────────────────────────────────────────────────────
#[cfg(test)]
mod s21_patterns {

    // ── Builder Pattern ──────────────────────────────────────────────────────
    // Common in Rust APIs where Go would use functional options or a config struct.
    // Each setter method takes `self` (by value) and returns `Self` — enabling chaining.
    struct Config {
        host: String,
        port: u16,
        timeout_secs: u64,
    }

    struct ConfigBuilder {
        host: String,
        port: u16,
        timeout_secs: u64,
    }

    impl ConfigBuilder {
        fn new() -> Self {
            Self { host: "localhost".into(), port: 8080, timeout_secs: 30 }
        }
        fn host(mut self, h: &str) -> Self { self.host = h.into(); self }
        fn port(mut self, p: u16) -> Self { self.port = p; self }
        fn timeout(mut self, t: u64) -> Self { self.timeout_secs = t; self }
        fn build(self) -> Config {
            Config { host: self.host, port: self.port, timeout_secs: self.timeout_secs }
        }
    }

    #[test]
    fn builder_pattern() {
        let c = ConfigBuilder::new().host("0.0.0.0").port(443).timeout(60).build();
        assert_eq!(c.host, "0.0.0.0");
        assert_eq!(c.port, 443);
        assert_eq!(c.timeout_secs, 60);
    }

    #[test]
    fn builder_defaults() {
        let c = ConfigBuilder::new().build(); // all defaults
        assert_eq!(c.host, "localhost");
        assert_eq!(c.port, 8080);
    }

    // ── Newtype Pattern ──────────────────────────────────────────────────────
    // Wrap a primitive in a newtype struct to get distinct types.
    // Prevents accidentally passing a UserId where an OrderId is expected.
    #[derive(Debug, PartialEq)]
    struct UserId(u64);
    #[derive(Debug, PartialEq)]
    struct OrderId(u64);

    fn describe_user(id: &UserId) -> String { format!("user-{}", id.0) }

    #[test]
    fn newtype_prevents_type_mixups() {
        let uid = UserId(42);
        let oid = OrderId(42);
        // describe_user(&oid); // compile error: expected &UserId, got &OrderId
        assert_eq!(describe_user(&uid), "user-42");
        assert_ne!(uid, UserId(99));
        // uid and oid are distinct types even though both wrap u64
        let _ = oid; // just to avoid unused warning
    }

    // ── Derive Macros for Free Functionality ─────────────────────────────────
    // `#[derive(...)]` auto-generates trait implementations.
    // No need to write boilerplate — the compiler generates it.
    #[derive(Debug, Clone, PartialEq, Default)]
    struct AppConfig {
        host: String,
        port: u16,
        debug: bool,
    }

    #[test]
    fn derive_macros() {
        let c1 = AppConfig::default(); // Default: "", 0, false
        assert_eq!(c1.host, "");
        assert_eq!(c1.port, 0);
        assert!(!c1.debug);

        let c2 = c1.clone();           // Clone
        assert_eq!(c1, c2);            // PartialEq
        let s = format!("{:?}", c2);   // Debug
        assert!(s.contains("host"));
    }

    // ── ? in Functions Returning Result ──────────────────────────────────────
    fn parse_positive(s: &str) -> Result<i32, String> {
        let n: i32 = s.parse().map_err(|e: std::num::ParseIntError| e.to_string())?;
        if n <= 0 {
            return Err(format!("{} is not positive", n));
        }
        Ok(n)
    }

    fn double_positive(s: &str) -> Result<i32, String> {
        let n = parse_positive(s)?; // propagate error automatically
        Ok(n * 2)
    }

    #[test]
    fn question_mark_error_chain() {
        assert_eq!(double_positive("21"), Ok(42));
        assert!(double_positive("abc").is_err());
        assert!(double_positive("-5").is_err());
    }
}
