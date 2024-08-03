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

The specifics of Operator Traits are subject to change.

```
impl operator +<Vec3, Vec3> for Vec3 {
    const func Add(lhs: Vec3, rhs: Vec3): Vec3
        => Vec3 {
            x: lhs.x + rhs.x,
            y: lhs.y + rhs.y,
            z: lhs.z + rhs.z,
        };
}
```

Here's a list of operator traits and their signatures

- `+ <TLhs, TRhs>` -> `Add(lhs: TLhs, rhs: TRhs): This`
- `- <TLhs, TRhs>` -> `Sub(lhs: TLhs, rhs: TRhs): This`
- `* <TLhs, TRhs>` -> `Mul(lhs: TLhs, rhs: TRhs): This`
- `/ <TLhs, TRhs>` -> `Div(lhs: TLhs, rhs: TRhs): This`
- `% <TLhs, TRhs>` -> `Rem(lhs: TLhs, rhs: TRhs): This`
- `> <TLhs, TRhs>` -> `Greater(lhs: TLhs, rhs: TRhs): This`
- `< <TLhs, TRhs>` -> `Less(lhs: TLhs, rhs: TRhs): This`
- `& <TLhs, TRhs>` -> `BitAnd(lhs: TLhs, rhs: TRhs): This`
- `| <TLhs, TRhs>` -> `BitOr(lhs: TLhs, rhs: TRhs): This`
- `^ <TLhs, TRhs>` -> `BitXor(lhs: TLhs, rhs: TRhs): This`
- `~ <TLhs, TRhs>` -> `BitNot(lhs: TLhs, rhs: TRhs): This`
- `>= <TLhs, TRhs>` -> `GreaterEqual(lhs: TLhs, rhs: TRhs): This`
    - Can only be implemented on `bool` type.
- `<= <TLhs, TRhs>` -> `LessEqual(lhs: TLhs, rhs: TRhs): This`
    - Can only be implemented on `bool` type.
- `== <TLhs, TRhs>` -> `Equal(lhs: TLhs, rhs: TRhs): This`
    - Can only be implemented on `bool` type.
- `!= <TLhs, TRhs>` -> `NotEqual(lhs: TLhs, rhs: TRhs): This`
    - Can only be implemented on `bool` type.
- `&& <TLhs, TRhs>` -> `BoolAnd(lhs: TLhs, rhs: TRhs): This`
    - Can only be implemented on `bool` type.
- `|| <TLhs, TRhs>` -> `BoolOr(lhs: TLhs, rhs: TRhs): This`
    - Can only be implemented on `bool` type.
- `^^ <TLhs, TRhs>` -> `BoolXor(lhs: TLhs, rhs: TRhs): This`
    - Can only be implemented on `bool` type.
- `! <TValue>` -> `BoolNot(value: TValue): This`
- `+ <TValue>` -> `UnaryPlus(value: TValue): This`
- `- <TValue>` -> `UnaryMinus(value: TValue): This`
- `=` -> `Copy(ref this, other: ref mut This)`
- `[] <TIndex, TResult>` -> `Index(ref this, index: TIndex): TResult`

Some other special-case operators

- `..`
    ```
    where
        TItem
    trait operator .. {
        const func Range(pair: Pair<TItem, TItem>): This;
        const func Next(ref mut this): ?TItem;
        const func Current(ref this): TItem;
        const func Min(ref this): TItem;
        const func Max(ref this): TItem;
    }

    // Automtaically implements
    where
        TItem,
        TRange : operator ..<TItem>
    impl Into<TRange> for Pair<TItem, TItem> {
        const func Into(this): TRange {
            return TRange.Range(this);
        }
    }
    ```
