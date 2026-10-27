#!/usr/bin/env python3
import re
import os

ASM = '/home/fsa/src/elite/zx_src/kladov.asm'
with open(ASM) as f:
    lines = f.readlines()

# Find all unique subroutines
funcs = set()
for i,l in enumerate(lines):
    m = re.match(r'^([A-Za-z_][A-Za-z0-9_]*):', l)
    if m:
        name = m.group(1)
        if name in ('segment','org'): continue
        if name.startswith(('loc_','word_','byte_','dword_','off_','unk_','a','asc','struc','ends','field_')): continue
        funcs.add(name)
    m2 = re.match(r'^(sub_[0-9A-Fa-f]+):', l)
    if m2: funcs.add(m2.group(1))
    m3 = re.match(r'^(j_[A-Za-z0-9_]+):', l)
    if m3: funcs.add(m3.group(1))

print(f'Уникальных функций в kladov.asm: {len(funcs)}')

# Check which ones in our extracted files
extracted = set()
for root, dirs, files in os.walk('/home/fsa/src/elite/kladov'):
    for fn in files:
        if fn.endswith('.asm'):
            with open(os.path.join(root,fn)) as f:
                for l in f:
                    m = re.match(r'^([A-Za-z_][A-Za-z0-9_]*):', l)
                    if m:
                        name = m.group(1)
                        if not name.startswith(('loc_','word_','byte_')):
                            extracted.add(name)
                    m2 = re.match(r'^(sub_[0-9A-Fa-f]+):', l)
                    if m2: extracted.add(m2.group(1))
                    m3 = re.match(r'^(j_[A-Za-z0-9_]+):', l)
                    if m3: extracted.add(m3.group(1))

print(f'Извлечено функций: {len(extracted)}')
missing = funcs - extracted
print(f'Не извлечено: {len(missing)}')
for m in sorted(list(missing))[:60]:
    print(f'  {m}')
