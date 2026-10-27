; 10_circles_planets - разобрано из kladov.asm
; Количество подпрограмм: 19

; === Cmd_P_InfoPlanet === CODE XREF: RAM:D00Fj
Cmd_P_InfoPlanet:			; CODE XREF: RAM:D00Fj
		call	SearchNearPlanet
		call	PrintRamkMenu
		ld	de, unk_FDF3
		call	SetAttrFrom_DE
		ld	a, 0A3h	; 'г'
		call	sub_E306
		ld	a, 3
		call	loc_E2C8
		call	loc_E107
		call	PrnCrd_CRLF
		call	sub_EEA8
		ld	a, 0A2h	; 'в'
		call	loc_E297
		ld	a, 0B1h	; '▒'
		ld	b, a
		ld	a, (PlanetGovernment)
		add	a, b
		call	sub_EE9D
		ld	a, 0C4h	; '─'
		call	loc_E297
		ld	a, 3
		ld	(byte_D0CE), a
		ld	a, (PlanetTechLevel)
		inc	a
		call	PrintNumber_A
		call	DubleCRLF
		ld	a, 0D7h	; '╫'

loc_EE23:
		call	sub_E292
		ld	a, (PlanetPopulation)
		call	sub_D2DE
		ld	a, 0C6h	; '╞'
		call	sub_EE9D
		call	PrintSymbol_SP
; ---------------------------------------------------------------------------
		db 28h
; ---------------------------------------------------------------------------
		call	sub_EECE
		call	PrintSymbol_SP
; ---------------------------------------------------------------------------
		db 53h
; ---------------------------------------------------------------------------
		ld	a, 29h ; ')'
		call	sub_EE9D
		ld	a, 6Fh ; 'o'
		call	sub_E292
		ld	hl, (PlanetGrossProduct)
		ld	a, 5
		ld	(byte_D0CE), a
		and	a
		call	PrintNumber_HL
		call	PrintSymbol_SP
; ---------------------------------------------------------------------------
		db 20h
; ---------------------------------------------------------------------------
		call	PrintSymbol_SP
; ---------------------------------------------------------------------------
		db 4Dh
; ---------------------------------------------------------------------------
		ld	a, 0E2h	; 'т'
		call	sub_E2F8
		call	DubleCRLF
		ld	a, 0FAh	; '·'
		call	loc_E297
		call	sub_EF21
		call	PrintSymbol_SP
; ---------------------------------------------------------------------------
		db 20h
; ---------------------------------------------------------------------------
		ld	a, 11h
		ld	(byte_E1DF), a
		call	sub_E31E
		call	DubleCRLF
		ld	a, (PilotMission3) ; =#06 при стыковке будет захват таргонами (вроде нереальная	ситуация)
					; =#04 в системе со танцией таргонов во	время войны
		and	a
		jr	z, loc_EE8C
		ld	hl, PlanetData
		call	CompareGalaxyData
		jr	c, loc_EE8C
		ld	a, 57h ; 'W'
		jp	PrintMessage2
; ---------------------------------------------------------------------------

loc_EE8C:				; CODE XREF: RAM:EE7Dj	RAM:EE85j
		call	loc_6ED4	; Печать описания планеты
		ld	(DescrPlanetData), hl
		ld	hl, (PlanetData2)
		ld	(DescrPlanetData+2), hl
		ld	a, 5
		ret
; ---------------------------------------------------------------------------
		db  5Fh	; _
		db 0D4h	; ╘

; =============== S U B	R O U T	I N E =======================================




; === CopyGalaxyToPlanet === CODE XREF: sub_DF24p
CopyGalaxyToPlanet:			; CODE XREF: sub_DF24p
					; SearchPlanetNearCursorp ...
		ld	hl, PilotGalaxyData0
		ld	de, DataOfPlanet
		jr	loc_E13C
; End of function CopyGalaxyToPlanet


; =============== S U B	R O U T	I N E =======================================


SavePlanetData:				; CODE XREF: CalcDistanceToCursor+19p
					; sub_DF24:loc_DF2Bp ...
		ld	hl, DataOfPlanet
		ld	de, DataOfPlanetSv
		jr	loc_E13C
; End of function SavePlanetData


; =============== S U B	R O U T	I N E =======================================


RestorePlanetData:			; CODE XREF: sub_DF24:loc_DF4Cp
					; SearchPlanetNearCursor+35p
		ld	hl, DataOfPlanetSv
		ld	de, DataOfPlanet

loc_E13C:				; CODE XREF: CopyGalaxyToPlanet+6j
					; SavePlanetData+6j
		ld	bc, 6
		ldir
		ret
; End of function RestorePlanetData


; =============== S U B	R O U T	I N E =======================================


GenerateProductPrice:			; CODE XREF: sub_DCE9+26p
		call	sub_ABAF
		ld	(PilotRnd), a
; End of function GenerateProductPrice


; =============== S U B	R O U T	I N E =======================================


GenerateAnotherPrice:			; CODE XREF: InitGame+23p
		call	sub_9B83
		ld	de, ProductPrice
		exx
		ld	hl, TableBasePrice ; +#28 Fireams
		ld	b, 11h

loc_E154:				; CODE XREF: GenerateAnotherPrice+6Cj
		push	bc
		ld	e, (hl)
		inc	hl
		ld	a, (hl)
		ld	d, a
		push	de
		and	1Fh
		jr	z, loc_E167
		ld	b, a
		ld	a, (PlanetEconomy)
		ld	c, a
		xor	a

loc_E164:				; CODE XREF: GenerateAnotherPrice+1Dj
		add	a, c
		djnz	loc_E164

loc_E167:				; CODE XREF: GenerateAnotherPrice+14j
		ld	e, a
		inc	hl
		ld	b, (hl)
		inc	hl
		ld	c, (hl)
		inc	hl
		ld	a, (PilotRnd)
		and	c
		add	a, b
		ld	b, a
		ld	a, e
		bit	7, d
		jr	nz, loc_E17B
		inc	a
		neg

loc_E17B:				; CODE XREF: GenerateAnotherPrice+2Ej
		add	a, b
		bit	7, a
		jr	z, loc_E181
		xor	a

loc_E181:				; CODE XREF: GenerateAnotherPrice+36j
		and	3Fh ; '?'
		ld	b, e
		pop	de
		push	af
		ld	a, (PilotRnd)
		and	c
		add	a, e
		ld	e, 0
		rl	e
		ld	c, a
		ld	a, b
		bit	7, d
		jr	z, loc_E19A
		ld	d, a
		ld	a, c
		sub	d
		jr	loc_E19E
; ---------------------------------------------------------------------------

loc_E19A:				; CODE XREF: GenerateAnotherPrice+4Bj
		add	a, c
		jr	nc, loc_E19E
		inc	e

loc_E19E:				; CODE XREF: GenerateAnotherPrice+50j
					; GenerateAnotherPrice+53j
		ld	b, e
		add	a, a
		rl	b
		add	a, a
		rl	b
		ld	c, a
		push	bc
		exx
		pop	bc
		ld	a, c
		ld	(de), a
		inc	de
		ld	a, b
		ld	(de), a
		inc	de
		pop	af

Amount:					; DATA XREF: InitGame+1Do
		ld	(hl), a
		inc	hl
		exx
		pop	bc
		djnz	loc_E154
		ld	hl, planetAlienItems
		ld	(hl), b
		ret
; End of function GenerateAnotherPrice

; ---------------------------------------------------------------------------
byte_E1BB:	db 0			; DATA XREF: sub_E962+6o sub_EF48+25o
		db 0


; === CopyPlanetName === CODE XREF: sub_DF24+34p
CopyPlanetName:				; CODE XREF: sub_DF24+34p
					; SearchPlanetNearCursor+3Ep
		ld	bc, 8
		ld	hl, TempName	; "LAVE"
		ld	de, PlanetName
		ldir
		ret
; End of function CopyPlanetName


; =============== S U B	R O U T	I N E =======================================


GenerateInfoPlanet:			; CODE XREF: sub_DF24+2Ep
					; SearchPlanetNearCursor+38p
		ld	hl, (DataOfPlanet)
		ld	(PlanetData), hl
		ld	a, h
		ld	(Pilot_CrdYTarget), a
		and	7
		ld	(PlanetEconomy), a
		ld	hl, (DataOfPlanet1)
		ld	(PlanetData1), hl
		ld	a, h
		ld	(Pilot_CrdXTarget), a
		ld	a, l
		srl	a
		srl	a
		srl	a
		and	7
		ld	(PlanetGovernment), a
		srl	a
		jr	nz, loc_E0BD
		ld	a, (PlanetEconomy)
		or	2
		ld	(PlanetEconomy), a

loc_E0BD:				; CODE XREF: GenerateInfoPlanet+27j
		ld	a, (PlanetEconomy)
		xor	7
		and	a
		ld	b, a
		ld	a, h
		and	3
		add	a, b
		ld	b, a
		ld	a, (PlanetGovernment)
		srl	a
		adc	a, b
		ld	(PlanetTechLevel), a
		add	a, a
		add	a, a
		ld	b, a
		ld	a, (PlanetGovernment)
		ld	c, a
		ld	a, (PlanetEconomy)
		adc	a, b
		adc	a, c
		adc	a, 1
		ld	(PlanetPopulation), a
		ld	a, (PlanetEconomy)
		xor	7
		adc	a, 3
		ld	d, a
		ld	a, (PlanetGovernment)
		adc	a, 4
		call	Mult_HL_D_A
		ld	a, (PlanetPopulation)
		ld	d, l
		call	Mult_HL_D_A
		add	hl, hl
		add	hl, hl
		add	hl, hl
		ld	(PlanetGrossProduct), hl
		ld	hl, (DataOfPlanet2)
		ld	(PlanetData2), hl
		ret
; End of function GenerateInfoPlanet

; ---------------------------------------------------------------------------

loc_E107:				; CODE XREF: RAM:E469p	RAM:EDF5p
		ld	hl, (FuelToPlanet)
		ld	a, h
		or	l
		jr	z, loc_E123
		ld	a, 0BFh	; '┐'
		call	loc_E297
		ld	a, 4
		ld	(byte_D0CE), a
		ld	hl, (FuelToPlanet)
		scf
		call	PrintNumber_HL
		call	PrintSymbol_SP
; ---------------------------------------------------------------------------
		db 0C3h
; ---------------------------------------------------------------------------

loc_E123:				; CODE XREF: RAM:E10Cj
		jp	PrnCrd_CRLF

; =============== S U B	R O U T	I N E =======================================


CopyGalaxyToPlanet:			; CODE XREF: sub_DF24p
					; SearchPlanetNearCursorp ...
		ld	hl, PilotGalaxyData0
		ld	de, DataOfPlanet
		jr	loc_E13C
; End of function CopyGalaxyToPlanet


; =============== S U B	R O U T	I N E =======================================


