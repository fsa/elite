; kladov/radar.asm
; Фрагменты кода, связанные с радаром и отрисовкой кругов
; Сверено с zx_bin/kladov.bin. Метки даны осмысленно по функционалу.

; === Переменные круга/проекции (см. kladov.asm) ===
; CircleCrdXlow/med/hi, CircleCrdYlow/med/hi, CircleCrdZlow/med/hi
; CircleCrdY, CircleFlg, CircleRadius, FillCircle, DirPrnCircle
; CrdXProjection, CrdYProjection, CrdXYsign, WhatOnScreen

; === Отрисовка круга ===
PrnCircle_Fill:        ; 0x9084
;   вызывает PrnSmallCircle_Fill, при r>=9 включает клиппинг
;   см. оригинальный листинг

PrnCircle_NoFill:      ; 0x9095
;   вызывает PrnSmallCircle_NoFill, при r>=9 включает клиппинг

PrnCircle_Clip:        ; 0x9115
;   основная процедура отрисовки круга с клиппингом/определением видимости
;   использует CircleCrdY/CircleFlg/CircleRadius

; === Отображение объектов ===
sub_7E02:              ; 0x7E02 - обработка отрисовки объекта
;   в зависимости от типа/состояния вызывает:
;   - ShipHyperJump (гиперпереход) 
;   - objDestroyed
;   - sub_A7D9 и др.

ShipHyperJump:         ; 0x7E4E - рисует концентрические круги при гиперпереходе
;   формирует CircleFlg/CircleCrdY/CircleRadius, вызывает PrnCircle_NoFill несколько раз

; === Примечания ===
; Кладовская версия имеет модификации радара/дисплея - требуется детальное
; сравнение с оригинальным elite.asm для выявления отличий.
