# Generics

Generics use a similar syntax to C#, but with a bit of C++ sprinkled in.

In C# you use the `where` keyword do constrain generics, while still using `<>`
after the name, while in C++ you use `template <>` before the declaration. I'm
meeting in the middle.

```
where
    T : operator +<T, T>
func Double(val: T): T
    => val + val;
```

This method takes in any type that has declaraed a + operator, and adds it to
itself.

If you want to compose a generic type of multiple traits, you can use the `+`
operator. Defining multiple generics uses a comma between them. If there's no
comma a declaration is required.

```
where
    T1 : Trait1 + Trait2,
    T2 : Trait3 + Trait4
func DoSomething();
```

Types can also be generic, so you can have the following

```
where TOk, TErr
enum Result {
    Ok(TOk),
    Err(TErr)
}
```

You can define implementations for specific generic types in functions. The
compiler will throw an error if more than one implementation exists for a given
type, or if there are no implementations for a given type.

```
where
    T : operator +<T, T>
func Double(val: T): T;

func Double<i32>(val: i32): i32
    => val * 2;
```

If you want to restrict a type to not being a specific type, you can use the `!`
operator. This is useful for defining wildcard implementations while not
throwing errors for specialized implementations

```
where
    T : operator +<T, T> + !i32
func Double<T>(val: T): T
    => val + val;
```
