# release for linux
release:
	cross build -r --target x86_64-unknown-linux-musl
	upx target/x86_64-unknown-linux-musl/release/rsume
# release for windows
release-win:
	cross build -r --target x86_64-pc-windows-gnu
	upx target/x86_64-pc-windows-gnu/release/rsume.exe
# test
test:
	cargo watch -w src -x "nextest r"
