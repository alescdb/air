BINARY  := release/air
VERSION := $(shell cat Cargo.toml | grep -E '^version' | head -n 1 | cut -d '=' -f 2 | sed -E 's/[" ]//g')
RELEASE := 1

release: clean ${BINARY}

${BINARY}:
	cargo build --release

clean:
	cargo clean

install: clean
	@echo VERSION=$(VERSION) RELEASE=$(RELEASE)
	cd archlinux && \
		sed -i -E "s/^pkgver=([0-9|.|r|-]+)/pkgver=$(VERSION)/g" PKGBUILD && \
		sed -i -E "s/^pkgrel=([0-9|.|r|-]+)/pkgrel=$(RELEASE)/g" PKGBUILD && \
		makepkg --printsrcinfo > .SRCINFO && \
		makepkg -si

install-local: ${BINARY}
	cargo install --path . --profile release