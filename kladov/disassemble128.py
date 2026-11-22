#!/usr/bin/env python3
"""Перегенерация полного листинга kladov128.bin с сохранением разбора kladov.asm.

Новая основа — zx_bin/kladov128.bin (8 банков по 16 КБ).
Старый zx_src/kladov.asm используется только как источник имён, меток и
комментариев: для байтов, которые не изменились, строки IDA переносятся
дословно. Изменившиеся и ранее не разобранные участки дизассемблируются
заново через z80dis.

Требуется: python3 -m pip install --user z80dis
"""

from __future__ import annotations

import re
import sys
from dataclasses import dataclass, field
from pathlib import Path

try:
    from z80dis.z80 import decode, decoded2str
except ImportError:  # pragma: no cover
    sys.exit("Нужен z80dis: python3 -m pip install --user z80dis")

ROOT = Path(__file__).resolve().parent.parent
OLD_ASM = ROOT / "zx_src" / "kladov.asm"
OLD_BIN = ROOT / "zx_bin" / "kladov.bin"
NEW_BIN = ROOT / "zx_bin" / "kladov128.bin"
OUT = ROOT / "kladov" / "kladov128.asm"

# Соответствие страниц ibm PC эмулятора адресам Z80.
OLD_PAGES = [(0x4000, 0), (0x8000, 1), (0xC000, 2)]
NEW_BANKS = [(0x4000, 5), (0x8000, 2), (0xC000, 0)]
EXTRA_BANKS = [(1, 0x4000), (3, 0x4000), (4, 0x4000), (6, 0x4000), (7, 0x4000)]


# ---------------------------------------------------------------------------
# Определение длины инструкций по тексту IDA.
# Длины берутся именно из текста, а не из kladov.bin: старый .asm и старый
# .bin расходятся в нескольких местах, из-за чего сдвигались все метки.
# ---------------------------------------------------------------------------

MEM_RE = re.compile(r"^\((.*)\)$")


def split_ops(s: str) -> list[str]:
    parts: list[str] = []
    depth = 0
    cur = ""
    inq = False
    for ch in s:
        if ch == '"':
            inq = not inq
        if not inq:
            if ch == "(":
                depth += 1
            elif ch == ")":
                depth -= 1
        if ch == "," and depth == 0 and not inq:
            parts.append(cur.strip())
            cur = ""
        else:
            cur += ch
    if cur.strip():
        parts.append(cur.strip())
    return parts


def classify(op: str) -> str:
    o = op.strip().lower()
    if o in ("a", "b", "c", "d", "e", "h", "l", "f", "ixh", "ixl", "iyh",
             "iyl", "xh", "xl", "yh", "yl", "i", "r", "sp", "af", "bc",
             "de", "hl", "ix", "iy"):
        return "reg"
    if o in ("(bc)", "(de)", "(hl)", "(sp)"):
        return "mem_reg"
    m = MEM_RE.match(o)
    if m:
        inner = m.group(1).strip()
        if re.search(r"\b(ix|iy)\s*[+-]", inner) or re.match(r"^[+-]", inner):
            return "mem_idx"
        return "mem_addr"
    if re.search(r"\b(ix|iy)\s*[+-]", o):
        return "mem_idx"
    imm = r"[0-9a-f]+h|[01]+b|\d+|\'\w\'|\"\w+\""
    if re.fullmatch(imm, o):
        return "imm"
    if (o.startswith(("loc_", "sub_", "j_"))
            or re.fullmatch(r"[A-Za-z_][A-Za-z0-9_]*([+-]\d+)?(\.[A-Za-z0-9_]+)?", o)):
        return "imm_addr"
    return "other"


ONEBYTE_NOPARAM = {
    "nop", "ret", "exx", "di", "ei", "halt", "cpl", "daa", "scf", "ccf",
    "rla", "rra", "rlca", "rrca",
}
TWOBYTE_NOPARAM = {
    "reti", "retn", "neg", "ldi", "ldd", "ldir", "lddr", "cpi", "cpd",
    "cpir", "cpdr", "ini", "ind", "inir", "indr", "outi", "outd", "otir",
    "otdr",
}
CONDITIONS = {"nz", "z", "nc", "c", "po", "pe", "p", "m"}


