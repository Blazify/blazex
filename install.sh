#!/bin/sh

if ! command -v cargo >/dev/null; then
	echo "Error: Rust is required to install Blazescript (see: https://www.rust-lang.org/tools/install)." 1>&2
	exit 1
fi

bzs_install="${BZS_INSTALL:-$HOME/.bzs}"

git clone https://github.com/BlazifyOrg/blazescript.git $bzs_install
cd $bzs_install
if [ "$OS" = "Windows_NT" ]; then
	start build.sh
else
    bash build.sh
fi

bin_dir="$bzs_install/bin"
exe="$bin_dir/blazescript"

if command -v blazescript >/dev/null; then
    echo "Run 'blazescript [file name][(.bzs)/(.bze)]' to get started"
else
    case $SHELL in
    	/bin/zsh) shell_profile=".zshrc" ;;
    	*) shell_profile=".bash_profile" ;;
    esac

    echo "Manually add the directory to your \$HOME/$shell_profile (or similar)"
    echo "  export BZS_INSTALL=\"$bzs_install\""
    echo "  export PATH=\"\$BZS_INSTALL/bin:\$PATH\""
    echo "Run '$exe [file name][(.bzs)/(.bze)]' to get started"
fi