.PHONY: run-ios xcodebuild install boot-ios-sim generate clean

DEVICE=AA1B7A57-1148-4011-AC84-1E8DA9D1A81A

boot-ios-sim:
	xcrun simctl boot $(DEVICE) || true

install-ios: xcodebuild-simulator boot-ios-sim
	xcrun simctl install $(DEVICE) build/Build/Products/Debug-iphonesimulator/bilder.app

xcodebuild-simulator:
	IOS_TARGETS=x86_64-apple-ios xcodebuild -scheme bilder -configuration Debug -derivedDataPath build -destination "id=$(DEVICE)"

xcodebuild-iphone:
	IOS_TARGETS=aarch64-apple-ios xcodebuild -scheme bilder -configuration Debug -derivedDataPath build -arch arm64

run-ios:
	rustup target add aarch64-apple-ios x86_64-apple-ios aarch64-apple-ios-sim
	cargo install xbuild
	cargo build -p mobile --target aarch64-apple-ios-sim
	make install-ios
	xcrun simctl launch --console $(DEVICE) studio.bilder.mobile

