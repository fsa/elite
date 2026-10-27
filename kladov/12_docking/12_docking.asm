; 12_docking - разобрано из kladov.asm
; Количество подпрограмм: 5

; === GoDocking === CODE XREF: sub_728D+223p
GoDocking:				; CODE XREF: sub_728D+223p
		ld	a, (ManualDocking)
		and	a
		jr	z, loc_7187
		xor	a
		ld	(ManualDocking), a
		jr	loc_71B3
; ---------------------------------------------------------------------------

loc_7187:				; CODE XREF: GoDocking+4j
		ld	a, (ControlKey3)
		ld	b, a
		and	80h ; 'А'
		res	7, b
		ld	a, b
		ld	(ControlKey3), a
		ret	z
		ld	a, (CoriolisNear)
		and	a
		ret	z
		ld	a, (PilotDocComputer)
		and	a
		ret	z
		ld	a, (IndDanger)
		and	a
		ret	z
		ld	a, (JumpDocking) ; =1 межпланетный прыжок/стыковка
					; =2 межгалактический прыжок
		and	a
		ret	nz
		ld	hl, byte_6069	; objCoriolis+oFlg_2
		bit	2, (hl)
		ret	nz
		ld	a, 1
		ld	(AutoDocking), a

loc_71B3:				; CODE XREF: GoDocking+Aj
		xor	a
		ld	(WhatOnScreen),	a
		call	PrnCocpit_Ifn

loc_71BA:				; CODE XREF: UseEscapePod+9j
		call	NewJumpDockingCrdObj
		jp	LaunchDocking
; End of function GoDocking


; =============== S U B	R O U T	I N E =======================================


NewJumpDockingCrdObj:			; CODE XREF: GoDocking:loc_71BAp
		ld	iy, SunCrd
		ld	ix, objCoriolis
		call	sub_71CE
		call	sub_71CE
; End of function NewJumpDockingCrdObj


; =============== S U B	R O U T	I N E =======================================




; === JumpDocked === CODE XREF: GoJump+15j
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



; === LaunchDocking === CODE XREF: GoDocking+42j
LaunchDocking:				; CODE XREF: GoDocking+42j
					; sub_80D3+3Cp

; FUNCTION CHUNK AT 5E55 SIZE 00000013 BYTES

		ld	a, (InFreeSpace)
		and	a
		ret	nz
		ld	hl, unk_93E3
		ld	(loc_92F9+1), hl
		ld	a, 8
		ld	(byte_93D0), a
		ld	a, 30h ; '0'
		ld	(loc_9325+1), a
		ld	a, 40h ; '@'
		ld	(byte_93D2), a
		ld	hl, 101h
		ld	(JumpDocking), hl ; =1 межпланетный прыжок/стыковка
					; =2 межгалактический прыжок
		ld	a, (IndDanger)
		and	a
		jr	z, BeginLaunch



; === PrnDangerEcmDock === CODE XREF: Process_MissBombEPod+6p
PrnDangerEcmDock:			; CODE XREF: Process_MissBombEPod+6p
		ld	a, (WhatOnScreen)
		cp	4
		ret	nc
		ld	hl, 5AD5h
		ld	a, (TimeWorkECM)
		call	sub_9B14
		inc	hl
		ld	a, (CoriolisNear)
; End of function PrnDangerEcmDock


; =============== S U B	R O U T	I N E =======================================




; === PrnDockingTonnel === CODE XREF: LaunchDocking+46p
PrnDockingTonnel:			; CODE XREF: LaunchDocking+46p
					; LaunchDocking+4Fp ...
		call	j_PrnTxtOnViewScr
		ld	a, (JumpDocking) ; =1 межпланетный прыжок/стыковка
					; =2 межгалактический прыжок
		and	a
		ret	z
		jr	loc_92D6
; End of function PrnDockingTonnel


; =============== S U B	R O U T	I N E =======================================

; Attributes: noreturn



