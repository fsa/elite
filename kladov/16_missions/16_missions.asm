; 16_missions - разобрано из kladov.asm
; Количество подпрограмм: 15

; === Cmd_L_PilotInfo === CODE XREF: j_Cmd_L_PilotInfoj
Cmd_L_PilotInfo:			; CODE XREF: j_Cmd_L_PilotInfoj
					; RAM:E9FDj ...
		ld	a, 0Bh
		ld	(WhatOnScreen),	a
		call	PrintRamkMenu
		call	_PrintIndDanger
		ld	de, unk_FD8E
		call	SetAttrFrom_DE
		call	sub_F90E
		nop
		call	PrintSymbol_SP
; ---------------------------------------------------------------------------
		db 7Eh
; ---------------------------------------------------------------------------
		call	loc_8D38
		nop
		nop
		ld	a, 2
		call	loc_F0A1
		ld	a, 3
		call	loc_F0A1
		ld	a, 6
		call	loc_E2C8
		call	PrintSymbol_SP
; ---------------------------------------------------------------------------
		db 5
; ---------------------------------------------------------------------------
		call	PrintSymbol_SP
; ---------------------------------------------------------------------------
		db 7
; ---------------------------------------------------------------------------
		ld	a, 71h ; 'q'
		call	sub_E292
		call	PrintSymbol_SP
; ---------------------------------------------------------------------------
		db 20h
; ---------------------------------------------------------------------------
		ld	b, 12h
		ld	a, (PilotLegalStatus)
		and	a

loc_EFCB:
		jr	z, loc_EFD3
		cp	32h ; '2'
		jr	c, loc_EFD2
		inc	b

loc_EFD2:				; CODE XREF: RAM:EFCFj
		inc	b

loc_EFD3:				; CODE XREF: RAM:loc_EFCBj
		ld	a, b
		call	PrnMsg_A_CRLF
		ld	a, 72h ; 'r'
		call	sub_E292
		call	PrintSymbol_SP
; ---------------------------------------------------------------------------
		db 20h
; ---------------------------------------------------------------------------
		ld	hl, PilotRating2 ; >=#84 в трюме десант
					; 7,=1 война была или идёт
					; 6,=1 война закончилать
					; 3-1 -	ранг (=текущая стадия войны)
		ld	a, (hl)
		and	a
		jr	nz, loc_EFEF
		dec	hl
		ld	de, byte_F0AE
		ld	b, 15h
		jr	loc_EFF4
; ---------------------------------------------------------------------------

loc_EFEF:				; CODE XREF: RAM:EFE5j
		ld	de, byte_F0B4
		ld	b, 1Ah

loc_EFF4:				; CODE XREF: RAM:EFEDj	RAM:EFFAj
		ld	a, (de)
		cp	(hl)
		jr	nc, loc_EFFC
		inc	b
		inc	de
		jr	loc_EFF4
; ---------------------------------------------------------------------------

loc_EFFC:				; CODE XREF: RAM:EFF6j
		ld	a, b
		call	sub_D755	; печать рейтинга и, если нужно	ранга
		ld	a, 11h
		call	sub_F06F
		ld	bc, 6FEh
		ld	hl,  loc_F098+2

loc_F00B:				; CODE XREF: RAM:F041j
		inc	c
		inc	hl
		push	bc
		push	hl
		ld	l, (hl)
		ld	h, 0D0h	; '╨'
		ld	a, (PilotExtWeapon) ; 7,=1 - ECM Jammer
					; 6,=1 - Cloaking device
					; 5,=1 - нельзя	купить товары(в	трюме беженцы или десант)
					; 4
					; 3 =1 3я миссия была выполнена
					; 2 =1 - третья	миссия выполнена
					; 1 =1 - второя	миссия выполнена
					; 0 =1 - первая	миссия выполнена
		and	c
		rlca
		ld	a, l
		push	af
		ld	a, (hl)
		and	a
		jr	z, loc_F03E
		ld	a, (byte_D08E)	; <>0 есть Cobra Mk4
		and	a
		jr	z, loc_F02C
		ld	b, (hl)
		set	7, b
		ld	a, 30h ; '0'
		sub	b
		call	PrintMessage1_A_Space

loc_F02C:				; CODE XREF: RAM:F021j
		pop	af
		push	af
		jr	nc, loc_F039
		ld	a, 0CFh	; '╧'
		call	PrintMessage2
		call	sub_F072
		xor	a

loc_F039:				; CODE XREF: RAM:F02Ej
		sub	19h
		call	nc, sub_F06F

