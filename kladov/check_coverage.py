#!/usr/bin/env python3
import os
import glob

kladov_dir = '/home/fsa/src/elite/kladov'
# Count files in our structure
total = 0
for root, dirs, files in os.walk(kladov_dir):
    for f in files:
        if f.endswith('.asm'):
            path = os.path.join(root,f)
            with open(path) as fh:
                total += len([l for l in fh if l.strip() and not l.strip().startswith(';')])
print(f'Строк в разобранных .asm файлах: {total}')
