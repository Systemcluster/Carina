# Reasons for design and implementation choices

## Eager parsing instead of Lazy parsing

While lazy parsing would cut down on syntax in case of function chaining, it would come with the disadvantage being unclear to read, as well as definition dependance. For instance adding or removing an argument from a function declaration would drastically change the meaning of every line in which it is called (see Example 1).

Eager parsing also allows for usage of function names as parameters without additional syntax overhead, simplifying function composition.

### Example 1:

Starting with two simple functions:

    add :: fn a b => a + b
    sub :: fn a b => a - b

A lazy parsing implementation of calling add with the result of a call to sub could look like the following:

    num : add sub 1 1 1

In the above case, add would be called with the result of [sub 1 1] and 1, resulting in value 1 being stored in num.
If the definition of sub would subsequently be modified to expect 3 arguments, num would store an incomplete function call expecting a number parameter, the second argument to add. This would result in inconsistent and possibly surprising error messages, delegating the point of failure downwards the point of use.

With eager parsing, the above code would call add with the function sub as its first parameter regardless its definition, so for the expected result the inner call has to be wrapped in a subexpression:

    num : add [sub 1 1] 1

Modifying the parameter list of sub would now result in add being called with an incomplete function call as the first argument, narrowing down the point of failure by delegating it upwards the point of implementation, or declaration in case of explcit argument type restrictions. In this case, the error would immediately show as the addition operator isn't implemented for incomplete function calls.
