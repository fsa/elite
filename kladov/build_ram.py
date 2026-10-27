#!/usr/bin/env python3
import re
import os
from collections import defaultdict

ASM = '/home/fsa/src/elite/zx_src/kladov.asm'
OUT = '/home/fsa/src/elite/kladov/ram'

with open(ASM) as f:
    content = f.read()
    lines = content.splitlines()

# Find variable definitions
vars = []
for i,l in enumerate(lines):
    # word_XXXX, byte_XXXX, unk_XXXX
    m = re.match(r'^([A-Za-z_][A-Za-z0-9_]+):\s+(db|dw|ds|equ|EQU|\.)', l, re.I)
    if m:
        name = m.group(1)
        if name in ('segment','org'):
            continue
        vars.append((i+1, name, l.strip()))
    # also known vars
    if re.match(r'^(Circle|Pilot|Ship|Obj|Crd|Scan|Planet|Sun|Laser|Missile|Fuel|Cargo|Cash|Game|Flag|Jump|Dock)', l):
        m2 = re.match(r'^([A-Za-z_][A-Za-z0-9_]+):', l)
        if m2:
            vars.append((i+1, m2.group(1), l.strip()))

# Categorize
def cat_var(name):
    n = name.lower()
    if any(x in n for x in ['pilot','cash','fuel','cargo','credit','legal','rating']):
        return 'игрок/состояние'
    if any(x in n for x in ['obj','slot','ship','missile','laser']):
        return 'объекты'
    if any(x in n for x in ['crd','coord','vector','x','y','z','angle']):
        return 'координаты/векторы'
    if any(x in n for x in ['circle','radar','scan','compass','blip']):
        return 'радар/круги'
    if any(x in n for x in ['planet','sun','galaxy','system','star']):
        return 'галактика/планеты'
    if any(x in n for x in ['draw','buf','screen','prn','attr']):
        return 'экран/буфер'
    if any(x in n for x in ['key','flag','game','mode','demo']):
        return 'флаги/режимы'
    if any(x in n for x in ['dock','jump','hyper','warp']):
        return 'стыковка/гипер'
    if any(x in n for x in ['equip','price','market','trade']):
        return 'торговля'
    if any(x in n for x in ['sound','music']):
        return 'звук'
    return 'прочее'

cats = defaultdict(list)
seen = set()
for ln,name,l in vars:
    if name in seen:
        continue
    seen.add(name)
    cats[cat_var(name)].append((ln,name,l))

with open(os.path.join(OUT,'ram_map.md'),'w') as f:
    f.write('# Карта памяти (переменные) kladov.asm\n\n')
    for cat in sorted(cats.keys()):
        f.write(f'## {cat}\n\n')
        for ln,name,l in sorted(cats[cat], key=lambda x: x[0]):
            f.write(f'- {ln:5d}: `{name}` - {l}\n')
        f.write('\n')

print('done', len(seen))
