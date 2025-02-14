#!/usr/bin/env bash
set -euo pipefail

export DEBIAN_FRONTEND=noninteractive
sudo apt-get update &&
    sudo apt-get install -y gcc-aarch64-linux-gnu g++-aarch64-linux-gnu libc6-dev-arm64-cross

export CARGO_TARGET_AARCH64_UNKNOWN_LINUX_GNU_LINKER=aarch64-linux-gnu-gcc
export CC_aarch64_unknown_linux_gnu=aarch64-linux-gnu-gcc
export CXX_aarch64_unknown_linux_gnu=aarch64-linux-gnu-g++
rustup target add aarch64-unknown-linux-gnu
cargo build --target aarch64-unknown-linux-gnu --release
echo "Build completed"

rm -rf /tmp/secretsquirrel/aarch64*
echo "Cleaned up old files"

ls -altrh ./
ls -altrh ./target
ls -altrh ./target/aarch64-unknown-linux-gnu

mv ./target/aarch64-unknown-linux-gnu/release/secretsquirrel /tmp/secretsquirrel/aarch64-secretsquirrel