loc_F03E:				; CODE XREF: RAM:F01Bj
		pop	af
		pop	hl
		pop	bc
		djnz	loc_F00B
		ld	hl, PilotFrontLaser

loc_F046:				; CODE XREF: RAM:F04Fj
		push	bc
		push	hl
		call	sub_F07A
		pop	hl
		pop	bc
		inc	hl
		dec	c
		jr	nz, loc_F046
		ld	a, (PilotAlienComputer)	; 0,=1 есть Alien Computer
		rrca
		ld	a, 0F3h	; 'є'
		call	c, sub_F06F
		ld	a, (PilotExtWeapon) ; 7,=1 - ECM Jammer
					; 6,=1 - Cloaking device
					; 5,=1 - нельзя	купить товары(в	трюме беженцы или десант)
					; 4
					; 3 =1 3я миссия была выполнена
					; 2 =1 - третья	миссия выполнена
					; 1 =1 - второя	миссия выполнена
					; 0 =1 - первая	миссия выполнена
		and	40h ; '@'
		call	sub_F068
; ---------------------------------------------------------------------------
		db 7Dh
; ---------------------------------------------------------------------------
		jp	loc_D646
; ---------------------------------------------------------------------------
		db  46h	; F
		db 0D6h	; ╓

; =============== S U B	R O U T	I N E =======================================




; === EndMission3 === CODE XREF: GoMission3+Cj
EndMission3:				; CODE XREF: GoMission3+Cj
		jp	sub_D14D
; END OF FUNCTION CHUNK	FOR GoMission3
; ---------------------------------------------------------------------------
		db    0
		db    0
		db    0

; =============== S U B	R O U T	I N E =======================================

; Attributes: thunk



; === GiveMission3 === CODE XREF: GoMission3:loc_727Fj
GiveMission3:				; CODE XREF: GoMission3:loc_727Fj
		jp	loc_D123
; END OF FUNCTION CHUNK	FOR GoMission3
; ---------------------------------------------------------------------------
		db 0C3h	; ├
		db  97h	; Ч
		db 0DEh	; ▐


; === GoMission1 === CODE XREF: Cmd_2_BuyProducts+Dp
GoMission1:				; CODE XREF: Cmd_2_BuyProducts+Dp
		ld	a, (PilotMission1)
		and	a
		scf
		ret	nz
		push	hl
		push	de
		call	TestPilotInZoneCombat ;	Определяем находится ли	пилот в	зоне #3C (активных боёв)
					; Вх:  ---
					; Вых: cy =1 пилот в зоне #3C
					;	  =0 пилот в другой галактике галактике
					;	     война окончена/не начиналась
					;	     пилот в другой зоне
					;      hl,de,b,a - не определены
		pop	de
		pop	hl
		ret	nc
		ld	d, 9Dh ; 'Э'
		ld	a, (PilotRating2) ; >=#84 в трюме десант
					; 7,=1 война была или идёт
					; 6,=1 война закончилать
					; 3-1 -	ранг (=текущая стадия войны)
		cp	84h ; 'Д'
		ret	c
		inc	d
		and	8Eh ; 'О'
		xor	86h ; 'Ж'
		ret	nz
		scf
		ret
; End of function GoMission1

; ---------------------------------------------------------------------------
unk_FD41:	db  20h			; DATA XREF: sub_FCBA+10o
		db  0Dh
		db  50h	; P
		db  30h	; 0
		db  31h	; 1
		db  51h	; Q
		db  41h	; A
		db 0FFh
		db 0FFh
		db  4Ch	; L
		db  4Fh	; O
		db  39h	; 9
		db  32h	; 2
		db  57h	; W
		db  53h	; S
		db  5Ah	; Z
		db  4Dh	; M
		db  4Bh	; K
		db  49h	; I
		db  38h	; 8
		db  33h	; 3
		db  45h	; E
		db  44h	; D
		db  58h	; X
		db  4Eh	; N
		db  4Ah	; J
		db  55h	; U
		db  37h	; 7
		db  34h	; 4
		db  52h	; R
		db  46h	; F
		db  43h	; C
		db  42h	; B
		db  48h	; H
		db  59h	; Y
		db  36h	; 6
		db  35h	; 5
		db  54h	; T
		db  47h	; G
		db  56h	; V

; =============== S U B	R O U T	I N E =======================================


ScanKeys:				; CODE XREF: sub_FF25+10p

