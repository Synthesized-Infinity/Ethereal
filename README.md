<div align=center>

# Ethereal 
> General Purpose Progamming Language. Built with Rust.

</div>

## Features

- Flexibility
- Performance
- Ease of use

## Docs

General documentation | Better documentation on the works 

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





## Running Locally

1. Clone the repository

```sh
git clone https://github.com/sythesized_infinity/Ethereal.git
    
cd Ethereal
```

2. Build the project

```
cargo build
```

3. Create a file with the extension `.etrl`

Example

```
set add = fun (x, y) {
    x + y
};

put(add(1, 2));
```

4. Run
```bash
./target/debug/ethereal run <file>.etrl
# Windows: 
# .\target\debug\ethereal.exe run <file>.etrl 
```



