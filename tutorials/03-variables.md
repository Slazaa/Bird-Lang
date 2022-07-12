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
Numbers can be separated with a an underscore `_` as a visual separator so numbers are easier to read such as `1_000_000`.
Numbers can be represented under different bases wich are:
| Base    | Example     |
|---------|-------------|
| Decimal | 42          |
| Hex     | 0xAF        |
| Octal   | 0o53        |
| Binary  | 0b0110_1110 |

### Compund
| Type  | Description |
|-------|-------------|
| tuple |             |
| array |             |