; FUNCTION CHUNK AT FC10 SIZE 00000015 BYTES
; FUNCTION CHUNK AT FC3D SIZE 0000007D BYTES

		xor	a
		ld	(byte_FC0F), a
		dec	a
		ld	(PressKeyPrnSymbol), a
		ld	bc, 7FFEh
		ld	d, 8

loc_FD76:				; CODE XREF: ScanKeys-158j
		rlc	b
		in	a, (c)
		and	1Fh
		xor	1Fh
		ld	e, 5

loc_FD80:				; CODE XREF: ScanKeys+1Cj
		rra
		call	c, sub_FCBA
		dec	e
		jr	nz, loc_FD80

loc_FD87:				; DATA XREF: RAM:EB03o
		jp	loc_FC10
; End of function ScanKeys

; ---------------------------------------------------------------------------
		db    5
		db    6
		db  0Dh
		db  0Eh
unk_FD8E:	db 0CAh	; ╩		; DATA XREF: RAM:EF92o
		db  71h	; q
		db  89h	; Й
		db  21h	; !
		db  21h	; !
		db  61h	; a
		db  31h	; 1
		db  20h
unk_FD96:	db  79h	; y		; DATA XREF: sub_E87E+847o
		db  21h	; !
		db  20h
unk_FD99:	db  40h	; @		; DATA XREF: sub_D470+127o
		db  40h	; @
unk_FD9B:	db  41h	; A		; DATA XREF: RAM:D893o
		db  42h	; B
		db  4Fh	; O
		db  55h	; U
		db  53h	; S
		db  45h	; E
		db  49h	; I
		db  54h	; T
		db  49h	; I
		db  4Ch	; L
		db  45h	; E
		db  54h	; T
		db  53h	; S
		db  54h	; T
		db  4Fh	; O
		db  4Eh	; N
		db  4Ch	; L
		db  4Fh	; O
		db  4Eh	; N
		db  55h	; U
		db  54h	; T
		db  48h	; H
		db  4Eh	; N
		db  4Fh	; O
byte_FDB3:	db 41h			; DATA XREF: PrintMessage1_A+179o
					; sub_E03D+9o
		db 4Ch
		db 4Ch
		db 45h
		db 58h
		db 45h
		db 47h
		db  45h	; E
		db  5Ah	; Z
		db  41h	; A
		db  43h	; C
		db  45h	; E
		db  42h	; B
		db  49h	; I
		db  53h	; S
		db  4Fh	; O
		db  55h	; U
		db  53h	; S
		db  45h	; E
		db  53h	; S
		db  41h	; A
		db  52h	; R
		db  4Dh	; M
		db  41h	; A
		db  49h	; I
		db  4Eh	; N
		db  44h	; D
		db  49h	; I
		db  52h	; R
		db  45h	; E
		db  41h	; A
		db  3Fh	; ?
		db  45h	; E
		db  52h	; R
		db  41h	; A
		db  54h	; T
		db  45h	; E
		db  4Eh	; N
		db  42h	; B
		db  45h	; E
		db  52h	; R
		db  41h	; A
		db  4Ch	; L
		db  41h	; A
		db  56h	; V
		db  45h	; E
		db  54h	; T
		db  49h	; I
		db  45h	; E
		db  44h	; D
		db  4Fh	; O
		db  52h	; R
		db  51h	; Q
		db  55h	; U
		db  41h	; A
		db  4Eh	; N
		db  54h	; T
		db  45h	; E
		db  49h	; I
		db  53h	; S
		db  52h	; R
		db  49h	; I
		db  4Fh	; O
		db  4Eh	; N
unk_FDF3:	db 0B1h	; ▒		; DATA XREF: RAM:EDE5o
		db  42h	; B
		db  3Ah	; :
		db  52h	; R
		db  52h	; R
		db  54h	; T
		db  92h	; Т
		db  70h	; p
unk_FDFB:	db 0D9h	; ┘		; DATA XREF: sub_EF48+16o
		db 0F1h	; ё
		db 0F0h	; Ё
		db 0FEh	; ■
		db 0FEh	; ■
		db 0FEh	; ■

; =============== S U B	R O U T	I N E =======================================




; === GoMission3 === CODE XREF: sub_5FA1+3p
GoMission3:				; CODE XREF: sub_5FA1+3p

; FUNCTION CHUNK AT D0DD SIZE 00000003 BYTES
; FUNCTION CHUNK AT D0ED SIZE 00000003 BYTES
; FUNCTION CHUNK AT D123 SIZE 00000025 BYTES

		ld	a, (PilotMission3) ; =#06 при стыковке будет захват таргонами (вроде нереальная	ситуация)
					; =#04 в системе со танцией таргонов во	время войны
		dec	a
		jr	z, loc_727F
		dec	a

