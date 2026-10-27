; 14_galaxy_map - разобрано из kladov.asm
; Количество подпрограмм: 8

; === Cmd_I_GlobalMap === CODE XREF: RAM:D009j
Cmd_I_GlobalMap:			; CODE XREF: RAM:D009j
		call	ClsAndPrintRamk
		ld	a, 18h
		ld	(byte_E52C), a
		ld	a, 98h ; 'Ш'
		ld	(CrdYDownGalaxyMap), a
		ld	a, 10h
		ld	(CursorWidth), a
		ld	a, (Pilot_CrdXSystem)
		ld	(PrnCrdXCursor), a
		ld	a, (Pilot_CrdYSystem)
		srl	a
		add	a, 18h
		ld	(PrnCrdYCursor), a
		call	PrintCursorOnScr
		ld	a, (Pilot_Fuel)
		srl	a
		srl	a
		ld	(RadiusCircle),	a
		call	sub_7119
		ld	hl, 4060h
		ld	a, 2
		call	sub_EBEE
		ld	hl, 5100h
		ld	a, 5
		call	sub_EBEE
		call	loc_D646
		ld	a, 0Ah
		ld	(CursorWidth), a
		call	PrintSmallCursOnMap
		ld	a, 0C7h	; '╟'
		call	PrintHeaderMap
		ld	a, 17h
		ld	hl, 5A60h
		ld	b, 0A0h	; 'а'
		call	Fill_HL_A_Len_B
		ld	hl, 5060h
		call	PrintHorLine
		call	CopyGalaxyToPlanet
		ld	b, 0

loc_EBE3:				; CODE XREF: Cmd_I_GlobalMap+6Fj
		push	bc
		call	PrnStarOnGalaxyMap
		call	NextPlanetData
		pop	bc
		djnz	loc_EBE3
		ret
; End of function Cmd_I_GlobalMap


; =============== S U B	R O U T	I N E =======================================




; === Cmd_O_LocalMap === CODE XREF: RAM:D00Cj
Cmd_O_LocalMap:				; CODE XREF: RAM:D00Cj
		xor	a
		ld	b, 18h
		ld	hl, BufferCrdY

loc_EC5E:				; CODE XREF: RAM:EC60j
		ld	(hl), a
		inc	hl
		djnz	loc_EC5E
		call	ClsAndPrintRamk
		ld	a, 0BEh	; '╛'
		call	PrintHeaderMap
		call	FillAttr_21line
		ld	a, (Pilot_CrdXSystem)
		sub	13h
		call	c, sub_EC3B
		ld	(loc_ECDE+1), a
		ld	a, (Pilot_CrdXSystem)
		add	a, 13h
		call	c, sub_EC3D
		ld	(loc_ECE2+1), a
		ld	a, (Pilot_CrdYSystem)
		sub	22h ; '"'
		call	c, sub_EC3B
		ld	(loc_ECE9+1), a
		ld	a, (Pilot_CrdYSystem)
		add	a, 22h ; '"'
		call	c, sub_EC3D
		ld	(loc_ECED+1), a
		ld	a, 18h
		ld	(byte_E52C), a
		ld	a, 0A7h	; 'з'
		ld	(CrdYDownGalaxyMap), a
		ld	a, 18h
		ld	(CursorWidth), a
		ld	a, 61h ; 'a'
		ld	(PrnCrdXCursor), a
		ld	a, 60h ; '`'
		ld	(PrnCrdYCursor), a
		call	PrintCursorOnScr
		ld	hl, PrnPlanetOnSmallMap
		call	loc_ECD2
		ld	hl, sub_ED78
		call	loc_ECD2
		ld	a, (Pilot_Fuel)
		ld	(RadiusCircle),	a
		call	sub_7119
		ld	a, 10h
		ld	(CursorWidth), a
		jp	PrintSmallCursOnMap
; ---------------------------------------------------------------------------

loc_ECD2:				; CODE XREF: RAM:ECB8p	RAM:ECBEp
		ld	(loc_ECF1+1), hl
		call	CopyGalaxyToPlanet
		ld	b, 0

loc_ECDA:				; CODE XREF: RAM:ECFAj
		push	bc
		ld	a, (DataOfPlanet1+1)

loc_ECDE:				; DATA XREF: RAM:EC75w
		cp	0
		jr	c, loc_ECF6

loc_ECE2:				; DATA XREF: RAM:EC80w
		cp	17h
		jr	nc, loc_ECF6
		ld	a, (DataOfPlanet+1)

loc_ECE9:				; DATA XREF: RAM:EC8Bw
		cp	15h
		jr	c, loc_ECF6

loc_ECED:				; DATA XREF: RAM:EC96w
		cp	59h ; 'Y'
		jr	nc, loc_ECF6

loc_ECF1:				; DATA XREF: RAM:loc_ECD2w
		call	sub_ED78
