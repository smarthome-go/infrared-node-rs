version := 0.1.0

clean:
	rm -rf build

lint:
	cargo clippy
	typos

build: ./src ./Cargo.toml
	# Older ARM
	cross build \
		--target-dir=target \
		--target=arm-unknown-linux-musleabihf \
		--release
	# Newer ARM
	cross build \
		--target-dir=target \
		--target=armv7-unknown-linux-musleabihf \
		--release

package: build
	# Remove old build directory
	rm -rf build
	# Create a fresh build directory
	mkdir -p build/arm
	mkdir -p build/armv7
	cp CHANGELOG.md build/arm
	cp CHANGELOG.md build/armv7
	cp install.sh build/arm
	cp install.sh build/armv7
	cp ifrs.service build/arm
	cp ifrs.service build/armv7
	cp target/arm-unknown-linux-musleabihf/release/infrared-node-rs build/armv7
	cp target/armv7-unknown-linux-musleabihf/release/infrared-node-rs build/arm
	# Create tar archives from the build directories
	cd build && tar -cvzf ifrs_v$(version)_arm.tar.gz arm --remove-files
	cd build && tar -cvzf ifrs_v$(version)_armv7.tar.gz armv7 --remove-files



release: clean lint package

gh-release:
	gh release create v$(version) ./build/*.tar.gz -F ./CHANGELOG.md -t 'Infrared Node v$(version)'
