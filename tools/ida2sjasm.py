#!/usr/bin/env python3
"""Convert IDA (Zilog macro assembler) listing into sjasmplus-compatible source.

Handles the constructs IDA emits for these dumps:
  * struc/ends type definitions (dropped)
  * ShipData <...> / SlotObj <...> struct initialisers -> db/dw stream
  * `.ascii 'TEXT',0` -> db with raw bytes (CP866 aware)
  * `segment RAM` / `; end of 'RAM'` (dropped)
  * backslash line continuations (IDA puts the XREF comment on the tail line)
  * `<label>.field_NN` struct member accesses -> `<label>+0xNN`
  * CP866 -> UTF-8 text
"""
import re
import sys


def read_text(path):
    data = open(path, "rb").read()
    for enc in ("cp866", "latin-1"):
        try:
            return data.decode(enc)
        except UnicodeDecodeError:
            continue
    return data.decode("latin-1")


def split_comment(line):
    in_str = False
    for i, ch in enumerate(line):
        if ch == "'":
            in_str = not in_str
        elif ch == ";" and not in_str:
            return line[:i], line[i:]
    return line, ""


def logical_lines(raw_lines):
    out = []
    buf = ""
    for raw in raw_lines:
        code, comment = split_comment(raw.rstrip("\n"))
        if not buf:
            first_comment = comment
        if code.rstrip().endswith("\\"):
            buf += code.rstrip()[:-1] + " "
            continue
        # A comment-only line can sit *inside* a backslash continuation (IDA
        # sometimes puts the XREF on its own line between two code fragments).
        # Don't flush the open continuation on such a line.
        if buf and not code.strip():
            continue
        line = buf + code
        buf = ""
        out.append((line, first_comment))
    if buf:
        out.append((buf, ""))
    return out


STRUCT_FIELDS = {
    "ShipData": ["b", "b", "b", "b", "b", "b", "b", "b", "w", "b", "b", "w", "w", "w", "b", "b", "b", "w"],
    "SlotObj": ["b", "b", "b", "b", "b", "b", "b", "b", "b", "b", "b", "b", "b", "b", "b", "b", "b", "b", "b", "b",
                "b", "b", "b", "b", "b", "w", "b", "b"],
}


def strip_label(code):
    m = re.match(r"^([A-Za-z_@.$?][\w@.$?]*):\s*(.*)$", code)
    if m:
        return m.group(1) + ":", m.group(2)
    return None, code


def parse_values(s):
    return [x.strip() for x in s.split(",") if x.strip() != ""]


def struct_to_bytes(vals, layout):
    while len(vals) < len(layout):
        vals.append("0")
    out = []
    for k, v in zip(layout, vals):
        v = v.strip() or "0"
        out.append(("db" if k == "b" else "dw") + " " + v)
    return out


def ascii_to_db(s):
    parts = parse_values(s)
    res = []
    for p in parts:
        p = p.strip()
        if not p:
            continue
        if p.startswith("'"):
            body = p.strip("'")
            res.extend(str(b) for b in body.encode("cp866", "replace"))
        else:
            res.append(p)
    return "db " + ", ".join(res)


FIELD_RE = re.compile(r"\b([A-Za-z_@.$?][\w@.$?]*)\.field_([0-9A-Fa-f]+)\b")
LABEL_RE = re.compile(r"^([A-Za-z_@.$?][\w@.$?]*):")
IDX_RE = re.compile(r"\((ix|iy)\s*\+\s*([0-9A-Fa-f]+)h\s*\)", re.I)


def rewrite_indexed(text):
    def repl(m):
        reg = m.group(1)
        val = int(m.group(2), 16)
        if val >= 0x80:
            return "(%s-%d)" % (reg, 256 - val)
        return "(%s+%d)" % (reg, val)
    return IDX_RE.sub(repl, text)


def rewrite_fields(text):
    text = FIELD_RE.sub(lambda m: "%s+0x%s" % (m.group(1), m.group(2)), text)
    return rewrite_indexed(text)


def unique_lines(pairs):
    lines = [p[0] for p in pairs]
    fixed = unique_names(lines)
    return [(fixed[i], pairs[i][1]) for i in range(len(pairs))]


