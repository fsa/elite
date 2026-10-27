# Быстрый старт для переноса

## Главные документы
- `porting_order.md` - рекомендуемый порядок переноса
- `xref/callgraph.md` - кто кого вызывает
- `xref/xrefs.md` - перекрёстные ссылки
- `ram/ram_map.md` - карта переменных

## По блокам (готовы к переносу)
- `01_start_init/` - старт
- `03_game_loop/` - основной цикл
- `07_3d_projection/` - 3D математика (база)
- `04_movement_physics/` - движение
- `10_circles_planets/` - круги/планеты
- `08_radar_scanner/` - радар
- `06_objects_render/` - отрисовка
- `11_combat_lasers/` - бой
- `12_docking/` - стыковка
- `13_hyperspace/` - гиперпереходы
- `14_galaxy_map/` - карта
- `15_trading_equip/` - торговля
- `16_missions/` - миссии Кладова
- `17_save_load/` - сохранение

## Детальный разбор радара (приоритет изучения)
- `00_radar_main.asm`, `01_objects_radar.asm`, `02_destroy_hyper.asm`, `03_circles_main.asm`, `04_sun_planet.asm`, `05_scanner.asm`

## Сопоставление с оригиналом
В `diffs/` можно добавить результаты сравнения. Уже видно: `ShipHyperJump`, `sub_7E02` есть только в Кладове - это отличия версии Кладова.
