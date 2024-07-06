# Data

## Primitives

### Integral types

For primitive number types, we follow Rust's semantics, `u` denotes an unsigned integer, `i` denotes a signed integer, `f` denotes a float, and the following number is the bit count.
An `f32` would be a 32-bit float, a `u128` would be a 128 bit unsigned integer, etc.

We also have `bool` and `char`, where `bool` is a boolean, and `char` is a utf8 code point (32 bits, to store any utf8 scalar value). A c string would be an array of `u8`s

### Unit and Never

There's `unit`, the top type, and `never`, the bottom type. 

`unit` has one possible value, and `never` has no possible values.  This is an important distinction that we will cover later on.

`unit` can be thought of as similar to `void` in C, though, it is not 100% analogous. You can instantiate `unit` with the `unit` keyword

### Reference Types

There's reference types, which store a reference to a variable. You can denote that with the `ref` keyword. References are always non-null.

References can be used to give a function a variable, without passing ownership.

If you want to be able to edit the referenced variable, you can use `ref mut`, which denotes a reference to a mutable variable

You take references to variables with `ref`, and if the type being referenced is mutable, the reference is automatically assumed to be mutable, downcasting if necessary.

```
i32 nonRefX
ref i32 x = &nonRefX;

mut i32 nonRefY;
ref mut i32 y = ref nonRefY;
ref i32 yNonMut = ref nonRefY;
```

### Pointers

Pointers are a lot like references, but without a lot of the compile-time checks that come with references. Pointers can access arbitrary places in memory, potentially causing unsafe behaviour. As such use of pointers is discouraged.

Pointers use a C-style syntax, declaring them with the `*` symbol, and using `->` to access members. Unlike C, the `*` symbol goes before the type name. You use `&` to reference a variable, and use `*` as well to dereference a pointer.

References can be automatically inferred to pointers, but not vice versa. There's also the `nullptr` keyword, which creates a pointer with a null reference.

```
i32 x = 5;
*i32 y = &x;
*i32 z = nullptr;
```

### Array types

There's arrays, which are a sequential layout of the same data type.
They are denoted by a `[]` containing the type name.

If you want to restrict the array to a certain size, you can use that by including the size after the type, separated by a comma.

Because non-sized arrays have an unknown size, they always need to be references.

```
[i32, 16] sixteenInts;

ref [i32] someIntArr = ref sixteenInts;
```

### Function types

There's function types, which are defined similarly (yet slightly differently) to C function types.

C function types use the syntax `ReturnType (*name)(parameters)`, while we use `ReturnType(parameters)`.

Since function sizes are unknown, they need to be reference types.

```
ref int(int, int) f = Multiply; 
```

## User-defined

User-defined data is represented relatively simply compared to other languages.

We have Structs (product types) and Enums (sum types). That's it.

```
struct Vec2 {
    i32 x,
    i32 y
}

enum IntOrFloat {
    Int = i32,
    Float = f32
}
```

Enums can also be used for integral types

```
enum u32 SomeEnum {
    Item1,
    Item2,
    Item3 = 5
}
```

We do not have tuples, because I believe them to be an antipatterm. Having anyonymous data types with unnamed fields often leads to confusion. Use anonymous structs instead.
