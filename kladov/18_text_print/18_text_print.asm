; 18_text_print - разобрано из kladov.asm
; Количество подпрограмм: 23

; === ClsAndPrintRamk === CODE XREF: sub_5FA1+16p
ClsAndPrintRamk:			; CODE XREF: sub_5FA1+16p
					; j_ClsAndPrintRamkj ...
		di
		ld	hl, 4200h

loc_D621:				; CODE XREF: ClsAndPrintRamk+17j
		push	hl
		ld	bc, 2000h

loc_D625:				; CODE XREF: ClsAndPrintRamk+Aj
		ld	(hl), c
		inc	hl
		djnz	loc_D625
		pop	hl
		inc	h
		ld	a, h
		and	7
		call	z, compDownScrAdr_HL
		ld	a, h

loc_D632:				; DATA XREF: RAM:D78Ao
		cp	58h ; 'X'
		jr	nz, loc_D621
		ld	l, 20h ; ' '
		ld	d, h
		ld	e, l
		inc	de
		ld	bc, 2FFh
		ld	(hl), 7
		ldir
		nop
		nop
		nop
		ei

loc_D646:				; CODE XREF: Cmd_I_GlobalMap+42p
					; RAM:F063j
		ld	hl, 4020h
		push	hl
		call	PrintHorLine
		pop	hl

loc_D64E:				; CODE XREF: ClsAndPrintRamk+46j
		push	hl
		pop	ix
		set	7, (ix+0)
		set	0, (ix+1Fh)
		inc	h
		ld	a, h
		and	7
		call	z, compDownScrAdr_HL
		ld	a, h
		cp	58h ; 'X'
		jr	nz, loc_D64E
		ld	hl, 57E0h
		call	PrintHorLine
		xor	a
		ld	hl, 200h
		ld	(CrdXPrnSymbol), hl
		ret
; End of function ClsAndPrintRamk

; ---------------------------------------------------------------------------
		db 0D0h	; ╨
		db 0C9h	; ╔

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




; === DubleCRLF === CODE XREF: RAM:E987p	RAM:EE1Ep ...
DubleCRLF:				; CODE XREF: RAM:E987p	RAM:EE1Ep ...
		call	PrnCrd_CRLF

loc_EEA5:				; CODE XREF: sub_EE9D+3j
		jp	PrnCrd_CRLF
; End of function DubleCRLF


; =============== S U B	R O U T	I N E =======================================




; === NextExtSlot === CODE XREF: sub_9D40+11Dp
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


; === PrintCursorOnScr === CODE XREF: PrintSmallCursOnMap:loc_E4D1j
PrintCursorOnScr:			; CODE XREF: PrintSmallCursOnMap:loc_E4D1j
					; Cmd_I_GlobalMap+22p ...
		ld	a, (CursorWidth)
		srl	a
		ld	b, a
		ld	a, (PrnCrdXCursor)
		sub	b
		jr	c, loc_E56F
		jr	z, loc_E56F
		ld	d, a
		ld	a, (PrnCrdYCursor)
		ld	c, a
		ld	a, d
		call	sub_E621
		ld	a, (PrnCrdXCursor)
		add	a, b
		jr	c, loc_E594
		and	7
		jr	nz, loc_E551
		ld	a, 8

loc_E551:				; CODE XREF: PrintSmallCursOnMap+DEj
		ld	e, a
		ld	a, (CursorWidth)
		sub	e
		ld	c, a
		and	7
		jr	nz, loc_E561
		ld	d, 8
		ld	a, c
		sub	d
		jr	loc_E563
; ---------------------------------------------------------------------------

loc_E561:				; CODE XREF: PrintSmallCursOnMap+EAj
		ld	d, a
		ld	a, c

loc_E563:				; CODE XREF: PrintSmallCursOnMap+F0j
		and	0F8h ; '°'
		jr	z, loc_E5B4
		rra
		rra
		rra
		and	1Fh
		jp	loc_E5CF
; ---------------------------------------------------------------------------

