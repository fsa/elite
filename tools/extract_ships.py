#!/usr/bin/env python3
# -*- coding: utf-8 -*-
"""Извлечение 3D-моделей объектов из образа игры версии Кладова.

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

Нормаль (грань) — 4 байта: x, y, z (по модулю) и байт: младшие 5 бит —
макс. дальность, старшие 3 бита — знаки компонентов (бит 7 — X, бит 6 — Y,
бит 5 — Z). Отдельного признака «перевёрнутой» грани в формате нет:
видимость определяется знаком Z повёрнутой нормали.

Смещения внутри блока могут указывать на другой блок того же образа — так
модели разделяют геометрию (например, `d_Minerals` и `dTargletH` в версии
Кладова).

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

# Образ: kladov.bin — 48К RAM с 0x4000 (см. tools/make_kladov_image.py).
IMAGE = {"bin": "zx_bin/kladov.bin", "asm": "zx_src/kladov.asm", "base": 0x4000}
TABLE_ADDR = 0x6180          # AdrDataShips
HDR = 0x17                   # sizeof(ShipData)

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
    порог дальности. Признака «перевёрнутой» грани в формате нет: в
    оригинале видимость грани определяется знаком Z повёрнутой нормали
    (см. `src/splash.rs`, `face_visibility`).
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





# -------------------- Разбор формата kladov --------------------

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
    #
    # Блок координат заканчивается перед ближайшим следующим блоком
    # (`eo` или `no`). У записей с общей геометрией (`d_Minerals`,
    # `dTargletH`) `eo` указывает назад, на чужой edge-блок, поэтому
    # опираться на `eo - vo` нельзя: координаты всё равно лежат между
    # aux-блоком и нормалями.
    aux_off = S + vo
    coords_off = S + vo + v * 3
    following = [S + off for off in (eo, no) if off > vo]
    coords_end = min(following) if following else S + len(img) - base
    ncoords = (coords_end - coords_off) // 3
    if ncoords <= 0:
        raise Error(f"{name}: v={v}, но в блоке нет координат "
                    f"(vo={vo:#x}, eo={eo:#x}, no={no:#x})")
    if coords_off - base + ncoords * 3 > len(img):
        raise Error(f"{name}: coords выходят за образ")
    if aux_off - base + v * 3 > len(img):
        raise Error(f"{name}: aux выходят за образ")

    vertices = []
    coords = []
    ci = -1
    for i in range(v):
        ax = img[aux_off - base + i * 3]
        ay = img[aux_off - base + i * 3 + 1]
        az = img[aux_off - base + i * 3 + 2]
        # Координаты хранятся группами: новая тройка начинается у вершины,
        # где младший бит первого aux-байта установлен; остальные вершины
        # группы переиспользуют ту же тройку, меняя только знаки (старшие
        # биты aux). Сверка с элитовской таблицей: у всех общих моделей
        # наборы вершин совпадают, а не «шипы» из служебных байт.
        if ax & 1:
            ci += 1
            if ci >= ncoords:
                raise Error(f"{name}: координата {ci} не помещается до рёбер")
            pos = coords_off - base + ci * 3
            if pos + 3 > len(img):
                raise Error(f"{name}: координата {ci} выходит за образ")
            coords.append(tuple(img[pos:pos + 3]))
        if ci < 0:
            raise Error(f"{name}: вершина {i} идёт до первой координаты")
        cx, cy, cz = coords[ci]
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
    
    # Нормали: сначала n байтов flags, затем координаты по модулю (знаки —
    # в старших битах flags, см. decode_normal). Координаты, как и у вершин,
    # хранятся группами: новая тройка начинается там, где младший бит flags
    # установлен, остальные грани группы переиспользуют её со своими знаками.
    n_off = S + no
    faces = []
    coords = []
    ci = -1
    for i in range(n):
        pos = n_off - base + i
        if pos >= len(img):
            raise Error(f"{name}: нормаль {i} выходит за образ")
        flg = img[pos]
        if flg & 1:
            ci += 1
            cpos = n_off - base + n + ci * 3
            if cpos + 3 > len(img):
                raise Error(f"{name}: координата нормали {ci} выходит за образ")
            coords.append(tuple(img[cpos:cpos + 3]))
        if ci < 0:
            raise Error(f"{name}: нормаль {i} идёт до первой координаты")
        nx, ny, nz = coords[ci]
        faces.append(decode_normal([nx, ny, nz, flg]))
    
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


def load():
    img = (ROOT / IMAGE["bin"]).read_bytes()
    names = table_names(ROOT / IMAGE["asm"])
    base, table = IMAGE["base"], TABLE_ADDR - IMAGE["base"]
    ships = []
    for i, name in enumerate(names):
        rec = img[table + i * 4:table + i * 4 + 4]
        if len(rec) < 4:
            raise Error(f"таблица обрывается на записи {i}")
        ptr = rec[1] | rec[2] << 8
        ships.append(parse_ship_kladov(img, base, ptr, name, rec[3]))
    return img, ships


def to_rust(ships):
    """Rust-модуль с единственным набором моделей — версии Кладова."""
    out = []
    w = out.append
    w("//! Автогенерированный файл — не править вручную.")
    w("//!")
    w("//! Извлечённые 3D-модели объектов игры из таблицы `AdrDataShips`")
    w("//! версии Кладова (`zx_bin/kladov.bin`).")
    w("//! Генерация: `python3 tools/extract_ships.py --rust src/shipdata.rs`.")
    w("//!")
    w("//! Координаты — знаковые байты локальной СКО корабля; `lod` — байт")
    w("//! детализации (младшие 5 бит = максимальная дальность отрисовки в")
    w("//! единицах 1/256, `0x1F` = всегда); `faces` — два сдвоенных индекса")
    w("//! граней (полубайт), видимость которых означает видимость элемента.")
    w("//!")
    w("//! В формате Кладова вершины и нормали сжаты: общая координата хранится")
    w("//! один раз, а признак начала новой группы — младший бит первого байта")
    w("//! (у вершины это байт lod, у грани — flags). Знаки компонентов — в")
    w("//! старших трёх битах того же байта. Смещения блоков могут указывать")
    w("//! на общую геометрию (`d_Minerals`, `dTargletH`) — она читается из")
    w("//! того же образа Кладова.")
    w("//!")
    w("//! У граней (`Face`) 4-й байт — младшие 5 бит: порог дальности,")
    w("//! старшие три: знаки компонентов нормали (бит 7 — X, бит 6 — Y,")
    w("//! бит 5 — Z); сами знаковые компоненты уже в `nx`/`ny`/`nz`.")
    w("//! Признака «перевёрнутой» грани в формате нет — видимость считается")
    w("//! по знаку Z повёрнутой нормали (см. `src/splash.rs`).")
    w("")
    w("/// Вершина модели (6 байт в ROM).")
    w("#[derive(Clone, Copy, Debug, PartialEq, Eq)]")
    w("pub struct Vertex {")
    w("    pub x: i16, pub y: i16, pub z: i16,")
    w("    pub lod: u8,")
    w("    pub faces: [u8; 2],")
    w("}")
    w("")
    w("/// Ребро модели (4 байта в ROM): две вершины и сдвоенные индексы граней.")
    w("#[derive(Clone, Copy, Debug, PartialEq, Eq)]")
    w("pub struct Edge {")
    w("    pub lod: u8,")
    w("    pub v: [u8; 2],")
    w("    pub faces: u8,")
    w("}")
    w("")
    w("/// Грань модели: нормаль и байт детализации/знаков (4 байта в ROM).")
    w("///")
    w("/// В 4-м байте младшие 5 бит — порог дальности отрисовки, старшие три —")
    w("/// знаки компонентов нормали (бит 7 — X, бит 6 — Y, бит 5 — Z). Знаковые")
    w("/// компоненты уже разобраны в `nx`/`ny`/`nz`, а `flags` хранит их")
    w("/// вместе с порогом дальности.")
    w("#[derive(Clone, Copy, Debug, PartialEq, Eq)]")
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
    w("    /// Адрес блока в ROM версии Кладова.")
    w("    pub addr: u16,")
    w("    /// 4-й байт записи таблицы (категория).")
    w("    pub category: u8,")
    w("    pub stats: Stats,")
    w("    pub vertices: &'static [Vertex],")
    w("    pub edges: &'static [Edge],")
    w("    pub faces: &'static [Face],")
    w("}")
    w("")

    for ship in ships:
        ident = re.sub(r"[^A-Za-z0-9]", "_", ship["name"]).upper()
        st = ship["stats"]
        w(f"/// `{ship['name']}` (адрес {ship['addr']:#06x}).")
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
        f"SHIP_{re.sub(r'[^A-Za-z0-9]', '_', s['name']).upper()}"
        for s in ships)
    w(f"/// Модели версии Кладова (`zx_bin/kladov.bin`, {len(ships)} шт.).")
    w(f"pub static SHIPS: &[Ship] = &[{refs}];")
    w("")
    w("/// Найти модель по внутреннему имени.")
    w("pub fn find(name: &str) -> Option<&'static Ship> {")
    w("    SHIPS.iter().find(|s| s.name == name)")
    w("}")
    return "\n".join(out)

def main():
    ap = argparse.ArgumentParser(description=__doc__.splitlines()[0])
    ap.add_argument("--json", metavar="PATH", help="выгрузить всё в JSON")
    ap.add_argument("--rust", metavar="PATH", help="сгенерировать Rust-модуль")
    args = ap.parse_args()

    img, ships = load()
    print(f"kladov: {len(ships)} объектов")
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
        path.write_text(json.dumps({"kladov": ships}, indent=1, ensure_ascii=False))
        print(f"json -> {path}")
    if args.rust:
        path = pathlib.Path(args.rust)
        path.write_text(to_rust(ships) + "\n", encoding="utf-8")
        print(f"rust -> {path}")
    return 0


if __name__ == "__main__":
    try:
        sys.exit(main())
    except Error as exc:
        print(f"ошибка: {exc}", file=sys.stderr)
        sys.exit(1)
