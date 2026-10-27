#!/usr/bin/env python3
import re
import os

ASM = '/home/fsa/src/elite/zx_src/kladov.asm'
OUTDIR = '/home/fsa/src/elite/kladov'

with open(ASM) as f:
    lines = f.readlines()

# Get already extracted
extracted = set()
for root, dirs, files in os.walk(OUTDIR):
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

# Find all functions
funcs = []
for i,l in enumerate(lines):
    m = re.match(r'^([A-Za-z_][A-Za-z0-9_]*):', l)
    if m:
        name = m.group(1)
        if name in ('segment','org'): continue
        if name.startswith(('loc_','word_','byte_','dword_','off_','unk_','a','asc','struc','ends','field_')): continue
        if name in extracted: continue
        funcs.append((name, i, l.strip()))
    m2 = re.match(r'^(sub_[0-9A-Fa-f]+):', l)
    if m2:
        name = m2.group(1)
        if name in extracted: continue
        funcs.append((name, i, l.strip()))
    m3 = re.match(r'^(j_[A-Za-z0-9_]+):', l)
    if m3:
        name = m3.group(1)
        if name in extracted: continue
        funcs.append((name, i, l.strip()))

print(f'Осталось извлечь: {len(funcs)}')

def find_end(start_idx):
    for j in range(start_idx+1, len(lines)):
        l = lines[j]
        m = re.match(r'^([A-Za-z_][A-Za-z0-9_]*):', l)
        if m:
            if m.group(1) in ('segment','org'): continue
            if m.group(1).startswith(('loc_','word_','byte_','dword_','off_','unk_','a','asc','struc','ends','field_')): continue
            return j
        if re.match(r'^(sub_[0-9A-Fa-f]+|j_[A-Za-z0-9_]+):', l):
            return j
    return len(lines)

# Categorize remaining
def categorize(name, comment):
    n = name.lower()
    c = comment.lower()
    s = n + ' ' + c
    if any(x in n for x in ['addnormal','crd','vector','angle','matrix','calc','getcrd']):
        return '07_3d_projection'
    if any(x in n for x in ['compas','scaner','radar','blip','indicator']):
        return '08_radar_scanner'
    if any(x in n for x in ['circle','sun','planet','dataof','altitude']):
        return '10_circles_planets'
    if any(x in n for x in ['laser','missile','fire','shot','damage','explode']):
        return '11_combat_lasers'
    if any(x in n for x in ['dock','auto']):
        return '12_docking'
    if any(x in n for x in ['hyper','jump']):
        return '13_hyperspace'
    if any(x in n for x in ['galaxy','map','system','cmd']):
        return '14_galaxy_map'
    if any(x in n for x in ['equip','trade','cargo','price','shop']):
        return '15_trading_equip'
    if any(x in n for x in ['mission','targon','war','pilot']):
        return '16_missions'
    if any(x in n for x in ['save','load','disk']):
        return '17_save_load'
    if any(x in n for x in ['print','text','msg','number']):
        return '18_text_print'
    if any(x in n for x in ['sound','beep']):
        return '19_sound'
    if any(x in n for x in ['move','rotat','turn','thrust','momentum']):
        return '04_movement_physics'
    if any(x in n for x in ['collis','hit','destroy']):
        return '05_collision_damage'
    if any(x in n for x in ['prn','draw','line','pixel','obj']):
        return '06_objects_render'
    if any(x in n for x in ['key','input','control']):
        return '02_input_keyboard'
    if any(x in n for x in ['init','game']):
        return '01_start_init'
    return '99_misc'

cats = {}
for name, idx, comment in funcs:
    cat = categorize(name, comment)
    if cat not in cats:
        cats[cat] = []
    end_idx = find_end(idx)
    block = ''.join(lines[idx:end_idx])
    cats[cat].append((name, block, comment))

# Append to existing files
for cat in sorted(cats.keys()):
    items = cats[cat]
    fname = os.path.join(OUTDIR, cat, f'{cat}_remaining.asm')
    with open(fname, 'w') as f:
        f.write(f'; {cat} - оставшиеся подпрограммы\n')
        f.write(f'; Количество: {len(items)}\n\n')
        for name, block, comment in items:
            f.write(f'; === {name} === {comment}\n')
            f.write(block)
            f.write('\n\n')
    print(f'{cat}: +{len(items)}')
