#!/usr/bin/env python3
"""Dump the screen (and optionally RAM) from a Fuse .szx (ZXST) snapshot.

Usage:
    python3 tools/szxdump.py <snapshot> <out.png|out.ppm> [--ram DIR]

Fuse stores RAM pages zlib-compressed; each RAM page chunk has the layout
<u16 flag=1><u8 page><zlib stream>.  Page 5 is the screen area (0x4000-0x7FFF)
for the standard mapping.

Saves a PNG if Pillow is available, otherwise a PPM (P6).
"""
import sys
import zlib

# Standard ZX Spectrum palette, matching src/machine.rs (`ZX_PALETTE`).
PALETTE = [
    (0x00, 0x00, 0x00), (0x00, 0x00, 0xD7), (0xD7, 0x00, 0x00), (0xD7, 0x00, 0xD7),
    (0x00, 0xD7, 0x00), (0x00, 0xD7, 0xD7), (0xD7, 0xD7, 0x00), (0xD7, 0xD7, 0xD7),
    (0x00, 0x00, 0x00), (0x00, 0x00, 0xFF), (0xFF, 0x00, 0x00), (0xFF, 0x00, 0xFF),
    (0x00, 0xFF, 0x00), (0x00, 0xFF, 0xFF), (0xFF, 0xFF, 0x00), (0x00, 0xFF, 0xFF),
]


def parse_szx(path):
    d = open(path, "rb").read()
    if d[:4] != b"ZXST":
        raise SystemExit("not a ZXST snapshot")
    off, chunks = 8, []
    while off + 8 <= len(d):
        cid = d[off:off + 4]
        ln = int.from_bytes(d[off + 4:off + 8], "little")
        chunks.append((cid, d[off + 8:off + 8 + ln]))
        off += 8 + ln
    return (d[4], d[5], d[6], d[7]), chunks


def ram_pages(chunks):
    out = {}
    for cid, body in chunks:
        if cid != b"RAMP":
            continue
        page = body[2]
        zi = body.find(b"\x78\xda")
        data = zlib.decompress(body[zi:]) if zi >= 0 else body[3:]
        out[page] = data
    return out


def registers(chunks):
    for cid, body in chunks:
        if cid == b"Z80R":
            return {
                "A": body[0], "F": body[1],
                "BC": int.from_bytes(body[2:4], "little"),
                "DE": int.from_bytes(body[4:6], "little"),
                "HL": int.from_bytes(body[6:8], "little"),
                "IX": int.from_bytes(body[16:18], "little"),
                "IY": int.from_bytes(body[18:20], "little"),
                "SP": int.from_bytes(body[20:22], "little"),
                "PC": int.from_bytes(body[22:24], "little"),
                "I": body[24], "R": body[25],
            }
    return {}


def screen_rgb(page):
    px = bytearray(256 * 192 * 3)
    for y in range(192):
        base = ((y & 0xC0) << 5) + ((y & 0x07) << 8) + ((y & 0x38) << 2)
        for x in range(256):
            byte = page[base + (x >> 3)]
            on = byte & (0x80 >> (x & 7))
            attr = page[0x1800 + (y // 8) * 32 + x // 8]
            idx = ((attr & 7) if on else ((attr >> 3) & 7)) | (8 if attr & 0x40 else 0)
            i = (y * 256 + x) * 3
            px[i:i + 3] = bytes(PALETTE[idx])
    return px


def save(path, px):
    try:
        from PIL import Image
        Image.frombytes("RGB", (256, 192), bytes(px)).save(path)
        return
    except ImportError:
        pass
    if not path.endswith((".ppm", ".PPM")):
        path += ".ppm"
    with open(path, "wb") as f:
        f.write(b"P6\n256 192\n255\n" + bytes(px))


def main():
    args = [a for a in sys.argv[1:] if not a.startswith("--")]
    if len(args) < 2:
        raise SystemExit(__doc__)
    snap, out = args[0], args[1]
    ram_dir = sys.argv[sys.argv.index("--ram") + 1] if "--ram" in sys.argv else None

    (major, minor, machine, flags), chunks = parse_szx(snap)
    print(f"ZXST {major}.{minor} machine={machine} flags={flags}")

    regs = registers(chunks)
    if regs:
        print("regs: " + " ".join(f"{k}=0x{v:04X}" if isinstance(v, int) and (v > 255 or k in ("PC", "SP"))
                                   else f"{k}=0x{v:02X}" for k, v in regs.items()))

    pages = ram_pages(chunks)
    print("RAM pages: " + ", ".join(f"{p}({len(d)})" for p, d in sorted(pages.items())))

    if ram_dir:
        import os
        os.makedirs(ram_dir, exist_ok=True)
        for p, d in pages.items():
            open(os.path.join(ram_dir, f"page{p:02d}.bin"), "wb").write(d)

    if 5 not in pages:
        raise SystemExit("page 5 (screen) missing")
    save(out, screen_rgb(pages[5]))
    print(f"saved {out}")


if __name__ == "__main__":
    main()
