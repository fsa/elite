; 06_objects_render - разобрано из kladov.asm
; Количество подпрограмм: 5

; === PrnObjIfVisible === CODE XREF: sub_A28D+28j
PrnObjIfVisible:			; CODE XREF: sub_A28D+28j
					; sub_9D40+5B88p

; FUNCTION CHUNK AT A73B SIZE 0000009E BYTES

		call	ResetSignInExtSlot
		ld	b, a
		cp	4
		ret	nc
		push	ix
		pop	hl
		bit	1, a
		jr	nz, loc_A6F1
		ld	de, 6
		add	hl, de

loc_A6F1:				; CODE XREF: PrnObjIfVisible+Cj
		ld	e, (hl)
		inc	hl
		ld	d, (hl)
		inc	hl
		ld	c, (hl)		; cde -	расстояние до объекта
		srl	a
		xor	b
		rrca
		xor	c
		and	80h ; 'А'
		ret	nz
		ld	(DistanceToObj), de
		ld	h, 0
		ld	l, (iy+1)
		push	hl
		and	a
		sbc	hl, de
		jr	c, loc_A70F
		pop	hl
		ret
; ---------------------------------------------------------------------------

loc_A70F:				; CODE XREF: PrnObjIfVisible+2Cj
		push	de
		push	de
		bit	1, b
		jr	nz, loc_A73B
		ex	de, hl
		ld	d, (ix+1)
		ld	e, (ix+0)
		xor	a
		ld	c, a

loc_A71E:
		call	Div_C_DEc_HLa
; ---------------------------------------------------------------------------
		ld	a, c
		ld	(CrdXProjection), a
		pop	hl
		ld	d, (ix+4)
		ld	e, (ix+3)
		xor	a
		ld	c, a
		call	Div_C_DEc_HLa
; ---------------------------------------------------------------------------
		ld	a, c
		ld	(CrdYProjection), a
		ld	b, (ix+2)
		jp	loc_A75E
; End of function PrnObjIfVisible

; ---------------------------------------------------------------------------
; START	OF FUNCTION CHUNK FOR PrnObjIfVisible

loc_A73B:				; CODE XREF: PrnObjIfVisible+34j
		ex	de, hl
		ld	d, (ix+7)
		ld	e, (ix+6)
		xor	a
		ld	c, a
		call	Div_C_DEc_HLa
; ---------------------------------------------------------------------------
		ld	a, c
		ld	(CrdXProjection), a
		pop	hl
		ld	d, (ix+4)
		ld	e, (ix+3)
		xor	a
		ld	c, a
		call	Div_C_DEc_HLa
; ---------------------------------------------------------------------------
		ld	a, c
		ld	(CrdYProjection), a
		ld	b, (ix+8)

loc_A75E:				; CODE XREF: PrnObjIfVisible+59j
		ld	a, (ix+5)
		sla	b
		rla
		rla
		ld	(CrdXYsign), a
		pop	hl
		pop	de
		xor	a
		ld	c, a
		call	Div_C_DEc_HLa
; ---------------------------------------------------------------------------
		ld	h, c
		ld	a, c
		ld	(RadiusBlast), a
		ld	e, c
		ld	a, (CrdYProjection)
		ld	c, a
		sub	h
		jr	c, loc_A781
		sub	40h ; '@'
		jp	nc, objPrintLaser ; de - координаты откуда стреляет

loc_A781:				; CODE XREF: PrnObjIfVisible+9Bj
		ld	a, (CrdXProjection)
		ld	b, a
		sub	h
		jr	c, loc_A78D
		sub	80h ; 'А'
		jp	nc, objPrintLaser ; de - координаты откуда стреляет

loc_A78D:				; CODE XREF: PrnObjIfVisible+A7j
		ld	a, b
		add	a, c
		ld	d, a
		jr	c, loc_A7D2	; вне экрана
		sub	e
		jr	nc, loc_A7A3	; далеко от центра экрана
		ld	a, (ix+7)
		cp	2Ch ; ','
		jr	nc, loc_A7A3

loc_A79C:				; DATA XREF: sub_9D40+5B78w
		ld	a, 5
		nop
		nop
		ld	(ObjInTarget), a

