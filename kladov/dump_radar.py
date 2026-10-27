#!/usr/bin/env python3
import sys

BIN = '/home/fsa/src/elite/zx_bin/kladov.bin'

def z80_to_off(addr):
    if 0x4000 <= addr < 0x8000:
        return addr - 0x4000
    elif 0x8000 <= addr < 0xC000:
        return 0x4000 + (addr - 0x8000)
    elif 0xC000 <= addr <= 0xFFFF:
        return 0x8000 + (addr - 0xC000)
    raise ValueError(f'bad addr {addr:#06x}')

def hexdump(data, start_off, count=64):
    for i in range(0, count, 16):
        off = start_off + i
        chunk = data[off:off+16]
        hexs = ' '.join(f'{b:02X}' for b in chunk)
        print(f'{0x4000 + (start_off if False else 0):04X}? ')  # placeholder
    # easier: show z80 addr
    pass

def main():
    with open(BIN,'rb') as f:
        data = f.read()
    # key radar/circle addrs from kladov.asm
    addrs = [
        0x7E02, # sub_7E02
        0x7E4E, # ShipHyperJump
        0x7DFF, # DestroyObj
        0x8689, 0x86A1, 0x86B8, 0x86C4,
        0x9084, 0x9095, # PrnCircle_Fill, PrnCircle_NoFill
        0x90FB, 0x9115, # PrnCircle_NoFill_NoRadius, PrnCircle_Clip
        0x75AD, # near sun/planet
    ]
    for a in addrs:
        off = z80_to_off(a)
        print(f'=== {a:04X} (off {off:04X}) ===')
        for i in range(32):
            b = data[off+i]
            print(f'{a+i:04X}: {b:02X}')
        print()

if __name__ == '__main__':
    main()
