#!/usr/bin/env python3
# -*- coding: utf-8 -*-
"""Извлечение 3D-моделей объектов из образов игры (Elite / Кладов).

Модели лежат в ROM по таблице `AdrDataShips` (0x6180): запись 4 байта —
`[tag][ptr][category]`, `ptr` — адрес блока `ShipData` (см. `ShipData struc`
в начале kladov.asm):

    +0x00  dCargo      грузовой отсек
    +0x01  dCollision  дистанция столкновения по X/Y/Z
    +0x02  dChanceFire шанс выстрела из лазера
    +0x03  dVertexN    кол-во вершин
    +0x04  dEdgeN      кол-во рёбер
    +0x05  dNormalN    кол-во нормалей (граней)
    +0x06  dMaxSpeed   максимальная скорость
    +0x07  dEnergy     максимальная энергия
    +0x08  dCash       dw денежный приз за уничтожение
    +0x0A  dFlg_1      что может запустить
    +0x0B  dSize       геом. размер = расстояние, после которого объект точка
    +0x0C  dVertex     dw смещение до вершин   (от начала блока)
    +0x0E  dEdge       dw смещение до рёбер
    +0x10  dNormal     dw смещение до нормалей
    +0x12  dFlg_2      флаги
    +0x13  dFactor     коэффициент нормалей (деление при повороте)
    +0x14  dLaser      номер вершины с лазером
    +0x15  dRating     dw очки к рейтингу

Вершина — 6 байт: x, y, z (знаковые), байт детализации (младшие 5 бит —
макс. дальность отрисовки, 0x1F = всегда, старшие 3 бита — признаки) и два
байта сдвоенных номеров граней (полубайт = индекс грани): вершина считается
видимой, если видна хотя бы одна из перечисленных граней. Полубайт `0xF` —
сентинел «всегда видима»: массив `VergesVisible` — 16 байт, инициализирован
единицами, а нормалей у моделей не больше 14, так что 15 никогда не пишется
(на нём сидят контейнеры без граней вроде сплавов).

Ребро — 4 байта: байт детализации (0x1F = всегда), две вершины, байт сдв.
номеров граней (рёбра рисуются только при видимой грани).

Нормаль (грань) — 4 байта: x, y, z (знаковые) и байт: младшие 5 бит —
макс. дальность, старшие 3 — признаки (бит 7 меняет знак фронтальной теста).

Смещения внутри блока могут указывать на чужой блок — так модели
разделяют геометрию (у Elite «отшельник» держит только шапку, координаты
берёт у астероида).

Использование:
    python3 tools/extract_ships.py --json tmp/ships.json
    python3 tools/extract_ships.py --rust src/shipdata.rs
"""

import argparse
import json
import pathlib
import re
import sys

ROOT = pathlib.Path(__file__).resolve().parent.parent

# Образы: elite.bin — «сырой» образ загрузки в 0x6048, kladov.bin — 48К RAM
# с 0x4000 (см. tools/make_kladov_image.py).
GAMES = {
    "elite": {"bin": "zx_bin/elite.bin", "asm": "zx_src/elite.asm", "base": 0x6048},
    "kladov": {"bin": "zx_bin/kladov.bin", "asm": "zx_src/kladov.asm", "base": 0x4000},
}
TABLE_ADDR = 0x6180          # AdrDataShips, общий для обеих версий
HDR = 0x17                   # sizeof(ShipData)
VERTEX_SZ, EDGE_SZ, NORMAL_SZ = 6, 4, 4

STAT_FIELDS = [
    ("cargo", 1), ("collision", 1), ("fire_chance", 1),
    ("vertex_n", 1), ("edge_n", 1), ("normal_n", 1),
    ("max_speed", 1), ("energy", 1), ("cash", 2),
    ("launch", 1), ("size", 1),
    ("vertex_off", 2), ("edge_off", 2), ("normal_off", 2),
    ("flags", 1), ("factor", 1), ("laser", 1), ("rating", 2),
]


class Error(Exception):
    pass


