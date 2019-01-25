# Random Sample 1

    # import traits
    im collections

    # define structure
    MyList :: st
      # ...

    # ?
    df Iterate MyList
      next : fn => #...
      prev : fn => #...

    main :: fn
      im math
      im list

      # value declaration and assignment
      var : Option I32 ' 5
      var = None

      # lists
      list : 1 2 3 4 5
      filtered :
        filter fn e l => Boolean ' mod e 2
        5 6 7 8 9
      # or
      filtered : filter [fn e l => Boolean ' mod e 2] 5..9
      # or
      even : Boolean ' mod _ 2
      filtered: filter even 5..9

      # @note: idea: generic "eat arguments" function that takes an arbitrary number of arguments
      # and passes only required ones to passed function
      # i.e.:
      #  eat :: fn a: [fn _ _...] b... => eat [a b~] ~b
      #  eat :: fn a: [fn _] b... => a b~

      # append a dot for explicit evaluation instead of function assignment
      # filter is side-effect free so evaluation is delayed
      filteredNow : filter list '

      # catch block for guarding early returns, e.g. from the questionmark operator
      success : ct
        v : mul o? 5
        print v

      # print has side-effects (io) and is executed immediately
      print ' "success? {?}" success

      mt someStructure
        x: st
            a: mt gt 23
            b: 3 | 5
            c: "hey"
          print' "it's a struct: {?}" x
        whatever: _
          print' "it's something else: {?}" x


      # types and type variables use
      # empty instantiation / default construction
      myvar: mytype =
      # structure construction
      myvar:mytype=
        a: 1
        b: ""foo
      # constructor construction
      myvar:mytype=mytype.new 1 "foo"


	foo1 : S32 2
	foo2 : 10
	foo3 : _
	foo3 =

	at foo3
