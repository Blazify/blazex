# BlazeScript

## Installation

- **1. Download and Install [Deno](https://deno.land/#installation)**
- **2. Clone this repository by the below given command**
```console
git clone git@github.com:RoMeAh/blazescript.git
```
- **3. Go to the Directory, Bundle it and then install it globally**
```console
cd blazescript/
deno bundle src/mod.ts bzs.js
deno install -f -A bzs.js
```

## Running Code

```console
bzs --eval 1 + 1 * 5
```
or
```console
bzs filename.bzs
```
# TODO
- [x] Tokens
- [x] Parser
- [x] Interpreter
- [x] Number (Int and Floats)
- [x] Maths Calculation (Addition, Substraction, Multiplication, Division)
- [x] Binary Operators
- [x] Unary Operators
- [x] Number Node
- [ ] Variables
- [ ] Strings
- [ ] Functions
- [ ] Classes
- [ ] Objects
- [ ] Standard Library
- [ ] Global Objects
- [ ] Global Variables

## Note
**It is in its very new born form.**
**BlazeScript uses "-A" permission flag which means it has all permissions**