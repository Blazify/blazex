# [BlazeX](https://blazify.tech)

- [Discord](https://discord.gg/9bnpjqY)
- [Gitter](https://gitter.im/BlazifyOrg-blazex/community)
- [Blazify](https://blazify.tech)

An object orineted AOT compiled language which is gradual typed

## Installing

```shell
$ git clone https://github.com/BlazifyOrg/blazex.git
$ cd blazex
$ make
$ cd ..
# Set $HOME/.blazex to PATH
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

## Example

- Printing the famous "Hello World"

```bzx
extern variadic fun printf(string): int;
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
fun sum(a, b) {
    var c = a + b;
    return c;
}

println("%i", sum(2, 2));
```

- Working around with objects

```bzx
var obj = {
    prop: 5 @ properties should be Identifier or there will be Invalid Syntax Error
}

println("%i", obj.prop); @ accessing object property

obj.prop = 10; @ editing object property value
println("%i", obj.prop) @ 10
```

- Classes

```bzx
class Main {
    var a = 10; @ this is a property

    @ this is constructor
    fun() {
        soul.a = 5; @ soul is the current object it's operating on
     }

    @ this is a method
    fun sum_to_a(b) {
        soul.a = soul.a + b;
        return soul.a;
    }
}

var ins = new Main(); @ creating/initializing a class, returns a object with the properties

println("%i", ins.sum_to_a(5));
```

## Dependencies

- llvm_sys (Interacting with LLVM)
- codespan-reporting (Errors)
- mimalloc (Memory allocation)
- structopt (Argument parsing)
- notify (Look for file changes)

## Contributing

- Fork the repository
- Create a branch with the patch/feature you want
- Make Changes to the code
- Commit the code (Use the [Emoji Commit Style](https://github.com/BlazifyOrg/pretty-commits)) and the message should **
  NOT** contain the word "release"
- Finally, push the code and make a pull request

## Project Structure

|                     Crate                     |        Description         |
|:---------------------------------------------:|:--------------------------:|
|            [blazex](crates/blazex)            |         The binary         |
|        [bzxc_lexer](crates/bzxc_lexer)        |    Lexer for Tokenizing    |
|       [bzxc_parser](crates/bzxc_parser)       |    Parser for AST Tree     |
|  [bzxc_type_system](crates/bzxc_type_system)  |        Type System         |
|         [bzxc_llvm](crates/bzxc_llvm)         |  LLVM IR Code Generation   |
|       [bzxc_shared](crates/bzxc_shared)       | Things Shared among crates |

## TODO

- [x] Type System
- [x] LLVM
- [x] Errors
    - [x] Lexer
    - [x] Parser
    - [x] Type System
    - [x] LLVM
- [x] Reading from file
- [x] Lexer
- [x] Parser
- [x] AST

## Author

- [Ronit "RoMeAh" Rahaman](https://blazify.tech/team/)
