#!/usr/bin/env python3
"""PPM (P6) -> PNG без внешних зависимостей (zlib + struct)."""
import sys
import zlib
import struct


def read_ppm(path):
    data = open(path, 'rb').read()
    # Разбор заголовка: P6, ширина, высота, maxval (с пробелами/комментариями).
    assert data[:2] == b'P6'
    pos = 2
    vals = []
    while len(vals) < 3:
        while data[pos:pos + 1].isspace():
            pos += 1
        if data[pos:pos + 1] == b'#':
            while data[pos:pos + 1] not in (b'\n', b''):
                pos += 1
            continue
        start = pos
        while not data[pos:pos + 1].isspace():
            pos += 1
        vals.append(int(data[start:pos]))
    pos += 1
    w, h, _ = vals
    px = data[pos:pos + w * h * 3]
    return w, h, px


def write_png(path, w, h, rgb):
    raw = bytearray()
    for y in range(h):
        raw.append(0)
        raw += rgb[y * w * 3:(y + 1) * w * 3]

    def chunk(tag, data):
        c = struct.pack('>I', len(data)) + tag + data
        return c + struct.pack('>I', zlib.crc32(tag + data) & 0xFFFFFFFF)

    png = b'\x89PNG\r\n\x1a\n'
    png += chunk(b'IHDR', struct.pack('>IIBBBBB', w, h, 8, 2, 0, 0, 0))
    png += chunk(b'IDAT', zlib.compress(bytes(raw), 9))
    png += chunk(b'IEND', b'')
    open(path, 'wb').write(png)


if __name__ == '__main__':
    src, dst = sys.argv[1], sys.argv[2]
    w, h, px = read_ppm(src)
    write_png(dst, w, h, px)
    print(f'{dst}: {w}x{h}')
