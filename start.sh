#!/bin/bash

set -e

echo "Starting Docker Compose..."
docker compose up -d

echo "Building the Rust project in release mode..."
cargo build --release

echo "Running the Rust project..."
./target/release/$(basename $(pwd))