# Statements

We have a lot of statements :3

## If / Else

We have basic if/else statements

```
if thing {
    //...
} else if otherThing {
    //...
} else {
    //...
}
```

Note: There are no parenthesis around the booleans.

## Loops

We also have some loops, that being, `for`, `for in`, `while`, and `loop`.

`for` is your basic, C-style, for loop.

```
for x = 0, x < 5, x++ {
    //...
}
```

`for in` iterates over a range

```
for x in 0..5 {
    //...
}
```

`while` executes while a value is true

```
while thing {
    //...
}
```

`loop` is an infinite loop (like `while true`)

```
loop {
    //...
}
```

## Return

We have basic return statements, like you're used to

```
return;

return value;
```

We also have `return if`, to easily create guard clauses.

```
return if someBool;

return Err(SomeErr) if someBool;
```

## Break / labels / goto

We also have break labels, and goto.

```
label x;

goto x;

label y {
    // Block for the label

    // Breaks out of y
    break y;
}

label z {
    break z if someBool;

    Print(":3");
}

label a {
    label b {
        label c {
            // Breaks out of c
            break;
        }
        // Breaks out of a and b
        break a;
    }
}
```

You can also have the label yield a value.
```
let x = label x {
    break x 15;
};

return label y {
    break y 2;
};
```

## Match

There's a couple forms of pattern matching.

First of all, there's the base `match` statement, which acts like a switch statement

```
match thing {
    1 | 2 => {
        Print("yippee");
    }
    _ => {
        Print("Awww");
    }
}
```

Note, the `_` (discard) keyword acts as the "else" case. 
You can also use an identifier to assign it to a variable. For example

```
match thing {
    Ok(1 | 2) => Print("yippee"),
    Ok(value) => Print(value),
    Err(_) => Print("Error!"),
}
```

There's also  `if let match` and `let match else`

```
// If let match

if let match Err(thing) => value {
    Print("Error! {}", thing);
    return;
}

// Let match else

let match Ok(thing) => value else {
    Print("Error!");
    return;
}
Print("{}", thing);
```

`return if` also works with this

```
return err if let match Err(err) => value;
```
