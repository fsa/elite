# Радар и отрисовка кругов (версия Кладова)

Основано на `zx_src/kladov.asm` и сверено с `zx_bin/kladov.bin` (маппинг: 4000-7fff->off0, 8000-bfff->4000, c000-ffff->8000).

## Ключевые переменные (по листингу)
- `CircleCrdXlow/med/hi`, `CircleCrdYlow/med/hi`, `CircleCrdZlow/med/hi` (около 0x14B0+ в области данных, см. листинг)
- `CircleCrdY` (word), `CircleFlg` (byte), `CircleRadius`, `FillCircle`, `DirPrnCircle`
- `CrdXProjection`, `CrdYProjection`, `CrdXYsign`, `WhatOnScreen` и др.

## Основные подпрограммы отрисовки кругов
- `PrnCircle_Fill` (0x9084) — заполняющий круг
- `PrnCircle_NoFill` (0x9095) — незаполненный круг  
- `PrnCircle_NoFill_NoRadius` (0x90A5) — без пересчёта радиуса
- `PrnCircle_Clip` (0x9115) — клиппинг/отрисовка круга с учётом экрана
- Вспомогательные: `PrnSmallCircle_Fill`, `PrnSmallCircle_NoFill`, `Sqr_DE_A` и цепочки через `sub_8689/sub_86A1/sub_86B8/sub_86C4`

## Отображение объектов (включая "радароподобное" поведение)
- `sub_7E02` (0x7E02) — обработка отображения объекта/шара гиперперехода/разных случаев
- `ShipHyperJump` (0x7E4E) — отрисовка концентрических кругов при гиперпереходе корабля
- `CounterShipHyperJump` (0x81??) — счётчик/обработка гиперперехода
- `objDestroyed`, `DestroyObj` — уничтожение объектов

В версии Кладова есть отличия от оригинальной Elite (мишени, интерфейс). При декомпиляции важно различать "классический радар" и изменённый вариант Кладова.
