.PHONY: run-wasm run-windows run-android run-ios

run-wasm:
	WASM_SERVER_RUNNER_ADDRESS=0.0.0.0:4000 cargo run --target wasm32-unknown-unknown

run-windows:
	cargo run -p desktop --target x86_64-pc-windows-gnu

run-android:
	rustup target add aarch64-linux-android armv7-linux-androideabi
	cargo install cargo-apk
	cargo apk run -p mobile

run-ios: 
	cd ./crates/mobile; make run-ios
