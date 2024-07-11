# Namespace

We have namespaces, like in C++.

```
namespace Std;

where
    T : Sized
func Malloc(): *T;
```

Namespaces are traversed with the `::` symbol.

```
let x = Std::Malloc<i32>()
```

There's also the `using` keyword

```
using Std::Random;
```

The `using` keyword would package all the symbols in the lattermost keyword.

```
using Std::Random;

func Main() {
    Random::Next<i32>();
}
```

You can also assign them to specific keywords

```
using Std::Random = Rand;
```

This also works for specific symbols in the package

```
using Std::Random::Next = NextRand;
```
