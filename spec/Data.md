# Data

## Primitives

### Integral types

For primitive number types, we follow Rust's semantics, `u` denotes an unsigned
integer, `i` denotes a signed integer, `f` denotes a float, and the following
number is the bit count. An `f32` would be a 32-bit float, a `u64` would be a 64
bit unsigned integer, etc.

We also have `bool` and `char`, where `bool` is a boolean, and `char` is a utf8
code point (32 bits, to store any utf8 scalar value). A c string would be an
array of `u8`s

### Unit and Never

There's `unit`, the top type, and `never`, the bottom type.

`unit` has one possible value, and `never` has no possible values. This is an
important distinction that we will cover later on.

`unit` can be thought of as similar to `void` in C, though, it is not 100%
analogous. You can instantiate `unit` with the `unit` keyword

### Reference Types

There's reference types, which store a reference to a variable. You can denote
that with the `ref` keyword. References are always non-null.

References can be used to give a function a variable, without passing ownership.

If you want to be able to edit the referenced variable, you can use `ref mut`,
which denotes a reference to a mutable variable

You take references to variables with `ref`, and if the type being referenced is
mutable, the reference is automatically assumed to be mutable, downcasting if
necessary.

```
let nonRefX: i32;
let x: ref i32 = &nonRefX;

let nonRefY: mut i32;
let y: ref mut i32 = ref nonRefY;
let yNonMut: ref i32 = ref nonRefY;
```

### Pointers

Pointers are a lot like references, but without a lot of the compile-time checks
that come with references. Pointers can access arbitrary places in memory,
potentially causing unsafe behaviour. As such use of pointers is discouraged.

Pointers use a C-style syntax, declaring them with the `*` symbol, and using
`->` to access members. Unlike C, the `*` symbol goes before the type name. You
use `&` to reference a variable, and use `*` as well to dereference a pointer.

References can be automatically inferred to pointers, but not vice versa.
There's also the `nullptr` keyword, which creates a pointer with a null
reference.

```
let x: i32 = 5;
let y: *i32 = &x;
let z: *i32 = nullptr;
```

If you want to access a field if a pointer, you can use the `->` operator

```
ptr->x
```

### Array types

There's arrays, which are a sequential layout of the same data type. They are
denoted by a `[]` containing the type name.

If you want to restrict the array to a certain size, you can use that by
including the size after the type, separated by a comma.

Because non-sized arrays have an unknown size, they always need to be
references.

```
let sixteenInts: [i32, 16];

let someIntArr: ref [i32] = ref sixteenInts;
```

There's also the built-in `str` type, which is internally a `[u8]`.


### Function types

There's function types, which are defined similarly to Rust function types, but
are a bit different.

Since function sizes are unknown, they need to be reference types.

```
let f: ref func(i32, i32): i32 = Multiply;
```

## User-defined

User-defined data is represented relatively simply compared to other languages.

We have Structs (product types) and Enums (sum types). That's it.

```
struct Vec2 {
    x: i32,
    y: i32
}

enum IntOrFloat {
    Int: i32,
    Float: f32
}
```

Enums can also be used for integral types. You need to explicitly declare them
as such.

```
enum SomeEnum: u32 {
    Item1,
    Item2,
    Item3 = 5
}
```

When assigning fields in a struct initializer, you can use `=`.

```
let x = Vec3 {
    .x = 1,
    .y = 2,
    .z = 3,
};
```

You can also merge the values of other instances, using the `...` symbol.

```
let x = Vec3 {
    .y = 5,
    ...other
};
```

You can also do ordered initialization, though this is discouraged in more complex structs.

```
let x = Vec3 { 1, 2, 3 };
```

We do not have tuples, because I believe them to be an antipatterm. Having
anyonymous data types with unnamed fields often leads to confusion. Use
anonymous structs instead.

There are no anonymous enums.

```
let x: struct { x: i32 };
```

They are instantiated using the `struct` keyword

```
let x: struct { x: i32 } = struct { .x = i32 };
```

If you have an enum value with an anonymous struct, you don't need the `struct` keyword.

```
enum SomeEnum {
    Value: struct {
        x: i32,
        y: f32,
    }
}

let x = SomeEnum::Value { .x: 1, .y: 2.5 };
```

## Syntax sugar for language builtin non-primitives

- `?T` -> `Option<T>`
- `..T` -> `Range<T>`
    - `Range` is a trait, so must be a reference (`ref ..i32`)

## Casting

We don't have an `as` keyword like some languages do, instead we use `:`.

This is done so you can have the following

```
thing:Type.x
```

Since you can't have a type contain a type, this is unambiguous.

```
thing:module::Type
```
