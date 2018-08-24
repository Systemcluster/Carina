# Notes

## Structure

Since types in unspecified-polymorphic functions are automatically inferred, we don't need type annotations anywhere.

However we need this for overloading based on specifics.

Match expressions are incomplete types used for matching.

Structural match expressions (note the `_` in the block):

    match_has_members :: st A B
        a: A
        b: B
        c: fn A -> B
        _

Functional match expressions (note the missing block):

    match_signature :: fn A B _... -> A

Named sum types:

    my_sum :: sm X
        Some: X
        None

Sum type match expressions:

    ...todo!

Traits:

    integer :: tr I
        add: fn I I -> I
    integer :: tr U32
        add: fn a b = builtin_add_U32

    addition_trait :: tr A
        addextra: fn A _ -> A
    addition_trait :: tr A: Integer
        addextra: fn a b
            add [add a b] a

Polymorphic structures are in essence equal to compile-time functions returning the defined structure.

    pstr :: st A B
        a: A
        b: B
    fstr :: fn A B = [st{a:a,b:b}]

Based on this we can implement monadic traits:

    monadic :: tr X
        create: fn a:X b ->
    ...todo!

## Types and Fundamentals

### Implicit Sum Types & Sum Type Resolution

An expression without a type specifier returning values of different types is evaluated to a sum type value when called.

Sum types are automatically resolved to their concrete type if it is known at compile time.


    matcher := mt i: _
        gt 100
            "{%} is huge!" i
        _
            100
    # matcher is of type: fn _ => Str|I64

    stringval := matcher 9001
    # stringval is of type Str

    intval := matcher 3
    # intval is of type I64

    sumval := matcher <| std.io.getline
    # sumval is of type Str|I64

Sum type values have to be matched before they can be used in type-specific contexts.

    mt sumval
        x :_ I64
            math.mul x 5 |> print
        x :_ Str
            print x

### Functions

## Syntax

The Placeholder `_` has a specific meaning depending on the Fundamental it's used in.

Function identifiers are evaluated if positioned at the start of a subexpression, otherwise they're parsed as a function reference.
If the call-site expects an argument matching the return type of a passed function with no arguments, the function gets evaluated instead.

    # evaluate filter with function math.even as first argument,
    # and the return value of get_my_numbers as the second

    p : filter math.even <| get_my_numbers

    # or making use of auto-evaluation:
    # p : filter math.even get_my_numbers

?? Assigning with `:=` denotes a mutable assignment, `::` denotes a constant assignment. `:!` enforces a compile-time evaluated assignment.

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

...The previous examples are probably wrong:

`|>` should work like in F# and put the previous subexpression behind the succeeding.

    math.greater x y |> math.multiply 2 |> math.greater z
    ## would be equal to
    math.greater z [math.multiply 2 [math.greater x y]]

And `<|` should put the succeeding subexpression after the preceeding.

    graphics.getstruct <| _.x |> math.multiply 2
    ## would be equal to
    math.multiply 2 [[graphics.getstruct].x]

`.` is in a different syntax category and acts as the indirection-resolving-operator, for struct and module member access.

Question: Should infixes be completely user-definable?

## Metaprogramming

### Tagging

Tags refer to compile-time functions that operate on fundamentals and blocks or lines.

How would this look regarding syntax and implementation?
-> Maybe like in Jai, by simple `@tag` annotation.

### Macros

How would these look?
