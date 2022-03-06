# Ethereal 

> General Purpose Programming Language. Built with Rust.

### Keywords

    * `include` - Import an external file
    * `set` - Set a variable
    * `anew` - Re-define a variable
    * `if/else` - Conditional statements
    * `fun` - Function definition
    * `return` - Return from a function

### Syntax

> Include

```etrl
include "file.etrl"
```

> Set & Anew
```etrl
set name = "Ethereal"
anew name = "Ethereal" + " " + "Programming" + " " + "Language"
```

> If & Else
```etrl
set number = 1
if number == 1 {
    put("Number is 1")
} else {
    put("Number is not 1")
}
```

> Fun
```etrl
set add = fun(x, y) {
    x + y
}
add(1, 2)
```

> Return (optional)
```etrl
set sub = fun(x, y) {
    x - y
}
return add(1, 2)
```

## Standard Library
> Ethereal has a built-in library of functions.

1. `std:utils` - Utilities
2. `std:array` - Array functions
3. `std:fs` - File system functions



