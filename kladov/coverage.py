#!/usr/bin/env python3
import re
import os

BIN = '/home/fsa/src/elite/zx_bin/kladov.bin'
ASM = '/home/fsa/src/elite/zx_src/kladov.asm'

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

# Read all labels from kladov.asm
with open(ASM) as f:
    lines = f.readlines()

# Find all subroutines/functions (not loc/word/byte/unk)
funcs = []
for i,l in enumerate(lines):
    m = re.match(r'^([A-Za-z_][A-Za-z0-9_]*):', l)
    if m:
        name = m.group(1)
        if name in ('segment','org'): continue
        if name.startswith(('loc_','word_','byte_','dword_','off_','unk_','a','asc','struc','ends','field_')): continue
        if name.startswith(('Data','Tabl','Msg','String')) and ':' in l and 'db' in l.lower():
            continue
        funcs.append((name, i+1))
    m2 = re.match(r'^(sub_[0-9A-Fa-f]+):', l)
    if m2:
        funcs.append((m2.group(1), i+1))
    m3 = re.match(r'^(j_[A-Za-z0-9_]+):', l)
    if m3:
        funcs.append((m3.group(1), i+1))

# Find ends
def find_end(start_idx):
    for j in range(start_idx, len(lines)):
        l = lines[j]
        if j == start_idx:
            continue
        if re.match(r'^[A-Za-z_][A-Za-z0-9_]+:', l):
            m = re.match(r'^([A-Za-z_][A-Za-z0-9_]*):', l)
            if m and m.group(1) not in ('segment','org'):
                if m.group(1).startswith(('loc_','word_','byte_','dword_','off_','unk_','a','asc')):
                    continue
                return j  # 0-based, exclusive
        if re.match(r'^(sub_[0-9A-Fa-f]+|j_[A-Za-z0-9_]+):', l):
            return j
    return len(lines)

# Collect covered ranges
covered = []
for name, line_no in funcs:
    idx = line_no - 1  # 0-based
    end_idx = find_end(idx)
    covered.append((name, idx, end_idx))

# Estimate size in bytes roughly - count instructions
total_lines = len(lines)
covered_lines = sum(e-s for _,s,e in covered)
print(f'Функций: {len(funcs)}')
print(f'Покрыто строк: {covered_lines} из {total_lines} ({covered_lines*100/total_lines:.1f}%)')

# Count by ranges
ranges = [(0x4000,0x5FFF,'4000-5FFF'),(0x6000,0x7FFF,'6000-7FFF'),(0x8000,0xBFFF,'8000-BFFF'),(0xC000,0xFFFF,'C000-FFFF')]
for rs,re_,rn in ranges:
    # rough count
    count = 0
    for _,s,e in covered:
        # check if in range by looking up address - hard to tell exact
        count += e-s
    print(f'{rn}: ~{count} строк')
