# Specification

## Types and Fundamentals

### Hierarchy

Types and values have a clear type hierarchy: `[instantiated types]` -> `[defined types]` -> `[fundamentals]`.
Defined types and fundamentals are required to be evaluated at compile-time.

    41      is of type S64           and has the value 41      | no specifier
    S64     is of type pd            and has the value S64     | specifier: '
    pd      is of type fundamental   and has the value fd      | no specifier?

Function types are defined by declaring the prototype, and are instantiated by giving them a body of statements/expressions.

    fn =>.. is of type fn ->..       and has the value fn =>..
    fn ->.. is of type fn            and has the value fn ->..
    fn      is of type fundamental   and has the value fn

Structure types are defined by declaring the members, and are instantiated with a block instantiating its members.
Polymorphic structures with unspecified arguments are equal to defined functions having the specified structure as their return value.

    foo     is of type st {..}       and has the value foo
    st {..} is of type st            and has the value st {..}
    st      is of type fundamental   and has the value st

Fundamentals include plaindata `pd`, structures `st`, functions `fn`, traits `tr`, enums `en`, expressions and subexpressions.

### Literals

Basic literals can be written everywhere.

SubExpressions: `[]`
Blocks: `{}` or indentation-based
Floats: `0.0` or `0e0` or `+0.0` or `0e+0` or `+0e+0`
Integers: `0` or `+0`

Fundamental literals can only follow a fundamental specifier.

Functions: `fn _ _ '_ -> _ {}`
Structures: `st {a: X b: X}`
Traits: `tr {a: X b: X}`

### Functions and Specialization

The definition of functions expects types of which an instantiation is expected when called.

The following function expects an instantiated S64 upon being called.

    fn a: S64 -> S64 => ...

The following function is specialized for an instantiated value 5.

    fn 5 -> S64 => ...

Functions can accept variadic arguments of any type. These list types can be unpacked by recursion.

    fn nonspec... => ...
    fn nonspec: T... => ...
    fn spec: U32... => ...

### Traits

Traits sit inbetween intantiated types and defined types and act as a description of functionality, i.e. functions or members with certain names being defined for a type.
The `%` in the trait block refers to the type for which the trait is checked.

    MyTrait :: tr
        a: S32
        b: S32
        c: MyTrait+Copy|None
        d: fn % f64 -> _

Functions can specify arguments to implement certain traits.

    fn a: MyTrait+OtherTrait => ...

### Sum Types

Sum types can hold values of different types.

    SumTypeAlias :: '[A | B C | B A]
    x_of_two_faces : S64 | F64

### Enums

An enumeration is a collection of aliases for values of the same type, defaulting to `U32` and counting up from `1`.

    Enum :: en
        A
        B
        C

### Structures

Structures are structured collections of values.

Structures are polymorphic over defined types.

    Struct :: st 'TypeA 'TypeB
        i: U32
        b: TypeA
        c: TypeB

### Pointers and References

References are treated like regular types, with the exception that their lifetime is checked to not exceed the lifetime of their original. Control flow is checked to ensure only one branch exists when a reference is manipulated. References can not point to invalid memory.

    moo : U32
    ref : &moo

Functions can be specified to take reference arguments.

    fn &_ -> _

### Moving and Copying

Values are moved by default, and can be copied by preceding them with `$`. Similarly to references, this can be speficied at the point of usage as well as in function declarations.

## Control Flow

### Matching and Branching

Types can be matched with the symbol [']. The following function expects the literal type S64 or S32 as its argument.

    m :: fn a: 'S64|'S32 => ...

It would be applied by passing the literal type.

    m 'S32

Destructured/incomplete types can also be matched in function declarations. The destructured sub-values can be referenced in the function block.

    e :: fn b: [st a: S32|S64 b: [st x: F64 _]_] => ..

Branching is performed with the match statement.

    mt great_integer
        eq 100
            print "% is equal to 100" great_integer
        lt 100
            print "% is lower than 100" great_ingeter
        gt 100
            print "% is greater than 100" great_integer

Branching on sum types requires matching as well. Match statements like these are required to specify all branches, or to specify a catch-all branch `_`.

    mt x_of_two_faces
        S64
            print "% is a S64" x_of_two_faces
        F64
            print "% is a F64" x_of_two_faces

Matches can be destructured as well.

    mt complex_structure
        st a: S32 b: [st x: F32]
            print "struct contains nested % and %" a x


## Metaprogramming

### Compile-time evaluation

Requiring expressions or functions to be compile-time evaluated can be denoted by preceding them with `!`.

    build :: !fn ...
    tsun : !Time.now
    !mt arg

