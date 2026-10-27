; 13_hyperspace - разобрано из kladov.asm
; Количество подпрограмм: 7

; === CalcFuelForJump === CODE XREF: sub_DF24+37j
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




; === CounterShipHyperJump === CODE XREF: sub_9D40:loc_9D50p
CounterShipHyperJump:			; CODE XREF: sub_9D40:loc_9D50p
		ld	a, (FrameCounter)
		ld	c, a
		ld	hl, (word_B452)
		ld	a, (hl)
		and	0F8h ; '°'
		cp	0A0h ; 'а'
		ret	nz
		inc	(hl)
		ld	a, (hl)
		and	7
		ret	nz
		ld	(hl), a
		ld	(ix+22h), a
		pop	hl
		push	bc
		call	OffTargetMissiles
		pop	bc
		jp	loc_9E5C
; End of function CounterShipHyperJump


; =============== S U B	R O U T	I N E =======================================

; поиск	объекта	в дополнительном слоте
; hl - адрес слота объекта
; Attributes: noreturn

FindObjInExtSlot:			; CODE XREF: sub_B71C+2p sub_B71C+16p	...
		ld	ix, byte_B457	; 5,+0 =0 - объектом был запущен корабль
					; +1,+2,+3 - печатные координаты вершины с лазером
					; +1,+2,+3 - печатные координаты вершины №3 xor	3
					; +8 биты 0-3,=0 можно увеличить рейтинг
		ld	a, 48h ; 'H'

loc_7ECE:				; CODE XREF: FindObjInExtSlot+11j
		cp	l
		ret	z		; выход	если станция
		ld	de, 0FFD9h
		add	hl, de
		ld	de, 0Ah
		add	ix, de
		jr	loc_7ECE
; End of function FindObjInExtSlot

; ---------------------------------------------------------------------------
		db 0F3h	; є
		db 0F3h	; є


; === GoJump === CODE XREF: j_GoJumpj
GoJump:					; CODE XREF: j_GoJumpj
		ld	hl, ControlKey2
		ld	a, (hl)
		and	30h ; '0'
		ret	z
		ld	b, a
		ld	a, (hl)
		and	0CFh ; '╧'
		ld	(hl), a
		ld	a, (JumpCountdown)
		and	a
		ret	nz
		ld	a, (IndDanger)
		and	a
		jr	z, JumpDocked
		bit	5, b
		jr	z, loc_DBCF
		bit	4, b
		ret	z
		ld	a, (PilotGyperdrive)
		and	a
		ret	z
		ld	a, 2
		ld	(JumpDocking), a ; =1 межпланетный прыжок/стыковка
					; =2 межгалактический прыжок
		ld	bc, 7406h
		jr	loc_DBEB
; ---------------------------------------------------------------------------

loc_DBCF:				; CODE XREF: GoJump+19j
		call	SearchNearPlanet
		ld	hl, (FuelToPlanet)
		ld	a, h
		and	a
		jr	nz, loc_DC04
		ld	a, l
		and	a
		ret	z
		ld	b, a
		ld	a, (Pilot_Fuel)
		sub	b
		jr	c, loc_DC04
		ld	a, 1
		ld	(JumpDocking), a ; =1 межпланетный прыжок/стыковка
					; =2 межгалактический прыжок
		ld	bc, 0C005h

loc_DBEB:				; CODE XREF: GoJump+2Bj
		ld	a, 15h
		ld	(JumpCountdown), a
		ld	(msgTimeLife), a
		call	sub_DAE6
		call	sub_E1E8
; ---------------------------------------------------------------------------
		db  3Ah	; :
		db 0E3h	; у
		db  70h	; p
		db 0FEh	; ■
		db    4
		db 0D0h	; ╨
		db 0AFh	; п
		db  32h	; 2
		db 0E3h	; у
		db  70h	; p
		db 0C9h	; ╔
; ---------------------------------------------------------------------------

loc_DC04:				; CODE XREF: GoJump+35j GoJump+3Fj
		ld	bc, byte_C106
		jp	loc_DADC