def insn_len(line: str) -> int:
    s = line.strip()
    q = False
    for i, ch in enumerate(s):
        if ch == '"':
            q = not q
        if ch == ";" and not q:
            s = s[:i].strip()
            break
    if not s:
        return 0
    lm = re.match(r"^([A-Za-z_][A-Za-z0-9_]*):\s*(.*)$", s)
    if lm:
        s = lm.group(2).strip()
    if not s:
        return 0
    dm = re.match(r"^(db|dw|dd|ds)\s+(.*)$", s, re.I)
    if dm:
        d = dm.group(1).lower()
        arg = dm.group(2).strip()
        if d == "ds":
            mm = re.match(r"^(\d+)", arg)
            return int(mm.group(1)) if mm else 1
        dup = re.search(r"(\d+)\s*dup\s*\(", arg, re.I)
        if dup:
            return int(dup.group(1)) * {"db": 1, "dw": 2, "dd": 4}[d]
        return len([x for x in split_ops(arg) if x.strip()]) * {
            "db": 1, "dw": 2, "dd": 4,
        }[d]
    if s.lower().startswith(("segment", "org", "ends")):
        return 0
    parts = s.split(None, 1)
    mnem = parts[0].lower()
    ops = split_ops(parts[1]) if len(parts) > 1 else []
    if not ops:
        return 2 if mnem in TWOBYTE_NOPARAM else 1
    o0 = ops[0].strip()
    c0 = classify(o0)

    if mnem in ("rlc", "rrc", "rl", "rr", "sla", "sra", "sll", "srl",
                "bit", "set", "res"):
        return 4 if c0 == "mem_idx" or any("ix" in o or "iy" in o for o in ops) else 2
    if mnem in ("jr", "djnz"):
        return 2
    if mnem == "rst":
        return 1
    if mnem in ("call", "jp"):
        if o0.lower() in CONDITIONS:
            return 3
        if c0 == "reg" or o0.lower() == "(hl)":
            return 1 if o0.lower() == "(hl)" else 2
        return 3
    if mnem == "ex":
        return 2 if any("ix" in o or "iy" in o for o in ops) else 1
    if mnem in ("push", "pop"):
        return 2 if "ix" in o0 or "iy" in o0 else 1
    if mnem in ("in", "out"):
        if c0 == "mem_addr":
            return 2
        if any("(c)" in o for o in ops):
            return 1
        return 2
    if (mnem in ("add", "adc", "sbc")
            and any("ix" in o or "iy" in o for o in ops)
            and len(ops) > 1 and classify(ops[1]) not in ("imm", "mem_idx")):
        return 2
    if mnem in ("adc", "sbc") and (
            any("ix" in o or "iy" in o for o in ops)
            or (len(ops) == 2 and o0.lower() in ("hl", "de", "bc", "sp")
                and classify(ops[1]) != "imm")):
        return 2
    if mnem in ("inc", "dec"):
        return 3 if c0 == "mem_idx" else 2 if ("ix" in o0 or "iy" in o0) else 1
    if mnem == "ld":
        if classify(ops[0]) == "mem_reg" and classify(ops[1]) == "imm":
            return 2
        if classify(ops[1]) in ("imm", "imm_addr") and o0.lower() in (
                "bc", "de", "hl", "sp", "ix", "iy", "af"):
            return 4 if o0.lower() in ("ix", "iy") else 3
        if classify(ops[1]) == "imm" and classify(ops[0]) == "mem_addr":
            return 3
        if any(classify(o) == "mem_idx" for o in ops):
            return 4 if (len(ops) == 2
                         and classify(ops[1]) in ("imm", "imm_addr")) else 3
        if o0.lower() in ("ix", "iy") and any(
                classify(o) == "mem_addr" for o in ops):
            return 4
        if c0 == "mem_addr" and o0.strip().startswith("(") and ops[1].strip().lower() in ("ix", "iy"):
            return 4
        if any(k in o for o in ops for k in ("ix", "iy", "xh", "xl", "yh", "yl")):
            return 2
        if len(ops) == 2 and classify(ops[1]) == "imm" and classify(ops[0]) != "reg":
            return 3
        if classify(ops[1]) == "imm":
            return 2
        if any(o.strip().lower() in ("i", "r", "ir") for o in ops) and c0 == "reg":
            return 2
        if c0 == "mem_addr" and ops[1].strip().lower() in ("bc", "de", "sp", "ix", "iy"):
            return 4
        if c0 == "reg" and o0.lower() in ("bc", "de", "sp") and any(
                classify(o) == "mem_addr" for o in ops):
            return 4
        if o0.lower() in ("ix", "iy") and c0 == "reg":
            return 2
        if o0.lower() in ("hl", "bc", "de") and c0 == "reg" and any(
                classify(o) == "mem_addr" for o in ops):
            return 3
        if c0 == "mem_addr" or any(classify(o) == "mem_addr" for o in ops):
            return 3
        return 1
    if mnem in ("add", "adc", "sub", "sbc", "and", "or", "xor", "cp"):
        if classify(ops[-1]) == "imm":
            return 2
        if c0 == "mem_idx" or any(classify(o) == "mem_idx" for o in ops):
            return 3
        if (len(ops) >= 2 and classify(ops[0]) == "reg"
                and o0.lower() in ("ix", "iy", "hl")):
            return 1
        if len(ops) == 2 and c0 == "reg" and classify(ops[1]) == "mem_reg":
            return 1
        if c0 in ("mem_reg", "mem_addr") or any(
                classify(o) in ("mem_reg", "mem_addr") for o in ops):
            return 1 if c0 == "mem_reg" else 3
        return 1
    return 1


