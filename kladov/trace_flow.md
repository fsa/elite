# Трассировка игрового цикла (kladov)

## Старт
- InitGame (01_start_init) - инициализация пилота/системы/цены
- Переход в игровой цикл

## GameMain (sub_728D / GameMain)
Основной цикл из kladov.asm (около 5117+):

Ключевые вызовы:
- CalcAngSpeed4Prn - расчёт углов/скорости
- sub_80D3 - обработка клавиш меню/панели
- ScanKeyInPause
- j_GoJump - прыжок
- ControlKeyIfMapOn
- MenuSaveLoad / Cmd_L_PilotInfo
- GoDocking
- CalcCrdCoriolis2
- Use_J_Drive
- CalcCrdSunPlanet
- j_j_MoveAllObjects / MoveAllObjects
- SetIndStation, Damage2Pilot
- CorrData4Nova
- j_PrnTxtOnViewScr
- PrnCocpit_Ifn, ClearScrBuffer
- PrnSunPlanet, PrintStars
- sub_9296
- sub_ED60, UpdateAllIndicators
- Use_Y_Cloacking, Process_MissBombEPod
- PrintCompass
- Возврат в GameMain

## Движение/объекты
- MoveAllObjects / ReactionObject / DirectObject / MoveObject
- PrnObjIfVisible - отрисовка объектов

## Проекция/рисование
- GetCrdsProjection / SetCrdCDEinExtSlot
- CalcPrnCrdVertex / GetPrnCrdVertex
- PrnLineInBuf_clip
- PrnCircle_Fill/NoFill/Clip - круги
