# Carina Programming Language

## Goals

- Easy to read with clean and aesthetic code
- Low-level access abilities of the C language, compatibility to C libraries
- Good error reporting
- Consistent syntax
- Statically, strongly typed
- Type inference wherever possible
- Full Introspection
- Zero-cost abstractions, easy to optimize
- Easy async code


## Influences

- Ruby: Blocks, Fibers
- Ocaml: ?
- Jai: Metaprogramming
- Caffeine: Block Syntax
- Crystal: ?
- Rust: ?
- Haskell: ?
- Swift: ?
- Odin: ?
- Pony: ?
- Pure: ?
- Koka: Side Effect Handling
- FStar: ?
- Julia: Multiple dispatch

## Ideas

- Lazy Evaluation
- RAII
- Indent Block Parsing
- Tail Calls
- Dependent Types, Algebraic Types, Type Classes
- Generators
  - @note this would enable something like "lazy evaluation of inifinite lists"?

- Ability to include the compiler in the executable to allow for run time scripting
- Reserve all 2-character identifiers for builtins
- Automatic detection of function side-effects to among other things allow lazy evaluation
  - @note This could be implemented by giving functions and variables pervasive tags like "io" and "nd"
- Automatic compile-time evaluation of all non-side-effect functions, and a keyword to enforce this for certain function calls
- Concept of "Indirections". Struct fn access, pointer dereferencing, etc. are indirections, as well as web calls or messaging.
  - @clarify How to integrate this?
- functions analogous to main that run at build time, e.g. "build"

- Construct and Drop functions for types
- Enforced Semantic Versioning like in Elm

- Functions can be polymorphic over Compile-time and Run-time application when they are side-effect-free. If side effects occur, a function defaults to Run-time only, unless specified otherwise either in the declaration or at the call-site.

- Defer args from function calls to enable arbitrary or convenient re-ordering of the arguments
  - With the Placeholder Mark?
- Named Arguments

- Capabilities type system support, see http://zsck.co/writing/capability-based-apis.html

- Values can have "multiple types" built-in.

- Time Travel Debugger with ability to jump back to point where objects where created

- Trivalent type, see https://news.ycombinator.com/item?id=17058183

- Functions without arguments are automatically evaluated at point of to-value coercion

- Every value is an array of 0 to n elements

- Division `md` checks if one if the divisor can be 0, e.g. when it has an IO flag, and returns a Maybe value in that case.
  - How is the function for this defined? Does it just return a sum type which is statically resolved if possible?
  - How is static sum type resolution supposed to go through anyway?

- Multiple dispatch

- Newtypes
  - Simple forwarding via Structs with only a single child?
    `InternalCount :: st {f64}`
  - Or via dedicated fundamental?
    `ExternalCount :: nt f64`
  - Or via constant assignment?
    `OtherCount :: f64`

- PGP encrypted / signed source code support

- Ranges and Restrictions on values, user-defined bounds behavior

- Context object

- null object / empty sink
  - allows any assignment/storage but results in pruned operations

- initialization form similar to C designated initializers

- Tail Calls with Trampolines
  - https://www.datchley.name/recursion-tail-calls-and-trampolines/
  - http://home.pipeline.com/~hbaker1/CheneyMTA.html
  - https://www.more-magic.net/posts/internals-gc.html

- Additional pipe operators for array-likes that apply the chain to all elements

- Getters/Setters for easy property to method refactoring

- No keywords except `use` - Keywords get included like functions etc.

- Allocations in the type system
  - https://www.fos.kuis.kyoto-u.ac.jp/~tanki/papers/memoryleak.pdf

- Literal values as types
  - https://news.ycombinator.com/item?id=16780068
  - https://www.cs.cmu.edu/~rwh/papers/ordered/popl.pdf


## Links

- https://hackernoon.com/considerations-for-programming-language-design-a-rebuttal-5fb7ef2fd4ba
- https://gendignoux.com/blog/2017/09/05/rust-vs-cpp-ocaml-part1.html
- http://beza1e1.tuxen.de/articles/proglang_mistakes.html
- https://graydon2.dreamwidth.org/253769.html
- https://en.wikipedia.org/wiki/Partial_evaluation
