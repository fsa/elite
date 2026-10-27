#!/usr/bin/env python3
"""Маппинг адресов Z80 <-> смещение в zx_bin/kladov.bin"""
import sys

BIN = '/home/fsa/src/elite/zx_bin/kladov.bin'

def z80_to_off(addr):
    if 0x4000 <= addr < 0x8000:
        return addr - 0x4000
    elif 0x8000 <= addr < 0xC000:
        return 0x4000 + (addr - 0x8000)
    elif 0xC000 <= addr <= 0xFFFF:
        return 0x8000 + (addr - 0xC000)
    raise ValueError(f'bad addr {addr:#06x}')

def read_bin():
    with open(BIN, 'rb') as f:
        return f.read()