loc_E56F:				; CODE XREF: PrintSmallCursOnMap+C9j
					; PrintSmallCursOnMap+CBj
		ld	a, (PrnCrdYCursor)
		ld	c, a
		xor	a
		call	sub_E621
		ld	a, (PrnCrdXCursor)
		add	a, b
		ld	c, a
		and	7
		jr	nz, loc_E586
		ld	e, 8
		ld	a, c
		sub	e
		jr	loc_E588
; ---------------------------------------------------------------------------

loc_E586:				; CODE XREF: PrintSmallCursOnMap+10Fj
		ld	e, a
		ld	a, c

loc_E588:				; CODE XREF: PrintSmallCursOnMap+115j
		and	0F8h ; '°'
		jr	z, loc_E5C0
		rra
		rra
		rra
		and	1Fh
		jp	loc_E5E4
; ---------------------------------------------------------------------------

loc_E594:				; CODE XREF: PrintSmallCursOnMap+DAj
		ld	a, (PrnCrdXCursor)
		ld	c, a
		xor	a
		sub	c
		add	a, b
		ld	c, a
		and	7
		jr	nz, loc_E5A6
		ld	d, 8
		ld	a, c
		sub	d
		jr	loc_E5A8
; ---------------------------------------------------------------------------

loc_E5A6:				; CODE XREF: PrintSmallCursOnMap+12Fj
		ld	d, a
		ld	a, c

loc_E5A8:				; CODE XREF: PrintSmallCursOnMap+135j
		and	0F8h ; '°'
		jr	z, loc_E5C7
		rra
		rra
		rra
		and	1Fh
		jp	loc_E5F9
; ---------------------------------------------------------------------------

loc_E5B4:				; CODE XREF: PrintSmallCursOnMap+F6j
		call	sub_E611
		call	sub_E619
		ld	a, (hl)
		xor	d
		ld	(hl), a
		inc	hl
		jr	loc_E5E1
; ---------------------------------------------------------------------------

loc_E5C0:				; CODE XREF: PrintSmallCursOnMap+11Bj
		call	sub_E611
		res	7, e
		jr	loc_E5E1
; ---------------------------------------------------------------------------

loc_E5C7:				; CODE XREF: PrintSmallCursOnMap+13Bj
		call	sub_E619
		res	0, d
		jp	loc_E644
; ---------------------------------------------------------------------------

loc_E5CF:				; CODE XREF: PrintSmallCursOnMap+FDj
		ld	b, a
		call	sub_E611
		call	sub_E619
		ld	a, (hl)
		xor	d
		ld	(hl), a
		inc	hl

loc_E5DA:				; CODE XREF: PrintSmallCursOnMap+170j
		ld	a, (hl)
		xor	0FFh
		ld	(hl), a
		inc	hl
		djnz	loc_E5DA

loc_E5E1:				; CODE XREF: PrintSmallCursOnMap+14Fj
					; PrintSmallCursOnMap+156j ...
		jp	loc_E63F
; ---------------------------------------------------------------------------

loc_E5E4:				; CODE XREF: PrintSmallCursOnMap+122j
		ld	b, a
		call	sub_E611
		ld	a, (hl)
		xor	7Fh ; ''
		ld	(hl), a
		inc	hl
		dec	b
		jr	z, loc_E5E1

loc_E5F0:				; CODE XREF: PrintSmallCursOnMap+186j
		ld	a, (hl)
		xor	0FFh
		ld	(hl), a
		inc	hl
		djnz	loc_E5F0
		jr	loc_E5E1
; ---------------------------------------------------------------------------

loc_E5F9:				; CODE XREF: PrintSmallCursOnMap+142j
		ld	b, a
		call	sub_E619
		ld	a, (hl)
		xor	d
		ld	(hl), a
		inc	hl
		dec	b
		jr	z, loc_E60B

loc_E604:				; CODE XREF: PrintSmallCursOnMap+19Aj
		ld	a, (hl)
		xor	0FFh
		ld	(hl), a
		inc	hl
		djnz	loc_E604

