#!/usr/bin/env python3
ASM = '/home/fsa/src/elite/zx_src/kladov.asm'
sections = [
    (275, 430, '34_coriolis_calc.asm'),
    (5822, 5980, '35_calc_sun_planet.asm'),
    (7535, 7665, '36_cockpit.asm'),
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
