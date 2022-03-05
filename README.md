<div align=center>

# Ethereal 
> General Purpose Progamming Language. Built with Rust.

</div>

## Running

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
```

4. Run
```bash
./target/debug/ethereal run <file>.etrl
# Windows: 
# .\target\debug\ethereal.exe run <file>.etrl 
```