SavePlanetData:				; CODE XREF: CalcDistanceToCursor+19p
					; sub_DF24:loc_DF2Bp ...
		ld	hl, DataOfPlanet
		ld	de, DataOfPlanetSv
		jr	loc_E13C
; End of function SavePlanetData


; =============== S U B	R O U T	I N E =======================================


RestorePlanetData:			; CODE XREF: sub_DF24:loc_DF4Cp
					; SearchPlanetNearCursor+35p
		ld	hl, DataOfPlanetSv
		ld	de, DataOfPlanet

loc_E13C:				; CODE XREF: CopyGalaxyToPlanet+6j
					; SavePlanetData+6j
		ld	bc, 6
		ldir
		ret
; End of function RestorePlanetData


; =============== S U B	R O U T	I N E =======================================


GenerateProductPrice:			; CODE XREF: sub_DCE9+26p
		call	sub_ABAF
		ld	(PilotRnd), a
; End of function GenerateProductPrice


; =============== S U B	R O U T	I N E =======================================


GenerateAnotherPrice:			; CODE XREF: InitGame+23p
		call	sub_9B83
		ld	de, ProductPrice
		exx
		ld	hl, TableBasePrice ; +#28 Fireams
		ld	b, 11h

loc_E154:				; CODE XREF: GenerateAnotherPrice+6Cj
		push	bc
		ld	e, (hl)
		inc	hl
		ld	a, (hl)
		ld	d, a
		push	de
		and	1Fh
		jr	z, loc_E167
		ld	b, a
		ld	a, (PlanetEconomy)
		ld	c, a
		xor	a

loc_E164:				; CODE XREF: GenerateAnotherPrice+1Dj
		add	a, c
		djnz	loc_E164

loc_E167:				; CODE XREF: GenerateAnotherPrice+14j
		ld	e, a
		inc	hl
		ld	b, (hl)
		inc	hl
		ld	c, (hl)
		inc	hl
		ld	a, (PilotRnd)
		and	c
		add	a, b
		ld	b, a
		ld	a, e
		bit	7, d
		jr	nz, loc_E17B
		inc	a
		neg

loc_E17B:				; CODE XREF: GenerateAnotherPrice+2Ej
		add	a, b
		bit	7, a
		jr	z, loc_E181
		xor	a

loc_E181:				; CODE XREF: GenerateAnotherPrice+36j
		and	3Fh ; '?'
		ld	b, e
		pop	de
		push	af
		ld	a, (PilotRnd)
		and	c
		add	a, e
		ld	e, 0
		rl	e
		ld	c, a
		ld	a, b
		bit	7, d
		jr	z, loc_E19A
		ld	d, a
		ld	a, c
		sub	d
		jr	loc_E19E
; ---------------------------------------------------------------------------

loc_E19A:				; CODE XREF: GenerateAnotherPrice+4Bj
		add	a, c
		jr	nc, loc_E19E
		inc	e

loc_E19E:				; CODE XREF: GenerateAnotherPrice+50j
					; GenerateAnotherPrice+53j
		ld	b, e
		add	a, a
		rl	b
		add	a, a
		rl	b
		ld	c, a
		push	bc
		exx
		pop	bc
		ld	a, c
		ld	(de), a
		inc	de
		ld	a, b
		ld	(de), a
		inc	de
		pop	af

Amount:					; DATA XREF: InitGame+1Do
		ld	(hl), a
		inc	hl
		exx
		pop	bc
		djnz	loc_E154
		ld	hl, planetAlienItems
		ld	(hl), b
		ret
; End of function GenerateAnotherPrice

; ---------------------------------------------------------------------------
byte_E1BB:	db 0			; DATA XREF: sub_E962+6o sub_EF48+25o
		db 0


; === CorrData4Nova === CODE XREF: sub_74C6:loc_74D2p
CorrData4Nova:				; CODE XREF: sub_74C6:loc_74D2p
					; PrnSunPlanet+13p
		ld	a, (PilotMission1_1)
		and	a
		ret	z
		ld	a, (SunRadiusNova)
		and	a
		jr	nz, loc_75CE
		ld	a, (CircleRadius)

loc_75CE:				; CODE XREF: CorrData4Nova+9j
		add	a, 2
		jr	z, loc_75D4
		jr	nc, loc_75DA

loc_75D4:				; CODE XREF: CorrData4Nova+10j
		ld	a, 1
		ld	(PlayerGameOver), a
		xor	a

loc_75DA:				; CODE XREF: CorrData4Nova+12j
		ld	(CircleRadius),	a
		ld	(SunRadiusNova), a
		ret
; End of function CorrData4Nova

; ---------------------------------------------------------------------------

loc_75E1:				; CODE XREF: CalcPrnCrdSunp
		exx
		ld	bc, 3
		exx
		ld	a, (WhatOnScreen)
		and	a
		jp	nz, loc_7695
		ld	a, (CircleCrdZhi)
		bit	7, a
		jp	nz, loc_782A
		and	7Fh ; ''
		ld	(CircleCrdZhi),	a
		ld	hl, (CircleCrdZmed)
		ex	de, hl
		ld	hl, (CircleCrdXmed)
		ld	a, (CircleCrdZlow)
		ld	b, a
		ld	a, (CircleCrdXlow)
		ld	c, a
		ld	a, h
		bit	7, a
		jr	nz, loc_7612
		exx
		set	0, b
		exx

loc_7612:				; CODE XREF: RAM:760Cj	RAM:loc_76C0j
		and	7Fh ; ''
		ld	h, a
		call	sub_77A0
		ld	a, d
		ld	(CircleCrdY+1),	a
		ld	hl, (CircleCrdZmed)
		ex	de, hl
		ld	hl, (CircleCrdYmed)
		ld	a, (CircleCrdZlow)
		ld	b, a
		ld	a, (CircleCrdYlow)
		ld	c, a
		ld	a, h
		bit	7, a
		jr	nz, loc_7634
		exx
		set	1, b
		exx

loc_7634:				; CODE XREF: RAM:762Ej
		and	7Fh ; ''
		ld	h, a
		call	sub_77A0
		ld	a, d
		ld	(CircleCrdY), a
		exx
		ld	a, b
		exx
		ld	(CircleFlg), a
		ld	a, (CircleCrdZhi)
		and	a
		jr	nz, loc_7659
		ld	hl, 0A000h
		ld	de, (CircleCrdZlow)
		sbc	hl, de
		jr	c, loc_7659
		ld	a, 0FFh
		jr	loc_7692
; ---------------------------------------------------------------------------

loc_7659:				; CODE XREF: RAM:7648j	RAM:7653j
		ld	hl, 0A000h
		ld	de, (CircleCrdZmed)
		and	a
		sbc	hl, de
		jp	c, loc_782A
		ld	a, (CircleCrdZhi)
		and	a
		jr	z, loc_7688
		ld	b, a
		ld	a, (CircleCrdZmed)
		ld	c, a
		xor	a
		ld	e, 0A0h	; 'а'

loc_7674:				; CODE XREF: RAM:767Dj
		sla	c
		rl	b
		jr	c, loc_767F
		sla	e
		rla
		jr	loc_7674
; ---------------------------------------------------------------------------

loc_767F:				; CODE XREF: RAM:7678j
		scf
		rr	b
		call	loc_783D
		ld	a, d
		jr	loc_7692
; ---------------------------------------------------------------------------

loc_7688:				; CODE XREF: RAM:766Aj
		ld	a, (CircleCrdZmed)
		ld	b, a
		ld	a, 0A0h	; 'а'
		call	loc_783B
		ld	a, d

loc_7692:				; CODE XREF: RAM:7657j	RAM:7686j
		jp	loc_7793
; ---------------------------------------------------------------------------

loc_7695:				; CODE XREF: RAM:75EAj
		dec	a
		jr	nz, loc_76C3
		ld	a, (CircleCrdZhi)
		bit	7, a
		jp	z, loc_782A
		and	7Fh ; ''
		ld	(CircleCrdZhi),	a
		ld	hl, (CircleCrdZmed)
		ex	de, hl
		ld	hl, (CircleCrdXmed)
		ld	a, (CircleCrdZlow)
		ld	b, a
		ld	a, (CircleCrdXlow)
		ld	c, a
		ld	a, h
		bit	7, a
		jr	z, loc_76C0
		and	7Fh ; ''
		ld	h, a
		exx
		set	0, b
		exx

loc_76C0:				; CODE XREF: RAM:76B7j
		jp	loc_7612
; ---------------------------------------------------------------------------

loc_76C3:				; CODE XREF: RAM:7696j
		dec	a
		jp	nz, loc_7770
		ld	a, (CircleCrdXhi)
		bit	7, a
		jp	z, loc_782A
		and	7Fh ; ''
		ld	(CircleCrdXhi),	a
		ld	hl, (CircleCrdXmed)
		ex	de, hl
		ld	hl, (CircleCrdZmed)
		ld	a, (CircleCrdXlow)
		ld	b, a
		ld	a, (CircleCrdZlow)
		ld	c, a
		ld	a, h
		bit	7, a
		jr	nz, loc_76EC
		exx
		set	0, b
		exx

loc_76EC:				; CODE XREF: RAM:76E6j	RAM:loc_7790j
		and	7Fh ; ''
		ld	h, a
		call	sub_77A0
		ld	a, d
		ld	(CircleCrdY+1),	a
		ld	hl, (CircleCrdXmed)
		ex	de, hl
		ld	hl, (CircleCrdYmed)
		ld	a, (CircleCrdXlow)
		ld	b, a
		ld	a, (CircleCrdYlow)
		ld	c, a
		ld	a, h
		bit	7, a
		jr	nz, loc_770E
		exx
		set	1, b
		exx

loc_770E:				; CODE XREF: RAM:7708j
		and	7Fh ; ''
		ld	h, a
		call	sub_77A0
		ld	a, d
		ld	(CircleCrdY), a
		exx
		ld	a, b
		exx
		ld	(CircleFlg), a
		ld	a, (CircleCrdXhi)
		and	a
		jr	nz, loc_7733
		ld	hl, 0A000h
		ld	de, (CircleCrdXlow)
		sbc	hl, de
		jr	c, loc_7733
		ld	a, 0FFh
		jr	loc_776D
; ---------------------------------------------------------------------------

loc_7733:				; CODE XREF: RAM:7722j	RAM:772Dj
		ld	hl, 0A000h
		ld	de, (CircleCrdXmed)
		and	a
		sbc	hl, de
		jp	c, loc_782A
		ld	a, (CircleCrdXhi)
		and	a
		jr	z, loc_7763
		ld	b, a
		ld	a, (CircleCrdXmed)
		ld	c, a
		xor	a
		ld	e, 0A0h	; 'а'

loc_774E:				; CODE XREF: RAM:7757j
		sla	c
		rl	b
		jr	c, loc_775A
		sla	e
		rla
		jp	loc_774E