def table_names(asm_path):
    """Имена блоков моделей в порядке записей таблицы `AdrDataShips`."""
    lines = asm_path.read_text(encoding="latin-1").splitlines()
    start = next((i for i, l in enumerate(lines) if l.startswith("AdrDataShips:")), None)
    if start is None:
        raise Error(f"{asm_path}: метка AdrDataShips не найдена")
    names = []
    for line in lines[start + 1:]:
        stripped = line.strip()
        if re.match(r"^[A-Za-z_][A-Za-z0-9_]*:", line):
            break                       # следующая метка — таблица кончилась
        m = re.match(r"^\s*dw\s+([A-Za-z_][A-Za-z0-9_]*)", line)
        if m:
            names.append(m.group(1))
        elif stripped and not stripped.startswith(";") and \
                not re.match(r"^\s*d[bwd]\s", line):
            break                       # код/директива — таблица кончилась
    if not names:
        raise Error(f"{asm_path}: записи AdrDataShips не найдены")
    return names


def signed(v):
    return v - 256 if v >= 128 else v


def decode_vertex(b):
    """Разбор 6 байт вершины.

    Координаты хранятся по модулю: байты 0..2 — |x|, |y|, |z|. Знаки
    закодированы в старших битах байта lod (b[3]): бит 7 — знак X, бит 6 —
    знак Y, бит 5 — знак Z (1 = отрицательная координата). Младшие 5 бит —
    порог дальности отрисовки (0x1F = всегда). Так же читает координаты
    Z80-рендер (`RotateCrdN_Vertex`: `b = (ix+0)`, знак — из `(ix+3)`).
    """
    lod = b[3]
    x, y, z = b[0], b[1], b[2]
    if lod & 0x80:
        x = -x
    if lod & 0x40:
        y = -y
    if lod & 0x20:
        z = -z
    return [x, y, z, lod & 0x1F, b[4], b[5]]


def decode_normal(b):
    """Разбор 4 байт нормали (грани).

    Компоненты хранятся по модулю, знаки — в старших битах 4-го байта
    (бит 7 — X, бит 6 — Y, бит 5 — Z), как у вершин. Младшие 5 бит —
    порог дальности; бит 7 в оригинале также меняет знак фронтального
    теста видимости грани (см. `Face::flags`).
    """
    flags = b[3]
    nx, ny, nz = b[0], b[1], b[2]
    if flags & 0x80:
        nx = -nx
    if flags & 0x40:
        ny = -ny
    if flags & 0x20:
        nz = -nz
    return [nx, ny, nz, flags]


def parse_ship(img, base, addr, name, cat):
    off = addr - base
    if off < 0 or off + HDR > len(img):
        raise Error(f"{name}: адрес {addr:#06x} вне образа")
    raw = img[off:off + HDR]
    stats, pos = {}, 0
    for key, width in STAT_FIELDS:
        value = int.from_bytes(raw[pos:pos + width], "little")
        stats[key] = value
        pos += width
    v, e, n = stats["vertex_n"], stats["edge_n"], stats["normal_n"]
    vo, eo, no = stats["vertex_off"], stats["edge_off"], stats["normal_off"]
    if max(v, e, n) > 64:
        raise Error(f"{name}: подозрительные счётчики v={v} e={e} n={n}")
    if v and e and eo != vo + v * VERTEX_SZ:
        raise Error(f"{name}: рёбра не следуют за вершинами: "
                    f"v@{vo:#x}+{v}*{VERTEX_SZ}!={eo:#x}")
    if e and n and no != eo + e * EDGE_SZ:
        raise Error(f"{name}: нормали не следуют за рёбрами: "
                    f"e@{eo:#x}+{e}*{EDGE_SZ}!={no:#x}")
    # При нулевых счётчиках смещение соответствующего блока может быть 0.
    end = no + n * NORMAL_SZ if n else (eo + e * EDGE_SZ if e else
                                        vo + v * VERTEX_SZ if v else HDR)
    if off + end > len(img):
        raise Error(f"{name}: блок выходит за образ (конец {addr + end:#06x})")

    vertices = []
    for i in range(v):
        b = img[off + vo + i * VERTEX_SZ:off + vo + (i + 1) * VERTEX_SZ]
        vertices.append(decode_vertex(b))
    edges = []
    for i in range(e):
        b = img[off + eo + i * EDGE_SZ:off + eo + (i + 1) * EDGE_SZ]
        edges.append([b[0], b[1], b[2], b[3]])
    faces = []
    for i in range(n):
        b = img[off + no + i * NORMAL_SZ:off + no + (i + 1) * NORMAL_SZ]
        faces.append(decode_normal(b))

    # --- проверки связности ---
    for k, (_, a, b2, _) in enumerate(edges):
        if a >= v or b2 >= v:
            raise Error(f"{name}: ребро {k} ссылается на вершину {max(a, b2)} "
                        f"при {v} вершинах")
    # Индексы граней (полубайты). `0xF` — сентинел «всегда видима»: массив
    # `VergesVisible` — 16 байт, инициализирован единицами, а перезаписываются
    # только первые `n` (максимум у моделей — 14), поэтому 15 всегда видна.
    if n:
        for k, b in enumerate(vertices):
            for packed in (b[4], b[5]):
                for nibble in (packed & 0x0F, packed >> 4):
                    if nibble >= n and nibble != 0xF:
                        raise Error(f"{name}: вершина {k}: грань {nibble} "
                                    f"при {n} гранях")
        for k, (_, _, _, packed) in enumerate(edges):
            for nibble in (packed & 0x0F, packed >> 4):
                if nibble >= n and nibble != 0xF:
                    raise Error(f"{name}: ребро {k}: грань {nibble} "
                                f"при {n} гранях")

    return {
        "name": name,
        "addr": addr,
        "category": cat,
        "size": end,
        "stats": stats,
        "vertices": vertices,
        "edges": edges,
        "faces": faces,
    }




