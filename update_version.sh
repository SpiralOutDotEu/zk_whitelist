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
