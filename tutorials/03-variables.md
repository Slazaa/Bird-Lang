# Variables
<!-- TODO: A definition of what a variable is -->

## Syntax
A variable is declared with the keyworkd `var` followed by its name, a colomn and its type:

```bird
func main()
{
	var x: int
}
```

You can initilize it or assign it a value after it has been declared:

```bird
func main()
{
	var x: int = 10 # We initialize the variable to 10
	# Or
	var x: int # We declare the variable
	x = 10 # Then assign it the value 10
}
```

Note that you cannot read from a variable that has not been asigned a value.
For instance, this code is invalid:

```bird
import bird::io::Console

func main()
{
	var x: int
	Console.output_line(x.to_str()) # We are reading from 'x', but it has no value
}
```

## Constants
Constants are variable with a value that cannot be changed.
They are written as followed:

```bird
func main()
{
	const my_const: int = 10
}
```

So the following code is invalid:

```bird
func main()
{
	const my_const: int = 10
	my_const = 25 # We try to change the value of a constant, wich is not allowed
}
```