# -------------------- Поддержка формата kladov --------------------

def decode_lod_rle(stream, need):
    """Распаковка RLE потока lod в kladov."""
    out = []
    i = 0
    while len(out) < need and i < len(stream):
        b = stream[i]; i += 1
        if (b & 0x20) == 0:
            cnt = b
            if i >= len(stream):
                break
            val = stream[i]; i += 1
            for _ in range(cnt):
                out.append(val)
                if len(out) >= need:
                    break
        else:
            out.append(b)
    return out


def parse_ship_kladov(img, base, addr, name, cat):
    """Парсер для формата Кладова (переструктурирован)."""
    off = addr - base
    if off < 0 or off + HDR > len(img):
        raise Error(f"{name}: адрес {addr:#06x} вне образа")
    raw = img[off:off + HDR]
    stats, pos = {}, 0
    for key, width in STAT_FIELDS:
        value = int.from_bytes(raw[pos:pos + width], "little")
        stats[key] = value
        pos += width
    v, e, n = stats["vertex_n"], stats["edge_n"], stats["normal_n"]
    vo, eo, no = stats["vertex_off"], stats["edge_off"], stats["normal_off"]
    if max(v, e, n) > 80:
        raise Error(f"{name}: подозрительные счётчики v={v} e={e} n={n}")
    
    S = addr  # начало ShipData
    # vo-блок (формат Кладова): сначала `v` записей "aux" по 3 байта
    # (lod + два полубайта граней — аналог полей Vertex в элитовском
    # формате), затем координаты по 3 байта. Координат в блоке может быть
    # меньше `v`: у больших кораблей совпадающие/мельчайшие точки хранятся
    # «свёрнуто», а лишние вершины ссылаются на уже имеющуюся координату.
    # Раньше здесь aux и coords были перепутаны местами, из-за чего в
    # координаты попадали служебные байты (например, `y = z = -103`).
    ncoords = (eo - vo) // 3 - v if eo > vo else 0
    if ncoords < 0:
        ncoords = 0
    aux_off = S + vo
    coords_off = S + vo + v * 3
    if coords_off - base + ncoords * 3 > len(img):
        raise Error(f"{name}: coords выходят за образ")
    if aux_off - base + v * 3 > len(img):
        raise Error(f"{name}: aux выходят за образ")
    if ncoords <= 0:
        raise Error(f"{name}: v={v}, но в блоке нет координат "
                    f"(vo={vo:#x}, eo={eo:#x})")

    vertices = []
    for i in range(v):
        # Лишние вершины (за пределами хранимых координат) используют
        # последнюю доступную точку — так исчезают «шипы» из служебных байт.
        ci = min(i, ncoords - 1)
        cx = img[coords_off - base + ci * 3]
        cy = img[coords_off - base + ci * 3 + 1]
        cz = img[coords_off - base + ci * 3 + 2]
        ax = img[aux_off - base + i * 3]
        ay = img[aux_off - base + i * 3 + 1]
        az = img[aux_off - base + i * 3 + 2]
        # Координаты — по модулю; знаки в старших битах aux-байта (см.
        # decode_vertex): бит 7 — X, бит 6 — Y, бит 5 — Z.
        vertices.append(decode_vertex([cx, cy, cz, ax, ay, az]))
    
    # Рёбра: edge-блок в eo
    eblock_off = eo  # относит к S? нет, eo = смещение от S
    eblock_mem = S + eo
    if eblock_mem - base + 4 > len(img):
        raise Error(f"{name}: edge-блок выходит за образ")
    ptr_lod = img[eblock_mem - base] | (img[eblock_mem + 1 - base] << 8)
    ptr_vtx = img[eblock_mem + 2 - base] | (img[eblock_mem + 3 - base] << 8)
    lod_stream = img[ptr_lod - base : ptr_lod - base + 256]
    de_stream = img[ptr_vtx - base : ptr_vtx - base + e]
    lod_vals = decode_lod_rle(lod_stream, e)
    
    edges = []
    ix = eblock_mem
    for k in range(e):
        if k >= len(lod_vals) or k >= len(de_stream):
            break
        lodbyte = lod_vals[k]
        lod = lodbyte & 0x1F
        rawv = de_stream[k]
        v1 = (rawv & 0x0F) | (0x10 if (lodbyte & 0x40) else 0)
        v2 = ((rawv >> 4) & 0x0F) | (0x10 if (lodbyte & 0x80) else 0)
        if ix + 5 - base < len(img) and ix+4 - base >= 0:
            f_raw = img[ix + 4 - base]
        else:
            f_raw = 0xFF
        faces = ((f_raw & 0xF) << 4) | (f_raw >> 4)
        edges.append([lod, v1, v2, faces])
        if ix + 5 - base < len(img) and img[ix + 5 - base] != 0xFF:
            ix += 1
    
    # Нормали: сначала flags[N], потом coords[3N] (координаты по модулю,
    # знаки — в старших битах байтов flags, см. decode_normal).
    n_off = S + no
    faces = []
    if n > 0 and n_off - base + n + n * 3 <= len(img):
        for i in range(n):
            flg = img[n_off - base + i]
            nx = img[n_off - base + n + i * 3]
            ny = img[n_off - base + n + i * 3 + 1]
            nz = img[n_off - base + n + i * 3 + 2]
            faces.append(decode_normal([nx, ny, nz, flg]))
    elif n > 0:
        for i in range(n):
            b = img[n_off - base + i * NORMAL_SZ:n_off - base + (i + 1) * NORMAL_SZ]
            faces.append(decode_normal(b))
    
    # проверки
    for k, (_, a, b2, _) in enumerate(edges):
        if a >= v or b2 >= v:
            raise Error(f"{name}: ребро {k} ссылается на вершину {max(a, b2)} при {v} вершинах")
    if n:
        for k, b in enumerate(vertices):
            for packed in (b[4], b[5]):
                for nibble in (packed & 0x0F, packed >> 4):
                    if nibble >= n and nibble != 0xF:
                        raise Error(f"{name}: вершина {k}: грань {nibble} при {n} гранях")
        for k, (_, _, _, packed) in enumerate(edges):
            for nibble in (packed & 0x0F, packed >> 4):
                if nibble >= n and nibble != 0xF:
                    raise Error(f"{name}: ребро {k}: грань {nibble} при {n} гранях")
    
    end = no + n + n * 3 if (n > 0) else eo
    return {
        "name": name,
        "addr": addr,
        "category": cat,
        "size": end,
        "stats": stats,
        "vertices": vertices,
        "edges": edges,
        "faces": faces,
    }


