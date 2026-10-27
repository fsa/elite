#!/usr/bin/env python3
# Разбор всего кода kladov.asm по блокам
import re
import os

ASM = '/home/fsa/src/elite/zx_src/kladov.asm'
OUTDIR = '/home/fsa/src/elite/kladov'

# Read all lines
with open(ASM) as f:
    content = f.read()
    lines = content.splitlines(True)  # keep endings

# Find all subroutines
subs = []
for i,l in enumerate(lines):
    m = re.match(r'^([A-Za-z_][A-Za-z0-9_]*):\s*;\s*(.*)', l)
    if m:
        name = m.group(1)
        if name in ('segment','org') or name.startswith('loc_') or name.startswith('word_') or name.startswith('byte_') or name.startswith('dword_'):
            continue
        subs.append((i, name, m.group(2)))
    # also sub_XXXX
    m2 = re.match(r'^(sub_[0-9A-Fa-f]+):\s*;\s*(.*)', l)
    if m2 and not any(s[0]==i for s in subs):
        subs.append((i, m2.group(1), m2.group(2)))
    m3 = re.match(r'^(j_[A-Za-z0-9_]+):\s*;\s*(.*)', l)
    if m3 and not any(s[0]==i for s in subs):
        subs.append((i, m3.group(1), m3.group(2)))

print(f'Found {len(subs)} labels')

# Categorize
categories = {
    '00_header': [],
    '01_start_init': [],
    '02_input_keyboard': [],
    '03_game_loop': [],
    '04_movement_physics': [],
    '05_collision_damage': [],
    '06_objects_render': [],
    '07_3d_projection': [],
    '08_radar_scanner': [],
    '09_drawing_primitives': [],
    '10_circles_planets': [],
    '11_combat_lasers': [],
    '12_docking': [],
    '13_hyperspace': [],
    '14_galaxy_map': [],
    '15_trading_equip': [],
    '16_missions': [],
    '17_save_load': [],
    '18_text_print': [],
    '19_sound': [],
    '20_data_tables': [],
    '99_misc': []
}

def categorize(name, comment):
    n = name.lower()
    c = comment.lower()
    s = n + ' ' + c
    if any(x in n for x in ['init','reset','start']) and 'game' in s:
        return '01_start_init'
    if any(x in n for x in ['key','scan','input','control']):
        return '02_input_keyboard'
    if 'gamemain' in n or 'game_loop' in s or (n.startswith('sub_728d')):
        return '03_game_loop'
    if any(x in n for x in ['move','rotat','turn','speed','veloc','momentum','thrust']):
        return '04_movement_physics'
    if any(x in n for x in ['collis','damag','explode','destroy']):
        return '05_collision_damage'
    if any(x in n for x in ['prnobj','object','ship']) and 'scan' not in n:
        return '06_objects_render'
    if any(x in n for x in ['proj','crd','vector','vertex','normal','matrix']):
        return '07_3d_projection'
    if any(x in n for x in ['radar','scaner','blip','compass']):
        return '08_radar_scanner'
    if any(x in n for x in ['line','pixel','plot','draw','prnline']):
        return '09_drawing_primitives'
    if any(x in n for x in ['circle','sun','planet','nova']):
        return '10_circles_planets'
    if any(x in n for x in ['laser','missile','shot','beam']):
        return '11_combat_lasers'
    if 'dock' in n:
        return '12_docking'
    if any(x in n for x in ['hyper','jump','countdown']):
        return '13_hyperspace'
    if any(x in n for x in ['galaxy','map','system','planetname']):
        return '14_galaxy_map'
    if any(x in n for x in ['equip','trade','cargo','price','shop','market']):
        return '15_trading_equip'
    if any(x in n for x in ['mission','targon','war','pilot']):
        return '16_missions'
    if any(x in n for x in ['save','load','disk']):
        return '17_save_load'
    if any(x in n for x in ['print','prntxt','text','msg','digit']):
        return '18_text_print'
    if 'sound' in n:
        return '19_sound'
    return '99_misc'

for idx,(line_no,name,comment) in enumerate(subs):
    cat = categorize(name,comment)
    categories[cat].append((line_no,name,comment))
    print(f'{cat}: {name}')
