# Данные игры (обзор)

## Игрок
- PilotName - имя пилота
- PilotCash, Fuel, Cargo и др. (см. ram_map.md)

## Корабли
- ShipData структура + таблицы моделей
- SlotObj для активных объектов

## Галактка/планеты
- DataOfPlanet, DataOfPlanet1, DataOfPlanet2
- PlanetName, SystemName и др.
- Генерация цен GenerateAnotherPrice

## Торговля
- TableBasePrice
- Товары, цены

## Таблицы
- TablSqr - таблица квадратов
- TablCrdPrnVertex - таблица координат вершин
- MatrixRotate - матрица вращения

Детальная информация в src/shipdata.rs и src/shipdata/*.rs (по файлу на
модель), src/galaxy.rs, src/equip.rs для версии Кладова.
