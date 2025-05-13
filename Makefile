# Makefile for yts-cli

PKGNAME := yts-cli
PKGVER  := 0.0.1a
SRC_TGZ := https://github.com/yourusername/yts-cli/archive/refs/tags/v$(PKGVER).tar.gz
TARBALL := $(PKGNAME)-$(PKGVER).tar.gz

.PHONY: all fetch build package install clean

all: build

## Download the source tarball (into project root)
fetch:
	curl -L $(SRC_TGZ) -o $(TARBALL)

## Build the Arch package (requires PKGBUILD present)
## This runs `makepkg -f --syncdeps --rmdeps --install`
package: fetch
	makepkg -f --syncdeps --rmdeps --install

## Build the Rust binary locally (for development)
build:
	cargo build --release

## Install the built binary system-wide
install: build
	install -Dm755 target/release/yts-cli /usr/local/bin/yts-cli

## Clean build artifacts
clean:
	cargo clean
	rm -f $(TARBALL) *.pkg.tar.zst
