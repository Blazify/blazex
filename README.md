# BlazeScript

## Installation

- **1. Download and Install [Rust](https://www.rust-lang.org/tools/install)**
- **2. Clone this repository by the below given command**

```console
git clone git@github.com:RoMeAh/blazescript.git
```

- **3. Go to the Directory, Compile it**

```console
cd blazescript/
cargo install --path ./ --bin blazescript
```

# Project Tree

- 📦src
- ┣ 📂core
- ┃ ┣ 📂interpreter
- ┃ ┃ ┣ 📜interpreter
- ┃ ┃ ┣ 📜runtime_result
- ┃ ┃ ┗ 📜type
- ┃ ┣ 📂lexer
- ┃ ┃ ┣ 📜lexer
- ┃ ┃ ┣ 📜lexer_method_result
- ┃ ┃ ┗ 📜lexer_result
- ┃ ┣ 📂parser
- ┃ ┃ ┣ 📜nodes
- ┃ ┃ ┣ 📜parser
- ┃ ┃ ┗ 📜parser_result
- ┃ ┗ 📜token
- ┣ 📂utils
- ┃ ┣ 📜constants
- ┃ ┣ 📜context
- ┃ ┣ 📜error
- ┃ ┣ 📜position
- ┃ ┣ 📜symbol
- ┃ ┗ 📜symbol_table
- ┣ 📜lib
- ┗ 📜main

# Dependencies

- Rustyline for the REPL

# TODO

- [ ] **MULTI-LINE SUPPORT**
- [x] Tokens
- [x] Parser
- [x] Interpreter
- [x] Number (Int and Floats)
- [x] Maths Calculation (Addition, Subtraction, Multiplication, Division)
- [x] Binary Operators
- [x] Unary Operators
- [x] Variables
- [x] Comparisons
- [x] If-Else Statements
- [x] For, When Loops
- [x] Strings
- [x] Chars
- [x] Functions
- [ ] Classes
- [ ] Objects
- [ ] Standard Library

## Note

- **It is in its very new born form.**

## Author

- [RoMeAh (Ronit Rahaman)](https://github.com/RoMeAh)
