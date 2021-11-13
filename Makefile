export LLVM_SYS_110_PREFIX := $(HOME)/.blazex/llvm-11.0.0

.PHONY: build

PLATFORM := $(shell uname -sm)
BLAZEX_DIR := $(HOME)/.blazex

# Download/build info
ifeq ($(shell $$OS), Windows_NT)
	TARGET := x86_64-pc-windows-msvc
	EXTENSION := ".exe"
endif
ifeq ($(PLATFORM), Darwin x86_64)
	TARGET := x86_64-apple-darwin
endif
ifeq ($(PLATFORM), Linux x86_64)
	TARGET := x86_64-unknown-linux-gnu
endif


build:
ifneq ($(shell test -d "$(LLVM_SYS_110_PREFIX)" ; echo $$?), 0)
	mkdir -p $(BLAZEX_DIR)
	cd $(BLAZEX_DIR) && \
	wget https://github.com/llvm/llvm-project/releases/download/llvmorg-11.0.0/llvm-11.0.0.src.tar.xz && \
	tar xJf llvm-11.0.0.src.tar.xz && \
	rm -rf llvm-11.0.0.src.tar.xz && \
	mkdir -p llvm-11.0.0.src/build && \
	cd llvm-11.0.0.src/build && \
	cmake ..  && \
	cmake --build . && \
	cmake --build . --target install && \
	cmake -DCMAKE_INSTALL_PREFIX=$(LLVM_SYS_110_PREFIX) -P cmake_install.cmake && \
	rm -rf llvm-11.0.0.src llvm-11.0.0.src.tar.xz
endif
	cargo build --locked --target $(TARGET) --release

ifneq ($(shell test -d "$(BLAZEX_DIR)/bin" ; echo $$?), 0)
	mkdir -p "$(BLAZEX_DIR)/bin"
endif

ifeq ($(shell test -e $(BLAZEX_DIR)/bin/blazex$(EXTENSION) ; echo $$?), 0)
	rm -r "$(BLAZEX_DIR)/bin/blazex$(EXTENSION)"
endif

	cp "./target/$(TARGET)/release/blazex$(EXTENSION)" "$(BLAZEX_DIR)/bin/blazex$(EXTENSION)"
	strip "$(BLAZEX_DIR)/bin/blazex$(EXTENSION)"
