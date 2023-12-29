#!/bin/sh

# Run cargo clippy
echo "Running cargo clippy..."
cargo clippy -- -D warnings
CLIPPY_EXIT_CODE=$?

# If clippy fails, exit
if [ $CLIPPY_EXIT_CODE -ne 0 ]; then
    echo "cargo clippy failed, aborting push..."
    exit 1
fi

# Run cargo build
echo "Running cargo build..."
cargo build
BUILD_EXIT_CODE=$?

# If build fails, exit
if [ $BUILD_EXIT_CODE -ne 0 ]; then
    echo "cargo build failed, aborting push..."
    exit 1
fi

# If all checks pass, allow the push
exit 0

