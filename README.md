# types-rs
Type utilities in Rust.

## Omit

Create new Struct that omit the specified fields

```rust
use types_rs::omit;

// Create a new struct `NewS` with omitted field `b`
#[omit(NewS, [b])]
struct S {
    a: i32,
    b: &str,
}
// `NewS` will only have field `a`
let _ = NewS { a: 1 };
```

When the fields dosn't specified, it will be same as the original struct

```rust
use types_rs::omit;

#[omit(NewS)]
struct S<'a> {
    a: i32,
    b: &'a str,
}
// `NewS` is same as the original struct `S`
let _ = NewS { a: 1, b: "hello" };
```

## Pick
Create new struct that pick the specified fields

```rust
use types_rs::pick;

// Create a new struct `NewS` with picked field `b`
#[pick(NewS, [b])]
struct S<'a> {
   a: i32,
   b: &'a str,
   c: f64,
}

// `NewS` will only have field `b`
let _ = NewS { b: "hello" };
```

When the fields dosn't specified, it will be empty struct

```rust
use types_rs::pick;

#[pick(NewS)]
struct S {
  a: i32,
  b: f64,
}

// `NewS` will be empty struct
let _ = NewS {};
```

## Partial

Change all fields to `Option` type

```rust
use types_rs::partial;

// Create a new struct `NewS` with all fields optional
#[partial(NewS)]
struct S<'a> {
  a: i32,
  b: &'a str,
  c: f64,
}

// `NewS` will have all fields optional
let _ = NewS { a: Some(1), b: Some("hello"), c: Some(1.5) };
```

When the field is already is `Option`, it's no effect

```rust
use types_rs::partial;

#[partial(NewS)]
struct S {
 a: i32,
 b: Option<f64>,
}

let _ = NewS { a: Some(1), b: Some(1.1) };
```
