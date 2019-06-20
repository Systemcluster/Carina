# Carina Specification

General overview of Carina Alpha language features.

## Operators

    :: define
    := evaluate and assign
    :! evaluate and assign at compile-time

    <- store in

## Concepts

### Expressions

An expression is an operation that returns a result. Expressions can be designated by `[]`.

### Blocks

A block is a collection of sequential expressions.
Blocks are designated by indentation or by `{}`.

    struct :: st A: [st B C: [st {x: i32}] {x: B}] z: i32
        y: A

The accepted expressions inside blocks are designated by the preceeding fundamental.

### Fundamentals
