# Functions

## Basics

Functions are defined using a Rust-style syntax, that being, the name comes
before the type

```
func Main(): unit {
    //...
}
```

Any function that returns `unit` does not need to specify a return value. Any
function without a return type is implicitly thought to be `unit`.

Any function that returns `never` is not allowed to return (You must infinitely
loop or terminate the program)

Any other function is required to have a return value.

If you have a function that immediately returns, you can use the `=>` symbol,
like in C#

```
func Nultiply(x: i32, y: i32): i32
    => x * y;
```

Functions that have no bodies are expected to be implemented elsewhere. This can
be beneficial for traits, generics with specific implementations, or
foreward-declaring functions (though it's not necessary like in C.)

```
func DoSomething();
```

`const` functions are pure functions. That means they aren't allowed I/O, they
aren't allowed to allocate on the heap, they aren't allowed to call non-const
functions, they aren't allowed to use pointers, they aren't allowed to access or
mutate outside state, and they aren't allowed to call foreign functions.

However, they can modify data from `ref mut`s.

```
const func DoSomething(x: i32): i32 {
    return x * 2;
}
```

## Attributes

Functions can have attributes.

```
[name]
func DoSomething()
    => unit;

[name(values...)]
func DoSomethingElse()
    => unit;
```

Read more about attributes in [Attributes](Attributes.md).

### Builtin function attributes

We have a number of builtin function attributes.

- Extern
    - Extern is used to change the semantics to follow other language's standards.
    ```
    // Looks for symbol `int32_t SDL_Init(uint32_t flags)`
    [Extern(C)]
    func SDL_Init(flags: u32): i32;

    // Looks for symbol `void glClearColor(float, float, float, float)`
    [Extern(C, symbol="glClearColor")]
    func Gl.ClearColor(r: f32, g: f32, b: f32, a: f32);

    // Creates the symbol `uint32_t GetRandomChar()`
    [Extern(C)]
    func GetRandomChar(): char {
        return Random.Next<char>();
    }

    // Creates the symbol `void SomeLib_RenderCube()`
    [Extern(C, symbol="SomeLib_RenderCube")]
    func RenderCube() {
        //...
    }

    // Creates the symbol `void Exit()`
    [Extern(C)]
    func Exit(): never {
        //...
    }
    ```
