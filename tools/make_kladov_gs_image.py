#!/usr/bin/env python3
"""Собрать 128K-образ Кладова с поддержкой General Sound.

Источник — два TR-DOS-образа из `zx_src`:

* `Elite3.trd`   — обычный Кладов, ему соответствует хранящийся в репозитории
  `zx_bin/kladov128.bin`;
* `Elite3GS.trd` — тот же загрузочный образ, сдвинутый на один сектор из-за
  дополнительного сектора с кодом General Sound, плюс 23 файла сэмплов GS.
  Они извлекаются отдельно в `zx_bin/gs_samples.bin` (`tools/extract_gs_samples.py`)
  и воспроизводятся нативным кодом, а не эмулятором General Sound.

Берём уже готовый снапшот `kladov128.bin` и переносим в него только байты,
которые отличаются между загрузочными телами двух дисков. Так сохраняются
состояние игры и runtime-таблицы из снапшота: образ не пересобирается
с нуля, а получается патчем существующего.

Раскладка загрузочного тела `Elite3.trd` (адреса — смещения в TRD):

    0x1200..0x2CFF -> банк 5 +0x0000 (0x4000..0x5AFF)
    0x2D00..0x4CFF -> банк 5 +0x2000 (0x6000..0x7FFF)
    0x4D00..0x8CFF -> банк 2        (0x8000..0xBFFF)
    0x8D00..0xCCFF -> банк 0        (0xC000..0xFFFF)
    0xCD00..0xDDFF -> банк 1        (0xC000..0xD0FF)
    0xDE00..0xE4FF -> банк 3 +0x1000 (0xD000..0xD6FF)

В `Elite3GS.trd` всё тело сдвинуто на `+0x100`. Дополнительный сектор
`0x1100..0x12FF` содержит подпрограммы General Sound; они отображаются в
память со смещением `0x4D3B`, поэтому `LD HL,0x5F7F` в загрузчике GS
указывает на `B[0x1244]`. Оттуда берётся укороченный до `0x49` байт блок,
который загрузчик копирует в банк 3 по `0xC000`.

Использование:
    python3 tools/make_kladov_gs_image.py [base.bin] [out.bin]
"""
import sys
from pathlib import Path

BOOT_A = 0x1200
BOOT_B = 0x1300
BOOT_LEN = 0xD300  # 211 sectors = 0xD3 * 0x100
PAYLOAD_LEN = 0x49
PAYLOAD_DISK = 0x1244
PAYLOAD_MEM = 0xC000


def map_boot(offset: int) -> int | None:
    """Смещение в загрузочном теле Elite3 -> абсолютный адрес в 128K-образе."""
    if 0x1200 <= offset <= 0x2CFF:
        return 0x14000 + (offset - 0x1200)
    if 0x2D00 <= offset <= 0x4CFF:
        return 0x16000 + (offset - 0x2D00)
    if 0x4D00 <= offset <= 0x8CFF:
        return 0x8000 + (offset - 0x4D00)
    if 0x8D00 <= offset <= 0xCCFF:
        return 0xC000 + (offset - 0x8D00)
    if 0xCD00 <= offset <= 0xDDFF:
        return 0x4000 + (offset - 0xCD00)
    if 0xDE00 <= offset <= 0xE4FF:
        return 0xD000 + (offset - 0xDE00)
    return None


def main() -> None:
    base = Path(sys.argv[1] if len(sys.argv) > 1 else "zx_bin/kladov128.bin")
    out = Path(sys.argv[2] if len(sys.argv) > 2 else "zx_bin/kladov128gs.bin")
    a = Path("zx_src/Elite3.trd").read_bytes()
    b = Path("zx_src/Elite3GS.trd").read_bytes()
    image = bytearray(base.read_bytes())

    if len(image) != 0x20000:
        raise SystemExit("ожидался 128K-образ на %d байт" % 0x20000)

    patched = 0
    unmapped = 0
    for rel in range(BOOT_LEN):
        da = BOOT_A + rel
        db = BOOT_B + rel
        if a[da] == b[db]:
            continue
        mem = map_boot(da)
        if mem is None:
            unmapped += 1
            continue
        image[mem] = b[db]
        patched += 1

    # GS-загрузчик копирует в банк 3 только 0x49 байт вместо 0x54.
    payload = b[PAYLOAD_DISK:PAYLOAD_DISK + PAYLOAD_LEN]
    image[PAYLOAD_MEM:PAYLOAD_MEM + PAYLOAD_LEN] = payload
    image[PAYLOAD_MEM + PAYLOAD_LEN:PAYLOAD_MEM + 0x54] = bytes(0x54 - PAYLOAD_LEN)

    out.parent.mkdir(parents=True, exist_ok=True)
    out.write_bytes(image)
    print(
        "записан %s (%d байт): patched=%d, unmapped=%d, payload=0x%X@0x%04X"
        % (out, len(image), patched, unmapped, PAYLOAD_LEN, PAYLOAD_MEM)
    )


if __name__ == "__main__":
    main()