# ---------------------------------------------------------------------------
# Разбор старого IDA-листинга.
# ---------------------------------------------------------------------------

@dataclass
class Record:
    addr: int
    length: int
    lines: list[str] = field(default_factory=list)
    kind: str = "insn"       # insn / data / struct / collapsed
    old_text: str = ""


LABEL_RE = re.compile(r"^([A-Za-z_][A-Za-z0-9_]*):\s*(.*)$")
COLLAPSED_RE = re.compile(r"\[(\d+) BYTES: COLLAPSED")


def parse_ida(path: Path) -> tuple[list[Record], dict[int, list[str]]]:
    lines = path.read_text(encoding="utf-8", errors="replace").splitlines()
    records: list[Record] = []
    labels: dict[int, list[str]] = {}
    addr = 0x5D00
    pending: list[str] = []
    in_struc = False
    i = 0
    while i < len(lines):
        line = lines[i]
        i += 1
        s = line.strip()
        if not s:
            continue
        if s.startswith(";"):
            m = COLLAPSED_RE.search(s)
            if m:
                n = int(m.group(1))
                records.append(Record(addr, n, pending + [line], "collapsed"))
                pending = []
                addr += n
            else:
                pending.append(line)
            continue
        if re.match(r"^(segment|org|ends)\b", s, re.I):
            continue
        m = re.match(r"^([A-Za-z_][A-Za-z0-9_]*)\s+(struc|ends)\b", s, re.I)
        if m:
            in_struc = m.group(2).lower() == "struc"
            continue
        if in_struc:
            continue

        lm = LABEL_RE.match(s)
        if lm:
            name = lm.group(1)
            hm = re.search(r"_([0-9A-Fa-f]{4,5})$", name)
            if hm:
                explicit = int(hm.group(1), 16)
                if 0x5D00 <= explicit < 0x10000 and explicit >= addr:
                    addr = explicit
            labels.setdefault(addr, []).append(name)
            # Метка может быть на одной строке с данными/инструкцией.
            if not lm.group(2).strip():
                pending.append(line)
                continue
            # Метка остаётся в строке; для расчёта длины берём только тело.
            s = lm.group(2).strip()

        kind = "insn"
        if re.match(r"^(db|dw|dd|ds)\b", s, re.I) or re.search(r":\s*(db|dw|dd|ds)\b", s, re.I):
            kind = "data"
        if re.match(r"^\w+:\s*\w+\s*<", s):
            kind = "struct"
        length = 0x17 if kind == "struct" else insn_len(s)
        block = pending + [line]
        if kind == "struct":
            while i < len(lines) and ">" not in lines[i - 1]:
                block.append(lines[i])
                i += 1
        elif re.match(r"^db\s+[0-9A-Fa-fx]+h?\s*,", s, re.I):
            # IDA иногда пишет несколько db подряд в одну строку.
            pass
        records.append(Record(addr, length, block, kind, line))
        pending = []
        addr += length
    return records, labels


# ---------------------------------------------------------------------------
# Байты и дизассемблирование.
# ---------------------------------------------------------------------------

OLD_BYTES = OLD_BIN.read_bytes()
NEW_BYTES = NEW_BIN.read_bytes()


def read_old(addr: int, n: int) -> bytes:
    for start, page in OLD_PAGES:
        if start <= addr and addr + n <= start + 0x4000:
            off = page * 0x4000 + addr - start
            return OLD_BYTES[off:off + n]
    return b""


def read_new(addr: int, n: int) -> bytes:
    for start, bank in NEW_BANKS:
        if start <= addr and addr + n <= start + 0x4000:
            off = bank * 0x4000 + addr - start
            return NEW_BYTES[off:off + n]
    return b""


def disasm(data: bytes, addr: int) -> tuple[str, int]:
    try:
        d = decode(data, addr)
        return decoded2str(d), max(1, d.len)
    except Exception:
        return "DB " + ", ".join(f"{b:02X}H" for b in data[:1]), 1


