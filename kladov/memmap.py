#!/usr/bin/env python3
BIN = '/home/fsa/src/elite/zx_bin/kladov.bin'
with open(BIN,'rb') as f:
    data = f.read()

def z80_to_off(addr):
    if 0x4000 <= addr < 0x8000:
        return addr - 0x4000
    elif 0x8000 <= addr < 0xC000:
        return 0x4000 + (addr - 0x8000)
    elif 0xC000 <= addr <= 0xFFFF:
        return 0x8000 + (addr - 0xC000)
    raise ValueError

# Analyze memory layout
# Look for code vs data
# First 6KB (4000-57FF) mentioned
print('# Карта памяти kladov.bin (Z80 адреса 4000-FFFF)\n')
print('## Обзор\n')
print('- bin: 49152 байт = page5(4000-7FFF) + page2(8000-BFFF) + page0(C000-FFFF)\n')

# Check 4000-5FFF
print('## 4000-5FFF (первые ~8KB)\n')
# 4000-47FF seems mostly zeros
# 4800+ has data
# Look around 5C00+ as per kladov.asm
for addr in [0x4000,0x4800,0x5000,0x5C00,0x5D00,0x6000]:
    o = z80_to_off(addr)
    print(f'{addr:04X}: ', ' '.join(f'{data[o+i]:02X}' for i in range(16)))
print('\n## 6000-7FFF\n')
for addr in [0x6000,0x6800,0x7000,0x7800]:
    o = z80_to_off(addr)
    print(f'{addr:04X}: ', ' '.join(f'{data[o+i]:02X}' for i in range(16)))
print('\n## 8000-BFFF (page2)\n')
for addr in [0x8000,0x8800,0x9000,0x9800,0xA000,0xB000]:
    o = z80_to_off(addr)
    print(f'{addr:04X}: ', ' '.join(f'{data[o+i]:02X}' for i in range(16)))
print('\n## C000-FFFF (page0)\n')
for addr in [0xC000,0xC800,0xD000,0xE000,0xF000]:
    o = z80_to_off(addr)
    print(f'{addr:04X}: ', ' '.join(f'{data[o+i]:02X}' for i in range(16)))
