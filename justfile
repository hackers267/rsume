# release for linux
release:
	cargo build -r --target x86_64-unknown-linux-musl
	upx target/x86_64-unknown-linux-musl/release/rsume
# release for windows
release-win:
	cargo build -r --target x86_64-pc-windows-gnu
