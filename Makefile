_MDPS_BUILD_COMMIT=$(shell git rev-parse --short HEAD)
_MDPS_PKG_NAME=$(shell awk -F ' = ' '$$1 ~ /name/ { gsub(/["]/, "", $$2); printf("%s",$$2) }' Cargo.toml)
_MDPS_PKG_VERSION=$(shell awk -F ' = ' '$$1 ~ /version/ { gsub(/["]/, "", $$2); printf("%s",$$2) }' Cargo.toml)
_MDPS_BUILD_DATE=$(shell date -u +%Y-%m-%dT%H:%M:%SZ)

build_web:
	@echo "Installing dependencies (yarn)"
	cd src_web && yarn
	@echo "Building web"
	cd src_web && yarn build
	@echo "Copying web to server"
	@mkdir -p assets
	mv src_web/build assets/web_build

build_rust_linux_x86_64:
	@echo "Installing dependencies (cargo)"
	cargo fetch --locked --target "x86_64-unknown-linux-gnu"
	@echo "Building rust"
	@export CARGO_TARGET_DIR=target
	@export RUSTUP_TOOLCHAIN=stable
	cargo build --frozen --release --all-features

build_linux_x86_64: build_web build_rust_linux_x86_64

create_target_dir_linux:
	@echo "Creating target directory"
	@mkdir -p pkg_target/usr/bin
	@mv target/release/$(_MDPS_PKG_NAME) pkg_target/usr/bin
	@mkdir -p pkg_target/etc
	@cp -r static_assets/templates.d pkg_target/etc

build_linux_x86_64_tar: build_linux_x86_64 create_target_dir_linux
	@echo "Creating tarball"
	tar -C pkg_target -czf $(_MDPS_PKG_NAME)-$(_MDPS_PKG_VERSION)-$(_MDPS_BUILD_COMMIT).tar.gz pkg_target/*
	@mkdir -p build
	@mv $(_MDPS_PKG_NAME)-$(_MDPS_PKG_VERSION)-$(_MDPS_BUILD_COMMIT).tar.gz build
	rm -rf pkg_target
