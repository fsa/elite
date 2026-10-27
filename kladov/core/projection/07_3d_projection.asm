; 07_3d_projection - разобрано из kladov.asm
; Количество подпрограмм: 35

; === AddCrdNobj_CrdNvertex === CODE XREF: MakeTablPrnCrdVertex+332p
AddCrdNobj_CrdNvertex:			; CODE XREF: MakeTablPrnCrdVertex+332p
					; MakeTablPrnCrdVertex+351p ...
		ld	c, d
		rl	e
		rl	d
		ld	a, b
		xor	c
		jr	nz, loc_AB5B
		add	hl, de
		ld	a, b
		ret
; ---------------------------------------------------------------------------

loc_AB5B:				; CODE XREF: AddCrdNobj_CrdNvertex+7j
		sbc	hl, de
		jr	nc, loc_AB66
		add	hl, de
		ex	de, hl
		and	a
		sbc	hl, de
		ld	a, c
		ret
; ---------------------------------------------------------------------------

loc_AB66:				; CODE XREF: AddCrdNobj_CrdNvertex+Ej
		ld	a, h
		or	l
		ld	a, c
		ret	z
		pop	de
		pop	af
		neg
		push	af
		push	de
		dec	hl
		ld	a, b
		ret
; End of function AddCrdNobj_CrdNvertex

; ---------------------------------------------------------------------------
; START	OF FUNCTION CHUNK FOR MakeTablPrnCrdVertex

RotateCrdVertex:			; CODE XREF: MakeTablPrnCrdVertex+2E0j
					; MakeTablPrnCrdVertex+2F8j
		ld	a, (VisiblePrevVertex)
		and	a
		jr	nz, loc_AB90
		ld	a, (CrdNVertex)	; координата N текущей вершины
		rrca
		jr	c, loc_AB90
		call	sub_ABC0
		ld	(VertexCrdX), hl
		call	sub_ABC0
		ld	(VertexCrdY), hl
		call	sub_ABC0
		jr	loc_AB9F
; ---------------------------------------------------------------------------

loc_AB90:				; CODE XREF: MakeTablPrnCrdVertex+1AEj
					; MakeTablPrnCrdVertex+1B4j
		call	RotateCrdN
		ld	(VertexCrdX), hl
		call	RotateCrdN
		ld	(VertexCrdY), hl
		call	RotateCrdN

loc_AB9F:				; CODE XREF: MakeTablPrnCrdVertex+1C5j
		ld	(VertexCrdZ), hl
		xor	a
		ld	(VertexCrdXlow), a
		ld	(VertexCrdYlow), a
		ld	(VertexCrdZlow), a
		jp	loc_ACDF
; END OF FUNCTION CHUNK	FOR MakeTablPrnCrdVertex

; =============== S U B	R O U T	I N E =======================================




; === CaclNormalVector === CODE XREF: CalcLenVector+19p
CaclNormalVector:			; CODE XREF: CalcLenVector+19p
					; CalcLenVector+1Cp ...
		ld	h, (ix+1)
		ld	l, (ix+0)
		push	hl
		res	7, h
		call	Div_BC_HLAdivDE_1
; ---------------------------------------------------------------------------
		srl	b
		rr	c
		pop	hl
		bit	7, h
		jr	z, loc_9EE6
		set	7, b

loc_9EE6:				; CODE XREF: CaclNormalVector+13j
		ld	(ix+0),	c
		ld	(ix+1),	b
		inc	ix
		inc	ix
		ret
; End of function CaclNormalVector


; =============== S U B	R O U T	I N E =======================================

; Attributes: noreturn

Sqrt_H_HL:				; CODE XREF: j_Sqrt_H_HLj
					; CalcLenVector+13p
		ld	a, h
		cp	10h
		jr	nc, sub_9F18
		and	a
		jr	z, loc_9F0A
		ld	b, 4

loc_9EFB:				; CODE XREF: Sqrt_H_HL+Bj
		add	hl, hl
		djnz	loc_9EFB
		call	sub_9F18
; ---------------------------------------------------------------------------
		srl	h
		rr	l
		srl	h
		rr	l
		ret
; ---------------------------------------------------------------------------

loc_9F0A:				; CODE XREF: Sqrt_H_HL+6j
		ld	h, l
		ld	l, a
		call	sub_9F18
; ---------------------------------------------------------------------------
		ld	b, 4

loc_9F11:				; CODE XREF: Sqrt_H_HL+24j
		srl	h
		rr	l
		djnz	loc_9F11
		ret
; End of function Sqrt_H_HL


; =============== S U B	R O U T	I N E =======================================

; Attributes: noreturn



; === CalcCrdCoriolis === CODE XREF: CalcCrdCoriolis2+5j
CalcCrdCoriolis:			; CODE XREF: CalcCrdCoriolis2+5j
		ld	a, (J_Drive_Coriolis)
		and	a
		ld	ix, objCoriolis
		jr	nz, loc_7874
		ld	a, (ix+22h)
		and	a
		ret	nz
		dec	a
		ld	(J_Drive_Coriolis), a

loc_7874:				; CODE XREF: CalcCrdCoriolis2+1AA1j
		call	ReCalcCrdObj_coriolis
		ld	a, (ix+8)
		and	7Fh ; ''
		ret	nz
		ld	a, (ix+2)
		and	7Fh ; ''
		ret	nz
		ld	a, (ix+5)
		and	7Fh ; ''
		ret	nz

loc_7889:				; CODE XREF: ReCalcCrdObj_coriolis+6Dj
		bit	7, (ix+26h)
		ret	nz
		xor	a
		ld	(J_Drive_Coriolis), a

loc_7892:				; CODE XREF: ReCalcCrdObj_coriolis+62p
		ld	a, 0F0h	; 'Ё'
		ld	(ix+22h), a
		ret
; END OF FUNCTION CHUNK	FOR CalcCrdCoriolis2

; =============== S U B	R O U T	I N E =======================================


PrintStars:				; CODE XREF: sub_728D+264p RAM:753Ep ...
		ld	a, (StarsFastScroll)
		dec	a
		jr	nz, sub_78B1
		call	sub_F9A8
		ld	a, 1Eh
		ld	(HighDeltaSpeed), a
		call	sub_78B1
		ld	a, 10h
		ld	(HighDeltaSpeed), a
		call	sub_78B1
; End of function PrintStars


; =============== S U B	R O U T	I N E =======================================




; === CalcCrdCoriolis2 === CODE XREF: sub_728D+226p
CalcCrdCoriolis2:			; CODE XREF: sub_728D+226p

; FUNCTION CHUNK AT 7861 SIZE 00000037 BYTES

		ld	a, (InFreeSpace)
		and	a
		ret	nz
		jp	CalcCrdCoriolis
; End of function CalcCrdCoriolis2

; ---------------------------------------------------------------------------
		db    0
		db    0
		db    0
		db    0
		db    0
		db    8
		db    8
		db  10h
		db    0
		db    0
		db    0
		db    0
		db  3Eh	; >
		db    0
		db    0
		db    0
		db    0
		db    0
		db    0
		db    0
		db    0
		db  18h
		db  18h
		db    0
		db    0
		db    0
		db    2
		db    4
		db    8
		db  10h
		db  20h
		db    0
; ---------------------------------------------------------------------------
; START	OF FUNCTION CHUNK FOR sub_D499

loc_5DF0:				; CODE XREF: sub_D499+AAj
		pop	de
		ld	b, 8
		nop
		nop
		call	sub_D546
		nop
		nop
		ret
; END OF FUNCTION CHUNK	FOR sub_D499

; =============== S U B	R O U T	I N E =======================================




; === CalcCrdSunPlanet === CODE XREF: sub_728D+22Cp
CalcCrdSunPlanet:			; CODE XREF: sub_728D+22Cp
					; LaunchDocking+F1p
		ld	ix, SunCrd
		call	j_CalcNewCrdObj_Pl
		ld	ix, PlanetCrd
		jp	CalcCrdSunPlanet_2
; End of function CalcCrdSunPlanet

; ---------------------------------------------------------------------------
; START	OF FUNCTION CHUNK FOR CalcCrdCoriolis2

CalcCrdCoriolis:			; CODE XREF: CalcCrdCoriolis2+5j
		ld	a, (J_Drive_Coriolis)
		and	a
		ld	ix, objCoriolis
		jr	nz, loc_7874
		ld	a, (ix+22h)
		and	a
		ret	nz
		dec	a
		ld	(J_Drive_Coriolis), a

loc_7874:				; CODE XREF: CalcCrdCoriolis2+1AA1j
		call	ReCalcCrdObj_coriolis
		ld	a, (ix+8)
		and	7Fh ; ''
		ret	nz
		ld	a, (ix+2)
		and	7Fh ; ''
		ret	nz
		ld	a, (ix+5)
		and	7Fh ; ''
		ret	nz

loc_7889:				; CODE XREF: ReCalcCrdObj_coriolis+6Dj
		bit	7, (ix+26h)
		ret	nz
		xor	a
		ld	(J_Drive_Coriolis), a

loc_7892:				; CODE XREF: ReCalcCrdObj_coriolis+62p
		ld	a, 0F0h	; 'Ё'
		ld	(ix+22h), a
		ret
; END OF FUNCTION CHUNK	FOR CalcCrdCoriolis2

; =============== S U B	R O U T	I N E =======================================


PrintStars:				; CODE XREF: sub_728D+264p RAM:753Ep ...
		ld	a, (StarsFastScroll)
		dec	a
		jr	nz, sub_78B1
		call	sub_F9A8
		ld	a, 1Eh
		ld	(HighDeltaSpeed), a
		call	sub_78B1
		ld	a, 10h
		ld	(HighDeltaSpeed), a
		call	sub_78B1
; End of function PrintStars


; =============== S U B	R O U T	I N E =======================================




; === CalcCrdSunPlanet_2 === CODE XREF: CalcCrdSunPlanet+Bj
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




; === CalcLenVector === CODE XREF: MoveObject+Cp
CalcLenVector:				; CODE XREF: MoveObject+Cp
		push	ix
		call	Sqr_HL_wordAdrIX
		push	hl
		call	Sqr_HL_wordAdrIX
		push	hl
		call	Sqr_HL_wordAdrIX
		pop	de
		add	hl, de
		pop	de
		add	hl, de
		pop	ix
		call	Sqrt_H_HL
; ---------------------------------------------------------------------------
		ex	de, hl
		push	ix
		call	CaclNormalVector
; ---------------------------------------------------------------------------
		call	CaclNormalVector
; ---------------------------------------------------------------------------
		call	CaclNormalVector
; ---------------------------------------------------------------------------
		pop	ix
		ret
; End of function CalcLenVector


; =============== S U B	R O U T	I N E =======================================


Sqr_HL_wordAdrIX:			; CODE XREF: CalcLenVector+2p
					; CalcLenVector+6p ...
		ld	b, (ix+1)
		ld	c, (ix+0)
		res	7, b
		push	ix
		ld	ix, 0
		call	Mult_HL_B_C
		xor	a
		ld	d, a
		add	hl, hl
		rl	d
		ld	e, h
		add	ix, de
		ld	h, l
		ld	l, a
		push	hl
		ld	h, 0BEh	; '╛'
		ld	l, b
		ld	e, (hl)
		inc	h
		ld	d, (hl)
		dec	h
		add	ix, de
		ld	l, c
		ld	e, (hl)
		inc	h
		ld	d, (hl)
		pop	hl
		add	hl, de
		jr	nc, loc_9EC7
		inc	ix

loc_9EC7:				; CODE XREF: Sqr_HL_wordAdrIX+2Bj
		ex	(sp), ix
		pop	hl
		inc	ix
		inc	ix
		ret
; End of function Sqr_HL_wordAdrIX


; =============== S U B	R O U T	I N E =======================================

; Attributes: noreturn

CaclNormalVector:			; CODE XREF: CalcLenVector+19p
					; CalcLenVector+1Cp ...
		ld	h, (ix+1)
		ld	l, (ix+0)
		push	hl
		res	7, h
		call	Div_BC_HLAdivDE_1
; ---------------------------------------------------------------------------
		srl	b
		rr	c
		pop	hl
		bit	7, h
		jr	z, loc_9EE6
		set	7, b

loc_9EE6:				; CODE XREF: CaclNormalVector+13j
		ld	(ix+0),	c
		ld	(ix+1),	b
		inc	ix
		inc	ix
		ret
; End of function CaclNormalVector


; =============== S U B	R O U T	I N E =======================================

; Attributes: noreturn

Sqrt_H_HL:				; CODE XREF: j_Sqrt_H_HLj
					; CalcLenVector+13p
		ld	a, h
		cp	10h
		jr	nc, sub_9F18
		and	a
		jr	z, loc_9F0A
		ld	b, 4

loc_9EFB:				; CODE XREF: Sqrt_H_HL+Bj
		add	hl, hl
		djnz	loc_9EFB
		call	sub_9F18
; ---------------------------------------------------------------------------
		srl	h
		rr	l
		srl	h
		rr	l
		ret
; ---------------------------------------------------------------------------

loc_9F0A:				; CODE XREF: Sqrt_H_HL+6j
		ld	h, l
		ld	l, a
		call	sub_9F18
; ---------------------------------------------------------------------------
		ld	b, 4

loc_9F11:				; CODE XREF: Sqrt_H_HL+24j
		srl	h
		rr	l
		djnz	loc_9F11
		ret
; End of function Sqrt_H_HL


; =============== S U B	R O U T	I N E =======================================

; Attributes: noreturn



; === CalcLenVectorCrd === CODE XREF: PrintCompass+1Dp
CalcLenVectorCrd:			; CODE XREF: PrintCompass+1Dp
					; PrintCompass+7Cp ...

; FUNCTION CHUNK AT 838F SIZE 00000001 BYTES

		call	GetCrdObject
		ld	a, 1

