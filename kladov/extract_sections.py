#!/usr/bin/env python3
ASM = '/home/fsa/src/elite/zx_src/kladov.asm'

sections = [
    (6980, 7180, '02_destroy_hyper.asm'),
    (9080, 9700, '03_circles_main.asm'),
    (5250, 5450, '04_sun_planet.asm'),
    (12600, 13300, '05_scanner.asm'),  # scaner
]

for start, end, fname in sections:
    with open(ASM) as f:
        lines = f.readlines()
    out = []
    for i,l in enumerate(lines[start-1:end], start):
        out.append(f';{i:5d}: {l}')
    with open(f'/home/fsa/src/elite/kladov/{fname}','w') as f:
        f.writelines(out)
    print(f'{fname}: {len(out)} lines')