; ---------------------------------------------------------------------------

loc_775A:				; CODE XREF: RAM:7752j
		scf
		rr	b
		call	loc_783D
		ld	a, d
		jr	loc_776D
; ---------------------------------------------------------------------------

loc_7763:				; CODE XREF: RAM:7744j
		ld	a, (CircleCrdXmed)
		ld	b, a
		ld	a, 0A0h	; 'а'
		call	loc_783B
		ld	a, d

loc_776D:				; CODE XREF: RAM:7731j	RAM:7761j
		jp	loc_7793
; ---------------------------------------------------------------------------

loc_7770:				; CODE XREF: RAM:76C4j
		ld	a, (CircleCrdXhi)
		bit	7, a
		jp	nz, loc_782A
		ld	hl, (CircleCrdXmed)
		ex	de, hl
		ld	hl, (CircleCrdZmed)
		ld	a, (CircleCrdXlow)
		ld	b, a
		ld	a, (CircleCrdZlow)
		ld	c, a
		ld	a, h
		bit	7, a
		jr	z, loc_7790
		exx
		set	0, b
		exx

loc_7790:				; CODE XREF: RAM:778Aj
		jp	loc_76EC
; ---------------------------------------------------------------------------

loc_7793:				; CODE XREF: RAM:loc_7692j
					; RAM:loc_776Dj
		ld	(CircleRadius),	a
		ld	a, 1
		ret
; ---------------------------------------------------------------------------
; START	OF FUNCTION CHUNK FOR sub_77A0

loc_7799:				; CODE XREF: sub_77A0+10j
		ld	h, l
		ld	l, c
		ld	d, e
		ld	e, b
		jp	loc_77C4
; END OF FUNCTION CHUNK	FOR sub_77A0

; =============== S U B	R O U T	I N E =======================================




; === GenerateInfoPlanet === CODE XREF: sub_DF24+2Ep
GenerateInfoPlanet:			; CODE XREF: sub_DF24+2Ep
					; SearchPlanetNearCursor+38p
		ld	hl, (DataOfPlanet)
		ld	(PlanetData), hl
		ld	a, h
		ld	(Pilot_CrdYTarget), a
		and	7
		ld	(PlanetEconomy), a
		ld	hl, (DataOfPlanet1)
		ld	(PlanetData1), hl
		ld	a, h
		ld	(Pilot_CrdXTarget), a
		ld	a, l
		srl	a
		srl	a
		srl	a
		and	7
		ld	(PlanetGovernment), a
		srl	a
		jr	nz, loc_E0BD
		ld	a, (PlanetEconomy)
		or	2
		ld	(PlanetEconomy), a

loc_E0BD:				; CODE XREF: GenerateInfoPlanet+27j
		ld	a, (PlanetEconomy)
		xor	7
		and	a
		ld	b, a
		ld	a, h
		and	3
		add	a, b
		ld	b, a
		ld	a, (PlanetGovernment)
		srl	a
		adc	a, b
		ld	(PlanetTechLevel), a
		add	a, a
		add	a, a
		ld	b, a
		ld	a, (PlanetGovernment)
		ld	c, a
		ld	a, (PlanetEconomy)
		adc	a, b
		adc	a, c
		adc	a, 1
		ld	(PlanetPopulation), a
		ld	a, (PlanetEconomy)
		xor	7
		adc	a, 3
		ld	d, a
		ld	a, (PlanetGovernment)
		adc	a, 4
		call	Mult_HL_D_A
		ld	a, (PlanetPopulation)
		ld	d, l
		call	Mult_HL_D_A
		add	hl, hl
		add	hl, hl
		add	hl, hl
		ld	(PlanetGrossProduct), hl
		ld	hl, (DataOfPlanet2)
		ld	(PlanetData2), hl
		ret
; End of function GenerateInfoPlanet

; ---------------------------------------------------------------------------

loc_E107:				; CODE XREF: RAM:E469p	RAM:EDF5p
		ld	hl, (FuelToPlanet)
		ld	a, h
		or	l
		jr	z, loc_E123
		ld	a, 0BFh	; '┐'
		call	loc_E297
		ld	a, 4
		ld	(byte_D0CE), a
		ld	hl, (FuelToPlanet)
		scf
		call	PrintNumber_HL
		call	PrintSymbol_SP
; ---------------------------------------------------------------------------
		db 0C3h
; ---------------------------------------------------------------------------

loc_E123:				; CODE XREF: RAM:E10Cj
		jp	PrnCrd_CRLF

; =============== S U B	R O U T	I N E =======================================


CopyGalaxyToPlanet:			; CODE XREF: sub_DF24p
					; SearchPlanetNearCursorp ...
		ld	hl, PilotGalaxyData0
		ld	de, DataOfPlanet
		jr	loc_E13C
; End of function CopyGalaxyToPlanet


; =============== S U B	R O U T	I N E =======================================


SavePlanetData:				; CODE XREF: CalcDistanceToCursor+19p
					; sub_DF24:loc_DF2Bp ...
		ld	hl, DataOfPlanet
		ld	de, DataOfPlanetSv
		jr	loc_E13C
; End of function SavePlanetData


; =============== S U B	R O U T	I N E =======================================


RestorePlanetData:			; CODE XREF: sub_DF24:loc_DF4Cp
					; SearchPlanetNearCursor+35p
		ld	hl, DataOfPlanetSv
		ld	de, DataOfPlanet

loc_E13C:				; CODE XREF: CopyGalaxyToPlanet+6j
					; SavePlanetData+6j
		ld	bc, 6
		ldir
		ret
; End of function RestorePlanetData


; =============== S U B	R O U T	I N E =======================================


GenerateProductPrice:			; CODE XREF: sub_DCE9+26p
		call	sub_ABAF
		ld	(PilotRnd), a
; End of function GenerateProductPrice


; =============== S U B	R O U T	I N E =======================================


GenerateAnotherPrice:			; CODE XREF: InitGame+23p
		call	sub_9B83
		ld	de, ProductPrice
		exx
		ld	hl, TableBasePrice ; +#28 Fireams
		ld	b, 11h

loc_E154:				; CODE XREF: GenerateAnotherPrice+6Cj
		push	bc
		ld	e, (hl)
		inc	hl
		ld	a, (hl)
		ld	d, a
		push	de
		and	1Fh
		jr	z, loc_E167
		ld	b, a
		ld	a, (PlanetEconomy)
		ld	c, a
		xor	a

loc_E164:				; CODE XREF: GenerateAnotherPrice+1Dj
		add	a, c
		djnz	loc_E164

loc_E167:				; CODE XREF: GenerateAnotherPrice+14j
		ld	e, a
		inc	hl
		ld	b, (hl)
		inc	hl
		ld	c, (hl)
		inc	hl
		ld	a, (PilotRnd)
		and	c
		add	a, b
		ld	b, a
		ld	a, e
		bit	7, d
		jr	nz, loc_E17B
		inc	a
		neg

loc_E17B:				; CODE XREF: GenerateAnotherPrice+2Ej
		add	a, b
		bit	7, a
		jr	z, loc_E181
		xor	a

loc_E181:				; CODE XREF: GenerateAnotherPrice+36j
		and	3Fh ; '?'
		ld	b, e
		pop	de
		push	af
		ld	a, (PilotRnd)
		and	c
		add	a, e
		ld	e, 0
		rl	e
		ld	c, a
		ld	a, b
		bit	7, d
		jr	z, loc_E19A
		ld	d, a
		ld	a, c
		sub	d
		jr	loc_E19E
; ---------------------------------------------------------------------------

loc_E19A:				; CODE XREF: GenerateAnotherPrice+4Bj
		add	a, c
		jr	nc, loc_E19E
		inc	e

loc_E19E:				; CODE XREF: GenerateAnotherPrice+50j
					; GenerateAnotherPrice+53j
		ld	b, e
		add	a, a
		rl	b
		add	a, a
		rl	b
		ld	c, a
		push	bc
		exx
		pop	bc
		ld	a, c
		ld	(de), a
		inc	de
		ld	a, b
		ld	(de), a
		inc	de
		pop	af

Amount:					; DATA XREF: InitGame+1Do
		ld	(hl), a
		inc	hl
		exx
		pop	bc
		djnz	loc_E154
		ld	hl, planetAlienItems
		ld	(hl), b
		ret
; End of function GenerateAnotherPrice

; ---------------------------------------------------------------------------
byte_E1BB:	db 0			; DATA XREF: sub_E962+6o sub_EF48+25o
		db 0


; === GeneratePlanetName === CODE XREF: sub_D857+Bp sub_DF24+Ap ...
GeneratePlanetName:			; CODE XREF: sub_D857+Bp sub_DF24+Ap ...
		ld	hl, TempName	; "LAVE"

loc_E01F:
		ld	(word_DF01), hl
		ld	b, 8

loc_E024:				; CODE XREF: GeneratePlanetName+Bj
		ld	(hl), 0
		inc	hl
		djnz	loc_E024
		ld	hl, (DataOfPlanet)
		ld	a, l
		and	40h ; '@'
		and	a
		push	af
		call	sub_E03D
		call	sub_E03D
		call	sub_E03D
		pop	af
		jr	z, ModifyPlanetData
; End of function GeneratePlanetName


; =============== S U B	R O U T	I N E =======================================




; === ModifyPlanetData === CODE XREF: NextPlanetData+3p
ModifyPlanetData:			; CODE XREF: NextPlanetData+3p
					; GeneratePlanetName+1Fj ...
		ld	hl, (DataOfPlanet)
		ld	de, (DataOfPlanet1)
		add	hl, de
		ex	de, hl
		ld	(DataOfPlanet),	hl
		ld	hl, (DataOfPlanet2)
		ld	(DataOfPlanet1), hl
		add	hl, de
		ld	(DataOfPlanet2), hl
		ret
; End of function ModifyPlanetData

; ---------------------------------------------------------------------------


; === NextPlanetData === CODE XREF: SearchPlanetNearCursor:loc_DF9Cp
NextPlanetData:				; CODE XREF: SearchPlanetNearCursor:loc_DF9Cp
					; Cmd_I_GlobalMap+6Bp ...
		ld	b, 4

loc_DFC6:				; CODE XREF: NextPlanetData+7j
		push	bc
		call	ModifyPlanetData
		pop	bc
		djnz	loc_DFC6
		ret
; End of function NextPlanetData


; =============== S U B	R O U T	I N E =======================================


CalcDistanceToCursor2:			; CODE XREF: CalcDistanceToCursor+Dp
					; SearchPlanetNearCursor+48p
		push	af
		ld	a, (Pilot_CrdXTarget)
		call	ABS_A_sub_B
		ld	d, a
		call	Mult_HL_D_A
		pop	bc
		push	hl
		ld	a, (Pilot_CrdYTarget)

