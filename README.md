# BlazeScript

## Installation

[Github Releases](https://github.com/BlazifyOrg/blazescript/releases)

## Installing

In Ubuntu Or MacOS

```shell
$ curl -fsSL https://raw.githubusercontent.com/BlazifyOrg/blazescript/main/install.sh | sh
```

In Windows

```shell
$ Invoke-WebRequest https://raw.githubusercontent.com/BlazifyOrg/blazescript/main/install.sh -o install.sh
$ start install.sh
```

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

## Dependencies

We don't use any external dependencies for the actual lexing, parsing, compiling or interpreting but we do use serde and bitcode for the intermediate code which is the executable and also mimalloc for allocation so that our language can be fast as possible

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

## Project Structure

- [blaze_vm](https://github.com/BlazifyOrg/blazescript/tree/main/crates/blaze_vm) This is the crate for the blaze virtual machine which interprets the bytecode
- [blazescript](https://github.com/BlazifyOrg/blazescript/tree/main/crates/blazescript) This is the crate which is the actual binary
- [bzs_shared](https://github.com/BlazifyOrg/blazescript/tree/main/crates/bzs_shared) This is the crate which contains stuff which is used in more than one crate
- [bzsc_bytecode](https://github.com/BlazifyOrg/blazescript/tree/main/crates/bzsc_bytecode) This is the crate for the actual Bytecode Generation
- [bzsc_lexer](https://github.com/BlazifyOrg/blazescript/tree/main/crates/bzsc_lexer) This is the crate which lexes the file and returns tokens
- [bzsc_parser](https://github.com/BlazifyOrg/blazescript/tree/main/crates/bzsc_parser) This is the crate which parses a sequence of token and forms a AST tree

After running "./build.sh"

```
.
├── bin
│   └── blazescript
├── build.sh
├── Cargo.lock
├── Cargo.toml
├── crates
│   ├── blazescript
│   │   ├── Cargo.toml
│   │   └── src
│   │       └── main.rs
│   ├── blaze_vm
│   │   ├── Cargo.toml
│   │   └── src
│   │       └── lib.rs
│   ├── bzsc_bytecode
│   │   ├── Cargo.toml
│   │   └── src
│   │       └── lib.rs
│   ├── bzsc_lexer
│   │   ├── Cargo.toml
│   │   └── src
│   │       └── lib.rs
│   ├── bzsc_parser
│   │   ├── Cargo.toml
│   │   └── src
│   │       └── lib.rs
│   └── bzs_shared
│       ├── Cargo.toml
│       └── src
│           └── lib.rs
├── examples
│   ├── main.bze
│   └── main.bzs
├── install.sh
├── LICENSE
└── README.md
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
  - [x] Functions
  - [x] Calling functions
  - [x] Arrays
  - [x] Objects
  - [ ] Classes
  - [ ] Proper Mutability

- [ ] VM Interpreter

- [x] Reading from file
- [x] Lexer
- [x] Parser
- [x] AST

## Author

- [RoMeAh (Ronit Rahaman)](https://www.romeah.me)
