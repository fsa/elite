; kladov/10_start_init.asm
; Старт/инициализация игры (по kladov.asm)

; === j_InitGame (0x60C3?) ===
; см. kladov.asm:24786

; === InitGame (0x6378?) ===
; см. kladov.asm:25452
InitGame:
		call	ResetBattleMessages
		ld	hl, aJameson_1		; "JAMESON   "
		ld	de, PilotName
		ld	bc, 5Dh
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
		ld	(hl), 77h
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

; === ResetBattleMessages ===
; === SearchPlanetNearCursor ===
; === GenerateAnotherPrice ===
