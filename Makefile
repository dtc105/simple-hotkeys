BIN=simple-hotkeys
BACKEND=wayland

.PHONY: x11 wayland dev build install

x11:
	BACKEND=x11

wayland:
	BACKEND=wayland

dev:
	cargo build -F $(BACKEND)

build:
	cargo build --profile small -F $(BACKEND)

install: build
	sudo install -m755 ./target/small/$(BIN) /usr/local/bin	
