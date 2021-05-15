#!/bin/sh
if [ "$OS" = "Windows_NT" ]; then
	target="x86_64-pc-windows-msvc"
    exe="blazescript.exe"
    executable="target/$target/release/$exe"
else
	case $(uname -sm) in
	"Darwin x86_64") target="x86_64-apple-darwin" ;;
	"Darwin arm64") target="aarch64-apple-darwin" ;;
	*) target="x86_64-unknown-linux-gnu" ;;
	esac
    exe="blazescript"
    executable="target/$target/release/$exe"
fi

cargo build --locked --target $target --release
strip $executable

if [ ! -d "bin" ]; then
	mkdir -p "bin"
fi
if [ -f "bin/$exe" ]; then
    rm -r "bin/$exe"
fi

cp $executable bin/
