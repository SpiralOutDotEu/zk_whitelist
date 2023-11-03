#!/bin/bash

# Usage: ./update_version.sh <new-version>
# Example: ./update_version.sh 0.2.0

NEW_VERSION=$1
CARGO_TOML="Cargo.toml"
CARGO_LOCK="Cargo.lock"

# Update the version in Cargo.toml
sed -i.bak -e "s/^version = .*/version = \"$NEW_VERSION\"/" $CARGO_TOML
# Update the version in Cargo.lock
sed -i.bak "/name = \"zk_whitelist\"/,/version =/s/version = \"[^\"]*\"/version = \"$NEW_VERSION\"/" $CARGO_LOCK

# Remove the backup file created by sed
rm $CARGO_TOML.bak
rm $CARGO_LOCK.bak

# Build the project with the updated version number
cargo build --release

# Prepare for multi-os build
sudo apt-get install podman
cargo install cross

# Build for Windows
rustup target add x86_64-pc-windows-gnu

# build for Mac
curl -L https://github.com/roblabla/MacOSX-SDKs/releases/download/13.3/MacOSX13.3.sdk.tar.xz | tar xJ
export SDKROOT=$(pwd)/MacOSX13.3.sdk/
export PATH=$PATH:~/.rustup/toolchains/stable-x86_64-unknown-linux-gnu/lib/rustlib/x86_64-unknown-linux-gnu/bin/
export CARGO_TARGET_X86_64_APPLE_DARWIN_LINKER=rust-lld
rustup target add x86_64-apple-darwin
cargo build --release --target x86_64-apple-darwin

