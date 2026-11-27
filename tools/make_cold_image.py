#!/usr/bin/env python3
"""Собрать 128K-образ холодного старта из TR-DOS-диска.

Рантайм эмулятора не знает ничего о TR-DOS: он кладёт готовый образ в ОЗУ и
сразу передаёт управление игре по `0x7394`. Этот скрипт однократно пересобирает
`zx_bin/kladov128gs.bin` из диска; раньше там лежал снапшот работающей игры,
теперь — состояние сразу после загрузчика, до первого кадра интро.

Порядок повторяет загрузчик `ELITE3  B` / `ELITE3GSB`:

* тело BASIC-файла кладётся по `0x5D3B`, оттуда берутся системные переменные
  и хвостовой блок кода для банка 3;
* основные блоки диска раскладываются по банкам 5, 2, 0, 1 и 3;
* хвостовой код банка 3 читает ещё `0x500` байт в `0xD000`, копирует их в
  `0x5B00`, чистит атрибуты экрана, ставит заглушку `ED 79 C9` по `0x4000`
  и оставляет на стеке адрес возврата `0x7394`.

Использование:
    python3 tools/make_cold_image.py                    # пересобрать zx_bin/kladov128gs.bin
    python3 tools/make_cold_image.py out.bin            # записать в другой файл
    python3 tools/make_cold_image.py --plain out.bin    # обычный вариант без GS
"""

import argparse
from pathlib import Path

ROOT = Path(__file__).resolve().parent.parent
BANK = 0x4000
BASIC_ADDR = 0x5D3B
BASIC_LEN = {"plain": 0x160, "gs": 0x2A0}
BOOT = {"plain": 0x1200, "gs": 0x1300}
PAYLOAD = {"plain": (0x10FE, 0x54), "gs": (0x1244, 0x49)}
# Номера страниц сэмплов General Sound, которые GS-ветка загрузчика пишет
# в таблицу по 0x51E0, 0x52E0, ... (по два байта через один).
GS_PAGES = (0x51E0, 0x51E2, 0x52E0, 0x52E2, 0x53E0, 0x53E2, 0x54E0, 0x54E2, 0x55E0, 0x55E2)
GS_PAGE_VALUES = (0x11, 0x11, 0x11, 0x11, 0x17, 0x17, 0x21, 0x21, 0x41, 0x41)


def ram_offset(addr: int) -> int:
    """Смещение фиксированного адреса ЗХ Spectrum в физическом образе."""
    if 0x4000 <= addr < 0x8000:
        return 5 * BANK + (addr - 0x4000)
    if 0x8000 <= addr < 0xC000:
        return 2 * BANK + (addr - 0x8000)
    raise ValueError("адрес должен быть в окне 0x4000..0xC000")


def build(trd: bytes, gs: bool) -> bytes:
    kind = "gs" if gs else "plain"
    boot = BOOT[kind]
    image = bytearray(0x20000)

    def put(src: int, dst: int, count: int) -> None:
        image[dst : dst + count] = trd[src : src + count]

    # Тело BASIC-файла: код загрузчика и хвостовой блок для банка 3.
    basic = ram_offset(BASIC_ADDR)
    put(0x1000, basic, BASIC_LEN[kind])

    # CALL 0x3D13: 0x1B секторов в 0x4000, затем 0xA0 секторов в 0x6000.
    # Второй блок идёт через 0xFFFF, поэтому он попадает в банки 5, 2 и 0.
    put(boot, 5 * BANK, 0x1B00)
    put(boot + 0x1B00, 5 * BANK + 0x2000, 0x2000)
    put(boot + 0x3B00, 2 * BANK, 0x4000)
    put(boot + 0x7B00, 0 * BANK, 0x4000)

    # Банк 1 (порт 0x11) и банк 3 (порт 0x13).
    put(boot + 0xBB00, 1 * BANK, 0x1100)
    put(boot + 0xCC00, 3 * BANK + 0x1000, 0x0500)

    # Код в конце банка 3: он исполняется загрузчиком и остаётся в образе.
    payload, payload_len = PAYLOAD[kind]
    put(payload, 3 * BANK, payload_len)

    if not gs:
        # Указатель TR-DOS после последнего прочитанного сектора и копия
        # системных переменных в банк 3 до переноса блока в 0x5B00.
        image[ram_offset(0x5CF4)] = 0x03
        image[ram_offset(0x5CF5)] = 0x0E
        image[0xE000:0xE200] = image[ram_offset(0x5C00) : ram_offset(0x5E00)]

    # Перенос 0xD000..0xD4FF в 0x5B00..0x5FFF и очистка атрибутов экрана.
    image[ram_offset(0x5B00) : ram_offset(0x6000)] = image[0xD000:0xD500]
    image[ram_offset(0x5800) : ram_offset(0x5B00)] = bytes(0x300)

    if gs:
        for addr, value in zip(GS_PAGES, GS_PAGE_VALUES):
            image[ram_offset(addr)] = value

    # Заглушка 0x4000 и адрес возврата на стеке (SP=0x4120).
    image[ram_offset(0x4000) : ram_offset(0x4000) + 3] = bytes.fromhex("ed79c9")
    image[ram_offset(0x411E) : ram_offset(0x4120)] = (0x7394).to_bytes(2, "little")
    return bytes(image)


def main() -> None:
    parser = argparse.ArgumentParser()
    parser.add_argument("--plain", action="store_true", help="собрать обычный вариант без GS")
    parser.add_argument("output", nargs="?", help="куда записать образ")
    args = parser.parse_args()

    gs = not args.plain
    kind = "gs" if gs else "plain"
    trd = (ROOT / ("zx_src/Elite3GS.trd" if gs else "zx_src/Elite3.trd")).read_bytes()
    default = "zx_bin/kladov128gs.bin" if gs else "zx_bin/kladov_cold.bin"
    output = Path(args.output) if args.output else ROOT / default
    output.write_bytes(build(trd, gs))
    print(f"{kind}: {output} ({output.stat().st_size} bytes)")


if __name__ == "__main__":
    main()
