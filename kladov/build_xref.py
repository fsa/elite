#!/usr/bin/env python3
import re
import os
import json
from collections import defaultdict

ASM = '/home/fsa/src/elite/zx_src/kladov.asm'
OUT = '/home/fsa/src/elite/kladov/xref'

with open(ASM) as f:
    lines = f.readlines()

# Collect labels
labels = set()
label_to_line = {}
for i,l in enumerate(lines):
    m = re.match(r'^([A-Za-z_][A-Za-z0-9_]*):', l)
    if m:
        name = m.group(1)
        if name in ('segment','org'):
            continue
        if name.startswith(('loc_','word_','byte_','dword_','off_','unk_','a','asc')):
            continue
        labels.add(name)
        label_to_line[name] = i+1
    m2 = re.match(r'^(sub_[0-9A-Fa-f]+):', l)
    if m2:
        labels.add(m2.group(1))
        label_to_line[m2.group(1)] = i+1
    m3 = re.match(r'^(j_[A-Za-z0-9_]+):', l)
    if m3:
        labels.add(m3.group(1))
        label_to_line[m3.group(1)] = i+1

# Find calls
calls = defaultdict(set)  # caller -> set of callees
callees = defaultdict(set) # callee -> set of callers
data_refs = defaultdict(set)

for i,l in enumerate(lines):
    # skip data definitions mostly
    # look for CALL/JR/JP
    for m in re.finditer(r'\b(CALL|JP|JR|RST)\s+([A-Za-z_][A-Za-z0-9_]+)\b', l):
        instr = m.group(1)
        target = m.group(2)
        if target in ('NZ','Z','NC','C','PO','PE','P','M'):  # conditions
            continue
        if target in labels or target.startswith(('sub_','j_')):
            # find current sub
            # look backwards for label
            for j in range(i, max(0,i-200), -1):
                lm = re.match(r'^([A-Za-z_][A-Za-z0-9_]*):', lines[j])
                if lm:
                    caller = lm.group(1)
                    if not caller.startswith(('loc_','word_','byte_')):
                        calls[caller].add(target)
                        callees[target].add(caller)
                    break
                lm2 = re.match(r'^(sub_[0-9A-Fa-f]+):', lines[j])
                if lm2:
                    caller = lm2.group(1)
                    calls[caller].add(target)
                    callees[target].add(caller)
                    break
                lm3 = re.match(r'^(j_[A-Za-z0-9_]+):', lines[j])
                if lm3:
                    caller = lm3.group(1)
                    calls[caller].add(target)
                    callees[target].add(caller)
                    break
    
    # data refs
    for m in re.finditer(r'\b(LD|LDIR|LDDR|CP|INC|DEC)\b.*\b([A-Za-z_][A-Za-z0-9_]{3,})\b', l):
        target = m.group(2)
        if target in labels and target not in ('HL','DE','BC','AF','IX','IY','SP'):
            pass

# Write callgraph
with open(os.path.join(OUT,'callgraph.md'),'w') as f:
    f.write('# Граф вызовов kladov.asm\n\n')
    f.write('CALL/JP/JR между подпрограммами\n\n')
    for caller in sorted(calls.keys(), key=lambda x: (x not in label_to_line, label_to_line.get(x,99999))):
        if not calls[caller]:
            continue
        f.write(f'## {caller} (строка {label_to_line.get(caller,"?")})\n')
        for c in sorted(calls[caller]):
            f.write(f'- -> {c}\n')
        f.write('\n')

# Write dot
with open(os.path.join(OUT,'callgraph.dot'),'w') as f:
    f.write('digraph kladov {\n')
    f.write('  rankdir=LR;\n')
    f.write('  node [shape=box];\n')
    for caller in calls:
        for c in calls[caller]:
            if caller in labels and c in labels:
                f.write(f'  "{caller}" -> "{c}";\n')
    f.write('}\n')

# Write xrefs
with open(os.path.join(OUT,'xrefs.md'),'w') as f:
    f.write('# Перекрёстные ссылки\n\n')
    for label in sorted(labels, key=lambda x: label_to_line.get(x,99999)):
        if label in callees and callees[label]:
            f.write(f'## {label} вызывается из:\n')
            for c in sorted(callees[label]):
                f.write(f'- {c}\n')
            f.write('\n')

print('done')
