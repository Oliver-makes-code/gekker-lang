# Functions

## Basics

Functions are defined using a C-style syntax, that being, the type comes before the name.

```
unit Main() {
    //...
}
```

Any function that returns `unit` does not need to specify a return value.

Any function that returns `never` is not allowed to return (You must infinitely loop or terminate the program)

Any other function is required to have a return value.

If you have a function that immediately returns, you can use the `=>` symbol, like in C#

```
i32 Nultiply(i32 x, i32 y)
    => x * y;
```

Functions that have no bodies are expected to be implemented elsewhere.
This can be beneficial for traits, generics with specific implementations, or foreward-declaring functions (though it's not necessary like in C.)

```
unit DoSomething();
```

## Attributes

Functions can have attributes.

```
[name]
unit DoSomething() => unit;

[name(values...)]
unit DoSomethingElse() => unit;
```

Read more about attributes in [Attributes](Attributes.md).

### Builtin function attributes

We have a number of builtin function attributes.

- Extern
    - Extern is used to change the semantics to follow other language's standards.
    ```
    // Looks for symbol `int32_t SDL_Init(uint32_t flags)`
    [Extern(C)]
    i32 SDL_Init(u32 flags);

    // Looks for symbol `void glClearColor(float, float, float, float)`
    [Extern(C, symbol="glClearColor")]
    unit Gl.ClearColor(f32 r, f32 g, f32 b, f32 a);

    // Creates the symbol `uint32_t GetRandomChar()`
    [Extern(C)]
    char GetRandomChar() {
        return Random.Next<char>();
    }

    // Creates the symbol `void SomeLib_RenderCube()`
    [Extern(C, symbol="SomeLib_RenderCube")]
    unit RenderCube() {
        //...
    }
    ```
