#!/bin/sh
# Build (if needed) and run the Elite emulator.
#
# Additional arguments are forwarded to the emulator, e.g.:
#   ./run.sh --headless --frames 300
#   ./run.sh --rom /path/to/48.rom
set -e
cd "$(dirname "$0")"

[ -f elite.bin ] || ./build.sh

exec cargo run --release -- zx_bin/elite.bin "$@"