loc_727F:				; CODE XREF: GoMission3+4j
		jp	z, GiveMission3
		sub	3
		jp	z, EndMission3
		dec	a
		ld	a, 0D4h	; '╘'
		jr	z, loc_7295
		ret
; End of function GoMission3


; =============== S U B	R O U T	I N E =======================================

; Attributes: noreturn



; === GoMissionWar === CODE XREF: sub_5FA1p
GoMissionWar:				; CODE XREF: sub_5FA1p
		call	TestPilotInZoneWar ; Определяем	находится ли пилот/планета в заданной зоне войны
					; Вх:  (sp) =#00 зона оккупированная таргонами
					;	    =#3C зона линии фронта (активные боевые действия)
					;	    =#78 прифронтовая зона
					; Вых: cy =1 координаты	в пределах проверяемой зоны
					;	  =0 пилот в другой галактике галактике
					;	     война окончена/не начиналась
					;	     координаты	слишком	далеко от левого края
					;      hl,de,b,a - не определены
; ---------------------------------------------------------------------------
		db 0
; ---------------------------------------------------------------------------
		jr	c, loc_D74F
		ld	a, (PilotTargonWar) ; =#81 при сбросе миссий
					; 7,=1 начало войны, предолжение участвовать
					; 6-0, чем больше число	тем дальше от левого края карты	захват планет таргонами
					;
					; =#01 война закончена
		dec	a
		jr	nz, loc_D715
		ld	l, 62h ; 'b'    ; PilotRating2
		bit	6, (hl)
		ret	nz
		ld	a, (hl)
		cp	82h ; 'В'
		ret	c
		set	6, (hl)
		inc	(hl)
		ld	d, 0B8h	; '╕'
		jr	loc_D74A
; ---------------------------------------------------------------------------

loc_D715:				; CODE XREF: GoMissionWar+Aj
		call	TestPilotInZoneCombat ;	Определяем находится ли	пилот в	зоне #3C (активных боёв)
					; Вх:  ---
					; Вых: cy =1 пилот в зоне #3C
					;	  =0 пилот в другой галактике галактике
					;	     война окончена/не начиналась
					;	     пилот в другой зоне
					;      hl,de,b,a - не определены
		ret	nc
		ld	hl, PilotRating2 ; >=#84 в трюме десант
					; 7,=1 война была или идёт
					; 6,=1 война закончилать
					; 3-1 -	ранг (=текущая стадия войны)
		ld	a, (hl)
		rrca
		ret	nc
		and	7
		jr	z, loc_D730	; предложение участвовать в войне
		ld	b, a		; b=ранг 1..7
		ld	d, 0B4h	; '┤'
		ld	a, (PilotExtWeapon) ; 7,=1 - ECM Jammer
					; 6,=1 - Cloaking device
					; 5,=1 - нельзя	купить товары(в	трюме беженцы или десант)
					; 4
					; 3 =1 3я миссия была выполнена
					; 2 =1 - третья	миссия выполнена
					; 1 =1 - второя	миссия выполнена
					; 0 =1 - первая	миссия выполнена

loc_D729:				; CODE XREF: GoMissionWar+32j
		rrca
		inc	d
		djnz	loc_D729
		ret	nc
		jr	loc_D749
; ---------------------------------------------------------------------------

loc_D730:				; CODE XREF: GoMissionWar+28j
		ld	a, (PilotExtWeapon) ; 7,=1 - ECM Jammer
					; 6,=1 - Cloaking device
					; 5,=1 - нельзя	купить товары(в	трюме беженцы или десант)
					; 4
					; 3 =1 3я миссия была выполнена
					; 2 =1 - третья	миссия выполнена
					; 1 =1 - второя	миссия выполнена
					; 0 =1 - первая	миссия выполнена
		bit	5, a
		ret	nz
		push	hl
		ld	de, 0AC1Fh
		ld	hl, 405h
		call	sub_D162
		call	sub_E1F7
		pop	hl		; hl=PilotRating2
		cp	59h ; 'Y'
		ret	nz
		ld	d, 0ADh	; 'н'

loc_D749:				; CODE XREF: GoMissionWar+35j
		inc	(hl)

loc_D74A:				; CODE XREF: GoMissionWar+1Aj
		ld	e, 17h
		jp	loc_D145