; ---------------------------------------------------------------------------

JumpDocked:				; CODE XREF: GoJump+15j
		call	ClsAndPrintRamk
		ld	hl, 70Bh
		ld	(CrdXPrnSymbol), hl
		ld	a, 1
		ld	(FlgPrnSym1), a
		ld	a, 0CDh	; '═'
		call	PrintMessage2
		call	sub_E1E8
; End of function GoJump

; ---------------------------------------------------------------------------
		db 0CDh	; ═
		db  5Eh	; ^
		db 0E3h	; у
		db 0CDh	; ═
		db    5
		db 0E2h	; т
		db 0C3h	; ├
		db  87h	; З
		db 0EFh	; я

; =============== S U B	R O U T	I N E =======================================


PrnCountdownJump:			; CODE XREF: sub_DAB4p

; FUNCTION CHUNK AT DC8E SIZE 0000005B BYTES
; FUNCTION CHUNK AT DE4C SIZE 0000001B BYTES

		ld	hl, JumpCountdown
		ld	a, (hl)
		and	a
		ret	z
		dec	(hl)
		jr	z, HyperspaceBegin
		ld	b, a
		ld	a, 2
		ld	(byte_D0CE), a
		ld	hl, 101h
		ld	(CrdXPrnSymbol), hl
		ld	a, b
		srl	a
		jp	PrintNumber_A
; ---------------------------------------------------------------------------

HyperspaceBegin:			; CODE XREF: PrnCountdownJump+7j
		call	ResetBattleMessages
		ld	a, (JumpDocking) ; =1 межпланетный прыжок/стыковка
					; =2 межгалактический прыжок
		sub	2
		jr	nz, loc_DC8E

loc_DC4E:				; CODE XREF: PrnCountdownJump+69j
		xor	a		; межгалактический прыжок
		ld	(PilotMission3), a ; =#06 при стыковке будет захват таргонами (вроде нереальная	ситуация)
					; =#04 в системе со танцией таргонов во	время войны
		ld	a, (PilotMission1)
		and	a
		jr	z, loc_DC5F
		xor	a
		ld	(PilotMission1), a
		ld	(PilotMission1_1), a

loc_DC5F:				; CODE XREF: PrnCountdownJump+2Dj
		call	sub_5D70
		ld	hl, PilotNumGalaxy
		push	bc
		push	de
		push	af
		call	sub_79F9	; опрос	клвиши во время	подготовки к прыжку
		pop	af
		pop	de
		pop	bc
		ld	a, (PilotNumGalaxy)
		ld	(byte_7060), a
		and	7
		ld	(PilotNumGalaxy), a
		nop
		xor	a
		ld	(PilotLegalStatus), a
		call	SearchNearPlanet
		jr	sub_DCE9	; Гиперпрыжок
; End of function PrnCountdownJump


; =============== S U B	R O U T	I N E =======================================


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



; === HyperspaceBegin === CODE XREF: PrnCountdownJump+7j
HyperspaceBegin:			; CODE XREF: PrnCountdownJump+7j
		call	ResetBattleMessages
		ld	a, (JumpDocking) ; =1 межпланетный прыжок/стыковка
					; =2 межгалактический прыжок
		sub	2
		jr	nz, loc_DC8E

loc_DC4E:				; CODE XREF: PrnCountdownJump+69j
		xor	a		; межгалактический прыжок
		ld	(PilotMission3), a ; =#06 при стыковке будет захват таргонами (вроде нереальная	ситуация)
					; =#04 в системе со танцией таргонов во	время войны
		ld	a, (PilotMission1)
		and	a
		jr	z, loc_DC5F
		xor	a
		ld	(PilotMission1), a
		ld	(PilotMission1_1), a

