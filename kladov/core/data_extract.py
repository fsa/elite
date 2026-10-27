#!/usr/bin/env python3
# Extract game data tables
import json
import os

BIN = '/home/fsa/src/elite/zx_bin/kladov.bin'
ASM = '/home/fsa/src/elite/zx_src/kladov.asm'
OUT = '/home/fsa/src/elite/kladov/core/data'

os.makedirs(OUT, exist_ok=True)

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
with open(ASM) as f:
    asm = f.read()

# Extract tables
# TableBasePrice
# Sqr table
# Others