loc_E60B:				; CODE XREF: PrintSmallCursOnMap+193j
		ld	a, (hl)
		xor	0FEh ; '■'
		ld	(hl), a
		jr	loc_E647
; End of function PrintSmallCursOnMap


; =============== S U B	R O U T	I N E =======================================




; === PrintIndDanger === CODE XREF: _PrintIndDangerj
PrintIndDanger:				; CODE XREF: _PrintIndDangerj
		ld	hl, IndDanger
		ld	a, (hl)
		and	a
		ret	z
		ld	a, (ShieldFront)
		and	a
		jr	z, loc_99B6
		ld	a, (ShieldRear)
		and	a
; ---------------------------------------------------------------------------
		db  20h

; =============== S U B	R O U T	I N E =======================================




; === PrintMenuRamk === CODE XREF: sub_E87E:loc_E8EAp
PrintMenuRamk:				; CODE XREF: sub_E87E:loc_E8EAp
					; RAM:Cmd_4_EquipShipp
		call	PrintRamkMenu
		ld	de, unk_E8D8
		call	SetAttrFrom_DE
		jp	FillAttr_21line
; End of function sub_E87E


; =============== S U B	R O U T	I N E =======================================


PrintRamkMenu:				; CODE XREF: sub_E87E:PrintMenuRamkp
					; RAM:EDE2p ...
		ld	a, 96h ; 'Ц'
		jp	PrintMessage2
; End of function PrintRamkMenu

; ---------------------------------------------------------------------------
; START	OF FUNCTION CHUNK FOR sub_E87E

loc_E8EA:				; CODE XREF: sub_E87E+34j sub_E87E+3Cj
		call	PrintMenuRamk
		call	PrintSymbol_SP
; ---------------------------------------------------------------------------
		db 0CDh
; ---------------------------------------------------------------------------
		ld	a, 0CEh	; '╬'
		call	sub_E306
		call	sub_E962

loc_E8F9:				; CODE XREF: sub_E87E+A6j
		ld	hl, (word_E1E2)
		inc	hl
		inc	hl
		ld	(word_E1E2), hl
		call	loc_E268
		jr	c, loc_E91D
		call	sub_E364

loc_E909:				; CODE XREF: sub_E87E+AFj
		call	sub_E87E
		ld	hl, sub_E935
		call	sub_E776
		jr	c, loc_E92C
		call	sub_E7CF
; ---------------------------------------------------------------------------

loc_E917:				; CODE XREF: sub_E87E+B2j
		call	sub_E38D
		call	PrnCrd_CRLF

loc_E91D:				; CODE XREF: sub_E87E+86j
		ld	hl, byte_E1DF
		inc	(hl)
		ld	a, (hl)
		cp	11h
		jp	c, loc_E8F9
		call	Pause_24Int
		jr	loc_E932
; ---------------------------------------------------------------------------

loc_E92C:				; CODE XREF: sub_E87E+94j
		dec	a
		jr	z, loc_E909
		dec	a
		jr	z, loc_E917

loc_E932:				; CODE XREF: sub_E87E+ACj
		jp	Cmd_Ent_Inventory
; END OF FUNCTION CHUNK	FOR sub_E87E

; =============== S U B	R O U T	I N E =======================================




; === PrintMessage1_A === CODE XREF: sub_5E1A+13j RAM:8D44p ...
PrintMessage1_A:			; CODE XREF: sub_5E1A+13j RAM:8D44p ...

; FUNCTION CHUNK AT D59C SIZE 00000018 BYTES
; FUNCTION CHUNK AT D7E2 SIZE 0000000F BYTES

		cp	0Eh
		jp	c, loc_D7E2
		cp	20h ; ' '
		jp	c, loc_D441
		cp	60h ; '`'
		jp	c, sub_D499
		cp	80h ; 'А'
		jr	c, loc_D443
		cp	0A0h ; 'а'
		jp	c, loc_D59C
		sub	0A0h ; 'а'
		jr	loc_D443
; ---------------------------------------------------------------------------

