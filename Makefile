BINARY  := release/air
VERSION := $(shell cat Cargo.toml | grep -E '^version' | head -n 1 | cut -d '=' -f 2 | sed -E 's/[" ]//g')
RELEASE := 1
SHELL   := /bin/bash -O extglob -c

release: clean ${BINARY}

${BINARY}:
	cargo build --release

clean:
	rm -rf $(filter-out archlinux/PKGBUILD,$(wildcard archlinux/*)) archlinux/.SRCINFO
	cargo clean

srcinfo:
	@echo VERSION=$(VERSION) RELEASE=$(RELEASE)
	cd archlinux && \
		sed -i -E "s/^pkgver=([0-9|.|r|-]+)/pkgver=$(VERSION)/g" PKGBUILD && \
		sed -i -E "s/^pkgrel=([0-9|.|r|-]+)/pkgrel=$(RELEASE)/g" PKGBUILD && \
		makepkg --printsrcinfo > .SRCINFO

install: clean srcinfo
	cd archlinux && \
		makepkg -si

install-home: ${BINARY}
	cargo install --path . --profile release

publish: srcinfo
	git tag -a "v$(VERSION)" -m "v$(VERSION)"
	git push origin "v$(VERSION)"
