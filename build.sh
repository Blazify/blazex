#!/bin/sh
cargo build --release

if [ "$OS" = "Windows_NT" ]; then
    target = "./target/release/blazescript.exe"
else
    target = "./target/release/blazescript"
fi
strip $target