loc_8327:				; CODE XREF: CalcLenVectorCrdH+Bj
					; CalcLenVectorCrd+14j
		ex	de, hl
		add	hl, hl
		ex	de, hl
		jr	c, loc_8338
		sla	c
		rl	b
		jr	c, loc_833C
		add	hl, hl
		jr	c, loc_8342
		add	a, a
		jr	nc, loc_8327

loc_8338:				; CODE XREF: CalcLenVectorCrd+8j
		rr	d
		jr	loc_8348
; ---------------------------------------------------------------------------

loc_833C:				; CODE XREF: CalcLenVectorCrd+Ej
		rr	b
		srl	d
		jr	loc_8348
; ---------------------------------------------------------------------------

loc_8342:				; CODE XREF: CalcLenVectorCrd+11j
		rr	h
		srl	b
		srl	d

loc_8348:				; CODE XREF: CalcLenVectorCrd+18j
					; CalcLenVectorCrd+1Ej
		ld	c, d
		push	af
		ld	a, h
		call	Sqr_DE_A
		ex	de, hl
		ld	a, b
		ld	b, 0
		call	Sqr_DE_A
		add	hl, de
		jr	nc, loc_8359
		inc	b

loc_8359:				; CODE XREF: CalcLenVectorCrd+34j
		ld	a, c
		call	Sqr_DE_A
		add	hl, de
		jr	nc, loc_8361
		inc	b

loc_8361:				; CODE XREF: CalcLenVectorCrd+3Cj
		ld	a, b
		and	a
		jr	z, loc_8377
		srl	b
		rr	h
		rr	l
		srl	b
		rr	h
		rr	l
		pop	af
		srl	a
		jr	z, loc_8386
		push	af

loc_8377:				; CODE XREF: CalcLenVectorCrd+41j
		call	j_Sqrt_H_HL
; ---------------------------------------------------------------------------
		pop	af

loc_837B:				; CODE XREF: CalcLenVectorCrd+61j
		srl	a
		jr	z, locret_838F
		srl	h
		rr	l
		jp	loc_837B
; ---------------------------------------------------------------------------

loc_8386:				; CODE XREF: CalcLenVectorCrd+52j
		call	j_Sqrt_H_HL
; End of function CalcLenVectorCrd

; ---------------------------------------------------------------------------
		add	hl, hl
		jr	nc, locret_838F
		ld	hl, 0
; START	OF FUNCTION CHUNK FOR CalcLenVectorCrd

locret_838F:				; CODE XREF: CalcLenVectorCrd+5Bj
					; RAM:838Aj
		ret
; END OF FUNCTION CHUNK	FOR CalcLenVectorCrd
; ---------------------------------------------------------------------------
; START	OF FUNCTION CHUNK FOR PrintCompass

RenewCompass:				; CODE XREF: PrintCompass+104j
		ld	a, (WhatOnScreen)
		cp	4
		ret	nc
		call	PrnCompass
		call	sub_8442
		ld	e, 4
		ld	a, (CompasPointerType)
		and	a
		jr	z, loc_83A5
		add	hl, de

loc_83A5:				; CODE XREF: PrintCompass+1F9j
		ex	de, hl
		ld	hl, 5135h
		ld	a, (CompasPointerCrdY)
		ld	b, a
		rlca
		jr	c, loc_83B5
		ld	a, 8
		sub	b
		jr	loc_83B8
; ---------------------------------------------------------------------------

loc_83B5:				; CODE XREF: PrintCompass+205j
		ld	a, 88h ; 'И'
		add	a, b

loc_83B8:				; CODE XREF: PrintCompass+20Aj
		ld	b, a

loc_83B9:				; CODE XREF: PrintCompass+217j
		inc	h
		ld	a, h
		and	7
		call	z, compDownScrAdr_HL
		djnz	loc_83B9
		ld	b, 4

loc_83C4:				; CODE XREF: PrintCompass+252j
		push	bc
		ld	a, (de)
		inc	de
		push	de
		ld	c, a
		ld	de, 0
		ld	a, (CompasPointerCrdX)
		ld	b, a
		res	7, b
		add	a, a
		jr	z, loc_83E5
		jr	c, loc_83DF

loc_83D7:				; CODE XREF: PrintCompass+232j
		srl	c
		rr	e
		djnz	loc_83D7
		jr	loc_83E5
; ---------------------------------------------------------------------------

loc_83DF:				; CODE XREF: PrintCompass+22Cj
					; PrintCompass+23Aj
		sla	c
		rl	d
		djnz	loc_83DF

loc_83E5:				; CODE XREF: PrintCompass+22Aj
					; PrintCompass+234j
		ld	a, d
		or	(hl)
		ld	(hl), a
		inc	hl
		ld	a, c
		or	(hl)
		ld	(hl), a
		inc	hl
		ld	a, e
		or	(hl)
		ld	(hl), a
		dec	hl
		dec	hl
		inc	h
		ld	a, h
		and	7
		call	z, compDownScrAdr_HL
		pop	de
		pop	bc
		djnz	loc_83C4
		ret
; END OF FUNCTION CHUNK	FOR PrintCompass

; =============== S U B	R O U T	I N E =======================================

; 83fe	 PlanetCrd
; 83fe+2 SunCrd



; === CalcLenVectorCrdH === CODE XREF: PrintCompass+D7p
CalcLenVectorCrdH:			; CODE XREF: PrintCompass+D7p
		call	GetCrdObject
		res	7, h
		res	7, d
		res	7, b
		ld	a, 1
		jr	loc_8327
; End of function CalcLenVectorCrdH


; =============== S U B	R O U T	I N E =======================================

; Attributes: noreturn

CalcLenVectorCrd:			; CODE XREF: PrintCompass+1Dp
					; PrintCompass+7Cp ...

; FUNCTION CHUNK AT 838F SIZE 00000001 BYTES

		call	GetCrdObject
		ld	a, 1

loc_8327:				; CODE XREF: CalcLenVectorCrdH+Bj
					; CalcLenVectorCrd+14j
		ex	de, hl
		add	hl, hl
		ex	de, hl
		jr	c, loc_8338
		sla	c
		rl	b
		jr	c, loc_833C
		add	hl, hl
		jr	c, loc_8342
		add	a, a
		jr	nc, loc_8327

loc_8338:				; CODE XREF: CalcLenVectorCrd+8j
		rr	d
		jr	loc_8348
; ---------------------------------------------------------------------------

loc_833C:				; CODE XREF: CalcLenVectorCrd+Ej
		rr	b
		srl	d
		jr	loc_8348
; ---------------------------------------------------------------------------

loc_8342:				; CODE XREF: CalcLenVectorCrd+11j
		rr	h
		srl	b
		srl	d

loc_8348:				; CODE XREF: CalcLenVectorCrd+18j
					; CalcLenVectorCrd+1Ej
		ld	c, d
		push	af
		ld	a, h
		call	Sqr_DE_A
		ex	de, hl
		ld	a, b
		ld	b, 0
		call	Sqr_DE_A
		add	hl, de
		jr	nc, loc_8359
		inc	b

loc_8359:				; CODE XREF: CalcLenVectorCrd+34j
		ld	a, c
		call	Sqr_DE_A
		add	hl, de
		jr	nc, loc_8361
		inc	b

loc_8361:				; CODE XREF: CalcLenVectorCrd+3Cj
		ld	a, b
		and	a
		jr	z, loc_8377
		srl	b
		rr	h
		rr	l
		srl	b
		rr	h
		rr	l
		pop	af
		srl	a
		jr	z, loc_8386
		push	af

loc_8377:				; CODE XREF: CalcLenVectorCrd+41j
		call	j_Sqrt_H_HL
; ---------------------------------------------------------------------------
		pop	af

loc_837B:				; CODE XREF: CalcLenVectorCrd+61j
		srl	a
		jr	z, locret_838F
		srl	h
		rr	l
		jp	loc_837B
; ---------------------------------------------------------------------------

loc_8386:				; CODE XREF: CalcLenVectorCrd+52j
		call	j_Sqrt_H_HL
; End of function CalcLenVectorCrd

; ---------------------------------------------------------------------------
		add	hl, hl
		jr	nc, locret_838F
		ld	hl, 0
; START	OF FUNCTION CHUNK FOR CalcLenVectorCrd

locret_838F:				; CODE XREF: CalcLenVectorCrd+5Bj
					; RAM:838Aj
		ret
; END OF FUNCTION CHUNK	FOR CalcLenVectorCrd
; ---------------------------------------------------------------------------
; START	OF FUNCTION CHUNK FOR PrintCompass

RenewCompass:				; CODE XREF: PrintCompass+104j
		ld	a, (WhatOnScreen)
		cp	4
		ret	nc
		call	PrnCompass
		call	sub_8442
		ld	e, 4
		ld	a, (CompasPointerType)
		and	a
		jr	z, loc_83A5
		add	hl, de

loc_83A5:				; CODE XREF: PrintCompass+1F9j
		ex	de, hl
		ld	hl, 5135h
		ld	a, (CompasPointerCrdY)
		ld	b, a
		rlca
		jr	c, loc_83B5
		ld	a, 8
		sub	b
		jr	loc_83B8
; ---------------------------------------------------------------------------

loc_83B5:				; CODE XREF: PrintCompass+205j
		ld	a, 88h ; 'И'
		add	a, b

loc_83B8:				; CODE XREF: PrintCompass+20Aj
		ld	b, a

loc_83B9:				; CODE XREF: PrintCompass+217j
		inc	h
		ld	a, h
		and	7
		call	z, compDownScrAdr_HL
		djnz	loc_83B9
		ld	b, 4

loc_83C4:				; CODE XREF: PrintCompass+252j
		push	bc
		ld	a, (de)
		inc	de
		push	de
		ld	c, a
		ld	de, 0
		ld	a, (CompasPointerCrdX)
		ld	b, a
		res	7, b
		add	a, a
		jr	z, loc_83E5
		jr	c, loc_83DF

loc_83D7:				; CODE XREF: PrintCompass+232j
		srl	c
		rr	e
		djnz	loc_83D7
		jr	loc_83E5
; ---------------------------------------------------------------------------

loc_83DF:				; CODE XREF: PrintCompass+22Cj
					; PrintCompass+23Aj
		sla	c
		rl	d
		djnz	loc_83DF

loc_83E5:				; CODE XREF: PrintCompass+22Aj
					; PrintCompass+234j
		ld	a, d
		or	(hl)
		ld	(hl), a
		inc	hl
		ld	a, c
		or	(hl)
		ld	(hl), a
		inc	hl
		ld	a, e
		or	(hl)
		ld	(hl), a
		dec	hl
		dec	hl
		inc	h
		ld	a, h
		and	7
		call	z, compDownScrAdr_HL
		pop	de
		pop	bc
		djnz	loc_83C4
		ret
; END OF FUNCTION CHUNK	FOR PrintCompass

; =============== S U B	R O U T	I N E =======================================

; 83fe	 PlanetCrd
; 83fe+2 SunCrd



; === CalcNewCrdNobj === CODE XREF: CalcNewCrdObj_Pl+Cp
CalcNewCrdNobj:				; CODE XREF: CalcNewCrdObj_Pl+Cp
					; CalcNewCrdObj_Pl+1Cp	...
		push	ix
		push	ix
		ex	(sp), iy
		add	ix, de
		ex	de, hl
		add	iy, de
		ld	a, (ix+2)
		ld	d, (ix+1)
		ld	e, (ix+0)
		ld	b, a
		xor	c
		and	80h ; 'А'
		push	af
		res	7, c
		res	7, b
		push	de
		call	Mult_HL_B_C
		pop	de
		push	hl
		ld	b, d
		push	de
		call	Mult_HL_B_C
		pop	de
		ld	b, e
		ld	a, l
		ld	l, h
		ld	h, 0
		pop	de
		add	hl, de
		ld	e, a
		push	hl
		call	Mult_A_hiB_C
		add	a, e
		pop	hl
		jr	nc, loc_A224
		inc	hl

loc_A224:				; CODE XREF: CalcNewCrdNobj+37j
		ld	b, a
		ld	a, h
		ld	h, l
		ld	l, b
		pop	bc
		ld	c, b
		ld	b, (iy+2)
		ld	d, (iy+1)
		ld	e, (iy+0)
		push	af
		ld	a, b
		xor	c
		rlc	c
		rlc	c
		rla
		rr	c
		rrc	c
		srl	a
		ld	b, a
		pop	af
		bit	6, c
		jr	nz, loc_A252
		add	a, a
		sla	c
		rra
		add	a, b
		add	hl, de
		jr	nc, loc_A273
		inc	a
		jr	loc_A273
; ---------------------------------------------------------------------------

loc_A252:				; CODE XREF: CalcNewCrdNobj+5Bj
		and	a
		ex	de, hl
		sbc	hl, de
		jr	nc, loc_A259
		inc	a

loc_A259:				; CODE XREF: CalcNewCrdNobj+6Cj
		ld	d, a
		ld	a, b
		sub	d
		ld	b, a
		jr	nc, loc_A26C
		cpl
		ld	b, a
		ld	a, c
		xor	80h ; 'А'
		ld	c, a
		ex	de, hl
		and	a
		ld	hl, 0
		sbc	hl, de

loc_A26C:				; CODE XREF: CalcNewCrdNobj+73j
		ld	a, b
		rla
		sla	c
		rra
		xor	80h ; 'А'

loc_A273:				; CODE XREF: CalcNewCrdNobj+63j
					; CalcNewCrdNobj+66j
		ld	(iy+2),	a
		ld	(iy+1),	h
		ld	(iy+0),	l
		pop	iy
		pop	ix
		ret
; End of function CalcNewCrdNobj


; =============== S U B	R O U T	I N E =======================================


