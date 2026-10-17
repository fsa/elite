#!/usr/bin/env python3
"""Собрать 48K-образ памяти версии Кладова для эмулятора.

Источник — снапшот Fuse `zx_src/kladov.snapshot` (ZXST), снятый сразу после
запуска ELITE3HL.TRD.  В снапшоте память 0x4000-0xFFFF лежит в трёх страницах:

    0x4000-0x7FFF  page 5
    0x8000-0xBFFF  page 2
    0xC000-0xFFFF  page 0

Результат — файл `zx_bin/kladov.bin` (49152 байта, адреса 0x4000-0xFFFF),
который эмулятор загружает по адресу 0x4000 и продолжает исполнение с
прерванного обработчика прерывания (см. KLADOV_STATE_* в src/main.rs).

Использование:
    python3 tools/make_kladov_image.py [snapshot] [out]
"""
import os
import sys
import zlib


def parse_szx(path):
    d = open(path, "rb").read()
    if d[:4] != b"ZXST":
        raise SystemExit("не ZXST-снапшот: %s" % path)
    off, chunks = 8, []
    while off + 8 <= len(d):
        cid = d[off:off + 4]
        ln = int.from_bytes(d[off + 4:off + 8], "little")
        chunks.append((cid, d[off + 8:off + 8 + ln]))
        off += 8 + ln
    return chunks


def ram_pages(chunks):
    out = {}
    for cid, body in chunks:
        if cid != b"RAMP":
            continue
        page = body[2]
        zi = body.find(b"\x78\xda")
        out[page] = zlib.decompress(body[zi:]) if zi >= 0 else body[3:]
    return out


def main():
    snap = sys.argv[1] if len(sys.argv) > 1 else "zx_src/kladov.snapshot"
    out = sys.argv[2] if len(sys.argv) > 2 else "zx_bin/kladov.bin"

    pages = ram_pages(parse_szx(snap))
    for p in (5, 2, 0):
        if p not in pages:
            raise SystemExit("в снапшоте нет страницы %d" % p)

    image = pages[5] + pages[2] + pages[0]
    if len(image) != 0xC000:
        raise SystemExit("неожиданный размер образа: %d" % len(image))

    os.makedirs(os.path.dirname(out) or ".", exist_ok=True)
    open(out, "wb").write(image)
    print("записан %s (%d байт)" % (out, len(image)))


if __name__ == "__main__":
    main()
