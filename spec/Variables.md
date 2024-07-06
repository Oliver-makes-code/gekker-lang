# Variables

Variables are defined similarly to C, having a type, then a name, then an initializer.

There are also a couple modifiers, like `mut`, which makes the variable mutable, `static`, which makes the variable a runtime constant (initialized only ever once), and `const`, which makes the variable a compile-time constant. All of those modifiers are mutually exclusive.

```
i32 x = 1;
mut i32 y = 2;
static i32 z = 3;
const i32 w = 4;
```
