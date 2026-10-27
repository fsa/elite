; 01_start_init - разобрано из kladov.asm
; Количество подпрограмм: 5

; === InitGame === CODE XREF: j_InitGamej
InitGame:				; CODE XREF: j_InitGamej
		call	ResetBattleMessages
		ld	hl, aJameson_1	; "JAMESON   "
		ld	de, PilotName	; "JAMESON   "
		ld	bc, 5Dh	; ']'
		ldir
		ld	a, (Pilot_CrdXSystem)
		ld	(Pilot_CrdXTarget), a
		ld	a, (Pilot_CrdYSystem)
		ld	(Pilot_CrdYTarget), a
		call	SearchPlanetNearCursor
		ld	hl, Amount
		ld	(hl), 0
		push	hl
		call	GenerateAnotherPrice
		pop	hl
		ld	(hl), 77h ; 'w'
		ld	hl, PlanetName
		ld	de, PlanetPresent
		ld	bc, 8
		ldir
		ld	hl, unk_D28F
		ld	de, SunCrd
		ld	bc, 9
		ldir
		xor	a
		ld	(JumpCountdown), a
		ret
; End of function InitGame

; ---------------------------------------------------------------------------
word_D2DC:	dw 0			; DATA XREF: RAM:D2F0w	RAM:loc_D31Dr ...

; =============== S U B	R O U T	I N E =======================================




; === ResetBattleMessages === CODE XREF: j_ResetBattleMessagesj
ResetBattleMessages:			; CODE XREF: j_ResetBattleMessagesj
					; InitGamep ...
		ld	b, 0Ah
		ld	hl, MissileJammed

loc_DB96:				; CODE XREF: ResetBattleMessages+9j
		res	7, (hl)
		inc	hl
		inc	hl
		djnz	loc_DB96
		xor	a
		ld	(msgNumber), a
		ret
; End of function ResetBattleMessages

; ---------------------------------------------------------------------------


; === SearchPlanetNearCursor === CODE XREF: sub_5FD5+3Ap InitGame+1Ap ...
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




; === j_InitGame === CODE XREF: sub_728D+178p
j_InitGame:				; CODE XREF: sub_728D+178p
		jp	InitGame
; End of function j_InitGame

; [00000003 BYTES: COLLAPSED FUNCTION j_ClsAndPrintRamk. PRESS KEYPAD "+" TO EXPAND]
; [00000003 BYTES: COLLAPSED FUNCTION j_PrintMessage2. PRESS KEYPAD "+" TO EXPAND]

; =============== S U B	R O U T	I N E =======================================

; Attributes: thunk



; === j_ResetBattleMessages === CODE XREF: sub_9946+3p
j_ResetBattleMessages:			; CODE XREF: sub_9946+3p
		jp	ResetBattleMessages
; End of function j_ResetBattleMessages

; ---------------------------------------------------------------------------
; START	OF FUNCTION CHUNK FOR GoMission3

GiveMission3:				; CODE XREF: GoMission3:loc_727Fj
		jp	loc_D123
; END OF FUNCTION CHUNK	FOR GoMission3
; ---------------------------------------------------------------------------
		db 0C3h	; ├
		db  97h	; Ч
		db 0DEh	; ▐


