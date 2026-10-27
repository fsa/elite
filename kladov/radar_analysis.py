#!/usr/bin/env python3
"""Анализ кода радара/кругов для версии Кладова"""
import os

BIN = '/home/fsa/src/elite/zx_bin/kladov.bin'
ASM = '/home/fsa/src/elite/zx_src/kladov.asm'

def z80_to_off(addr):
    if 0x4000 <= addr < 0x8000:
        return addr - 0x4000
    elif 0x8000 <= addr < 0xC000:
        return 0x4000 + (addr - 0x8000)
    elif 0xC000 <= addr <= 0xFFFF:
        return 0x8000 + (addr - 0xC000)
    raise ValueError

with open(BIN,'rb') as f:
    data = f.read()

# Check circle code matches
for name, addr in [('PrnCircle_Fill',0x9084),('PrnCircle_NoFill',0x9095),('PrnCircle_NoFill_NoRadius',0x90A5),('PrnCircle_Clip',0x9115)]:
    off = z80_to_off(addr)
    print(f'{name} {addr:04X}:', ' '.join(f'{b:02X}' for b in data[off:off:8]))
