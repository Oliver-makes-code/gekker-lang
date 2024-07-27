# Fallible

Instead of a `Result`, like in Rust, we have a `Fallible`.
This is pretty much the same as Rust's `Result`, with the distinction of including a call stack when erroring.

```
enum Fallible<T, E> {
    Ok: T,
    Err: struct {
        value: E,
        stack: CallStack
    }
}
```