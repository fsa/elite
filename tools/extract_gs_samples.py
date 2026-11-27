#!/usr/bin/env python3
"""Извлечь сэмплы General Sound из `zx_src/Elite3GS.trd`.

Каталог GS-дискеты начинается со смещения 0x10. Каждая запись — 16 байт:

    [0..2]   номер звука в ASCII HEX (01..17, то есть 1..23)
    [2..8]   параметры сэмпла (в разборе не используются)
    [8]      0x73 ('s') — признак звукового файла
    [10]     0x60 — период сэмплирования General Sound
    [13]     длина в секторах
    [14]     начальный сектор (0..15)
    [15]     начальный трек

Физическое смещение данных: `(track * 16 + sector) * 0x100`. Сэмплы идут
непрерывным блоком: конец одного файла является началом следующего.

Результат — `zx_bin/gs_samples.bin` (сэмплы по номерам 1..23 подряд) и
`src/gs_samples.rs` с таблицей смещений для нативного проигрывателя.
"""
from pathlib import Path

TRD = Path("zx_src/Elite3GS.trd")
BIN = Path("zx_bin/gs_samples.bin")
RS = Path("src/gs_samples.rs")

DIR = 0x10
ENTRY = 0x10
TYPE_SAMPLE = 0x73
SECTOR = 0x100
TRACK = 16 * SECTOR
SAMPLE_PERIOD = 0x60  # байт 0x0A каталога: 1 МГц / 96 ≈ 10417 Гц


def main() -> None:
    data = TRD.read_bytes()
    entries = []
    for off in range(DIR, len(data), ENTRY):
        e = data[off : off + ENTRY]
        if len(e) != ENTRY or not any(e):
            break
        if e[8] != TYPE_SAMPLE:
            continue
        try:
            ident = int(e[0:2], 16)
        except ValueError:
            continue
        sectors = e[13]
        start = e[15] * TRACK + e[14] * SECTOR
        entries.append((ident, e[:8].decode("ascii"), e[10], sectors, start))

    entries.sort()
    if [e[0] for e in entries] != list(range(1, len(entries) + 1)):
        raise SystemExit("номера сэмплов должны идти подряд от 1")

    cursor = entries[0][4]
    chunks = []
    for ident, name, period, sectors, start in entries:
        if start != cursor:
            raise SystemExit(f"сэмпл {ident:02X}: ожидалось {cursor:#x}, найдено {start:#x}")
        if period != SAMPLE_PERIOD:
            raise SystemExit(f"сэмпл {ident:02X}: неожидаемый период {period:#x}")
        chunks.append(data[start : start + sectors * SECTOR])
        cursor = start + sectors * SECTOR

    BIN.parent.mkdir(parents=True, exist_ok=True)
    BIN.write_bytes(b"".join(chunks))

    offsets = [0]
    for chunk in chunks:
        offsets.append(offsets[-1] + len(chunk))
    lines = [
        "//! Сэмплы General Sound, извлечённые из `zx_src/Elite3GS.trd`.",
        "//!",
        "//! Файл генерируется `tools/extract_gs_samples.py`; вручную не править.",
        "//! Порядок соответствует номеру звука из защёлки порта `0xB3`.",
        "",
        f"/// Период сэмплирования из каталога (`0x{ SAMPLE_PERIOD:02X}`), 1 МГц / период.",
        f"pub const GS_SAMPLE_RATE: u32 = 1_000_000 / {SAMPLE_PERIOD};",
        "",
        "/// Все сэмплы подряд: `GS_SAMPLE_OFFSETS[id]..GS_SAMPLE_OFFSETS[id + 1]`.",
        'pub const GS_SAMPLE_DATA: &[u8] = include_bytes!("../zx_bin/gs_samples.bin");',
        "",
        "/// Смещения сэмплов в `GS_SAMPLE_DATA`; индекс 0 — пустой сэмпл.",
        "pub const GS_SAMPLE_OFFSETS: [usize; %d] = [" % (len(chunks) + 1),
    ]
    lines.extend(f"    {off}," for off in offsets)
    lines.append("];")
    lines.append("")
    lines.append("/// Вернуть 8-битный сэмпл по номеру из защёлки порта `0xB3`.")
    lines.append("pub fn sample(id: u8) -> Option<&'static [u8]> {")
    lines.append("    let i = id as usize;")
    lines.append("    if i == 0 || i >= GS_SAMPLE_OFFSETS.len() {")
    lines.append("        return None;")
    lines.append("    }")
    lines.append("    Some(&GS_SAMPLE_DATA[GS_SAMPLE_OFFSETS[i - 1]..GS_SAMPLE_OFFSETS[i]])")
    lines.append("}")
    lines.append("")
    RS.parent.mkdir(parents=True, exist_ok=True)
    RS.write_text("\n".join(lines), encoding="utf-8")

    print(
        "извлечено %d сэмплов (%d байт) в %s; таблица в %s"
        % (len(chunks), offsets[-1], BIN, RS)
    )


if __name__ == "__main__":
    main()
