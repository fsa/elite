#!/usr/bin/env python3
import json
import os

BIN = '/home/fsa/src/elite/zx_bin/kladov.bin'
OUT = '/home/fsa/src/elite/kladov/core/data'

def z80_to_off(addr):
    if 0x4000 <= addr < 0x8000: return addr - 0x4000
    elif 0x8000 <= addr < 0xC000: return 0x4000 + (addr - 0x8000)
    elif 0xC000 <= addr <= 0xFFFF: return 0x8000 + (addr - 0xC000)

with open(BIN,'rb') as f:
    data = f.read()

tables = {}

# TableBasePrice at 701B
o = z80_to_off(0x701B)
tables['TableBasePrice'] = [data[o+i] for i in range(4)]

# TablSqr at BE00
o = z80_to_off(0xBE00)
tables['TablSqr'] = [data[o+i] for i in range(64)]

# TablCrdPrnVertex at ADCD?
# Search for TablCrdPrnVertex
# From listing 16314

with open(os.path.join(OUT,'tables.json'),'w') as f:
    json.dump(tables, f, indent=2)

# Write readable
with open(os.path.join(OUT,'tables.md'),'w') as f:
    f.write('# Таблицы данных\n\n')
    f.write('## TableBasePrice (0x701B)\n')
    f.write('```hex\n')
    f.write(' '.join(f'{b:02X}' for b in tables['TableBasePrice']))
    f.write('\n```\n\n')
    f.write('## TablSqr (0xBE00)\n')
    f.write('```hex\n')
    for i in range(0,64,16):
        f.write(' '.join(f'{b:02X}' for b in tables['TablSqr'][i:i+16]))
        f.write('\n')
    f.write('```\n')

print('done')
