# BlazeScript

## Installation

- **1. Download and Install [Rust](https://www.rust-lang.org/tools/install)**
- **2. Clone this repository by the below given command**

```console
git clone git@github.com:BlazifyOrg/blazescript.git
```

- **3. Go to the Directory, Compile it**

```console
cd blazescript/
cargo install --path ./ --bin blazescript
```

# Known bugs

- Unusual behaviour in windows
- In `VarAccessNode`, regardless of what SymbolTable it came from it always sets to the nearest one

# Project Tree

- 📦src
- ┣ 📂core
- ┃ ┣ 📂interpreter
- ┃ ┃ ┣ 📜interpreter
- ┃ ┃ ┣ 📜runtime_result
- ┃ ┃ ┗ 📜value
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

# External Dependencies

- rustyline for the REPL
- structopt for Argument parsing

# TODO

- [x] **MULTI-LINE SUPPORT**
- [x] Reading from file
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
- [x] Comments
- [x] Strings
- [x] Chars
- [x] Functions
- [x] Arrays
- [x] Objects
- [x] Classes
- [ ] Standard Library

## Note

- **It is in its very new born form.**

## Author

- [RoMeAh (Ronit Rahaman)](https://github.com/RoMeAh)
