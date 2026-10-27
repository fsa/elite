#!/usr/bin/env python3
import re
import os

ASM = '/home/fsa/src/elite/zx_src/kladov.asm'
with open(ASM) as f:
    lines = f.readlines()

# Count all code lines (not comments only, not data definitions)
code_lines = 0
data_lines = 0
for l in lines:
    s = l.strip()
    if not s or s.startswith(';'):
        continue
    if re.match(r'^[A-Za-z_][A-Za-z0-9_]+:\s*$', s):
        continue
    if re.match(r'^(loc_|word_|byte_|dword_|off_|unk_|a[A-Za-z]|asc_|db |dw |ds |equ )', s, re.I):
        if re.match(r'^(loc_|word_|byte_|dword_|off_|unk_):', s):
            continue
        # data
        if re.search(r'\b(db|dw|ds)\b', s, re.I) and not re.search(r'\b(CALL|JP|JR|LD|ADD|SUB|CP|RET)\b', s, re.I):
            data_lines += 1
            continue
    code_lines += 1

print(f'Всего непустых строк: {len([l for l in lines if l.strip() and not l.strip().startswith(";")])}')
print(f'Кодоподобных: {code_lines}')
print(f'Данных: {data_lines}')
print(f'Всего: {len(lines)}')

# Count by extracting all