; ---------------------------------------------------------------------------

loc_D74F:				; CODE XREF: GoMissionWar+4j
		ld	a, 4
		ld	(PilotMission3), a ; =#06 при стыковке будет захват таргонами (вроде нереальная	ситуация)
					; =#04 в системе со танцией таргонов во	время войны
		ret
; End of function GoMissionWar


; =============== S U B	R O U T	I N E =======================================

; печать рейтинга и, если нужно	ранга



; === ProgressMissionWar === CODE XREF: ReCalcCrdObj_coriolis+3p
ProgressMissionWar:			; CODE XREF: ReCalcCrdObj_coriolis+3p
		ld	hl, byte_70DC
		ld	a, (PilotTargonWar) ; =#81 при сбросе миссий
					; 7,=1 начало войны, предолжение участвовать
					; 6-0, чем больше число	тем дальше от левого края карты	захват планет таргонами
					;
					; =#01 война закончена
		cpl
		add	a, (hl)
		ld	(hl), a
		ret	nc
		inc	hl
		inc	(hl)
		ret	nz
		ld	hl, PilotTargonWar ; =#81 при сбросе миссий
					; 7,=1 начало войны, предолжение участвовать
					; 6-0, чем больше число	тем дальше от левого края карты	захват планет таргонами
					;
					; =#01 война закончена
		ld	a, (hl)
		cp	2
		ret	c
		rlca
		jr	c, loc_9B5A
		dec	(hl)
		ret
; ---------------------------------------------------------------------------

loc_9B5A:				; CODE XREF: ProgressMissionWar+15j
		inc	(hl)
		jr	z, loc_9B65
		ld	l, 0C7h	; '╟'   ; hl = CargoFound
		ld	(hl), 82h ; 'В' ; сообщение о вторжении в 1ю галактику
		inc	hl
		ld	(hl), 0F0h ; 'Ё'
		ret
; ---------------------------------------------------------------------------

loc_9B65:				; CODE XREF: ProgressMissionWar+1Aj
		ld	(hl), 7Fh ; ''
		ret
; End of function ProgressMissionWar


; =============== S U B	R O U T	I N E =======================================


CalcCrdSunPlanet_2:			; CODE XREF: CalcCrdSunPlanet+Bj
		call	j_CalcNewCrdObj_Pl
		ld	ix, fsSunCrdXl	; координаты "солнца" и "планеты" в межпланетном пространстве
		ld	hl, loc_A1AD
		ld	(hl), 0C9h ; '╔'
		push	hl
		call	j_CalcNewCrdObj_Pl
		ld	ix, fsPlCrdXl
		call	j_CalcNewCrdObj_Pl
		pop	hl
		ld	(hl), 3Ah ; ':'
		ret
; End of function CalcCrdSunPlanet_2


; =============== S U B	R O U T	I N E =======================================




; === Set3_4Targons === CODE XREF: CallFunctionObj_1+Cj
Set3_4Targons:				; CODE XREF: CallFunctionObj_1+Cj
		call	SetTargonStation ; Установка станции таргонов в	межпланетном пространстве для последней	подмиссии в войне с таргонами
		and	3
		add	a, 3
		ld	c, a

loc_B973:				; CODE XREF: CallFunctionObj_1+132j
		call	FindSlot4Obj
		ret	nz
		ld	a, 0Fh
		push	bc
		call	SetObjInSlot	; a - номер объекта для	установки
		set	0, (ix+25h)
		call	IntRnd_A
		and	0Fh
		or	4
		ld	(ix+7),	a
		rra
		rra
		rra
		and	1
		ld	(ix+1),	a
		xor	a
		ld	(ix+4),	a
		call	RndDirectionObj
		pop	bc
		dec	c
		jr	nz, loc_B973
		ret
; ---------------------------------------------------------------------------

SetObjMissile:				; CODE XREF: CallFunctionObj_1+10j
		ld	a, c		; Запуск ракеты
					; Вх:  с - цель	ракеты
					;	 =0 игрок (запустил противник)
					;	<>0 номер объекта по которому запустил ракету игрок
					; Вых: z=1 успешно
		and	a
		jr	z, loc_B9CF
		ld	ix, objCoriolis
		ld	de, 111h
		add	ix, de
		ld	a, (ix+22h)
		and	a
		jr	z, loc_B9B6
		call	FindSlot4Obj
		ret	nz