loc_A7A3:				; CODE XREF: PrnObjIfVisible+B4j
					; PrnObjIfVisible+BBj
		bit	6, (ix+20h)
		jr	z, loc_A7D2
		bit	2, (ix+21h)
		jr	z, loc_A7D2
		ld	a, (WhatOnScreen)
		bit	1, a
		ld	a, (ix+0Eh)
		ld	e, (ix+7)
		jr	z, loc_A7C2
		ld	a, (ix+0Ah)
		ld	e, (ix+1)

loc_A7C2:				; CODE XREF: PrnObjIfVisible+DBj
		ld	b, a
		and	7Fh ; ''
		cp	40h ; '@'
		jr	c, loc_A7D2
		ld	a, d
		cp	20h ; ' '
		jr	nc, loc_A7D2
		res	5, (ix+26h)

loc_A7D2:				; CODE XREF: PrnObjIfVisible+B1j
					; PrnObjIfVisible+C8j ...
		bit	4, (ix+20h)
		jp	sub_7E02
; END OF FUNCTION CHUNK	FOR PrnObjIfVisible

; =============== S U B	R O U T	I N E =======================================

; если включена	невидимость и это ASP из 2й миссии, то печать только луча лазера



; === ShipSet2Alloys === CODE XREF: CallFunctionObj_1+22Cj
ShipSet2Alloys:				; CODE XREF: CallFunctionObj_1+22Cj
		ld	a, 3
		call	SetShipMinAlloys
; ---------------------------------------------------------------------------
		ret	nz
		ld	a, 3
		jr	SetShipMinAlloys
; ---------------------------------------------------------------------------

SetCargo:				; CODE XREF: CallFunctionObj_1+1Ej
		bit	0, c
		ld	a, 4
		jr	z, SetShipEPodCargo
		jr	SetPlayerCargo
; ---------------------------------------------------------------------------

SetCobra_EscPod:			; CODE XREF: CallFunctionObj_1+22j
		bit	0, c
		ld	a, 2
		jr	z, SetShipEPodCargo
		ld	a, 9
		ld	ix, SlotObj_05
		call	SetPlayerObjSlot
		call	RndDirectionObj
		ld	a, 80h ; 'А'
		ld	(ix+5),	a
		ld	a, 1Ch
		ld	(ix+3),	a
		res	6, (ix+20h)
		ld	a, 4Ch ; 'L'
		ld	(ix+6),	a
		ld	a, 0C0h	; '└'
		ld	(ix+1Eh), a
		ld	a, (HighDeltaSpeed)
		add	a, (ix+1Bh)
		ld	(ix+1Bh), a
		xor	a
		ret
; ---------------------------------------------------------------------------

SetShipEPodCargo:			; CODE XREF: CallFunctionObj_1+28Cj
					; CallFunctionObj_1+294j
		push	ix
		call	ShipSetObject
; ---------------------------------------------------------------------------
		ex	(sp), ix
		pop	hl
		ret	nz
		push	hl
		ex	(sp), ix
		res	6, (ix+20h)
		ld	a, 0Ah
		ld	(ix+26h), a
		ld	a, 80h ; 'А'
		ld	(ix+1Dh), a
		xor	a
		pop	ix
		ret
; ---------------------------------------------------------------------------

Set2Minerals:				; CODE XREF: CallFunctionObj_1+26j
		ld	a, 5
		call	SetShipMinAlloys
; ---------------------------------------------------------------------------
		ret	nz
		ld	a, 5
; END OF FUNCTION CHUNK	FOR CallFunctionObj_1

; =============== S U B	R O U T	I N E =======================================

; Attributes: noreturn

SetShipMinAlloys:			; CODE XREF: CallFunctionObj_1+280p
					; CallFunctionObj_1+286j ...
		push	ix
		call	ShipSetObject
; ---------------------------------------------------------------------------
		ex	(sp), ix
		pop	hl
		ret	nz
		push	hl
		ex	(sp), ix
		call	RotateSKOobj
		ld	a, 0Ah
		ld	(ix+26h), a
		res	6, (ix+20h)
		set	7, (ix+1Dh)
		pop	ix
		xor	a
		ret
; End of function SetShipMinAlloys


; =============== S U B	R O U T	I N E =======================================