loc_DFDE:				; CODE XREF: sub_E6C6+Dj
		call	ABS_A_sub_B
		srl	a
		ld	d, a
		call	Mult_HL_D_A
		pop	de
		add	hl, de
		ret
; End of function CalcDistanceToCursor2


; =============== S U B	R O U T	I N E =======================================


ABS_A_sub_B:				; CODE XREF: CalcDistanceToCursor2+4p
					; CalcDistanceToCursor2:loc_DFDEp ...
		sub	b
		ret	nc
		cpl
		ret
; End of function ABS_A_sub_B


; =============== S U B	R O U T	I N E =======================================




; === PrnCircle_Clip === CODE XREF: PrnCircle_Fill:PrnCircle_NoFill_NoRadiusp
PrnCircle_Clip:				; CODE XREF: PrnCircle_Fill:PrnCircle_NoFill_NoRadiusp

; FUNCTION CHUNK AT 87C8 SIZE 0000000C BYTES
; FUNCTION CHUNK AT 87F4 SIZE 00000191 BYTES
; FUNCTION CHUNK AT 8FB8 SIZE 00000022 BYTES

		ld	bc, (CircleCrdY)
		ld	(word_861E), bc
		ld	de, (CircleFlg)

loc_86FA:				; DATA XREF: RAM:off_8ECDo
		ld	a, 40h ; '@'
		bit	1, e
		jr	nz, loc_8702
		add	a, c
		add	a, c

loc_8702:				; CODE XREF: PrnCircle_Clip+10j
		sub	c
		ld	(word_8620), a
		ld	a, 80h ; 'А'
		bit	0, e
		jr	z, loc_870E
		add	a, b
		add	a, b

loc_870E:				; CODE XREF: PrnCircle_Clip+1Cj
		sub	b
		ld	(word_8620+1), a
		ld	a, c

loc_8713:				; DATA XREF: RAM:8ECFo
		cp	40h ; '@'
		bit	3, e
		ex	af, af'
		ld	a, b

loc_8719:				; DATA XREF: RAM:8ED1o
		cp	78h ; 'x'
		bit	2, e
		jr	z, loc_872B
		jr	nc, loc_872C
		ex	af, af'
		jr	z, loc_8728
		jr	nc, loc_875E
		jr	loc_8736
; ---------------------------------------------------------------------------

loc_8728:				; CODE XREF: PrnCircle_Clip+34j
		ret	nc
		jr	loc_875E
; ---------------------------------------------------------------------------

loc_872B:				; CODE XREF: PrnCircle_Clip+2Fj
		ret	nc

loc_872C:				; CODE XREF: PrnCircle_Clip+31j
		ex	af, af'
		jr	z, loc_8733
		jr	nc, loc_878E
		jr	loc_874B
; ---------------------------------------------------------------------------

loc_8733:				; CODE XREF: PrnCircle_Clip+3Fj
		ret	nc
		jr	loc_878E
; ---------------------------------------------------------------------------

loc_8736:				; CODE XREF: PrnCircle_Clip+38j
		ld	a, b
		add	a, d
		jr	c, loc_873F

loc_873A:				; DATA XREF: RAM:8ED3o
		cp	78h ; 'x'
		jp	c, loc_87C8

loc_873F:				; CODE XREF: PrnCircle_Clip+4Aj
		ld	a, d
		sub	b
		jr	c, loc_8745

loc_8743:				; DATA XREF: RAM:8ED5o
		cp	78h ; 'x'

loc_8745:				; CODE XREF: PrnCircle_Clip+53j
		jp	c, loc_8898
		jp	loc_87FE
; ---------------------------------------------------------------------------

loc_874B:				; CODE XREF: PrnCircle_Clip+43j
		ld	a, b
		sub	78h ; 'x'
		ld	b, a
		ld	(word_861E+1), a
		ld	a, d
		sub	b
		ret	c
		ret	z
		sub	0F0h ; 'Ё'
		jp	nc, loc_87F4
		jp	loc_888E
; ---------------------------------------------------------------------------

loc_875E:				; CODE XREF: PrnCircle_Clip+36j
					; PrnCircle_Clip+3Bj
		ld	a, c
		sub	40h ; '@'
		ld	c, a
		ld	(word_861E), a
		ld	a, d
		sub	c
		ret	c
		ld	a, d
		call	Sqr_DE_A
		ld	h, d
		ld	l, e
		ld	a, c
		call	Sqr_DE_A
		sbc	hl, de
		ld	a, 78h ; 'x'
		sub	b
		call	Sqr_DE_A
		sbc	hl, de
		jp	c, loc_893F
		add	hl, de
		ld	a, 78h ; 'x'
		add	a, b
		call	Sqr_DE_A
		sbc	hl, de
		jp	nc, loc_884A
		jp	loc_88C2
; ---------------------------------------------------------------------------

loc_878E:				; CODE XREF: PrnCircle_Clip+41j
					; PrnCircle_Clip+46j
		ld	a, b
		sub	78h ; 'x'
		ld	b, a
		ld	a, c
		sub	40h ; '@'
		ld	c, a
		ld	(word_861E), bc
		ld	a, d
		call	Sqr_DE_A
		ld	h, d
		ld	l, e
		ld	a, c
		call	Sqr_DE_A
		sbc	hl, de
		ret	c
		ld	a, b
		call	Sqr_DE_A
		sbc	hl, de
		ret	c
		add	hl, de
		ld	a, b
		add	a, 0F8h	; '°'
		jr	c, loc_87BC
		call	Sqr_DE_A
		sbc	hl, de
		jp	nc, loc_8840

loc_87BC:				; CODE XREF: PrnCircle_Clip+C4j
		jp	loc_88B8
; End of function PrnCircle_Clip


; =============== S U B	R O U T	I N E =======================================


Sqr_DE_A:				; CODE XREF: CalcLenVectorCrd+29p
					; CalcLenVectorCrd+30p	...
		push	hl
		ld	h, 0BEh	; '╛'
		ld	l, a
		ld	e, (hl)
		inc	h
		ld	d, (hl)
		pop	hl
		ret
; End of function Sqr_DE_A

; ---------------------------------------------------------------------------
; START	OF FUNCTION CHUNK FOR PrnCircle_Clip

loc_87C8:				; CODE XREF: PrnCircle_Clip+4Ej
		call	sub_87D4
		call	sub_86A1
		call	sub_87D4
		jp	sub_8689
; END OF FUNCTION CHUNK	FOR PrnCircle_Clip

; =============== S U B	R O U T	I N E =======================================




; === PrnCircle_Fill === CODE XREF: PrnSunPlanet+1Ap
PrnCircle_Fill:				; CODE XREF: PrnSunPlanet+1Ap
					; sub_F8D7+3p
		call	PrnSmallCircle_Fill ; ld a,#A0
		cp	9
		ret	c
		di
		xor	a
		ld	(FillCircle), a
		jp	PrnCircle_NoFill_NoRadius
; ---------------------------------------------------------------------------

PrnCircle_NoFill:			; CODE XREF: PrnSunPlanet+39j
					; sub_7E02+8Bp	...
		call	PrnSmallCircle_NoFill
		cp	9
		ret	c

loc_86E4:				; CODE XREF: sub_9296+79p
		ld	a, 1
		ld	(FillCircle), a

PrnCircle_NoFill_NoRadius:		; CODE XREF: PrnCircle_Fill+Bj
		call	PrnCircle_Clip
		ei
		ret
; End of function PrnCircle_Fill


; =============== S U B	R O U T	I N E =======================================


PrnCircle_Clip:				; CODE XREF: PrnCircle_Fill:PrnCircle_NoFill_NoRadiusp

; FUNCTION CHUNK AT 87C8 SIZE 0000000C BYTES
; FUNCTION CHUNK AT 87F4 SIZE 00000191 BYTES
; FUNCTION CHUNK AT 8FB8 SIZE 00000022 BYTES

		ld	bc, (CircleCrdY)
		ld	(word_861E), bc
		ld	de, (CircleFlg)

loc_86FA:				; DATA XREF: RAM:off_8ECDo
		ld	a, 40h ; '@'
		bit	1, e
		jr	nz, loc_8702
		add	a, c
		add	a, c

loc_8702:				; CODE XREF: PrnCircle_Clip+10j
		sub	c
		ld	(word_8620), a
		ld	a, 80h ; 'А'
		bit	0, e
		jr	z, loc_870E
		add	a, b
		add	a, b

loc_870E:				; CODE XREF: PrnCircle_Clip+1Cj
		sub	b
		ld	(word_8620+1), a
		ld	a, c

loc_8713:				; DATA XREF: RAM:8ECFo
		cp	40h ; '@'
		bit	3, e
		ex	af, af'
		ld	a, b

loc_8719:				; DATA XREF: RAM:8ED1o
		cp	78h ; 'x'
		bit	2, e
		jr	z, loc_872B
		jr	nc, loc_872C
		ex	af, af'
		jr	z, loc_8728
		jr	nc, loc_875E
		jr	loc_8736
; ---------------------------------------------------------------------------

loc_8728:				; CODE XREF: PrnCircle_Clip+34j
		ret	nc
		jr	loc_875E
; ---------------------------------------------------------------------------

loc_872B:				; CODE XREF: PrnCircle_Clip+2Fj
		ret	nc

loc_872C:				; CODE XREF: PrnCircle_Clip+31j
		ex	af, af'
		jr	z, loc_8733
		jr	nc, loc_878E
		jr	loc_874B
; ---------------------------------------------------------------------------

loc_8733:				; CODE XREF: PrnCircle_Clip+3Fj
		ret	nc
		jr	loc_878E
; ---------------------------------------------------------------------------

loc_8736:				; CODE XREF: PrnCircle_Clip+38j
		ld	a, b
		add	a, d
		jr	c, loc_873F

loc_873A:				; DATA XREF: RAM:8ED3o
		cp	78h ; 'x'
		jp	c, loc_87C8

loc_873F:				; CODE XREF: PrnCircle_Clip+4Aj
		ld	a, d
		sub	b
		jr	c, loc_8745

loc_8743:				; DATA XREF: RAM:8ED5o
		cp	78h ; 'x'

loc_8745:				; CODE XREF: PrnCircle_Clip+53j
		jp	c, loc_8898
		jp	loc_87FE
; ---------------------------------------------------------------------------

loc_874B:				; CODE XREF: PrnCircle_Clip+43j
		ld	a, b
		sub	78h ; 'x'
		ld	b, a
		ld	(word_861E+1), a
		ld	a, d
		sub	b
		ret	c
		ret	z
		sub	0F0h ; 'Ё'
		jp	nc, loc_87F4
		jp	loc_888E
; ---------------------------------------------------------------------------