# Имена, под которыми одну и ту же геометрию называют разные версии.
ALIAS = {"dTargletH": "d_Targlet"}


def load(game):
    cfg = GAMES[game]
    img = (ROOT / cfg["bin"]).read_bytes()
    names = table_names(ROOT / cfg["asm"])
    base, table = cfg["base"], TABLE_ADDR - cfg["base"]
    ships = []
    for i, name in enumerate(names):
        rec = img[table + i * 4:table + i * 4 + 4]
        if len(rec) < 4:
            raise Error(f"{game}: таблица обрывается на записи {i}")
        ptr = rec[1] | rec[2] << 8
        parse = parse_ship_kladov if game == 'kladov' else parse_ship
        try:
            ships.append(parse(img, base, ptr, name, rec[3]))
        except Error as err:
            # В части кладовских записей смещения блоков указывают на общую
            # (разделяемую) геометрию, из-за чего координаты не читаются.
            # Подставляем геометрию того же корабля из элитовской таблицы —
            # корпус у версий одинаковый, а характеристики оставляем кладовские.
            if game != 'kladov':
                raise
            ref = find_elite_geometry(ALIAS.get(name, name))
            if ref is None:
                print(f"  предупреждение: {name}: {err}", file=sys.stderr)
                raise
            print(f"  {name}: смещения вне блока ({err}); "
                  f"геометрия взята из элитовской таблицы", file=sys.stderr)
            ship = dict(ref)
            ship["name"] = name
            ship["addr"] = ptr
            ship["category"] = rec[3]
            ship["stats"] = parse_kladov_header(img, base, ptr)
            ships.append(ship)
    return img, ships


