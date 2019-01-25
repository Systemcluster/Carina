# Carina Specification

General overview of Carina Alpha language features.

## Overview



## Types & Values

### Hierarchy

    Base         :  Base type system component. Not referable.
    Fundamental  :  Instantiated Base. Not referable.
    Type         :  Instantiated Fundamental.
    Instance     :  Instantiated Type.

### Instantiation

Instantiation requires the create operator.

    instance := new SomeStructure
        size: 20
        elem: otherElem
        _ # initialize the rest as default

### Unions

Exclusive unions are declared with the alternation operator. They have to be type matched before their properties can be accessed and before they can be passed to other functions, except for functions specified for each variant.

    union :: Matrix 5 5 | Vector 5 | Number

Inclusive unions are declared with the combination operator. They can be used as any of their separate components.

    union :: Number + Printable + Iterator


### Conversion

Conversion between types is explicit.


## Operators

### Compiler-Level Operators

    :: define statically immutable
    := define dynamically mutable
    :! define dynamically immutable at compile time

    :+ add to
    :- remove from

    =: replace value of mutable identifier

    .. spread a Container into its separate elements


### Language-Level Operators

    =? assume equality, evaluate to Result


## Tags

Annotations to be used for various compile-time functionalities.

### Assumptions

Specifies compilation assumptions that are checked unconditionally at compile-time.

    @assume compile-time-evaluated
    e :: fn a b c -> Result

### Definitions

Specifies compilation behavior.

    @define non-static
    u := calculateMatrix 20 30

### Aliases

Aliases for tags.

    @alias cte [assume compile-time-evaluated]
