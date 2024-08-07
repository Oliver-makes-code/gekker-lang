# Variables

Variables are defined similarly to Rust, having a variable keyword, a name, a
type, then an initializer.

There are also a couple veriants, like `let`, which makes the variable constant
after initialization, but not constant between function calls, `mut`, which
makes the variable mutable, `static`, which makes the variable a runtime
constant (initialized only ever once), and `const`, which makes the variable a
compile-time constant. All of those modifiers are mutually exclusive.

Only `mut` variables can be made into `ref mut`s.

```
let x: i32 = 1;
mut y: i32 = 2;
static z: i32 = 3;
const w: i32 = 4;
```

# None coalescing and cascading

```
let opt = GetOpt();

opt?.thing; // thing's type is wrapped in Option
opt!.thing; // None is returned
```

You can also coalesce and cascare pointers into references.
```
let value: *i32 = Malloc<i32>();
let _: ?ref i32 = ptr?;
let _: ref i32 = ptr!; // Will return None if ptr is nullptr.
```