loc_DC5F:				; CODE XREF: PrnCountdownJump+2Dj
		call	sub_5D70
		ld	hl, PilotNumGalaxy
		push	bc
		push	de
		push	af
		call	sub_79F9	; опрос	клвиши во время	подготовки к прыжку
		pop	af
		pop	de
		pop	bc
		ld	a, (PilotNumGalaxy)
		ld	(byte_7060), a
		and	7
		ld	(PilotNumGalaxy), a
		nop
		xor	a
		ld	(PilotLegalStatus), a
		call	SearchNearPlanet
		jr	sub_DCE9	; Гиперпрыжок
; End of function PrnCountdownJump


; =============== S U B	R O U T	I N E =======================================


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



; === PrnCountdownJump === CODE XREF: sub_DAB4p
PrnCountdownJump:			; CODE XREF: sub_DAB4p

; FUNCTION CHUNK AT DC8E SIZE 0000005B BYTES
; FUNCTION CHUNK AT DE4C SIZE 0000001B BYTES

		ld	hl, JumpCountdown
		ld	a, (hl)
		and	a
		ret	z
		dec	(hl)
		jr	z, HyperspaceBegin
		ld	b, a
		ld	a, 2
		ld	(byte_D0CE), a
		ld	hl, 101h
		ld	(CrdXPrnSymbol), hl
		ld	a, b
		srl	a
		jp	PrintNumber_A
; ---------------------------------------------------------------------------

HyperspaceBegin:			; CODE XREF: PrnCountdownJump+7j
		call	ResetBattleMessages
		ld	a, (JumpDocking) ; =1 межпланетный прыжок/стыковка
					; =2 межгалактический прыжок
		sub	2
		jr	nz, loc_DC8E

loc_DC4E:				; CODE XREF: PrnCountdownJump+69j
		xor	a		; межгалактический прыжок
		ld	(PilotMission3), a ; =#06 при стыковке будет захват таргонами (вроде нереальная	ситуация)
					; =#04 в системе со танцией таргонов во	время войны
		ld	a, (PilotMission1)
		and	a
		jr	z, loc_DC5F
		xor	a
		ld	(PilotMission1), a
		ld	(PilotMission1_1), a

loc_DC5F:				; CODE XREF: PrnCountdownJump+2Dj
		call	sub_5D70
		ld	hl, PilotNumGalaxy
		push	bc
		push	de
		push	af
		call	sub_79F9	; опрос	клвиши во время	подготовки к прыжку
		pop	af
		pop	de
		pop	bc
		ld	a, (PilotNumGalaxy)
		ld	(byte_7060), a
		and	7
		ld	(PilotNumGalaxy), a
		nop
		xor	a
		ld	(PilotLegalStatus), a
		call	SearchNearPlanet
		jr	sub_DCE9	; Гиперпрыжок
; End of function PrnCountdownJump


; =============== S U B	R O U T	I N E =======================================


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



; === ShipHyperJump === CODE XREF: sub_7E02j
ShipHyperJump:				; CODE XREF: sub_7E02j
		ld	hl, (word_B452)
		ld	a, (hl)
		and	0F8h ; '°'
		cp	0A0h ; 'а'
		jr	z, loc_7E4E	; уход в гипертоннель
		bit	3, a
		jp	z, sub_A7D9	; если включена	невидимость и это ASP из 2й миссии, то печать только луча лазера
		call	GetCrdsProjection ; получение координат	и знаков проекции объекта
		call	SetCrdCDEinExtSlot
		inc	hl
		ld	a, (ix+20h)
		and	a
		jr	z, loc_7E9D
		ld	a, (ix+22h)
		ld	e, 78h ; 'x'
		cp	e
		jr	nc, loc_7E38
		ld	e, 38h ; '8'
		cp	e
		jr	nc, loc_7E38
		ld	e, 0Ch

loc_7E38:				; CODE XREF: sub_7E02+2Dj sub_7E02+32j
		ld	a, (hl)

loc_7E39:				; CODE XREF: sub_7E02+3Cj
		rrc	e
		jr	c, loc_7E40
		rrca
		jr	loc_7E39
; ---------------------------------------------------------------------------

loc_7E40:				; CODE XREF: sub_7E02+39j
		rrca
		inc	(ix+22h)
		jr	c, loc_7E9D
		dec	(ix+22h)
		dec	(ix+22h)
		jr	loc_7EA6