loc_D441:				; CODE XREF: PrintMessage1_A+7j
		add	a, 72h ; 'r'

loc_D443:				; CODE XREF: PrintMessage1_A+11j
					; PrintMessage1_A+1Aj
		ld	hl, (word_F11A)
		ld	d, a
		inc	d

loc_D448:				; CODE XREF: PrintMessage1_A+2Dj
		dec	d
		jp	z, loc_D454
		ld	bc, 0FFFFh
		xor	a
		cpir
		jr	loc_D448
; ---------------------------------------------------------------------------

loc_D454:				; CODE XREF: PrintMessage1_A+24j
					; PrintMessage1_A+38j
		ld	a, (hl)
		and	a
		ret	z
		push	hl
		call	PrintMessage1_A
		pop	hl
		inc	hl
		jr	loc_D454
; End of function PrintMessage1_A


; =============== S U B	R O U T	I N E =======================================


PrintMessage2:				; CODE XREF: sub_5E1A+19j sub_5FA1+1Bp ...

; FUNCTION CHUNK AT D48E SIZE 0000000B BYTES

		ld	hl, (word_F11C)
		ld	d, a

loc_D463:				; CODE XREF: PrintMessage2+Ej
		dec	d
		jr	z, loc_D48E
		ld	bc, 0FFFFh
		ld	a, 0
		cpir
		jp	loc_D463
; End of function PrintMessage2


; =============== S U B	R O U T	I N E =======================================




; === PrintMessage1_A_Space === CODE XREF: RAM:E97Cp	RAM:E9E8p ...
PrintMessage1_A_Space:			; CODE XREF: RAM:E97Cp	RAM:E9E8p ...
		call	PrintMessage1_A
		ld	a, 20h ; ' '

loc_E2A3:				; CODE XREF: sub_E292+Aj
		jp	PrintMessage1_A
; End of function PrintMessage1_A_Space


; =============== S U B	R O U T	I N E =======================================


PrnMsg_A_CRLF:				; CODE XREF: sub_D755p
					; sub_D822:loc_D848j ...
		call	PrintMessage1_A
; End of function PrnMsg_A_CRLF


; =============== S U B	R O U T	I N E =======================================


PrnCrd_CRLF:				; CODE XREF: RAM:6F0Ej
					; sub_D755:loc_D785j ...
		ld	a, 1
		ld	(byte_D0D2), a
		call	loc_E2B7
; End of function PrnCrd_CRLF


; =============== S U B	R O U T	I N E =======================================




; === PrintMessage2 === CODE XREF: sub_5E1A+19j sub_5FA1+1Bp ...
PrintMessage2:				; CODE XREF: sub_5E1A+19j sub_5FA1+1Bp ...

; FUNCTION CHUNK AT D48E SIZE 0000000B BYTES

		ld	hl, (word_F11C)
		ld	d, a

loc_D463:				; CODE XREF: PrintMessage2+Ej
		dec	d
		jr	z, loc_D48E
		ld	bc, 0FFFFh
		ld	a, 0
		cpir
		jp	loc_D463
; End of function PrintMessage2


; =============== S U B	R O U T	I N E =======================================




; === PrintNumber_A === CODE XREF: RAM:6EF7p	sub_D84B+9j ...
PrintNumber_A:				; CODE XREF: RAM:6EF7p	sub_D84B+9j ...
		and	a

loc_D2E2:				; CODE XREF: sub_D2DE+1j
		ld	h, 0
		ld	l, a
; End of function PrintNumber_A


PrintNumber_HL:				; CODE XREF: PrintNumber_HLDE+Bp
					; PrintNumber_HLDE+20j	...
		push	af
		push	hl
		call	sub_D37D
		pop	hl
		ld	a, 1
		ld	(DigitsInNumber), a
		ld	(word_D2DC), hl
		ld	de, 2710h
		call	sub_D357
		ld	de, 3E8h
		call	sub_D357
		ld	de, 64h	; 'd'
		call	sub_D357
		ld	de, 0Ah
		call	sub_D357
		pop	af
		jr	nc, loc_D31D
		ld	a, (DigitsInNumber)
		and	a
		jp	z, loc_D319
		xor	a
		call	sub_D371

