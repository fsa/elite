# Индекс разбора кода Кладова

## Фокус: радар и отрисовка кругов

### Основные файлы
- `00_radar_main.asm` - ключевые подпрограммы отрисовки кругов (PrnCircle_Fill/NoFill/Clip)
- `01_objects_radar.asm` - обработка объектов, связанная с отображением
- `02_destroy_hyper.asm` - DestroyObj, ShipHyperJump, гиперпереходы
- `03_circles_main.asm` - полный код кругов из kladov.asm (строки 9080-9700)
- `04_sun_planet.asm` - отрисовка солнца/планеты через круги
- `05_scanner.asm` - сканер/радар (PrintObjScaner, CopyScaner2Scr и др.)

### Переменные
- `radar_and_circles.md` - описание
- `circle_vars.inc` - заглушки переменных

### Новая основа (128K)
- `kladov128.asm` - полный листинг из `zx_bin/kladov128.bin` с сохранёнными метками и комментариями
- `gs_patches.asm` - аннотированный разбор GS-патчей (`0x7344`, точки вызова `0x7346`, команды загрузчика)
- `disassemble128.py` - генератор листинга (нужен `z80dis`)

### Инструменты
- `binmap.py` - маппинг адресов
- `simple_z80_disasm.py` - простой дизассемблер
- `dump_labels.py` - поиск меток
- `extract_sections.py` - извлечение секций

## Важно
- Код проверен по kladov.bin (маппинг 4000-7fff/page5, 8000-bfff/page2, c000-ffff/page0)
- Сравнивать с zx_src/elite.asm для выявления отличий Кладова
- В kladov.asm есть комментарии на русском языке
