# Gekker

This is a programming language built specifically towards my needs. It is (by definition)
extremely opinionated.

## Look at the [Specification](spec/)!

## Core Principles

- The language should be unambiguous, for both the reader and compiler.
    Syntax should feel close to "plain english"
- Language features should encourage the way in which I program
- The parser should run with as little lookahead as possible. The syntax should be without context wherever possible.

## Parse Tree Todo

- Finish implementing statements
    - loop, while, for, label, goto, break
- Array intializer expressions
- Parse operator traits in types

## Semantic Analysis Todo

- Attribute expansion
    - Symbol resolution
- Only keeps track of what symbols were created by the file, no other info
- Type resolution
    - Checks struct dependencies and generics
- Top level variable and function type expansion
- TBD
