#!/usr/bin/env python3
ASM = '/home/fsa/src/elite/zx_src/kladov.asm'
sections = [
    (24780, 24900, '11_j_init_and_data.asm'),
    (27630, 27750, '12_reset_battle.asm'),
    (28420, 28600, '13_search_planet.asm'),
    (28830, 29000, '14_generate_price.asm'),
]
for start,end,fname in sections:
    with open(ASM) as f:
        lines = f.readlines()
    out = []
    for i,l in enumerate(lines[start-1:end], start):
        out.append(f';{i:5d}: {l}')
    with open(f'/home/fsa/src/elite/kladov/{fname}','w') as f:
        f.writelines(out)
    print(fname, len(out))
