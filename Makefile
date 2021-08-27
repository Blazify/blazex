.PHONY: install build

# Installation and bin directories
INSTALL_DIR := $(shell echo $${BZS_INSTALL:-$$HOME/.bzs})
BIN_DIR := $(INSTALL_DIR)/bin
EXECUTABLE := $(BIN_DIR)/blazex

# Platform info
PLATFORM := $(shell uname -sm)
ifeq ($$SHELL, /bin/zsh)
	SHELL_PROFILE := .zshrc
else
	SHELL_PROFILE := .bash_profile
endif

# Download/build info
ifeq ($(shell $$OS), Windows_NT)
	TARGET := x86_64-pc-windows-msvc
	EXTENSION := ".exe"
endif
ifeq ($(PLATFORM), Darwin x86_64)
	TARGET := x86_64-apple-darwin
	DOWNLOAD_TARGET := blazex-macos
endif
ifeq ($(PLATFORM), Linux x86_64)
	TARGET := x86_64-unknown-linux-gnu
	DOWNLOAD_TARGET := blazex-linux
endif

ifeq ($(shell echo $$BZS_VER),)
	DOWNLOAD_URI := https://github.com/BlazifyOrg/blazex/releases/latest/download/$(DOWNLOAD_TARGET)
else
	DOWNLOAD_URI := https://github.com/BlazifyOrg/blazex/releases/download/$$BZS_VER/$(DOWNLOAD_TARGET)
endif

build:
	cargo build --locked --target $(TARGET) --release

ifneq ($(shell test -d $(BIN_DIR) ; echo $$?), 0)
	@mkdir -p "./bin"
endif

ifeq ($(shell test -e ./bin/blazex$(EXTENSION) ; echo $$?), 0)
	@rm -r "./bin/blazex$(EXTENSION)"
endif

	cp "./target/$(TARGET)/release/blazex$(EXTENSION)" "./bin/blazex$(EXTENSION)"
	strip "./bin/blazex$(EXTENSION)"

install:
ifeq ($(DOWNLOAD_TARGET),)
	@echo You are not using a supported operating system. Please submit a request to our team.
	@echo -----------------------------------------------------
	@echo "$(PLATFORM) is not supported by Blazex currently."
	@echo -----------------------------------------------------
	@echo If you would like to use it then please drop a request at https://github.com/BlazifyOrg/issues. Thanks!
else
	@echo STARTING BLAZEX INSTALLATION
	@echo ----------------------------
	@echo
	@echo "Installation directory -> $(INSTALL_DIR)"
	@echo "Bin directory -> $(BIN_DIR)"
	@echo "Download target for your OS -> $(DOWNLOAD_TARGET)"
	@echo ---------------------------------------------
	@echo

ifneq ($(shell test -d $(BIN_DIR) ; echo $$?), 0)
	@mkdir -p $(BIN_DIR)
endif

	@cd $(BIN_DIR)
	
	@echo "Downloading executable from GitHub Releases..."
	@echo ---------------------------------------------
	curl --fail --location --progress-bar --output "$(EXECUTABLE)" "$(DOWNLOAD_URI)"
	@echo ---------------------------------------------
	@echo

	@chmod u+x "$(EXECUTABLE)"

ifeq ($(shell command -v blazex), 0)
	@echo "Run 'blazex path/to/file.bzx' to get started."
else
	@echo "Manually add the directory to your \"\$$HOME/$(SHELL_PROFILE)\" (or similar)"
	@echo "  export BZS_INSTALL=\"$(INSTALL_DIR)\""
	@echo "  export PATH=\"\$$BZS_INSTALL/bin:\$$PATH\""
	@echo "Run \"$(EXECUTABLE) path/to/file.bzx\" to get started."
endif
endif
