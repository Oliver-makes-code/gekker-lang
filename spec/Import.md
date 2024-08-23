# Import

To import other files, we use a similar syntax to how C does it. That does not mean we have a C-style preprocessor, though.

```
#import "./some_file.gek";
```

Note: Importing a gekker file doesn't include it's code in compilation (unless generics are used), it's only used to provide definitions.

The import statement also supports importing C headers, for interop with C code (or C++ with a C interface)

```
#import "./some_header.h"
```

At some point, compiler plugins will exist that can handle importing of other file types.

## Import paths

Import paths are similar to unix file paths.

- `./` is the current directory.
    - If your file is in `src/some_module/file.gek`, `./other_file.gek` would become `src/some_module/other_file.gek`
    - Paths without `./`, `~/`, or `$/` are implicitly `./`
- `~/` is the project root directory, aka the current working directory of the compiler.
    - `~/src/some_module/file.gek` would become `src/some_module/file.gek`
- `$/` is "system directories", which, by default, include your OS's system C headers, and the gekker stdlib files.
    - `%/SDL2/SDL.h` would become `/usr/include/SDL2/SDL.h`
    - This can be controlled with the `GEKKER_PATH` env var.