; ---------------------------------------------------------------------------
		jr	loc_ECF9
; ---------------------------------------------------------------------------

loc_ECF6:				; CODE XREF: RAM:ECE0j	RAM:ECE4j ...
		call	NextPlanetData

loc_ECF9:				; CODE XREF: RAM:ECF4j
		pop	bc
		djnz	loc_ECDA
		ret

; =============== S U B	R O U T	I N E =======================================




; === CompareGalaxyData === CODE XREF: sub_DCE9+132p RAM:EE82p
CompareGalaxyData:			; CODE XREF: sub_DCE9+132p RAM:EE82p
		ld	de, PilotGalaxyData0
		ld	b, 6

loc_EF3C:				; CODE XREF: CompareGalaxyData+Bj
		ld	a, (de)
		cp	(hl)
		jr	nz, loc_EF46
		inc	de
		inc	hl
		djnz	loc_EF3C
		and	a
		ret
; ---------------------------------------------------------------------------

loc_EF46:				; CODE XREF: CompareGalaxyData+7j
		scf
		ret
; End of function CompareGalaxyData


; =============== S U B	R O U T	I N E =======================================




; === ControlKeyIfMapOn === CODE XREF: sub_728D+208p
ControlKeyIfMapOn:			; CODE XREF: sub_728D+208p
					; sub_74C6+15p
		ld	a, (WhatOnScreen)
		cp	7
		ret	c
		cp	9
		ret	nc
		ld	a, (JumpDocking) ; =1 межпланетный прыжок/стыковка
					; =2 межгалактический прыжок
		and	a
		ret	nz
		ld	a, (ControlKeyMove)
		and	0Fh
		ld	c, a
		ld	a, (ControlKeyInMap)
		and	70h ; 'p'
		or	c
		ret	z
		ld	b, 8

loc_7210:				; CODE XREF: ControlKeyIfMapOn+20j
		inc	b
		srl	a
		jr	nc, loc_7210
		ld	a, b
		call	sub_813E
		xor	a
		ld	(ControlKeyInMap), a
		ld	(ControlKeyMove), a
		ret
; End of function ControlKeyIfMapOn

; ---------------------------------------------------------------------------
byte_7221:	db 2			; DATA XREF: Use_Y_Cloackingo
					; Use_Y_Cloacking+Cw

; =============== S U B	R O U T	I N E =======================================


Use_Y_Cloacking:			; CODE XREF: RAM:7509p
		ld	hl, byte_7221
		dec	(hl)
		ld	hl, ControlKey1
		res	7, (hl)
		ret	nz
		ld	a, 2
		ld	(byte_7221), a
		halt
		bit	7, (hl)
		ret	z
		ld	hl, PilotExtWeapon ; 7,=1 - ECM	Jammer
					; 6,=1 - Cloaking device
					; 5,=1 - нельзя	купить товары(в	трюме беженцы или десант)
					; 4
					; 3 =1 3я миссия была выполнена
					; 2 =1 - третья	миссия выполнена
					; 1 =1 - второя	миссия выполнена
					; 0 =1 - первая	миссия выполнена
		bit	6, (hl)
		ret	z
		ld	hl, CloakingDeviceOn
		dec	(hl)
		jr	z, loc_7257
		ld	a, (byte_70FA)
		cp	28h ; '('
		ret	c
		ld	a, 1
		ld	(hl), a
		ld	hl, byte_6069
		res	2, (hl)
		xor	a
		ld	(PlayerPirate),	a

loc_7253:				; CODE XREF: sub_7265+4j
		ld	l, 1111b
		jr	loc_7259
; ---------------------------------------------------------------------------

loc_7257:				; CODE XREF: Use_Y_Cloacking+1Dj
		ld	l, 7

loc_7259:				; CODE XREF: Use_Y_Cloacking+33j
					; sub_725C+6j
		jp	j_SetAtrViewScr
; End of function Use_Y_Cloacking


; =============== S U B	R O U T	I N E =======================================




; === NextGalaxyData === CODE XREF: sub_79F9:loc_7A1Ep
NextGalaxyData:				; CODE XREF: sub_79F9:loc_7A1Ep
		ld	e, (hl)
		rlc	e
		ld	(hl), e
		inc	hl
		ld	d, (hl)
		rlc	d
		ld	(hl), d
		inc	hl
		ret
; End of function NextGalaxyData

; ---------------------------------------------------------------------------
; START	OF FUNCTION CHUNK FOR PrnCountdownJump

