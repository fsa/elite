#!/usr/bin/env python3
# Create labeled disassembly files from kladov.asm references + binary
# Focus on radar/circle code
import re

ASM = '/home/fsa/src/elite/zx_src/kladov.asm'
OUT = '/home/fsa/src/elite/kladov'

# Extract labels and their addresses
labels = {}
with open(ASM) as f:
    for i,line in enumerate(f,1):
        # look for label: (no spaces at start in some forms, or just word:)
        m = re.match(r'^([A-Za-z_][A-Za-z0-9_]*):', line)
        if m:
            labels[m.group(1)] = i
        # also loc_XXXX
        m2 = re.match(r'^(loc_[0-9A-Fa-f]+):', line)
        if m2:
            labels[m2.group(1)] = i
        m3 = re.match(r'^(sub_[0-9A-Fa-f]+):', line)
        if m3:
            labels[m3.group(1)] = i

# Focus on radar related
radar_labels = []
for k in labels:
    kl = k.lower()
    if any(x in kl for x in ['radar','circle','blip','scan','hyp','hyper','destroy','object','prn','draw']):
        radar_labels.append((k, labels[k]))
radar_labels.sort(key=lambda x: x[1])
for k,lno in radar_labels[:80]:
    print(f'{k}: line {lno}')