; ---------------------------------------------------------------------------

loc_7E4E:				; CODE XREF: sub_7E02+13j
		ld	a, (hl)
		and	111b
		ld	b, a
		ld	a, (RadiusBlast)
		sub	b
		jr	c, loc_7E5C
		cp	4
		jr	nc, loc_7E5E

loc_7E5C:				; CODE XREF: sub_7E02+54j
		ld	a, 4

loc_7E5E:				; CODE XREF: sub_7E02+58j
		ld	h, a
		ld	a, (WhatOnScreen)
		rrca
		ld	a, (CrdXYsign)
		jr	c, loc_7E6A
		xor	2

loc_7E6A:				; CODE XREF: sub_7E02+64j
		ld	c, a
		xor	a
		rr	c
		rla
		rr	c
		rla
		xor	0Eh
		ld	l, a
		ld	(CircleFlg), hl
		ld	hl, (CrdXProjection)
		ex	de, hl
		ld	l, d
		ld	h, e
		ld	(CircleCrdY), hl
		ld	c, b
		ld	a, b
		bit	2, a
		jr	z, loc_7E89
		xor	7

loc_7E89:				; CODE XREF: sub_7E02+83j
		ld	b, a
		inc	b
		inc	b

loc_7E8C:				; CODE XREF: sub_7E02+96j
		push	bc
		call	PrnCircle_NoFill
		ld	hl, CircleRadius
		ld	a, (hl)
		pop	bc
		add	a, b
		add	a, b
		ld	(hl), a
		djnz	loc_7E8C
		ld	a, c
		cp	3

loc_7E9D:				; CODE XREF: sub_7E02+25j sub_7E02+42j
		jp	c, sub_A7D9	; если включена	невидимость и это ASP из 2й миссии, то печать только луча лазера
		call	GetCrdsProjection ; получение координат	и знаков проекции объекта
		call	SetCrdCDEinExtSlot

loc_7EA6:				; CODE XREF: sub_7E02+4Aj
		jp	loc_A878
; End of function sub_7E02


; =============== S U B	R O U T	I N E =======================================


CounterShipHyperJump:			; CODE XREF: sub_9D40:loc_9D50p
		ld	a, (FrameCounter)
		ld	c, a
		ld	hl, (word_B452)
		ld	a, (hl)
		and	0F8h ; '°'
		cp	0A0h ; 'а'
		ret	nz
		inc	(hl)
		ld	a, (hl)
		and	7
		ret	nz
		ld	(hl), a
		ld	(ix+22h), a
		pop	hl
		push	bc
		call	OffTargetMissiles
		pop	bc
		jp	loc_9E5C
; End of function CounterShipHyperJump


; =============== S U B	R O U T	I N E =======================================

; поиск	объекта	в дополнительном слоте
; hl - адрес слота объекта
; Attributes: noreturn

FindObjInExtSlot:			; CODE XREF: sub_B71C+2p sub_B71C+16p	...
		ld	ix, byte_B457	; 5,+0 =0 - объектом был запущен корабль
					; +1,+2,+3 - печатные координаты вершины с лазером
					; +1,+2,+3 - печатные координаты вершины №3 xor	3
					; +8 биты 0-3,=0 можно увеличить рейтинг
		ld	a, 48h ; 'H'

loc_7ECE:				; CODE XREF: FindObjInExtSlot+11j
		cp	l
		ret	z		; выход	если станция
		ld	de, 0FFD9h
		add	hl, de
		ld	de, 0Ah
		add	ix, de
		jr	loc_7ECE
; End of function FindObjInExtSlot

; ---------------------------------------------------------------------------
		db 0F3h	; є
		db 0F3h	; є


; === j_GoJump === CODE XREF: sub_728D:loc_748Cp
j_GoJump:				; CODE XREF: sub_728D:loc_748Cp
		jp	GoJump
; End of function j_GoJump

; ---------------------------------------------------------------------------


