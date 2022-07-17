# Variables
<!-- TODO: A definition of what a variable is -->

## Syntax
A variable is declared with the keyworkd `var` followed by its name, a colomn and its type:

```bird
func main()
{
	var x: int;
}
```

You can initilize it or assign it a value after it has been declared:

```bird
func main()
{
	var x: int = 10; // We initialize the variable to 10
	// Or
	var x: int; // We declare the variable
	x = 10; // Then assign it the value 10
}
```

You cannot read from a variable that has not been initialized
For instance, this code is invalid:

```bird
import bird::io::console;

func main()
{
	var x: int;
	console::output_line(x.parse<String>().unwrap()); // We are reading from 'x' but it has no value
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