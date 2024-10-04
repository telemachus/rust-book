# Chapter 3: Common Programming Concepts

## Variables and Mutability

Rust variables are immutable by default.  If you want a variable to be mutable,
you must add `mut` in its initialization.  Rust uses `let` to declare variables,
both immutable and mutable ones.  Thus, the declarations can get rather long.

```rust
let x = 10;
x = 20;          // Error: you cannot assign a new value to x.
let mut y = 10;
y = 20;          // This is fine since y is a mutable variable.
```

You can also create a constant with `const`.  Why do we need `const` if
variables are immutable by default?  Constants are more than just immutable.
For constants, you *must* annotate the type of the variable (i.e., you cannot
rely on type inference), and you can only set a constant to a value that is
itself constant.  That is, you cannot assign a `const` a value that can only be
worked out at runtime.  (I assume that this all leads to various optimizations
of constants versus other immutable variables.)  By convention, Rust programmers
write constants in all caps with underscores.

```rust
const SECONDS_PER_DAY: u32 = 60 * 60 * 24;
```

Although variables are immutable by default, there is a way that you can put
a new value—and even a new type—into an old variable: shadowing.  Consider the
examples below.

```rust
// This is illegal and will not compile.
let x = 10;
x = 20;

// This, however, is perfectly legal and it will compile.
let x = 10;
let x = 20;
let x = "Hello, world!";
```

Shadowing and making a variable mutable with `mut` are different in several
ways.  First, you must use `let` when you shadow a variable.  As a result, the
variable itself is *immutable* after you have declared (or redeclared) it with
`let`.  Second, a mutable variable declared with `mut` must remain of one type.
When you shadow a variable, however, you can change its type each time you
redeclare it with `let` (if you want).

## Data Types

Every value in a Rust program has a specific data type.  You often don't need to
declare the type of a value because Rust can often infer it.  However, sometimes
you do need to explicitly declare what type you want.  (If you do not in such
cases, the compiler will tell you that you should.)  In this section, the book
looks at two main types of values: scalar and compound values.  A scalar type is
a single value, and a compound type is multiple or complex.

### Scalar Types

There are four main scalar types in Rust: integers, floating-point numbers,
Booleans, and characters.

Rust has several integer types.

| Length   | Signed  | Unsigned |
---------------------------------
| 8-bit    | i8      | u8       |
| 16-bit   | i16     | u16      |
| 32-bit   | i32     | u32      |
| 64-bit   | i64     | u64      |
| 128-bit  | i128    | u128     |
| arch     | isize   | usize    |
---------------------------------

Signed numbers can be positive, negative, and zero.  Unsigned numbers can only
be zero or positive.  Signed numbers can store numbers from -(2^n-1^) to
(2^n-1^)-1 inclusive.  Thus, and i8 can store from -(2^7^) to (2^7^)-1
inclusive.  That is, an i8 can store from -128 to 127.

Unsigned numbers can store from 0 to (2^n^)-1.  So a u8 can store from 0 to
(2^8^)-1.  That is, a u8 can store from 0 to 255.

The size of a usize or isize integer is determined by the architecture of the
machine.  It will be 64 bits on a 64-bit machine and 32 bits on a 32-bit
machine.

You can write integer literals as decimal, hex, octal, and binary.  You prefix
hex literals with `0x`, octal literals with `0o`, and binary literals with `0b`.
In addition, you can write u8 literals as bytes.  E.g., `b'A'`.  When you write
out integer literals, you can add underscores to visually separate meaningful
parts of the number.  E.g., you can write 89_000 or 0b1111_0000.  Finally, you
can put a type suffix at the end of an integer literal to specify its type.
E.g., 84u8.

Which integer should you pick? If you don't have a specific need in mind, then
i32 is a good start.  You will use `isize` and `usize` most often to index
a collection.  (Why would you need `isize` for a collection?  How can
a collection be negative?)

Finally, what happens if you try to use a number that is too large for the type
of variable you have?  In this case, Rust may overflow.  E.g., 256 in a u8
becomes 0, 257 becomes 1, and so on.  Rust provides various ways to deal with
overflow.  See the box on page 38 for discussion.

### Floating-Point Types

Rust provides two primitive types for numbers with fractional parts: f32 and
f64.  The default floating-point type is f64 because it has more precision than
an f32.  Both floating-point types are signed.

### Numeric Operations

Rust supports all the basic math you would expect: addition, subtraction,
multiplication, division, and remainder.  If you divide integers, Rust truncates
towards zero.  Rust does not seem to support exponentiation with an operator.
(You can use [`pow`][pow] for exponentiation.)

### The Boolean Type

Booleans in Rust can be `true` or `false`, you specify them using `bool`, and
they are a single byte in size.  For the most part, you use booleans in an `if`
expression and for control flow.

### The Character Type

