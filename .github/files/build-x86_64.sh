#!/usr/bin/env bash
set -euo pipefail

echo "iam running the build script"
exit 1

mkdir -p /tmp/secretsquirrel && rm -rf /tmp/secretsquirrel/x86_64*

mv ./target/x86_64-unknown-linux-gnu/release/secretsquirrel /tmp/secretsquirrel/x86_64-secretsquirrel

cargo build --target x86_64-unknown-linux-gnu --release

mv ./target/x86_64-unknown-linux-gnu/release/secretsquirrel /tmp/secretsquirrel/x86_64-secretsquirrel