loc_B9B6:				; CODE XREF: CallFunctionObj_1+146j
		ld	a, 1
		push	bc
		call	sub_9391	; c = номер объекта по которому	запустил ракету	игрок
		pop	bc
		ld	a, 6Ah ; 'j'
		ld	(ix+6),	a
		ld	a, 29h ; ')'
		ld	(ix+3),	a
		ld	a, 80h ; 'А'
		ld	(ix+5),	a
		ld	a, c
		jr	loc_B9DE
; ---------------------------------------------------------------------------

loc_B9CF:				; CODE XREF: CallFunctionObj_1+137j
		ld	a, 1
		call	ShipSetObject
; ---------------------------------------------------------------------------
		ret	nz
		ld	a, (HighDeltaSpeed)
		add	a, (ix+1Bh)
		ld	(ix+1Bh), a

loc_B9DE:				; CODE XREF: CallFunctionObj_1+163j
		ld	(ix+25h), c
		set	0, (ix+26h)
		xor	a
		ret
; ---------------------------------------------------------------------------

ShipSetShip:				; CODE XREF: CallFunctionObj_1+16j
		ld	a, 8
		dec	c
		jr	z, loc_BA0D
		call	IntRnd_A
		and	1
		add	a, 9
		dec	c
		jr	z, loc_BA0D
		ld	a, 12h
		dec	c
		jr	z, loc_BA0D
		ld	a, 0Fh
		dec	c
		jr	z, loc_BA0D
		dec	c
		ret	nz
		call	IntRnd_A
		bit	0, a
		ld	a, 10h
		jr	z, loc_BA0D
		ld	a, 11h

loc_BA0D:				; CODE XREF: CallFunctionObj_1+180j
					; CallFunctionObj_1+18Aj ...
		push	ix
		call	ShipSetObject
; ---------------------------------------------------------------------------
		pop	iy
		ret	nz
		bit	2, (iy+26h)
		call	z, RotateSKOobj
		bit	2, (iy+21h)
		ret	z
		set	2, (ix+21h)
		xor	a
		ret
; END OF FUNCTION CHUNK	FOR CallFunctionObj_1

; =============== S U B	R O U T	I N E =======================================


RotateSKOobj:				; CODE XREF: CallFunctionObj_1+1AFp
					; SetShipMinAlloys+Cp

; FUNCTION CHUNK AT BA58 SIZE 0000002B BYTES

		ld	a, (byte_B866)
		xor	1
		ld	(byte_B866), a
		jr	z, loc_BA58
		push	ix
		call	sub_BA3F
		call	sub_BA3F
		call	sub_BA3F
		pop	ix
		ret
; End of function RotateSKOobj


; =============== S U B	R O U T	I N E =======================================




; === SetTargonStation === CODE XREF: sub_5FD5:loc_6028p
SetTargonStation:			; CODE XREF: sub_5FD5:loc_6028p
					; CallFunctionObj_1:Set3_4Targonsp
		call	TestTargonStation ; определение	возможности установки станции таргонов в межпланетном пространстве для последней подмиссии в войне с таргонами
					; Вх:  ---
					; Вых: cy =1 станция установлена
					;      hl,bc,a - не определены
		jp	nz, IntRnd_A
		ld	hl, objCoriolis
		ld	b, 8

loc_7F6F:				; CODE XREF: SetTargonStation+Ej
		ld	(hl), 0
		inc	hl
		djnz	loc_7F6F
		ld	(hl), 1
		ld	l, 6Ah ; 'j'
		ld	(hl), 0F0h ; 'Ё'
		ld	a, 4
		ld	(PilotMission3), a ; =#06 при стыковке будет захват таргонами (вроде нереальная	ситуация)
					; =#04 в системе со танцией таргонов во	время войны
		pop	hl
		ret
; End of function SetTargonStation


; =============== S U B	R O U T	I N E =======================================




; === SetVars4War === CODE XREF: sub_5FD5+6Bj
SetVars4War:				; CODE XREF: sub_5FD5+6Bj
					; InitSpeedAngle+3p
		ld	a, (PilotNumGalaxy)
		and	a
		jr	nz, loc_EEF9
		call	TestPilotInZoneCombat ;	Определяем находится ли	пилот в	зоне #3C (активных боёв)
					; Вх:  ---
					; Вых: cy =1 пилот в зоне #3C
					;	  =0 пилот в другой галактике галактике
					;	     война окончена/не начиналась
					;	     пилот в другой зоне
					;      hl,de,b,a - не определены
		jr	nc, loc_EEED
		xor	a
		ld	(planetTextiles), a
		ld	(planetFireams), a