_elite_cache = None


def find_elite_geometry(name):
    """Геометрия объекта с тем же именем из элитовской таблицы."""
    global _elite_cache
    if _elite_cache is None:
        img, ships = load("elite")
        _elite_cache = {s["name"]: s for s in ships}
    return _elite_cache.get(name)


def parse_kladov_header(img, base, addr):
    """Только шапка `ShipData` (характеристики) без геометрии."""
    off = addr - base
    if off < 0 or off + HDR > len(img):
        raise Error(f"адрес {addr:#06x} вне образа")
    raw = img[off:off + HDR]
    stats, pos = {}, 0
    for key, width in STAT_FIELDS:
        stats[key] = int.from_bytes(raw[pos:pos + width], "little")
        pos += width
    return stats

def to_rust(games):
    out = []
    w = out.append
    w("//! Автогенерированный файл — не править вручную.")
    w("//!")
    w("//! Извлечённые 3D-модели объектов игры из таблицы `AdrDataShips`.")
    w("//! Генерация: `python3 tools/extract_ships.py --rust src/shipdata.rs`.")
    w("//!")
    w("//! Координаты — знаковые байты локальной СКО корабля; `lod` — байт")
    w("//! детализации (младшие 5 бит = максимальная дальность отрисовки в")
    w("//! единицах 1/256, `0x1F` = всегда); `faces` — два сдвоенных индекса")
    w("//! граней (полубайт), видимость которых означает видимость элемента.")
    w("//!")
    w("//! Формат Кладова отличается: `aux`-записи вершины идут отдельным")
    w("//! блоком перед координатами, и координат может храниться меньше, чем")
    w("//! вершин (лишние вершины ссылаются на ближайшую сохранённую точку).")
    w("//! Для записей со смещениями на общую геометрию берётся корпус из")
    w("//! элитовской таблицы — см. `tools/extract_ships.py`.")
    w("")
    w("/// Вершина модели (6 байт в ROM).")
    w("#[derive(Clone, Copy, Debug)]")
    w("pub struct Vertex {")
    w("    pub x: i16, pub y: i16, pub z: i16,")
    w("    pub lod: u8,")
    w("    pub faces: [u8; 2],")
    w("}")
    w("")
    w("/// Ребро модели (4 байта в ROM): две вершины и сдвоенные индексы граней.")
    w("#[derive(Clone, Copy, Debug)]")
    w("pub struct Edge {")
    w("    pub lod: u8,")
    w("    pub v: [u8; 2],")
    w("    pub faces: u8,")
    w("}")
    w("")
    w("/// Грань модели: нормаль и байт детализации/признаков (4 байта в ROM).")
    w("#[derive(Clone, Copy, Debug)]")
    w("pub struct Face {")
    w("    pub nx: i16, pub ny: i16, pub nz: i16,")
    w("    pub flags: u8,")
    w("}")
    w("")
    w("/// Лётно-тактические характеристики из шапки `ShipData`.")
    w("#[derive(Clone, Copy, Debug)]")
    w("pub struct Stats {")
    w("    pub cargo: u8, pub collision: u8, pub fire_chance: u8,")
    w("    pub max_speed: u8, pub energy: u8, pub cash: u16,")
    w("    pub launch: u8, pub size: u8, pub flags: u8,")
    w("    pub factor: u8, pub laser: u8, pub rating: u16,")
    w("}")
    w("")
    w("/// Один объект игры: геометрия плюс характеристики.")
    w("#[derive(Clone, Copy, Debug)]")
    w("pub struct Ship {")
    w("    pub name: &'static str,")
    w("    /// Адрес блока в ROM версии, из которой извлечено.")
    w("    pub addr: u16,")
    w("    /// 4-й байт записи таблицы (категория).")
    w("    pub category: u8,")
    w("    pub stats: Stats,")
    w("    pub vertices: &'static [Vertex],")
    w("    pub edges: &'static [Edge],")
    w("    pub faces: &'static [Face],")
    w("}")
    w("")

    for game, ships in games.items():
        mod = "ELITE" if game == "elite" else "KLADOV"
        for ship in ships:
            ident_base = re.sub(r"[^A-Za-z0-9]", "_", ship["name"]).upper()
            ident = f"{mod}_{ident_base}"
            st = ship["stats"]
            w(f"/// `{ship['name']}` ({game}, адрес {ship['addr']:#06x}).")
            w(f"static SHIP_{ident}: Ship = Ship {{")
            w(f"    name: {json.dumps(ship['name'])},")
            w(f"    addr: {ship['addr']:#06x},")
            w(f"    category: {ship['category']},")
            w("    stats: Stats {")
            w(f"        cargo: {st['cargo']}, collision: {st['collision']}, "
              f"fire_chance: {st['fire_chance']},")
            w(f"        max_speed: {st['max_speed']}, energy: {st['energy']}, "
              f"cash: {st['cash']},")
            w(f"        launch: {st['launch']}, size: {st['size']}, "
              f"flags: {st['flags']},")
            w(f"        factor: {st['factor']}, laser: {st['laser']}, "
              f"rating: {st['rating']},")
            w("    },")
            w("    vertices: &[")
            for x, y, z, lod, f1, f2 in ship["vertices"]:
                w(f"        Vertex {{ x: {x}, y: {y}, z: {z}, lod: {lod:#x}, "
                  f"faces: [{f1}, {f2}] }},")
            w("    ],")
            w("    edges: &[")
            for lod, a, b, faces in ship["edges"]:
                w(f"        Edge {{ lod: {lod:#x}, v: [{a}, {b}], "
                  f"faces: {faces:#x} }},")
            w("    ],")
            w("    faces: &[")
            for nx, ny, nz, flags in ship["faces"]:
                w(f"        Face {{ nx: {nx}, ny: {ny}, nz: {nz}, "
                  f"flags: {flags:#x} }},")
            w("    ],")
            w("};")
            w("")
        refs = ", ".join(
            f"SHIP_{mod}_{re.sub(r'[^A-Za-z0-9]', '_', s['name']).upper()}"
            for s in ships)
        w(f"/// Модели версии `{GAMES[game]['bin']}` ({len(ships)} шт.).")
        w(f"pub static {mod}_SHIPS: &[Ship] = &[{refs}];")
        w("")
    w("/// Найти модель по внутреннему имени (`kladov` — какая версия в приоритете).")
    w("pub fn find(name: &str, kladov: bool) -> Option<&'static Ship> {")
    w("    let prefer = if kladov { KLADOV_SHIPS } else { ELITE_SHIPS };")
    w("    let other = if kladov { ELITE_SHIPS } else { KLADOV_SHIPS };")
    w("    prefer")
    w("        .iter()")
    w("        .chain(other.iter())")
    w("        .find(|s| s.name == name)")
    w("}")
    return "\n".join(out)


