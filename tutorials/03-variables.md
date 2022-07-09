# Variables
<!-- TODO: A definition of what a variable is -->

## Syntax
A variable is declared with the keyworkd `var` followed by its name, a colomn and its type:
```bird
func main()
{
	var my_var: int;
}
```
You can initilize it or assign it a value after it is declared:
```bird
func main()
{
	var my_var: int = 10; // We are initializing the variable to 10
	// Or
	var my_var: int; // We declare the variable
	my_var = 10; // Then assign it a value
}
```
When initialized, the type of the variable can be omitted:
```bird
func main()
{
	var my_var = 10; // We don't need to precise the type because 10 is of type int by default
}
```

## Data Types

### Scalar
| Type  | Description |
|-------|-------------|
| bool  |             |
| int   |             |
| uint  |             |
| float |             |

### Compund
| Type  | Description |
|-------|-------------|
| tuple |             |
| array |             |

## Mutability
By default variables are immutable.
For example this code won't compile:
```bird
import standard.console;

func main()
{
	var my_var = 10;
	console.output_line("The value of 'x' is: " + string.parse(my_var));
	my_var = 25;
	console.output_line("The value of 'x' is: " + string.parse(my_var));
}
```
Because the variable 'x' is immutable by default, changing its value causes an error.
To fix this problem, we just have to use the keyword `mut` after the `var` keyword like this:
```bird
import standard.console;

func main()
{
	var mut my_var = 10;
	console.output_line("The value of 'x' is: " + string.parse(my_var));
	my_var = 25;
	console.output_line("The value of 'x' is: " + string.parse(my_var));
}
```
Now the program should work fine!