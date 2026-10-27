#!/usr/bin/env python3
# Compare radar/circle code between elite.asm and kladov.asm
import re

ELITE = '/home/fsa/src/elite/zx_src/elite.asm'
KLADOV = '/home/fsa/src/elite/zx_src/kladov.asm'

def get_func(asm_path, name):
    with open(asm_path) as f:
        lines = f.readlines()
    # find label
    start = None
    for i,l in enumerate(lines):
        if re.match(rf'^{re.escape(name)}:', l):
            start = i
            break
    if start is None:
        return None
    # find end
    for j in range(start+1, len(lines)):
        l = lines[j]
        if re.match(r'^[A-Za-z_][A-Za-z0-9_]+:', l) and not re.match(r'^[A-Za-z_][A-Za-z0-9_]+:\s*;', l):
            m = re.match(r'^([A-Za-z_][A-Za-z0-9_]*):', l)
            if m and not m.group(1).startswith(('loc_','word_','byte_','dword_')):
                return ''.join(lines[start:j])
        if re.match(r'^(sub_[0-9A-Fa-f]+|j_[A-Za-z0-9_]+):', l):
            return ''.join(lines[start:j])
    return ''.join(lines[start:])

for func in ['PrnCircle_Fill','PrnCircle_NoFill','PrnCircle_Clip','ShipHyperJump','sub_7E02']:
    e = get_func(ELITE, func)
    k = get_func(KLADOV, func)
    print(f'=== {func} ===')
    print(f'elite: {"found" if e else "NOT FOUND"} ({len(e) if e else 0} lines)')
    print(f'kladov: {"found" if k else "NOT FOUND"} ({len(k) if k else 0} lines)')
    if e and k:
        print(f'same: {e==k}')
    print()
