#!/usr/bin/env python3
# Very basic Z80 disassembler for inspection
import sys

BIN = '/home/fsa/src/elite/zx_bin/kladov.bin'

def z80_to_off(addr):
    if 0x4000 <= addr < 0x8000:
        return addr - 0x4000
    elif 0x8000 <= addr < 0xC000:
        return 0x4000 + (addr - 0x8000)
    elif 0xC000 <= addr <= 0xFFFF:
        return 0x8000 + (addr - 0xC000)
    raise ValueError

# Minimal disasm - just enough for our purposes
OP = {
    0x00: 'NOP',
    0x01: lambda d,p: (3,'LD BC,%04X'%((d[p+2]<<8)|d[p+1])),
    0x03: 'INC BC',
    0x09: 'ADD HL,BC',
    0x0A: 'LD A,(BC)',
    0x0B: 'DEC BC',
    0x11: lambda d,p: (3,'LD DE,%04X'%((d[p+2]<<8)|d[p+1])),
    0x13: 'INC DE',
    0x19: 'ADD HL,DE',
    0x1A: 'LD A,(DE)',
    0x1B: 'DEC DE',
    0x21: lambda d,p: (3,'LD HL,%04X'%((d[p+2]<<8)|d[p+1])),
    0x22: lambda d,p: (3,'LD (%04X),HL'%((d[p+2]<<8)|d[p+1])),
    0x23: 'INC HL',
    0x29: 'ADD HL,HL',
    0x2A: lambda d,p: (3,'LD HL,(%04X)'%((d[p+2]<<8)|d[p+1])),
    0x2B: 'DEC HL',
    0x31: lambda d,p: (3,'LD SP,%04X'%((d[p+2]<<8)|d[p+1])),
    0x32: lambda d,p: (3,'LD (%04X),A'%((d[p+2]<<8)|d[p+1])),
    0x3A: lambda d,p: (3,'LD A,(%04X)'%((d[p+2]<<8)|d[p+1])),
    0x3E: lambda d,p: (2,'LD A,%02X'%d[p+1]),
    0x76: 'HALT',
    0xAF: 'XOR A',
    0xC0: 'RET NZ',
    0xC3: lambda d,p: (3,'JP %04X'%((d[p+2]<<8)|d[p+1])),
    0xC8: 'RET Z',
    0xC9: 'RET',
    0xCD: lambda d,p: (3,'CALL %04X'%((d[p+2]<<8)|d[p+1])),
    0xD0: 'RET NC',
    0xD8: 'RET C',
    0xE1: 'POP HL',
    0xE3: 'EX (SP),HL',
    0xE5: 'PUSH HL',
    0xE9: 'JP (HL)',
    0xF1: 'POP AF',
    0xF3: 'DI',
    0xF5: 'PUSH AF',
    0xFB: 'EI',
    0xFE: lambda d,p: (2,'CP %02X'%d[p+1]),
}

CB = {
    0x06: 'RLC (HL)',
    0x0E: 'RRC (HL)',
    0x16: 'RL (HL)',
    0x1E: 'RR (HL)',
    0x26: 'SLA (HL)',
    0x2E: 'SRA (HL)',
    0x3E: 'SRL (HL)',
}

ED = {
    0xB0: 'LDIR',
    0xB8: 'LDDR',
}

DD = {}
FD = {}
def disasm_range(data, start_z80, length):
    p = start_z80
    out = []
    end = start_z80 + length
    while p < end:
        off = z80_to_off(p)
        if off >= len(data):
            break
        op = data[off]
        if op == 0xCB:
            # simplified
            if p+1 < end:
                c2 = data[z80_to_off(p+1)]
                out.append((p, [op,c2], f'CB {c2:02X}'))
                p += 2
                continue
        if op == 0xED:
            if p+1 < end:
                e2 = data[z80_to_off(p+1)]
                if e2 in ED:
                    out.append((p,[op,e2], ED[e2]))
                else:
                    out.append((p,[op,e2], f'ED {e2:02X}'))
                p += 2
                continue
        if op == 0xDD or op == 0xFD:
            # prefix
            if p+1 < end:
                out.append((p,[op,data[z80_to_off(p+1)]], f'{op:02X} {data[z80_to_off(p+1)]:02X}...'))
                p += 2
                continue
        if op in OP:
            fn = OP[op]
            if callable(fn):
                try:
                    res = fn(data, off)
                    if isinstance(res, tuple):
                        l, s = res
                        out.append((p, list(data[off:off+l]), s))
                        p += l
                        continue
                except:
                    pass
            else:
                out.append((p,[op], fn))
                p += 1
                continue
        out.append((p,[op], f'DB {op:02X}'))
        p += 1
    return out

def main():
    with open(BIN,'rb') as f:
        data = f.read()
    for addr in [0x9084,0x9095,0x90A5,0x9115,0x7E02,0x7E4E]:
        print(f'=== {addr:04X} ===')
        for a,bs,s in disasm_range(data, addr, 60):
            print(f'{a:04X}: {" ".join(f"{b:02X}" for b in bs):12s} {s}')
        print()

if __name__=='__main__':
    main()
