# Chapter 4: Understanding Ownership

The authors list three key rules of ownership.

+ Every value in Rust has an owner.
+ There can only be one owner at a time.
+ When the owner goes out of scope, the value will be dropped.

The authors use the `String` data type to discuss ownership.  String literals
are of limited use.  We cannot mutate them, and we have to know the value up
front.  The `String` type, however, can be used when we will need text that may
change and whose value is unknown to us at compile time.  In order to make this
work, the `String` type allocates room for the data on the heap.  (This is more
costly in various ways, but there is no way around it.)  When you allocate on
the heap, you (or your runtime) must do two things.

1. The memory must be requested from an allocator at runtime.
1. The memory must be returned to the allocator when it is no longer needed.

In Rust, programmers do the first thing themselves, and ownership takes care of
the second for them.  By contrast, in languages with a garbage collector, the GC
frees memory for the programmer.  And in languages like C, programmers must free
memory themselves.  Both garbage collection and manual freeing of memory come
with significant problems.  (Briefly, garbage collection incurs a significant
runtime cost, and manual freeing of memory is prone to errors and dangerous
bugs.)  Rust takes a third path: "memory is automatically returned once the
variable that owns it goes out of scope" (64).  Rust automatically calls
a function `drop`, and that function returns memory when variables go out of
scope.

Single ownership means that assigning a variable to another variable can have
surprising effects.  For example, in Rusty the following assignments work very
differently.

```rust
// Both x and y end up with independent immutable values of 5.
let x = 5;
let y = x;

// s2 takes over s1. s1 is no longer valid, and it cannot be used.
let s1 = String::from("Hello, world!");
let s2 = s1;
```

What other languages would call a *shallow copy*, where a new variable has the
same pointer reference as an old one, Rust calls a *move*.  So in the example
above, s1 has been moved into s2.

Rust never automatically does a deep copy.  Thus, you can always assume that
automatic copying is cheap in terms of performance.

If you need to deep copy a string, you use its `clone` method.  E.g., `let s2
= s1.clone();`.

However, stack-only data is copied on assignment.  Thus, `let x = 5; let y = x;`
assigns the *value* of x to y.  In addition, both variables remain valid after
the copy.  Values like integers that are stored on the stack have the `Copy`
trait in Rust.  This means that it is copied (as a value) rather than moved like
`String` items are.  You can always check documentation to see what types have
the `Copy` trait, but here are some other notes.  All integer types have the
trait, so do booleans, floating-point types, the character type, and tuples if
they only contain types that have the `Copy` trait.

Ownership and the `Copy` and `Drop` traits work about the way you would expect
when it comes to functions.  If you pass something with the `Copy` trait to
a function, the value is copied.  If you pass something with the `Drop` trait to
a function, the value is moved.  One way around this is to return the value and
reassign it at the end of the function.

```rust
let s1 = String::from("Hello, world!");
let s2 = takes_and_returns(s1);

fn takes_and_returns(s: String) -> String {
    // Do whatever
    s
}
```

However, this can be annoying, so Rust provides an easier way.  Instead of
accepting and returning a value, a function can accept a reference to a string.
In that case, the function can work on the value without (ever) taking ownership
of that value.  This is much better than having to return a value every time.
Compare the following.

```rust
// If we must return the value, we must reassign.
let s1 = String::from("Hello, world!");
let (s1, s1_len) = get_len_and_return(s1);

fn get_len_and_return(s: String) -> (String, usize) {
    (s, s.len())
}

// If we use a reference, we can avoid the extra return value which causes us to
// need a tuple.
let s2 = String::from("Goodbye, return!");
let s2_len = get_len(s2);

fn get_len(s: &String) -> usize {
    s.len()
}
```

Rust refers to the use of references in this way as *borrowing*.  Since you are
only borrowing the item, so to speak, ownership remains single and the original
is not moved.

By default, references are immutable.  That means, for example, that the code
above works great, but if we try to change the string that we have a reference
to, then the code will not compile.  However, we can fix this by using mutable
references.

```rust
// Broken!
let s1 = String::from("I am trying things.");

fn change_string(s: &String) {
    s.push_string(" This is fun, right?")
}

// Works!
let mut s2 = String::from("I am trying more things.");

fn change_string(s: &mut String) {
    s.push_str(" This is fun, right?")
}
```

Note, however, that you can only have one mutable reference to anything at one
time.  In addition, you cannot have an immutable and mutable reference to one
thing at one time.  (You can, however, have multiple immutable references to one
thing at one time.  I suppose that this makes sense: since the reference is
immutable, Rust still protects you against data races.)

The authors provide two rules for references.

+ At any given time, you can have *either* one mutable reference or any number
  of immutable references.  (Rust protects you against data races.)
+ References must always be valid.  (Rust will not let you create a dangling
  reference.)

Slices refer to a contiguous sequence of items in a collection rather than an
entire collection.  Since a slice is a reference, it does not have ownership.

A string slice refers to part of a `String`.  You create one this way.

```rust
let s = String::from("Hello, world!");

let hello = &s[0..5];
let world = &s[6..11];
```

The range in a slice is a `[closed, open)` interval.  That is, it includes its
starting point, but not its end point.  Thus, the slice `&s[0..5]` contains 0,
1, 2, 3, and 4, but not 5.  If you omit the first number, Rust assumes that you
mean `0`, and you start at the start of the item.  If you omit the second
number, Rust assumes that you mean `item.len()`, and you end at the end of the
item.  Thus, `&s[:]` is shorthand for a slice that is the whole thing.

This brings us back around to string literals and their type.  String literals
have a type `&str` because they are slices pointing to a specific place in
a binary.