loc_875E:				; CODE XREF: PrnCircle_Clip+36j
					; PrnCircle_Clip+3Bj
		ld	a, c
		sub	40h ; '@'
		ld	c, a
		ld	(word_861E), a
		ld	a, d
		sub	c
		ret	c
		ld	a, d
		call	Sqr_DE_A
		ld	h, d
		ld	l, e
		ld	a, c
		call	Sqr_DE_A
		sbc	hl, de
		ld	a, 78h ; 'x'
		sub	b
		call	Sqr_DE_A
		sbc	hl, de
		jp	c, loc_893F
		add	hl, de
		ld	a, 78h ; 'x'
		add	a, b
		call	Sqr_DE_A
		sbc	hl, de
		jp	nc, loc_884A
		jp	loc_88C2
; ---------------------------------------------------------------------------

loc_878E:				; CODE XREF: PrnCircle_Clip+41j
					; PrnCircle_Clip+46j
		ld	a, b
		sub	78h ; 'x'
		ld	b, a
		ld	a, c
		sub	40h ; '@'
		ld	c, a
		ld	(word_861E), bc
		ld	a, d
		call	Sqr_DE_A
		ld	h, d
		ld	l, e
		ld	a, c
		call	Sqr_DE_A
		sbc	hl, de
		ret	c
		ld	a, b
		call	Sqr_DE_A
		sbc	hl, de
		ret	c
		add	hl, de
		ld	a, b
		add	a, 0F8h	; '°'
		jr	c, loc_87BC
		call	Sqr_DE_A
		sbc	hl, de
		jp	nc, loc_8840

loc_87BC:				; CODE XREF: PrnCircle_Clip+C4j
		jp	loc_88B8
; End of function PrnCircle_Clip


; =============== S U B	R O U T	I N E =======================================


Sqr_DE_A:				; CODE XREF: CalcLenVectorCrd+29p
					; CalcLenVectorCrd+30p	...
		push	hl
		ld	h, 0BEh	; '╛'
		ld	l, a
		ld	e, (hl)
		inc	h
		ld	d, (hl)
		pop	hl
		ret
; End of function Sqr_DE_A

; ---------------------------------------------------------------------------
; START	OF FUNCTION CHUNK FOR PrnCircle_Clip

loc_87C8:				; CODE XREF: PrnCircle_Clip+4Ej
		call	sub_87D4
		call	sub_86A1
		call	sub_87D4
		jp	sub_8689
; END OF FUNCTION CHUNK	FOR PrnCircle_Clip

; =============== S U B	R O U T	I N E =======================================




; === PrnCircle_NoFill === CODE XREF: PrnSunPlanet+39j
PrnCircle_NoFill:			; CODE XREF: PrnSunPlanet+39j
					; sub_7E02+8Bp	...
		call	PrnSmallCircle_NoFill
		cp	9
		ret	c

loc_86E4:				; CODE XREF: sub_9296+79p
		ld	a, 1
		ld	(FillCircle), a

PrnCircle_NoFill_NoRadius:		; CODE XREF: PrnCircle_Fill+Bj
		call	PrnCircle_Clip
		ei
		ret
; End of function PrnCircle_Fill


; =============== S U B	R O U T	I N E =======================================


PrnCircle_Clip:				; CODE XREF: PrnCircle_Fill:PrnCircle_NoFill_NoRadiusp

; FUNCTION CHUNK AT 87C8 SIZE 0000000C BYTES
; FUNCTION CHUNK AT 87F4 SIZE 00000191 BYTES
; FUNCTION CHUNK AT 8FB8 SIZE 00000022 BYTES

		ld	bc, (CircleCrdY)
		ld	(word_861E), bc
		ld	de, (CircleFlg)

loc_86FA:				; DATA XREF: RAM:off_8ECDo
		ld	a, 40h ; '@'
		bit	1, e
		jr	nz, loc_8702
		add	a, c
		add	a, c

loc_8702:				; CODE XREF: PrnCircle_Clip+10j
		sub	c
		ld	(word_8620), a
		ld	a, 80h ; 'А'
		bit	0, e
		jr	z, loc_870E
		add	a, b
		add	a, b

loc_870E:				; CODE XREF: PrnCircle_Clip+1Cj
		sub	b
		ld	(word_8620+1), a
		ld	a, c

loc_8713:				; DATA XREF: RAM:8ECFo
		cp	40h ; '@'
		bit	3, e
		ex	af, af'
		ld	a, b

loc_8719:				; DATA XREF: RAM:8ED1o
		cp	78h ; 'x'
		bit	2, e
		jr	z, loc_872B
		jr	nc, loc_872C
		ex	af, af'
		jr	z, loc_8728
		jr	nc, loc_875E
		jr	loc_8736
; ---------------------------------------------------------------------------

loc_8728:				; CODE XREF: PrnCircle_Clip+34j
		ret	nc
		jr	loc_875E
; ---------------------------------------------------------------------------

loc_872B:				; CODE XREF: PrnCircle_Clip+2Fj
		ret	nc

loc_872C:				; CODE XREF: PrnCircle_Clip+31j
		ex	af, af'
		jr	z, loc_8733
		jr	nc, loc_878E
		jr	loc_874B
; ---------------------------------------------------------------------------

loc_8733:				; CODE XREF: PrnCircle_Clip+3Fj
		ret	nc
		jr	loc_878E
; ---------------------------------------------------------------------------

loc_8736:				; CODE XREF: PrnCircle_Clip+38j
		ld	a, b
		add	a, d
		jr	c, loc_873F

loc_873A:				; DATA XREF: RAM:8ED3o
		cp	78h ; 'x'
		jp	c, loc_87C8

loc_873F:				; CODE XREF: PrnCircle_Clip+4Aj
		ld	a, d
		sub	b
		jr	c, loc_8745

loc_8743:				; DATA XREF: RAM:8ED5o
		cp	78h ; 'x'

loc_8745:				; CODE XREF: PrnCircle_Clip+53j
		jp	c, loc_8898
		jp	loc_87FE
; ---------------------------------------------------------------------------

loc_874B:				; CODE XREF: PrnCircle_Clip+43j
		ld	a, b
		sub	78h ; 'x'
		ld	b, a
		ld	(word_861E+1), a
		ld	a, d
		sub	b
		ret	c
		ret	z
		sub	0F0h ; 'Ё'
		jp	nc, loc_87F4
		jp	loc_888E
; ---------------------------------------------------------------------------

loc_875E:				; CODE XREF: PrnCircle_Clip+36j
					; PrnCircle_Clip+3Bj
		ld	a, c
		sub	40h ; '@'
		ld	c, a
		ld	(word_861E), a
		ld	a, d
		sub	c
		ret	c
		ld	a, d
		call	Sqr_DE_A
		ld	h, d
		ld	l, e
		ld	a, c
		call	Sqr_DE_A
		sbc	hl, de
		ld	a, 78h ; 'x'
		sub	b
		call	Sqr_DE_A
		sbc	hl, de
		jp	c, loc_893F
		add	hl, de
		ld	a, 78h ; 'x'
		add	a, b
		call	Sqr_DE_A
		sbc	hl, de
		jp	nc, loc_884A
		jp	loc_88C2
; ---------------------------------------------------------------------------

loc_878E:				; CODE XREF: PrnCircle_Clip+41j
					; PrnCircle_Clip+46j
		ld	a, b
		sub	78h ; 'x'
		ld	b, a
		ld	a, c
		sub	40h ; '@'
		ld	c, a
		ld	(word_861E), bc
		ld	a, d
		call	Sqr_DE_A
		ld	h, d
		ld	l, e
		ld	a, c
		call	Sqr_DE_A
		sbc	hl, de
		ret	c
		ld	a, b
		call	Sqr_DE_A
		sbc	hl, de
		ret	c
		add	hl, de
		ld	a, b
		add	a, 0F8h	; '°'
		jr	c, loc_87BC
		call	Sqr_DE_A
		sbc	hl, de
		jp	nc, loc_8840

loc_87BC:				; CODE XREF: PrnCircle_Clip+C4j
		jp	loc_88B8
; End of function PrnCircle_Clip


; =============== S U B	R O U T	I N E =======================================


Sqr_DE_A:				; CODE XREF: CalcLenVectorCrd+29p
					; CalcLenVectorCrd+30p	...
		push	hl
		ld	h, 0BEh	; '╛'
		ld	l, a
		ld	e, (hl)
		inc	h
		ld	d, (hl)
		pop	hl
		ret
; End of function Sqr_DE_A

; ---------------------------------------------------------------------------
; START	OF FUNCTION CHUNK FOR PrnCircle_Clip

loc_87C8:				; CODE XREF: PrnCircle_Clip+4Ej
		call	sub_87D4
		call	sub_86A1
		call	sub_87D4
		jp	sub_8689
; END OF FUNCTION CHUNK	FOR PrnCircle_Clip

; =============== S U B	R O U T	I N E =======================================




; === PrnCircle_NoFill_NoRadius === CODE XREF: PrnCircle_Fill+Bj
PrnCircle_NoFill_NoRadius:		; CODE XREF: PrnCircle_Fill+Bj
		call	PrnCircle_Clip
		ei
		ret
; End of function PrnCircle_Fill


; =============== S U B	R O U T	I N E =======================================


PrnCircle_Clip:				; CODE XREF: PrnCircle_Fill:PrnCircle_NoFill_NoRadiusp

; FUNCTION CHUNK AT 87C8 SIZE 0000000C BYTES
; FUNCTION CHUNK AT 87F4 SIZE 00000191 BYTES
; FUNCTION CHUNK AT 8FB8 SIZE 00000022 BYTES

		ld	bc, (CircleCrdY)
		ld	(word_861E), bc
		ld	de, (CircleFlg)

loc_86FA:				; DATA XREF: RAM:off_8ECDo
		ld	a, 40h ; '@'
		bit	1, e
		jr	nz, loc_8702
		add	a, c
		add	a, c

loc_8702:				; CODE XREF: PrnCircle_Clip+10j
		sub	c
		ld	(word_8620), a
		ld	a, 80h ; 'А'
		bit	0, e
		jr	z, loc_870E
		add	a, b
		add	a, b

loc_870E:				; CODE XREF: PrnCircle_Clip+1Cj
		sub	b
		ld	(word_8620+1), a
		ld	a, c

loc_8713:				; DATA XREF: RAM:8ECFo
		cp	40h ; '@'
		bit	3, e
		ex	af, af'
		ld	a, b

loc_8719:				; DATA XREF: RAM:8ED1o
		cp	78h ; 'x'
		bit	2, e
		jr	z, loc_872B
		jr	nc, loc_872C
		ex	af, af'
		jr	z, loc_8728
		jr	nc, loc_875E
		jr	loc_8736
; ---------------------------------------------------------------------------

loc_8728:				; CODE XREF: PrnCircle_Clip+34j
		ret	nc
		jr	loc_875E
; ---------------------------------------------------------------------------

loc_872B:				; CODE XREF: PrnCircle_Clip+2Fj
		ret	nc

