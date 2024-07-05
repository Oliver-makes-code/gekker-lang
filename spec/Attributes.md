# Attributes

Attributes are a lot like decorators in Python, annotations in Java, or attributes in C#.
They can modify the behaviour of a declarators at compile time, check semantics of a declarator, etc.

They can be as simple as `[OkWhen(true)]` or `[NoneWhen(false)]`, or as complicated as `[Derive(Debug)]` or `[Extern(C)]`
