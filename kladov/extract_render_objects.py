#!/usr/bin/env python3
ASM = '/home/fsa/src/elite/zx_src/kladov.asm'
sections = [
    (15905, 16540, '43_prnobj_visible.asm'),
    (16540, 17140, '44_make_table_vertex.asm'),
    (17140, 17850, '45_calc_get_vertex.asm'),
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
