#!/usr/bin/env python3
"""Собрать образ памяти версии Кладова для эмулятора.

Основной источник — SNA-снапшот Spectrum 128 `zx_src/kladov128.snapshot`.
В нём все восемь банков ОЗУ по 16K; результат — `zx_bin/kladov128.bin`
(131072 байта) в физическом порядке банков 0..7. Эмулятор отображает банк 5
по `0x4000`, банк 2 по `0x8000`, а банк из младших битов `0x7FFD` — по
`0xC000`. Начальное значение порта пейджинга берётся из заголовка SNA
(см. `KLADOV_PAGE_REG` в `src/assets.rs`).

Для совместимости скрипт умеет и старый Fuse ZXST-снапшот 48K: из
`zx_src/kladov.snapshot` он по-прежнему собирает плоский `zx_bin/kladov.bin`
(49152 байта, адреса `0x4000..0xFFFF`). Этот режим нужен только для старых
скриптов разбора в `kladov/`.

Использование:
    python3 tools/make_kladov_image.py [snapshot] [out]
"""
import os
import sys
import zlib

PAGE = 0x4000


def parse_szx(d):
    if d[:4] != b"ZXST":
        raise SystemExit("не ZXST-снапшот")
    off, chunks = 8, []
    while off + 8 <= len(d):
        cid = d[off:off + 4]
        ln = int.from_bytes(d[off + 4:off + 8], "little")
        chunks.append((cid, d[off + 8:off + 8 + ln]))
        off += 8 + ln
    return chunks


def make_zxst(d, path):
    pages = {}
    for cid, body in parse_szx(d):
        if cid != b"RAMP":
            continue
        page = body[2]
        zi = body.find(b"\x78\xda")
        pages[page] = zlib.decompress(body[zi:]) if zi >= 0 else body[3:]
    for p in (5, 2, 0):
        if p not in pages:
            raise SystemExit("в снапшоте %s нет страницы %d" % (path, p))
    image = pages[5] + pages[2] + pages[0]
    if len(image) != 3 * PAGE:
        raise SystemExit("неожиданный размер образа: %d" % len(image))
    return image, None


def make_sna128(d, path):
    # SNA 128: 27-байтовый заголовок, банки 5, 2 и текущий, PC, 0x7FFD,
    # флаг TR-DOS и оставшиеся банки в возрастающем порядке.
    if len(d) != 27 + 8 * PAGE + 4:
        raise SystemExit("неожиданный размер SNA-снапшота: %d" % len(d))
    pc = int.from_bytes(d[49179:49181], "little")
    page_reg = d[49181] & 0x1F
    bank = page_reg & 7
    pages = {
        5: d[27:27 + PAGE],
        2: d[27 + PAGE:27 + 2 * PAGE],
        bank: d[32795:49179],
    }
    rest = [p for p in range(8) if p not in pages]
    tail = d[49183:]
    if len(tail) != len(rest) * PAGE:
        raise SystemExit("не хватает банков в SNA-снапшоте")
    for i, p in enumerate(rest):
        pages[p] = tail[i * PAGE:(i + 1) * PAGE]
    return b"".join(pages[p] for p in range(8)), page_reg


def main():
    snap = sys.argv[1] if len(sys.argv) > 1 else "zx_src/kladov128.snapshot"
    out = sys.argv[2] if len(sys.argv) > 2 else "zx_bin/kladov128.bin"
    d = open(snap, "rb").read()
    if d[:4] == b"ZXST":
        image, page_reg = make_zxst(d, snap)
    else:
        image, page_reg = make_sna128(d, snap)

    os.makedirs(os.path.dirname(out) or ".", exist_ok=True)
    open(out, "wb").write(image)
    if page_reg is None:
        print("записан %s (%d байт, 48K)" % (out, len(image)))
    else:
        print(
            "записан %s (%d байт, 128K, 0x7FFD=0x%02X, PC=0x%04X)"
            % (out, len(image), page_reg, int.from_bytes(d[49179:49181], "little"))
        )


if __name__ == "__main__":
    main()