loc_D319:				; CODE XREF: RAM:D312j
		call	PrintSymbol_SP
; ---------------------------------------------------------------------------
		db 2Eh
; ---------------------------------------------------------------------------

loc_D31D:				; CODE XREF: RAM:D30Cj
		ld	hl, (word_D2DC)
		ld	a, l
		call	sub_D371
		xor	a
		ld	(byte_D0CE), a
		inc	a
		ld	(DigitsInNumber), a
		ret

; =============== S U B	R O U T	I N E =======================================




; === PrintNumber_HL === CODE XREF: PrintNumber_HLDE+Bp
PrintNumber_HL:				; CODE XREF: PrintNumber_HLDE+Bp
					; PrintNumber_HLDE+20j	...
		push	af
		push	hl
		call	sub_D37D
		pop	hl
		ld	a, 1
		ld	(DigitsInNumber), a
		ld	(word_D2DC), hl
		ld	de, 2710h
		call	sub_D357
		ld	de, 3E8h
		call	sub_D357
		ld	de, 64h	; 'd'
		call	sub_D357
		ld	de, 0Ah
		call	sub_D357
		pop	af
		jr	nc, loc_D31D
		ld	a, (DigitsInNumber)
		and	a
		jp	z, loc_D319
		xor	a
		call	sub_D371

loc_D319:				; CODE XREF: RAM:D312j
		call	PrintSymbol_SP
; ---------------------------------------------------------------------------
		db 2Eh
; ---------------------------------------------------------------------------

loc_D31D:				; CODE XREF: RAM:D30Cj
		ld	hl, (word_D2DC)
		ld	a, l
		call	sub_D371
		xor	a
		ld	(byte_D0CE), a
		inc	a
		ld	(DigitsInNumber), a
		ret

; =============== S U B	R O U T	I N E =======================================




; === PrintNumber_HLDE === CODE XREF: RAM:6F0Bp	RAM:E9CEp
PrintNumber_HLDE:			; CODE XREF: RAM:6F0Bp	RAM:E9CEp
		push	de
		ld	a, l
		or	h
		jr	z, loc_D34D
		ld	a, 3
		ld	(byte_D0CE), a
		and	a
		call	PrintNumber_HL
		xor	a
		ld	(DigitsInNumber), a
		ld	a, 4
		ld	(byte_D0CE), a
		jr	loc_D352
; ---------------------------------------------------------------------------

loc_D34D:				; CODE XREF: PrintNumber_HLDE+3j
		ld	a, 7
		ld	(byte_D0CE), a

loc_D352:				; CODE XREF: PrintNumber_HLDE+17j
		pop	hl
		scf
		jp	PrintNumber_HL
; End of function PrintNumber_HLDE


; =============== S U B	R O U T	I N E =======================================




; === PrintObject === CODE XREF: objPrnObj+28p
PrintObject:				; CODE XREF: objPrnObj+28p
		call	PrintObject_2
		jp	SetPrnCrdInExtSlot ; номер вершины с лазером
; End of function PrintObject

; ---------------------------------------------------------------------------


; === PrintObject_2 === CODE XREF: PrintObjectp
PrintObject_2:				; CODE XREF: PrintObjectp
					; DATA XREF: PrintObject_2+3w ...
		ld	a, 2Dh ; '-'
		inc	a
		ld	(PrintObject_2+1), a
		jr	z, loc_6F5B
		ld	a, (DistanceToObj)
		cp	8
		jr	c, loc_6F5B
		ld	a, (ix+26h)
		and	a
		jr	nz, loc_6F5B
		ld	a, 0A8h	; 'и'   ; xor b
		ld	(loc_AA1E), a

loc_6F5B:				; CODE XREF: PrintObject_2+6j
					; PrintObject_2+Dj ...
		ld	e, (iy+0Eh)
		ld	d, (iy+0Fh)
		ld	b, (iy+4)
		ld	c, 1
		push	iy
		ex	(sp), ix
		add	ix, de
		ld	l, (ix+0)
		ld	h, (ix+1)
		ld	e, (ix+2)
		ld	d, (ix+3)