loc_EEED:				; CODE XREF: SetVars4War+9j
		call	TestPlaneInZoneWar ; Определяем	находится ли пилот/планета в заданной зоне войны
					; Вх:  (sp) =#00 зона оккупированная таргонами
					;	    =#3C зона линии фронта (активные боевые действия)
					;	    =#78 прифронтовая зона
					; Вых: cy =1 координаты	в пределах проверяемой зоны
					;	  =0 пилот в другой галактике галактике
					;	     война окончена/не начиналась
					;	     координаты	слишком	далеко от левого края
					;      hl,de,b,a - не определены
; ---------------------------------------------------------------------------
		db 0
; ---------------------------------------------------------------------------
		ld	a, 4
		jr	c, loc_EEF6
		xor	a

loc_EEF6:				; CODE XREF: SetVars4War+18j
		ld	(PilotMission3), a ; =#06 при стыковке будет захват таргонами (вроде нереальная	ситуация)
					; =#04 в системе со танцией таргонов во	время войны

loc_EEF9:				; CODE XREF: SetVars4War+4j
		call	sub_D5B4	; гибель минеральной жизни при недостаточном количестве
					; Вых: z=1 есть	минеральная жизнь в трюме
					;	=0 минеральная жизнь померла
					;	   нет минеральной жизни
		ret	nz
		ld	(planetMinerals), a
		ret
; End of function SetVars4War


; =============== S U B	R O U T	I N E =======================================

; a - номер объекта



; === TestPilotInZoneCombat === CODE XREF: sub_7FA1+10p sub_7FF0+1Cp ...
TestPilotInZoneCombat:			; CODE XREF: sub_7FA1+10p sub_7FF0+1Cp ...
		call	TestPilotInZoneWar ; Определяем	находится ли пилот/планета в заданной зоне войны
					; Вх:  (sp) =#00 зона оккупированная таргонами
					;	    =#3C зона линии фронта (активные боевые действия)
					;	    =#78 прифронтовая зона
					; Вых: cy =1 координаты	в пределах проверяемой зоны
					;	  =0 пилот в другой галактике галактике
					;	     война окончена/не начиналась
					;	     координаты	слишком	далеко от левого края
					;      hl,de,b,a - не определены
; ---------------------------------------------------------------------------
		db 3Ch
; ---------------------------------------------------------------------------
		ret	nc
		call	TestPilotInZoneWar ; Определяем	находится ли пилот/планета в заданной зоне войны
					; Вх:  (sp) =#00 зона оккупированная таргонами
					;	    =#3C зона линии фронта (активные боевые действия)
					;	    =#78 прифронтовая зона
					; Вых: cy =1 координаты	в пределах проверяемой зоны
					;	  =0 пилот в другой галактике галактике
					;	     война окончена/не начиналась
					;	     координаты	слишком	далеко от левого края
					;      hl,de,b,a - не определены
; ---------------------------------------------------------------------------
		db 0
; ---------------------------------------------------------------------------
		ccf
		ret
; End of function TestPilotInZoneCombat


; =============== S U B	R O U T	I N E =======================================




; === TestPilotInZoneWar === CODE XREF: ReCalcCrdObj_coriolis+11p
TestPilotInZoneWar:			; CODE XREF: ReCalcCrdObj_coriolis+11p
					; sub_7FF0+16p	...
		ld	hl, Pilot_CrdYSystem

loc_D67D:				; CODE XREF: TestPlaneInZoneWar+3j
		ld	a, (PilotNumGalaxy)
		and	a
		ret	nz
		ld	a, (PilotTargonWar) ; =#81 при сбросе миссий
					; 7,=1 начало войны, предолжение участвовать
					; 6-0, чем больше число	тем дальше от левого края карты	захват планет таргонами
					;
					; =#01 война закончена
		cp	2
		ccf
		ret	nc		; выход, если a	=0/1
		add	a, a
		ex	(sp), hl
		ld	b, 5Ch ; '\'
		add	a, (hl)
		jr	nc, loc_D691
		sbc	a, a

loc_D691:				; CODE XREF: TestPilotInZoneWar+14j
		inc	hl
		ex	(sp), hl
		push	hl
		call	sub_E6C6
		jr	c, loc_D6A8
		ex	(sp), hl
		ld	b, (hl)
		inc	hl
		inc	hl
		ld	a, (hl)
		call	sub_E6C6
		pop	de
		ccf
		ret	nc
		and	a
		sbc	hl, de
		ret
; ---------------------------------------------------------------------------

