#!/bin/sh
set -e

bzs_install="${BZS_INSTALL:-$HOME/.bzs}"
bin_dir="$bzs_install/bin"

if [ ! -d "$bin_dir" ]; then
	mkdir -p "$bin_dir"
fi

cd $bin_dir

if [ "$OS" = "Windows_NT" ]; then
	target="x86_64-pc-windows-msvc"
    exe="$bin_dir/blazescript.exe"
else
	case $(uname -sm) in
	"Darwin x86_64") target="x86_64-apple-darwin" ;;
	"Darwin arm64") target="aarch64-apple-darwin" ;;
	*) target="blazescript-linux" ;;
	esac
    exe="$bin_dir/blazescript"
fi


if [ $# -eq 0 ]; then
	bzs_uri="https://github.com/BlazifyOrg/blazescript/releases/latest/download/${target}"
else
	bzs_uri="https://github.com/BlazifyOrg/blazescript/releases/download/${1}/${target}"
fi

curl --fail --location --progress-bar --output "$exe" "$bzs_uri"
chmod +x "$exe"

if command -v blazescript >/dev/null; then
    echo "Run 'blazescript path/to/file.bz(s/e)' to get started"
else
    case $SHELL in
    	/bin/zsh) shell_profile=".zshrc" ;;
    	*) shell_profile=".bash_profile" ;;
    esac

    echo "Manually add the directory to your \$HOME/$shell_profile (or similar)"
    echo "  export BZS_INSTALL=\"$bzs_install\""
    echo "  export PATH=\"\$BZS_INSTALL/bin:\$PATH\""
    echo "Run '$exe path/to/file.bz(s/e)' to get started"
fi
