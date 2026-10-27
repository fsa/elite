; 09_drawing_primitives - разобрано из kladov.asm
; Количество подпрограмм: 5

; === FillAttrLineFrom_HL === CODE XREF: RAM:F0E4p
FillAttrLineFrom_HL:			; CODE XREF: RAM:F0E4p
		ld	b, 1Eh
		call	Fill_HL_A_Len_B
		inc	hl
		inc	hl
		ret
; End of function FillAttrLineFrom_HL

; ---------------------------------------------------------------------------

locret_D55C:				; CODE XREF: RAM:D8C0j
		ret
; ---------------------------------------------------------------------------
		db    0
		db    0

; =============== S U B	R O U T	I N E =======================================


PrintHorLine:				; CODE XREF: ClsAndPrintRamk+2Dp
					; ClsAndPrintRamk+4Bp ...
		ld	a, 0FFh
		ld	b, 20h ; ' '
; End of function PrintHorLine


; =============== S U B	R O U T	I N E =======================================


Fill_HL_A_Len_B:			; CODE XREF: FillAttrLineFrom_HL+2p
					; Fill_HL_A_Len_B+2j ...
		ld	(hl), a
		inc	hl
		djnz	Fill_HL_A_Len_B
		ret
; End of function Fill_HL_A_Len_B


; =============== S U B	R O U T	I N E =======================================


SetAttrFrom_DE:				; CODE XREF: RAM:E719p	sub_E87E+61p ...
		ld	hl, 5841h

loc_D56B:				; CODE XREF: SetAttrFrom_DE+22j
		ld	a, (de)
		inc	de
		push	af
		and	0F8h ; '°'
		rrca
		rrca
		rrca
		ld	b, a
		push	hl
		ld	a, 5
		call	Fill_HL_A_Len_B
		pop	hl
		pop	af
		and	7
		jr	z, loc_D58C
		ld	b, a

loc_D581:				; CODE XREF: SetAttrFrom_DE:loc_D588j
		ld	a, 20h ; ' '
		add	a, l
		ld	l, a
		jr	nc, loc_D588
		inc	h

loc_D588:				; CODE XREF: SetAttrFrom_DE+1Dj
		djnz	loc_D581
		jr	loc_D56B
; ---------------------------------------------------------------------------

loc_D58C:				; CODE XREF: SetAttrFrom_DE+16j
		ld	hl, 5822h
		ld	a, 7
		ld	(hl), a
		inc	hl
		ld	(hl), a
		ret
; End of function SetAttrFrom_DE

; ---------------------------------------------------------------------------
; START	OF FUNCTION CHUNK FOR sub_D470

loc_D595:				; CODE XREF: sub_D470+1Bj
		sub	0D7h ; '╫'
		ld	hl, unk_FD99
		jr	loc_D5A1
; END OF FUNCTION CHUNK	FOR sub_D470
; ---------------------------------------------------------------------------
; START	OF FUNCTION CHUNK FOR PrintMessage1_A

loc_D59C:				; CODE XREF: PrintMessage1_A+15j
		sub	80h ; 'А'
		ld	hl, byte_FDB3

loc_D5A1:				; CODE XREF: sub_D470+12Aj
		add	a, a
		ld	d, 0
		ld	e, a
		add	hl, de
		ld	a, (hl)
		push	hl
		call	sub_D499
		pop	hl
		inc	hl
		ld	a, (hl)
		cp	3Fh ; '?'
		ret	z
		jp	sub_D499
; END OF FUNCTION CHUNK	FOR PrintMessage1_A

; =============== S U B	R O U T	I N E =======================================

; гибель минеральной жизни при недостаточном количестве
; Вых: z=1 есть	минеральная жизнь в трюме
;	=0 минеральная жизнь померла
;	   нет минеральной жизни



; === FillAttr_21line === CODE XREF: RAM:E71Cp	sub_E87E+64j ...
FillAttr_21line:			; CODE XREF: RAM:E71Cp	sub_E87E+64j ...
		ld	hl, 5AA1h
		ld	a, 5
; End of function FillAttr_21line


; =============== S U B	R O U T	I N E =======================================


FillAttrLineFrom_HL:			; CODE XREF: RAM:F0E4p
		ld	b, 1Eh
		call	Fill_HL_A_Len_B
		inc	hl
		inc	hl
		ret
; End of function FillAttrLineFrom_HL

; ---------------------------------------------------------------------------

