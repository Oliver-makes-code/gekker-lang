# Traits

This is pretty much the trait system from Rust. A trait is like an interface, except with some more features.

Trait functions are considered instance functions if their parameter list contains `this` (lowercase). You can ensure a a type is the type implementing the trait by using `This` (capital)

```
trait Default {
    func Default(): This;
}

trait Copy {
    func Copy(ref this, other: ref mut This);
}
```

## Operator Traits

We our version of operator overloads are with Operator Traits. Functions for operator traits must always be `const func`s.

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
