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

## Announcment

The Bytecode Compiler and the VM are in progress, to use all features of the language use the code from the [70th Commit](https://github.com/BlazifyOrg/blazescript/commit/a2e2186bec75bc05a86ebd3192fa9d931475cb80)

## Example

Note: '@' means comments

- Printing the famous "Hello World"

```bzs
println("Hello World!") @ yep as simple as that
```

- Creating and calling functions

```bzs
fun sum(a, b) => {
    var c = a + b;
    return c;
}

println(sum(2, 2));
```

- Working around with objects

```bzs
var obj = {
    "prop": 5 @ properties should be of String type or there will be Invalid Syntax Error
}

println(obj.prop); @ accessing object property

obj.prop = 10; @ editing object property value
println(obj.prop) @ 10
```

- Classes

```bzs
class Main {
    var a = 10; @ this is a property

    @ this is constructor
    fun() {
        soul.a = 5; @ soul is the current object it's operating on
    }

    @ this is a method
    fun sum_to_a(b) => {
        soul.a = soul.a + b;
        return soul;
    }
}

var ins = new Main(); @ creating/initializing a class, returns a object with the properties
println(ins);

println(ins.sum_to_a(5));
```

## Contributing

- Please use the given below commands

```shell
$ npm i -g commitizen
$ npm i -g cz-conventional-changelog
$ echo '{ "path": "cz-conventional-changelog" }' > ~/.czrc
```

For commiting use the command `cz`. But make sure to use `git add .` before that too.

## Project Tree

```
📦src
 ┣ 📂compiler
 ┃ ┣ 📂bytecode
 ┃ ┃ ┣ 📜bytecode.rs
 ┃ ┃ ┗ 📜opcode.rs
 ┃ ┣ 📂lexer
 ┃ ┃ ┗ 📜lexer.rs
 ┃ ┣ 📂parser
 ┃ ┃ ┣ 📜nodes.rs
 ┃ ┃ ┣ 📜parser.rs
 ┃ ┃ ┗ 📜parser_result.rs
 ┃ ┗ 📜token.rs
 ┣ 📂utils
 ┃ ┣ 📜constants.rs
 ┃ ┣ 📜error.rs
 ┃ ┗ 📜position.rs
 ┣ 📂vm
 ┃ ┗ 📜vm.rs
 ┣ 📜lib.rs
 ┗ 📜main.rs
```

# TODO

- [ ] Bytecode Generator

  - [x] Statements
  - [x] Int & Floats
  - [x] Strings
  - [x] Chars
  - [x] Booleans
  - [x] Binary Operations
  - [x] Unary Operations
  - [x] Variable Assignment
  - [x] Variable Reassignment
  - [x] Variable Access
  - [x] If else
  - [x] For loop
  - [x] While loop
  - [ ] Functions
  - [ ] Calling functions
  - [ ] Arrays
  - [ ] Objects
  - [ ] Getting Object properties
  - [ ] Reassigning Object Properties
  - [ ] Classes
  - [ ] Class Initializing

- [ ] VM Interpreter

- [x] Reading from file
- [x] Tokens
- [x] Parser

## Author

- [RoMeAh (Ronit Rahaman)](https://www.romeah.me)
