# Carina Experimantal

Take element 0 of val, if it exists, take its reference, call reuse with it and flags.Any as parameters.

    val#0?&.reuse flags.Any


Split a file at comments, create new files with comments as name and fill with content below

    lines file |> splitAt re"^[ ]*[\/]{2,}([a-zA-Z0-9]*)[ !]*" |> each [fn matches content => writeFile ["{_}.css" matches#0] <| join content]

Calculate infinite Fibonacci list

    fib :: fn a b => (a ..[fib b [add a b]])
    fib :: fn => fib 0 1

