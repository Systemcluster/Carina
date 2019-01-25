# Ideas

## General

We could build a tree of side-effects through function calls, would also be nice as a visualization.

## Function calls

When a function call is side-effect free and called with a statically known data, the computation is performed at compile-time.

When a function-call is side-effect free and called with run-time data, it might get re-ordered with other function calls if none of their arguments depend on each other.

## Syntax

Many syntax elements could be defined inside the language as part of the standard import.

fn a: Foo + Bar | Zub i64 -> None

fn a: MatchesAny (MatchesAll (Foo Bar) [Zub i64]) -> None

Grundbausteine matching:

- is value of type
- is value of trait
- is value of pattern
- is type of trait
- is type of pattern
