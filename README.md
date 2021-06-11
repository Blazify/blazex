# BlazeX

[![Join the chat at https://gitter.im/BlazifyOrg-blazex/community](https://badges.gitter.im/BlazifyOrg-blazex/community.svg)](https://gitter.im/BlazifyOrg-blazex/community?utm_source=badge&utm_medium=badge&utm_campaign=pr-badge&utm_content=badge)

A intermediate AOT compiled language which is kinda statically typed (wanna move to gradual typing) and object oriented

## Installing

In Ubuntu Or MacOS

```shell
$ curl -fsSL https://raw.githubusercontent.com/BlazifyOrg/blazex/main/install | bash
```

In Windows

```shell
$ Invoke-WebRequest https://raw.githubusercontent.com/BlazifyOrg/blazex/main/install -o install
$ bash install
```

Confirm Installation

```shell
$ blazex
```

```
error: The following required arguments were not provided:
    <path>

USAGE:
    blazex [FLAGS] [OPTIONS] <path>

For more information try --help
```

## Note

This language is very much work in-progress. We are also working on a [VSCode Extension](https://github.com/BlazifyOrg/blazexscript-vscode) and i am also looking for collaborators

## Example

- Printing the famous "Hello World"

```bzx
printf("Hello World!") @ yep as simple as that
```

- Comments

```bzx
@ single line comment
@@
	multi-line comment
@@
```

- Creating and calling functions

```bzx
fun sum(a: int, b: int): int {
    var c = a + b;
    return c;
}

printf(sum(2, 2));
```

- Working around with objects

```bzx
var obj = {
    prop: 5 @ properties should be Identifier or there will be Invalid Syntax Error
}

println(obj.prop); @ accessing object property

obj.prop = 10; @ editing object property value
printf(obj.prop) @ 10
```

- Classes

```bzx
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
- inkwell (Safe LLVM Wrapper) (Forked in crates/bzxc_llvm_wrapper)
- codespan-reporting (Errors)
- mimalloc (Memory allocation)
- structopt (Argument parsing)
- notify (Look for file changes)


## Contributing

- Fork the repository
- Create a branch with the patch/feature you want
- Make Changes to the code
- Commit the code (Use the [Emoji Commit Style](https://gist.github.com/RoMeAh/29cb5008266ab14ace12ac865bfe0538)) and the message should NOT contain the word "release"
- Finally push the code and make a pull request

## Project Structure

|                     Crate                     |           Description            |
| :-------------------------------------------: | :------------------------------: |
|            [blazex](crates/blazex)            |            The binary            |
|        [bzxc_lexer](crates/bzxc_lexer)        |       Lexer for Tokenizing       |
|       [bzxc_parser](crates/bzxc_parser)       |       Parser for AST Tree        |
| [bzxc_type_checker](crates/bzxc_type_checker) |    Typer checker for AST Tree    |
|         [bzxc_llvm](crates/bzxc_llvm)         |  W.I.P. LLVM IR Code Generation  |
| [bzxc_llvm_wrapper](crates/bzxc_llvm_wrapper) |         Fork of Inkwell          |
|       [bzxc_shared](crates/bzxc_shared)       | Things Shared among other crates |

## Need Help with
- Gradual type system (Hybrid Dynamic & Static Typing)
- Implementing return-ing in functions
- Class Definition and Initializing LLVM code generation

## TODO

- [ ] Bugs Fixed
- [ ] Type System (gradual typing)
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
  - [x] Arrays
  - [x] Objects
  - [ ] Classes
- [x] Errors
  - [x] Lexer
  - [x] Parser
  - [x] LLVM 
- [x] Reading from file
- [x] Lexer
- [x] Parser
- [x] AST

## Author

- [Ronit "RoMeAh" Rahaman](https://blazify.rocks/team/)
