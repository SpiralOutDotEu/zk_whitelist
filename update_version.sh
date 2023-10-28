#!/bin/bash

# Usage: ./update_version.sh <new-version>
# Example: ./update_version.sh 0.2.0

NEW_VERSION=$1
CARGO_TOML="Cargo.toml"

# Update the version in Cargo.toml
sed -i.bak -e "s/^version = .*/version = \"$NEW_VERSION\"/" $CARGO_TOML

# Remove the backup file created by sed
rm $CARGO_TOML.bak

# Update Cargo.lock file
cargo update

# Build the project with the updated version number
cargo build --release