SetShip4Intro:				; CODE XREF: sub_728D+132p
					; CallFunctionObj_1+2Aj
		ld	ix, objMissile
		ld	a, c
		add	a, 2
		call	SetObjInSlot_Intro
		ld	a, 0C0h	; '└'
		ld	(ix+1Dh), a
		ld	(ix+1Eh), a
		xor	a
		ld	(ix+6),	a
		ld	(ix+0),	a
		ld	(ix+1),	a
		ld	(ix+3),	a
		ld	(ix+4),	a
		ld	(ix+8),	a
		ld	(ix+1Bh), a
		res	6, (ix+20h)
		inc	a
		inc	a
		ld	(ix+7),	a
		push	ix
		pop	hl
		ld	de, 9
		ld	b, d
		ld	c, e
		add	hl, de
		ex	de, hl
		ld	hl, SKO4Intro

loc_BBB3:				; CODE XREF: SetShip4Intro+41j
		inc	de
		ldi
		jp	pe, loc_BBB3
		ret
; End of function SetShip4Intro

; ---------------------------------------------------------------------------
; START	OF FUNCTION CHUNK FOR CallFunctionObj_1

SetRndShip:				; CODE XREF: CallFunctionObj_1+8j
		ld	a, (cargoSlaves)
		ld	c, a
		ld	a, (cargoNarc)
		add	a, c
		add	a, a
		ld	c, a
		ld	a, (cargoFireams)
		add	a, c
		ld	c, a
		ld	a, (Mission2)
		and	a
		jr	nz, loc_BBFD
		call	IntRnd_A
		cp	1Eh
		jr	nc, loc_BBFD
		ld	a, (PilotExtWeapon) ; возможность выдать ASP для 2й миссии
		and	42h ; 'B'
		cp	2
		jr	nz, loc_BBFD
		dec	a		; ставим ASP для 2й миссии
		ld	(Mission2), a
		ld	a, 0Fh
		ld	hl, sub_BBF2
		push	hl
		push	af
		call	FindSlot4Obj
		ret	nz
		ld	a, 0Dh
		jr	loc_BC65
; END OF FUNCTION CHUNK	FOR CallFunctionObj_1

; =============== S U B	R O U T	I N E =======================================

; Attributes: noreturn



; === ShipSetObject === CODE XREF: CallFunctionObj_1+167p
ShipSetObject:				; CODE XREF: CallFunctionObj_1+167p
					; CallFunctionObj_1+1A5p ...
		ld	c, a
		push	ix
		call	sub_FACA
; ---------------------------------------------------------------------------
		pop	hl
		ret	nz
		push	hl
		ld	a, c
		call	sub_B71C	; hl - адрес слота родиителя
; ---------------------------------------------------------------------------
		pop	hl
		push	ix
		pop	de
		ld	bc, 1Bh
		ldir
		xor	a
		ret
; End of function ShipSetObject


; =============== S U B	R O U T	I N E =======================================