loc_6F78:				; CODE XREF: PrintObject_2:loc_6FE1j
		ld	a, (hl)
		bit	5, a
		jr	nz, loc_6F80
		ld	c, a
		inc	hl
		ld	a, (hl)

loc_6F80:				; CODE XREF: PrintObject_2+3Aj
		ld	(CrdNVertex), a	; координата N текущей вершины
		and	1Fh
		dec	c
		jr	nz, loc_6F8A
		inc	c
		inc	hl

loc_6F8A:				; CODE XREF: PrintObject_2+45j
		push	bc
		push	de
		push	hl
		cp	1Fh
		jr	z, loc_6F98
		ld	b, a
		ld	a, (DistanceToObj)
		cp	b
		jr	nc, loc_6FD5

loc_6F98:				; CODE XREF: PrintObject_2+4Ej
		ld	a, (ix+4)
		call	TestVergesVisible
		jr	z, loc_6FD5
		ld	hl, CrdNVertex	; координата N текущей вершины
		rl	(hl)
		ld	a, (de)
		rra
		rl	(hl)
		ld	(hl), a
		ld	a, (de)
		jr	nc, loc_6FB3
		and	0Fh
		add	a, 10h
		jr	loc_6FB5
; ---------------------------------------------------------------------------

loc_6FB3:				; CODE XREF: PrintObject_2+6Aj
		and	0Fh

loc_6FB5:				; CODE XREF: PrintObject_2+70j
		call	GetPrnCrdVertex	; Получение печатных координат вершины из таблицы по номеру
					; Вх:  a - номер вершины
					; Вых: de - печатные координаты	вершины
					;      a - знаки координат
					;      z =1 вершина печатается
					;      hl- не определен
		jr	nz, loc_6FD5
		ld	c, a
		push	de
		ld	a, (CrdNVertex)	; координата N текущей вершины
		rrca
		rrca
		rrca
		and	1Fh
		call	GetPrnCrdVertex	; Получение печатных координат вершины из таблицы по номеру
					; Вх:  a - номер вершины
					; Вых: de - печатные координаты	вершины
					;      a - знаки координат
					;      z =1 вершина печатается
					;      hl- не определен
		ex	de, hl
		pop	de
		jr	nz, loc_6FD5
		rrca
		rra
		rl	c
		rla
		rl	c
		call	PrnLineInBuf_clip

loc_6FD5:				; CODE XREF: PrintObject_2+55j
					; PrintObject_2+5Dj ...
		pop	hl
		pop	de
		pop	bc
		inc	de
		ld	a, (ix+5)
		inc	a
		jr	z, loc_6FE1
		inc	ix

loc_6FE1:				; CODE XREF: PrintObject_2+9Cj
		djnz	loc_6F78
		pop	ix
		ld	a, 0B0h	; '░'   ; or b
		ld	(loc_AA1E), a
		ret
; End of function PrintObject_2

; ---------------------------------------------------------------------------


; === PrintRamkMenu === CODE XREF: sub_E87E:PrintMenuRamkp
PrintRamkMenu:				; CODE XREF: sub_E87E:PrintMenuRamkp
					; RAM:EDE2p ...
		ld	a, 96h ; 'Ц'
		jp	PrintMessage2
; End of function PrintRamkMenu

; ---------------------------------------------------------------------------
; START	OF FUNCTION CHUNK FOR sub_E87E

loc_E8EA:				; CODE XREF: sub_E87E+34j sub_E87E+3Cj
		call	PrintMenuRamk
		call	PrintSymbol_SP
; ---------------------------------------------------------------------------
		db 0CDh
; ---------------------------------------------------------------------------
		ld	a, 0CEh	; '╬'
		call	sub_E306
		call	sub_E962

