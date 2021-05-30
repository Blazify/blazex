# Blaze

> A intermediate jit compiled language which is kinda statically typed (for now) and object oriented

## Note
> We are moving to LLVM as the VM wasn't performant, and to compile to native executabe you must link to libc.

## Installation

[Github Releases](https://github.com/BlazifyOrg/blaze/releases)

## Installing

In Ubuntu Or MacOS

```shell
$ curl -fsSL https://raw.githubusercontent.com/BlazifyOrg/blaze/main/install | bash
```

In Windows

```shell
$ Invoke-WebRequest https://raw.githubusercontent.com/BlazifyOrg/blaze/main/install -o install
$ bash install
```

Confirm Installation

```shell
$ blaze
```

```
error: The following required arguments were not provided:
    <path>

USAGE:
    blaze [FLAGS] [OPTIONS] <path>

For more information try --help
```

## Note

This language is very much work in-progress. We are also working on a [VSCode Extension](https://github.com/BlazifyOrg/blazescript-vscode) and we are also looking for collaborators

## Example

- Printing the famous "Hello World"

```bz
printf("Hello World!") @ yep as simple as that
```

- Comments

```bz
@ single line comment
@@
	multi-line comment
@@
```

- Creating and calling functions

```bz
fun sum(a: int, b: int): int {
    var c = a + b;
    return c;
}

printf(sum(2, 2));
```

- Working around with objects

```bz
var obj = {
    "prop": 5 @ properties should be of String type or there will be Invalid Syntax Error
}

println(obj.prop); @ accessing object property

obj.prop = 10; @ editing object property value
printf(obj.prop) @ 10
```

- Classes

```bz
class Main {
    var a = 10; @ this is a property

    @ this is constructor
    fun(): Main {
        soul.a = 5; @ soul is the current object it's operating on
    }

    @ this is a method
    fun sum_to_a(b): Main {
        soul.a = soul.a + b;
        return soul;
    }
}

var ins = new Main(); @ creating/initializing a class, returns a object with the properties
printf(ins);

printf(ins.sum_to_a(5));
```

## Dependencies
- inkwell (Safe LLVM Wrapper)
- codespan-reporting (Errors)
- mimalloc (Allocation for the binary)
- structopt (Argument parsing)
- notify (Look for file changes)


## Contributing

- Fork the repository
- Create a branch with the patch/feature you want
- Make Changes to the code
- Commit the code (Use the [Emoji Commit Style](https://gist.github.com/RoMeAh/29cb5008266ab14ace12ac865bfe0538)) and the message should NOT contain the word "release"
- Finally push the code and make a pull request

## Project Structure

|            Codebase             |                   Description                   |
| :-----------------------------: | :---------------------------------------------: |
|      [blaze](crates/blaze)      |                   The binary                    |
|  [bzc_lexer](crates/bzc_lexer)  |              Lexer for Tokenizing               |
| [bzc_parser](crates/bzc_parser) |               Parser for AST Tree               |
|   [bzc_llvm](crates/bzc_llvm)   |         W.I.P. LLVM IR Code Generation          |
| [bzc_shared](crates/bzc_shared) | Structs, Methods, etc Shared among other crates |

# TODO

- [ ] Bugs Fixed
- [x] Errors
  - [x] Lexer
  - [x] Parser
  - [x] LLVM 
- [ ] LLVM

  - [x] Executables
  - [x] Statements
  - [x] Int & Floats
  - [x] Strings
  - [x] Chars
  - [x] Booleans
  - [x] Binary Operations
  - [x] Unary Operations
  - [x] Variables
  - [x] If else
  - [x] For loop
  - [x] While loop
  - [x] Functions
  - [ ] Arrays
  - [ ] Objects
  - [ ] Classes
  - [ ] Proper Mutability

- [x] Reading from file
- [x] Lexer
- [x] Parser
- [ ] Type System
- [x] AST

## Author

- [Ronit "RoMeAh" Rahaman](https://www.romeah.me)