loc_872C:				; CODE XREF: PrnCircle_Clip+31j
		ex	af, af'
		jr	z, loc_8733
		jr	nc, loc_878E
		jr	loc_874B
; ---------------------------------------------------------------------------

loc_8733:				; CODE XREF: PrnCircle_Clip+3Fj
		ret	nc
		jr	loc_878E
; ---------------------------------------------------------------------------

loc_8736:				; CODE XREF: PrnCircle_Clip+38j
		ld	a, b
		add	a, d
		jr	c, loc_873F

loc_873A:				; DATA XREF: RAM:8ED3o
		cp	78h ; 'x'
		jp	c, loc_87C8

loc_873F:				; CODE XREF: PrnCircle_Clip+4Aj
		ld	a, d
		sub	b
		jr	c, loc_8745

loc_8743:				; DATA XREF: RAM:8ED5o
		cp	78h ; 'x'

loc_8745:				; CODE XREF: PrnCircle_Clip+53j
		jp	c, loc_8898
		jp	loc_87FE
; ---------------------------------------------------------------------------

loc_874B:				; CODE XREF: PrnCircle_Clip+43j
		ld	a, b
		sub	78h ; 'x'
		ld	b, a
		ld	(word_861E+1), a
		ld	a, d
		sub	b
		ret	c
		ret	z
		sub	0F0h ; 'Ё'
		jp	nc, loc_87F4
		jp	loc_888E
; ---------------------------------------------------------------------------

loc_875E:				; CODE XREF: PrnCircle_Clip+36j
					; PrnCircle_Clip+3Bj
		ld	a, c
		sub	40h ; '@'
		ld	c, a
		ld	(word_861E), a
		ld	a, d
		sub	c
		ret	c
		ld	a, d
		call	Sqr_DE_A
		ld	h, d
		ld	l, e
		ld	a, c
		call	Sqr_DE_A
		sbc	hl, de
		ld	a, 78h ; 'x'
		sub	b
		call	Sqr_DE_A
		sbc	hl, de
		jp	c, loc_893F
		add	hl, de
		ld	a, 78h ; 'x'
		add	a, b
		call	Sqr_DE_A
		sbc	hl, de
		jp	nc, loc_884A
		jp	loc_88C2
; ---------------------------------------------------------------------------

loc_878E:				; CODE XREF: PrnCircle_Clip+41j
					; PrnCircle_Clip+46j
		ld	a, b
		sub	78h ; 'x'
		ld	b, a
		ld	a, c
		sub	40h ; '@'
		ld	c, a
		ld	(word_861E), bc
		ld	a, d
		call	Sqr_DE_A
		ld	h, d
		ld	l, e
		ld	a, c
		call	Sqr_DE_A
		sbc	hl, de
		ret	c
		ld	a, b
		call	Sqr_DE_A
		sbc	hl, de
		ret	c
		add	hl, de
		ld	a, b
		add	a, 0F8h	; '°'
		jr	c, loc_87BC
		call	Sqr_DE_A
		sbc	hl, de
		jp	nc, loc_8840

loc_87BC:				; CODE XREF: PrnCircle_Clip+C4j
		jp	loc_88B8
; End of function PrnCircle_Clip


; =============== S U B	R O U T	I N E =======================================


Sqr_DE_A:				; CODE XREF: CalcLenVectorCrd+29p
					; CalcLenVectorCrd+30p	...
		push	hl
		ld	h, 0BEh	; '╛'
		ld	l, a
		ld	e, (hl)
		inc	h
		ld	d, (hl)
		pop	hl
		ret
; End of function Sqr_DE_A

; ---------------------------------------------------------------------------
; START	OF FUNCTION CHUNK FOR PrnCircle_Clip

loc_87C8:				; CODE XREF: PrnCircle_Clip+4Ej
		call	sub_87D4
		call	sub_86A1
		call	sub_87D4
		jp	sub_8689
; END OF FUNCTION CHUNK	FOR PrnCircle_Clip

; =============== S U B	R O U T	I N E =======================================




; === PrnPlanetOnSmallMap === DATA XREF: RAM:ECB5o
PrnPlanetOnSmallMap:			; DATA XREF: RAM:ECB5o
		call	sub_ECFD
		ld	a, h
		ex	af, af'
		ld	hl, sub_FB00
		ld	(hl), 0AEh ; 'о'
		push	hl
		ld	a, (byte_DEC5)
		and	0Ch
		rra
		rra
		jr	nz, loc_ED3C
		inc	a

loc_ED3C:				; CODE XREF: PrnPlanetOnSmallMap+12j
		add	a, 2
		push	af
		push	bc
		call	sub_FB03	; подготовка для печати	маленькой окружности
					; a = радиус окружности	2..8
		pop	bc
		pop	af
		sub	b
		neg
		add	a, 4
		ld	b, a
		push	de
		call	22AAh
		pop	de
		call	PrnSmallCircle
		pop	hl
		ld	(hl), 0B6h ; '╢'

loc_ED56:				; DATA XREF: sub_FB03+3o
		jp	NextPlanetData
; End of function PrnPlanetOnSmallMap

; ---------------------------------------------------------------------------
		db  81h	; Б
		db  41h	; A
		db  42h	; B
		db  22h	; "
		db  23h	; #
		db  13h
		db  14h

; =============== S U B	R O U T	I N E =======================================




; === PrnSmallCircle === CODE XREF: PrnPlanetOnSmallMap+29p
PrnSmallCircle:				; CODE XREF: PrnPlanetOnSmallMap+29p
		ld	c, a
		push	ix
		ld	ix, TablCrdPrnVertex
		xor	a

loc_FB83:				; CODE XREF: PrnSmallCircle+57j
					; PrnSmallCircle+60j
		push	af
		ld	a, h
		dec	h
		and	7
		call	z, compUpScrAdr_HL
		ld	a, h
		cp	60h ; '`'
		jr	c, loc_FB98
		cp	0D0h ; '╨'
		jr	nc, loc_FBC9
		cp	0C0h ; '└'
		jr	c, loc_FBC9

loc_FB98:				; CODE XREF: PrnSmallCircle+13j
		push	hl
		ld	a, (ix+0)
		ld	b, 8

loc_FB9E:				; CODE XREF: PrnSmallCircle+26j
		rlca
		rr	h
		djnz	loc_FB9E
		add	a, a
		ld	l, a
		ld	a, 8
		sub	c
		ld	b, a
		xor	a

loc_FBAA:				; CODE XREF: PrnSmallCircle+31j
		add	hl, hl
		rla
		djnz	loc_FBAA
		ld	b, d
		ex	de, hl
		ex	(sp), hl
		rrc	b
		jr	c, loc_FBBA
		dec	hl
		call	sub_FB00
		inc	hl

loc_FBBA:				; CODE XREF: PrnSmallCircle+38j
		ld	a, d
		call	sub_FB00
		rrc	b
		jr	c, loc_FBC8
		inc	hl
		ld	a, e
		call	sub_FB00
		dec	hl

loc_FBC8:				; CODE XREF: PrnSmallCircle+45j
		pop	de

loc_FBC9:				; CODE XREF: PrnSmallCircle+17j
					; PrnSmallCircle+1Bj
		pop	af
		bit	7, d
		jr	nz, loc_FBD8
		inc	a
		inc	ix
		cp	e
		jr	nz, loc_FB83
		set	7, d
		dec	ix

loc_FBD8:				; CODE XREF: PrnSmallCircle+51j
		dec	ix
		dec	a
		jr	nz, loc_FB83
		pop	ix
		ret
; End of function PrnSmallCircle


; =============== S U B	R O U T	I N E =======================================




; === PrnSmallCircle_NoFill === CODE XREF: PrnCircle_Fill:PrnCircle_NoFillp
PrnSmallCircle_NoFill:			; CODE XREF: PrnCircle_Fill:PrnCircle_NoFillp
		xor	a
		ex	af, af'
		ld	a, (WhatOnScreen)
		cp	4
		ld	a, (CircleRadius)
		ret	nc
		cp	9
		ret	nc
		cp	2
		ret	c
		call	sub_FB03	; подготовка для печати	маленькой окружности
					; a = радиус окружности	2..8
		ld	hl, (CircleCrdY)
		ld	a, (CircleFlg)
		ld	b, a
		ld	a, 7Fh ; ''
		cp	h
		jr	c, loc_FB6A
		add	a, h
		rrc	b
		jr	c, loc_FB59
		sub	h
		sub	h

loc_FB59:				; CODE XREF: PrnSmallCircle_NoFill+23j
		ld	c, a
		cp	8
		jr	nc, loc_FB5F
		inc	d

loc_FB5F:				; CODE XREF: PrnSmallCircle_NoFill+2Aj
		cp	0F8h ; '°'
		jr	c, loc_FB65
		inc	d
		inc	d

loc_FB65:				; CODE XREF: PrnSmallCircle_NoFill+2Fj
		ld	a, 48h ; 'H'
		cp	l
		jr	nc, loc_FB6C

loc_FB6A:				; CODE XREF: PrnSmallCircle_NoFill+1Ej
		xor	a
		ret
; ---------------------------------------------------------------------------

loc_FB6C:				; CODE XREF: PrnSmallCircle_NoFill+36j
		ld	a, 40h ; '@'
		add	a, l
		rrc	b
		jr	nc, loc_FB75
		sub	l
		sub	l

loc_FB75:				; CODE XREF: PrnSmallCircle_NoFill+3Fj
		add	a, e
		call	22B0h
		set	7, h
; End of function PrnSmallCircle_NoFill


; =============== S U B	R O U T	I N E =======================================


PrnSmallCircle:				; CODE XREF: PrnPlanetOnSmallMap+29p
		ld	c, a
		push	ix
		ld	ix, TablCrdPrnVertex
		xor	a

loc_FB83:				; CODE XREF: PrnSmallCircle+57j
					; PrnSmallCircle+60j
		push	af
		ld	a, h
		dec	h
		and	7
		call	z, compUpScrAdr_HL
		ld	a, h
		cp	60h ; '`'
		jr	c, loc_FB98
		cp	0D0h ; '╨'
		jr	nc, loc_FBC9
		cp	0C0h ; '└'
		jr	c, loc_FBC9

loc_FB98:				; CODE XREF: PrnSmallCircle+13j
		push	hl
		ld	a, (ix+0)
		ld	b, 8

loc_FB9E:				; CODE XREF: PrnSmallCircle+26j
		rlca
		rr	h
		djnz	loc_FB9E
		add	a, a
		ld	l, a
		ld	a, 8
		sub	c
		ld	b, a
		xor	a

loc_FBAA:				; CODE XREF: PrnSmallCircle+31j
		add	hl, hl
		rla
		djnz	loc_FBAA
		ld	b, d
		ex	de, hl
		ex	(sp), hl
		rrc	b
		jr	c, loc_FBBA
		dec	hl
		call	sub_FB00
		inc	hl