loc_E8F9:				; CODE XREF: sub_E87E+A6j
		ld	hl, (word_E1E2)
		inc	hl
		inc	hl
		ld	(word_E1E2), hl
		call	loc_E268
		jr	c, loc_E91D
		call	sub_E364

loc_E909:				; CODE XREF: sub_E87E+AFj
		call	sub_E87E
		ld	hl, sub_E935
		call	sub_E776
		jr	c, loc_E92C
		call	sub_E7CF
; ---------------------------------------------------------------------------

loc_E917:				; CODE XREF: sub_E87E+B2j
		call	sub_E38D
		call	PrnCrd_CRLF

loc_E91D:				; CODE XREF: sub_E87E+86j
		ld	hl, byte_E1DF
		inc	(hl)
		ld	a, (hl)
		cp	11h
		jp	c, loc_E8F9
		call	Pause_24Int
		jr	loc_E932
; ---------------------------------------------------------------------------

loc_E92C:				; CODE XREF: sub_E87E+94j
		dec	a
		jr	z, loc_E909
		dec	a
		jr	z, loc_E917

loc_E932:				; CODE XREF: sub_E87E+ACj
		jp	Cmd_Ent_Inventory
; END OF FUNCTION CHUNK	FOR sub_E87E

; =============== S U B	R O U T	I N E =======================================




; === PrintStars === CODE XREF: sub_728D+264p RAM:753Ep ...
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




; === PrintSymbol_SP === CODE XREF: sub_5FA1+1Ep RAM:6EECp ...
PrintSymbol_SP:				; CODE XREF: sub_5FA1+1Ep RAM:6EECp ...
		pop	hl
		ld	a, (hl)
		inc	hl
		push	hl
; End of function PrintSymbol_SP


; =============== S U B	R O U T	I N E =======================================


PrintMessage1_A:			; CODE XREF: sub_5E1A+13j RAM:8D44p ...

; FUNCTION CHUNK AT D59C SIZE 00000018 BYTES
; FUNCTION CHUNK AT D7E2 SIZE 0000000F BYTES

		cp	0Eh
		jp	c, loc_D7E2
		cp	20h ; ' '
		jp	c, loc_D441
		cp	60h ; '`'
		jp	c, sub_D499
		cp	80h ; 'А'
		jr	c, loc_D443
		cp	0A0h ; 'а'
		jp	c, loc_D59C
		sub	0A0h ; 'а'
		jr	loc_D443
; ---------------------------------------------------------------------------

loc_D441:				; CODE XREF: PrintMessage1_A+7j
		add	a, 72h ; 'r'

loc_D443:				; CODE XREF: PrintMessage1_A+11j
					; PrintMessage1_A+1Aj
		ld	hl, (word_F11A)
		ld	d, a
		inc	d

loc_D448:				; CODE XREF: PrintMessage1_A+2Dj
		dec	d
		jp	z, loc_D454
		ld	bc, 0FFFFh
		xor	a
		cpir
		jr	loc_D448
; ---------------------------------------------------------------------------

loc_D454:				; CODE XREF: PrintMessage1_A+24j
					; PrintMessage1_A+38j
		ld	a, (hl)
		and	a
		ret	z
		push	hl
		call	PrintMessage1_A
		pop	hl
		inc	hl
		jr	loc_D454
; End of function PrintMessage1_A


; =============== S U B	R O U T	I N E =======================================


PrintMessage2:				; CODE XREF: sub_5E1A+19j sub_5FA1+1Bp ...

; FUNCTION CHUNK AT D48E SIZE 0000000B BYTES

		ld	hl, (word_F11C)
		ld	d, a

loc_D463:				; CODE XREF: PrintMessage2+Ej
		dec	d
		jr	z, loc_D48E
		ld	bc, 0FFFFh
		ld	a, 0
		cpir
		jp	loc_D463
; End of function PrintMessage2


; =============== S U B	R O U T	I N E =======================================




; === PrnMsg_A_CRLF === CODE XREF: sub_D755p
PrnMsg_A_CRLF:				; CODE XREF: sub_D755p
					; sub_D822:loc_D848j ...
		call	PrintMessage1_A
