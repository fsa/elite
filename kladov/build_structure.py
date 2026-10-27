#!/usr/bin/env python3
import re
import os

ASM = '/home/fsa/src/elite/zx_src/kladov.asm'
OUTDIR = '/home/fsa/src/elite/kladov'

with open(ASM) as f:
    lines = f.readlines()

# Get all labels again
subs = []
for i,l in enumerate(lines):
    m = re.match(r'^([A-Za-z_][A-Za-z0-9_]*):\s*;\s*(.*)', l)
    if m:
        name = m.group(1)
        if name in ('segment','org') or name.startswith(('loc_','word_','byte_','dword_','off_','unk_','a','asc','struc','ends')):
            continue
        subs.append((i, name, m.group(2).strip()))
    m2 = re.match(r'^(sub_[0-9A-Fa-f]+):\s*;\s*(.*)', l)
    if m2 and not any(s[0]==i for s in subs):
        subs.append((i, m2.group(1), m2.group(2).strip()))
    m3 = re.match(r'^(j_[A-Za-z0-9_]+):\s*;\s*(.*)', l)
    if m3 and not any(s[0]==i for s in subs):
        subs.append((i, m3.group(1), m3.group(2).strip()))

def categorize(name, comment):
    n = name.lower()
    c = comment.lower()
    s = n + ' ' + c
    if any(x in n for x in ['initgame','j_init','resetbattle','searchplanet','generateprice']):
        return '01_start_init'
    if any(x in n for x in ['scankey','keyboard','input']):
        return '02_input_keyboard'
    if 'gamemain' in n or 'sub_728d' == n:
        return '03_game_loop'
    if any(x in n for x in ['moveobject','moveall','reaction','directobject','momentum','rotat','turn']):
        return '04_movement_physics'
    if any(x in n for x in ['collis','damage','explode','destroy','objdestroy']):
        return '05_collision_damage'
    if any(x in n for x in ['prnobj','patternobject','shipset']) and 'scan' not in n:
        return '06_objects_render'
    if any(x in n for x in ['proj','crd','vector','vertex','normal','matrix','getcrdsprojection','setcrdcde']):
        return '07_3d_projection'
    if any(x in n for x in ['radar','scaner','blip','compass','prncompass']):
        return '08_radar_scanner'
    if any(x in n for x in ['prnline','prnpixel','line','pixel','plot','bufclip']):
        return '09_drawing_primitives'
    if any(x in n for x in ['circle','sun','planet','nova','smallcircle']):
        return '10_circles_planets'
    if any(x in n for x in ['laser','missile','shot','beam','setagrfire']):
        return '11_combat_lasers'
    if 'dock' in n:
        return '12_docking'
    if any(x in n for x in ['hyper','jump','countdown','hyperspace']):
        return '13_hyperspace'
    if any(x in n for x in ['galaxy','map','system','cmd_o','cmd_p']):
        return '14_galaxy_map'
    if any(x in n for x in ['equip','trade','cargo','price','shop','market','inventory']):
        return '15_trading_equip'
    if any(x in n for x in ['mission','targon','war','pilotinfo','testpilot']):
        return '16_missions'
    if any(x in n for x in ['save','load']):
        return '17_save_load'
    if any(x in n for x in ['print','prntxt','text','msg','digit','number','crlf']):
        return '18_text_print'
    if 'sound' in n or 'beep' in n:
        return '19_sound'
    if name.startswith(('loc_','word_','byte_')) or name.startswith('unk_'):
        return None
    return '99_misc'

# Create category dirs
cats = ['01_start_init','02_input_keyboard','03_game_loop','04_movement_physics','05_collision_damage','06_objects_render','07_3d_projection','08_radar_scanner','09_drawing_primitives','10_circles_planets','11_combat_lasers','12_docking','13_hyperspace','14_galaxy_map','15_trading_equip','16_missions','17_save_load','18_text_print','19_sound','99_misc']
for cat in cats:
    os.makedirs(os.path.join(OUTDIR, cat), exist_ok=True)

def find_end(start_idx):
    for j in range(start_idx+1, len(lines)):
        l = lines[j]
        if re.match(r'^[A-Za-z_][A-Za-z0-9_]+:', l) and not re.match(r'^[A-Za-z_][A-Za-z0-9_]+:\s*;(.*)', l) and l.strip() and not l.strip().startswith(('loc_','word_','byte_','dword_','off_','unk_','a','asc')):
            # check if it's a real label
            m = re.match(r'^([A-Za-z_][A-Za-z0-9_]*):', l)
            if m and m.group(1) not in ('segment','org') and not m.group(1).startswith(('loc_','word_','byte_')):
                return j
        if re.match(r'^(sub_[0-9A-Fa-f]+|j_[A-Za-z0-9_]+):', l):
            return j
    return len(lines)

# Categorize and extract
cat_files = {c: [] for c in cats}
for idx,(line_no,name,comment) in enumerate(subs):
    cat = categorize(name,comment)
    if cat is None:
        continue
    end_idx = find_end(line_no)
    # extract
    block = ''.join(lines[line_no:end_idx])
    cat_files[cat].append((name, block, comment))

# Write files
for cat in cats:
    items = cat_files[cat]
    # sort by name for consistency
    items.sort(key=lambda x: x[0])
    fname = os.path.join(OUTDIR, cat, f'{cat}.asm')
    with open(fname, 'w') as f:
        f.write(f'; {cat} - разобрано из kladov.asm\n')
        f.write(f'; Количество подпрограмм: {len(items)}\n\n')
        for name, block, comment in items:
            f.write(f'; === {name} === {comment}\n')
            f.write(block)
            f.write('\n\n')
    print(f'{cat}: {len(items)} subroutines')