loc_FBBA:				; CODE XREF: PrnSmallCircle+38j
		ld	a, d
		call	sub_FB00
		rrc	b
		jr	c, loc_FBC8
		inc	hl
		ld	a, e
		call	sub_FB00
		dec	hl

loc_FBC8:				; CODE XREF: PrnSmallCircle+45j
		pop	de

loc_FBC9:				; CODE XREF: PrnSmallCircle+17j
					; PrnSmallCircle+1Bj
		pop	af
		bit	7, d
		jr	nz, loc_FBD8
		inc	a
		inc	ix
		cp	e
		jr	nz, loc_FB83
		set	7, d
		dec	ix

loc_FBD8:				; CODE XREF: PrnSmallCircle+51j
		dec	ix
		dec	a
		jr	nz, loc_FB83
		pop	ix
		ret
; End of function PrnSmallCircle


; =============== S U B	R O U T	I N E =======================================




; === PrnSunPlanet === CODE XREF: sub_728D+261p
PrnSunPlanet:				; CODE XREF: sub_728D+261p
					; LaunchDocking+FAp
		xor	a
		ld	(TypeOfPrint), a
		ld	hl, SunCrd
		ld	de, CircleCrdXlow
		ld	bc, 9
		ldir
		call	CalcPrnCrdSun
		push	af
		call	CorrData4Nova
		pop	af
		and	a
		jr	z, loc_75AD
		call	PrnCircle_Fill
		ld	a, (CircleRadius)
		cp	28h ; '('
		jr	c, loc_75AD
		ld	a, 1
		ld	(TypeOfPrint), a

loc_75AD:				; CODE XREF: PrnSunPlanet+18j
					; PrnSunPlanet+22j
		ld	hl, PlanetCrd
		ld	de, CircleCrdXlow
		ld	bc, 9
		ldir
		call	CalcPrnCrdSun
		and	a
		ret	z
		jp	PrnCircle_NoFill
; End of function PrnSunPlanet


; =============== S U B	R O U T	I N E =======================================


CorrData4Nova:				; CODE XREF: sub_74C6:loc_74D2p
					; PrnSunPlanet+13p
		ld	a, (PilotMission1_1)
		and	a
		ret	z
		ld	a, (SunRadiusNova)
		and	a
		jr	nz, loc_75CE
		ld	a, (CircleRadius)

loc_75CE:				; CODE XREF: CorrData4Nova+9j
		add	a, 2
		jr	z, loc_75D4
		jr	nc, loc_75DA

loc_75D4:				; CODE XREF: CorrData4Nova+10j
		ld	a, 1
		ld	(PlayerGameOver), a
		xor	a

loc_75DA:				; CODE XREF: CorrData4Nova+12j
		ld	(CircleRadius),	a
		ld	(SunRadiusNova), a
		ret
; End of function CorrData4Nova

; ---------------------------------------------------------------------------

loc_75E1:				; CODE XREF: CalcPrnCrdSunp
		exx
		ld	bc, 3
		exx
		ld	a, (WhatOnScreen)
		and	a
		jp	nz, loc_7695
		ld	a, (CircleCrdZhi)
		bit	7, a
		jp	nz, loc_782A
		and	7Fh ; ''
		ld	(CircleCrdZhi),	a
		ld	hl, (CircleCrdZmed)
		ex	de, hl
		ld	hl, (CircleCrdXmed)
		ld	a, (CircleCrdZlow)
		ld	b, a
		ld	a, (CircleCrdXlow)
		ld	c, a
		ld	a, h
		bit	7, a
		jr	nz, loc_7612
		exx
		set	0, b
		exx

loc_7612:				; CODE XREF: RAM:760Cj	RAM:loc_76C0j
		and	7Fh ; ''
		ld	h, a
		call	sub_77A0
		ld	a, d
		ld	(CircleCrdY+1),	a
		ld	hl, (CircleCrdZmed)
		ex	de, hl
		ld	hl, (CircleCrdYmed)
		ld	a, (CircleCrdZlow)
		ld	b, a
		ld	a, (CircleCrdYlow)
		ld	c, a
		ld	a, h
		bit	7, a
		jr	nz, loc_7634
		exx
		set	1, b
		exx

loc_7634:				; CODE XREF: RAM:762Ej
		and	7Fh ; ''
		ld	h, a
		call	sub_77A0
		ld	a, d
		ld	(CircleCrdY), a
		exx
		ld	a, b
		exx
		ld	(CircleFlg), a
		ld	a, (CircleCrdZhi)
		and	a
		jr	nz, loc_7659
		ld	hl, 0A000h
		ld	de, (CircleCrdZlow)
		sbc	hl, de
		jr	c, loc_7659
		ld	a, 0FFh
		jr	loc_7692
; ---------------------------------------------------------------------------

loc_7659:				; CODE XREF: RAM:7648j	RAM:7653j
		ld	hl, 0A000h
		ld	de, (CircleCrdZmed)
		and	a
		sbc	hl, de
		jp	c, loc_782A
		ld	a, (CircleCrdZhi)
		and	a
		jr	z, loc_7688
		ld	b, a
		ld	a, (CircleCrdZmed)
		ld	c, a
		xor	a
		ld	e, 0A0h	; 'а'

loc_7674:				; CODE XREF: RAM:767Dj
		sla	c
		rl	b
		jr	c, loc_767F
		sla	e
		rla
		jr	loc_7674
; ---------------------------------------------------------------------------

loc_767F:				; CODE XREF: RAM:7678j
		scf
		rr	b
		call	loc_783D
		ld	a, d
		jr	loc_7692
; ---------------------------------------------------------------------------

loc_7688:				; CODE XREF: RAM:766Aj
		ld	a, (CircleCrdZmed)
		ld	b, a
		ld	a, 0A0h	; 'а'
		call	loc_783B
		ld	a, d

loc_7692:				; CODE XREF: RAM:7657j	RAM:7686j
		jp	loc_7793
; ---------------------------------------------------------------------------

loc_7695:				; CODE XREF: RAM:75EAj
		dec	a
		jr	nz, loc_76C3
		ld	a, (CircleCrdZhi)
		bit	7, a
		jp	z, loc_782A
		and	7Fh ; ''
		ld	(CircleCrdZhi),	a
		ld	hl, (CircleCrdZmed)
		ex	de, hl
		ld	hl, (CircleCrdXmed)
		ld	a, (CircleCrdZlow)
		ld	b, a
		ld	a, (CircleCrdXlow)
		ld	c, a
		ld	a, h
		bit	7, a
		jr	z, loc_76C0
		and	7Fh ; ''
		ld	h, a
		exx
		set	0, b
		exx

loc_76C0:				; CODE XREF: RAM:76B7j
		jp	loc_7612
; ---------------------------------------------------------------------------

loc_76C3:				; CODE XREF: RAM:7696j
		dec	a
		jp	nz, loc_7770
		ld	a, (CircleCrdXhi)
		bit	7, a
		jp	z, loc_782A
		and	7Fh ; ''
		ld	(CircleCrdXhi),	a
		ld	hl, (CircleCrdXmed)
		ex	de, hl
		ld	hl, (CircleCrdZmed)
		ld	a, (CircleCrdXlow)
		ld	b, a
		ld	a, (CircleCrdZlow)
		ld	c, a
		ld	a, h
		bit	7, a
		jr	nz, loc_76EC
		exx
		set	0, b
		exx

loc_76EC:				; CODE XREF: RAM:76E6j	RAM:loc_7790j
		and	7Fh ; ''
		ld	h, a
		call	sub_77A0
		ld	a, d
		ld	(CircleCrdY+1),	a
		ld	hl, (CircleCrdXmed)
		ex	de, hl
		ld	hl, (CircleCrdYmed)
		ld	a, (CircleCrdXlow)
		ld	b, a
		ld	a, (CircleCrdYlow)
		ld	c, a
		ld	a, h
		bit	7, a
		jr	nz, loc_770E
		exx
		set	1, b
		exx

loc_770E:				; CODE XREF: RAM:7708j
		and	7Fh ; ''
		ld	h, a
		call	sub_77A0
		ld	a, d
		ld	(CircleCrdY), a
		exx
		ld	a, b
		exx
		ld	(CircleFlg), a
		ld	a, (CircleCrdXhi)
		and	a
		jr	nz, loc_7733
		ld	hl, 0A000h
		ld	de, (CircleCrdXlow)
		sbc	hl, de
		jr	c, loc_7733
		ld	a, 0FFh
		jr	loc_776D
; ---------------------------------------------------------------------------

loc_7733:				; CODE XREF: RAM:7722j	RAM:772Dj
		ld	hl, 0A000h
		ld	de, (CircleCrdXmed)
		and	a
		sbc	hl, de
		jp	c, loc_782A
		ld	a, (CircleCrdXhi)
		and	a
		jr	z, loc_7763
		ld	b, a
		ld	a, (CircleCrdXmed)
		ld	c, a
		xor	a
		ld	e, 0A0h	; 'а'

loc_774E:				; CODE XREF: RAM:7757j
		sla	c
		rl	b
		jr	c, loc_775A
		sla	e
		rla
		jp	loc_774E
; ---------------------------------------------------------------------------

loc_775A:				; CODE XREF: RAM:7752j
		scf
		rr	b
		call	loc_783D
		ld	a, d
		jr	loc_776D
; ---------------------------------------------------------------------------

loc_7763:				; CODE XREF: RAM:7744j
		ld	a, (CircleCrdXmed)
		ld	b, a
		ld	a, 0A0h	; 'а'
		call	loc_783B
		ld	a, d

loc_776D:				; CODE XREF: RAM:7731j	RAM:7761j
		jp	loc_7793
; ---------------------------------------------------------------------------

loc_7770:				; CODE XREF: RAM:76C4j
		ld	a, (CircleCrdXhi)
		bit	7, a
		jp	nz, loc_782A
		ld	hl, (CircleCrdXmed)
		ex	de, hl
		ld	hl, (CircleCrdZmed)
		ld	a, (CircleCrdXlow)
		ld	b, a
		ld	a, (CircleCrdZlow)
		ld	c, a
		ld	a, h
		bit	7, a
		jr	z, loc_7790
		exx
		set	0, b
		exx

loc_7790:				; CODE XREF: RAM:778Aj
		jp	loc_76EC
; ---------------------------------------------------------------------------

loc_7793:				; CODE XREF: RAM:loc_7692j
					; RAM:loc_776Dj
		ld	(CircleRadius),	a
		ld	a, 1
		ret
; ---------------------------------------------------------------------------
; START	OF FUNCTION CHUNK FOR sub_77A0

loc_7799:				; CODE XREF: sub_77A0+10j
		ld	h, l
		ld	l, c
		ld	d, e
		ld	e, b
		jp	loc_77C4
; END OF FUNCTION CHUNK	FOR sub_77A0

