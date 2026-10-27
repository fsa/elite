; 03_game_loop - разобрано из kladov.asm
; Количество подпрограмм: 2

; === GameMain === CODE XREF: sub_728D+197j
GameMain:				; CODE XREF: sub_728D+197j
					; sub_728D+210j ...
		ld	a, (BlockControl)
		and	a
		jr	nz, loc_747A
		call	CalcAngSpeed4Prn

loc_747A:				; CODE XREF: sub_728D+1E8j
		call	sub_80D3	; проверка нажатия (и соответствующий переход) клавиш 1, 2, 3, 4, I, O,	P, K, L, enter
		call	ScanKeyInPause
		jr	nc, loc_748C
		xor	a
		ld	(WhatOnScreen),	a
		call	PrnCocpit_Ifn
		jp	loc_739E
; ---------------------------------------------------------------------------

loc_748C:				; CODE XREF: sub_728D+1F3j
		call	j_GoJump
		ld	a, (IndDanger)
		and	a
		jr	nz, loc_74AA
		call	ControlKeyIfMapOn
		ld	a, (ControlKeyMove)
		bit	5, a
		jr	z, GameMain
		call	MenuSaveLoad
; END OF FUNCTION CHUNK	FOR sub_728D
; ---------------------------------------------------------------------------
		jp	nz, loc_7394
		call	j_Cmd_L_PilotInfo
		jr	GameMain
; ---------------------------------------------------------------------------
; START	OF FUNCTION CHUNK FOR sub_728D

loc_74AA:				; CODE XREF: sub_728D+206j
		ld	a, (InFreeSpace)
		and	a
		jr	nz, loc_74B6
		call	GoDocking
; ---------------------------------------------------------------------------
		call	CalcCrdCoriolis2

loc_74B6:				; CODE XREF: sub_728D+221j
		call	Use_J_Drive
		call	CalcCrdSunPlanet
		ld	a, (WhatOnScreen)
		cp	4
		jr	c, loc_74E4
		call	j_j_MoveAllObjects
; END OF FUNCTION CHUNK	FOR sub_728D

; =============== S U B	R O U T	I N E =======================================




; === sub_728D === CODE XREF: LaunchDocking+68p
sub_728D:				; CODE XREF: LaunchDocking+68p

; FUNCTION CHUNK AT 739E SIZE 0000003A BYTES
; FUNCTION CHUNK AT 73E2 SIZE 00000044 BYTES
; FUNCTION CHUNK AT 7471 SIZE 00000031 BYTES
; FUNCTION CHUNK AT 74AA SIZE 0000001C BYTES
; FUNCTION CHUNK AT 74E4 SIZE 00000016 BYTES

		ld	hl, byte_6065
		call	sub_841A
		nop
		nop

loc_7295:				; CODE XREF: GoMission3+12j
		call	sub_FC33
		call	PrnCocpit_Ifn
		ld	sp, 4120h
		nop
		jp	loc_739E
; End of function sub_728D

; ---------------------------------------------------------------------------
byte_72A2:	db 3			; DATA XREF: sub_728D:loc_73F7w

; =============== S U B	R O U T	I N E =======================================


ScanKeyInPause:				; CODE XREF: sub_728D+1F0p

; FUNCTION CHUNK AT 733F SIZE 00000027 BYTES

		ld	a, (ControlKey2)
		and	80h ; 'А'
		ret	z
		ld	hl, FlagsSetUp

loc_72AC:				; CODE XREF: ScanKeyInPause+Ej
					; DATA XREF: ScanKeyInPause:loc_7340o
		halt
		ld	a, (PressKeyPrnSymbol)
		inc	a
		jr	nz, loc_72AC

loc_72B3:				; CODE XREF: ScanKeyInPause+25j
		halt
		ld	hl, PressKeyPrnSymbol
		ld	a, (hl)
		inc	hl
		push	hl
		ld	hl, unk_72FB
		call	sub_72D5
		jr	c, loc_72CA
		call	sub_72D4
		jr	c, loc_72D0
		pop	hl
		jr	loc_72B3
; ---------------------------------------------------------------------------

loc_72CA:				; CODE XREF: ScanKeyInPause+1Dj
		ld	a, (hl)
		pop	hl
		xor	(hl)
		ld	(hl), a
		jr	loc_733F
; ---------------------------------------------------------------------------

loc_72D0:				; CODE XREF: ScanKeyInPause+22j
		ld	l, (hl)
		ex	(sp), hl
		ld	a, (hl)
		ret
; End of function ScanKeyInPause


; =============== S U B	R O U T	I N E =======================================




