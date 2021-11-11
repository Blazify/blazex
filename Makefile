.PHONY: build

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
ifneq ($(shell test -e "llvm-config --version" ; echo $$?), 0)
	cargo install llvmenv
	llvmenv build-entry 11.0.0
endif
	cargo build --locked --target $(TARGET) --release

ifneq ($(shell test -d "bin" ; echo $$?), 0)
	@mkdir -p "./bin"
endif

ifeq ($(shell test -e ./bin/blazex$(EXTENSION) ; echo $$?), 0)
	@rm -r "./bin/blazex$(EXTENSION)"
endif

	cp "./target/$(TARGET)/release/blazex$(EXTENSION)" "./bin/blazex$(EXTENSION)"
	strip "./bin/blazex$(EXTENSION)"