; =============== S U B	R O U T	I N E =======================================




; === RestorePlanetData === CODE XREF: sub_DF24:loc_DF4Cp
RestorePlanetData:			; CODE XREF: sub_DF24:loc_DF4Cp
					; SearchPlanetNearCursor+35p
		ld	hl, DataOfPlanetSv
		ld	de, DataOfPlanet

loc_E13C:				; CODE XREF: CopyGalaxyToPlanet+6j
					; SavePlanetData+6j
		ld	bc, 6
		ldir
		ret
; End of function RestorePlanetData


; =============== S U B	R O U T	I N E =======================================


GenerateProductPrice:			; CODE XREF: sub_DCE9+26p
		call	sub_ABAF
		ld	(PilotRnd), a
; End of function GenerateProductPrice


; =============== S U B	R O U T	I N E =======================================


GenerateAnotherPrice:			; CODE XREF: InitGame+23p
		call	sub_9B83
		ld	de, ProductPrice
		exx
		ld	hl, TableBasePrice ; +#28 Fireams
		ld	b, 11h

loc_E154:				; CODE XREF: GenerateAnotherPrice+6Cj
		push	bc
		ld	e, (hl)
		inc	hl
		ld	a, (hl)
		ld	d, a
		push	de
		and	1Fh
		jr	z, loc_E167
		ld	b, a
		ld	a, (PlanetEconomy)
		ld	c, a
		xor	a

loc_E164:				; CODE XREF: GenerateAnotherPrice+1Dj
		add	a, c
		djnz	loc_E164

loc_E167:				; CODE XREF: GenerateAnotherPrice+14j
		ld	e, a
		inc	hl
		ld	b, (hl)
		inc	hl
		ld	c, (hl)
		inc	hl
		ld	a, (PilotRnd)
		and	c
		add	a, b
		ld	b, a
		ld	a, e
		bit	7, d
		jr	nz, loc_E17B
		inc	a
		neg

loc_E17B:				; CODE XREF: GenerateAnotherPrice+2Ej
		add	a, b
		bit	7, a
		jr	z, loc_E181
		xor	a

loc_E181:				; CODE XREF: GenerateAnotherPrice+36j
		and	3Fh ; '?'
		ld	b, e
		pop	de
		push	af
		ld	a, (PilotRnd)
		and	c
		add	a, e
		ld	e, 0
		rl	e
		ld	c, a
		ld	a, b
		bit	7, d
		jr	z, loc_E19A
		ld	d, a
		ld	a, c
		sub	d
		jr	loc_E19E
; ---------------------------------------------------------------------------

loc_E19A:				; CODE XREF: GenerateAnotherPrice+4Bj
		add	a, c
		jr	nc, loc_E19E
		inc	e

loc_E19E:				; CODE XREF: GenerateAnotherPrice+50j
					; GenerateAnotherPrice+53j
		ld	b, e
		add	a, a
		rl	b
		add	a, a
		rl	b
		ld	c, a
		push	bc
		exx
		pop	bc
		ld	a, c
		ld	(de), a
		inc	de
		ld	a, b
		ld	(de), a
		inc	de
		pop	af

Amount:					; DATA XREF: InitGame+1Do
		ld	(hl), a
		inc	hl
		exx
		pop	bc
		djnz	loc_E154
		ld	hl, planetAlienItems
		ld	(hl), b
		ret
; End of function GenerateAnotherPrice

; ---------------------------------------------------------------------------
byte_E1BB:	db 0			; DATA XREF: sub_E962+6o sub_EF48+25o
		db 0


; === SavePlanetData === CODE XREF: CalcDistanceToCursor+19p
SavePlanetData:				; CODE XREF: CalcDistanceToCursor+19p
					; sub_DF24:loc_DF2Bp ...
		ld	hl, DataOfPlanet
		ld	de, DataOfPlanetSv
		jr	loc_E13C
; End of function SavePlanetData


; =============== S U B	R O U T	I N E =======================================


RestorePlanetData:			; CODE XREF: sub_DF24:loc_DF4Cp
					; SearchPlanetNearCursor+35p
		ld	hl, DataOfPlanetSv
		ld	de, DataOfPlanet

loc_E13C:				; CODE XREF: CopyGalaxyToPlanet+6j
					; SavePlanetData+6j
		ld	bc, 6
		ldir
		ret
; End of function RestorePlanetData


; =============== S U B	R O U T	I N E =======================================


GenerateProductPrice:			; CODE XREF: sub_DCE9+26p
		call	sub_ABAF
		ld	(PilotRnd), a
; End of function GenerateProductPrice


; =============== S U B	R O U T	I N E =======================================


GenerateAnotherPrice:			; CODE XREF: InitGame+23p
		call	sub_9B83
		ld	de, ProductPrice
		exx
		ld	hl, TableBasePrice ; +#28 Fireams
		ld	b, 11h

loc_E154:				; CODE XREF: GenerateAnotherPrice+6Cj
		push	bc
		ld	e, (hl)
		inc	hl
		ld	a, (hl)
		ld	d, a
		push	de
		and	1Fh
		jr	z, loc_E167
		ld	b, a
		ld	a, (PlanetEconomy)
		ld	c, a
		xor	a

loc_E164:				; CODE XREF: GenerateAnotherPrice+1Dj
		add	a, c
		djnz	loc_E164

loc_E167:				; CODE XREF: GenerateAnotherPrice+14j
		ld	e, a
		inc	hl
		ld	b, (hl)
		inc	hl
		ld	c, (hl)
		inc	hl
		ld	a, (PilotRnd)
		and	c
		add	a, b
		ld	b, a
		ld	a, e
		bit	7, d
		jr	nz, loc_E17B
		inc	a
		neg

loc_E17B:				; CODE XREF: GenerateAnotherPrice+2Ej
		add	a, b
		bit	7, a
		jr	z, loc_E181
		xor	a

loc_E181:				; CODE XREF: GenerateAnotherPrice+36j
		and	3Fh ; '?'
		ld	b, e
		pop	de
		push	af
		ld	a, (PilotRnd)
		and	c
		add	a, e
		ld	e, 0
		rl	e
		ld	c, a
		ld	a, b
		bit	7, d
		jr	z, loc_E19A
		ld	d, a
		ld	a, c
		sub	d
		jr	loc_E19E
; ---------------------------------------------------------------------------

loc_E19A:				; CODE XREF: GenerateAnotherPrice+4Bj
		add	a, c
		jr	nc, loc_E19E
		inc	e

loc_E19E:				; CODE XREF: GenerateAnotherPrice+50j
					; GenerateAnotherPrice+53j
		ld	b, e
		add	a, a
		rl	b
		add	a, a
		rl	b
		ld	c, a
		push	bc
		exx
		pop	bc
		ld	a, c
		ld	(de), a
		inc	de
		ld	a, b
		ld	(de), a
		inc	de
		pop	af

Amount:					; DATA XREF: InitGame+1Do
		ld	(hl), a
		inc	hl
		exx
		pop	bc
		djnz	loc_E154
		ld	hl, planetAlienItems
		ld	(hl), b
		ret
; End of function GenerateAnotherPrice

; ---------------------------------------------------------------------------
byte_E1BB:	db 0			; DATA XREF: sub_E962+6o sub_EF48+25o
		db 0


; === SearchNearPlanet === CODE XREF: GoJump:loc_DBCFp
SearchNearPlanet:			; CODE XREF: GoJump:loc_DBCFp
					; PrnCountdownJump+55p	...
		ld	a, (WhatOnScreen)
		cp	7
		jr	c, SearchPlanetNearCursor
		cp	9
		jr	nc, SearchPlanetNearCursor
		call	PrintSmallCursOnMap
		call	SearchPlanetNearCursor
		jp	PrintSmallCursOnMap
; End of function SearchNearPlanet


; =============== S U B	R O U T	I N E =======================================


SearchPlanetNearCursor:			; CODE XREF: sub_5FD5+3Ap InitGame+1Ap ...
		call	CopyGalaxyToPlanet
		ld	hl, 0FFFFh
		ld	(MinDistance), hl
		ld	b, 0

loc_DF7F:				; CODE XREF: SearchPlanetNearCursor+2Cj
		push	bc
		ld	a, (DataOfPlanet1+1)
		ld	b, a
		ld	a, (DataOfPlanet+1)
		call	CalcDistanceToCursor
		jr	c, loc_DF9C
		ex	de, hl
		ld	hl, (MinDistance)
		and	a
		sbc	hl, de
		jr	c, loc_DF9C
		ld	(MinDistance), de
		call	SavePlanetData

loc_DF9C:				; CODE XREF: SearchPlanetNearCursor+16j
					; SearchPlanetNearCursor+1Fj
		call	NextPlanetData
		pop	bc
		djnz	loc_DF7F

loc_DFA2:				; CODE XREF: CalcDistanceToCursor+1Fj
		ld	hl, (MinDistance)
		and	a
		call	CalcFuelForDistance
		call	RestorePlanetData
		call	GenerateInfoPlanet
		call	GeneratePlanetName
		call	CopyPlanetName

CalcFuelForJump:			; CODE XREF: sub_DF24+37j
		ld	a, (Pilot_CrdXSystem)
		ld	b, a
		ld	a, (Pilot_CrdYSystem)
		call	CalcDistanceToCursor2
		call	CalcFuelForDistance
		and	a
		ret
; End of function SearchPlanetNearCursor


; =============== S U B	R O U T	I N E =======================================


NextPlanetData:				; CODE XREF: SearchPlanetNearCursor:loc_DF9Cp
					; Cmd_I_GlobalMap+6Bp ...
		ld	b, 4

loc_DFC6:				; CODE XREF: NextPlanetData+7j
		push	bc
		call	ModifyPlanetData
		pop	bc
		djnz	loc_DFC6
		ret
; End of function NextPlanetData


; =============== S U B	R O U T	I N E =======================================


CalcDistanceToCursor2:			; CODE XREF: CalcDistanceToCursor+Dp
					; SearchPlanetNearCursor+48p
		push	af
		ld	a, (Pilot_CrdXTarget)
		call	ABS_A_sub_B
		ld	d, a
		call	Mult_HL_D_A
		pop	bc
		push	hl
		ld	a, (Pilot_CrdYTarget)

loc_DFDE:				; CODE XREF: sub_E6C6+Dj
		call	ABS_A_sub_B
		srl	a
		ld	d, a
		call	Mult_HL_D_A
		pop	de
		add	hl, de
		ret
; End of function CalcDistanceToCursor2


; =============== S U B	R O U T	I N E =======================================


ABS_A_sub_B:				; CODE XREF: CalcDistanceToCursor2+4p
					; CalcDistanceToCursor2:loc_DFDEp ...
		sub	b
		ret	nc
		cpl
		ret
; End of function ABS_A_sub_B


; =============== S U B	R O U T	I N E =======================================




