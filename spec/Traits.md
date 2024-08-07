# Traits

This is pretty much the trait system from Rust. A trait is like an interface,
except with some more features.

Trait functions are considered instance functions if their parameter list
contains `this` (lowercase). You can ensure a a type is the type implementing
the trait by using `This` (capital)

```
trait Default {
    func Default(): This;
}

trait Copy {
    func Copy(ref this, other: ref mut This);
}
```

## Operator Traits

We our version of operator overloads are with Operator Traits. Functions for
operator traits must always be `const func`s.

The specifics of operator traits are subject to change.

Operator traits use underscores to determine the semantics of the operation. For example, `_+_` is addition, `~_` is bitwise not, `_!` is none/error cascading, etc.

Here's some operator traits and their signatures

- `_+_<T>` -> `Add(this, other: T): This;`
- `_-_<T>` -> `Sub(this, other: T): This;`
- `_*_<T>` -> `Mul(this, other: T): This;`
- `_/_<T>` -> `Div(this, other: T): This;`
- `_%_<T>` -> `Rem(this, other: T): This;`
- `!_` -> `BoolNot(this): bool;`

And some special-case operator traits

```
where TOk; TErr;
trait operator _! {
    const func IsOk(this): bool;
    const func UnwrapOk(this): TOk;
    const func UnwrapErr(this): TErr;
}

// Used like

let _ = thing!;

// Turns into
if !thing.IsOk() {
    return Err(thing.UnwrapErr());
}
let _ = thing.UnwrapOk();
```