Characters in Rust are specified with `char`, and you write a character literal
in single quotes.  In Rust, a char is four bytes in size, and thus it can store
any single Unicode character.  However, the book stresses that Unicode does not
have the concept of "character."  So we cannot rely on a char in Rust to be
exactly what we expect of a character in writing.  They say more about this
later when they discuss UTF8 strings.

## Compound Types

Compound types allow you to group multiple scalar values.  Rust provides two
basic compound types: tuples and arrays.

A tuple groups together a number of values of multiple types into one compound
type.  You can use pattern matching to destructure and assign values from
a tuple.  You can also index into a specific item in a tuple using `.`.

```rust
let tup: (i32, f64, u8) = (500, 6.4, 1);
let (x, y, z) = tup;
let five_hundred = tup.0;
```

A tuple with no values is special name in Rust: "unit".  You represent a unit
with an empty set of parentheses.  This is one way to represent an empty value
or empty return type in Rust.  An expression that does not explicitly return
a value returns the unit value.  You can also explicitly use `()` to signal that
you have, e.g., a do-nothing expression or match in a pattern.

An array is a collection of multiple values of the same type.  Rust arrays have
a fixed length.  Arrays are allocated on the stack rather than the heap.  (More
on the stack and the heap in a later chapter.)  Arrays are also useful if you
always have a fixed number of items.  If you need a collection of multiple
values that can grow and shrink, you want a vector.  The book will discuss
vectors later.

You can write an array's type in square brackets; you use a semicolon to
separate the type of elements and the size of the array.

```rust
let a: [i32; 5] = [1, 2, 3, 4, 5];
```

Often, you don't need to specify type and length in this way because Rust can
infer both values.  You can also initialize an array with a single value
a specific number of times.  The following creates a thousand-item array where
every value is initially zero.

```rust
let a = [0; 1000];
```

You use brackets to index the items of an array.  E.g., `a[0]`.  If you try to
index beyond the bounds of the array, you will cause a runtime panic.

## Functions

You declare functions in Rust using the `fn` keyword.  By convention, Rust uses
snake case for function names and variable names.  You can declare function
names after they are called in a piece of Rust code.

```rust
fn main() {
    println!("Hello, world!");
    another_function();
}

fn another_function() {
    println!("Another function!");
}
```

If a function takes parameters, you specify their name and type in the
declaration of the function.

```rust
fn another_function(x: i32) {
    println!("The value of x is {x}.");
}
```

The body of a function in Rust is a series of statements and an optional final
expression.  Statements "are instructions that perform some action and do not
return a value" (46).  An expression "evaluates to a resultant value" (46).
Variable declarations and assignments are statements; so are function
definitions.  Statements do not return a value, so you cannot use an assignment
as the value for another assignment.

```rust
// This will not compile.
let x = (let y = 6);
```

Expressions in Rust do not include a semicolon at their end.  If you add
a semicolon to the end of an expression, you turn it into a statement.  At that
point, the (former) expression no longer returns a value.  Keep this in mind
when you write functions that you want to return a value.

If a function returns a value, Rust code does not name the return value.  You
specify the type after an ASCII arrow.

```rust
fn five() -> i52 {
    5    // Note: no final semicolon!
}
```

## Control Flow

Rust provides `if` expressions and loops for control flow, just like most
languages.  Here are a few key notes about each.

### `if` Expressions

+ The condition must be (or return) a boolean.  Unlike, e.g., Ruby, Rust will
  not coerce a number or string into a boolean value.
+ You can follow `if` with any number of optional `else if` blocks and an
  optional final `else` block.  But if you have a lot of `else` blocks, you
  should probably use `match` instead.
+ The `if` block is itself an expression.  Thus, you can use `if` with `let` in
  order to assign values: `let n = if bool_var { 5 } else { 6 };`.

### Repetition with Loops

Rust provides three types of loops: `loop`, `while`, and `for`.  Here are some
notes about all three.

+ `loop` kicks off an infinite loop.  You must manually break out of it using
  `break`.  You can also use `continue` within a loop to jump to the next full
  iteration of the loop and skip some of the block.  `loop`, like `if`, is an
  expression.  Thus, you can use it with `let` to assign values.
+ You can set a label on a loop.  Labels must begin with a single quote mark.
  These labels allow you to target a `break` or `continue`.  By default, both
  `break` and `continue` operate on the innermost loop.  You may want to label
  loops if you have nested loops.
+ `while` provides a simple way to run a loop for as long as a condition
  evaluates to true.  (You could write your own equivalent of `while` using
  `loop`, `if`, `else`, and `break`, but `while` makes things simpler and
  clearer.)
+ `for` provides a way to execute some code for each item in a collection.
  Again, you could write your own equivalent using `while` or even `loop`, but
  you should use `for` instead.  It is simpler, clearer, and less error prone.
+ You can use `for` instead of `while` if you use a range.


[pow]: https://doc.rust-lang.org/std/primitive.i32.html#method.pow
