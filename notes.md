# Notes

## Types and Fundamentals

### Functions

## Syntax

### Infix

The `.` infix places the preceding subexpression after the succeeding subexpression.

    ## v.f -> f v
    infix :: fn "." p: SubExpr s: SubExpr => s e

(Maybe this is wrong. The `.` infix should be a generic "dereference" operator.)
(Or is it? We don't have anything to dereference in the language except struct members.)
(Maybe the operator can be specialized for struct members, as SubExpr is a fundamental.)

    infix :: fn "." p: [st{_}] s: Identifier => [builtin_st_access p s]

The `|>` infix wraps the preceding expression into a subexpression.

    ## v.f x |> .g z |> .h -> [[v.f x].g z].h -> h [g [f v x] z]
    infix :: fn "|>" p: Expr s: Expr => [p] s

Should infixes be completely user-definable?

## Metaprogramming

### Tagging

Tags refer to compile-time functions that operate on fundamentals and blocks.

How would this look regarding syntax and implementation?