loc_DC8E:				; CODE XREF: PrnCountdownJump+23j
		ld	a, (PilotMission1) ; прыжок к другой планете
		and	a
		jr	nz, loc_DC4E	; межгалактический прыжок
		ld	a, (FuelToPlanet)
		ld	b, a
		ld	a, (Pilot_Fuel)
		sub	b
		ld	c, a
		call	SetPilotFuel
		ld	a, 46h ; 'F'
		sub	c
		ld	b, a
		ld	a, (PlanetEconomy)
		sla	a
		add	a, b
		nop
		nop
		nop
		ld	hl, PilotLegalStatus
		srl	(hl)
		ld	a, (FlagsSetUp)
		bit	6, a
		jr	nz, loc_DCC0
		call	loc_8C3C	; шанс перехвата таргонами
		cp	0F9h ; '∙'
		jr	nz, sub_DCE9	; Гиперпрыжок

loc_DCC0:				; CODE XREF: PrnCountdownJump+8Ej
		inc	a
		ld	(InFreeSpace), a
		ld	hl, Pilot_CrdXSystem
		ld	de, Pilot_CrdXTarget
		ld	a, (de)
		add	a, (hl)
		rr	a
		ld	(hl), a
		ld	(de), a
		ld	hl, Pilot_CrdYSystem
		ld	de, Pilot_CrdYTarget
		ld	a, (de)
		add	a, (hl)
		rr	a
		ld	(hl), a
		ld	(de), a
		xor	a
		ld	(CoriolisNear),	a
		ld	(PlanetPresent), a
		ld	(PlanetName), a
		jp	loc_DE4C
; END OF FUNCTION CHUNK	FOR PrnCountdownJump

; =============== S U B	R O U T	I N E =======================================

; Гиперпрыжок



; === PrintHeaderMap === CODE XREF: Cmd_I_GlobalMap+4Fp
PrintHeaderMap:				; CODE XREF: Cmd_I_GlobalMap+4Fp
					; RAM:EC67p
		push	af
		call	SetCrdX_SP
; ---------------------------------------------------------------------------
		db 6
; ---------------------------------------------------------------------------
		pop	af
		call	sub_E306
		ld	a, 17h
		ld	hl, 5820h
		ld	b, 40h ; '@'
		call	Fill_HL_A_Len_B
		ld	hl, 4740h
		jp	PrintHorLine
; End of function PrintHeaderMap


; =============== S U B	R O U T	I N E =======================================


Cmd_I_GlobalMap:			; CODE XREF: RAM:D009j
		call	ClsAndPrintRamk
		ld	a, 18h
		ld	(byte_E52C), a
		ld	a, 98h ; 'Ш'
		ld	(CrdYDownGalaxyMap), a
		ld	a, 10h
		ld	(CursorWidth), a
		ld	a, (Pilot_CrdXSystem)
		ld	(PrnCrdXCursor), a
		ld	a, (Pilot_CrdYSystem)
		srl	a
		add	a, 18h
		ld	(PrnCrdYCursor), a
		call	PrintCursorOnScr
		ld	a, (Pilot_Fuel)
		srl	a
		srl	a
		ld	(RadiusCircle),	a
		call	sub_7119
		ld	hl, 4060h
		ld	a, 2
		call	sub_EBEE
		ld	hl, 5100h
		ld	a, 5
		call	sub_EBEE
		call	loc_D646
		ld	a, 0Ah
		ld	(CursorWidth), a
		call	PrintSmallCursOnMap
		ld	a, 0C7h	; '╟'
		call	PrintHeaderMap
		ld	a, 17h
		ld	hl, 5A60h
		ld	b, 0A0h	; 'а'
		call	Fill_HL_A_Len_B
		ld	hl, 5060h
		call	PrintHorLine
		call	CopyGalaxyToPlanet
		ld	b, 0

loc_EBE3:				; CODE XREF: Cmd_I_GlobalMap+6Fj
		push	bc
		call	PrnStarOnGalaxyMap
		call	NextPlanetData
		pop	bc
		djnz	loc_EBE3
		ret
; End of function Cmd_I_GlobalMap


; =============== S U B	R O U T	I N E =======================================




; === PrintSmallCursOnMap === CODE XREF: sub_DF24+2Bp sub_DF24+31p ...
PrintSmallCursOnMap:			; CODE XREF: sub_DF24+2Bp sub_DF24+31p ...

; FUNCTION CHUNK AT E63F SIZE 00000064 BYTES

		ld	a, (CursorWidth)
		cp	0Ah
		jr	z, loc_E4C1
		ld	a, (Pilot_CrdXSystem)
		ld	b, a
		ld	a, (Pilot_CrdXTarget)
		sub	b
		jr	c, loc_E48B
		add	a, a
		ret	c
		add	a, a
		ret	c
		ld	b, a
		ld	a, 61h ; 'a'
		add	a, b
		ret	c
		jr	loc_E496
; ---------------------------------------------------------------------------

loc_E48B:				; CODE XREF: PrintSmallCursOnMap+Fj
		neg
		add	a, a
		ret	c
		add	a, a
		ret	c
		ld	b, a
		ld	a, 61h ; 'a'
		sub	b
		ret	c

