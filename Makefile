.PHONY: build install

PLATFORM := $(shell uname -sm)
BLAZEX_DIR := $(HOME)/.blazex

build:
ifeq ($(PLATFORM), Linux x86_64)
ifeq ($(shell llvm-config-10 --version), 10.0.1)
	@echo "------------------------------"
	@echo "LLVM-10 already installed"
	@echo "------------------------------"
else
	@sudo apt-get update
	@sudo apt-get install -y build-essential libssl-dev libelf-dev libdwarf-dev libiberty-dev libunwind-dev libc++-dev libc++abi-dev llvm-10 zlib1g-dev
	@echo "------------------------------"
	@echo "LLVM-10 installed"
	@echo "------------------------------"
endif
endif

ifeq ($(PLATFORM), Darwin x86_64)
ifeq ($(shell llvm-config-10 --version), 10.0.1)
	@echo "------------------------------"
	@echo "LLVM-10 already installed"
	@echo "------------------------------"
else
	@sudo port install llvm-10
	@echo "------------------------------"
	@echo "LLVM-10 installed"
	@echo "------------------------------"
endif
endif
	cargo build --locked --release

ifneq ($(shell test -d "blazex/bin" ; echo $$?), 0)
	@mkdir -p "blazex/bin"
else
	@rm -rf "blazex/bin"
	@mkdir -p "blazex/bin"
endif

	@cp "./target/$(TARGET)/release/blazex$(EXTENSION)" "blazex/bin/blazex$(EXTENSION)"
	@strip "blazex/bin/blazex$(EXTENSION)"
	@cp -r "./target/$(TARGET)/release/stdlib" "blazex/stdlib"

install: build
	@rm -rf "$(BLAZEX_DIR)"
	@mv "blazex/" $(BLAZEX_DIR)