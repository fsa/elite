#!/usr/bin/env python3
"""Extract the Kladov game RAM image from the ELITE3HL.TRD disk image.

The disk stores the 48K game as the 'ELITE3' CODE file (catalog entry 2),
which is a plain memory image of addresses 0x4000-0xFFFF.  On the TRD the
file starts at track 5 / sector 4 (16 sectors per track) i.e. byte offset
0x5400, so for this file `TRD_offset = address + 0x1400`.

Usage: extract_trd.py [trd] [out]
"""
import sys

DEFAULT_TRD = "zx_src/ELITE3HL.TRD"
DEFAULT_OUT = "build/kladov_ram.bin"
LOAD = 0x4000
SIZE = 0xC000
TRD_OFF = LOAD + 0x1400


def main() -> None:
    src = sys.argv[1] if len(sys.argv) > 1 else DEFAULT_TRD
    dst = sys.argv[2] if len(sys.argv) > 2 else DEFAULT_OUT
    data = open(src, "rb").read()
    img = data[TRD_OFF:TRD_OFF + SIZE]
    if len(img) != SIZE:
        raise SystemExit(f"{src}: too short for {SIZE} bytes at 0x{TRD_OFF:X}")
    open(dst, "wb").write(img)
    print(f"wrote {dst}: {SIZE} bytes (addresses 0x{LOAD:04X}-0x{LOAD + SIZE - 1:04X})")


if __name__ == "__main__":
    main()