def main():
    ap = argparse.ArgumentParser(description=__doc__.splitlines()[0])
    ap.add_argument("--json", metavar="PATH", help="выгрузить всё в JSON")
    ap.add_argument("--rust", metavar="PATH", help="сгенерировать Rust-модуль")
    ap.add_argument("--game", choices=sorted(GAMES), help="только одна версия")
    args = ap.parse_args()

    games = {}
    for game in (GAMES if args.game is None else [args.game]):
        img, ships = load(game)
        games[game] = ships
        print(f"{game}: {len(ships)} объектов")
        for s in ships:
            st = s["stats"]
            print(f"  {s['name']:<14} {s['addr']:#06x} cat={s['category']} "
                  f"разм={s['size']:#05x} V={st['vertex_n']:>3} "
                  f"E={st['edge_n']:>3} N={st['normal_n']:>3} "
                  f"скор={st['max_speed']:>3} энерг={st['energy']:>3} "
                  f"приз={st['cash']:>5} лазер@{st['laser']}")

    if args.json:
        path = pathlib.Path(args.json)
        path.parent.mkdir(parents=True, exist_ok=True)
        payload = {g: ships for g, ships in games.items()}
        path.write_text(json.dumps(payload, indent=1, ensure_ascii=False))
        print(f"json -> {path}")
    if args.rust:
        path = pathlib.Path(args.rust)
        path.write_text(to_rust(games) + "\n", encoding="utf-8")
        print(f"rust -> {path}")
    return 0


if __name__ == "__main__":
    try:
        sys.exit(main())
    except Error as exc:
        print(f"ошибка: {exc}", file=sys.stderr)
        sys.exit(1)