Mult_Ax2_sign:				; CODE XREF: CalcNewCrdObj_Pl+14p
					; CalcNewCrdObj_Pl+22p
		ld	c, a
		rl	c
		rra
		rl	c
		rla
		rr	c
		ret
; End of function Mult_Ax2_sign

; ---------------------------------------------------------------------------
		db    0
		db    0

; =============== S U B	R O U T	I N E =======================================




; === CalcNewCrdObjSpeed === CODE XREF: DemoRotateObj+40p
CalcNewCrdObjSpeed:			; CODE XREF: DemoRotateObj+40p
					; DemoRotateObj+43p ...
		push	bc
		ld	d, (ix+0Ah)
		ld	e, (ix+9)
		call	sub_A076
		ld	a, (iy+2)
		ld	h, (iy+1)
		ld	l, (iy+0)
		call	sub_A098
		ld	(iy+2),	b
		ld	(iy+1),	h
		ld	(iy+0),	l
		inc	iy
		inc	iy
		inc	iy
		inc	ix
		inc	ix
		pop	bc
		ret
; End of function CalcNewCrdObjSpeed


; =============== S U B	R O U T	I N E =======================================




; === CalcNewCrdObj_Pl === CODE XREF: j_CalcNewCrdObj_Plj
CalcNewCrdObj_Pl:			; CODE XREF: j_CalcNewCrdObj_Plj
					; ReCalcCrdObjp
		call	sub_7F81
		xor	80h ; 'А'
		ld	c, a
		ld	de, 3
		ld	hl, 6
		call	CalcNewCrdNobj
		ld	a, (HighAngleLR)
		xor	80h ; 'А'
		call	Mult_Ax2_sign
		ld	de, 3
		ld	h, d
		ld	l, d
		call	CalcNewCrdNobj
		ld	a, (HighAngleLR)
		call	Mult_Ax2_sign
		ld	hl, 3
		ld	d, h
		ld	e, h
		call	CalcNewCrdNobj
		ld	a, (HighAngleDU)
		ld	c, a
		ld	hl, 3
		ld	de, 6
		call	CalcNewCrdNobj

loc_A1AD:				; DATA XREF: CalcCrdSunPlanet_2+7o
		ld	a, (HighDeltaSpeed)
		inc	a
		add	a, a
		ld	b, a
		ld	h, (ix+8)
		ld	l, (ix+7)
		ld	a, (ix+6)
		bit	7, h
		jr	z, loc_A1D0
		add	a, b
		ld	(ix+6),	a
		jr	nc, locret_A1E9
		inc	hl
		bit	7, h
		jr	nz, loc_A1E3
		jp	sub_5FD5
; ---------------------------------------------------------------------------
		db  18h
		db  0Eh
; ---------------------------------------------------------------------------

loc_A1D0:				; CODE XREF: CalcNewCrdObj_Pl+4Bj
		sub	b
		ld	(ix+6),	a
		jr	nc, locret_A1E9
		dec	hl
		bit	7, h
		jr	z, loc_A1E3
		ld	hl, 8000h
		neg
		ld	(ix+6),	a

loc_A1E3:				; CODE XREF: CalcNewCrdObj_Pl+56j
					; CalcNewCrdObj_Pl+66j
		ld	(ix+8),	h
		ld	(ix+7),	l

locret_A1E9:				; CODE XREF: CalcNewCrdObj_Pl+51j
					; CalcNewCrdObj_Pl+61j
		ret
; End of function CalcNewCrdObj_Pl


; =============== S U B	R O U T	I N E =======================================


CalcNewCrdNobj:				; CODE XREF: CalcNewCrdObj_Pl+Cp
					; CalcNewCrdObj_Pl+1Cp	...
		push	ix
		push	ix
		ex	(sp), iy
		add	ix, de
		ex	de, hl
		add	iy, de
		ld	a, (ix+2)
		ld	d, (ix+1)
		ld	e, (ix+0)
		ld	b, a
		xor	c
		and	80h ; 'А'
		push	af
		res	7, c
		res	7, b
		push	de
		call	Mult_HL_B_C
		pop	de
		push	hl
		ld	b, d
		push	de
		call	Mult_HL_B_C
		pop	de
		ld	b, e
		ld	a, l
		ld	l, h
		ld	h, 0
		pop	de
		add	hl, de
		ld	e, a
		push	hl
		call	Mult_A_hiB_C
		add	a, e
		pop	hl
		jr	nc, loc_A224
		inc	hl

loc_A224:				; CODE XREF: CalcNewCrdNobj+37j
		ld	b, a
		ld	a, h
		ld	h, l
		ld	l, b
		pop	bc
		ld	c, b
		ld	b, (iy+2)
		ld	d, (iy+1)
		ld	e, (iy+0)
		push	af
		ld	a, b
		xor	c
		rlc	c
		rlc	c
		rla
		rr	c
		rrc	c
		srl	a
		ld	b, a
		pop	af
		bit	6, c
		jr	nz, loc_A252
		add	a, a
		sla	c
		rra
		add	a, b
		add	hl, de
		jr	nc, loc_A273
		inc	a
		jr	loc_A273
; ---------------------------------------------------------------------------

loc_A252:				; CODE XREF: CalcNewCrdNobj+5Bj
		and	a
		ex	de, hl
		sbc	hl, de
		jr	nc, loc_A259
		inc	a

loc_A259:				; CODE XREF: CalcNewCrdNobj+6Cj
		ld	d, a
		ld	a, b
		sub	d
		ld	b, a
		jr	nc, loc_A26C
		cpl
		ld	b, a
		ld	a, c
		xor	80h ; 'А'
		ld	c, a
		ex	de, hl
		and	a
		ld	hl, 0
		sbc	hl, de

loc_A26C:				; CODE XREF: CalcNewCrdNobj+73j
		ld	a, b
		rla
		sla	c
		rra
		xor	80h ; 'А'

loc_A273:				; CODE XREF: CalcNewCrdNobj+63j
					; CalcNewCrdNobj+66j
		ld	(iy+2),	a
		ld	(iy+1),	h
		ld	(iy+0),	l
		pop	iy
		pop	ix
		ret
; End of function CalcNewCrdNobj


; =============== S U B	R O U T	I N E =======================================


Mult_Ax2_sign:				; CODE XREF: CalcNewCrdObj_Pl+14p
					; CalcNewCrdObj_Pl+22p
		ld	c, a
		rl	c
		rra
		rl	c
		rla
		rr	c
		ret
; End of function Mult_Ax2_sign

; ---------------------------------------------------------------------------
		db    0
		db    0

; =============== S U B	R O U T	I N E =======================================




; === CalcPrnCrdSun === CODE XREF: PrnSunPlanet+Fp
CalcPrnCrdSun:				; CODE XREF: PrnSunPlanet+Fp
					; PrnSunPlanet+34p
		call	loc_75E1
		and	a
		ret	z
		push	ix
		ld	ix, CircleCrdZlow
		ld	a, (WhatOnScreen)
		cp	2
		jr	c, loc_D7CE
		ld	xl, 7Ah	; 'z'

loc_D7CE:				; CODE XREF: CalcPrnCrdSun+10j
		push	iy
		ld	iy,  loc_7575+2
		call	sub_FBE0
		call	nz, sub_FBE0
		call	nz, sub_FBE0
		pop	iy
		pop	ix
		ret
; End of function CalcPrnCrdSun

; ---------------------------------------------------------------------------
; START	OF FUNCTION CHUNK FOR PrintMessage1_A

loc_D7E2:				; CODE XREF: PrintMessage1_A+2j
					; sub_D470+Bj
		ld	hl, 2EF2h
		nop
		dec	l
		ld	h, 0D7h	; '╫'
		ld	d, 0
		ld	e, a
		add	hl, de
		ld	l, (hl)
		ld	h, 0D8h	; '╪'
		jp	(hl)
