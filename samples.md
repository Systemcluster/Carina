# Samples

Samples are currently experimental and may be non-representative.

## Create a number array, map it to a string array, and print it

    main :: fn
        v : create Array 1 2 3 4 |> map toString |> create Array
        print "Hello, world! %?" v