FindSlot4Obj:				; CODE XREF: ReCalcCrdObj_coriolis+81p
					; SetPlayerObj+1p ...
		ld	ix, objCoriolis
		ld	de, 27h	; '''
		ld	b, 6

loc_B93F:				; CODE XREF: FindSlot4Obj+10j
		add	ix, de
		ld	a, (ix+22h)
		and	a
		ret	z
		djnz	loc_B93F
		inc	b
		ret
; End of function FindSlot4Obj


; =============== S U B	R O U T	I N E =======================================


SetPlayerObj:				; CODE XREF: SetPlayerAlloys:SetPlayerCargop
		ld	c, a
		call	FindSlot4Obj
		ret	nz
		ld	a, c
; End of function SetPlayerObj


; =============== S U B	R O U T	I N E =======================================


SetPlayerObjSlot:			; CODE XREF: sub_9391+18j
					; CallFunctionObj_1+29Cp
		call	sub_5EC5	; запуск ракеты	игроком
					; запуск спасательной капсулы (установка кобры)
		res	7, (ix+0Eh)
		res	7, (ix+16h)
		ld	(ix+0),	a
		ld	(ix+1),	a
		ld	(ix+4),	a
		ld	(ix+6),	a
		ld	(ix+7),	a
		ret
; End of function SetPlayerObjSlot

; ---------------------------------------------------------------------------
; START	OF FUNCTION CHUNK FOR CallFunctionObj_1

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




; === ShipSetShip === CODE XREF: CallFunctionObj_1+16j
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




; === objPrnObj === CODE XREF: sub_A7D9+14j
objPrnObj:				; CODE XREF: sub_A7D9+14j
		push	iy
		call	CalcMatrixRotate
		pop	iy
		ld	hl, (DistanceToObj)
		add	hl, hl
		ld	b, 1Fh
		jr	c, loc_A861
		ld	a, h
		cp	b
		jr	nc, loc_A861
		ld	b, h

loc_A861:				; CODE XREF: objPrnObj+Dj
					; objPrnObj+11j
		ld	a, b
		sub	2
		jr	nc, loc_A867
		xor	a

loc_A867:				; CODE XREF: objPrnObj+17j
		ld	(DistanceToObj), a
		ld	a, 0FDh	; '¤'
		ld	(PointTablCrdPrn), a
		call	CalcVisibleVerges
		call	MakeTablPrnCrdVertex
		call	PrintObject

loc_A878:				; CODE XREF: sub_7E02:loc_7EA6j
		bit	3, (ix+20h)
		ret	z		; нет выстрела из лазера
		ld	a, (iy+14h)	; вершина с лазером
		call	GetPrnCrdVertex	; Получение печатных координат вершины из таблицы по номеру
					; Вх:  a - номер вершины
					; Вых: de - печатные координаты	вершины
					;      a - знаки координат
					;      z =1 вершина печатается
					;      hl- не определен
		ld	c, a

objPrnLaser:				; CODE XREF: sub_A7D9+110j
		call	SetBit2ExtSlot_rndA
		ld	l, a
		sub	0FFh
		neg
		ld	h, a
		ld	a, c
		cpl
		and	3
		sla	c
		sla	c
		or	c
		ld	c, a
		call	PrnLineInBuf_clip
		jr	loc_A8A7
; ---------------------------------------------------------------------------
unk_A89C:	db  68h	; h		; DATA XREF: CopyLocator2Scr+3Fo
		db  94h	; Ф
		db  95h	; Х
		db 0B4h	; ┤
		db 0B5h	; ╡
		db  87h	; З
		db  88h	; И
		db 0A7h	; з
		db 0A8h	; и
		db 0A8h	; и
		db 0FFh
; ---------------------------------------------------------------------------

loc_A8A7:				; CODE XREF: objPrnObj+4Dj
		res	3, (ix+20h)
		ret
; End of function objPrnObj


; =============== S U B	R O U T	I N E =======================================


objDestroyed2:				; CODE XREF: sub_7E02+8j
		ld	a, (CrdXProjection)
		ld	(ix+1Ch), a
		ld	a, (CrdYProjection)
		ld	(ix+1Dh), a
		ld	a, (RadiusBlast)
		ld	(ix+1Fh), a
		ld	a, (WhatOnScreen)
		bit	0, a
		ld	a, (CrdXYsign)
		jr	z, loc_A8CA
		xor	2

loc_A8CA:				; CODE XREF: objDestroyed2+1Aj
		ld	c, a
		xor	a
		rr	c
		rla
		rr	c
		rla
		xor	3
		add	a, 0Ch
		ld	(ix+1Eh), a
		set	5, (ix+20h)
		ret
; End of function objDestroyed2

; ---------------------------------------------------------------------------
; START	OF FUNCTION CHUNK FOR sub_A7D9

objPrintLaser:				; CODE XREF: PrnObjIfVisible+9Fj
					; PrnObjIfVisible+ABj ...
		call	GetCrdsProjection ; de - координаты откуда стреляет
		bit	3, (ix+20h)
		call	SetCrdCDEinExtSlot
		ret	z
		jp	objPrnLaser
; END OF FUNCTION CHUNK	FOR sub_A7D9

; =============== S U B	R O U T	I N E =======================================


NextExtSlot:				; CODE XREF: sub_9D40+11Dp
		ld	hl, (word_B452)
		ld	de, 0Ah
		add	hl, de
		ld	(word_B452), hl
		ld	e, 27h ; '''
		ret
; End of function NextExtSlot

; ---------------------------------------------------------------------------
		db  7Bh	; {


