#!/usr/bin/env python3
ASM = '/home/fsa/src/elite/zx_src/kladov.asm'
sections = [
    (4560, 4675, '30_use_devices.asm'),
    (7205, 7395, '31_use_j_drive.asm'),
    (8580, 8730, '32_update_indicators.asm'),
    (13430, 13650, '33_process_missile.asm'),
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
