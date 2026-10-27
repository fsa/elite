;27750: 		db 0EFh	; я
;27751: 
;27752: ; =============== S U B	R O U T	I N E =======================================
;27753: 
;27754: 
;27755: PrnCountdownJump:			; CODE XREF: sub_DAB4p
;27756: 
;27757: ; FUNCTION CHUNK AT DC8E SIZE 0000005B BYTES
;27758: ; FUNCTION CHUNK AT DE4C SIZE 0000001B BYTES
;27759: 
;27760: 		ld	hl, JumpCountdown
;27761: 		ld	a, (hl)
;27762: 		and	a
;27763: 		ret	z
;27764: 		dec	(hl)
;27765: 		jr	z, HyperspaceBegin
;27766: 		ld	b, a
;27767: 		ld	a, 2
;27768: 		ld	(byte_D0CE), a
;27769: 		ld	hl, 101h
;27770: 		ld	(CrdXPrnSymbol), hl
;27771: 		ld	a, b
;27772: 		srl	a
;27773: 		jp	PrintNumber_A
;27774: ; ---------------------------------------------------------------------------
;27775: 
;27776: HyperspaceBegin:			; CODE XREF: PrnCountdownJump+7j
;27777: 		call	ResetBattleMessages
;27778: 		ld	a, (JumpDocking) ; =1 межпланетный прыжок/стыковка
;27779: 					; =2 межгалактический прыжок
;27780: 		sub	2
;27781: 		jr	nz, loc_DC8E
;27782: 
;27783: loc_DC4E:				; CODE XREF: PrnCountdownJump+69j
;27784: 		xor	a		; межгалактический прыжок
;27785: 		ld	(PilotMission3), a ; =#06 при стыковке будет захват таргонами (вроде нереальная	ситуация)
;27786: 					; =#04 в системе со танцией таргонов во	время войны
;27787: 		ld	a, (PilotMission1)
;27788: 		and	a
;27789: 		jr	z, loc_DC5F
;27790: 		xor	a
;27791: 		ld	(PilotMission1), a
;27792: 		ld	(PilotMission1_1), a
;27793: 
;27794: loc_DC5F:				; CODE XREF: PrnCountdownJump+2Dj
;27795: 		call	sub_5D70
;27796: 		ld	hl, PilotNumGalaxy
;27797: 		push	bc
;27798: 		push	de
;27799: 		push	af
;27800: 		call	sub_79F9	; опрос	клвиши во время	подготовки к прыжку
;27801: 		pop	af
;27802: 		pop	de
;27803: 		pop	bc
;27804: 		ld	a, (PilotNumGalaxy)
;27805: 		ld	(byte_7060), a
;27806: 		and	7
;27807: 		ld	(PilotNumGalaxy), a
;27808: 		nop
;27809: 		xor	a
;27810: 		ld	(PilotLegalStatus), a
;27811: 		call	SearchNearPlanet
;27812: 		jr	sub_DCE9	; Гиперпрыжок
;27813: ; End of function PrnCountdownJump
;27814: 
;27815: 
;27816: ; =============== S U B	R O U T	I N E =======================================
;27817: 
;27818: 
;27819: NextGalaxyData:				; CODE XREF: sub_79F9:loc_7A1Ep
;27820: 		ld	e, (hl)
;27821: 		rlc	e
;27822: 		ld	(hl), e
;27823: 		inc	hl
;27824: 		ld	d, (hl)
;27825: 		rlc	d
;27826: 		ld	(hl), d
;27827: 		inc	hl
;27828: 		ret
;27829: ; End of function NextGalaxyData
;27830: 
;27831: ; ---------------------------------------------------------------------------
;27832: ; START	OF FUNCTION CHUNK FOR PrnCountdownJump
;27833: 
;27834: loc_DC8E:				; CODE XREF: PrnCountdownJump+23j
;27835: 		ld	a, (PilotMission1) ; прыжок к другой планете
;27836: 		and	a
;27837: 		jr	nz, loc_DC4E	; межгалактический прыжок
;27838: 		ld	a, (FuelToPlanet)
;27839: 		ld	b, a
;27840: 		ld	a, (Pilot_Fuel)
;27841: 		sub	b
;27842: 		ld	c, a
;27843: 		call	SetPilotFuel
;27844: 		ld	a, 46h ; 'F'
;27845: 		sub	c
;27846: 		ld	b, a
;27847: 		ld	a, (PlanetEconomy)
;27848: 		sla	a
;27849: 		add	a, b
;27850: 		nop
;27851: 		nop
;27852: 		nop
;27853: 		ld	hl, PilotLegalStatus
;27854: 		srl	(hl)
;27855: 		ld	a, (FlagsSetUp)
;27856: 		bit	6, a
;27857: 		jr	nz, loc_DCC0
;27858: 		call	loc_8C3C	; шанс перехвата таргонами
;27859: 		cp	0F9h ; '∙'
;27860: 		jr	nz, sub_DCE9	; Гиперпрыжок
;27861: 
;27862: loc_DCC0:				; CODE XREF: PrnCountdownJump+8Ej
;27863: 		inc	a
;27864: 		ld	(InFreeSpace), a
;27865: 		ld	hl, Pilot_CrdXSystem
;27866: 		ld	de, Pilot_CrdXTarget
;27867: 		ld	a, (de)
;27868: 		add	a, (hl)
;27869: 		rr	a
;27870: 		ld	(hl), a
;27871: 		ld	(de), a
;27872: 		ld	hl, Pilot_CrdYSystem
;27873: 		ld	de, Pilot_CrdYTarget
;27874: 		ld	a, (de)
;27875: 		add	a, (hl)
;27876: 		rr	a
;27877: 		ld	(hl), a
;27878: 		ld	(de), a
;27879: 		xor	a
;27880: 		ld	(CoriolisNear),	a
;27881: 		ld	(PlanetPresent), a
;27882: 		ld	(PlanetName), a
;27883: 		jp	loc_DE4C
;27884: ; END OF FUNCTION CHUNK	FOR PrnCountdownJump
;27885: 
;27886: ; =============== S U B	R O U T	I N E =======================================
;27887: 
;27888: ; Гиперпрыжок
;27889: 
;27890: sub_DCE9:				; CODE XREF: sub_5FD5+4Dp
;27891: 					; PrnCountdownJump+58j	...
;27892: 
;27893: ; FUNCTION CHUNK AT DDF7 SIZE 00000055 BYTES
;27894: ; FUNCTION CHUNK AT DE6F SIZE 00000028 BYTES
;27895: 
;27896: 		xor	a
;27897: 		ld	(InFreeSpace), a
;27898: 		call	sub_D8DD
;27899: 		ld	de, PilotPlanetData
;27900: 		ld	bc, 6