def hexbytes(data: bytes) -> str:
    return " ".join(f"{b:02X}" for b in data)


def db_lines(data: bytes, addr: int, prefix: str = "\t\tdb ") -> list[str]:
    out = []
    for off in range(0, len(data), 12):
        chunk = data[off:off + 12]
        out.append(f"{prefix}" + ", ".join(f"{b:02X}h" for b in chunk))
    return out


def emit_record(rec: Record, out: list[str]) -> tuple[int, bool]:
    """Добавляет одну запись в выход. Возвращает (размер, изменилось)."""
    old = read_old(rec.addr, rec.length)
    new = read_new(rec.addr, rec.length)
    changed = old != new
    if not changed:
        out.extend(rec.lines)
        out.append("")
        return rec.length, False

    # Сохраняем метки и комментарии, но заменяем тело.
    header = [l for l in rec.lines if l.lstrip().startswith(";")]
    labels_only = [l for l in rec.lines if LABEL_RE.match(l.strip())]
    body_old = rec.old_text.strip()
    out.extend(header)
    out.extend(labels_only)
    out.append(f"\t\t; ИЗМЕНЕНО {rec.addr:04X}h: было {hexbytes(old)}")
    if body_old:
        out.append(f"\t\t; БЫЛО: {body_old}")

    if rec.kind in ("data", "struct", "collapsed"):
        out.extend(db_lines(new, rec.addr))
        used = rec.length
    else:
        # Старая инструкция могла быть короче новой: берём запас байтов,
        # чтобы дизассемблер увидел полную команду.
        lookahead = read_new(rec.addr, max(8, rec.length))
        text, used = disasm(lookahead, rec.addr)
        out.append(f"\t\t{text}")
        if used < rec.length:
            out.extend(db_lines(lookahead[used:rec.length], rec.addr + used))
    out.append("")
    return used, True


def emit_gap(start: int, end: int, out: list[str]) -> None:
    if end <= start:
        return
    data = read_new(start, end - start)
    if not data:
        return
    out.append(f"\t\t; Неразобранный участок {start:04X}h-{end - 1:04X}h")
    for off in range(0, len(data), 12):
        chunk = data[off:off + 12]
        out.append("\t\tdb " + ", ".join(f"{b:02X}h" for b in chunk))
    out.append("")


def emit_extra_bank(bank: int, origin: int, out: list[str]) -> None:
    data = NEW_BYTES[bank * 0x4000:(bank + 1) * 0x4000]
    out.append(f"; ============== ДОПОЛНИТЕЛЬНЫЙ БАНК {bank} (org {origin:04X}h) ==============")
    out.append(f"\torg {origin:04X}h")
    addr = origin
    off = 0
    while off < len(data):
        text, used = disasm(data[off:off + 8], addr)
        out.append(f"; {addr:04X}h\t{text}\t; {hexbytes(data[off:off + used])}")
        addr += used
        off += used
    out.append("")


def main() -> None:
    records, labels = parse_ida(OLD_ASM)
    records.sort(key=lambda r: r.addr)

    out: list[str] = []
    out.extend([
        "; =============================================================",
        "; kladov128.asm — автоматически перегенерирован из kladov128.bin",
        "; =============================================================",
        "; Основа: zx_bin/kladov128.bin, 8 банков по 16 КБ.",
        "; Основные банки: bank 5 -> 4000h, bank 2 -> 8000h, bank 0 -> C000h.",
        "; Метки и комментарии сохранены из zx_src/kladov.asm для тех байтов,",
        "; которые не изменились; изменившиеся участки помечены «ИЗМЕНЕНО».",
        "; Дополнительные банки 1, 3, 4, 6, 7 приведены в конце файла.",
        "; =============================================================",
        "",
    ])

    changed_bytes = 0
    changed_records = 0
    cursor = 0x4000
    for rec in records:
        if rec.addr < cursor:
            continue
        emit_gap(cursor, rec.addr, out)
        used, changed = emit_record(rec, out)
        cursor = rec.addr + used
        if changed:
            changed_records += 1
            changed_bytes += used
    emit_gap(cursor, 0x10000, out)

    for bank, origin in EXTRA_BANKS:
        emit_extra_bank(bank, origin, out)

    OUT.write_text("\n".join(out) + "\n", encoding="utf-8")
    print(f"Записано: {OUT}")
    print(f"Меток сохранено: {sum(len(v) for v in labels.values())}")
    print(f"Изменённых записей: {changed_records}, байт: {changed_bytes}")
    print(f"Всего строк: {len(out)}")


if __name__ == "__main__":
    main()