locret_D55C:				; CODE XREF: RAM:D8C0j
		ret
; ---------------------------------------------------------------------------
		db    0
		db    0

; =============== S U B	R O U T	I N E =======================================


PrintHorLine:				; CODE XREF: ClsAndPrintRamk+2Dp
					; ClsAndPrintRamk+4Bp ...
		ld	a, 0FFh
		ld	b, 20h ; ' '
; End of function PrintHorLine


; =============== S U B	R O U T	I N E =======================================


Fill_HL_A_Len_B:			; CODE XREF: FillAttrLineFrom_HL+2p
					; Fill_HL_A_Len_B+2j ...
		ld	(hl), a
		inc	hl
		djnz	Fill_HL_A_Len_B
		ret
; End of function Fill_HL_A_Len_B


; =============== S U B	R O U T	I N E =======================================


SetAttrFrom_DE:				; CODE XREF: RAM:E719p	sub_E87E+61p ...
		ld	hl, 5841h

loc_D56B:				; CODE XREF: SetAttrFrom_DE+22j
		ld	a, (de)
		inc	de
		push	af
		and	0F8h ; '°'
		rrca
		rrca
		rrca
		ld	b, a
		push	hl
		ld	a, 5
		call	Fill_HL_A_Len_B
		pop	hl
		pop	af
		and	7
		jr	z, loc_D58C
		ld	b, a

loc_D581:				; CODE XREF: SetAttrFrom_DE:loc_D588j
		ld	a, 20h ; ' '
		add	a, l
		ld	l, a
		jr	nc, loc_D588
		inc	h

loc_D588:				; CODE XREF: SetAttrFrom_DE+1Dj
		djnz	loc_D581
		jr	loc_D56B
; ---------------------------------------------------------------------------

loc_D58C:				; CODE XREF: SetAttrFrom_DE+16j
		ld	hl, 5822h
		ld	a, 7
		ld	(hl), a
		inc	hl
		ld	(hl), a
		ret
; End of function SetAttrFrom_DE

; ---------------------------------------------------------------------------
; START	OF FUNCTION CHUNK FOR sub_D470

loc_D595:				; CODE XREF: sub_D470+1Bj
		sub	0D7h ; '╫'
		ld	hl, unk_FD99
		jr	loc_D5A1
; END OF FUNCTION CHUNK	FOR sub_D470
; ---------------------------------------------------------------------------
; START	OF FUNCTION CHUNK FOR PrintMessage1_A

loc_D59C:				; CODE XREF: PrintMessage1_A+15j
		sub	80h ; 'А'
		ld	hl, byte_FDB3

loc_D5A1:				; CODE XREF: sub_D470+12Aj
		add	a, a
		ld	d, 0
		ld	e, a
		add	hl, de
		ld	a, (hl)
		push	hl
		call	sub_D499
		pop	hl
		inc	hl
		ld	a, (hl)
		cp	3Fh ; '?'
		ret	z
		jp	sub_D499
; END OF FUNCTION CHUNK	FOR PrintMessage1_A

; =============== S U B	R O U T	I N E =======================================

; гибель минеральной жизни при недостаточном количестве
; Вых: z=1 есть	минеральная жизнь в трюме
;	=0 минеральная жизнь померла
;	   нет минеральной жизни



; === PrintHorLine === CODE XREF: ClsAndPrintRamk+2Dp
PrintHorLine:				; CODE XREF: ClsAndPrintRamk+2Dp
					; ClsAndPrintRamk+4Bp ...
		ld	a, 0FFh
		ld	b, 20h ; ' '
; End of function PrintHorLine


; =============== S U B	R O U T	I N E =======================================


Fill_HL_A_Len_B:			; CODE XREF: FillAttrLineFrom_HL+2p
					; Fill_HL_A_Len_B+2j ...
		ld	(hl), a
		inc	hl
		djnz	Fill_HL_A_Len_B
		ret
; End of function Fill_HL_A_Len_B


; =============== S U B	R O U T	I N E =======================================


SetAttrFrom_DE:				; CODE XREF: RAM:E719p	sub_E87E+61p ...
		ld	hl, 5841h

loc_D56B:				; CODE XREF: SetAttrFrom_DE+22j
		ld	a, (de)
		inc	de
		push	af
		and	0F8h ; '°'
		rrca
		rrca
		rrca
		ld	b, a
		push	hl
		ld	a, 5
		call	Fill_HL_A_Len_B
		pop	hl
		pop	af
		and	7
		jr	z, loc_D58C
		ld	b, a

