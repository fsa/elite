; 08_radar_scanner - разобрано из kladov.asm
; Количество подпрограмм: 5

; === CopyScaner2Scr === CODE XREF: RAM:7529p
CopyScaner2Scr:				; CODE XREF: RAM:7529p
					; SaveCocpitInBuffer+1p ...

; FUNCTION CHUNK AT 9935 SIZE 00000010 BYTES

		ld	a, (FlgSaveScaner)
		dec	a
		ret	nz
		ld	(FlgSaveScaner), a
		xor	a
		ld	c, 8
		ld	hl, 502Ah

loc_98DB:				; CODE XREF: CopyScaner2Scr+18j
		ld	b, 0Bh
		push	hl

loc_98DE:				; CODE XREF: CopyScaner2Scr+13j
		ld	(hl), a
		inc	l
		djnz	loc_98DE
		pop	hl
		inc	h
		dec	c
		jr	nz, loc_98DB
		ld	bc, 5
		xor	a
		ld	hl, loc_9912
		ld	(hl), a
		ld	a, 0D5h	; '╒'
		ld	hl, loc_9915
		ld	(hl), a
		add	hl, bc
		ld	(hl), a
		ld	a, 0D1h	; '╤'
		ld	hl, loc_9935
		ld	(hl), a
		inc	hl
		ld	a, 14h
		ld	(hl), a
		ld	a, 0EBh	; 'ы'
		ld	hl, loc_9940
		ld	(hl), a
		xor	a
		ld	hl, loc_9939
		ld	(hl), a

loc_990C:				; CODE XREF: RAM:98CBj
		ld	de, 5049h
		ld	hl, 5B00h

loc_9912:				; DATA XREF: RAM:98AAo
					; CopyScaner2Scr+1Eo
		nop
		ld	a, 6

loc_9915:				; CODE XREF: CopyScaner2Scr+75j
					; DATA XREF: RAM:98B0o	...
		push	de
		ld	b, 8
		ld	c, 0FFh

loc_991A:				; CODE XREF: CopyScaner2Scr+6Aj
		push	de
		push	bc
		ld	bc, 0Dh
		ldir
		pop	bc
		jr	loc_9935
; End of function CopyScaner2Scr


; =============== S U B	R O U T	I N E =======================================




; === PrintCompass === CODE XREF: sub_74C6:loc_750Fp
PrintCompass:				; CODE XREF: sub_74C6:loc_750Fp

; FUNCTION CHUNK AT 821E SIZE 00000053 BYTES
; FUNCTION CHUNK AT 827A SIZE 00000036 BYTES
; FUNCTION CHUNK AT 82D1 SIZE 00000006 BYTES
; FUNCTION CHUNK AT 8390 SIZE 0000006E BYTES

		xor	a
		ld	(J_Drive_Ready2), a
		ld	ix, SunCrd
		ld	a, (InFreeSpace)
		or	(ix+2)
		or	(ix+5)
		or	(ix+8)
		and	7Fh ; ''
		nop
		nop
		nop
		nop
		nop
		jr	nz, loc_81F7
		call	CalcLenVectorCrd
; ---------------------------------------------------------------------------
		ld	a, h
		or	l
		jr	z, loc_81F7
		ld	a, 1
		ld	(J_Drive_Ready2), a
		ex	de, hl
		ld	hl, 0
		sbc	hl, de
		ex	de, hl
		ld	hl, 4000h
		and	a
		sbc	hl, de
		jp	c, loc_82D1
		ld	hl, 2000h
		xor	a
		sbc	hl, de
		jr	nc, loc_81EC
		ld	a, 1

loc_81EC:				; CODE XREF: PrintCompass+3Fj
		ld	(FuelScoopsOn),	a
		ex	de, hl
		call	CalcTempAltitude
		ld	a, h
		call	sub_8B3F

loc_81F7:				; CODE XREF: PrintCompass+1Bj
					; PrintCompass+22j
		xor	a
		ld	(J_Drive_Ready3), a
		ld	ix, PlanetCrd
		ld	a, (InFreeSpace)
		and	a
		jr	nz, loc_827A
		ld	a, (ix+2)
		or	(ix+5)
		or	(ix+8)
		and	7Fh ; ''
		cp	2
		jr	nc, loc_827A
		and	a
		jr	loc_821E
; End of function PrintCompass


; =============== S U B	R O U T	I N E =======================================




; === PrintObjScaner === CODE XREF: sub_ED60+6j
PrintObjScaner:				; CODE XREF: sub_ED60+6j
		call	CopyLocator2Scr
; END OF FUNCTION CHUNK	FOR sub_ED60

; =============== S U B	R O U T	I N E =======================================




; === PrnCompass === CODE XREF: PrintCompass+1EDp
PrnCompass:				; CODE XREF: PrintCompass+1EDp
					; PrnCocpit_Ifn:loc_8459p
		ld	hl, DataCompass	; круг компаса
		ld	de, 5035h
		ld	bc, 1823h

loc_84BE:				; CODE XREF: PrnCompass+21j
		ldi
		ldi
		ldi
		bit	7, c
		jr	z, loc_84CE
		push	bc
		ld	bc, 0FFFAh
		add	hl, bc
		pop	bc

loc_84CE:				; CODE XREF: PrnCompass+11j
		dec	e
		dec	e
		dec	e
		ex	de, hl
		call	sub_7FE7
		ex	de, hl
		djnz	loc_84BE
		ret
; End of function PrnCompass


; =============== S U B	R O U T	I N E =======================================


UpdateAllIndicators:			; CODE XREF: RAM:7505p
					; LaunchDocking+D0p
		ld	de, word_8524
		exx
		ld	c, 0Bh
		exx
		ld	hl, ShieldFront

loc_84E3:				; CODE XREF: UpdateAllIndicators+23j
		ld	a, (hl)
		dec	hl
		cp	(hl)
		jr	z, loc_84F4
		ld	(hl), a
		srl	a
		srl	a
		push	hl
		push	de
		call	sub_854F
		pop	de
		pop	hl

loc_84F4:				; CODE XREF: UpdateAllIndicators+Dj
		inc	de
		inc	de
		inc	hl
		inc	hl
		inc	hl
		exx
		dec	c
		exx
		jr	nz, loc_84E3
		ld	a, 1
		ld	(byte_8523), a
		exx
		ld	c, 2
		exx

loc_8507:				; CODE XREF: UpdateAllIndicators+43j
		ld	a, (hl)
		dec	hl
		cp	(hl)
		jr	z, loc_8514
		ld	(hl), a
		push	hl
		push	de
		call	sub_8540
		pop	de
		pop	hl

loc_8514:				; CODE XREF: UpdateAllIndicators+31j
		inc	de
		inc	de
		inc	hl
		inc	hl
		inc	hl
		exx
		dec	c
		exx
		jr	nz, loc_8507
		xor	a
		ld	(byte_8523), a
		ret
; End of function UpdateAllIndicators

; ---------------------------------------------------------------------------
byte_8523:	db 0			; DATA XREF: UpdateAllIndicators+27w
					; UpdateAllIndicators+46w ...
word_8524:	dw 5123h		; DATA XREF: UpdateAllIndicatorso
		dw 5143h
		dw 5163h
		dw 5183h
		dw 51A3h
		dw 51C3h
		dw 5138h
		dw 5198h
		dw 51B8h
		dw 51D8h
		dw 51F8h
		dw 5158h
		dw 5178h
		dw 51E3h

; =============== S U B	R O U T	I N E =======================================




; === RenewCompass === CODE XREF: PrintCompass+104j
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



