all: build

.PHONY: build
build:
	$(shell [[ $EUID -eq 0 ]] && echo "build can not be run as root" && exit 1)
	@echo ":: Rebuilding in release mode..."
	@cargo build --release

.PHONY: install
install:
	@echo ":: Installing binaries..."
	@cp -f target/release/mytwm /usr/local/bin
	@chmod 755 /usr/local/bin/mytwm
	@echo ":: Done"

.PHONY: uninstall
uninstall:
	@echo ":: Removing binaries..."
	@rm -f /usr/local/bin/mytwm
	@echo ":: Done"
