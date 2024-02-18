# Check supported version from rust-bert library on the README fro crate.io
setup:
		wget -O "libtorch-cxx11-abi-shared-with-deps-2.1.0+cpu.zip" https://download.pytorch.org/libtorch/cpu/libtorch-cxx11-abi-shared-with-deps-2.1.0%2Bcpu.zip
		rm -fr 	libtorch
		unzip "libtorch-cxx11-abi-shared-with-deps-2.1.0+cpu.zip" -d ~/
		rm "libtorch-cxx11-abi-shared-with-deps-2.1.0+cpu.zip"

run:
		cargo run --release

build:
		cargo build