loc_D581:				; CODE XREF: SetAttrFrom_DE:loc_D588j
		ld	a, 20h ; ' '
		add	a, l
		ld	l, a
		jr	nc, loc_D588
		inc	h

loc_D588:				; CODE XREF: SetAttrFrom_DE+1Dj
		djnz	loc_D581
		jr	loc_D56B
; ---------------------------------------------------------------------------

loc_D58C:				; CODE XREF: SetAttrFrom_DE+16j
		ld	hl, 5822h
		ld	a, 7
		ld	(hl), a
		inc	hl
		ld	(hl), a
		ret
; End of function SetAttrFrom_DE

; ---------------------------------------------------------------------------
; START	OF FUNCTION CHUNK FOR sub_D470

loc_D595:				; CODE XREF: sub_D470+1Bj
		sub	0D7h ; '╫'
		ld	hl, unk_FD99
		jr	loc_D5A1
; END OF FUNCTION CHUNK	FOR sub_D470
; ---------------------------------------------------------------------------
; START	OF FUNCTION CHUNK FOR PrintMessage1_A

loc_D59C:				; CODE XREF: PrintMessage1_A+15j
		sub	80h ; 'А'
		ld	hl, byte_FDB3

loc_D5A1:				; CODE XREF: sub_D470+12Aj
		add	a, a
		ld	d, 0
		ld	e, a
		add	hl, de
		ld	a, (hl)
		push	hl
		call	sub_D499
		pop	hl
		inc	hl
		ld	a, (hl)
		cp	3Fh ; '?'
		ret	z
		jp	sub_D499
; END OF FUNCTION CHUNK	FOR PrintMessage1_A

; =============== S U B	R O U T	I N E =======================================

; гибель минеральной жизни при недостаточном количестве
; Вых: z=1 есть	минеральная жизнь в трюме
;	=0 минеральная жизнь померла
;	   нет минеральной жизни



; === PrnLineInBuf_clip === CODE XREF: PrintObject_2+91p
PrnLineInBuf_clip:			; CODE XREF: PrintObject_2+91p
					; sub_7C60+C5p	...

; FUNCTION CHUNK AT B0B6 SIZE 00000082 BYTES
; FUNCTION CHUNK AT B162 SIZE 00000029 BYTES

		ld	a, d
		and	l
		inc	a
		ret	z
		ld	a, e
		and	h
		inc	a
		ret	z
		ld	a, c
		add	a, a
		add	a, a
		xor	c
		cpl
		ld	c, a
		call	sub_B095
		ld	a, d
		sub	78h ; 'x'
		jr	c, loc_B065
		ld	b, 14h
		call	loc_B21B
		ld	e, a
		ld	b, 0
		call	sub_B09C

loc_B065:				; CODE XREF: PrnLineInBuf_clip+14j
		ld	a, e
		sub	3Fh ; '?'
		jr	c, loc_B073
		ld	b, 28h ; '('
		call	loc_B1D7
		ld	d, a
		call	sub_B095

loc_B073:				; CODE XREF: PrnLineInBuf_clip+24j
		ld	a, h
		sub	78h ; 'x'
		jr	c, loc_B085
		ex	de, hl
		ld	b, 5
		call	loc_B21B
		ld	e, a
		ex	de, hl
		ld	b, 0
		call	loc_B0A6

loc_B085:				; CODE XREF: PrnLineInBuf_clip+32j
		ld	a, l
		sub	3Fh ; '?'
		jr	c, loc_B0B6
		ex	de, hl
		ld	b, 0Ah
		call	loc_B1D7
		ld	d, a
		ex	de, hl
		jp	loc_B0B6
; End of function PrnLineInBuf_clip


; =============== S U B	R O U T	I N E =======================================




; === objPrnPixel === CODE XREF: sub_78B1+13Fp
objPrnPixel:				; CODE XREF: sub_78B1+13Fp
					; sub_A7D9:loc_A82Ap ...
		ld	a, b
		call	22B1h
		ld	d, a
		inc	d
		ld	a, 1

loc_A844:				; CODE XREF: objPrnPixel+Aj
		rrca
		dec	d
		jr	nz, loc_A844

loc_A848:				; DATA XREF: RAM:B35Dw	RAM:B38Fw
		set	7, h
		xor	(hl)
		ld	(hl), a
		ret
; End of function objPrnPixel


; =============== S U B	R O U T	I N E =======================================


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