loc_D6A8:				; CODE XREF: TestPilotInZoneWar+1Dj
		pop	hl
		ret
; End of function TestPilotInZoneWar


; =============== S U B	R O U T	I N E =======================================




; === TestPlaneInZoneWar === CODE XREF: RAM:loc_6ED4p RAM:6EDCp ...
TestPlaneInZoneWar:			; CODE XREF: RAM:loc_6ED4p RAM:6EDCp ...
		ld	hl,  DataOfPlanetSv+1
		jr	loc_D67D
; End of function TestPlaneInZoneWar


; =============== S U B	R O U T	I N E =======================================

; Определяем находится ли пилот/планета	в заданной зоне	войны
; Вх:  (sp) =#00 зона оккупированная таргонами
;	    =#3C зона линии фронта (активные боевые действия)
;	    =#78 прифронтовая зона
; Вых: cy =1 координаты	в пределах проверяемой зоны
;	  =0 пилот в другой галактике галактике
;	     война окончена/не начиналась
;	     координаты	слишком	далеко от левого края
;      hl,de,b,a - не определены

TestPilotInZoneWar:			; CODE XREF: ReCalcCrdObj_coriolis+11p
					; sub_7FF0+16p	...
		ld	hl, Pilot_CrdYSystem

loc_D67D:				; CODE XREF: TestPlaneInZoneWar+3j
		ld	a, (PilotNumGalaxy)
		and	a
		ret	nz
		ld	a, (PilotTargonWar) ; =#81 при сбросе миссий
					; 7,=1 начало войны, предолжение участвовать
					; 6-0, чем больше число	тем дальше от левого края карты	захват планет таргонами
					;
					; =#01 война закончена
		cp	2
		ccf
		ret	nc		; выход, если a	=0/1
		add	a, a
		ex	(sp), hl
		ld	b, 5Ch ; '\'
		add	a, (hl)
		jr	nc, loc_D691
		sbc	a, a

loc_D691:				; CODE XREF: TestPilotInZoneWar+14j
		inc	hl
		ex	(sp), hl
		push	hl
		call	sub_E6C6
		jr	c, loc_D6A8
		ex	(sp), hl
		ld	b, (hl)
		inc	hl
		inc	hl
		ld	a, (hl)
		call	sub_E6C6
		pop	de
		ccf
		ret	nc
		and	a
		sbc	hl, de
		ret
; ---------------------------------------------------------------------------

loc_D6A8:				; CODE XREF: TestPilotInZoneWar+1Dj
		pop	hl
		ret
; End of function TestPilotInZoneWar


; =============== S U B	R O U T	I N E =======================================




; === TestTargonStation === CODE XREF: SetTargonStationp
TestTargonStation:			; CODE XREF: SetTargonStationp
		ld	a, (PilotRating2) ; >=#84 в трюме десант
					; 7,=1 война была или идёт
					; 6,=1 война закончилать
					; 3-1 -	ранг (=текущая стадия войны)
		and	40h ; '@'
		ret	nz
		call	TestPilotInZoneWar ; Определяем	находится ли пилот/планета в заданной зоне войны
					; Вх:  (sp) =#00 зона оккупированная таргонами
					;	    =#3C зона линии фронта (активные боевые действия)
					;	    =#78 прифронтовая зона
					; Вых: cy =1 координаты	в пределах проверяемой зоны
					;	  =0 пилот в другой галактике галактике
					;	     война окончена/не начиналась
					;	     координаты	слишком	далеко от левого края
					;      hl,de,b,a - не определены
; ---------------------------------------------------------------------------
		db 3Ch
; ---------------------------------------------------------------------------
		ld	a, (Pilot_CrdYSystem)
		inc	a
		ret	nc
		xor	5Dh ; ']'
		ret	nz
		ld	a, (Pilot_CrdXSystem)
		xor	3
		ret
; End of function TestTargonStation


; =============== S U B	R O U T	I N E =======================================




; === j_Cmd_L_PilotInfo === CODE XREF: sub_728D+191p RAM:74A5p
j_Cmd_L_PilotInfo:			; CODE XREF: sub_728D+191p RAM:74A5p
		jp	Cmd_L_PilotInfo
; End of function j_Cmd_L_PilotInfo

; ---------------------------------------------------------------------------
		jp	Cmd_Ent_Inventory
; ---------------------------------------------------------------------------
		jp	loc_E4F9
; ---------------------------------------------------------------------------
		jp	loc_E509

; =============== S U B	R O U T	I N E =======================================

; Attributes: thunk



