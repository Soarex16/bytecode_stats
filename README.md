# rusty-lama
Simple interpreter for Lama lang (https://github.com/PLTools/Lama)

# Project structure

The project consists of the following modules:

- bytecode parsing ([lama-bc](/lama-bc))
- utility for analyzing frequencies of parameterized opcodes ([lama-bc-stats](/lama-bc-stats))

# How to build

First, you need to install Rust and cargo. Then simply run `cargo build --release` inside the project directory. 
After build, the `lama-bc-stats` binary will be located in the `target` directory.
To run application use this binary and pass `.bc` file as an argument.
