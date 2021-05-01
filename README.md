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

- [In Methods, you can't access the class properties](https://github.com/BlazifyOrg/blazescript/issues/2)

# Project Tree

```
📦src
 ┣ 📂core
 ┃ ┣ 📂interpreter
 ┃ ┃ ┣ 📜interpreter.rs
 ┃ ┃ ┣ 📜runtime_result.rs
 ┃ ┃ ┗ 📜value.rs
 ┃ ┣ 📂lexer
 ┃ ┃ ┣ 📜lexer.rs
 ┃ ┃ ┣ 📜lexer_method_result.rs
 ┃ ┃ ┗ 📜lexer_result.rs
 ┃ ┣ 📂parser
 ┃ ┃ ┣ 📜nodes.rs
 ┃ ┃ ┣ 📜parser.rs
 ┃ ┃ ┗ 📜parser_result.rs
 ┃ ┗ 📜token.rs
 ┣ 📂std
 ┃ ┗ 📜lib.rs
 ┣ 📂utils
 ┃ ┣ 📜constants.rs
 ┃ ┣ 📜context.rs
 ┃ ┣ 📜error.rs
 ┃ ┣ 📜position.rs
 ┃ ┗ 📜symbol.rs
 ┣ 📜lib.rs
 ┗ 📜main.rs
```

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
- [x] Standard Library (A basic one)

## Note

- **It is in its very new born form.**

## Author

- [RoMeAh (Ronit Rahaman)](https://github.com/RoMeAh)
