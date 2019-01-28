# Carina Experimantal

Take element 0 of val, if it exists, take its reference, call reuse with it and flags.Any as parameters.

    val#0?&.reuse flags.Any


Split a file at comments, create new files with comments as name and fill with content below

    lines file |> split-at re"^[ ]*[\/]{2,}([a-zA-Z0-9]*)[ !]*" |> each [fn matches content => write-file ["{_}.css" matches#0] <| join content]

Calculate infinite Fibonacci list

    fib :: fn a b => (a ..[fib b [add a b]])
    fib :: fn => fib 0 1

Check non-null

    a := x - 1
    b := y / a ##error, possibly div 0

    b := mt a
        NonNull
            y / a
        _
            panic!

    b := y / [NonZero a]?

