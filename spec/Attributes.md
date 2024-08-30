# Attributes

Currently disabled until I can figure them out further.

Attributes are a lot like decorators in Python, annotations in Java, or
attributes in C#. They can modify the behaviour of a declarators at compile
time, check semantics of a declarator, etc.

They can be as simple as `#[OkWhen(true)]` or `#[NoneWhen(false)]`, or as
complicated as `#[Derive(Debug)]` or `#[Extern(C)]`

## The `Derive` attribute

The `Derive` attribute is used to derive traits on types, a lot like Rust.
