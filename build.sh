#!/bin/sh
# Assemble the game image (elite.asm -> elite.bin).
#
# elite.asm is an IDA disassembly; it is first rewritten into sjasmplus syntax
# and then assembled into the raw binary the emulator loads at 0x6048.
set -e
cd "$(dirname "$0")"

mkdir -p build
python3 tools/ida2sjasm.py zx_src/elite.asm build/elite.s.asm
tools/sjasmplus --raw=zx_bin/elite.bin --sym=build/elite.sym build/elite.s.asm

echo "built zx_bin/elite.bin"
