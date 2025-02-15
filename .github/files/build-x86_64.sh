#!/usr/bin/env bash
set -euo pipefail

mkdir -p /tmp/secretsquirrel && rm -rf /tmp/secretsquirrel/x86_64*

cargo build --target x86_64-unknown-linux-gnu --release

mv ./target/x86_64-unknown-linux-gnu/release/secretsquirrel /tmp/secretsquirrel/x86_64-secretsquirrel
