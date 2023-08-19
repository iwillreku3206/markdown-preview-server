build_web:
	@echo "Installing dependencies (yarn)"
	cd src_web && yarn
	@echo "Building web"
	cd src_web && yarn build
	@echo "Copying web to server"
	mkdir -p assets
	mv src_web/build assets/web_build

build_rust_linux_x86_64:
	@echo "Installing dependencies (cargo)"
	cargo fetch --locked --target "x86_64-unknown-linux-gnu"
	@echo "Building rust"
	export CARGO_TARGET_DIR=target
	export RUSTUP_TOOLCHAIN=stable
	cargo build --frozen --release --all-features

build_linux_x86_64: build_web build_rust_linux_x86_64

create_target_dir:
	mkdir -p pkg_target/usr/bin
	mv target/release/markdown-preview-server pkg_target/usr/bin
	mkdir -p pkg_target/etc
	cp -r static_assets/templates.d pkg_target/etc

build_linux_x86_64_tar: build_linux_x86_64 create_target_dir
	@echo "Creating tarball"
	tar -czf markdown-preview-server.tar.gz pkg_target
	rm -rf pkg_target