; END OF FUNCTION CHUNK	FOR PrintMessage1_A
; ---------------------------------------------------------------------------
		db 84h
		db 4Bh
		db  6Ah	; j
		db  72h	; r
		db 2Eh
		db  37h	; 7
		db 0AEh	; о
		db  22h	; "
		db  33h	; 3
		db 0B4h	; ┤
		db 0E5h	; х
		db  57h	; W
		db  84h	; Д
		db  84h	; Д
		db  84h	; Д
		db 0AEh	; о
		db  33h	; 3
		db  72h	; r
		db  2Eh	; .
		db 0E5h	; х
		db 0E9h	; щ
		db 0B7h	; ╖
		db  7Bh	; {
		db 0BAh	; ║
		db 0BDh	; ╜
		db 0C0h	; └
		db 0B4h	; ┤
		db 0A9h	; й
		db  84h	; Д
		db  84h	; Д
		db  57h	; W
		db  84h	; Д
		db  85h	; Е
		db  7Fh	; 
		db  84h	; Д
		db 0C3h	; ├
		db 0C6h	; ╞
		db 0F3h	; є
		db  1Fh
		db  1Fh
		db 0C9h	; ╔
		db  84h	; Д
		db  84h	; Д
		db 0EFh	; я
		db  84h	; Д
		db  84h	; Д
		db 0C3h	; ├
		db  5Eh	; ^
		db 0E3h	; у

; =============== S U B	R O U T	I N E =======================================




; === CalcPrnCrdVertex === CODE XREF: MakeTablPrnCrdVertex:loc_AA03j
CalcPrnCrdVertex:			; CODE XREF: MakeTablPrnCrdVertex:loc_AA03j
		push	iy		; расчёт печатных координат вершины
					; вх:  ix - адрес вершины
					;      (sp-2) -	адрес слота объекта
		ld	a, (DistanceToObj)
		cp	4
		jp	nc, RotateCrdVertex
		ld	a, (ix+0)
		or	(ix+1)
		or	(ix+2)
		ld	b, 0FFh
		scf

loc_ACB8:				; CODE XREF: MakeTablPrnCrdVertex+2F1j
		rla
		inc	b
		jr	nc, loc_ACB8
		ld	a, b
		ld	(VertexIncFactor), a
		and	a
		jp	z, RotateCrdVertex
		call	RotateCrdN_Vertex
		ld	(VertexCrdXlow), a
		ld	(VertexCrdX), hl
		call	RotateCrdN_Vertex
		ld	(VertexCrdYlow), a
		ld	(VertexCrdY), hl
		call	RotateCrdN_Vertex
		ld	(VertexCrdZlow), a
		ld	(VertexCrdZ), hl

loc_ACDF:				; CODE XREF: MakeTablPrnCrdVertex+1E3j
		xor	a
		ld	(VisiblePrevVertex), a
		pop	iy
		pop	hl
		ex	(sp), ix
		push	hl
		ld	l, (ix+0)
		ld	h, (ix+1)
		ld	b, (ix+2)
		ld	de, (VertexCrdX)
		ld	a, (VertexCrdXlow)
		rla
		push	af
		call	AddCrdNobj_CrdNvertex
		ld	(VertexCrdX), hl
		ld	(VertexSignCrdX), a
		pop	af
		ld	(VertexCrdXlow), a
		ld	l, (ix+3)
		ld	h, (ix+4)
		ld	b, (ix+5)
		ld	de, (VertexCrdY)
		ld	a, (VertexCrdYlow)
		rla
		push	af
		call	AddCrdNobj_CrdNvertex
		ld	(VertexCrdY), hl
		ld	(VertexSignCrdY), a
		pop	af
		ld	(VertexCrdYlow), a
		ld	l, (ix+6)
		ld	h, (ix+7)
		ld	b, (ix+8)
		ld	de, (VertexCrdZ)
		ld	a, (VertexCrdZlow)
		rla
		push	af
		call	AddCrdNobj_CrdNvertex
		ld	(VertexCrdZ), hl
		ld	(VertexSignCrdZ), a
		pop	af
		ld	(VertexCrdZlow), a
		ld	a, (WhatOnScreen)
		bit	1, a
		jr	nz, loc_AD82
		ld	de, (VertexCrdX)
		ld	a, (VertexCrdXlow)
		ld	c, a
		ld	a, (VertexCrdZlow)
		call	Div_C_DEc_HLa
; END OF FUNCTION CHUNK	FOR MakeTablPrnCrdVertex
; ---------------------------------------------------------------------------
		ld	a, c
		ld	(CrdXProjection), a
		ld	de, (VertexCrdY)
		ld	hl, (VertexCrdZ)
		ld	a, (VertexCrdYlow)
		ld	c, a
		ld	a, (VertexCrdZlow)
		call	Div_C_DEc_HLa
; ---------------------------------------------------------------------------
		ld	a, c
		ld	(CrdYProjection), a
		ld	a, (VertexSignCrdX)
		rla
		rl	c
		ld	a, (VertexSignCrdZ)
		rla
		rl	b
		jr	loc_ADB5
; ---------------------------------------------------------------------------
; START	OF FUNCTION CHUNK FOR MakeTablPrnCrdVertex

loc_AD82:				; CODE XREF: MakeTablPrnCrdVertex+382j
		ex	de, hl
		ld	hl, (VertexCrdX)
		ld	a, (VertexCrdZlow)
		ld	c, a
		ld	a, (VertexCrdXlow)
		call	Div_C_DEc_HLa
; END OF FUNCTION CHUNK	FOR MakeTablPrnCrdVertex
; ---------------------------------------------------------------------------
		ld	a, c
		ld	(CrdXProjection), a
		ld	de, (VertexCrdY)
		ld	hl, (VertexCrdX)
		ld	a, (VertexCrdYlow)
		ld	c, a
		ld	a, (VertexCrdXlow)
		call	Div_C_DEc_HLa
; ---------------------------------------------------------------------------
		ld	a, c
		ld	(CrdYProjection), a
		ld	a, (VertexSignCrdX)
		rla
		rl	b
		ld	a, (VertexSignCrdZ)
		rla
		rl	c

loc_ADB5:				; CODE XREF: RAM:AD80j
		ld	a, (VertexSignCrdY)
		rla
		rl	c
		ld	a, (WhatOnScreen)
		bit	0, a
		ld	a, c
		jr	z, loc_ADC5
		xor	2

loc_ADC5:				; CODE XREF: RAM:ADC1j
		and	3
		ld	(CrdXYsign), a
		ld	a, (PointTablCrdPrn)
		ld	hl, TablCrdPrnVertex
		add	a, l
		jr	nc, loc_ADD4
		inc	h

loc_ADD4:				; CODE XREF: RAM:ADD1j
		ld	l, a
		ld	a, (CrdXProjection)
		ld	(hl), a
		inc	hl
		ld	a, (CrdYProjection)
		ld	(hl), a
		inc	hl
		ld	a, (WhatOnScreen)
		ld	c, a
		rra
		xor	c
		xor	b
		rra
		ld	a, (CrdXYsign)
		jr	nc, loc_ADEE
		ld	a, 0FFh

loc_ADEE:				; CODE XREF: RAM:ADEAj
		ld	(hl), a
		pop	af
		ex	(sp), ix
; START	OF FUNCTION CHUNK FOR MakeTablPrnCrdVertex

loc_ADF2:				; CODE XREF: MakeTablPrnCrdVertex+42j
		call	sub_AE1D
		add	ix, bc
		inc	de
		dec	a
		jp	nz, loc_A9DE
		pop	ix
		pop	iy
		ret
; END OF FUNCTION CHUNK	FOR MakeTablPrnCrdVertex

; =============== S U B	R O U T	I N E =======================================




; === CompasCrdNnear === CODE XREF: PrintCompass+B6p
CompasCrdNnear:				; CODE XREF: PrintCompass+B6p
					; PrintCompass+C3p ...
		xor	a
		ld	l, (ix+0)
		ld	h, (ix+1)
		jr	loc_82BB
; End of function CompasCrdNnear

; ---------------------------------------------------------------------------
; START	OF FUNCTION CHUNK FOR PrintCompass

loc_827A:				; CODE XREF: PrintCompass+5Aj
					; PrintCompass+69j
		nop
		call	sub_FD02

loc_827E:				; CODE XREF: PrintCompass+AAj
		inc	ix
		call	CalcLenVectorCrdH
; ---------------------------------------------------------------------------
		ld	a, h
		or	l
		jr	nz, loc_8288
		dec	hl

loc_8288:				; CODE XREF: PrintCompass+DCj
		ex	de, hl
		dec	ix
		push	de
		call	CompasCrdXplanet
; ---------------------------------------------------------------------------
		ld	(CompasPointerCrdX), a
		inc	ix
		inc	ix
		inc	ix
		pop	de
		call	CompasCrdXplanet
; ---------------------------------------------------------------------------

loc_829C:				; CODE XREF: PrintCompass+C6j
		ld	(CompasPointerCrdY), a
		ld	a, (ix+5)
		and	80h ; 'А'
		ld	a, 0
		jr	z, loc_82AA
		ld	a, 1

loc_82AA:				; CODE XREF: PrintCompass+FDj
		ld	(CompasPointerType), a
		jp	RenewCompass
; END OF FUNCTION CHUNK	FOR PrintCompass

; =============== S U B	R O U T	I N E =======================================

; Attributes: noreturn

CompasCrdXplanet:			; CODE XREF: PrintCompass+E3p
					; PrintCompass+F0p
		ld	a, (ix+0)
		ld	l, (ix+1)
		ld	h, (ix+2)
		res	7, h

loc_82BB:				; CODE XREF: CompasCrdNnear+7j
		call	Div_BC_HLAdivDE
; ---------------------------------------------------------------------------
		rl	b
		rla
		rl	b
		rla
		rl	b
		rla
		ld	h, (ix+2)
		bit	7, h
		jr	z, locret_82D0
		or	80h ; 'А'

locret_82D0:				; CODE XREF: CompasCrdXplanet+1Cj
		ret
; End of function CompasCrdXplanet

; ---------------------------------------------------------------------------
; START	OF FUNCTION CHUNK FOR PrintCompass

loc_82D1:				; CODE XREF: PrintCompass+36j
					; PrintCompass+90j
		ld	a, 1
		ld	(PlayerGameOver), a
		ret
; END OF FUNCTION CHUNK	FOR PrintCompass

; =============== S U B	R O U T	I N E =======================================


CalcTempAltitude:			; CODE XREF: PrintCompass+47p
					; PrintCompass+94p
		push	hl
		add	hl, hl
		pop	de
		srl	d
		rr	e
		add	hl, de
		ld	a, (IndDanger)
		ld	b, a
		ld	a, h
		cp	50h ; 'P'
		jr	c, loc_82F8
		cp	78h ; 'x'
		jr	nc, loc_82F4
		ld	a, b
		cp	63h ; 'c'
		ret	z
		ld	a, 2
		jr	loc_82FE
; ---------------------------------------------------------------------------

loc_82F4:				; CODE XREF: CalcTempAltitude+13j
		ld	a, 63h ; 'c'
		jr	loc_82FE
; ---------------------------------------------------------------------------

loc_82F8:				; CODE XREF: CalcTempAltitude+Fj
		ld	a, b
		cp	4
		ret	nz
		ld	a, 6

loc_82FE:				; CODE XREF: CalcTempAltitude+1Bj
					; CalcTempAltitude+1Fj
		ld	(IndDanger), a
		ret
; End of function CalcTempAltitude


; =============== S U B	R O U T	I N E =======================================


GetCrdObject:				; CODE XREF: CalcLenVectorCrdHp
					; CalcLenVectorCrdp
		ld	e, (ix+0)
		ld	d, (ix+1)
		ld	c, (ix+3)
		ld	b, (ix+4)
		ld	l, (ix+6)
		ld	h, (ix+7)
		ret
; End of function GetCrdObject


; =============== S U B	R O U T	I N E =======================================

; Attributes: noreturn

CalcLenVectorCrdH:			; CODE XREF: PrintCompass+D7p
		call	GetCrdObject
		res	7, h
		res	7, d
		res	7, b
		ld	a, 1
		jr	loc_8327
; End of function CalcLenVectorCrdH


; =============== S U B	R O U T	I N E =======================================

; Attributes: noreturn

CalcLenVectorCrd:			; CODE XREF: PrintCompass+1Dp
					; PrintCompass+7Cp ...

; FUNCTION CHUNK AT 838F SIZE 00000001 BYTES

		call	GetCrdObject
		ld	a, 1

loc_8327:				; CODE XREF: CalcLenVectorCrdH+Bj
					; CalcLenVectorCrd+14j
		ex	de, hl
		add	hl, hl
		ex	de, hl
		jr	c, loc_8338
		sla	c
		rl	b
		jr	c, loc_833C
		add	hl, hl
		jr	c, loc_8342
		add	a, a
		jr	nc, loc_8327

loc_8338:				; CODE XREF: CalcLenVectorCrd+8j
		rr	d
		jr	loc_8348
; ---------------------------------------------------------------------------

loc_833C:				; CODE XREF: CalcLenVectorCrd+Ej
		rr	b
		srl	d
		jr	loc_8348
; ---------------------------------------------------------------------------

loc_8342:				; CODE XREF: CalcLenVectorCrd+11j
		rr	h
		srl	b
		srl	d

loc_8348:				; CODE XREF: CalcLenVectorCrd+18j
					; CalcLenVectorCrd+1Ej
		ld	c, d
		push	af
		ld	a, h
		call	Sqr_DE_A
		ex	de, hl
		ld	a, b
		ld	b, 0
		call	Sqr_DE_A
		add	hl, de
		jr	nc, loc_8359
		inc	b

loc_8359:				; CODE XREF: CalcLenVectorCrd+34j
		ld	a, c
		call	Sqr_DE_A
		add	hl, de
		jr	nc, loc_8361
		inc	b

loc_8361:				; CODE XREF: CalcLenVectorCrd+3Cj
		ld	a, b
		and	a
		jr	z, loc_8377
		srl	b
		rr	h
		rr	l
		srl	b
		rr	h
		rr	l
		pop	af
		srl	a
		jr	z, loc_8386
		push	af

loc_8377:				; CODE XREF: CalcLenVectorCrd+41j
		call	j_Sqrt_H_HL
; ---------------------------------------------------------------------------
		pop	af

loc_837B:				; CODE XREF: CalcLenVectorCrd+61j
		srl	a
		jr	z, locret_838F
		srl	h
		rr	l
		jp	loc_837B
; ---------------------------------------------------------------------------

loc_8386:				; CODE XREF: CalcLenVectorCrd+52j
		call	j_Sqrt_H_HL
; End of function CalcLenVectorCrd

; ---------------------------------------------------------------------------
		add	hl, hl
		jr	nc, locret_838F
		ld	hl, 0
; START	OF FUNCTION CHUNK FOR CalcLenVectorCrd

locret_838F:				; CODE XREF: CalcLenVectorCrd+5Bj
					; RAM:838Aj
		ret
; END OF FUNCTION CHUNK	FOR CalcLenVectorCrd
; ---------------------------------------------------------------------------
; START	OF FUNCTION CHUNK FOR PrintCompass

RenewCompass:				; CODE XREF: PrintCompass+104j
		ld	a, (WhatOnScreen)
		cp	4
		ret	nc
		call	PrnCompass
		call	sub_8442
		ld	e, 4
		ld	a, (CompasPointerType)
		and	a
		jr	z, loc_83A5
		add	hl, de

loc_83A5:				; CODE XREF: PrintCompass+1F9j
		ex	de, hl
		ld	hl, 5135h
		ld	a, (CompasPointerCrdY)
		ld	b, a
		rlca
		jr	c, loc_83B5
		ld	a, 8
		sub	b
		jr	loc_83B8
; ---------------------------------------------------------------------------

loc_83B5:				; CODE XREF: PrintCompass+205j
		ld	a, 88h ; 'И'
		add	a, b

loc_83B8:				; CODE XREF: PrintCompass+20Aj
		ld	b, a

loc_83B9:				; CODE XREF: PrintCompass+217j
		inc	h
		ld	a, h
		and	7
		call	z, compDownScrAdr_HL
		djnz	loc_83B9
		ld	b, 4

loc_83C4:				; CODE XREF: PrintCompass+252j
		push	bc
		ld	a, (de)
		inc	de
		push	de
		ld	c, a
		ld	de, 0
		ld	a, (CompasPointerCrdX)
		ld	b, a
		res	7, b
		add	a, a
		jr	z, loc_83E5
		jr	c, loc_83DF

loc_83D7:				; CODE XREF: PrintCompass+232j
		srl	c
		rr	e
		djnz	loc_83D7
		jr	loc_83E5
; ---------------------------------------------------------------------------

loc_83DF:				; CODE XREF: PrintCompass+22Cj
					; PrintCompass+23Aj
		sla	c
		rl	d
		djnz	loc_83DF

loc_83E5:				; CODE XREF: PrintCompass+22Aj
					; PrintCompass+234j
		ld	a, d
		or	(hl)
		ld	(hl), a
		inc	hl
		ld	a, c
		or	(hl)
		ld	(hl), a
		inc	hl
		ld	a, e
		or	(hl)
		ld	(hl), a
		dec	hl
		dec	hl
		inc	h
		ld	a, h
		and	7
		call	z, compDownScrAdr_HL
		pop	de
		pop	bc
		djnz	loc_83C4
		ret
; END OF FUNCTION CHUNK	FOR PrintCompass

; =============== S U B	R O U T	I N E =======================================

; 83fe	 PlanetCrd
; 83fe+2 SunCrd



; === CompasCrdXplanet === CODE XREF: PrintCompass+E3p
CompasCrdXplanet:			; CODE XREF: PrintCompass+E3p
					; PrintCompass+F0p
		ld	a, (ix+0)
		ld	l, (ix+1)
		ld	h, (ix+2)
		res	7, h

loc_82BB:				; CODE XREF: CompasCrdNnear+7j
		call	Div_BC_HLAdivDE
; ---------------------------------------------------------------------------
		rl	b
		rla
		rl	b
		rla
		rl	b
		rla
		ld	h, (ix+2)
		bit	7, h
		jr	z, locret_82D0
		or	80h ; 'А'

locret_82D0:				; CODE XREF: CompasCrdXplanet+1Cj
		ret
; End of function CompasCrdXplanet

; ---------------------------------------------------------------------------
; START	OF FUNCTION CHUNK FOR PrintCompass

loc_82D1:				; CODE XREF: PrintCompass+36j
					; PrintCompass+90j
		ld	a, 1
		ld	(PlayerGameOver), a
		ret
; END OF FUNCTION CHUNK	FOR PrintCompass

; =============== S U B	R O U T	I N E =======================================


CalcTempAltitude:			; CODE XREF: PrintCompass+47p
					; PrintCompass+94p
		push	hl
		add	hl, hl
		pop	de
		srl	d
		rr	e
		add	hl, de
		ld	a, (IndDanger)
		ld	b, a
		ld	a, h
		cp	50h ; 'P'
		jr	c, loc_82F8
		cp	78h ; 'x'
		jr	nc, loc_82F4
		ld	a, b
		cp	63h ; 'c'
		ret	z
		ld	a, 2
		jr	loc_82FE
; ---------------------------------------------------------------------------

loc_82F4:				; CODE XREF: CalcTempAltitude+13j
		ld	a, 63h ; 'c'
		jr	loc_82FE
; ---------------------------------------------------------------------------

loc_82F8:				; CODE XREF: CalcTempAltitude+Fj
		ld	a, b
		cp	4
		ret	nz
		ld	a, 6

loc_82FE:				; CODE XREF: CalcTempAltitude+1Bj
					; CalcTempAltitude+1Fj
		ld	(IndDanger), a
		ret
; End of function CalcTempAltitude


; =============== S U B	R O U T	I N E =======================================


GetCrdObject:				; CODE XREF: CalcLenVectorCrdHp
					; CalcLenVectorCrdp
		ld	e, (ix+0)
		ld	d, (ix+1)
		ld	c, (ix+3)
		ld	b, (ix+4)
		ld	l, (ix+6)
		ld	h, (ix+7)
		ret
; End of function GetCrdObject


; =============== S U B	R O U T	I N E =======================================

; Attributes: noreturn

CalcLenVectorCrdH:			; CODE XREF: PrintCompass+D7p
		call	GetCrdObject
		res	7, h
		res	7, d
		res	7, b
		ld	a, 1
		jr	loc_8327
; End of function CalcLenVectorCrdH


; =============== S U B	R O U T	I N E =======================================

; Attributes: noreturn

CalcLenVectorCrd:			; CODE XREF: PrintCompass+1Dp
					; PrintCompass+7Cp ...

; FUNCTION CHUNK AT 838F SIZE 00000001 BYTES

		call	GetCrdObject
		ld	a, 1

loc_8327:				; CODE XREF: CalcLenVectorCrdH+Bj
					; CalcLenVectorCrd+14j
		ex	de, hl
		add	hl, hl
		ex	de, hl
		jr	c, loc_8338
		sla	c
		rl	b
		jr	c, loc_833C
		add	hl, hl
		jr	c, loc_8342
		add	a, a
		jr	nc, loc_8327

loc_8338:				; CODE XREF: CalcLenVectorCrd+8j
		rr	d
		jr	loc_8348
; ---------------------------------------------------------------------------

loc_833C:				; CODE XREF: CalcLenVectorCrd+Ej
		rr	b
		srl	d
		jr	loc_8348
; ---------------------------------------------------------------------------

loc_8342:				; CODE XREF: CalcLenVectorCrd+11j
		rr	h
		srl	b
		srl	d

loc_8348:				; CODE XREF: CalcLenVectorCrd+18j
					; CalcLenVectorCrd+1Ej
		ld	c, d
		push	af
		ld	a, h
		call	Sqr_DE_A
		ex	de, hl
		ld	a, b
		ld	b, 0
		call	Sqr_DE_A
		add	hl, de
		jr	nc, loc_8359
		inc	b

loc_8359:				; CODE XREF: CalcLenVectorCrd+34j
		ld	a, c
		call	Sqr_DE_A
		add	hl, de
		jr	nc, loc_8361
		inc	b

loc_8361:				; CODE XREF: CalcLenVectorCrd+3Cj
		ld	a, b
		and	a
		jr	z, loc_8377
		srl	b
		rr	h
		rr	l
		srl	b
		rr	h
		rr	l
		pop	af
		srl	a
		jr	z, loc_8386
		push	af

loc_8377:				; CODE XREF: CalcLenVectorCrd+41j
		call	j_Sqrt_H_HL
; ---------------------------------------------------------------------------
		pop	af

loc_837B:				; CODE XREF: CalcLenVectorCrd+61j
		srl	a
		jr	z, locret_838F
		srl	h
		rr	l
		jp	loc_837B
; ---------------------------------------------------------------------------

loc_8386:				; CODE XREF: CalcLenVectorCrd+52j
		call	j_Sqrt_H_HL
; End of function CalcLenVectorCrd

; ---------------------------------------------------------------------------
		add	hl, hl
		jr	nc, locret_838F
		ld	hl, 0
; START	OF FUNCTION CHUNK FOR CalcLenVectorCrd

locret_838F:				; CODE XREF: CalcLenVectorCrd+5Bj
					; RAM:838Aj
		ret
; END OF FUNCTION CHUNK	FOR CalcLenVectorCrd
; ---------------------------------------------------------------------------
; START	OF FUNCTION CHUNK FOR PrintCompass

RenewCompass:				; CODE XREF: PrintCompass+104j
		ld	a, (WhatOnScreen)
		cp	4
		ret	nc
		call	PrnCompass
		call	sub_8442
		ld	e, 4
		ld	a, (CompasPointerType)
		and	a
		jr	z, loc_83A5
		add	hl, de

loc_83A5:				; CODE XREF: PrintCompass+1F9j
		ex	de, hl
		ld	hl, 5135h
		ld	a, (CompasPointerCrdY)
		ld	b, a
		rlca
		jr	c, loc_83B5
		ld	a, 8
		sub	b
		jr	loc_83B8
; ---------------------------------------------------------------------------

loc_83B5:				; CODE XREF: PrintCompass+205j
		ld	a, 88h ; 'И'
		add	a, b

loc_83B8:				; CODE XREF: PrintCompass+20Aj
		ld	b, a

loc_83B9:				; CODE XREF: PrintCompass+217j
		inc	h
		ld	a, h
		and	7
		call	z, compDownScrAdr_HL
		djnz	loc_83B9
		ld	b, 4

loc_83C4:				; CODE XREF: PrintCompass+252j
		push	bc
		ld	a, (de)
		inc	de
		push	de
		ld	c, a
		ld	de, 0
		ld	a, (CompasPointerCrdX)
		ld	b, a
		res	7, b
		add	a, a
		jr	z, loc_83E5
		jr	c, loc_83DF

loc_83D7:				; CODE XREF: PrintCompass+232j
		srl	c
		rr	e
		djnz	loc_83D7
		jr	loc_83E5
; ---------------------------------------------------------------------------

loc_83DF:				; CODE XREF: PrintCompass+22Cj
					; PrintCompass+23Aj
		sla	c
		rl	d
		djnz	loc_83DF

loc_83E5:				; CODE XREF: PrintCompass+22Aj
					; PrintCompass+234j
		ld	a, d
		or	(hl)
		ld	(hl), a
		inc	hl
		ld	a, c
		or	(hl)
		ld	(hl), a
		inc	hl
		ld	a, e
		or	(hl)
		ld	(hl), a
		dec	hl
		dec	hl
		inc	h
		ld	a, h
		and	7
		call	z, compDownScrAdr_HL
		pop	de
		pop	bc
		djnz	loc_83C4
		ret
; END OF FUNCTION CHUNK	FOR PrintCompass

; =============== S U B	R O U T	I N E =======================================

; 83fe	 PlanetCrd
; 83fe+2 SunCrd



; === CorrAxisVector === CODE XREF: sub_9D40+F6p sub_9D40+F9p ...
CorrAxisVector:				; CODE XREF: sub_9D40+F6p sub_9D40+F9p ...
		ld	h, 0BFh	; '┐'
		ld	l, (ix+1)
		sla	l
		ld	a, (hl)
		ld	l, (ix+3)
		sla	l
		add	a, (hl)
		ld	l, (ix+5)
		sla	l
		add	a, (hl)
		sra	a
		sra	a
		neg
		xor	80h ; 'А'
		ld	b, a
		call	sub_9F68
		call	sub_9F68
		call	sub_9F68
		ret
; End of function CorrAxisVector


; =============== S U B	R O U T	I N E =======================================




; === GetCrdObject === CODE XREF: CalcLenVectorCrdHp
GetCrdObject:				; CODE XREF: CalcLenVectorCrdHp
					; CalcLenVectorCrdp
		ld	e, (ix+0)
		ld	d, (ix+1)
		ld	c, (ix+3)
		ld	b, (ix+4)
		ld	l, (ix+6)
		ld	h, (ix+7)
		ret
; End of function GetCrdObject


; =============== S U B	R O U T	I N E =======================================

; Attributes: noreturn

CalcLenVectorCrdH:			; CODE XREF: PrintCompass+D7p
		call	GetCrdObject
		res	7, h
		res	7, d
		res	7, b
		ld	a, 1
		jr	loc_8327
; End of function CalcLenVectorCrdH


; =============== S U B	R O U T	I N E =======================================

; Attributes: noreturn

CalcLenVectorCrd:			; CODE XREF: PrintCompass+1Dp
					; PrintCompass+7Cp ...

; FUNCTION CHUNK AT 838F SIZE 00000001 BYTES

		call	GetCrdObject
		ld	a, 1

loc_8327:				; CODE XREF: CalcLenVectorCrdH+Bj
					; CalcLenVectorCrd+14j
		ex	de, hl
		add	hl, hl
		ex	de, hl
		jr	c, loc_8338
		sla	c
		rl	b
		jr	c, loc_833C
		add	hl, hl
		jr	c, loc_8342
		add	a, a
		jr	nc, loc_8327

loc_8338:				; CODE XREF: CalcLenVectorCrd+8j
		rr	d
		jr	loc_8348
; ---------------------------------------------------------------------------

loc_833C:				; CODE XREF: CalcLenVectorCrd+Ej
		rr	b
		srl	d
		jr	loc_8348
; ---------------------------------------------------------------------------

loc_8342:				; CODE XREF: CalcLenVectorCrd+11j
		rr	h
		srl	b
		srl	d

loc_8348:				; CODE XREF: CalcLenVectorCrd+18j
					; CalcLenVectorCrd+1Ej
		ld	c, d
		push	af
		ld	a, h
		call	Sqr_DE_A
		ex	de, hl
		ld	a, b
		ld	b, 0
		call	Sqr_DE_A
		add	hl, de
		jr	nc, loc_8359
		inc	b

loc_8359:				; CODE XREF: CalcLenVectorCrd+34j
		ld	a, c
		call	Sqr_DE_A
		add	hl, de
		jr	nc, loc_8361
		inc	b

loc_8361:				; CODE XREF: CalcLenVectorCrd+3Cj
		ld	a, b
		and	a
		jr	z, loc_8377
		srl	b
		rr	h
		rr	l
		srl	b
		rr	h
		rr	l
		pop	af
		srl	a
		jr	z, loc_8386
		push	af

loc_8377:				; CODE XREF: CalcLenVectorCrd+41j
		call	j_Sqrt_H_HL
; ---------------------------------------------------------------------------
		pop	af

loc_837B:				; CODE XREF: CalcLenVectorCrd+61j
		srl	a
		jr	z, locret_838F
		srl	h
		rr	l
		jp	loc_837B
; ---------------------------------------------------------------------------

loc_8386:				; CODE XREF: CalcLenVectorCrd+52j
		call	j_Sqrt_H_HL
; End of function CalcLenVectorCrd

; ---------------------------------------------------------------------------
		add	hl, hl
		jr	nc, locret_838F
		ld	hl, 0
; START	OF FUNCTION CHUNK FOR CalcLenVectorCrd

locret_838F:				; CODE XREF: CalcLenVectorCrd+5Bj
					; RAM:838Aj
		ret
; END OF FUNCTION CHUNK	FOR CalcLenVectorCrd
; ---------------------------------------------------------------------------
; START	OF FUNCTION CHUNK FOR PrintCompass

RenewCompass:				; CODE XREF: PrintCompass+104j
		ld	a, (WhatOnScreen)
		cp	4
		ret	nc
		call	PrnCompass
		call	sub_8442
		ld	e, 4
		ld	a, (CompasPointerType)
		and	a
		jr	z, loc_83A5
		add	hl, de

loc_83A5:				; CODE XREF: PrintCompass+1F9j
		ex	de, hl
		ld	hl, 5135h
		ld	a, (CompasPointerCrdY)
		ld	b, a
		rlca
		jr	c, loc_83B5
		ld	a, 8
		sub	b
		jr	loc_83B8
; ---------------------------------------------------------------------------

loc_83B5:				; CODE XREF: PrintCompass+205j
		ld	a, 88h ; 'И'
		add	a, b

loc_83B8:				; CODE XREF: PrintCompass+20Aj
		ld	b, a

loc_83B9:				; CODE XREF: PrintCompass+217j
		inc	h
		ld	a, h
		and	7
		call	z, compDownScrAdr_HL
		djnz	loc_83B9
		ld	b, 4

loc_83C4:				; CODE XREF: PrintCompass+252j
		push	bc
		ld	a, (de)
		inc	de
		push	de
		ld	c, a
		ld	de, 0
		ld	a, (CompasPointerCrdX)
		ld	b, a
		res	7, b
		add	a, a
		jr	z, loc_83E5
		jr	c, loc_83DF

loc_83D7:				; CODE XREF: PrintCompass+232j
		srl	c
		rr	e
		djnz	loc_83D7
		jr	loc_83E5
; ---------------------------------------------------------------------------

loc_83DF:				; CODE XREF: PrintCompass+22Cj
					; PrintCompass+23Aj
		sla	c
		rl	d
		djnz	loc_83DF

loc_83E5:				; CODE XREF: PrintCompass+22Aj
					; PrintCompass+234j
		ld	a, d
		or	(hl)
		ld	(hl), a
		inc	hl
		ld	a, c
		or	(hl)
		ld	(hl), a
		inc	hl
		ld	a, e
		or	(hl)
		ld	(hl), a
		dec	hl
		dec	hl
		inc	h
		ld	a, h
		and	7
		call	z, compDownScrAdr_HL
		pop	de
		pop	bc
		djnz	loc_83C4
		ret
; END OF FUNCTION CHUNK	FOR PrintCompass

; =============== S U B	R O U T	I N E =======================================

; 83fe	 PlanetCrd
; 83fe+2 SunCrd



; === GetCrdsProjection === CODE XREF: sub_7E02:objDestroyedp
GetCrdsProjection:			; CODE XREF: sub_7E02:objDestroyedp
					; sub_7E02+1Ap	...
		ld	de, (CrdXProjection)
		ld	a, d
		ld	d, e
		ld	e, a
		ld	a, (WhatOnScreen)
		bit	0, a
		ld	a, (CrdXYsign)
		jr	z, loc_F958
		xor	2

loc_F958:				; CODE XREF: GetCrdsProjection+Fj
		ld	c, a
		ret
; End of function GetCrdsProjection


; =============== S U B	R O U T	I N E =======================================

; проверка есть	ли ракета нацеленная на	текущий	объект



; === GetPrnCrdVertex === CODE XREF: PrintObject_2:loc_6FB5p
GetPrnCrdVertex:			; CODE XREF: PrintObject_2:loc_6FB5p
					; PrintObject_2+83p ...
		ld	b, a
		add	a, a
		ld	hl, TablCrdPrnVertex
		jr	nc, loc_AE58
		inc	h

loc_AE58:				; CODE XREF: GetPrnCrdVertex+5j
		add	a, b
		jr	nc, loc_AE5C
		inc	h

loc_AE5C:				; CODE XREF: GetPrnCrdVertex+9j
		add	a, l
		ld	l, a
		jr	nc, loc_AE61
		inc	h

loc_AE61:				; CODE XREF: GetPrnCrdVertex+Ej
		ld	d, (hl)
		inc	hl
		ld	e, (hl)
		inc	hl
		ld	a, (hl)
		bit	7, a
		ret
; End of function GetPrnCrdVertex


; =============== S U B	R O U T	I N E =======================================


CorrAxirY_SKO:				; CODE XREF: sub_9D40+ECp sub_A28D+17p
		push	ix
		push	ix
		ex	(sp), iy
		ld	de, 9
		add	ix, de
		ld	e, 0Fh
		add	iy, de
		call	ScalarMultVectors
		ld	e, a
		call	sub_AE8A
		call	sub_AE8A
		call	sub_AE8A
		pop	iy
		pop	ix
		ret
; End of function CorrAxirY_SKO


; =============== S U B	R O U T	I N E =======================================




; === HL_CrdNdiv4 === CODE XREF: DirectObject+4Fp
HL_CrdNdiv4:				; CODE XREF: DirectObject+4Fp
					; DirectObject+55p ...
		ld	a, (ix+2)

loc_A572:				; CODE XREF: HL_CrdNdiv4_InvSign+5j
		ld	l, (ix+0)
		ld	h, (ix+1)
		srl	h
		rr	l
		rla
		rr	h
		rr	l
		ld	de, 3
		add	ix, de
		ret
; End of function HL_CrdNdiv4


; =============== S U B	R O U T	I N E =======================================

; Attributes: noreturn

MoveObject:				; CODE XREF: DirectObject:MoveObject1j
		push	iy
		ld	iy, deltaCrdX
		push	ix
		ld	ix, deltaCrdX
		call	CalcLenVector
; ---------------------------------------------------------------------------
		pop	ix
		ld	(LenVector), de
		push	ix
		ld	de, 9
		add	ix, de
		call	ScalarMultVectors
		ld	de, 6
		add	ix, de
		ld	h, a
		push	hl
		call	ScalarMultVectors
		pop	hl
		ld	l, a
		push	hl
		ld	de, 6
		add	ix, de
		call	ScalarMultVectors
		pop	hl
		pop	ix
		pop	iy
		ld	d, a
		and	7Fh ; ''
		jr	z, loc_A5E7
		ld	c, a
		srl	c
		srl	c
		srl	c
		adc	a, c
		srl	a
		srl	a
		cp	0Fh
		jr	c, loc_A5D7
		ld	a, 0Fh

loc_A5D7:				; CODE XREF: MoveObject+4Cj
		ld	b, a
		ld	a, d
		xor	l
		and	80h ; 'А'
		rra
		or	b
		ld	(ix+1Dh), a
		and	3Fh ; '?'
		cp	4
		jr	nc, loc_A605

loc_A5E7:				; CODE XREF: MoveObject+3Cj
		ld	a, h
		bit	7, a
		jr	z, loc_A5FB
		and	7Fh ; ''
		add	a, 50h ; 'P'
		srl	a
		cpl
		add	a, 68h ; 'h'
		srl	a
		cp	40h ; '@'
		jr	c, loc_A5FD

loc_A5FB:				; CODE XREF: MoveObject+63j
		ld	a, 3Fh ; '?'

loc_A5FD:				; CODE XREF: MoveObject+72j
		sla	l
		rra
		srl	a
		ld	(ix+1Eh), a

loc_A605:				; CODE XREF: MoveObject+5Ej
		xor	a
		bit	7, (ix+25h)
		jr	z, loc_A60E
		ld	a, 2

loc_A60E:				; CODE XREF: MoveObject+83j
		ld	(ix+1Ch), a
		ret	nz
		ld	de, (LenVector)
		ld	a, d
		sla	h
		jr	nc, loc_A686
		ld	b, 0Ah
		bit	0, (ix+26h)
		jr	z, loc_A625
		ld	b, 2

loc_A625:				; CODE XREF: MoveObject+9Aj
		cp	b
		jr	c, loc_A632
		ld	b, 1
		cp	10h
		jr	nc, loc_A62F
		inc	b

loc_A62F:				; CODE XREF: MoveObject+A5j
		ld	(ix+1Ch), b

loc_A632:				; CODE XREF: MoveObject+9Fj
		bit	2, (ix+21h)
		ret	z
		ld	a, (ix+8)
		xor	(ix+0Eh)
		and	80h ; 'А'
		ret	z
		ld	b, (iy+2)
		ld	a, (PilotRating2) ; >=#84 в трюме десант
					; 7,=1 война была или идёт
					; 6,=1 война закончилать
					; 3-1 -	ранг (=текущая стадия войны)
		and	a
		jr	nz, loc_A64B
		srl	b

loc_A64B:				; CODE XREF: MoveObject+C0j
		call	IntRnd_A
		cp	b
		ret	nc
		ld	a, (ix+26h)
		and	0Fh
		ret	nz
		ld	a, d
		cp	0Ch
		ret	nc
		sla	h
		ret	nc
		call	sub_FA84
		ret	nc
		ld	a, d
		cp	4
		jr	c, loc_A669
		sla	h
		ret	nc

loc_A669:				; CODE XREF: MoveObject+DDj
		ld	hl, nBeepFX
		set	6, (hl)
		bit	7, (ix+8)
		jr	z, loc_A67D
		ld	a, (DamageRearShield)
		add	a, 7
		ld	(DamageRearShield), a
		ret
; ---------------------------------------------------------------------------

loc_A67D:				; CODE XREF: MoveObject+EBj
		ld	a, (DamageFrontShield)
		add	a, 7
		ld	(DamageFrontShield), a
		ret
; ---------------------------------------------------------------------------

loc_A686:				; CODE XREF: MoveObject+92j
		ld	b, 0Bh
		bit	0, (ix+26h)
		jr	z, loc_A690
		ld	b, 3

loc_A690:				; CODE XREF: MoveObject+105j
		cp	b
		ret	c
		ld	b, 0FFh
		cp	12h
		jr	c, loc_A699
		dec	b

loc_A699:				; CODE XREF: MoveObject+10Fj
		ld	(ix+1Ch), b
		ret
; End of function MoveObject


; =============== S U B	R O U T	I N E =======================================

; Умножение hl = -1 * high((a*2) * (b*2))
; Вх:  b - первый множитель, 7,b - знак
;      a - второй множитель, 7,a - знак
; Вых: l,a = high((a*2)	* (b*2))
;	l,a = high((a*2) * (b*2))
;	7,h - знак

Mult_HL_A2_B2:				; CODE XREF: ScalarMultVectors+6p
					; ScalarMultVectors+10p ...
		ld	c, a
		xor	b
		xor	80h ; 'А'
		sla	b
		sla	c
		push	af
		call	Mult_A_hiB_C
		pop	hl
		ld	l, a
		ret
; End of function Mult_HL_A2_B2


; =============== S U B	R O U T	I N E =======================================


ScalarMultVectors:			; CODE XREF: MoveObject+1Cp
					; MoveObject+26p ...
		ld	b, (ix+1)
		ld	a, (iy+1)
		call	Mult_HL_A2_B2	; Умножение hl = -1 * high((a*2) * (b*2))
					; Вх:  b - первый множитель, 7,b - знак
					;      a - второй множитель, 7,a - знак
					; Вых: l,a = high((a*2)	* (b*2))
					;	l,a = high((a*2) * (b*2))
					;	7,h - знак
		push	hl
		ld	b, (ix+3)
		ld	a, (iy+3)
		call	Mult_HL_A2_B2	; Умножение hl = -1 * high((a*2) * (b*2))
					; Вх:  b - первый множитель, 7,b - знак
					;      a - второй множитель, 7,a - знак
					; Вых: l,a = high((a*2)	* (b*2))
					;	l,a = high((a*2) * (b*2))
					;	7,h - знак
		push	hl
		ld	b, (ix+5)
		ld	a, (iy+5)
		call	Mult_HL_A2_B2	; Умножение hl = -1 * high((a*2) * (b*2))
; End of function ScalarMultVectors	; Вх:  b - первый множитель, 7,b - знак
					;      a - второй множитель, 7,a - знак
					; Вых: l,a = high((a*2)	* (b*2))
					;	l,a = high((a*2) * (b*2))
					;	7,h - знак

; =============== S U B	R O U T	I N E =======================================




; === HL_CrdNdiv4_InvSign === CODE XREF: DirectObject+6Bp
HL_CrdNdiv4_InvSign:			; CODE XREF: DirectObject+6Bp
					; DirectObject+71p ...
		ld	a, (ix+2)
		xor	80h ; 'А'
		jr	loc_A572
; End of function HL_CrdNdiv4_InvSign


; =============== S U B	R O U T	I N E =======================================


HL_CrdNdiv4:				; CODE XREF: DirectObject+4Fp
					; DirectObject+55p ...
		ld	a, (ix+2)

loc_A572:				; CODE XREF: HL_CrdNdiv4_InvSign+5j
		ld	l, (ix+0)
		ld	h, (ix+1)
		srl	h
		rr	l
		rla
		rr	h
		rr	l
		ld	de, 3
		add	ix, de
		ret
; End of function HL_CrdNdiv4


; =============== S U B	R O U T	I N E =======================================

; Attributes: noreturn

MoveObject:				; CODE XREF: DirectObject:MoveObject1j
		push	iy
		ld	iy, deltaCrdX
		push	ix
		ld	ix, deltaCrdX
		call	CalcLenVector
; ---------------------------------------------------------------------------
		pop	ix
		ld	(LenVector), de
		push	ix
		ld	de, 9
		add	ix, de
		call	ScalarMultVectors
		ld	de, 6
		add	ix, de
		ld	h, a
		push	hl
		call	ScalarMultVectors
		pop	hl
		ld	l, a
		push	hl
		ld	de, 6
		add	ix, de
		call	ScalarMultVectors
		pop	hl
		pop	ix
		pop	iy
		ld	d, a
		and	7Fh ; ''
		jr	z, loc_A5E7
		ld	c, a
		srl	c
		srl	c
		srl	c
		adc	a, c
		srl	a
		srl	a
		cp	0Fh
		jr	c, loc_A5D7
		ld	a, 0Fh

loc_A5D7:				; CODE XREF: MoveObject+4Cj
		ld	b, a
		ld	a, d
		xor	l
		and	80h ; 'А'
		rra
		or	b
		ld	(ix+1Dh), a
		and	3Fh ; '?'
		cp	4
		jr	nc, loc_A605

loc_A5E7:				; CODE XREF: MoveObject+3Cj
		ld	a, h
		bit	7, a
		jr	z, loc_A5FB
		and	7Fh ; ''
		add	a, 50h ; 'P'
		srl	a
		cpl
		add	a, 68h ; 'h'
		srl	a
		cp	40h ; '@'
		jr	c, loc_A5FD

loc_A5FB:				; CODE XREF: MoveObject+63j
		ld	a, 3Fh ; '?'

loc_A5FD:				; CODE XREF: MoveObject+72j
		sla	l
		rra
		srl	a
		ld	(ix+1Eh), a

loc_A605:				; CODE XREF: MoveObject+5Ej
		xor	a
		bit	7, (ix+25h)
		jr	z, loc_A60E
		ld	a, 2

loc_A60E:				; CODE XREF: MoveObject+83j
		ld	(ix+1Ch), a
		ret	nz
		ld	de, (LenVector)
		ld	a, d
		sla	h
		jr	nc, loc_A686
		ld	b, 0Ah
		bit	0, (ix+26h)
		jr	z, loc_A625
		ld	b, 2

loc_A625:				; CODE XREF: MoveObject+9Aj
		cp	b
		jr	c, loc_A632
		ld	b, 1
		cp	10h
		jr	nc, loc_A62F
		inc	b

loc_A62F:				; CODE XREF: MoveObject+A5j
		ld	(ix+1Ch), b

loc_A632:				; CODE XREF: MoveObject+9Fj
		bit	2, (ix+21h)
		ret	z
		ld	a, (ix+8)
		xor	(ix+0Eh)
		and	80h ; 'А'
		ret	z
		ld	b, (iy+2)
		ld	a, (PilotRating2) ; >=#84 в трюме десант
					; 7,=1 война была или идёт
					; 6,=1 война закончилать
					; 3-1 -	ранг (=текущая стадия войны)
		and	a
		jr	nz, loc_A64B
		srl	b

loc_A64B:				; CODE XREF: MoveObject+C0j
		call	IntRnd_A
		cp	b
		ret	nc
		ld	a, (ix+26h)
		and	0Fh
		ret	nz
		ld	a, d
		cp	0Ch
		ret	nc
		sla	h
		ret	nc
		call	sub_FA84
		ret	nc
		ld	a, d
		cp	4
		jr	c, loc_A669
		sla	h
		ret	nc

loc_A669:				; CODE XREF: MoveObject+DDj
		ld	hl, nBeepFX
		set	6, (hl)
		bit	7, (ix+8)
		jr	z, loc_A67D
		ld	a, (DamageRearShield)
		add	a, 7
		ld	(DamageRearShield), a
		ret
; ---------------------------------------------------------------------------

loc_A67D:				; CODE XREF: MoveObject+EBj
		ld	a, (DamageFrontShield)
		add	a, 7
		ld	(DamageFrontShield), a
		ret
; ---------------------------------------------------------------------------

loc_A686:				; CODE XREF: MoveObject+92j
		ld	b, 0Bh
		bit	0, (ix+26h)
		jr	z, loc_A690
		ld	b, 3

loc_A690:				; CODE XREF: MoveObject+105j
		cp	b
		ret	c
		ld	b, 0FFh
		cp	12h
		jr	c, loc_A699
		dec	b

loc_A699:				; CODE XREF: MoveObject+10Fj
		ld	(ix+1Ch), b
		ret
; End of function MoveObject


; =============== S U B	R O U T	I N E =======================================

; Умножение hl = -1 * high((a*2) * (b*2))
; Вх:  b - первый множитель, 7,b - знак
;      a - второй множитель, 7,a - знак
; Вых: l,a = high((a*2)	* (b*2))
;	l,a = high((a*2) * (b*2))
;	7,h - знак

Mult_HL_A2_B2:				; CODE XREF: ScalarMultVectors+6p
					; ScalarMultVectors+10p ...
		ld	c, a
		xor	b
		xor	80h ; 'А'
		sla	b
		sla	c
		push	af
		call	Mult_A_hiB_C
		pop	hl
		ld	l, a
		ret
; End of function Mult_HL_A2_B2


; =============== S U B	R O U T	I N E =======================================


ScalarMultVectors:			; CODE XREF: MoveObject+1Cp
					; MoveObject+26p ...
		ld	b, (ix+1)
		ld	a, (iy+1)
		call	Mult_HL_A2_B2	; Умножение hl = -1 * high((a*2) * (b*2))
					; Вх:  b - первый множитель, 7,b - знак
					;      a - второй множитель, 7,a - знак
					; Вых: l,a = high((a*2)	* (b*2))
					;	l,a = high((a*2) * (b*2))
					;	7,h - знак
		push	hl
		ld	b, (ix+3)
		ld	a, (iy+3)
		call	Mult_HL_A2_B2	; Умножение hl = -1 * high((a*2) * (b*2))
					; Вх:  b - первый множитель, 7,b - знак
					;      a - второй множитель, 7,a - знак
					; Вых: l,a = high((a*2)	* (b*2))
					;	l,a = high((a*2) * (b*2))
					;	7,h - знак
		push	hl
		ld	b, (ix+5)
		ld	a, (iy+5)
		call	Mult_HL_A2_B2	; Умножение hl = -1 * high((a*2) * (b*2))
; End of function ScalarMultVectors	; Вх:  b - первый множитель, 7,b - знак
					;      a - второй множитель, 7,a - знак
					; Вых: l,a = high((a*2)	* (b*2))
					;	l,a = high((a*2) * (b*2))
					;	7,h - знак

; =============== S U B	R O U T	I N E =======================================




; === MakeTablPrnCrdVertex === CODE XREF: objPrnObj+25p
MakeTablPrnCrdVertex:			; CODE XREF: objPrnObj+25p

; FUNCTION CHUNK AT AB73 SIZE 0000003C BYTES
; FUNCTION CHUNK AT ACA2 SIZE 000000B9 BYTES
; FUNCTION CHUNK AT AD82 SIZE 0000000E BYTES
; FUNCTION CHUNK AT ADF2 SIZE 0000000F BYTES

		push	iy
		push	iy
		ld	a, (iy+3)	; a - кол-во вершин
		ex	(sp), ix
		ld	d, (iy+0Dh)
		call	sub_AE01	; de= кол-во вершин * 3
		add	ix, de		; ix = адрес начала вершин + кол-во вершин * 3
		ld	iy, MatrixRotate

loc_A9DE:				; CODE XREF: MakeTablPrnCrdVertex+430j
		push	af
		ld	a, (PointTablCrdPrn)
		add	a, 3
		ld	(PointTablCrdPrn), a
		call	sub_AA2B
		and	1Fh
		cp	1Fh
		jr	z, loc_A9F7
		ld	b, a
		ld	a, (DistanceToObj)
		cp	b
		jr	nc, loc_AA03

loc_A9F7:				; CODE XREF: MakeTablPrnCrdVertex+25j
		inc	hl
		ld	a, (hl)
		call	TestVergesVisible
		jr	nz, loc_AA03
		inc	hl
		ld	a, (hl)
		call	TestVergesVisible

loc_AA03:				; CODE XREF: MakeTablPrnCrdVertex+2Cj
					; MakeTablPrnCrdVertex+33j
		jp	nz, CalcPrnCrdVertex
		inc	a
		ld	(VisiblePrevVertex), a
		pop	af
		jp	loc_ADF2
; End of function MakeTablPrnCrdVertex

; ---------------------------------------------------------------------------
		db 3Fh


; === MultCrdsVectors === CODE XREF: CalcMatrixRotate+10p
MultCrdsVectors:			; CODE XREF: CalcMatrixRotate+10p
					; CalcMatrixRotate:loc_AEFAp ...
		sla	b
		rra
		xor	c
		sla	c
		and	80h ; 'А'
		ld	h, a
		sla	d
		rra
		xor	e
		sla	e
		and	80h ; 'А'
		xor	h
		sla	h
		rra
		xor	80h ; 'А'
		push	af
		push	de
		call	Mult_HL_B_C
		pop	bc
		push	hl
		call	Mult_HL_B_C
		pop	de
		pop	af
		bit	6, a
		jr	z, loc_AF51
		add	hl, de
		jr	nc, loc_AF5E
		ld	hl, 0FFFFh
		jr	loc_AF5E
; ---------------------------------------------------------------------------

loc_AF51:				; CODE XREF: MultCrdsVectors+25j
		ex	de, hl
		and	a
		sbc	hl, de
		jr	nc, loc_AF5E
		add	hl, de
		ex	de, hl
		and	a
		sbc	hl, de
		xor	80h ; 'А'

loc_AF5E:				; CODE XREF: MultCrdsVectors+28j
					; MultCrdsVectors+2Dj ...
		rla
		rr	h
		rr	l
		ret
; End of function MultCrdsVectors

; ---------------------------------------------------------------------------
		db 0DDh	; ▌
		db 0E5h	; х
		db 0DDh	; ▌
		db  21h	; !
		db    0
		db    0
		db  47h	; G
		db 0C5h	; ┼
		db 0D5h	; ╒
		db 0C5h	; ┼
		db 0D5h	; ╒
		db 0C5h	; ┼
		db 0D5h	; ╒
		db  43h	; C
		db 0CDh	; ═
		db  34h	; 4
		db 0ABh	; л
		db 0EBh	; ы
		db 0DDh	; ▌
		db  19h
		db 0D1h	; ╤
		db 0C1h	; ┴
		db  42h	; B
		db 0CDh	; ═
		db  34h	; 4
		db 0ABh	; л
		db  1Eh
		db    0
		db  55h	; U
		db 0DDh	; ▌
		db  19h
		db  6Ch	; l
		db  63h	; c
		db  30h	; 0
		db    1
		db  23h	; #
		db 0C1h	; ┴
		db 0D1h	; ╤
		db 0E5h	; х
		db  42h	; B
		db 0CDh	; ═
		db  34h	; 4
		db 0ABh	; л
		db  1Eh
		db    0
		db  55h	; U
		db 0DDh	; ▌
		db  19h
		db 0D1h	; ╤
		db  6Ch	; l
		db  26h	; &
		db    0
		db  30h	; 0
		db    1
		db  23h	; #
		db  19h
		db 0C1h	; ┴
		db 0D1h	; ╤
		db  4Ah	; J
		db 0E5h	; х
		db 0CDh	; ═
		db  34h	; 4
		db 0ABh	; л
		db 0D1h	; ╤
		db  19h
		db 0DDh	; ▌
		db 0E1h	; с
		db 0C9h	; ╔

; =============== S U B	R O U T	I N E =======================================

; Attributes: noreturn

Div_BC_HLAdivDE_1:			; CODE XREF: Div_BC_HLAdivDEj
					; CaclNormalVector+9p ...
		ld	bc, 0FFFEh

loc_AFAB:				; CODE XREF: Div_BC_HLAdivDE_1+14j
					; Div_BC_HLAdivDE_1+1Fj
		add	a, a
		rl	l
		rl	h
		jr	c, loc_AFBE
		sbc	hl, de
		jr	nc, loc_AFC1
		add	hl, de
		sla	c
		rl	b
		ret	nc
		jr	loc_AFAB
; ---------------------------------------------------------------------------

loc_AFBE:				; CODE XREF: Div_BC_HLAdivDE_1+8j
		and	a
		sbc	hl, de

loc_AFC1:				; CODE XREF: Div_BC_HLAdivDE_1+Cj
		scf
		rl	c
		rl	b
		ret	nc
		jr	loc_AFAB
; End of function Div_BC_HLAdivDE_1


; =============== S U B	R O U T	I N E =======================================




; === NewJumpDockingCrdObj === CODE XREF: GoDocking:loc_71BAp
NewJumpDockingCrdObj:			; CODE XREF: GoDocking:loc_71BAp
		ld	iy, SunCrd
		ld	ix, objCoriolis
		call	sub_71CE
		call	sub_71CE
; End of function NewJumpDockingCrdObj


; =============== S U B	R O U T	I N E =======================================




; === PrnCrd_CRLF === CODE XREF: RAM:6F0Ej
PrnCrd_CRLF:				; CODE XREF: RAM:6F0Ej
					; sub_D755:loc_D785j ...
		ld	a, 1
		ld	(byte_D0D2), a
		call	loc_E2B7
; End of function PrnCrd_CRLF


; =============== S U B	R O U T	I N E =======================================




; === ReCalcCrdObj === CODE XREF: sub_9D40:loc_9E29p
ReCalcCrdObj:				; CODE XREF: sub_9D40:loc_9E29p
		call	CalcNewCrdObj_Pl
		bit	2, (ix+26h)
		jr	nz, loc_9F90
		ld	a, l
		cp	60h ; '`'
		jr	nc, loc_9F95

loc_9F90:				; CODE XREF: ReCalcCrdObj+7j
		ld	a, h
		and	7Fh ; ''
		jr	z, loc_9F9C

loc_9F95:				; CODE XREF: ReCalcCrdObj+Cj
					; ReCalcCrdObj+1Ej ...
		xor	a
		ld	(ix+22h), a
		jp	OffTargetMissiles
; ---------------------------------------------------------------------------

loc_9F9C:				; CODE XREF: ReCalcCrdObj+11j
		ld	a, (ix+2)
		add	a, a
		jr	nz, loc_9F95
		ld	a, (ix+5)
		add	a, a
		jr	nz, loc_9F95
		ld	hl, (HighAngleLR)
		ld	(AngleLR4Calc),	hl
		push	ix
		ld	de, 9
		add	ix, de
		call	RotateAxisSCO
		call	RotateAxisSCO
		call	RotateAxisSCO
		pop	ix
		ld	a, (ix+1Dh)
		ld	b, a
		bit	7, a
		jr	nz, loc_9FD3
		and	0Fh
		ld	a, 0
		jr	z, loc_9FDD
		ld	a, b
		dec	a
		ld	(ix+1Dh), a

loc_9FD3:				; CODE XREF: ReCalcCrdObj+44j
		ld	a, 0Ch
		bit	6, (ix+1Dh)
		jr	z, loc_9FDD
		add	a, 80h ; 'А'

loc_9FDD:				; CODE XREF: ReCalcCrdObj+4Aj
					; ReCalcCrdObj+57j
		ld	(AngleLR4Calc),	a
		ld	a, (ix+1Eh)
		ld	b, a
		bit	7, a
		jr	nz, loc_9FF3
		and	0Fh
		ld	a, 0
		jr	z, DemoRotateObj
		ld	a, b
		dec	a
		ld	(ix+1Eh), a

loc_9FF3:				; CODE XREF: ReCalcCrdObj+64j
		ld	a, 8
		bit	6, (ix+1Eh)
		jr	z, DemoRotateObj
		add	a, 80h ; 'А'
; End of function ReCalcCrdObj


; =============== S U B	R O U T	I N E =======================================


DemoRotateObj:				; CODE XREF: ReCalcCrdObj+6Aj
					; ReCalcCrdObj+77j ...
		ld	(AngleLR4Calc+1), a
		push	ix
		call	RotateCrdNSCO
		call	RotateCrdNSCO
		call	RotateCrdNSCO
		pop	ix
		ld	a, (ix+1Ch)
		and	a
		jr	z, loc_A032
		ld	b, (ix+1Bh)
		add	a, b
		bit	7, a
		jr	z, loc_A01E
		xor	a
		jr	loc_A02F
; ---------------------------------------------------------------------------

loc_A01E:				; CODE XREF: DemoRotateObj+1Cj
		ld	c, (iy+6)
		bit	0, (ix+21h)
		jr	z, loc_A02B
		sla	c
		sla	c

loc_A02B:				; CODE XREF: DemoRotateObj+28j
		cp	c
		jr	c, loc_A02F
		ld	a, c

loc_A02F:				; CODE XREF: DemoRotateObj+1Fj
					; DemoRotateObj+2Fj
		ld	(ix+1Bh), a

loc_A032:				; CODE XREF: DemoRotateObj+14j
		push	ix
		push	iy
		ld	c, (ix+1Bh)
		push	ix
		pop	iy
		call	CalcNewCrdObjSpeed
		call	CalcNewCrdObjSpeed
		call	CalcNewCrdObjSpeed
		pop	iy
		pop	ix
		ret
; End of function DemoRotateObj


; =============== S U B	R O U T	I N E =======================================


CalcNewCrdObjSpeed:			; CODE XREF: DemoRotateObj+40p
					; DemoRotateObj+43p ...
		push	bc
		ld	d, (ix+0Ah)
		ld	e, (ix+9)
		call	sub_A076
		ld	a, (iy+2)
		ld	h, (iy+1)
		ld	l, (iy+0)
		call	sub_A098
		ld	(iy+2),	b
		ld	(iy+1),	h
		ld	(iy+0),	l
		inc	iy
		inc	iy
		inc	iy
		inc	ix
		inc	ix
		pop	bc
		ret
; End of function CalcNewCrdObjSpeed


; =============== S U B	R O U T	I N E =======================================




; === ReCalcCrdObj_coriolis === CODE XREF: CalcCrdCoriolis2:loc_7874p
ReCalcCrdObj_coriolis:			; CODE XREF: CalcCrdCoriolis2:loc_7874p
		call	j_CalcNewCrdObj_Pl
		call	ProgressMissionWar ; Увеличение/уменьшение прогресса таргонов, и выдача	сообщения о войне
					; Вх:  ---
					; Вых: hl,a - не определены
		call	TestMainDiagonal ; Определение главной диагонали
					; Вых: z=1 на главной диагонале
		jr	z, loc_5F3F
		call	sub_83FE	; расстояние до	планеты
		cp	7
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
		ld	b, 0Fh		; Targon
		jr	c, loc_5F87
		ld	a, (PilotNumGalaxy)
		and	a
		ret	z
		ld	a, (PilotMission3) ; =#06 при стыковке будет захват таргонами (вроде нереальная	ситуация)
					; =#04 в системе со танцией таргонов во	время войны
		cp	4
		ret	z
		ld	a, (PilotLegalStatus)
		cp	7Fh ; ''
		ld	b, 8		; Viper
		jr	nc, loc_5F87
		ret
; ---------------------------------------------------------------------------

loc_5F3F:				; CODE XREF: ReCalcCrdObj_coriolis+9j
		call	sub_83FE+2	; 83fe	 PlanetCrd
					; 83fe+2 SunCrd
		ret	c		; далеко от солнца
		cp	0Ch
		ret	nc		; далеко от солнца
		cp	6
		ret	z		; близко к солнцу
		ld	b, 0Dh		; ASP
		jr	nc, loc_5F81	; предложение о	покупке	кобры мк4
		ld	a, (ix+49h)
		and	a
		ret	nz
		push	ix
		pop	hl
		ld	b, 3
		ld	de, SunCrd

loc_5F5A:				; CODE XREF: ReCalcCrdObj_coriolis+55j
		inc	hl
		inc	de
		ld	a, (de)
		ld	(hl), a
		inc	hl
		inc	de
		ld	a, (de)
		and	80h ; 'А'
		ld	(hl), a
		inc	hl
		inc	de
		djnz	loc_5F5A
		ld	l, 48h ; 'H'
		ld	de, DataOfAround
		ld	c, 27h ; '''
		res	2, (ix+21h)
		call	loc_7892
		ldir
		inc	hl
		ld	a, (hl)
		xor	4
		ld	(hl), a
		pop	hl
		jp	loc_7889
; ---------------------------------------------------------------------------

loc_5F81:				; CODE XREF: ReCalcCrdObj_coriolis+3Aj
		ld	hl, 0F480h	; предложение о	покупке	кобры мк4
		ld	(CargoFound), hl

loc_5F87:				; CODE XREF: ReCalcCrdObj_coriolis+17j
					; ReCalcCrdObj_coriolis+2Bj
		push	ix
		ld	a, (DataOfAround.field_22)
		push	af
		ld	(ix+49h), 7Eh ;	'~'
		push	bc
		call	FindSlot4Obj
		pop	bc
		ld	a, b
		call	z, SetObjInSlot	; a - номер объекта для	установки
		pop	af
		ld	(DataOfAround.field_22), a
		pop	ix
		ret
; End of function ReCalcCrdObj_coriolis


; =============== S U B	R O U T	I N E =======================================

; предложение покупки кобры МК-4
; Attributes: noreturn



; === ScalarMultVectors === CODE XREF: MoveObject+1Cp
ScalarMultVectors:			; CODE XREF: MoveObject+1Cp
					; MoveObject+26p ...
		ld	b, (ix+1)
		ld	a, (iy+1)
		call	Mult_HL_A2_B2	; Умножение hl = -1 * high((a*2) * (b*2))
					; Вх:  b - первый множитель, 7,b - знак
					;      a - второй множитель, 7,a - знак
					; Вых: l,a = high((a*2)	* (b*2))
					;	l,a = high((a*2) * (b*2))
					;	7,h - знак
		push	hl
		ld	b, (ix+3)
		ld	a, (iy+3)
		call	Mult_HL_A2_B2	; Умножение hl = -1 * high((a*2) * (b*2))
					; Вх:  b - первый множитель, 7,b - знак
					;      a - второй множитель, 7,a - знак
					; Вых: l,a = high((a*2)	* (b*2))
					;	l,a = high((a*2) * (b*2))
					;	7,h - знак
		push	hl
		ld	b, (ix+5)
		ld	a, (iy+5)
		call	Mult_HL_A2_B2	; Умножение hl = -1 * high((a*2) * (b*2))
; End of function ScalarMultVectors	; Вх:  b - первый множитель, 7,b - знак
					;      a - второй множитель, 7,a - знак
					; Вых: l,a = high((a*2)	* (b*2))
					;	l,a = high((a*2) * (b*2))
					;	7,h - знак

; =============== S U B	R O U T	I N E =======================================




; === ScalarMultVectors16_8 === CODE XREF: CalcVisibleVerges+6Dp
ScalarMultVectors16_8:			; CODE XREF: CalcVisibleVerges+6Dp
		ld	e, (iy+1)
		ld	c, (iy+0)
		ld	a, (ix+0)
		call	Mult_HL_EC_A
		push	hl
		ld	e, (iy+3)
		ld	c, (iy+2)
		ld	a, (ix+1)
		call	Mult_HL_EC_A
		push	hl
		ld	e, (iy+5)
		ld	c, (iy+4)
		ld	a, (ix+2)
		call	Mult_HL_EC_A
		pop	de
		call	AddSign_HL_DE
		adc	hl, hl
		bit	7, b
		jr	z, loc_BD91
		set	7, h

loc_BD91:				; CODE XREF: ScalarMultVectors16_8+2Ej
		pop	de
		jp	AddSign_HL_DE
; End of function ScalarMultVectors16_8


; =============== S U B	R O U T	I N E =======================================


Mult_HL_EC_A:				; CODE XREF: ScalarMultVectors16_8+9p
					; ScalarMultVectors16_8+16p ...
		ld	b, a
		sla	b
		and	80h ; 'А'
		xor	e
		ld	e, a
		call	Mult_A_hiB_C
		ld	d, e
		ld	e, a
		ld	c, d
		res	7, c
		push	de
		call	Mult_HL_B_C
		pop	de
		ld	a, d
		ld	d, 0
		add	hl, de
		add	hl, hl
		rla
		rr	h
		rr	l
		ret
; End of function Mult_HL_EC_A


; =============== S U B	R O U T	I N E =======================================


RotateCrdN_Normal:			; CODE XREF: CalcVisibleVerges+47p
					; CalcVisibleVerges+50p ...
		call	RotateCrdN
		pop	bc
		ex	(sp), ix
		ld	e, (ix+0)
		ld	d, (ix+1)
		ld	a, (ix+2)
		inc	ix
		inc	ix
		inc	ix
		ex	(sp), ix
		push	bc
		ld	c, a
		ld	a, (IY_dFactor)
		sla	l
		push	hl
		ld	b, a
		inc	b

loc_BDD5:				; CODE XREF: RotateCrdN_Normal+23j
		srl	l
		djnz	loc_BDD5
		rl	l
		ld	a, c
		xor	h
		sla	h
		rla
		jr	c, loc_BDE5
		add	hl, de
		jr	loc_BDF4
; ---------------------------------------------------------------------------

loc_BDE5:				; CODE XREF: RotateCrdN_Normal+2Cj
		ex	de, hl
		and	a
		sbc	hl, de
		jr	nc, loc_BDF4
		add	hl, de
		ex	de, hl
		and	a
		sbc	hl, de
		ld	a, c
		xor	80h ; 'А'
		ld	c, a

loc_BDF4:				; CODE XREF: RotateCrdN_Normal+2Fj
					; RotateCrdN_Normal+35j
		sla	c
		rr	h
		rr	l
		pop	de
		ld	a, e
		rl	d
		rra
		ret
; End of function RotateCrdN_Normal

; ---------------------------------------------------------------------------


; === SetCrdCDEinExtSlot === CODE XREF: sub_7E02+5p sub_7E02+1Dp	...
SetCrdCDEinExtSlot:			; CODE XREF: sub_7E02+5p sub_7E02+1Dp	...
		push	af
		ld	hl, (word_B452)
		inc	hl
		ld	(hl), c
		inc	hl
		ld	(hl), d
		inc	hl
		ld	(hl), e
		inc	hl
		ld	(hl), c
		inc	hl
		ld	a, d
		xor	1
		ld	(hl), a
		inc	hl
		ld	a, e
		xor	1
		ld	(hl), a
		pop	af
		ret
; End of function SetCrdCDEinExtSlot


; =============== S U B	R O U T	I N E =======================================

; получение координат и	знаков проекции	объекта

GetCrdsProjection:			; CODE XREF: sub_7E02:objDestroyedp
					; sub_7E02+1Ap	...
		ld	de, (CrdXProjection)
		ld	a, d
		ld	d, e
		ld	e, a
		ld	a, (WhatOnScreen)
		bit	0, a
		ld	a, (CrdXYsign)
		jr	z, loc_F958
		xor	2

loc_F958:				; CODE XREF: GetCrdsProjection+Fj
		ld	c, a
		ret
; End of function GetCrdsProjection


; =============== S U B	R O U T	I N E =======================================

; проверка есть	ли ракета нацеленная на	текущий	объект



; === SetCrdSunPlInFreeSpace === CODE XREF: sub_5FD5+1Bp
SetCrdSunPlInFreeSpace:			; CODE XREF: sub_5FD5+1Bp
		exx
		ld	bc, 600h
		ld	de, unk_738E	; начальные координаты "солнца" и "планеты" в межпланетном пространстве
		ld	hl, fsSunCrdXl	; координаты "солнца" и "планеты" в межпланетном пространстве

loc_D5D5:				; CODE XREF: SetCrdSunPlInFreeSpace+12j
		ld	a, (de)
		inc	de
		ld	(hl), c
		inc	hl
		ld	(hl), c
		inc	hl
		ld	(hl), a
		inc	hl
		djnz	loc_D5D5
		exx
		ret
; End of function SetCrdSunPlInFreeSpace


; =============== S U B	R O U T	I N E =======================================




; === SetCrdX_SP === CODE XREF: RAM:6EFAp	RAM:6F02p ...
SetCrdX_SP:				; CODE XREF: RAM:6EFAp	RAM:6F02p ...
		pop	hl
		ld	a, (hl)
		inc	hl
		push	hl

loc_E2C0:				; CODE XREF: RAM:EA9Fp	sub_F072+5j
		ld	(CrdXPrnSymbol), a
		cp	20h ; ' '
		ret	c
		jr	PrnCrd_CRLF
; ---------------------------------------------------------------------------

loc_E2C8:				; CODE XREF: RAM:8D49j	RAM:loc_D8F5j ...
		ld	(CrdXPrnSymbol+1), a
		cp	18h
		ret	c
		ld	a, 1
		ld	(CrdXPrnSymbol+1), a
		jr	loc_E2B7
; End of function sub_E2B1


; =============== S U B	R O U T	I N E =======================================




; === SetPrnCrdInExtSlot === CODE XREF: PrintObject+3j
SetPrnCrdInExtSlot:			; CODE XREF: PrintObject+3j
		ld	a, (iy+14h)
		call	GetPrnCrdVertex	; Получение печатных координат вершины из таблицы по номеру
					; Вх:  a - номер вершины
					; Вых: de - печатные координаты	вершины
					;      a - знаки координат
					;      z =1 вершина печатается
					;      hl- не определен
		ld	hl, (word_B452)
		inc	hl
		ld	(hl), a
		inc	hl
		ld	(hl), d
		inc	hl
		ld	(hl), e
		push	hl
		ld	a, 3
		call	GetPrnCrdVertex	; Получение печатных координат вершины из таблицы по номеру
					; Вх:  a - номер вершины
					; Вых: de - печатные координаты	вершины
					;      a - знаки координат
					;      z =1 вершина печатается
					;      hl- не определен
		pop	hl
		inc	hl
		ld	(hl), a
		inc	hl
		ld	a, d
		xor	3
		ld	(hl), a
		inc	hl
		ld	a, e
		xor	3
		ld	(hl), a
		ret
; End of function SetPrnCrdInExtSlot


; =============== S U B	R O U T	I N E =======================================


SetBit2ExtSlot_rndA:			; CODE XREF: objPrnObj:objPrnLaserp
		ld	hl, (word_B452)
		ld	a, (hl)
		push	hl
		ld	hl, PlayerAggressor
		call	bit2A_equ_bit2HL
		pop	hl
		ld	(hl), a
		inc	hl
		ld	(hl), 0FFh
		jp	IntRnd_A
; End of function SetBit2ExtSlot_rndA


; =============== S U B	R O U T	I N E =======================================




; === j_CalcNewCrdObj_Pl === CODE XREF: ReCalcCrdObj_coriolisp
j_CalcNewCrdObj_Pl:			; CODE XREF: ReCalcCrdObj_coriolisp
					; CalcCrdSunPlanet+4p ...
		jp	CalcNewCrdObj_Pl
; End of function j_CalcNewCrdObj_Pl


; =============== S U B	R O U T	I N E =======================================

; Attributes: noreturn thunk



