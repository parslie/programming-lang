# CURRENTLY UNNAMED LANGUAGE

I've always wanted to create my own programming language, but I've never really had the energy to do so. I'm going to attempt to do so now though!

## Planned Syntax

```
use models # Imports a module in the same directory
use ../routes # Imports a module in the parent directory

data DataStructure
	i32 an_integer
	f32 a_float
	bool a_boolean
	str a_string

function EmptyFunction : i32 a : bool b : str c

function NonEmptyFunction
	Print : "Hello, World!"
	EmptyFunction : 42 : true : "foobar"

function Main
    # This is the entry point of your program
	exit 0
```

## Current Syntax

```
exit 0

exit\
0
```