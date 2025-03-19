# Rust Basics

## Variable Declaration

```rust
let x = 5;
```
You could not change the amount of Rust variables and by default they act like constants in other languages and are **"immutable"** but you can actually make them variable by adding the keyword **"mut"** *(stands for mutable)* to it:

```rust
let mut x = 5;
x = 6;
```

but what if you want to do it without the **mut** keyword? well you do it without the mut keyword and that is called **"Shadowing"** in Rust.

## Shadowing

The diffrence between *shadowing* and the *mut* keyword is that in *shadowing* you have to declare your variable again and again.
```rust
let x = 5;
let x = x + 2;
let x = x - 1;
let x = x * 5;
```

Shadowing helps with memory efficiency by removing the old value of the variables:

```rust
let spaces = "   ";
let spaces = spaces.len();
```

In this way we won't need to declare another variable, but we can't do it as follows:

```rust
// THIS CODE DOES NOT COMPILE
let mut spaces = "   ";
spaces = spaces.len();
```

## Data Types

Every type in Rust is a **Scalar**(numerical) type or a **Compound** type.

### Scalar Types

#### Integers

|Length  |Signed   |Unsigned     |
|--------|---------|-------------|
|8-bit   |i8       |u8           |
|16-bit  |i16      |u16          |
|32-bit  |i32      |u32          |
|64-bit  |i64      |u64          |
|**arch**|**isize**|**usize**    |

`isize` and `usize` choose the size based on your CPU architecture.

`i32` is the default for integer.

#### Floating Points
Floating points are the same (f32, f64, ...)
`f32` is the default for float.

*Operations are the same as other languages so I won't say anything go on and read* **the book** *for yourselves;)*

#### Booleans
```rust
let t = true;
let f: bool = false; //with explicit type annotation.
```

#### Characters

- Every **character** is 4 bytes in Rust.
- Every **character** is UTF-8 encoded in Rust.

### Compound Types

#### Tuples

Tuples can show different values in one group.
```rust
let tup: (i32, u8, f64) = (500, 1, 6.5);
```
##### Accessing Values in a Tuple
 You can access values in a tuple by asigning them to variables.

```rust
let tup: (i32, u8, f64) = (500, 1, 6.5);
let (x, y, z) = tup; //x=500, y=1, z=6.5
```

Or you can use dots to do access that like a property

```rust
let tup: (i32, u8, f64) = (500, 1, 6.5);
let x = tup.0;
let y = tup.1;
let z = tup.2;
```

#### Arrays

It works like tuples, but every element of an array should have one type. Rust arrays have constant length.

```rust
let months = ["January", "February", "March"];
```

```rust
let a = [1, 2, 3, 4, 5, 6];

let first = a[0]; // 1
let second = a[1]; // 2
```

You can also use things like `a[n-1]` or `a[i]` too.

```rust
let i = 2;
let a = [2, 5, 4];
println!("The last element is {}", a[i]);
```
