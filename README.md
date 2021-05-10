# BlazeScript

## Installation

[Github Releases](https://github.com/BlazifyOrg/blazescript/releases)

## Note

This language is very much work in-progress. We are also working on a [VSCode Extension](https://github.com/BlazifyOrg/blazescript-vscode) and we are also looking for collaborators

## Announcment

The Bytecode Compiler and the VM are in progress, to use all features of the language use the code from the [70th Commit](https://github.com/BlazifyOrg/blazescript/tree/a2e2186bec75bc05a86ebd3192fa9d931475cb80)

## Example

- Printing the famous "Hello World"

```bzs
println("Hello World!") @ yep as simple as that
```

- Comments

```bzs
@ single line comment
@@
	multi-line comment
@@
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
$ npm root -g
# use the output of the above command instead of PATH
# edit PATH/commitizen/node_modules/conventional-commit-types/index.json
# use https://gist.github.com/RoMeAh/29cb5008266ab14ace12ac865bfe0538
```

For commiting use the command `cz`. But make sure to use `git add .` before that too.

## Project Tree

```
📦src
 ┣ 📂blazevm
 ┃ ┗ 📜vm.rs
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
 ┣ 📜lib.rs
 ┗ 📜main.rs
```

# TODO

- [ ] Bytecode Compiler

  - [x] Executables
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
- [x] Lexer
- [x] Parser

## Author

- [RoMeAh (Ronit Rahaman)](https://www.romeah.me)
