# Advent of code 2024

I originally wrote this code in Rust as a challange to learn a new language. Day 1 was my first time using Rust.

I'm currently working on writing python and c++ versions for some of the solutions.

All these configuration are for MAC/UNIX.

## Running the projects

### Rust
Cargo will need to be install to run Rust.
https://doc.rust-lang.org/cargo/getting-started/installation.html

Rust can be run by using these commands
```
cargo build
cargo run
```
or the release version, which will run faster
```
cargo build --release
cargo run
```

### Python
You'll need to instal python and also may need to create a virtual enviroment and install the requirements.txt

Once that's done, it can be run from and IDE or temrinal using these commands.

activate the venv
```
source .venv/bin/activate
```

run main

```
python3 -m main
```

deactive the venv when finished
```
deactivate
```

### Cpp
You will need cmake to run this project.

Configure the project
```
cmake -S . -B build
```
or if using Xcode
```
cmake -S . -B build -G "Xcode"
```

Then build the project
```
cmake --build build --parallel
```

then run the exe
```
./build/Debug/AdventOfCode2024 
```
