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

## Note
**BlazeScript uses "-A" permission flag which means it has all permissions**