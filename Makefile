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

bump:
	$(eval V=$(shell echo "$(VERSION)" | cut -d '.' -f 1))
	$(eval R=$(shell echo "$(VERSION)" | cut -d '.' -f 2))
	$(eval B=$(shell echo "$(VERSION)" | cut -d '.' -f 3))
	$(eval X=$(shell expr ${B} + 1))
	$(eval NEW=$(shell echo "${V}.${R}.${X}"))
	@echo "Version Bump : $(VERSION) => $(NEW)"
	@sed -E -i 's/^version(.*)/version = "$(NEW)"/' Cargo.toml
	@make srcinfo

static: clean
	RUSTFLAGS='-C target-feature=+crt-static' cargo build --release --target x86_64-unknown-linux-gnu

static-install:
	rm -fv /usr/bin/{aid,air} 
	install -m 755 ./target/release/air /usr/bin
	ln -sf /usr/bin/air /usr/bin/aid