def unique_names(lines):
    """IDA can reuse a local label name.  Rename 2nd+ occurrences and redirect
    every reference to the nearest preceding definition (or the first one for
    forward references)."""
    defs = {}
    for idx, line in enumerate(lines):
        m = LABEL_RE.match(line.lstrip())
        if m:
            defs.setdefault(m.group(1), []).append(idx)
    dups = {name: idxs for name, idxs in defs.items() if len(idxs) > 1}
    if not dups:
        return lines
    rename = {}
    for name, idxs in dups.items():
        for k, idx in enumerate(idxs):
            rename[(name, k)] = name if k == 0 else "%s__%d" % (name, k + 1)
    out = []
    for idx, line in enumerate(lines):
        stripped = line.lstrip()
        indent = line[: len(line) - len(stripped)]
        m = LABEL_RE.match(stripped)
        label = None
        body = stripped
        if m:
            label = m.group(1)
            body = stripped[m.end():]
            if label in dups:
                k = dups[label].index(idx)
                label = rename[(label, k)]
        # replace references to duplicated names
        for name, idxs in dups.items():
            if name not in body:
                continue
            # choose the nearest definition (IDA local labels are used locally)
            chosen = min(range(len(idxs)), key=lambda k: (abs(idxs[k] - idx), idxs[k] > idx))
            newname = rename[(name, chosen)]
            if newname != name:
                body = re.sub(r"\b%s\b" % re.escape(name), newname, body)
        if label is not None:
            out.append("%s%s:%s" % (indent, label, body))
        else:
            out.append(indent + body)
    return out


def convert(src_path, out_path):
    text = read_text(src_path)
    llines = logical_lines(text.splitlines())

    out = []
    in_struct = False
    filtered = []
    for line, comment in llines:
        if re.match(r"^\s*\w+\s+struc\b", line):
            in_struct = True
            continue
        if in_struct:
            if re.match(r"^\s*\w+\s+ends\b", line):
                in_struct = False
            continue
        filtered.append((line, comment))

    filtered = unique_lines(filtered)

    for line, comment in filtered:
        stripped = line.strip()
        if not stripped:
            # IDA "collapsed" functions hide their 3-byte body behind a comment.
            # They are always `jp <target>` thunks named `j_<target>`.
            m = re.search(r"COLLAPSED FUNCTION ([A-Za-z_][A-Za-z0-9_]*)", comment)
            if m:
                name = m.group(1)
                target = name[2:] if name.startswith("j_") else name
                out.append("%s: jp %s" % (name, target))
            continue
        if re.match(r"^segment\b", stripped, re.I):
            continue
        if re.match(r"^end\s*$", stripped, re.I):
            out.append("\tend")
            break

        label, rest = strip_label(line)
        rest_s = rest.strip()

        m = re.match(r"^org\s+([0-9A-Fa-f]+)h?\s*$", rest_s, re.I)
        if m and label is None:
            out.append("\torg %04Xh" % int(m.group(1), 16))
            continue

        m = re.match(r"^(\w+)\s*<([^>]*)>\s*$", rest_s)
        if m and m.group(1) in STRUCT_FIELDS:
            vals = parse_values(m.group(2))
            fields = struct_to_bytes(vals, STRUCT_FIELDS[m.group(1)])
            for i, f in enumerate(fields):
                prefix = (label + " ") if (label and i == 0) else "\t"
                out.append("%s\t%s" % (prefix, f))
            continue

        m = re.match(r"^\.ascii\s+(.*)$", rest_s)
        if m:
            prefix = (label + " ") if label else "\t"
            out.append("%s\t%s" % (prefix, ascii_to_db(m.group(1))))
            continue

        # IDA repeat-data syntax:  db [ count ], value  /  db [ count ]
        m = re.match(r"^db\s+\[\s*([0-9A-Fa-f]+)h?\s*\]\s*,\s*(.+)$", rest_s)
        if m:
            prefix = (label + " ") if label else "\t"
            out.append("%s\tdefs %sh, %s" % (prefix, m.group(1), m.group(2)))
            continue
        m = re.match(r"^db\s+\[\s*([0-9A-Fa-f]+)h?\s*\]\s*$", rest_s)
        if m:
            prefix = (label + " ") if label else "\t"
            out.append("%s\tdefs %sh" % (prefix, m.group(1)))
            continue

        # IDA 'dl' = 4-byte value
        m = re.match(r"^dl\s+(.+)$", rest_s)
        if m:
            prefix = (label + " ") if label else "\t"
            out.append("%s\tdw %s, %s" % (prefix, m.group(1), m.group(1)))
            continue

        prefix = (label + " ") if label else "\t"
        body = rewrite_fields(rest_s)
        out.append(("%s\t%s" % (prefix, body)) if body else prefix)

    with open(out_path, "w") as f:
        f.write("\n".join(out) + "\n")
    print("wrote %s: %d lines" % (out_path, len(out)))


if __name__ == "__main__":
    convert(sys.argv[1], sys.argv[2])
