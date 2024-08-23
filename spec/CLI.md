# Gekker CLI

Gekker's CLI is designed to be used alongside existing C/C++ toolchains.

I understand that C toolchains suck ass, but I want gekker to be able to used alongside C codebases.

## Arguments

`-o <file>` / `--out <file>` specifies the output file. The default value is the same as the input file, but with `.gek` replaced with `.o`.
`-h <file?>` / `--header <file?>` specifies a C header generation file. A C header is not generated unless this option is specified.
`-md <file?>` / `--make-deps <file?>` will generate a make-compatible `.d` file. It will also have the generated header depend on the gekker file.