; End of function PrnMsg_A_CRLF


; =============== S U B	R O U T	I N E =======================================


PrnCrd_CRLF:				; CODE XREF: RAM:6F0Ej
					; sub_D755:loc_D785j ...
		ld	a, 1
		ld	(byte_D0D2), a
		call	loc_E2B7
; End of function PrnCrd_CRLF


; =============== S U B	R O U T	I N E =======================================




; === PrnTxtOnViewScr === CODE XREF: j_PrnTxtOnViewScrj
PrnTxtOnViewScr:			; CODE XREF: j_PrnTxtOnViewScrj
		xor	a
		ld	(FlgPrnSym1), a
		ld	a, (WhatOnScreen)
		cp	4
		jr	c, PrnNameViewScrInBuf
		ld	hl, loc_D548
		ld	(hl), 0
		call	sub_DAB4	; печать индикатора опасности и	обратного отсчета в менюшках во	время полёта
		ld	hl, loc_D548
		ld	(hl), 0A9h ; 'й'
		ret
; ---------------------------------------------------------------------------

PrnNameViewScrInBuf:			; CODE XREF: PrnTxtOnViewScr+9j
		ld	hl,  loc_D508+2
		set	7, (hl)
		ld	hl, 10Ah
		ld	(CrdXPrnSymbol), hl
		ld	a, (WhatOnScreen)
		add	a, 60h ; '`'
		call	PrintMessage1_A
		ld	a, 0AFh	; 'п'
		call	PrnMsg_A_CRLF
		call	sub_DAB4	; печать индикатора опасности и	обратного отсчета в менюшках во	время полёта
		ld	hl,  loc_D508+2
		res	7, (hl)
		ret
; End of function PrnTxtOnViewScr


; =============== S U B	R O U T	I N E =======================================

; печать индикатора опасности и	обратного отсчета в менюшках во	время полёта



; === SetFirstExtSlot === CODE XREF: MoveAllObjects+74p
SetFirstExtSlot:			; CODE XREF: MoveAllObjects+74p
		ld	(nMissileObj), a
		ld	hl, byte_B457	; 5,+0 =0 - объектом был запущен корабль
					; +1,+2,+3 - печатные координаты вершины с лазером
					; +1,+2,+3 - печатные координаты вершины №3 xor	3
					; +8 биты 0-3,=0 можно увеличить рейтинг
		ld	(word_B452), hl
		ret
; End of function SetFirstExtSlot


; =============== S U B	R O U T	I N E =======================================

; hl - адрес слота родиителя
; Attributes: noreturn



; === _PrintIndDanger === CODE XREF: sub_DA2F+1Fj sub_DAB4+3p	...
_PrintIndDanger:			; CODE XREF: sub_DA2F+1Fj sub_DAB4+3p	...
		jp	PrintIndDanger
; End of function _PrintIndDanger

; ---------------------------------------------------------------------------


; === j_PrintMessage1_A === CODE XREF: RAM:7139j	sub_7447+Fp ...
j_PrintMessage1_A:			; CODE XREF: RAM:7139j	sub_7447+Fp ...
		jp	PrintMessage1_A
; End of function j_PrintMessage1_A

; ---------------------------------------------------------------------------
word_D04B:	dw 0D548h		; DATA XREF: sub_99B5:loc_99F8r
					; sub_99B5+80r	...
		db    0

; =============== S U B	R O U T	I N E =======================================

; Attributes: noreturn thunk

MenuSaveLoad:				; CODE XREF: sub_728D+212p
		jp	MenuSaveLoad2
; End of function MenuSaveLoad

; ---------------------------------------------------------------------------


; === j_PrnTxtOnViewScr === CODE XREF: sub_74C6+Fp
j_PrnTxtOnViewScr:			; CODE XREF: sub_74C6+Fp
					; sub_728D:loc_74F4p ...
		jp	PrnTxtOnViewScr
; End of function j_PrnTxtOnViewScr

; ---------------------------------------------------------------------------


