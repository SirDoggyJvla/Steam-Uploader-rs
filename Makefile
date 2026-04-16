.PHONY: help build release clean run test fmt lint

help:
	@echo "Usage: make [target]"
	@echo "Available targets:"
	@echo "  build   - Build the project in debug mode"
	@echo "  release - Build the project in release mode"
	@echo "  clean   - Clean the project"
	@echo "  run     - Run the project"
	@echo "  test    - Run tests"
	@echo "  fmt     - Format the code using rustfmt"
	@echo "  lint    - Lint the code using clippy"

build:
	cargo build

release:
	cargo build --release

clean:
	cargo clean

run:
	cargo run

test_upload:
# 	cargo run -- create --appid 108600

	cargo run -- upload --appid 108600 --workshopid 3707650648 --content "/home/simon/Documents/Repositories/Steam-Uploader-tool/SteamUploader-rs/testMod/Contents/hello-world" --preview "/home/simon/Documents/Repositories/Steam-Uploader-tool/SteamUploader-rs/testMod/Contents/preview.png"

test:
	cargo test

fmt:
	cargo fmt

lint:
	cargo clippy -- -D warnings