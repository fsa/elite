; kladov/00_radar_main.asm
; Основной код радара/кругов - выделено из kladov.asm с метками
; Для удобства переноса на другие платформы

; Переменные (из kladov.asm, адреса см. в оригинале)
; CircleCrdXlow/med/hi, CircleCrdYlow/med/hi, CircleCrdZlow/med/hi
; CircleCrdY (0x22B8)
; CircleFlg (0x22B9) 
; CircleRadius (0x22BA)
; DirPrnCircle (0x22BF)
; FillCircle (0x22C1)

; === PrnCircle_Fill (0x9084) ===
PrnCircle_Fill:
		CALL	PrnSmallCircle_Fill	; ld a,#A0
		CP	9
		RET	C
		DI
		XOR	A
		LD	(FillCircle), A
		JP	PrnCircle_NoFill_NoRadius

; === PrnCircle_NoFill (0x9095) ===
PrnCircle_NoFill:
		CALL	PrnSmallCircle_NoFill
		CP	9
		RET	C
loc_86E4:
		LD	A, 1
		LD	(FillCircle), A

; === PrnCircle_NoFill_NoRadius (0x90A5) ===
PrnCircle_NoFill_NoRadius:
		CALL	PrnCircle_Clip
		EI
		RET

; === PrnCircle_Clip (0x9115) ===
PrnCircle_Clip:
		LD	BC, (CircleCrdY)
		LD	(word_861E), BC
		LD	DE, (CircleFlg)
; ... (полный код см. в zx_src/kladov.asm с 9115)

; Примечание: Кладовская версия имеет отличия от оригинала
