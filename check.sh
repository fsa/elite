#!/bin/sh
# Quick checks: rebuild the game image and run the unit tests.
#   ./check.sh          -- build + unit tests
#   ./check.sh --full   -- above + full ZEXDOC instruction audit (needs
#                          tmp/zexdoc.com in the project, ~5 minutes)
set -e
cd "$(dirname "$0")"

./build.sh
cargo test --release

if [ "$1" = "--full" ]; then
    cargo test --release zexdoc -- --ignored --nocapture
fi

echo "check: OK"
