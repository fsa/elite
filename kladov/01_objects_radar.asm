; kladov/01_objects_radar.asm
; Отображение объектов - связано с радаром/кругами

; === sub_7E02 (0x7E02) ===
; Обработка отображения объекта
sub_7E02:
		JR	Z, ShipHyperJump
objDestroyed:
		CALL	GetCrdsProjection
		CALL	SetCrdCDEinExtSlot
		JP	objDestroyed2

; === ShipHyperJump (0x7E4E) ===
; Рисует концентрические круги при гиперпереходе корабля
ShipHyperJump:
		LD	HL, (word_B452)
		LD	A, (HL)
		AND	0F8h
		CP	0A0h
		JR	Z, loc_7E4E_hyper	; уход в гипертоннель
		BIT	3, A
		JP	Z, sub_A7D9
		...
; См. полный код в zx_src/kladov.asm

; === DestroyObj (0x1B4F?) ===
; см. kladov.asm:6983

; Отличия Кладова: возможно изменённая логика радара/обработки объектов