loc_E496:				; CODE XREF: PrintSmallCursOnMap+1Aj
		ld	(PrnCrdXCursor), a
		ld	a, (Pilot_CrdYSystem)
		ld	b, a
		ld	a, (Pilot_CrdYTarget)
		sub	b
		jr	c, loc_E4AC
		add	a, a
		ret	c
		ld	b, a
		ld	a, 60h ; '`'
		add	a, b
		ret	c
		jr	loc_E4B6
; ---------------------------------------------------------------------------

loc_E4AC:				; CODE XREF: PrintSmallCursOnMap+32j
		neg
		sla	a
		ret	c
		ld	b, a
		ld	a, 60h ; '`'
		sub	b
		ret	c

loc_E4B6:				; CODE XREF: PrintSmallCursOnMap+3Bj
		cp	19h
		ret	c
		cp	0A7h ; 'з'
		ret	nc
		ld	(PrnCrdYCursor), a
		jr	loc_E4D1
; ---------------------------------------------------------------------------

loc_E4C1:				; CODE XREF: PrintSmallCursOnMap+5j
		ld	a, (Pilot_CrdXTarget)
		ld	(PrnCrdXCursor), a
		ld	a, (Pilot_CrdYTarget)
		srl	a
		add	a, 18h
		ld	(PrnCrdYCursor), a

loc_E4D1:				; CODE XREF: PrintSmallCursOnMap+50j
		jp	PrintCursorOnScr
; ---------------------------------------------------------------------------

loc_E4D4:				; CODE XREF: RAM:D024j
		call	PrintSmallCursOnMap
		ld	a, (Pilot_CrdYTarget)
		dec	a
		dec	a
		cp	0FEh ; '■'
		jr	c, loc_E4E1
		xor	a

loc_E4E1:				; CODE XREF: PrintSmallCursOnMap+6Fj
		ld	(Pilot_CrdYTarget), a
		jr	loc_E506
; ---------------------------------------------------------------------------

loc_E4E6:				; CODE XREF: sub_D021j
		call	PrintSmallCursOnMap
		ld	a, (Pilot_CrdYTarget)
		inc	a
		inc	a
		cp	2
		jr	nc, loc_E4F4
		ld	a, 0FFh

loc_E4F4:				; CODE XREF: PrintSmallCursOnMap+81j
		ld	(Pilot_CrdYTarget), a
		jr	loc_E506
; ---------------------------------------------------------------------------

loc_E4F9:				; CODE XREF: RAM:D01Bj
		call	PrintSmallCursOnMap
		ld	a, (Pilot_CrdXTarget)
		dec	a
		jr	nz, loc_E503
		inc	a

loc_E503:				; CODE XREF: PrintSmallCursOnMap+91j
		ld	(Pilot_CrdXTarget), a

loc_E506:				; CODE XREF: PrintSmallCursOnMap+75j
					; PrintSmallCursOnMap+88j ...
		jp	PrintSmallCursOnMap
; ---------------------------------------------------------------------------

loc_E509:				; CODE XREF: RAM:D01Ej
		call	PrintSmallCursOnMap
		ld	a, (Pilot_CrdXTarget)
		inc	a
		cp	0FFh
		jr	nz, loc_E515
		dec	a

loc_E515:				; CODE XREF: PrintSmallCursOnMap+A3j
		ld	(Pilot_CrdXTarget), a
		jr	loc_E506
; ---------------------------------------------------------------------------

loc_E51A:				; CODE XREF: RAM:D027j
		call	PrintSmallCursOnMap
		ld	a, (Pilot_CrdXSystem)
		ld	(Pilot_CrdXTarget), a
		ld	a, (Pilot_CrdYSystem)
		ld	(Pilot_CrdYTarget), a
		jr	loc_E506
; ---------------------------------------------------------------------------


; === PrnStarOnGalaxyMap === CODE XREF: Cmd_I_GlobalMap+68p
PrnStarOnGalaxyMap:			; CODE XREF: Cmd_I_GlobalMap+68p
		ld	a, (DataOfPlanet+1)
		srl	a
		xor	7Fh ; ''
		add	a, 18h
		ld	b, a
		ld	a, (DataOfPlanet1+1)
		ld	c, a
		call	22AAh
		xor	7
		sla	a
		sla	a
		sla	a
		or	0C0h ; '└'
		ld	de,  loc_EC35+1
		ld	(de), a
		ld	b, 0

loc_EC35:				; DATA XREF: PrnStarOnGalaxyMap+1Bo
		set	7, b
		ld	a, (hl)
		xor	b
		ld	(hl), a
		ret
; End of function PrnStarOnGalaxyMap


; =============== S U B	R O U T	I N E =======================================




