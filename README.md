# BlazeScript

> A intermediate compiled and vm interpreted language which is dynamically typed (for now) and object oriented

## Note
> We are moving to LLVM as the VM wasn't performant, to build the code with llvm support use --features llvm and to compile to llvm use -l or --llvm flag

## Installation

[Github Releases](https://github.com/BlazifyOrg/blazescript/releases)

## Installing

In Ubuntu Or MacOS

```shell
$ curl -fsSL https://raw.githubusercontent.com/BlazifyOrg/blazescript/main/install | bash
```

In Windows

```shell
$ Invoke-WebRequest https://raw.githubusercontent.com/BlazifyOrg/blazescript/main/install -o install
$ bash install
```

Confirm Installation

```shell
$ blazescript
```

```
error: The following required arguments were not provided:
    <path>

USAGE:
    blazescript [FLAGS] [OPTIONS] <path>

For more information try --help
```

## Note

This language is very much work in-progress. We are also working on a [VSCode Extension](https://github.com/BlazifyOrg/blazescript-vscode) and we are also looking for collaborators

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

We don't use any external dependencies for the actual lexing, parsing, compiling or interpreting but we do use serde and bitcode for the intermediate code which is the executable and also mimalloc for allocation so that our language can be fast as possible and codespan-reporting for errors, structopt for argument parsing and notify for watching files. Note the only branch which use dependencies are `blazescript` and `bzs_shared`

## Contributing

- Fork the repository
- Create a branch with the patch/feature you want
- Make Changes to the code
- Commit the code (Use the [Emoji Commit Style](https://gist.github.com/RoMeAh/29cb5008266ab14ace12ac865bfe0538)) and the message should NOT contain the word "release"
- Finally push the code and make a pull request

## Project Structure

|               Codebase                |                   Description                   |
| :-----------------------------------: | :---------------------------------------------: |
|   [blazescript](crates/blazescript)   |                   The binary                    |
|    [bzsc_lexer](crates/bzsc_lexer)    |              Lexer for Tokenizing               |
|   [bzsc_parser](crates/bzsc_parser)   |               Parser for AST Tree               |
| [bzsc_bytecode](crates/bzsc_bytecode) |               Bytecode Generator                |
|      [blaze_vm](crates/blaze_vm)      |            The bytecode interpreter             |
|     [bzsc_llvm](crates/blaze_vm)      |         W.I.P. LLVM IR Code Generation          |
|    [bzs_shared](crates/bzs_shared)    | Structs, Methods, etc Shared among other crates |

# TODO

- [ ] Bugs Fixed
- [x] LLVM

  - [x] Executables
  - [x] Statements
  - [x] Int & Floats
  - [x] Strings
  - [x] Chars
  - [x] Booleans
  - [x] Binary Operations
  - [x] Unary Operations
  - [x] Variables
  - [ ] If else
  - [x] For loop
  - [ ] While loop
  - [ ] Functions
  - [ ] Arrays
  - [ ] Objects
  - [ ] Classes
  - [ ] Proper Mutability

- [x] Reading from file
- [x] Lexer
- [x] Parser
- [x] AST

## Author

- [RoMeAh (Ronit Rahaman)](https://www.romeah.me)
