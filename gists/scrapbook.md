# Carina Scrapbook

## tuple expressions

	a : Tuple (Tuple i32) = tu
		1 2 3
		4 5 6
		7 8 9
	a :=
		[
			[1 2 3]
			[4 5 6]
			[7 8 9]
		]


## structure expressions

	Structure :: st
		type: 'Number|'String = i32
		x: type
		y: type


## enum expressions

	Truth :: en
		False : 0u8
		True  : 1u8

	b := eq x y |> _.when (=>println "equal!") |> _.else (=>println "not equal!")
	b := (eq x y) .when (=>println "equal") .else(=>println "not equal")
	b := when (eq x y) => println "equal" |> else _ => println "not equal"
	## True|False -> Result(None) -> None|None


## functions

fn a: i32 b: fn x y; c: 'x -> 'y


## assignment

[a b] = [b a]
a, b  = b, a


## match-and-assign statement,
## like https://www.openmymind.net/Elixirs-With-Statement/
## ?
