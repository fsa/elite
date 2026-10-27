#!/usr/bin/env python3
ASM = '/home/fsa/src/elite/zx_src/kladov.asm'
sections = [
    (4375, 4520, '27_calc_ang_speed.asm'),
    (4520, 4675, '28_control_keys.asm'),
    (7660, 7820, '29_sub80d3.asm'),
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
