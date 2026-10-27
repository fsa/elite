; 99_misc - разобрано из kladov.asm
; Количество подпрограмм: 376

; === ABS_A_sub_B === CODE XREF: CalcDistanceToCursor2+4p
ABS_A_sub_B:				; CODE XREF: CalcDistanceToCursor2+4p
					; CalcDistanceToCursor2:loc_DFDEp ...
		sub	b
		ret	nc
		cpl
		ret
; End of function ABS_A_sub_B


; =============== S U B	R O U T	I N E =======================================




; === AddCashObj === CODE XREF: sub_9D40+AEp
AddCashObj:				; CODE XREF: sub_9D40+AEp
		push	iy
		ld	iy, (word_B452)
		ld	a, (iy+8)
		pop	iy
		and	0Fh
		ret	nz
		jp	AddCashObj3
; End of function AddCashObj


; =============== S U B	R O U T	I N E =======================================


IncBattleRating:			; CODE XREF: sub_9D40+BBp
		ld	iy, (word_B452)
		ld	a, (iy+8)
		and	0Fh
		jp	z, j_IncBattleRating2
		ld	(ix+21h), a
		ret
; End of function IncBattleRating


; =============== S U B	R O U T	I N E =======================================

; Attributes: noreturn

DestroyObj:				; CODE XREF: FireLaser+CEp
					; j_DestroyObjj
		bit	6, (ix+26h)
		jr	z, loc_7DDD
		ld	a, (Mission2)
		and	a
		jr	z, loc_7DDD

loc_7DD1:
		cp	3
		jr	z, loc_7DDD
		ld	a, 3
		ld	(Mission2), a
		add	a, a
		jr	loc_7DFF
; ---------------------------------------------------------------------------

loc_7DDD:				; CODE XREF: DestroyObj+4j
					; DestroyObj+Aj ...
		ld	h, (ix+24h)
		ld	l, (ix+23h)
		ld	a, (hl)
		and	0Fh
		ld	a, 6
		jr	nz, loc_7DFF
		ld	de, 7
		add	hl, de
		ld	a, (hl)
		cp	55h ; 'U'
		ld	a, 5
		jr	nc, loc_7DFF
		ld	a, (hl)
		cp	3Ch ; '<'
		ret	nz
		ld	a, c
		cp	32h ; '2'
		ret	nz
		ld	a, 8

loc_7DFF:				; CODE XREF: DestroyObj+16j
					; DestroyObj+23j ...
		jp	sub_B867
; End of function DestroyObj


; =============== S U B	R O U T	I N E =======================================




; === AddCashObj2 === CODE XREF: RAM:AddCashObj3j
AddCashObj2:				; CODE XREF: RAM:AddCashObj3j
		ld	a, d
		or	e
		ret	z
		ld	(LastBonusCash), de
		call	IncCASH
		ld	hl, CashBonus

loc_D1E6:				; CODE XREF: IncBattleRating2+10j
		set	7, (hl)
		ret
; End of function IncBattleRating2


; =============== S U B	R O U T	I N E =======================================




; === AddCashObj3 === CODE XREF: AddCashObj+Ej
AddCashObj3:				; CODE XREF: AddCashObj+Ej
		jp	AddCashObj2

; =============== S U B	R O U T	I N E =======================================

; Attributes: thunk



; === AddSign_HL_DE === CODE XREF: ScalarMultVectors16_8+27p
AddSign_HL_DE:				; CODE XREF: ScalarMultVectors16_8+27p
					; ScalarMultVectors16_8+33j
		ld	a, h
		xor	d
		ld	b, h
		ld	c, d
		res	7, h
		res	7, d
		and	80h ; 'А'
		jr	nz, loc_AFDC
		add	hl, de
		jr	loc_AFE6
; ---------------------------------------------------------------------------

loc_AFDC:				; CODE XREF: AddSign_HL_DE+Aj
		sbc	hl, de
		jr	nc, loc_AFE6
		add	hl, de
		ex	de, hl
		and	a
		sbc	hl, de
		ld	b, c

loc_AFE6:				; CODE XREF: AddSign_HL_DE+Dj
					; AddSign_HL_DE+11j
		srl	h
		rr	l
		bit	7, b
		ret	z
		set	7, h
		ret
; End of function AddSign_HL_DE

; ---------------------------------------------------------------------------
		db    0
		db    4
		db    2
		db    3
		db    0
		db    4
		db    6
		db    5
		db    0
		db    8
		db    2
		db    1
		db    0
		db    8
		db    6
		db    7
		db    0
		db    0
		db    0
		db    2
		db    1
		db    2
		db    0
		db    0
		db    0
		db    0
		db    0
		db    2
		db    2
		db    1
		db    2
		db    2
		db    0
		db    0
		db    0
		db    0
		db    0
		db    2
		db    1
		db    2
		db    2
		db    2
		db    0
		db    0
		db    0
		db    2
		db    2
		db    1
		db    1
		db    2
		db    0
		db    0
		db    0
		db    0
		db    0
		db    2
		db    2
		db    1
		db    2
		db    2
		db    0
		db    0
		db    0
		db    2
		db    0
		db    2
		db    1
		db    2
		db    0
		db    0
		db    0
		db    0
		db    0
		db    2
		db    2
		db    1
		db    2
		db    2
		db    0
		db    0
		db    0
		db    0
		db    0
		db    0

; =============== S U B	R O U T	I N E =======================================


PrnLineInBuf_clip:			; CODE XREF: PrintObject_2+91p
					; sub_7C60+C5p	...

; FUNCTION CHUNK AT B0B6 SIZE 00000082 BYTES
; FUNCTION CHUNK AT B162 SIZE 00000029 BYTES

		ld	a, d
		and	l
		inc	a
		ret	z
		ld	a, e
		and	h
		inc	a
		ret	z
		ld	a, c
		add	a, a
		add	a, a
		xor	c
		cpl
		ld	c, a
		call	sub_B095
		ld	a, d
		sub	78h ; 'x'
		jr	c, loc_B065
		ld	b, 14h
		call	loc_B21B
		ld	e, a
		ld	b, 0
		call	sub_B09C

loc_B065:				; CODE XREF: PrnLineInBuf_clip+14j
		ld	a, e
		sub	3Fh ; '?'
		jr	c, loc_B073
		ld	b, 28h ; '('
		call	loc_B1D7
		ld	d, a
		call	sub_B095

loc_B073:				; CODE XREF: PrnLineInBuf_clip+24j
		ld	a, h
		sub	78h ; 'x'
		jr	c, loc_B085
		ex	de, hl
		ld	b, 5
		call	loc_B21B
		ld	e, a
		ex	de, hl
		ld	b, 0
		call	loc_B0A6

loc_B085:				; CODE XREF: PrnLineInBuf_clip+32j
		ld	a, l
		sub	3Fh ; '?'
		jr	c, loc_B0B6
		ex	de, hl
		ld	b, 0Ah
		call	loc_B1D7
		ld	d, a
		ex	de, hl
		jp	loc_B0B6
; End of function PrnLineInBuf_clip


; =============== S U B	R O U T	I N E =======================================




; === AddSign_L_E === CODE XREF: sub_A6C9+1p sub_A6C9+Ap ...
AddSign_L_E:				; CODE XREF: sub_A6C9+1p sub_A6C9+Ap ...
		ld	a, h
		xor	d
		rla
		jr	c, loc_AB03
		ld	a, l
		add	a, e
		ret
; ---------------------------------------------------------------------------

loc_AB03:				; CODE XREF: AddSign_L_E+3j
		ld	a, l
		sub	e
		ret	nc
		neg
		and	a
		ld	h, d
		ret
; End of function AddSign_L_E


; =============== S U B	R O U T	I N E =======================================


Mult_A_hiB_C:				; CODE XREF: sub_A0FD+7p
					; CalcNewCrdNobj+32p ...
		ld	a, b
		sub	c
		jr	nc, loc_AB11
		neg

loc_AB11:				; CODE XREF: Mult_A_hiB_C+2j
		srl	a
		ld	h, 0BFh	; '┐'
		ld	l, a
		ld	d, (hl)
		ld	a, b
		add	a, c
		rra
		ld	l, a
		ld	a, (hl)
		sub	d
		ret
; End of function Mult_A_hiB_C


; =============== S U B	R O U T	I N E =======================================




; === Amount === DATA XREF: InitGame+1Do
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


; === AttackTargetObj === CODE XREF: sub_7C13+17p sub_7C13+1Dp ...
AttackTargetObj:			; CODE XREF: sub_7C13+17p sub_7C13+1Dp ...
		and	0Fh
		ret	z
		ld	hl, loc_6021
		ld	iy,  loc_B44C+1

loc_7B74:				; CODE XREF: AttackTargetObj+15j
		ld	de, 27h	; '''
		add	hl, de
		ld	e, 0Ah
		add	iy, de
		inc	a
		cp	9
		jr	nz, loc_7B74

AttackTargetObjIY:			; CODE XREF: sub_7C13+36p
		ld	a, (iy+0)
		and	a
		bit	7, a
		ret	z
		bit	6, a
		jr	nz, loc_7BA7
		bit	2, a
		ret	z
		push	ix
		push	hl
		pop	ix
		ld	a, (byte_B455)
		xor	(ix+25h)
		and	0Fh
		pop	ix
		ret	nz
		call	IntRnd_A
		and	0Fh
		ret	nz
		jr	loc_7BBE
; ---------------------------------------------------------------------------

loc_7BA7:				; CODE XREF: AttackTargetObj+20j
		ld	e, a
		or	c
		and	3
		ret	z
		ld	a, e
		xor	c
		and	3
		ret	z
		bit	0, (ix+26h)
		jr	nz, loc_7BBE
		bit	2, e
		jr	nz, loc_7BBE
		bit	2, c
		ret	z

loc_7BBE:				; CODE XREF: AttackTargetObj+3Bj
					; AttackTargetObj+4Bj ...
		push	iy
		push	hl
		pop	iy
		push	ix
		call	sub_7B4A
		call	c, sub_7B4A
		call	c, sub_7B4A
		pop	ix
		pop	iy
		ret	nc
		ld	a, (byte_B455)
		ld	e, a
		ld	a, (iy+8)
		add	a, a
		add	a, a
		add	a, a
		add	a, a
		or	e
		ld	(iy+8),	a
		push	hl
		ex	(sp), iy
		ld	a, (ix+22h)
		rrca
		rrca
		and	3Fh ; '?'
		xor	0Fh
		ld	e, a
		ld	a, (iy+22h)
		sub	e
		jr	nc, loc_7C0C
		set	4, (iy+20h)
		push	hl
		push	ix
		push	iy
		pop	ix
		push	ix
		call	j_DestroyObj
; ---------------------------------------------------------------------------
		db 0FDh	; ¤
		db 0E1h	; с
		db 0DDh	; ▌
		db 0E1h	; с
		db 0E1h	; с
		db  3Eh	; >
		db    1
; ---------------------------------------------------------------------------

loc_7C0C:				; CODE XREF: AttackTargetObj+89j
		ld	(iy+22h), a
		pop	iy
		scf
		ret
; End of function AttackTargetObj


; =============== S U B	R O U T	I N E =======================================




; === AttackTargetObjIY === CODE XREF: sub_7C13+36p
AttackTargetObjIY:			; CODE XREF: sub_7C13+36p
		ld	a, (iy+0)
		and	a
		bit	7, a
		ret	z
		bit	6, a
		jr	nz, loc_7BA7
		bit	2, a
		ret	z
		push	ix
		push	hl
		pop	ix
		ld	a, (byte_B455)
		xor	(ix+25h)
		and	0Fh
		pop	ix
		ret	nz
		call	IntRnd_A
		and	0Fh
		ret	nz
		jr	loc_7BBE
; ---------------------------------------------------------------------------

loc_7BA7:				; CODE XREF: AttackTargetObj+20j
		ld	e, a
		or	c
		and	3
		ret	z
		ld	a, e
		xor	c
		and	3
		ret	z
		bit	0, (ix+26h)
		jr	nz, loc_7BBE
		bit	2, e
		jr	nz, loc_7BBE
		bit	2, c
		ret	z

loc_7BBE:				; CODE XREF: AttackTargetObj+3Bj
					; AttackTargetObj+4Bj ...
		push	iy
		push	hl
		pop	iy
		push	ix
		call	sub_7B4A
		call	c, sub_7B4A
		call	c, sub_7B4A
		pop	ix
		pop	iy
		ret	nc
		ld	a, (byte_B455)
		ld	e, a
		ld	a, (iy+8)
		add	a, a
		add	a, a
		add	a, a
		add	a, a
		or	e
		ld	(iy+8),	a
		push	hl
		ex	(sp), iy
		ld	a, (ix+22h)
		rrca
		rrca
		and	3Fh ; '?'
		xor	0Fh
		ld	e, a
		ld	a, (iy+22h)
		sub	e
		jr	nc, loc_7C0C
		set	4, (iy+20h)
		push	hl
		push	ix
		push	iy
		pop	ix
		push	ix
		call	j_DestroyObj
; ---------------------------------------------------------------------------
		db 0FDh	; ¤
		db 0E1h	; с
		db 0DDh	; ▌
		db 0E1h	; с
		db 0E1h	; с
		db  3Eh	; >
		db    1
; ---------------------------------------------------------------------------

loc_7C0C:				; CODE XREF: AttackTargetObj+89j
		ld	(iy+22h), a
		pop	iy
		scf
		ret
; End of function AttackTargetObj


; =============== S U B	R O U T	I N E =======================================




; === BeginLaunch === CODE XREF: LaunchDocking+24j
BeginLaunch:				; CODE XREF: LaunchDocking+24j
		ld	a, 4
		ld	(IndDanger), a
		call	FX_Launch
		xor	a
		ld	(ManualDocking), a
		ld	(WhatOnScreen),	a
		ld	(J_Drive_Coriolis), a
		call	sub_E6EE
		ld	a, 32h ; '2'
		ld	(loc_932A+1), a
		call	PrnCocpit_Ifn
		call	sub_9946
		call	sub_726C
		ld	hl, PlanetCrd
		ld	b, 7
		call	ClearMemory
		ld	(hl), 80h ; 'А'
		inc	hl
		ld	(hl), 1
		ld	a, (SunCrdZhi)
		xor	80h ; 'А'
		ld	(SunCrdZhi), a
		ld	hl, objCoriolis
		ld	b, 1Bh
		call	ClearMemory
		ld	ix, objCoriolis
		ld	a, 0F0h	; 'Ё'
		ld	(ix+6),	a
		ld	a, 80h ; 'А'
		ld	(ix+8),	a
		dec	a
		ld	(ix+0Eh), a
		ld	(ix+12h), a
		ld	(ix+16h), a
		ld	a, 0F0h	; 'Ё'
		ld	(ix+22h), a
		call	ClearAroundObj
		call	UpdateAllIndicators
		call	ClearScrBuffer
		call	PrnDockingTonnel ; обратный отсчет до прыжка, круги
; ---------------------------------------------------------------------------
		ld	b, 24h ; '$'

loc_94C8:				; CODE XREF: LaunchDocking+E9j
		push	bc
		call	ClearScrBuffer
		call	PrintStars
		call	PrnDockingTonnel ; обратный отсчет до прыжка, круги
; ---------------------------------------------------------------------------
		call	CopyBuf2Screen
		pop	bc
		djnz	loc_94C8
		ld	b, 0Bh

loc_94DA:				; CODE XREF: LaunchDocking+101j
		push	bc
		call	ClearScrBuffer
		call	CalcCrdSunPlanet
		call	PrintStars
		call	PrnDockingTonnel ; обратный отсчет до прыжка, круги
; ---------------------------------------------------------------------------
		call	PrnSunPlanet
		call	CopyBuf2Screen
		pop	bc
		djnz	loc_94DA
		xor	a
		ld	(BlockControl),	a
; End of function LaunchDocking


; =============== S U B	R O U T	I N E =======================================


InitSpeedAngle2:			; CODE XREF: InitSpeedAnglep RAM:736Ap ...
		ld	b, 5
		call	sub_F9AB
		call	ClearMemory
		ld	(hl), 0Ch

loc_94FE:				; CODE XREF: PrnCocpit_Ifn+6p
					; sub_813E+1Cj
		ld	b, 5
		ld	hl, ControlKey1
; End of function InitSpeedAngle2


; =============== S U B	R O U T	I N E =======================================


ClearMemory:				; CODE XREF: LaunchDocking+98p
					; LaunchDocking+ADp ...
		xor	a

loc_9504:				; CODE XREF: ClearMemory+3j
		ld	(hl), a
		inc	hl
		djnz	loc_9504
		ret
; End of function ClearMemory


; =============== S U B	R O U T	I N E =======================================


ClearAroundObj:				; CODE XREF: RAM:7566p	Use_J_Drive+29p ...
		ld	ix, objCoriolis
		res	2, (ix+21h)
		xor	a
		ld	(PlayerPirate),	a
		ld	de, 27h	; '''
		ld	b, 7

loc_951A:				; CODE XREF: ClearAroundObj+16j
		add	ix, de
		ld	(ix+22h), a
		djnz	loc_951A
		ret
; End of function ClearAroundObj

; ---------------------------------------------------------------------------
byte_9522:	db 0			; DATA XREF: sub_9548:loc_9551r
					; sub_9579+6r ...
word_9523:	dw 8000h		; DATA XREF: sub_9579+30r sub_9579+47w ...
word_9525:	dw 0B27h		; DATA XREF: sub_9579:loc_95A6r
					; sub_9579+4Bw	...

; =============== S U B	R O U T	I N E =======================================


FX_Launch:				; CODE XREF: RAM:7532p
					; LaunchDocking+58p ...
		ld	a, (FlagsSetUp)
		bit	7, a
		ret	z
		call	sub_969B
		jr	loc_9551
; End of function FX_Launch

; ---------------------------------------------------------------------------
; START	OF FUNCTION CHUNK FOR sub_9296

loc_9532:				; CODE XREF: sub_9296+137j
		ld	a, (FlagsSetUp)
		bit	7, a
		ret	z
		call	sub_96A9
		jr	loc_9551
; END OF FUNCTION CHUNK	FOR sub_9296

; =============== S U B	R O U T	I N E =======================================




; === CalcAngSpeed4Prn === CODE XREF: sub_728D+1EAp
CalcAngSpeed4Prn:			; CODE XREF: sub_728D+1EAp
		ld	hl,  AngleLeftRight+1
		ld	a, (hl)
		ld	(HighAngleLR), a
		bit	7, a
		jr	nz, loc_714B
		add	a, 15h
		jr	loc_7151
; ---------------------------------------------------------------------------

loc_714B:				; CODE XREF: CalcAngSpeed4Prn+9j
		and	7Fh ; ''
		ld	c, a
		ld	a, 15h
		sub	c

loc_7151:				; CODE XREF: CalcAngSpeed4Prn+Dj
		ld	(byte_70FE), a
		inc	l
		inc	l
		ld	a, (hl)
		ld	(HighAngleDU), a
		bit	7, a
		jr	nz, loc_7162
		add	a, 15h
		jr	loc_7168
; ---------------------------------------------------------------------------

loc_7162:				; CODE XREF: CalcAngSpeed4Prn+20j
		and	7Fh ; ''
		ld	c, a
		ld	a, 15h
		sub	c

loc_7168:				; CODE XREF: CalcAngSpeed4Prn+24j
		ld	(byte_7100), a
		inc	l
		inc	l
		ld	a, (hl)
		ld	(HighDeltaSpeed), a
		ld	b, a
		add	a, a
		add	a, a
		add	a, b
		add	a, 5
		ld	(byte_70F4), a
		ret
; End of function CalcAngSpeed4Prn


; =============== S U B	R O U T	I N E =======================================

; Attributes: noreturn

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




; === CalcDistanceToCursor === CODE XREF: SearchPlanetNearCursor+13p
CalcDistanceToCursor:			; CODE XREF: SearchPlanetNearCursor+13p
		push	af
		ld	hl, Pilot_CrdYTarget
		xor	(hl)
		jr	nz, loc_802C
		dec	hl
		ld	a, b
		xor	(hl)
		jr	z, loc_8033

loc_802C:				; CODE XREF: CalcDistanceToCursor+5j
		pop	af
		call	CalcDistanceToCursor2
		set	0, l
		ret
; ---------------------------------------------------------------------------

loc_8033:				; CODE XREF: CalcDistanceToCursor+Aj
		ld	hl, 0
		ld	(MinDistance), hl
		call	SavePlanetData
		pop	af
		pop	hl
		pop	bc
		jp	loc_DFA2
; End of function CalcDistanceToCursor


; =============== S U B	R O U T	I N E =======================================


PrnCocpit_Ifn:				; CODE XREF: GoDocking+3Cp sub_728D+Bp ...

; FUNCTION CHUNK AT 8459 SIZE 00000006 BYTES

		ld	de, 4020h
		ld	a, (de)
		inc	a
		ret	nz
		call	loc_94FE
		call	j_ClsAndPrintRamk
		ld	de, 1Fh
		ld	b, 17h
		ld	a, 0
		ld	hl, 5800h
		ld	(hl), a

loc_8059:				; CODE XREF: PrnCocpit_Ifn+1Bj
		add	hl, de
		ld	(hl), a
		inc	hl
		ld	(hl), a
		djnz	loc_8059
		add	hl, de
		ld	(hl), a
		ld	de, 5000h
		ld	hl, DataCocpit
		ld	bc, 800h
		ldir
		ld	de, 5A00h
		ld	hl, 5B00h
		ld	bc, 100h
		ldir
		call	sub_725C
		call	sub_7265
		jp	loc_8459
; End of function PrnCocpit_Ifn


; =============== S U B	R O U T	I N E =======================================


ClearScrBuffer:				; CODE XREF: sub_728D:loc_73C3p
					; sub_728D+25Ap ...

; FUNCTION CHUNK AT DA5F SIZE 00000005 BYTES

		di
		ld	hl, 0CFE0h
		ld	de, 0C020h
		ld	bc, 5720h

loc_808A:				; CODE XREF: ClearScrBuffer+10j
		dec	l
		dec	e
		dec	c
		ld	a, (bc)
		ld	(hl), a
		ld	(de), a
		jr	nz, loc_808A
		ld	(loc_DA5F+1), sp
		ld	sp, hl
		ex	de, hl
		add	hl, hl
		ld	e, l
		ld	d, l
		ld	b, 7Dh ; '}'

loc_809D:				; CODE XREF: ClearScrBuffer+2Fj
		inc	e
		push	de
		dec	e
		push	de
		push	de
		push	de
		push	de
		push	de
		push	de
		push	de
		push	de
		push	de
		push	de
		push	de
		push	de
		push	de
		push	de
		push	hl
		djnz	loc_809D
		jp	loc_DA5F
; End of function ClearScrBuffer

; ---------------------------------------------------------------------------

loc_80B4:				; CODE XREF: sub_EB2F+4p
		cp	0Eh
		jr	nc, loc_80BC
		ld	b, a
		add	a, a
		add	a, b
		ret
; ---------------------------------------------------------------------------

loc_80BC:				; CODE XREF: RAM:80B6j
		ld	hl, 667h
		xor	a
		ret
; ---------------------------------------------------------------------------

SetMaxTonnage:				; CODE XREF: sub_5EC5+1Bp
					; UseEscapePod+3Fp ...
		ld	a, (PilotLargeCargo) ; установка максимальной грузоподъемности
		ld	b, a
		dec	b
		ld	a, 5

loc_80C8:				; CODE XREF: RAM:80CBj
		add	a, 0Fh
		inc	b
		jr	nz, loc_80C8
		ld	(PilotFreeCargo), a
		ret
; ---------------------------------------------------------------------------
word_80D1:	dw 411Ah		; DATA XREF: sub_813Ew	sub_813E+18r

; =============== S U B	R O U T	I N E =======================================

; проверка нажатия (и соответствующий переход) клавиш 1, 2, 3, 4, I, O,	P, K, L, enter



; === CalcDistanceToCursor2 === CODE XREF: CalcDistanceToCursor+Dp
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




; === CalcFuelForDistance === CODE XREF: SearchPlanetNearCursor+32p
CalcFuelForDistance:			; CODE XREF: SearchPlanetNearCursor+32p
					; SearchPlanetNearCursor+4Bp
		jr	c, loc_DFF6
		jr	loc_DFF9
; ---------------------------------------------------------------------------

loc_DFF6:				; CODE XREF: CalcFuelForDistancej
		ld	hl, 0FFFFh

loc_DFF9:				; CODE XREF: CalcFuelForDistance+2j
		ld	a, h
		or	l
		jr	z, loc_E005
		call	j_Sqrt_H_HL
; ---------------------------------------------------------------------------
		ld	l, h
		ld	h, 0
		add	hl, hl
		add	hl, hl

loc_E005:				; CODE XREF: CalcFuelForDistance+9j
		ld	(FuelToPlanet),	hl
		ret
; End of function CalcFuelForDistance


; =============== S U B	R O U T	I N E =======================================


Mult_HL_D_A:				; CODE XREF: CalcDistanceToCursor2+8p
					; CalcDistanceToCursor2+16p ...
		ld	e, 0
		ld	b, 8
		ld	hl, 0

loc_E010:				; CODE XREF: Mult_HL_D_A:loc_E019j
		srl	d
		rr	e
		rl	a
		jr	nc, loc_E019
		add	hl, de

loc_E019:				; CODE XREF: Mult_HL_D_A+Dj
		djnz	loc_E010
		ret
; End of function Mult_HL_D_A


; =============== S U B	R O U T	I N E =======================================


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




; === CalcTempAltitude === CODE XREF: PrintCompass+47p
CalcTempAltitude:			; CODE XREF: PrintCompass+47p
					; PrintCompass+94p
		push	hl
		add	hl, hl
		pop	de
		srl	d
		rr	e
		add	hl, de
		ld	a, (IndDanger)
		ld	b, a
		ld	a, h
		cp	50h ; 'P'
		jr	c, loc_82F8
		cp	78h ; 'x'
		jr	nc, loc_82F4
		ld	a, b
		cp	63h ; 'c'
		ret	z
		ld	a, 2
		jr	loc_82FE
; ---------------------------------------------------------------------------

loc_82F4:				; CODE XREF: CalcTempAltitude+13j
		ld	a, 63h ; 'c'
		jr	loc_82FE
; ---------------------------------------------------------------------------

loc_82F8:				; CODE XREF: CalcTempAltitude+Fj
		ld	a, b
		cp	4
		ret	nz
		ld	a, 6

loc_82FE:				; CODE XREF: CalcTempAltitude+1Bj
					; CalcTempAltitude+1Fj
		ld	(IndDanger), a
		ret
; End of function CalcTempAltitude


; =============== S U B	R O U T	I N E =======================================


GetCrdObject:				; CODE XREF: CalcLenVectorCrdHp
					; CalcLenVectorCrdp
		ld	e, (ix+0)
		ld	d, (ix+1)
		ld	c, (ix+3)
		ld	b, (ix+4)
		ld	l, (ix+6)
		ld	h, (ix+7)
		ret
; End of function GetCrdObject


; =============== S U B	R O U T	I N E =======================================

; Attributes: noreturn

CalcLenVectorCrdH:			; CODE XREF: PrintCompass+D7p
		call	GetCrdObject
		res	7, h
		res	7, d
		res	7, b
		ld	a, 1
		jr	loc_8327
; End of function CalcLenVectorCrdH


; =============== S U B	R O U T	I N E =======================================

; Attributes: noreturn

CalcLenVectorCrd:			; CODE XREF: PrintCompass+1Dp
					; PrintCompass+7Cp ...

; FUNCTION CHUNK AT 838F SIZE 00000001 BYTES

		call	GetCrdObject
		ld	a, 1

loc_8327:				; CODE XREF: CalcLenVectorCrdH+Bj
					; CalcLenVectorCrd+14j
		ex	de, hl
		add	hl, hl
		ex	de, hl
		jr	c, loc_8338
		sla	c
		rl	b
		jr	c, loc_833C
		add	hl, hl
		jr	c, loc_8342
		add	a, a
		jr	nc, loc_8327

loc_8338:				; CODE XREF: CalcLenVectorCrd+8j
		rr	d
		jr	loc_8348
; ---------------------------------------------------------------------------

loc_833C:				; CODE XREF: CalcLenVectorCrd+Ej
		rr	b
		srl	d
		jr	loc_8348
; ---------------------------------------------------------------------------

loc_8342:				; CODE XREF: CalcLenVectorCrd+11j
		rr	h
		srl	b
		srl	d

loc_8348:				; CODE XREF: CalcLenVectorCrd+18j
					; CalcLenVectorCrd+1Ej
		ld	c, d
		push	af
		ld	a, h
		call	Sqr_DE_A
		ex	de, hl
		ld	a, b
		ld	b, 0
		call	Sqr_DE_A
		add	hl, de
		jr	nc, loc_8359
		inc	b

loc_8359:				; CODE XREF: CalcLenVectorCrd+34j
		ld	a, c
		call	Sqr_DE_A
		add	hl, de
		jr	nc, loc_8361
		inc	b

loc_8361:				; CODE XREF: CalcLenVectorCrd+3Cj
		ld	a, b
		and	a
		jr	z, loc_8377
		srl	b
		rr	h
		rr	l
		srl	b
		rr	h
		rr	l
		pop	af
		srl	a
		jr	z, loc_8386
		push	af

loc_8377:				; CODE XREF: CalcLenVectorCrd+41j
		call	j_Sqrt_H_HL
; ---------------------------------------------------------------------------
		pop	af

loc_837B:				; CODE XREF: CalcLenVectorCrd+61j
		srl	a
		jr	z, locret_838F
		srl	h
		rr	l
		jp	loc_837B
; ---------------------------------------------------------------------------

loc_8386:				; CODE XREF: CalcLenVectorCrd+52j
		call	j_Sqrt_H_HL
; End of function CalcLenVectorCrd

; ---------------------------------------------------------------------------
		add	hl, hl
		jr	nc, locret_838F
		ld	hl, 0
; START	OF FUNCTION CHUNK FOR CalcLenVectorCrd

locret_838F:				; CODE XREF: CalcLenVectorCrd+5Bj
					; RAM:838Aj
		ret
; END OF FUNCTION CHUNK	FOR CalcLenVectorCrd
; ---------------------------------------------------------------------------
; START	OF FUNCTION CHUNK FOR PrintCompass

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



; === CalcVisibleVerges === CODE XREF: objPrnObj+22p
CalcVisibleVerges:			; CODE XREF: objPrnObj+22p
		push	ix
		push	iy
		push	iy
		ex	(sp), ix
		pop	hl
		ld	(AdrSlotObj), hl
		ld	d, (ix+11h)
		call	sub_AE0E
		add	ix, de
		ld	de, VergesVisible
		ld	a, (iy+13h)
		ld	(IY_dFactor), a
		ld	a, (iy+5)
		and	a
		ld	iy, MatrixRotate
		jr	nz, loc_BCFF
		inc	a
		ld	(de), a
		jr	loc_BD5A
; ---------------------------------------------------------------------------

loc_BCFF:				; CODE XREF: CalcVisibleVerges+25j
					; CalcVisibleVerges+84j
		push	af
		push	de
		call	sub_AA31
		and	1Fh
		cp	1Fh
		jr	z, loc_BD15
		ld	b, a
		ld	a, (DistanceToObj)
		cp	b
		jr	c, loc_BD15
		pop	de
		xor	a
		jr	loc_BD4F
; ---------------------------------------------------------------------------

loc_BD15:				; CODE XREF: CalcVisibleVerges+34j
					; CalcVisibleVerges+3Bj
		push	iy
		ld	hl, (AdrSlotObj)
		push	hl
		call	RotateCrdN_Normal
		ld	(NormalCrdX), a
		ld	(AddNormalCrdX), hl
		call	RotateCrdN_Normal
		ld	(NormalCrdY), a
		ld	(AddNormalCrdY), hl
		call	RotateCrdN_Normal
		ld	(NormalCrdZ), a
		ld	(AddNormalCrdZ), hl
		pop	hl
		push	ix
		ld	iy, AddNormalCrdX
		ld	ix, NormalCrdX
		call	ScalarMultVectors16_8
		pop	ix
		pop	iy
		pop	de
		xor	a
		sla	h
		jr	nc, loc_BD4F
		inc	a

loc_BD4F:				; CODE XREF: CalcVisibleVerges+3Fj
					; CalcVisibleVerges+78j
		ld	(de), a
		inc	de
		call	sub_AE26
		add	ix, bc
		pop	af
		dec	a
		jr	nz, loc_BCFF

loc_BD5A:				; CODE XREF: CalcVisibleVerges+29j
		pop	iy
		pop	ix
		ret
; End of function CalcVisibleVerges


; =============== S U B	R O U T	I N E =======================================


ScalarMultVectors16_8:			; CODE XREF: CalcVisibleVerges+6Dp
		ld	e, (iy+1)
		ld	c, (iy+0)
		ld	a, (ix+0)
		call	Mult_HL_EC_A
		push	hl
		ld	e, (iy+3)
		ld	c, (iy+2)
		ld	a, (ix+1)
		call	Mult_HL_EC_A
		push	hl
		ld	e, (iy+5)
		ld	c, (iy+4)
		ld	a, (ix+2)
		call	Mult_HL_EC_A
		pop	de
		call	AddSign_HL_DE
		adc	hl, hl
		bit	7, b
		jr	z, loc_BD91
		set	7, h

loc_BD91:				; CODE XREF: ScalarMultVectors16_8+2Ej
		pop	de
		jp	AddSign_HL_DE
; End of function ScalarMultVectors16_8


; =============== S U B	R O U T	I N E =======================================


Mult_HL_EC_A:				; CODE XREF: ScalarMultVectors16_8+9p
					; ScalarMultVectors16_8+16p ...
		ld	b, a
		sla	b
		and	80h ; 'А'
		xor	e
		ld	e, a
		call	Mult_A_hiB_C
		ld	d, e
		ld	e, a
		ld	c, d
		res	7, c
		push	de
		call	Mult_HL_B_C
		pop	de
		ld	a, d
		ld	d, 0
		add	hl, de
		add	hl, hl
		rla
		rr	h
		rr	l
		ret
; End of function Mult_HL_EC_A


; =============== S U B	R O U T	I N E =======================================


RotateCrdN_Normal:			; CODE XREF: CalcVisibleVerges+47p
					; CalcVisibleVerges+50p ...
		call	RotateCrdN
		pop	bc
		ex	(sp), ix
		ld	e, (ix+0)
		ld	d, (ix+1)
		ld	a, (ix+2)
		inc	ix
		inc	ix
		inc	ix
		ex	(sp), ix
		push	bc
		ld	c, a
		ld	a, (IY_dFactor)
		sla	l
		push	hl
		ld	b, a
		inc	b

loc_BDD5:				; CODE XREF: RotateCrdN_Normal+23j
		srl	l
		djnz	loc_BDD5
		rl	l
		ld	a, c
		xor	h
		sla	h
		rla
		jr	c, loc_BDE5
		add	hl, de
		jr	loc_BDF4
; ---------------------------------------------------------------------------

loc_BDE5:				; CODE XREF: RotateCrdN_Normal+2Cj
		ex	de, hl
		and	a
		sbc	hl, de
		jr	nc, loc_BDF4
		add	hl, de
		ex	de, hl
		and	a
		sbc	hl, de
		ld	a, c
		xor	80h ; 'А'
		ld	c, a

loc_BDF4:				; CODE XREF: RotateCrdN_Normal+2Fj
					; RotateCrdN_Normal+35j
		sla	c
		rr	h
		rr	l
		pop	de
		ld	a, e
		rl	d
		rra
		ret
; End of function RotateCrdN_Normal

; ---------------------------------------------------------------------------


; === CallFunctionObj === CODE XREF: sub_9296+1Ap
CallFunctionObj:			; CODE XREF: sub_9296+1Ap
					; UseMissile+5Ap ...
		jp	sub_B867
; End of function CallFunctionObj


; =============== S U B	R O U T	I N E =======================================

; Attributes: noreturn thunk

SetShipwreck:				; CODE XREF: RAM:752Cp
		jp	SetShipwreck2
; End of function SetShipwreck

; ---------------------------------------------------------------------------


; === CallFunctionObj_1 === CODE XREF: sub_DA51+Bp
CallFunctionObj_1:			; CODE XREF: sub_DA51+Bp

; FUNCTION CHUNK AT B96B SIZE 000000BC BYTES
; FUNCTION CHUNK AT BA94 SIZE 00000008 BYTES
; FUNCTION CHUNK AT BAE8 SIZE 0000006D BYTES
; FUNCTION CHUNK AT BBBA SIZE 00000038 BYTES
; FUNCTION CHUNK AT BBFD SIZE 000000CB BYTES

		srl	c
		srl	c
		srl	c
		and	0Fh
		jp	z, SetRndShip
		dec	a
		jp	z, Set3_4Targons
		dec	a
		jp	z, SetObjMissile
		dec	a
		ret	z
		dec	a
		jp	z, ShipSetShip
		dec	a
		jp	z, Set2Alloys
		dec	a
		jp	z, SetCargo
		dec	a
		jp	z, SetCobra_EscPod
		dec	a
		jp	z, Set2Minerals
		dec	a
		jp	z, SetShip4Intro
		ret
; End of function CallFunctionObj_1


; =============== S U B	R O U T	I N E =======================================

; a - номер объекта для	установки

SetObjInSlot:				; CODE XREF: sub_5EC5p
					; ReCalcCrdObj_coriolis+86p ...
		call	sub_EF01	; a - номер объекта
		ld	a, b

loc_B89C:				; CODE XREF: SetObjInSlot_Intro+4j
		and	3Fh ; '?'
		add	a, a
		add	a, a
		add	a, l
		ld	l, a
		jr	nc, loc_B8A5
		inc	h

loc_B8A5:				; CODE XREF: SetObjInSlot+Aj
		inc	hl
		ld	e, (hl)
		inc	hl
		ld	d, (hl)
		push	de
		pop	iy
		ld	(ix+24h), d
		ld	(ix+23h), e
		push	ix
		pop	de
		ld	bc, 1Fh
		ld	hl, PatternObject
		ldir
		ld	a, (iy+7)
		ld	(ix+22h), a
		ld	a, (iy+12h)
		ld	c, a
		and	6Fh ; 'o'
		ld	(ix+21h), a
		call	sub_D60B
		and	7
		rla
		rl	c
		rra
		ld	(ix+20h), a
		ld	a, (iy+6)
		srl	a
		ld	(ix+1Bh), a
		ld	a, 7Eh ; '~'
		ld	(ix+25h), a
		xor	a
		ld	(ix+26h), a
		set	6, (ix+20h)
		bit	2, (ix+21h)
		jr	nz, loc_B903
		call	IntRnd_A
		and	7
		ld	(ix+1),	a
		call	IntRnd_A
		and	7
		ld	(ix+4),	a

loc_B903:				; CODE XREF: SetObjInSlot+59j
		call	IntRnd_A
		ld	(ix+0),	a
		call	IntRnd_A
		ld	(ix+3),	a
		call	IntRnd_A
		rra
		rr	(ix+2)
		rra
		rr	(ix+5)
		xor	a
		ret
; End of function SetObjInSlot


; =============== S U B	R O U T	I N E =======================================

; Attributes: noreturn

ShipSetObject:				; CODE XREF: CallFunctionObj_1+167p
					; CallFunctionObj_1+1A5p ...
		ld	c, a
		push	ix
		call	sub_FACA
; ---------------------------------------------------------------------------
		pop	hl
		ret	nz
		push	hl
		ld	a, c
		call	sub_B71C	; hl - адрес слота родиителя
; ---------------------------------------------------------------------------
		pop	hl
		push	ix
		pop	de
		ld	bc, 1Bh
		ldir
		xor	a
		ret
; End of function ShipSetObject


; =============== S U B	R O U T	I N E =======================================


FindSlot4Obj:				; CODE XREF: ReCalcCrdObj_coriolis+81p
					; SetPlayerObj+1p ...
		ld	ix, objCoriolis
		ld	de, 27h	; '''
		ld	b, 6

loc_B93F:				; CODE XREF: FindSlot4Obj+10j
		add	ix, de
		ld	a, (ix+22h)
		and	a
		ret	z
		djnz	loc_B93F
		inc	b
		ret
; End of function FindSlot4Obj


; =============== S U B	R O U T	I N E =======================================


SetPlayerObj:				; CODE XREF: SetPlayerAlloys:SetPlayerCargop
		ld	c, a
		call	FindSlot4Obj
		ret	nz
		ld	a, c
; End of function SetPlayerObj


; =============== S U B	R O U T	I N E =======================================


SetPlayerObjSlot:			; CODE XREF: sub_9391+18j
					; CallFunctionObj_1+29Cp
		call	sub_5EC5	; запуск ракеты	игроком
					; запуск спасательной капсулы (установка кобры)
		res	7, (ix+0Eh)
		res	7, (ix+16h)
		ld	(ix+0),	a
		ld	(ix+1),	a
		ld	(ix+4),	a
		ld	(ix+6),	a
		ld	(ix+7),	a
		ret
; End of function SetPlayerObjSlot

; ---------------------------------------------------------------------------
; START	OF FUNCTION CHUNK FOR CallFunctionObj_1

Set3_4Targons:				; CODE XREF: CallFunctionObj_1+Cj
		call	SetTargonStation ; Установка станции таргонов в	межпланетном пространстве для последней	подмиссии в войне с таргонами
		and	3
		add	a, 3
		ld	c, a

loc_B973:				; CODE XREF: CallFunctionObj_1+132j
		call	FindSlot4Obj
		ret	nz
		ld	a, 0Fh
		push	bc
		call	SetObjInSlot	; a - номер объекта для	установки
		set	0, (ix+25h)
		call	IntRnd_A
		and	0Fh
		or	4
		ld	(ix+7),	a
		rra
		rra
		rra
		and	1
		ld	(ix+1),	a
		xor	a
		ld	(ix+4),	a
		call	RndDirectionObj
		pop	bc
		dec	c
		jr	nz, loc_B973
		ret
; ---------------------------------------------------------------------------

SetObjMissile:				; CODE XREF: CallFunctionObj_1+10j
		ld	a, c		; Запуск ракеты
					; Вх:  с - цель	ракеты
					;	 =0 игрок (запустил противник)
					;	<>0 номер объекта по которому запустил ракету игрок
					; Вых: z=1 успешно
		and	a
		jr	z, loc_B9CF
		ld	ix, objCoriolis
		ld	de, 111h
		add	ix, de
		ld	a, (ix+22h)
		and	a
		jr	z, loc_B9B6
		call	FindSlot4Obj
		ret	nz

loc_B9B6:				; CODE XREF: CallFunctionObj_1+146j
		ld	a, 1
		push	bc
		call	sub_9391	; c = номер объекта по которому	запустил ракету	игрок
		pop	bc
		ld	a, 6Ah ; 'j'
		ld	(ix+6),	a
		ld	a, 29h ; ')'
		ld	(ix+3),	a
		ld	a, 80h ; 'А'
		ld	(ix+5),	a
		ld	a, c
		jr	loc_B9DE
; ---------------------------------------------------------------------------

loc_B9CF:				; CODE XREF: CallFunctionObj_1+137j
		ld	a, 1
		call	ShipSetObject
; ---------------------------------------------------------------------------
		ret	nz
		ld	a, (HighDeltaSpeed)
		add	a, (ix+1Bh)
		ld	(ix+1Bh), a

loc_B9DE:				; CODE XREF: CallFunctionObj_1+163j
		ld	(ix+25h), c
		set	0, (ix+26h)
		xor	a
		ret
; ---------------------------------------------------------------------------

ShipSetShip:				; CODE XREF: CallFunctionObj_1+16j
		ld	a, 8
		dec	c
		jr	z, loc_BA0D
		call	IntRnd_A
		and	1
		add	a, 9
		dec	c
		jr	z, loc_BA0D
		ld	a, 12h
		dec	c
		jr	z, loc_BA0D
		ld	a, 0Fh
		dec	c
		jr	z, loc_BA0D
		dec	c
		ret	nz
		call	IntRnd_A
		bit	0, a
		ld	a, 10h
		jr	z, loc_BA0D
		ld	a, 11h

loc_BA0D:				; CODE XREF: CallFunctionObj_1+180j
					; CallFunctionObj_1+18Aj ...
		push	ix
		call	ShipSetObject
; ---------------------------------------------------------------------------
		pop	iy
		ret	nz
		bit	2, (iy+26h)
		call	z, RotateSKOobj
		bit	2, (iy+21h)
		ret	z
		set	2, (ix+21h)
		xor	a
		ret
; END OF FUNCTION CHUNK	FOR CallFunctionObj_1

; =============== S U B	R O U T	I N E =======================================


RotateSKOobj:				; CODE XREF: CallFunctionObj_1+1AFp
					; SetShipMinAlloys+Cp

; FUNCTION CHUNK AT BA58 SIZE 0000002B BYTES

		ld	a, (byte_B866)
		xor	1
		ld	(byte_B866), a
		jr	z, loc_BA58
		push	ix
		call	sub_BA3F
		call	sub_BA3F
		call	sub_BA3F
		pop	ix
		ret
; End of function RotateSKOobj


; =============== S U B	R O U T	I N E =======================================




; === CallFunction_Obj === CODE XREF: ReactionObject+6Ep
CallFunction_Obj:			; CODE XREF: ReactionObject+6Ep
					; ReactionObject+93p ...
		push	iy
		push	ix
		call	SetObjbyShip	; установка объекта кораблем противника
; ---------------------------------------------------------------------------
		pop	ix
		pop	iy
		ret
; End of function CallFunction_Obj


; =============== S U B	R O U T	I N E =======================================


TestObjInRadius:			; CODE XREF: DirectObject+14p
					; DirectObject+21p
		ld	h, (ix+1)
		ld	l, (ix+0)
		and	a
		sbc	hl, de
		ret	nc
		ld	h, (ix+4)
		ld	l, (ix+3)
		and	a
		sbc	hl, de
		ret	nc
		ld	h, (ix+7)
		ld	l, (ix+6)
		inc	d
		inc	d
		and	a
		sbc	hl, de
		ret
; End of function TestObjInRadius


; =============== S U B	R O U T	I N E =======================================

; Attributes: noreturn

DirectObject:				; CODE XREF: ReactionObject+D2j
					; ReactionObject+117j
		call	GetDirectObj
		ret	nc
		jr	z, loc_A440
		ld	a, (PilotRating0)
		cp	28h ; '('
		jr	c, loc_A440
		set	2, (ix+21h)

loc_A440:				; CODE XREF: DirectObject+4j
					; DirectObject+Bj
		ld	de, 200h
		call	TestObjInRadius	; в радиусе реакции
		jr	c, MoveObjFromPlayer
		bit	5, (ix+26h)
		jr	z, loc_A455
		ld	d, 0Ah
		call	TestObjInRadius
		jr	c, MoveObjFromPlayer

loc_A455:				; CODE XREF: DirectObject+1Dj
		bit	2, (ix+21h)
		ret	z
		ld	a, (PilotMission3) ; =#06 при стыковке будет захват таргонами (вроде нереальная	ситуация)
					; =#04 в системе со танцией таргонов во	время войны
		cp	4
		jr	z, loc_A46F
		ld	a, (byte_606A)	; objCoriolis+oEnerg
		and	a
		jr	z, loc_A46F
		bit	3, (ix+21h)
		jr	z, MoveObjToPlayer
		jr	MoveObjFromPlayer
; ---------------------------------------------------------------------------

loc_A46F:				; CODE XREF: DirectObject+30j
					; DirectObject+36j
		ld	a, (ix+25h)
		and	7Fh ; ''
		cp	8
		jr	nc, MoveObjToPlayer
		set	7, (ix+25h)

MoveObjFromPlayer:			; CODE XREF: DirectObject+17j
					; DirectObject+24j ...
		push	ix
		call	HL_CrdNdiv4
		ld	(deltaCrdX), hl
		call	HL_CrdNdiv4
		ld	(deltaCrdY), hl
		call	HL_CrdNdiv4
		ld	(deltaCrdZ), hl
		pop	ix
		set	5, (ix+26h)
		jr	MoveObject1
; ---------------------------------------------------------------------------

MoveObjToPlayer:			; CODE XREF: DirectObject+3Cj
					; DirectObject+47j ...
		push	ix
		call	HL_CrdNdiv4_InvSign
		ld	(deltaCrdX), hl
		call	HL_CrdNdiv4_InvSign
		ld	(deltaCrdY), hl
		call	HL_CrdNdiv4_InvSign
		ld	(deltaCrdZ), hl
		pop	ix
		res	5, (ix+26h)

MoveObject1:				; CODE XREF: DirectObject+67j
					; MoveMissile+7Fj
		jp	MoveObject
; End of function DirectObject


; =============== S U B	R O U T	I N E =======================================


MoveMissile:				; CODE XREF: sub_9D40+DFp
		ld	a, (TimeWorkECM)
		and	a
		jr	z, loc_A4C4
		call	sub_7D3E
		ret	nz
		set	4, (ix+20h)
		ret
; ---------------------------------------------------------------------------

loc_A4C4:				; CODE XREF: MoveMissile+4j
		ld	a, c
		and	0Fh
		and	a
		jr	z, MoveObjToPlayer
		push	iy
		ld	iy, loc_6021
		ld	de, 27h	; '''
		sub	0Ah
		cpl
		ld	b, a

loc_A4D7:				; CODE XREF: MoveMissile+24j
		add	iy, de
		djnz	loc_A4D7
		push	ix
		push	iy
		call	sub_A53F
		ld	(deltaCrdX), hl
		call	sub_A53F
		ld	(deltaCrdY), hl
		call	sub_A53F
		ld	(deltaCrdZ), hl
		pop	iy
		pop	ix
		call	sub_A537
		jr	nz, loc_A532
		ld	hl, (deltaCrdY)
		call	sub_A537
		jr	nz, loc_A532
		ld	hl, (deltaCrdX)
		call	sub_F97E
		jr	nz, loc_A532
		bit	2, (iy+26h)
		jr	z, loc_A527
		ld	hl, PilotMission3 ; =#06 при стыковке будет захват таргонами (вроде нереальная ситуация)
					; =#04 в системе со танцией таргонов во	время войны
		call	sub_D6AA
		jr	nz, loc_A52B
		ld	hl, planetAlienItems
		xor	a
		ld	(hl), a
		ld	de, 0FFFFh
		call	j_IncBattleRating2
		set	7, (iy+26h)

loc_A527:				; CODE XREF: MoveMissile+59j
		set	4, (iy+20h)

loc_A52B:				; CODE XREF: MoveMissile+61j
		pop	iy
		ret
; ---------------------------------------------------------------------------

loc_A52E:				; CODE XREF: sub_9D40+1j
		ld	(hl), a
		jp	loc_9E5C
; ---------------------------------------------------------------------------

loc_A532:				; CODE XREF: MoveMissile+43j
					; MoveMissile+4Bj ...
		pop	iy
		jp	MoveObject1
; End of function MoveMissile


; =============== S U B	R O U T	I N E =======================================




; === ClearAroundObj === CODE XREF: RAM:7566p	Use_J_Drive+29p ...
ClearAroundObj:				; CODE XREF: RAM:7566p	Use_J_Drive+29p ...
		ld	ix, objCoriolis
		res	2, (ix+21h)
		xor	a
		ld	(PlayerPirate),	a
		ld	de, 27h	; '''
		ld	b, 7

loc_951A:				; CODE XREF: ClearAroundObj+16j
		add	ix, de
		ld	(ix+22h), a
		djnz	loc_951A
		ret
; End of function ClearAroundObj

; ---------------------------------------------------------------------------
byte_9522:	db 0			; DATA XREF: sub_9548:loc_9551r
					; sub_9579+6r ...
word_9523:	dw 8000h		; DATA XREF: sub_9579+30r sub_9579+47w ...
word_9525:	dw 0B27h		; DATA XREF: sub_9579:loc_95A6r
					; sub_9579+4Bw	...

; =============== S U B	R O U T	I N E =======================================


FX_Launch:				; CODE XREF: RAM:7532p
					; LaunchDocking+58p ...
		ld	a, (FlagsSetUp)
		bit	7, a
		ret	z
		call	sub_969B
		jr	loc_9551
; End of function FX_Launch

; ---------------------------------------------------------------------------
; START	OF FUNCTION CHUNK FOR sub_9296

loc_9532:				; CODE XREF: sub_9296+137j
		ld	a, (FlagsSetUp)
		bit	7, a
		ret	z
		call	sub_96A9
		jr	loc_9551
; END OF FUNCTION CHUNK	FOR sub_9296

; =============== S U B	R O U T	I N E =======================================




; === ClearMemory === CODE XREF: LaunchDocking+98p
ClearMemory:				; CODE XREF: LaunchDocking+98p
					; LaunchDocking+ADp ...
		xor	a

loc_9504:				; CODE XREF: ClearMemory+3j
		ld	(hl), a
		inc	hl
		djnz	loc_9504
		ret
; End of function ClearMemory


; =============== S U B	R O U T	I N E =======================================


ClearAroundObj:				; CODE XREF: RAM:7566p	Use_J_Drive+29p ...
		ld	ix, objCoriolis
		res	2, (ix+21h)
		xor	a
		ld	(PlayerPirate),	a
		ld	de, 27h	; '''
		ld	b, 7

loc_951A:				; CODE XREF: ClearAroundObj+16j
		add	ix, de
		ld	(ix+22h), a
		djnz	loc_951A
		ret
; End of function ClearAroundObj

; ---------------------------------------------------------------------------
byte_9522:	db 0			; DATA XREF: sub_9548:loc_9551r
					; sub_9579+6r ...
word_9523:	dw 8000h		; DATA XREF: sub_9579+30r sub_9579+47w ...
word_9525:	dw 0B27h		; DATA XREF: sub_9579:loc_95A6r
					; sub_9579+4Bw	...

; =============== S U B	R O U T	I N E =======================================


FX_Launch:				; CODE XREF: RAM:7532p
					; LaunchDocking+58p ...
		ld	a, (FlagsSetUp)
		bit	7, a
		ret	z
		call	sub_969B
		jr	loc_9551
; End of function FX_Launch

; ---------------------------------------------------------------------------
; START	OF FUNCTION CHUNK FOR sub_9296

loc_9532:				; CODE XREF: sub_9296+137j
		ld	a, (FlagsSetUp)
		bit	7, a
		ret	z
		call	sub_96A9
		jr	loc_9551
; END OF FUNCTION CHUNK	FOR sub_9296

; =============== S U B	R O U T	I N E =======================================




; === ClearScrBuffer === CODE XREF: sub_728D:loc_73C3p
ClearScrBuffer:				; CODE XREF: sub_728D:loc_73C3p
					; sub_728D+25Ap ...

; FUNCTION CHUNK AT DA5F SIZE 00000005 BYTES

		di
		ld	hl, 0CFE0h
		ld	de, 0C020h
		ld	bc, 5720h

loc_808A:				; CODE XREF: ClearScrBuffer+10j
		dec	l
		dec	e
		dec	c
		ld	a, (bc)
		ld	(hl), a
		ld	(de), a
		jr	nz, loc_808A
		ld	(loc_DA5F+1), sp
		ld	sp, hl
		ex	de, hl
		add	hl, hl
		ld	e, l
		ld	d, l
		ld	b, 7Dh ; '}'

loc_809D:				; CODE XREF: ClearScrBuffer+2Fj
		inc	e
		push	de
		dec	e
		push	de
		push	de
		push	de
		push	de
		push	de
		push	de
		push	de
		push	de
		push	de
		push	de
		push	de
		push	de
		push	de
		push	de
		push	hl
		djnz	loc_809D
		jp	loc_DA5F
; End of function ClearScrBuffer

; ---------------------------------------------------------------------------

loc_80B4:				; CODE XREF: sub_EB2F+4p
		cp	0Eh
		jr	nc, loc_80BC
		ld	b, a
		add	a, a
		add	a, b
		ret
; ---------------------------------------------------------------------------

loc_80BC:				; CODE XREF: RAM:80B6j
		ld	hl, 667h
		xor	a
		ret
; ---------------------------------------------------------------------------

SetMaxTonnage:				; CODE XREF: sub_5EC5+1Bp
					; UseEscapePod+3Fp ...
		ld	a, (PilotLargeCargo) ; установка максимальной грузоподъемности
		ld	b, a
		dec	b
		ld	a, 5

loc_80C8:				; CODE XREF: RAM:80CBj
		add	a, 0Fh
		inc	b
		jr	nz, loc_80C8
		ld	(PilotFreeCargo), a
		ret
; ---------------------------------------------------------------------------
word_80D1:	dw 411Ah		; DATA XREF: sub_813Ew	sub_813E+18r

; =============== S U B	R O U T	I N E =======================================

; проверка нажатия (и соответствующий переход) клавиш 1, 2, 3, 4, I, O,	P, K, L, enter



; === Cmd_2_BuyProducts === CODE XREF: RAM:FunctionKernelj
Cmd_2_BuyProducts:			; CODE XREF: RAM:FunctionKernelj

; FUNCTION CHUNK AT E707 SIZE 00000009 BYTES

		ld	a, (PilotExtWeapon) ; 7,=1 - ECM Jammer
					; 6,=1 - Cloaking device
					; 5,=1 - нельзя	купить товары(в	трюме беженцы или десант)
					; 4
					; 3 =1 3я миссия была выполнена
					; 2 =1 - третья	миссия выполнена
					; 1 =1 - второя	миссия выполнена
					; 0 =1 - первая	миссия выполнена
		bit	5, a
		jr	nz, loc_E6F1+1
		ld	de, 5B17h
		ld	hl, 201h
		call	GoMission1
		nop
		jr	nc, loc_E707
		call	sub_D162
		jr	sub_E6D6
; End of function Cmd_2_BuyProducts


; =============== S U B	R O U T	I N E =======================================




; === Cmd_3_SellProducrs === CODE XREF: RAM:D003j
Cmd_3_SellProducrs:			; CODE XREF: RAM:D003j
		ld	a, (PilotMission1)
		and	a
		jr	nz, loc_E8EA
		ld	hl, PilotExtWeapon ; 7,=1 - ECM	Jammer
					; 6,=1 - Cloaking device
					; 5,=1 - нельзя	купить товары(в	трюме беженцы или десант)
					; 4
					; 3 =1 3я миссия была выполнена
					; 2 =1 - третья	миссия выполнена
					; 1 =1 - второя	миссия выполнена
					; 0 =1 - первая	миссия выполнена
		call	sub_7FA1
		jr	z, loc_E8EA
		res	5, (hl)
		xor	a
		ld	(PilotMineralLife), a ;	<>0 в трюме есть минеральная жизнь
					; =#FF минеральная жизнь померла
		ld	a, 100
		ld	(cargoGemStones), a
		call	SetMaxTonnage	; установка максимальной грузоподъемности
		nop
		nop
		ld	de, 0F0Fh
		ld	hl, 700h
		call	sub_D148
		jp	Cmd_Ent_Inventory
; ---------------------------------------------------------------------------
unk_E8D8:	db 0F0h	; Ё		; DATA XREF: sub_E87E+5Eo
; ---------------------------------------------------------------------------

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




; === CopyBuf2Screen === CODE XREF: sub_728D+13Fp RAM:755Ap ...
CopyBuf2Screen:				; CODE XREF: sub_728D+13Fp RAM:755Ap ...
		ld	hl, 0C000h
		ld	de, 4020h
		call	sub_7FCD
		ld	hl, 0C100h
		ld	de, 4120h
; End of function CopyBuf2Screen


; =============== S U B	R O U T	I N E =======================================




; === CopyLocator2Scr === CODE XREF: sub_ED60:PrintObjScanerp
CopyLocator2Scr:			; CODE XREF: sub_ED60:PrintObjScanerp
					; DATA XREF: sub_7A84+7o
		ld	a, 7
		and	a
		jr	nz, loc_B3EA
		ld	a, (FlgSaveScaner)
		dec	a
		ret	nz
		ld	de, 502Ah
		ld	hl, byte_992B

loc_B3C2:				; CODE XREF: CopyLocator2Scr+36j
		ld	a, (hl)
		and	a
		jr	z, loc_B3EF
		push	hl
		ld	hl, loc_B38C
		ld	bc, 9

loc_B3CD:				; CODE XREF: CopyLocator2Scr+1Dj
		add	hl, bc
		rra
		jr	nc, loc_B3CD

loc_B3D1:				; CODE XREF: CopyLocator2Scr+32j
		push	hl
		push	de
		ld	c, 0Bh
		ldir
		pop	hl
		inc	h
		ex	af, af'
		ld	a, h
		and	7
		call	z, compDownScrAdr_HL
		ex	de, hl
		pop	hl
		ex	af, af'
		dec	a
		jr	nz, loc_B3D1
		pop	hl
		inc	hl
		jr	loc_B3C2
; ---------------------------------------------------------------------------

loc_B3EA:				; CODE XREF: CopyLocator2Scr+3j
		push	af
		call	CopyScaner2Scr
		pop	af

loc_B3EF:				; CODE XREF: CopyLocator2Scr+12j
		ld	d, 5Ah ; 'Z'
		ld	hl, unk_A89C

loc_B3F4:				; CODE XREF: CopyLocator2Scr+47j
		ld	e, (hl)
		inc	hl
		inc	e
		ret	z
		ld	(de), a
		jr	loc_B3F4
; End of function CopyLocator2Scr

; ---------------------------------------------------------------------------
		db  44h	; D
		db 0CBh	; ╦
		db  50h	; P
		db  20h
		db    2
		db 0FFh
		db  80h	; А
		db 0C0h	; └
		db 0E0h	; р
		db 0F0h	; Ё
		db 0F8h	; °
		db 0FCh	; №
		db 0FEh	; ■
		db  7Fh	; 
		db  40h	; @
		db  60h	; `
		db  70h	; p
		db  78h	; x
		db  7Ch	; |
		db  7Eh	; ~
		db  7Fh	; 
		db  3Fh	; ?
		db  20h
		db  30h	; 0
		db  38h	; 8
		db  3Ch	; <
		db  3Eh	; >
		db  3Fh	; ?
		db  3Fh	; ?
		db  1Fh
		db  10h
		db  18h
		db  1Ch
		db  1Eh
		db  1Fh
		db  1Fh
		db  1Fh
		db  0Fh
		db    8
		db  0Ch
		db  0Eh
		db  0Fh
		db  0Fh
		db  0Fh
		db  0Fh
		db    7
		db    4
		db    6
		db    7
		db    7
		db    7
		db    7
		db    7
		db    3
		db    2
		db    3
		dw 303h
		db    3
		db    3
		db    3
		db    1
		db    1
		db    1
		db    1
		db    1
		db    1
		db    1
		db    1

; =============== S U B	R O U T	I N E =======================================




; === CorrAxirY_SKO === CODE XREF: sub_9D40+ECp sub_A28D+17p
CorrAxirY_SKO:				; CODE XREF: sub_9D40+ECp sub_A28D+17p
		push	ix
		push	ix
		ex	(sp), iy
		ld	de, 9
		add	ix, de
		ld	e, 0Fh
		add	iy, de
		call	ScalarMultVectors
		ld	e, a
		call	sub_AE8A
		call	sub_AE8A
		call	sub_AE8A
		pop	iy
		pop	ix
		ret
; End of function CorrAxirY_SKO


; =============== S U B	R O U T	I N E =======================================




; === Div_BC_HLAdivDE === CODE XREF: CompasCrdXplanet:loc_82BBp
Div_BC_HLAdivDE:			; CODE XREF: CompasCrdXplanet:loc_82BBp
					; sub_96DC+57p	...
		jp	Div_BC_HLAdivDE_1
; End of function Div_BC_HLAdivDE

; ---------------------------------------------------------------------------
		db 0C3h	; ├
		db  44h	; D
		db 0B0h	; ░
		db 0C3h	; ├
		db 0B4h	; ┤
		db 0B7h	; ╖
		db    0


; === Div_BC_HLAdivDE_1 === CODE XREF: Div_BC_HLAdivDEj
Div_BC_HLAdivDE_1:			; CODE XREF: Div_BC_HLAdivDEj
					; CaclNormalVector+9p ...
		ld	bc, 0FFFEh

loc_AFAB:				; CODE XREF: Div_BC_HLAdivDE_1+14j
					; Div_BC_HLAdivDE_1+1Fj
		add	a, a
		rl	l
		rl	h
		jr	c, loc_AFBE
		sbc	hl, de
		jr	nc, loc_AFC1
		add	hl, de
		sla	c
		rl	b
		ret	nc
		jr	loc_AFAB
; ---------------------------------------------------------------------------

loc_AFBE:				; CODE XREF: Div_BC_HLAdivDE_1+8j
		and	a
		sbc	hl, de

loc_AFC1:				; CODE XREF: Div_BC_HLAdivDE_1+Cj
		scf
		rl	c
		rl	b
		ret	nc
		jr	loc_AFAB
; End of function Div_BC_HLAdivDE_1


; =============== S U B	R O U T	I N E =======================================




; === Div_C_DEc_HLa === CODE XREF: PrnObjIfVisible:loc_A71Ep
Div_C_DEc_HLa:				; CODE XREF: PrnObjIfVisible:loc_A71Ep
					; PrnObjIfVisible+4Fp ...
		ld	b, 9

loc_AC6B:				; CODE XREF: Div_C_DEc_HLa+10j
		dec	b
		jr	z, loc_AC84
		rla
		adc	hl, hl
		jr	c, loc_AC80
		rl	c
		rl	e
		rl	d
		jr	nc, loc_AC6B
		rr	d
		rr	e
		scf

loc_AC80:				; CODE XREF: Div_C_DEc_HLa+8j
		rr	h
		rr	l

loc_AC84:				; CODE XREF: Div_C_DEc_HLa+3j
		ld	a, d
		ld	c, 0FFh
		cp	h
		ret	nc
		ld	b, h
		dec	c

loc_AC8B:				; CODE XREF: Div_C_DEc_HLa+2Dj
					; Div_C_DEc_HLa+34j
		sla	e
		rla
		jr	c, loc_AC98
		cp	b
		jr	nc, loc_AC98
		sla	c
		ret	nc
		jr	loc_AC8B
; ---------------------------------------------------------------------------

loc_AC98:				; CODE XREF: Div_C_DEc_HLa+25j
					; Div_C_DEc_HLa+28j
		sub	b
		scf
		rl	c
		ret	nc
		jr	loc_AC8B
; End of function Div_C_DEc_HLa

; ---------------------------------------------------------------------------


; === ECM_On === CODE XREF: sub_B7F4+1Bj
ECM_On:					; CODE XREF: sub_B7F4+1Bj
		ld	a, c
		and	a
		ret	z
		bit	4, b
		ret	z
		or	80h ; 'А'
		ld	(PilotECMSystem), a
		ld	a, 0Ah
		ld	(TimeWorkECM), a
		ret
; End of function sub_B7F4

; ---------------------------------------------------------------------------
		db    0


; === EnergyRecovery === CODE XREF: Damage2Pilot:loc_9A6Bp
EnergyRecovery:				; CODE XREF: Damage2Pilot:loc_9A6Bp
		ld	a, (byte_D08B)
		and	a
		call	nz, loc_8481
; End of function EnergyRecovery


; =============== S U B	R O U T	I N E =======================================




; === FX_Launch === CODE XREF: RAM:7532p
FX_Launch:				; CODE XREF: RAM:7532p
					; LaunchDocking+58p ...
		ld	a, (FlagsSetUp)
		bit	7, a
		ret	z
		call	sub_969B
		jr	loc_9551
; End of function FX_Launch

; ---------------------------------------------------------------------------
; START	OF FUNCTION CHUNK FOR sub_9296

loc_9532:				; CODE XREF: sub_9296+137j
		ld	a, (FlagsSetUp)
		bit	7, a
		ret	z
		call	sub_96A9
		jr	loc_9551
; END OF FUNCTION CHUNK	FOR sub_9296

; =============== S U B	R O U T	I N E =======================================




; === FastSetAtrInViewScr === CODE XREF: sub_70A9+2p sub_70A9+Bj
FastSetAtrInViewScr:			; CODE XREF: sub_70A9+2p sub_70A9+Bj
		ld	a, (PilotMission1)
		and	a
		jr	z, loc_70C4
		ld	a, l
		and	111010b
		ld	l, a

loc_70C4:				; CODE XREF: FastSetAtrInViewScr+4j
		halt
		ld	a, l
		ld	hl, 59FEh
		ld	b, 0Fh

loc_70CB:				; CODE XREF: FastSetAtrInViewScr+1Fj
		ld	(hl), a
		ld	d, h
		ld	e, l
		dec	de
		push	bc
		ld	bc, 1Dh
		lddr
		dec	hl
		dec	hl
		dec	hl
		pop	bc
		djnz	loc_70CB
		ret
; End of function FastSetAtrInViewScr

; ---------------------------------------------------------------------------
byte_70DC:	db 5Eh			; DATA XREF: ProgressMissionWaro
		db 43h
		db 0FCh
		db 0FFh
		db 0C3h
		db 66h
		db 73h


; === Fill_HL_A_Len_B === CODE XREF: FillAttrLineFrom_HL+2p
Fill_HL_A_Len_B:			; CODE XREF: FillAttrLineFrom_HL+2p
					; Fill_HL_A_Len_B+2j ...
		ld	(hl), a
		inc	hl
		djnz	Fill_HL_A_Len_B
		ret
; End of function Fill_HL_A_Len_B


; =============== S U B	R O U T	I N E =======================================


SetAttrFrom_DE:				; CODE XREF: RAM:E719p	sub_E87E+61p ...
		ld	hl, 5841h

loc_D56B:				; CODE XREF: SetAttrFrom_DE+22j
		ld	a, (de)
		inc	de
		push	af
		and	0F8h ; '°'
		rrca
		rrca
		rrca
		ld	b, a
		push	hl
		ld	a, 5
		call	Fill_HL_A_Len_B
		pop	hl
		pop	af
		and	7
		jr	z, loc_D58C
		ld	b, a

loc_D581:				; CODE XREF: SetAttrFrom_DE:loc_D588j
		ld	a, 20h ; ' '
		add	a, l
		ld	l, a
		jr	nc, loc_D588
		inc	h

loc_D588:				; CODE XREF: SetAttrFrom_DE+1Dj
		djnz	loc_D581
		jr	loc_D56B
; ---------------------------------------------------------------------------

loc_D58C:				; CODE XREF: SetAttrFrom_DE+16j
		ld	hl, 5822h
		ld	a, 7
		ld	(hl), a
		inc	hl
		ld	(hl), a
		ret
; End of function SetAttrFrom_DE

; ---------------------------------------------------------------------------
; START	OF FUNCTION CHUNK FOR sub_D470

loc_D595:				; CODE XREF: sub_D470+1Bj
		sub	0D7h ; '╫'
		ld	hl, unk_FD99
		jr	loc_D5A1
; END OF FUNCTION CHUNK	FOR sub_D470
; ---------------------------------------------------------------------------
; START	OF FUNCTION CHUNK FOR PrintMessage1_A

loc_D59C:				; CODE XREF: PrintMessage1_A+15j
		sub	80h ; 'А'
		ld	hl, byte_FDB3

loc_D5A1:				; CODE XREF: sub_D470+12Aj
		add	a, a
		ld	d, 0
		ld	e, a
		add	hl, de
		ld	a, (hl)
		push	hl
		call	sub_D499
		pop	hl
		inc	hl
		ld	a, (hl)
		cp	3Fh ; '?'
		ret	z
		jp	sub_D499
; END OF FUNCTION CHUNK	FOR PrintMessage1_A

; =============== S U B	R O U T	I N E =======================================

; гибель минеральной жизни при недостаточном количестве
; Вых: z=1 есть	минеральная жизнь в трюме
;	=0 минеральная жизнь померла
;	   нет минеральной жизни



; === FindObjInExtSlot === CODE XREF: sub_B71C+2p sub_B71C+16p	...
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


; === FindSlot4Obj === CODE XREF: ReCalcCrdObj_coriolis+81p
FindSlot4Obj:				; CODE XREF: ReCalcCrdObj_coriolis+81p
					; SetPlayerObj+1p ...
		ld	ix, objCoriolis
		ld	de, 27h	; '''
		ld	b, 6

loc_B93F:				; CODE XREF: FindSlot4Obj+10j
		add	ix, de
		ld	a, (ix+22h)
		and	a
		ret	z
		djnz	loc_B93F
		inc	b
		ret
; End of function FindSlot4Obj


; =============== S U B	R O U T	I N E =======================================


SetPlayerObj:				; CODE XREF: SetPlayerAlloys:SetPlayerCargop
		ld	c, a
		call	FindSlot4Obj
		ret	nz
		ld	a, c
; End of function SetPlayerObj


; =============== S U B	R O U T	I N E =======================================


SetPlayerObjSlot:			; CODE XREF: sub_9391+18j
					; CallFunctionObj_1+29Cp
		call	sub_5EC5	; запуск ракеты	игроком
					; запуск спасательной капсулы (установка кобры)
		res	7, (ix+0Eh)
		res	7, (ix+16h)
		ld	(ix+0),	a
		ld	(ix+1),	a
		ld	(ix+4),	a
		ld	(ix+6),	a
		ld	(ix+7),	a
		ret
; End of function SetPlayerObjSlot

; ---------------------------------------------------------------------------
; START	OF FUNCTION CHUNK FOR CallFunctionObj_1

Set3_4Targons:				; CODE XREF: CallFunctionObj_1+Cj
		call	SetTargonStation ; Установка станции таргонов в	межпланетном пространстве для последней	подмиссии в войне с таргонами
		and	3
		add	a, 3
		ld	c, a

loc_B973:				; CODE XREF: CallFunctionObj_1+132j
		call	FindSlot4Obj
		ret	nz
		ld	a, 0Fh
		push	bc
		call	SetObjInSlot	; a - номер объекта для	установки
		set	0, (ix+25h)
		call	IntRnd_A
		and	0Fh
		or	4
		ld	(ix+7),	a
		rra
		rra
		rra
		and	1
		ld	(ix+1),	a
		xor	a
		ld	(ix+4),	a
		call	RndDirectionObj
		pop	bc
		dec	c
		jr	nz, loc_B973
		ret
; ---------------------------------------------------------------------------

SetObjMissile:				; CODE XREF: CallFunctionObj_1+10j
		ld	a, c		; Запуск ракеты
					; Вх:  с - цель	ракеты
					;	 =0 игрок (запустил противник)
					;	<>0 номер объекта по которому запустил ракету игрок
					; Вых: z=1 успешно
		and	a
		jr	z, loc_B9CF
		ld	ix, objCoriolis
		ld	de, 111h
		add	ix, de
		ld	a, (ix+22h)
		and	a
		jr	z, loc_B9B6
		call	FindSlot4Obj
		ret	nz

loc_B9B6:				; CODE XREF: CallFunctionObj_1+146j
		ld	a, 1
		push	bc
		call	sub_9391	; c = номер объекта по которому	запустил ракету	игрок
		pop	bc
		ld	a, 6Ah ; 'j'
		ld	(ix+6),	a
		ld	a, 29h ; ')'
		ld	(ix+3),	a
		ld	a, 80h ; 'А'
		ld	(ix+5),	a
		ld	a, c
		jr	loc_B9DE
; ---------------------------------------------------------------------------

loc_B9CF:				; CODE XREF: CallFunctionObj_1+137j
		ld	a, 1
		call	ShipSetObject
; ---------------------------------------------------------------------------
		ret	nz
		ld	a, (HighDeltaSpeed)
		add	a, (ix+1Bh)
		ld	(ix+1Bh), a

loc_B9DE:				; CODE XREF: CallFunctionObj_1+163j
		ld	(ix+25h), c
		set	0, (ix+26h)
		xor	a
		ret
; ---------------------------------------------------------------------------

ShipSetShip:				; CODE XREF: CallFunctionObj_1+16j
		ld	a, 8
		dec	c
		jr	z, loc_BA0D
		call	IntRnd_A
		and	1
		add	a, 9
		dec	c
		jr	z, loc_BA0D
		ld	a, 12h
		dec	c
		jr	z, loc_BA0D
		ld	a, 0Fh
		dec	c
		jr	z, loc_BA0D
		dec	c
		ret	nz
		call	IntRnd_A
		bit	0, a
		ld	a, 10h
		jr	z, loc_BA0D
		ld	a, 11h

loc_BA0D:				; CODE XREF: CallFunctionObj_1+180j
					; CallFunctionObj_1+18Aj ...
		push	ix
		call	ShipSetObject
; ---------------------------------------------------------------------------
		pop	iy
		ret	nz
		bit	2, (iy+26h)
		call	z, RotateSKOobj
		bit	2, (iy+21h)
		ret	z
		set	2, (ix+21h)
		xor	a
		ret
; END OF FUNCTION CHUNK	FOR CallFunctionObj_1

; =============== S U B	R O U T	I N E =======================================


RotateSKOobj:				; CODE XREF: CallFunctionObj_1+1AFp
					; SetShipMinAlloys+Cp

; FUNCTION CHUNK AT BA58 SIZE 0000002B BYTES

		ld	a, (byte_B866)
		xor	1
		ld	(byte_B866), a
		jr	z, loc_BA58
		push	ix
		call	sub_BA3F
		call	sub_BA3F
		call	sub_BA3F
		pop	ix
		ret
; End of function RotateSKOobj


; =============== S U B	R O U T	I N E =======================================




; === FunctionKernel === DATA XREF: sub_813E+Ao
FunctionKernel:				; DATA XREF: sub_813E+Ao
		jp	Cmd_2_BuyProducts
; ---------------------------------------------------------------------------
		jp	Cmd_3_SellProducrs
; ---------------------------------------------------------------------------
		jp	Cmd_4_EquipShip
; ---------------------------------------------------------------------------
		jp	Cmd_I_GlobalMap
; ---------------------------------------------------------------------------
		jp	Cmd_O_LocalMap
; ---------------------------------------------------------------------------
		jp	Cmd_P_InfoPlanet
; ---------------------------------------------------------------------------
		jp	sub_EF48

; =============== S U B	R O U T	I N E =======================================

; Attributes: thunk



; === GetDirectObj === CODE XREF: DirectObjectp
GetDirectObj:				; CODE XREF: DirectObjectp
		ld	hl, (word_B452)
		ld	de, 8
		add	hl, de
		ld	a, (hl)
		and	0Fh
		cp	1
		bit	1, (ix+21h)
		ret
; End of function GetDirectObj

; ---------------------------------------------------------------------------
		db    0
byte_B4D6:	db 0			; DATA XREF: FireLaser+114w
					; FireLaser+139r ...
byte_B4D7:	db 0			; DATA XREF: FireLaserr FireLaser+5w ...

; =============== S U B	R O U T	I N E =======================================


CollisionWithObj:			; CODE XREF: sub_9D40:loc_9E41p

; FUNCTION CHUNK AT D0E4 SIZE 00000003 BYTES
; FUNCTION CHUNK AT D183 SIZE 00000032 BYTES

		ld	a, (PlayerGameOver)
		and	a
		ret	nz
		ld	d, 0
		ld	e, (iy+1)
		bit	0, (ix+26h)
		jr	z, loc_B4F0
		ld	a, (ix+25h)
		and	a
		jr	nz, loc_B4F0
		ld	d, 1

loc_B4F0:				; CODE XREF: CollisionWithObj+Ej
					; CollisionWithObj+14j
		bit	1, (ix+26h)
		jr	z, loc_B4F8
		ld	e, 0B4h	; '┤'

loc_B4F8:				; CODE XREF: CollisionWithObj+1Cj
		ld	l, (ix+0)
		ld	h, (ix+1)
		sbc	hl, de
		ret	nc
		and	a
		ld	l, (ix+3)
		ld	h, (ix+4)
		sbc	hl, de
		ret	nc
		and	a
		ld	l, (ix+6)
		ld	h, (ix+7)
		sbc	hl, de
		ret	nc
		ld	a, (ix+8)
		add	a, a
		ret	nz
		bit	1, (ix+26h)
		jr	z, loc_B55D	; столкновение со станцией
		bit	7, (ix+5)
		jp	z, objCollision
		ld	a, (PilotFuelScoops)
		and	a
		jr	z, objCollision
		xor	a
		ld	(ix+22h), a
		call	OffTargetMissiles_2
		ld	a, (PilotFreeCargo)
		and	a
		jr	z, objCollision
		ld	a, (iy+0)
		and	0F0h ; 'Ё'
		jr	z, loc_B548
		rrca
		rrca
		rrca
		rrca
		inc	a
		jr	AddFoundCargo2
; ---------------------------------------------------------------------------

loc_B548:				; CODE XREF: CollisionWithObj+67j
		ld	a, (Mission2)
		sub	3
		jr	nz, loc_B555
		ld	(Mission2), a
		dec	a
		jr	AddFoundCargo2
; ---------------------------------------------------------------------------

loc_B555:				; CODE XREF: CollisionWithObj+75j
		call	IntRnd_A
		and	7

AddFoundCargo2:				; CODE XREF: CollisionWithObj+6Ej
					; CollisionWithObj+7Bj
		jp	AddFoundCargo
; ---------------------------------------------------------------------------

loc_B55D:				; CODE XREF: CollisionWithObj+46j
		bit	2, (ix+26h)	; столкновение со станцией
		jr	z, objCollision
		bit	7, (ix+0Eh)
		jr	z, killPlayer
		call	sub_8429
		jp	c, loc_B595
		bit	2, (ix+21h)
		jr	nz, killPlayer
		ld	a, (ix+0Ah)
		add	a, a
		cp	0DCh ; '▄'
		jr	nc, killPlayer
		ld	b, a
		ld	a, (ix+0Ch)
		add	a, a
		cp	0DCh ; '▄'
		jr	nc, killPlayer
		cp	3Ch ; '<'
		jr	nc, loc_B595
		ld	a, b
		cp	3Ch ; '<'
		jr	nc, loc_B595
		ld	a, 1
		ld	(ManualDocking), a
		ret
; ---------------------------------------------------------------------------

loc_B595:				; CODE XREF: CollisionWithObj+94j
					; CollisionWithObj+B0j	...
		ld	hl, nBeepFX
		set	6, (hl)
		ld	hl, HighDeltaSpeed
		ld	a, (DamageFrontShield)
		add	a, 0Ch
		add	a, (hl)
		add	a, (hl)
		ld	(DamageFrontShield), a
		ret
; ---------------------------------------------------------------------------

objCollision:				; CODE XREF: CollisionWithObj+4Cj
					; CollisionWithObj+53j	...
		xor	a
		call	sub_B743
		ld	b, (iy+1)
		bit	6, (ix+26h)
		jr	z, loc_B5BF
		ld	a, (Mission2)
		and	a
		jr	z, loc_B5BF
		xor	a
		ld	(Mission2), a

loc_B5BF:				; CODE XREF: CollisionWithObj+DBj
					; CollisionWithObj+E1j
		ld	hl, nBeepFX
		set	2, (hl)
		bit	7, (ix+8)
		jr	z, loc_B5D8
		ld	a, (iy+15h)
		or	b
		ld	(DamageRearShield), a
		jr	OffTargetMissiles
; ---------------------------------------------------------------------------

killPlayer:				; CODE XREF: CollisionWithObj+8Fj
					; CollisionWithObj+9Bj	...
		inc	a
		ld	(PlayerGameOver), a
		ret
; ---------------------------------------------------------------------------

loc_B5D8:				; CODE XREF: CollisionWithObj+F0j
		ld	a, (iy+15h)
		or	b
		ld	(DamageFrontShield), a
; End of function CollisionWithObj


; =============== S U B	R O U T	I N E =======================================


OffTargetMissiles:			; CODE XREF: CounterShipHyperJump+18p
					; sub_9D40+A5p	...
		ld	a, (InverseNumSlotIX)
		ld	h, a
		push	iy
		ld	iy, objCoriolis
		ld	de, 27h	; '''
		ld	b, 7

loc_B5EE:				; CODE XREF: OffTargetMissiles:loc_B600j
		add	iy, de
		ld	a, (iy+22h)
		and	a
		jr	z, loc_B600
		ld	a, (iy+25h)
		xor	h
		call	z, sub_8C29
		call	sub_8AA3

loc_B600:				; CODE XREF: OffTargetMissiles+15j
		djnz	loc_B5EE
		pop	iy
		ld	a, (Missile1)
		cp	2
		ret	nz
		ld	a, (nObjMissile)
		sub	h
		ret	nz
		ld	(Missile1), a
		ld	hl, UntargetMissile
		set	7, (hl)
		ret
; End of function OffTargetMissiles


; =============== S U B	R O U T	I N E =======================================

; Attributes: noreturn

SetShipwreck2:				; CODE XREF: SetShipwreckj
		ld	hl, nBeepFX
		set	2, (hl)
		xor	a
		ld	(WhatOnScreen),	a
		ld	ix, loc_6021
		ld	de, 27h	; '''
		ld	b, 8

loc_B62A:				; CODE XREF: SetShipwreck2+17j
		add	ix, de
		ld	(ix+22h), a
		djnz	loc_B62A
		ld	a, 16h
		call	sub_B867
; End of function SetShipwreck2

; ---------------------------------------------------------------------------
		ld	a, 15h
		call	sub_B867
; ---------------------------------------------------------------------------
		ret

; =============== S U B	R O U T	I N E =======================================


FireLaser:				; CODE XREF: RAM:B7F1p

; FUNCTION CHUNK AT B749 SIZE 00000050 BYTES

		ld	a, (byte_B4D7)
		ld	c, a
		xor	a
		ld	(byte_B4D7), a
		ld	a, (TempLaser)
		cp	2
		jr	nc, loc_B64D
		ld	a, 2

loc_B64D:				; CODE XREF: FireLaser+Dj
		sub	2
		ld	(TempLaser), a
		ld	hl, ControlKey3
		bit	0, (hl)
		ret	z
		res	0, (hl)
		cp	90h ; 'Р'
		ret	nc
		ld	b, a
		ld	a, (WhatOnScreen)
		ld	hl, PilotFrontLaser
		ld	d, 0
		ld	e, a
		add	hl, de
		ld	a, (hl)
		and	a
		ret	z
		ld	e, c
		ld	c, a
		dec	a
		jr	nz, loc_B672
		ld	d, 1

loc_B672:				; CODE XREF: FireLaser+32j
		sub	3
		jr	nz, loc_B678
		ld	d, 7

loc_B678:				; CODE XREF: FireLaser+38j
		ld	a, e
		and	e
		ld	a, 1
		ld	(byte_B4D7), a
		jr	z, loc_B686
		ld	a, (FrameCounter)
		and	d
		ret	nz

loc_B686:				; CODE XREF: FireLaser+43j
		ld	hl, DamageEnergyUnit
		inc	(hl)
		call	sub_9389
		ld	hl, nBeepFX
		set	4, (hl)
		ld	a, c
		sub	3
		ld	a, 0Fh
		jr	c, loc_B69F
		jr	z, loc_B69D
		add	a, 1Bh

loc_B69D:				; CODE XREF: FireLaser+5Dj
		add	a, 8

loc_B69F:				; CODE XREF: FireLaser+5Bj
		ld	c, a
		ld	a, d
		dec	a
		and	1
		rla
		add	a, 8
		add	a, b
		cp	0A0h ; 'а'
		jr	c, loc_B6AE
		ld	a, 0A0h	; 'а'

loc_B6AE:				; CODE XREF: FireLaser+6Ej
		ld	(TempLaser), a
		ld	a, (ObjInTarget)
		and	a
		jp	z, PrnLaser
		dec	a
		sub	9
		cpl
		ld	ix, objCoriolis
		ld	de, 27h	; '''

loc_B6C3:				; CODE XREF: FireLaser+8Cj
		dec	a
		jr	z, loc_B6CA
		add	ix, de
		jr	loc_B6C3
; ---------------------------------------------------------------------------

loc_B6CA:				; CODE XREF: FireLaser+88j
		call	sub_F9E1
		nop
		ld	hl, nBeepFX
		set	3, (hl)
		bit	2, (ix+26h)
		jr	nz, PrnLaser
		bit	5, (ix+20h)
		jr	nz, PrnLaser
		ld	a, (ix+25h)
		ld	b, a

loc_B6E3:
		sub	0Eh
		jr	nc, loc_B6EB
		and	1
		add	a, 6

loc_B6EB:				; CODE XREF: FireLaser+A9j
		rla
		sla	b
		rra
		ld	(ix+25h), a
		ld	a, (ix+22h)
		cp	0FFh
		jr	z, PrnLaser
		sub	c
		ld	(ix+22h), a
		jr	z, loc_B701
		jr	nc, PrnLaser

loc_B701:				; CODE XREF: FireLaser+C1j
		xor	a
		dec	a
		set	4, (ix+20h)
		ld	(ix+22h), a
		call	DestroyObj
; ---------------------------------------------------------------------------
		jr	PrnLaser
; End of function FireLaser

; [00000003 BYTES: COLLAPSED FUNCTION j_DestroyObj. PRESS KEYPAD "+" TO EXPAND]

; =============== S U B	R O U T	I N E =======================================


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



; === IncBattleRating === CODE XREF: sub_9D40+BBp
IncBattleRating:			; CODE XREF: sub_9D40+BBp
		ld	iy, (word_B452)
		ld	a, (iy+8)
		and	0Fh
		jp	z, j_IncBattleRating2
		ld	(ix+21h), a
		ret
; End of function IncBattleRating


; =============== S U B	R O U T	I N E =======================================

; Attributes: noreturn

DestroyObj:				; CODE XREF: FireLaser+CEp
					; j_DestroyObjj
		bit	6, (ix+26h)
		jr	z, loc_7DDD
		ld	a, (Mission2)
		and	a
		jr	z, loc_7DDD

loc_7DD1:
		cp	3
		jr	z, loc_7DDD
		ld	a, 3
		ld	(Mission2), a
		add	a, a
		jr	loc_7DFF
; ---------------------------------------------------------------------------

loc_7DDD:				; CODE XREF: DestroyObj+4j
					; DestroyObj+Aj ...
		ld	h, (ix+24h)
		ld	l, (ix+23h)
		ld	a, (hl)
		and	0Fh
		ld	a, 6
		jr	nz, loc_7DFF
		ld	de, 7
		add	hl, de
		ld	a, (hl)
		cp	55h ; 'U'
		ld	a, 5
		jr	nc, loc_7DFF
		ld	a, (hl)
		cp	3Ch ; '<'
		ret	nz
		ld	a, c
		cp	32h ; '2'
		ret	nz
		ld	a, 8

loc_7DFF:				; CODE XREF: DestroyObj+16j
					; DestroyObj+23j ...
		jp	sub_B867
; End of function DestroyObj


; =============== S U B	R O U T	I N E =======================================




; === IncBattleRating2 === CODE XREF: j_IncBattleRating2j
IncBattleRating2:			; CODE XREF: j_IncBattleRating2j
					; sub_D6AA+49p
		ld	hl, PilotRating0
		ld	a, (hl)
		add	a, e
		ld	(hl), a
		inc	hl
		ld	a, (hl)
		adc	a, d
		ld	(hl), a
		ret	nc
		inc	hl
		call	sub_B24E	; вызывается при увеличении 3го	байта рейтинга
		nop
		jr	loc_D1E6
; ---------------------------------------------------------------------------

AddCashObj2:				; CODE XREF: RAM:AddCashObj3j
		ld	a, d
		or	e
		ret	z
		ld	(LastBonusCash), de
		call	IncCASH
		ld	hl, CashBonus

loc_D1E6:				; CODE XREF: IncBattleRating2+10j
		set	7, (hl)
		ret
; End of function IncBattleRating2


; =============== S U B	R O U T	I N E =======================================




; === IncCASH === CODE XREF: IncBattleRating2+19p
IncCASH:				; CODE XREF: IncBattleRating2+19p
					; sub_E935+25p	...
		ld	hl, (PilotCash1)
		add	hl, de
		ld	(PilotCash1), hl
		and	a
		ld	de, 10000
		sbc	hl, de
		ret	c
		ld	(PilotCash1), hl
		ld	de, 1

loc_D3E7:				; CODE XREF: RAM:EB11p
		ld	hl, (PilotCash2low)
		add	hl, de
		ld	(PilotCash2low), hl
		ret
; End of function IncCASH


; =============== S U B	R O U T	I N E =======================================




; === InitSpeedAngle === CODE XREF: sub_9296+5p
InitSpeedAngle:				; CODE XREF: sub_9296+5p
		call	InitSpeedAngle2
		call	SetVars4War	; Установка переменных,	после влёта в систему своим ходом
					; Вх:  ---
					; Вых: hl,de,b,a - не определены
					; Вых: z=1 есть	минеральная жизнь в трюме
					;	=0 минеральная жизнь померла
					;	   нет минеральной жизни
		ret	nz
		ld	hl, cargoMinerals ; кормление минеральной жизни
		ld	b, (hl)
		ld	l, 7Ah ; 'z'    ; cargoGemStones
		ld	a, (hl)
		sub	b
		jr	nc, loc_6ECF
		ld	l, 87h ; 'З'    ; PilotMineralLife

loc_6ECF:				; CODE XREF: InitSpeedAngle+Fj
		ld	(hl), a
		ret	c
		jp	loc_7344
; End of function InitSpeedAngle

; ---------------------------------------------------------------------------

loc_6ED4:				; CODE XREF: RAM:loc_EE8Cp
		call	TestPlaneInZoneWar ; Печать описания планеты
; ---------------------------------------------------------------------------
		db 3Ch
; ---------------------------------------------------------------------------
		ld	hl, (PlanetData1)
		ret	nc
		call	TestPlaneInZoneWar ; Определяем	находится ли пилот/планета в заданной зоне войны
					; Вх:  (sp) =#00 зона оккупированная таргонами
					;	    =#3C зона линии фронта (активные боевые действия)
					;	    =#78 прифронтовая зона
					; Вых: cy =1 координаты	в пределах проверяемой зоны
					;	  =0 пилот в другой галактике галактике
					;	     война окончена/не начиналась
					;	     координаты	слишком	далеко от левого края
					;      hl,de,b,a - не определены
; ---------------------------------------------------------------------------
		db 0
; ---------------------------------------------------------------------------
		ld	a, 57h ; 'W'
		sbc	a, 0
		pop	hl
		jp	PrintMessage2
; ---------------------------------------------------------------------------

loc_6EE8:				; CODE XREF: RAM:loc_E9DFp
		ld	hl, sub_E36B
		push	hl
		call	PrintSymbol_SP
; ---------------------------------------------------------------------------
		db 20h
; ---------------------------------------------------------------------------
		ld	a, (PilotPlanetTechLvl)
		cp	0Eh
		ret	c
		inc	a
		call	PrintNumber_A
		call	SetCrdX_SP
; ---------------------------------------------------------------------------
		db 4
; ---------------------------------------------------------------------------
		call	PrintSymbol_SP
; ---------------------------------------------------------------------------
		db 0F6h
; ---------------------------------------------------------------------------
		call	SetCrdX_SP
; ---------------------------------------------------------------------------
		db 16h
; ---------------------------------------------------------------------------
		ld	hl, 21h	; '!'
		ld	d, h
		ld	e, h
		call	PrintNumber_HLDE
		jp	PrnCrd_CRLF
; ---------------------------------------------------------------------------


; === InitSpeedAngle2 === CODE XREF: InitSpeedAnglep RAM:736Ap ...
InitSpeedAngle2:			; CODE XREF: InitSpeedAnglep RAM:736Ap ...
		ld	b, 5
		call	sub_F9AB
		call	ClearMemory
		ld	(hl), 0Ch

loc_94FE:				; CODE XREF: PrnCocpit_Ifn+6p
					; sub_813E+1Cj
		ld	b, 5
		ld	hl, ControlKey1
; End of function InitSpeedAngle2


; =============== S U B	R O U T	I N E =======================================


ClearMemory:				; CODE XREF: LaunchDocking+98p
					; LaunchDocking+ADp ...
		xor	a

loc_9504:				; CODE XREF: ClearMemory+3j
		ld	(hl), a
		inc	hl
		djnz	loc_9504
		ret
; End of function ClearMemory


; =============== S U B	R O U T	I N E =======================================


ClearAroundObj:				; CODE XREF: RAM:7566p	Use_J_Drive+29p ...
		ld	ix, objCoriolis
		res	2, (ix+21h)
		xor	a
		ld	(PlayerPirate),	a
		ld	de, 27h	; '''
		ld	b, 7

loc_951A:				; CODE XREF: ClearAroundObj+16j
		add	ix, de
		ld	(ix+22h), a
		djnz	loc_951A
		ret
; End of function ClearAroundObj

; ---------------------------------------------------------------------------
byte_9522:	db 0			; DATA XREF: sub_9548:loc_9551r
					; sub_9579+6r ...
word_9523:	dw 8000h		; DATA XREF: sub_9579+30r sub_9579+47w ...
word_9525:	dw 0B27h		; DATA XREF: sub_9579:loc_95A6r
					; sub_9579+4Bw	...

; =============== S U B	R O U T	I N E =======================================


FX_Launch:				; CODE XREF: RAM:7532p
					; LaunchDocking+58p ...
		ld	a, (FlagsSetUp)
		bit	7, a
		ret	z
		call	sub_969B
		jr	loc_9551
; End of function FX_Launch

; ---------------------------------------------------------------------------
; START	OF FUNCTION CHUNK FOR sub_9296

loc_9532:				; CODE XREF: sub_9296+137j
		ld	a, (FlagsSetUp)
		bit	7, a
		ret	z
		call	sub_96A9
		jr	loc_9551
; END OF FUNCTION CHUNK	FOR sub_9296

; =============== S U B	R O U T	I N E =======================================




; === IntRnd_A === CODE XREF: sub_78B1+10Ap
IntRnd_A:				; CODE XREF: sub_78B1+10Ap
					; sub_78B1+10Fp ...
		push	hl
		push	bc
		ld	hl, (word_B7AA)
		ld	a, h
		adc	a, a
		ld	b, a
		adc	a, l
		ld	h, a
		ld	l, b
		ld	(word_B7AA), hl
		pop	bc
		pop	hl
		ret
; End of function IntRnd_A

; ---------------------------------------------------------------------------
word_B7AA:	dw 0A26Ch		; DATA XREF: IntRnd_A+2r IntRnd_A+Bw

; =============== S U B	R O U T	I N E =======================================


Process_LaserECM2:			; CODE XREF: j_MoveAllObjects+1B03j
		xor	a
		ld	(TableBasePrice+28h), a	; +#28 Fireams
		ld	(TableBasePrice+29h), a	; +#28 Fireams
		ret
; End of function Process_LaserECM2

; ---------------------------------------------------------------------------
; START	OF FUNCTION CHUNK FOR j_MoveAllObjects

Processing_LaserECM:			; CODE XREF: RAM:9CB8j
		ld	a, (PlayerGameOver)
		and	a
		jr	nz, Process_LaserECM2
		ld	a, (WhatOnScreen)
		cp	4
		ret	nc
		ld	hl, PilotFrontLaser
		ld	d, 0
		ld	e, a
		add	hl, de
		ld	a, (hl)
		and	a
		jr	z, sub_B7F4	; ;проверка 3й миссии
		ld	a, (unk_B138)	; тип печати OR/XOR
		ld	(loc_B7DB), a
		ld	(loc_B7EA), a
		ld	hl, unk_C7EE
		ld	b, 2

loc_B7D9:				; CODE XREF: RAM:B7E0j
		ld	a, 0FEh	; '■'
; END OF FUNCTION CHUNK	FOR j_MoveAllObjects

loc_B7DB:				; DATA XREF: j_MoveAllObjects+1B19w
		or	(hl)
		ld	(hl), a
		ld	l, 0F1h	; 'ё'
		ld	c, b
		djnz	loc_B7D9
		ld	de, unk_CF2F
		ld	l, d
		ld	b, 0Eh

loc_B7E8:				; CODE XREF: RAM:B7EDj
		ld	a, c
		dec	h

loc_B7EA:				; DATA XREF: j_MoveAllObjects+1B1Cw
		or	(hl)
		ld	(hl), a
		ex	de, hl
		djnz	loc_B7E8
		ld	b, 0
		call	FireLaser

; =============== S U B	R O U T	I N E =======================================

; ;проверка 3й миссии



; === MoveObjFromPlayer === CODE XREF: DirectObject+17j
MoveObjFromPlayer:			; CODE XREF: DirectObject+17j
					; DirectObject+24j ...
		push	ix
		call	HL_CrdNdiv4
		ld	(deltaCrdX), hl
		call	HL_CrdNdiv4
		ld	(deltaCrdY), hl
		call	HL_CrdNdiv4
		ld	(deltaCrdZ), hl
		pop	ix
		set	5, (ix+26h)
		jr	MoveObject1
; ---------------------------------------------------------------------------

MoveObjToPlayer:			; CODE XREF: DirectObject+3Cj
					; DirectObject+47j ...
		push	ix
		call	HL_CrdNdiv4_InvSign
		ld	(deltaCrdX), hl
		call	HL_CrdNdiv4_InvSign
		ld	(deltaCrdY), hl
		call	HL_CrdNdiv4_InvSign
		ld	(deltaCrdZ), hl
		pop	ix
		res	5, (ix+26h)

MoveObject1:				; CODE XREF: DirectObject+67j
					; MoveMissile+7Fj
		jp	MoveObject
; End of function DirectObject


; =============== S U B	R O U T	I N E =======================================


MoveMissile:				; CODE XREF: sub_9D40+DFp
		ld	a, (TimeWorkECM)
		and	a
		jr	z, loc_A4C4
		call	sub_7D3E
		ret	nz
		set	4, (ix+20h)
		ret
; ---------------------------------------------------------------------------

loc_A4C4:				; CODE XREF: MoveMissile+4j
		ld	a, c
		and	0Fh
		and	a
		jr	z, MoveObjToPlayer
		push	iy
		ld	iy, loc_6021
		ld	de, 27h	; '''
		sub	0Ah
		cpl
		ld	b, a

loc_A4D7:				; CODE XREF: MoveMissile+24j
		add	iy, de
		djnz	loc_A4D7
		push	ix
		push	iy
		call	sub_A53F
		ld	(deltaCrdX), hl
		call	sub_A53F
		ld	(deltaCrdY), hl
		call	sub_A53F
		ld	(deltaCrdZ), hl
		pop	iy
		pop	ix
		call	sub_A537
		jr	nz, loc_A532
		ld	hl, (deltaCrdY)
		call	sub_A537
		jr	nz, loc_A532
		ld	hl, (deltaCrdX)
		call	sub_F97E
		jr	nz, loc_A532
		bit	2, (iy+26h)
		jr	z, loc_A527
		ld	hl, PilotMission3 ; =#06 при стыковке будет захват таргонами (вроде нереальная ситуация)
					; =#04 в системе со танцией таргонов во	время войны
		call	sub_D6AA
		jr	nz, loc_A52B
		ld	hl, planetAlienItems
		xor	a
		ld	(hl), a
		ld	de, 0FFFFh
		call	j_IncBattleRating2
		set	7, (iy+26h)

loc_A527:				; CODE XREF: MoveMissile+59j
		set	4, (iy+20h)

loc_A52B:				; CODE XREF: MoveMissile+61j
		pop	iy
		ret
; ---------------------------------------------------------------------------

loc_A52E:				; CODE XREF: sub_9D40+1j
		ld	(hl), a
		jp	loc_9E5C
; ---------------------------------------------------------------------------

loc_A532:				; CODE XREF: MoveMissile+43j
					; MoveMissile+4Bj ...
		pop	iy
		jp	MoveObject1
; End of function MoveMissile


; =============== S U B	R O U T	I N E =======================================




; === MoveObjToPlayer === CODE XREF: DirectObject+3Cj
MoveObjToPlayer:			; CODE XREF: DirectObject+3Cj
					; DirectObject+47j ...
		push	ix
		call	HL_CrdNdiv4_InvSign
		ld	(deltaCrdX), hl
		call	HL_CrdNdiv4_InvSign
		ld	(deltaCrdY), hl
		call	HL_CrdNdiv4_InvSign
		ld	(deltaCrdZ), hl
		pop	ix
		res	5, (ix+26h)

MoveObject1:				; CODE XREF: DirectObject+67j
					; MoveMissile+7Fj
		jp	MoveObject
; End of function DirectObject


; =============== S U B	R O U T	I N E =======================================


MoveMissile:				; CODE XREF: sub_9D40+DFp
		ld	a, (TimeWorkECM)
		and	a
		jr	z, loc_A4C4
		call	sub_7D3E
		ret	nz
		set	4, (ix+20h)
		ret
; ---------------------------------------------------------------------------

loc_A4C4:				; CODE XREF: MoveMissile+4j
		ld	a, c
		and	0Fh
		and	a
		jr	z, MoveObjToPlayer
		push	iy
		ld	iy, loc_6021
		ld	de, 27h	; '''
		sub	0Ah
		cpl
		ld	b, a

loc_A4D7:				; CODE XREF: MoveMissile+24j
		add	iy, de
		djnz	loc_A4D7
		push	ix
		push	iy
		call	sub_A53F
		ld	(deltaCrdX), hl
		call	sub_A53F
		ld	(deltaCrdY), hl
		call	sub_A53F
		ld	(deltaCrdZ), hl
		pop	iy
		pop	ix
		call	sub_A537
		jr	nz, loc_A532
		ld	hl, (deltaCrdY)
		call	sub_A537
		jr	nz, loc_A532
		ld	hl, (deltaCrdX)
		call	sub_F97E
		jr	nz, loc_A532
		bit	2, (iy+26h)
		jr	z, loc_A527
		ld	hl, PilotMission3 ; =#06 при стыковке будет захват таргонами (вроде нереальная ситуация)
					; =#04 в системе со танцией таргонов во	время войны
		call	sub_D6AA
		jr	nz, loc_A52B
		ld	hl, planetAlienItems
		xor	a
		ld	(hl), a
		ld	de, 0FFFFh
		call	j_IncBattleRating2
		set	7, (iy+26h)

loc_A527:				; CODE XREF: MoveMissile+59j
		set	4, (iy+20h)

loc_A52B:				; CODE XREF: MoveMissile+61j
		pop	iy
		ret
; ---------------------------------------------------------------------------

loc_A52E:				; CODE XREF: sub_9D40+1j
		ld	(hl), a
		jp	loc_9E5C
; ---------------------------------------------------------------------------

loc_A532:				; CODE XREF: MoveMissile+43j
					; MoveMissile+4Bj ...
		pop	iy
		jp	MoveObject1
; End of function MoveMissile


; =============== S U B	R O U T	I N E =======================================




; === Mult_A_hiB_C === CODE XREF: sub_A0FD+7p
Mult_A_hiB_C:				; CODE XREF: sub_A0FD+7p
					; CalcNewCrdNobj+32p ...
		ld	a, b
		sub	c
		jr	nc, loc_AB11
		neg

loc_AB11:				; CODE XREF: Mult_A_hiB_C+2j
		srl	a
		ld	h, 0BFh	; '┐'
		ld	l, a
		ld	d, (hl)
		ld	a, b
		add	a, c
		rra
		ld	l, a
		ld	a, (hl)
		sub	d
		ret
; End of function Mult_A_hiB_C


; =============== S U B	R O U T	I N E =======================================




; === Mult_Ax2_sign === CODE XREF: CalcNewCrdObj_Pl+14p
Mult_Ax2_sign:				; CODE XREF: CalcNewCrdObj_Pl+14p
					; CalcNewCrdObj_Pl+22p
		ld	c, a
		rl	c
		rra
		rl	c
		rla
		rr	c
		ret
; End of function Mult_Ax2_sign

; ---------------------------------------------------------------------------
		db    0
		db    0

; =============== S U B	R O U T	I N E =======================================




; === Mult_HL_A2_B2 === CODE XREF: ScalarMultVectors+6p
Mult_HL_A2_B2:				; CODE XREF: ScalarMultVectors+6p
					; ScalarMultVectors+10p ...
		ld	c, a
		xor	b
		xor	80h ; 'А'
		sla	b
		sla	c
		push	af
		call	Mult_A_hiB_C
		pop	hl
		ld	l, a
		ret
; End of function Mult_HL_A2_B2


; =============== S U B	R O U T	I N E =======================================


ScalarMultVectors:			; CODE XREF: MoveObject+1Cp
					; MoveObject+26p ...
		ld	b, (ix+1)
		ld	a, (iy+1)
		call	Mult_HL_A2_B2	; Умножение hl = -1 * high((a*2) * (b*2))
					; Вх:  b - первый множитель, 7,b - знак
					;      a - второй множитель, 7,a - знак
					; Вых: l,a = high((a*2)	* (b*2))
					;	l,a = high((a*2) * (b*2))
					;	7,h - знак
		push	hl
		ld	b, (ix+3)
		ld	a, (iy+3)
		call	Mult_HL_A2_B2	; Умножение hl = -1 * high((a*2) * (b*2))
					; Вх:  b - первый множитель, 7,b - знак
					;      a - второй множитель, 7,a - знак
					; Вых: l,a = high((a*2)	* (b*2))
					;	l,a = high((a*2) * (b*2))
					;	7,h - знак
		push	hl
		ld	b, (ix+5)
		ld	a, (iy+5)
		call	Mult_HL_A2_B2	; Умножение hl = -1 * high((a*2) * (b*2))
; End of function ScalarMultVectors	; Вх:  b - первый множитель, 7,b - знак
					;      a - второй множитель, 7,a - знак
					; Вых: l,a = high((a*2)	* (b*2))
					;	l,a = high((a*2) * (b*2))
					;	7,h - знак

; =============== S U B	R O U T	I N E =======================================




; === Mult_HL_B_C === CODE XREF: Sqr_HL_wordAdrIX+Ep
Mult_HL_B_C:				; CODE XREF: Sqr_HL_wordAdrIX+Ep
					; sub_9F68+5p ...
		ld	a, b
		sub	c
		jr	nc, loc_AB3A
		neg

loc_AB3A:				; CODE XREF: Mult_HL_B_C+2j
		srl	a
		ld	h, 0BEh	; '╛'
		ld	l, a
		ld	e, (hl)
		inc	h
		ld	d, (hl)
		dec	h
		ld	a, b
		add	a, c
		rra
		ld	l, a
		ld	a, (hl)
		inc	h
		ld	h, (hl)
		ld	l, a
		and	a
		sbc	hl, de
		ret
; End of function Mult_HL_B_C


; =============== S U B	R O U T	I N E =======================================


AddCrdNobj_CrdNvertex:			; CODE XREF: MakeTablPrnCrdVertex+332p
					; MakeTablPrnCrdVertex+351p ...
		ld	c, d
		rl	e
		rl	d
		ld	a, b
		xor	c
		jr	nz, loc_AB5B
		add	hl, de
		ld	a, b
		ret
; ---------------------------------------------------------------------------

loc_AB5B:				; CODE XREF: AddCrdNobj_CrdNvertex+7j
		sbc	hl, de
		jr	nc, loc_AB66
		add	hl, de
		ex	de, hl
		and	a
		sbc	hl, de
		ld	a, c
		ret
; ---------------------------------------------------------------------------

loc_AB66:				; CODE XREF: AddCrdNobj_CrdNvertex+Ej
		ld	a, h
		or	l
		ld	a, c
		ret	z
		pop	de
		pop	af
		neg
		push	af
		push	de
		dec	hl
		ld	a, b
		ret
; End of function AddCrdNobj_CrdNvertex

; ---------------------------------------------------------------------------
; START	OF FUNCTION CHUNK FOR MakeTablPrnCrdVertex

RotateCrdVertex:			; CODE XREF: MakeTablPrnCrdVertex+2E0j
					; MakeTablPrnCrdVertex+2F8j
		ld	a, (VisiblePrevVertex)
		and	a
		jr	nz, loc_AB90
		ld	a, (CrdNVertex)	; координата N текущей вершины
		rrca
		jr	c, loc_AB90
		call	sub_ABC0
		ld	(VertexCrdX), hl
		call	sub_ABC0
		ld	(VertexCrdY), hl
		call	sub_ABC0
		jr	loc_AB9F
; ---------------------------------------------------------------------------

loc_AB90:				; CODE XREF: MakeTablPrnCrdVertex+1AEj
					; MakeTablPrnCrdVertex+1B4j
		call	RotateCrdN
		ld	(VertexCrdX), hl
		call	RotateCrdN
		ld	(VertexCrdY), hl
		call	RotateCrdN

loc_AB9F:				; CODE XREF: MakeTablPrnCrdVertex+1C5j
		ld	(VertexCrdZ), hl
		xor	a
		ld	(VertexCrdXlow), a
		ld	(VertexCrdYlow), a
		ld	(VertexCrdZlow), a
		jp	loc_ACDF
; END OF FUNCTION CHUNK	FOR MakeTablPrnCrdVertex

; =============== S U B	R O U T	I N E =======================================




; === Mult_HL_D_A === CODE XREF: CalcDistanceToCursor2+8p
Mult_HL_D_A:				; CODE XREF: CalcDistanceToCursor2+8p
					; CalcDistanceToCursor2+16p ...
		ld	e, 0
		ld	b, 8
		ld	hl, 0

loc_E010:				; CODE XREF: Mult_HL_D_A:loc_E019j
		srl	d
		rr	e
		rl	a
		jr	nc, loc_E019
		add	hl, de

loc_E019:				; CODE XREF: Mult_HL_D_A+Dj
		djnz	loc_E010
		ret
; End of function Mult_HL_D_A


; =============== S U B	R O U T	I N E =======================================


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




; === Mult_HL_EC_A === CODE XREF: ScalarMultVectors16_8+9p
Mult_HL_EC_A:				; CODE XREF: ScalarMultVectors16_8+9p
					; ScalarMultVectors16_8+16p ...
		ld	b, a
		sla	b
		and	80h ; 'А'
		xor	e
		ld	e, a
		call	Mult_A_hiB_C
		ld	d, e
		ld	e, a
		ld	c, d
		res	7, c
		push	de
		call	Mult_HL_B_C
		pop	de
		ld	a, d
		ld	d, 0
		add	hl, de
		add	hl, hl
		rla
		rr	h
		rr	l
		ret
; End of function Mult_HL_EC_A


; =============== S U B	R O U T	I N E =======================================


RotateCrdN_Normal:			; CODE XREF: CalcVisibleVerges+47p
					; CalcVisibleVerges+50p ...
		call	RotateCrdN
		pop	bc
		ex	(sp), ix
		ld	e, (ix+0)
		ld	d, (ix+1)
		ld	a, (ix+2)
		inc	ix
		inc	ix
		inc	ix
		ex	(sp), ix
		push	bc
		ld	c, a
		ld	a, (IY_dFactor)
		sla	l
		push	hl
		ld	b, a
		inc	b

loc_BDD5:				; CODE XREF: RotateCrdN_Normal+23j
		srl	l
		djnz	loc_BDD5
		rl	l
		ld	a, c
		xor	h
		sla	h
		rla
		jr	c, loc_BDE5
		add	hl, de
		jr	loc_BDF4
; ---------------------------------------------------------------------------

loc_BDE5:				; CODE XREF: RotateCrdN_Normal+2Cj
		ex	de, hl
		and	a
		sbc	hl, de
		jr	nc, loc_BDF4
		add	hl, de
		ex	de, hl
		and	a
		sbc	hl, de
		ld	a, c
		xor	80h ; 'А'
		ld	c, a

loc_BDF4:				; CODE XREF: RotateCrdN_Normal+2Fj
					; RotateCrdN_Normal+35j
		sla	c
		rr	h
		rr	l
		pop	de
		ld	a, e
		rl	d
		rra
		ret
; End of function RotateCrdN_Normal

; ---------------------------------------------------------------------------


; === Pause_24Int === CODE XREF: RAM:D11Dp	RAM:E835p ...
Pause_24Int:				; CODE XREF: RAM:D11Dp	RAM:E835p ...
		ld	b, 24h ; '$'

loc_E360:				; CODE XREF: sub_E35A+2j
					; Pause_24Int+3j
		halt
		djnz	loc_E360
		ret
; End of function Pause_24Int


; =============== S U B	R O U T	I N E =======================================




; === PrnCocpit_Ifn === CODE XREF: GoDocking+3Cp sub_728D+Bp ...
PrnCocpit_Ifn:				; CODE XREF: GoDocking+3Cp sub_728D+Bp ...

; FUNCTION CHUNK AT 8459 SIZE 00000006 BYTES

		ld	de, 4020h
		ld	a, (de)
		inc	a
		ret	nz
		call	loc_94FE
		call	j_ClsAndPrintRamk
		ld	de, 1Fh
		ld	b, 17h
		ld	a, 0
		ld	hl, 5800h
		ld	(hl), a

loc_8059:				; CODE XREF: PrnCocpit_Ifn+1Bj
		add	hl, de
		ld	(hl), a
		inc	hl
		ld	(hl), a
		djnz	loc_8059
		add	hl, de
		ld	(hl), a
		ld	de, 5000h
		ld	hl, DataCocpit
		ld	bc, 800h
		ldir
		ld	de, 5A00h
		ld	hl, 5B00h
		ld	bc, 100h
		ldir
		call	sub_725C
		call	sub_7265
		jp	loc_8459
; End of function PrnCocpit_Ifn


; =============== S U B	R O U T	I N E =======================================


ClearScrBuffer:				; CODE XREF: sub_728D:loc_73C3p
					; sub_728D+25Ap ...

; FUNCTION CHUNK AT DA5F SIZE 00000005 BYTES

		di
		ld	hl, 0CFE0h
		ld	de, 0C020h
		ld	bc, 5720h

loc_808A:				; CODE XREF: ClearScrBuffer+10j
		dec	l
		dec	e
		dec	c
		ld	a, (bc)
		ld	(hl), a
		ld	(de), a
		jr	nz, loc_808A
		ld	(loc_DA5F+1), sp
		ld	sp, hl
		ex	de, hl
		add	hl, hl
		ld	e, l
		ld	d, l
		ld	b, 7Dh ; '}'

loc_809D:				; CODE XREF: ClearScrBuffer+2Fj
		inc	e
		push	de
		dec	e
		push	de
		push	de
		push	de
		push	de
		push	de
		push	de
		push	de
		push	de
		push	de
		push	de
		push	de
		push	de
		push	de
		push	de
		push	hl
		djnz	loc_809D
		jp	loc_DA5F
; End of function ClearScrBuffer

; ---------------------------------------------------------------------------

loc_80B4:				; CODE XREF: sub_EB2F+4p
		cp	0Eh
		jr	nc, loc_80BC
		ld	b, a
		add	a, a
		add	a, b
		ret
; ---------------------------------------------------------------------------

loc_80BC:				; CODE XREF: RAM:80B6j
		ld	hl, 667h
		xor	a
		ret
; ---------------------------------------------------------------------------

SetMaxTonnage:				; CODE XREF: sub_5EC5+1Bp
					; UseEscapePod+3Fp ...
		ld	a, (PilotLargeCargo) ; установка максимальной грузоподъемности
		ld	b, a
		dec	b
		ld	a, 5

loc_80C8:				; CODE XREF: RAM:80CBj
		add	a, 0Fh
		inc	b
		jr	nz, loc_80C8
		ld	(PilotFreeCargo), a
		ret
; ---------------------------------------------------------------------------
word_80D1:	dw 411Ah		; DATA XREF: sub_813Ew	sub_813E+18r

; =============== S U B	R O U T	I N E =======================================

; проверка нажатия (и соответствующий переход) клавиш 1, 2, 3, 4, I, O,	P, K, L, enter



; === PrnNameViewScrInBuf === CODE XREF: PrnTxtOnViewScr+9j
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



; === Process_MissBombEPod === CODE XREF: RAM:750Cp
Process_MissBombEPod:			; CODE XREF: RAM:750Cp
		ld	a, (WhatOnScreen)
		cp	4
		ret	nc
		call	PrnDangerEcmDock
		call	UseMissile
		call	UseEscapePod
		call	j_UseEnergyBomb
		ld	hl, ControlKey3
		ld	a, (hl)
		and	10010001b
		ld	(hl), a
; End of function Process_MissBombEPod


; =============== S U B	R O U T	I N E =======================================


Damage2Pilot:				; CODE XREF: sub_74C6+9p
		ld	hl, IndDanger
		xor	a
		cp	(hl)
		ret	z
		ld	a, (ShieldUnit4)
		cp	0A0h ; 'а'
		jr	z, loc_9A6B
		ld	a, 63h ; 'c'
		ld	(IndDanger), a

loc_9A6B:				; CODE XREF: Damage2Pilot+Bj
		call	EnergyRecovery
		ld	hl, ShieldFront
		ld	a, (DamageFrontShield)
		call	sub_9AA7
		ld	hl, ShieldRear
		ld	a, (DamageRearShield)
		call	sub_9AA7
		ld	a, (CloakingDeviceOn)
		and	a
		ld	a, (DamageEnergyUnit)
		jr	z, loc_9A8A
		inc	a

loc_9A8A:				; CODE XREF: Damage2Pilot+2Ej
		ld	c, a
		ld	b, 4
; End of function Damage2Pilot


; =============== S U B	R O U T	I N E =======================================




; === ResetPressedKeys === CODE XREF: RAM:D120j	sub_E1F7+3p ...
ResetPressedKeys:			; CODE XREF: RAM:D120j	sub_E1F7+3p ...
		di
		ld	hl, ControlKey1
		ld	b, 4

loc_E20B:				; CODE XREF: ResetPressedKeys+9j
		ld	(hl), 0
		inc	hl
		djnz	loc_E20B
		ei
		ret
; End of function ResetPressedKeys


; =============== S U B	R O U T	I N E =======================================




; === ResetSignInExtSlot === CODE XREF: PrnObjIfVisiblep
ResetSignInExtSlot:			; CODE XREF: PrnObjIfVisiblep
		ld	hl, (word_B452)
		inc	hl
		ld	(hl), 0FFh
		inc	hl
		inc	hl
		inc	hl
		ld	(hl), 0FFh
		ld	a, (WhatOnScreen)
		ret
; End of function ResetSignInExtSlot


; =============== S U B	R O U T	I N E =======================================


CalcPrnCrdSun:				; CODE XREF: PrnSunPlanet+Fp
					; PrnSunPlanet+34p
		call	loc_75E1
		and	a
		ret	z
		push	ix
		ld	ix, CircleCrdZlow
		ld	a, (WhatOnScreen)
		cp	2
		jr	c, loc_D7CE
		ld	xl, 7Ah	; 'z'

loc_D7CE:				; CODE XREF: CalcPrnCrdSun+10j
		push	iy
		ld	iy,  loc_7575+2
		call	sub_FBE0
		call	nz, sub_FBE0
		call	nz, sub_FBE0
		pop	iy
		pop	ix
		ret
; End of function CalcPrnCrdSun

; ---------------------------------------------------------------------------
; START	OF FUNCTION CHUNK FOR PrintMessage1_A

loc_D7E2:				; CODE XREF: PrintMessage1_A+2j
					; sub_D470+Bj
		ld	hl, 2EF2h
		nop
		dec	l
		ld	h, 0D7h	; '╫'
		ld	d, 0
		ld	e, a
		add	hl, de
		ld	l, (hl)
		ld	h, 0D8h	; '╪'
		jp	(hl)
; END OF FUNCTION CHUNK	FOR PrintMessage1_A
; ---------------------------------------------------------------------------
		db 84h
		db 4Bh
		db  6Ah	; j
		db  72h	; r
		db 2Eh
		db  37h	; 7
		db 0AEh	; о
		db  22h	; "
		db  33h	; 3
		db 0B4h	; ┤
		db 0E5h	; х
		db  57h	; W
		db  84h	; Д
		db  84h	; Д
		db  84h	; Д
		db 0AEh	; о
		db  33h	; 3
		db  72h	; r
		db  2Eh	; .
		db 0E5h	; х
		db 0E9h	; щ
		db 0B7h	; ╖
		db  7Bh	; {
		db 0BAh	; ║
		db 0BDh	; ╜
		db 0C0h	; └
		db 0B4h	; ┤
		db 0A9h	; й
		db  84h	; Д
		db  84h	; Д
		db  57h	; W
		db  84h	; Д
		db  85h	; Е
		db  7Fh	; 
		db  84h	; Д
		db 0C3h	; ├
		db 0C6h	; ╞
		db 0F3h	; є
		db  1Fh
		db  1Fh
		db 0C9h	; ╔
		db  84h	; Д
		db  84h	; Д
		db 0EFh	; я
		db  84h	; Д
		db  84h	; Д
		db 0C3h	; ├
		db  5Eh	; ^
		db 0E3h	; у

; =============== S U B	R O U T	I N E =======================================




; === RndDirectionObj === CODE XREF: CallFunctionObj_1+12Dp
RndDirectionObj:			; CODE XREF: CallFunctionObj_1+12Dp
					; SetPlayerAlloys+6p ...
		call	IntRnd_A
		and	87h ; 'З'
		ld	(ix+0Ah), a
		call	IntRnd_A
		and	87h ; 'З'
		ld	(ix+0Ch), a
		ret
; End of function RndDirectionObj

; ---------------------------------------------------------------------------
; START	OF FUNCTION CHUNK FOR CallFunctionObj_1

Set2Alloys:				; CODE XREF: CallFunctionObj_1+1Aj
		bit	0, c
		jr	z, ShipSet2Alloys
		call	SetPlayerAlloys
		ret	nz
; END OF FUNCTION CHUNK	FOR CallFunctionObj_1

; =============== S U B	R O U T	I N E =======================================


SetPlayerAlloys:			; CODE XREF: CallFunctionObj_1+22Ep
		ld	a, 3

SetPlayerCargo:				; CODE XREF: CallFunctionObj_1+28Ej
		call	SetPlayerObj
		ret	nz
		call	RndDirectionObj
		res	6, (ix+20h)
		call	IntRnd_A
		and	3Fh ; '?'
		or	10h
		ld	(ix+0),	a
		ld	a, (ix+0Ah)
		and	80h ; 'А'
		xor	80h ; 'А'
		ld	(ix+2),	a
		call	IntRnd_A
		and	3Fh ; '?'
		or	10h
		ld	(ix+3),	a
		ld	a, (ix+0Ch)
		and	80h ; 'А'
		xor	80h ; 'А'
		ld	(ix+5),	a
		ld	a, 19h
		ld	(ix+6),	a
		call	IntRnd_A
		ld	(ix+1Dh), a
		call	IntRnd_A
		and	1Fh
		or	8
		ld	(ix+1Bh), a
		xor	a
		ret
; End of function SetPlayerAlloys

; ---------------------------------------------------------------------------
; START	OF FUNCTION CHUNK FOR CallFunctionObj_1

ShipSet2Alloys:				; CODE XREF: CallFunctionObj_1+22Cj
		ld	a, 3
		call	SetShipMinAlloys
; ---------------------------------------------------------------------------
		ret	nz
		ld	a, 3
		jr	SetShipMinAlloys
; ---------------------------------------------------------------------------

SetCargo:				; CODE XREF: CallFunctionObj_1+1Ej
		bit	0, c
		ld	a, 4
		jr	z, SetShipEPodCargo
		jr	SetPlayerCargo
; ---------------------------------------------------------------------------

SetCobra_EscPod:			; CODE XREF: CallFunctionObj_1+22j
		bit	0, c
		ld	a, 2
		jr	z, SetShipEPodCargo
		ld	a, 9
		ld	ix, SlotObj_05
		call	SetPlayerObjSlot
		call	RndDirectionObj
		ld	a, 80h ; 'А'
		ld	(ix+5),	a
		ld	a, 1Ch
		ld	(ix+3),	a
		res	6, (ix+20h)
		ld	a, 4Ch ; 'L'
		ld	(ix+6),	a
		ld	a, 0C0h	; '└'
		ld	(ix+1Eh), a
		ld	a, (HighDeltaSpeed)
		add	a, (ix+1Bh)
		ld	(ix+1Bh), a
		xor	a
		ret
; ---------------------------------------------------------------------------

SetShipEPodCargo:			; CODE XREF: CallFunctionObj_1+28Cj
					; CallFunctionObj_1+294j
		push	ix
		call	ShipSetObject
; ---------------------------------------------------------------------------
		ex	(sp), ix
		pop	hl
		ret	nz
		push	hl
		ex	(sp), ix
		res	6, (ix+20h)
		ld	a, 0Ah
		ld	(ix+26h), a
		ld	a, 80h ; 'А'
		ld	(ix+1Dh), a
		xor	a
		pop	ix
		ret
; ---------------------------------------------------------------------------

Set2Minerals:				; CODE XREF: CallFunctionObj_1+26j
		ld	a, 5
		call	SetShipMinAlloys
; ---------------------------------------------------------------------------
		ret	nz
		ld	a, 5
; END OF FUNCTION CHUNK	FOR CallFunctionObj_1

; =============== S U B	R O U T	I N E =======================================

; Attributes: noreturn

SetShipMinAlloys:			; CODE XREF: CallFunctionObj_1+280p
					; CallFunctionObj_1+286j ...
		push	ix
		call	ShipSetObject
; ---------------------------------------------------------------------------
		ex	(sp), ix
		pop	hl
		ret	nz
		push	hl
		ex	(sp), ix
		call	RotateSKOobj
		ld	a, 0Ah
		ld	(ix+26h), a
		res	6, (ix+20h)
		set	7, (ix+1Dh)
		pop	ix
		xor	a
		ret
; End of function SetShipMinAlloys


; =============== S U B	R O U T	I N E =======================================


SetShip4Intro:				; CODE XREF: sub_728D+132p
					; CallFunctionObj_1+2Aj
		ld	ix, objMissile
		ld	a, c
		add	a, 2
		call	SetObjInSlot_Intro
		ld	a, 0C0h	; '└'
		ld	(ix+1Dh), a
		ld	(ix+1Eh), a
		xor	a
		ld	(ix+6),	a
		ld	(ix+0),	a
		ld	(ix+1),	a
		ld	(ix+3),	a
		ld	(ix+4),	a
		ld	(ix+8),	a
		ld	(ix+1Bh), a
		res	6, (ix+20h)
		inc	a
		inc	a
		ld	(ix+7),	a
		push	ix
		pop	hl
		ld	de, 9
		ld	b, d
		ld	c, e
		add	hl, de
		ex	de, hl
		ld	hl, SKO4Intro

loc_BBB3:				; CODE XREF: SetShip4Intro+41j
		inc	de
		ldi
		jp	pe, loc_BBB3
		ret
; End of function SetShip4Intro

; ---------------------------------------------------------------------------
; START	OF FUNCTION CHUNK FOR CallFunctionObj_1

SetRndShip:				; CODE XREF: CallFunctionObj_1+8j
		ld	a, (cargoSlaves)
		ld	c, a
		ld	a, (cargoNarc)
		add	a, c
		add	a, a
		ld	c, a
		ld	a, (cargoFireams)
		add	a, c
		ld	c, a
		ld	a, (Mission2)
		and	a
		jr	nz, loc_BBFD
		call	IntRnd_A
		cp	1Eh
		jr	nc, loc_BBFD
		ld	a, (PilotExtWeapon) ; возможность выдать ASP для 2й миссии
		and	42h ; 'B'
		cp	2
		jr	nz, loc_BBFD
		dec	a		; ставим ASP для 2й миссии
		ld	(Mission2), a
		ld	a, 0Fh
		ld	hl, sub_BBF2
		push	hl
		push	af
		call	FindSlot4Obj
		ret	nz
		ld	a, 0Dh
		jr	loc_BC65
; END OF FUNCTION CHUNK	FOR CallFunctionObj_1

; =============== S U B	R O U T	I N E =======================================

; Attributes: noreturn



; === Set2Alloys === CODE XREF: CallFunctionObj_1+1Aj
Set2Alloys:				; CODE XREF: CallFunctionObj_1+1Aj
		bit	0, c
		jr	z, ShipSet2Alloys
		call	SetPlayerAlloys
		ret	nz
; END OF FUNCTION CHUNK	FOR CallFunctionObj_1

; =============== S U B	R O U T	I N E =======================================


SetPlayerAlloys:			; CODE XREF: CallFunctionObj_1+22Ep
		ld	a, 3

SetPlayerCargo:				; CODE XREF: CallFunctionObj_1+28Ej
		call	SetPlayerObj
		ret	nz
		call	RndDirectionObj
		res	6, (ix+20h)
		call	IntRnd_A
		and	3Fh ; '?'
		or	10h
		ld	(ix+0),	a
		ld	a, (ix+0Ah)
		and	80h ; 'А'
		xor	80h ; 'А'
		ld	(ix+2),	a
		call	IntRnd_A
		and	3Fh ; '?'
		or	10h
		ld	(ix+3),	a
		ld	a, (ix+0Ch)
		and	80h ; 'А'
		xor	80h ; 'А'
		ld	(ix+5),	a
		ld	a, 19h
		ld	(ix+6),	a
		call	IntRnd_A
		ld	(ix+1Dh), a
		call	IntRnd_A
		and	1Fh
		or	8
		ld	(ix+1Bh), a
		xor	a
		ret
; End of function SetPlayerAlloys

; ---------------------------------------------------------------------------
; START	OF FUNCTION CHUNK FOR CallFunctionObj_1

ShipSet2Alloys:				; CODE XREF: CallFunctionObj_1+22Cj
		ld	a, 3
		call	SetShipMinAlloys
; ---------------------------------------------------------------------------
		ret	nz
		ld	a, 3
		jr	SetShipMinAlloys
; ---------------------------------------------------------------------------

SetCargo:				; CODE XREF: CallFunctionObj_1+1Ej
		bit	0, c
		ld	a, 4
		jr	z, SetShipEPodCargo
		jr	SetPlayerCargo
; ---------------------------------------------------------------------------

SetCobra_EscPod:			; CODE XREF: CallFunctionObj_1+22j
		bit	0, c
		ld	a, 2
		jr	z, SetShipEPodCargo
		ld	a, 9
		ld	ix, SlotObj_05
		call	SetPlayerObjSlot
		call	RndDirectionObj
		ld	a, 80h ; 'А'
		ld	(ix+5),	a
		ld	a, 1Ch
		ld	(ix+3),	a
		res	6, (ix+20h)
		ld	a, 4Ch ; 'L'
		ld	(ix+6),	a
		ld	a, 0C0h	; '└'
		ld	(ix+1Eh), a
		ld	a, (HighDeltaSpeed)
		add	a, (ix+1Bh)
		ld	(ix+1Bh), a
		xor	a
		ret
; ---------------------------------------------------------------------------

SetShipEPodCargo:			; CODE XREF: CallFunctionObj_1+28Cj
					; CallFunctionObj_1+294j
		push	ix
		call	ShipSetObject
; ---------------------------------------------------------------------------
		ex	(sp), ix
		pop	hl
		ret	nz
		push	hl
		ex	(sp), ix
		res	6, (ix+20h)
		ld	a, 0Ah
		ld	(ix+26h), a
		ld	a, 80h ; 'А'
		ld	(ix+1Dh), a
		xor	a
		pop	ix
		ret
; ---------------------------------------------------------------------------

Set2Minerals:				; CODE XREF: CallFunctionObj_1+26j
		ld	a, 5
		call	SetShipMinAlloys
; ---------------------------------------------------------------------------
		ret	nz
		ld	a, 5
; END OF FUNCTION CHUNK	FOR CallFunctionObj_1

; =============== S U B	R O U T	I N E =======================================

; Attributes: noreturn

SetShipMinAlloys:			; CODE XREF: CallFunctionObj_1+280p
					; CallFunctionObj_1+286j ...
		push	ix
		call	ShipSetObject
; ---------------------------------------------------------------------------
		ex	(sp), ix
		pop	hl
		ret	nz
		push	hl
		ex	(sp), ix
		call	RotateSKOobj
		ld	a, 0Ah
		ld	(ix+26h), a
		res	6, (ix+20h)
		set	7, (ix+1Dh)
		pop	ix
		xor	a
		ret
; End of function SetShipMinAlloys


; =============== S U B	R O U T	I N E =======================================


SetShip4Intro:				; CODE XREF: sub_728D+132p
					; CallFunctionObj_1+2Aj
		ld	ix, objMissile
		ld	a, c
		add	a, 2
		call	SetObjInSlot_Intro
		ld	a, 0C0h	; '└'
		ld	(ix+1Dh), a
		ld	(ix+1Eh), a
		xor	a
		ld	(ix+6),	a
		ld	(ix+0),	a
		ld	(ix+1),	a
		ld	(ix+3),	a
		ld	(ix+4),	a
		ld	(ix+8),	a
		ld	(ix+1Bh), a
		res	6, (ix+20h)
		inc	a
		inc	a
		ld	(ix+7),	a
		push	ix
		pop	hl
		ld	de, 9
		ld	b, d
		ld	c, e
		add	hl, de
		ex	de, hl
		ld	hl, SKO4Intro

loc_BBB3:				; CODE XREF: SetShip4Intro+41j
		inc	de
		ldi
		jp	pe, loc_BBB3
		ret
; End of function SetShip4Intro

; ---------------------------------------------------------------------------
; START	OF FUNCTION CHUNK FOR CallFunctionObj_1

SetRndShip:				; CODE XREF: CallFunctionObj_1+8j
		ld	a, (cargoSlaves)
		ld	c, a
		ld	a, (cargoNarc)
		add	a, c
		add	a, a
		ld	c, a
		ld	a, (cargoFireams)
		add	a, c
		ld	c, a
		ld	a, (Mission2)
		and	a
		jr	nz, loc_BBFD
		call	IntRnd_A
		cp	1Eh
		jr	nc, loc_BBFD
		ld	a, (PilotExtWeapon) ; возможность выдать ASP для 2й миссии
		and	42h ; 'B'
		cp	2
		jr	nz, loc_BBFD
		dec	a		; ставим ASP для 2й миссии
		ld	(Mission2), a
		ld	a, 0Fh
		ld	hl, sub_BBF2
		push	hl
		push	af
		call	FindSlot4Obj
		ret	nz
		ld	a, 0Dh
		jr	loc_BC65
; END OF FUNCTION CHUNK	FOR CallFunctionObj_1

; =============== S U B	R O U T	I N E =======================================

; Attributes: noreturn



; === Set2Minerals === CODE XREF: CallFunctionObj_1+26j
Set2Minerals:				; CODE XREF: CallFunctionObj_1+26j
		ld	a, 5
		call	SetShipMinAlloys
; ---------------------------------------------------------------------------
		ret	nz
		ld	a, 5
; END OF FUNCTION CHUNK	FOR CallFunctionObj_1

; =============== S U B	R O U T	I N E =======================================

; Attributes: noreturn

SetShipMinAlloys:			; CODE XREF: CallFunctionObj_1+280p
					; CallFunctionObj_1+286j ...
		push	ix
		call	ShipSetObject
; ---------------------------------------------------------------------------
		ex	(sp), ix
		pop	hl
		ret	nz
		push	hl
		ex	(sp), ix
		call	RotateSKOobj
		ld	a, 0Ah
		ld	(ix+26h), a
		res	6, (ix+20h)
		set	7, (ix+1Dh)
		pop	ix
		xor	a
		ret
; End of function SetShipMinAlloys


; =============== S U B	R O U T	I N E =======================================


SetShip4Intro:				; CODE XREF: sub_728D+132p
					; CallFunctionObj_1+2Aj
		ld	ix, objMissile
		ld	a, c
		add	a, 2
		call	SetObjInSlot_Intro
		ld	a, 0C0h	; '└'
		ld	(ix+1Dh), a
		ld	(ix+1Eh), a
		xor	a
		ld	(ix+6),	a
		ld	(ix+0),	a
		ld	(ix+1),	a
		ld	(ix+3),	a
		ld	(ix+4),	a
		ld	(ix+8),	a
		ld	(ix+1Bh), a
		res	6, (ix+20h)
		inc	a
		inc	a
		ld	(ix+7),	a
		push	ix
		pop	hl
		ld	de, 9
		ld	b, d
		ld	c, e
		add	hl, de
		ex	de, hl
		ld	hl, SKO4Intro

loc_BBB3:				; CODE XREF: SetShip4Intro+41j
		inc	de
		ldi
		jp	pe, loc_BBB3
		ret
; End of function SetShip4Intro

; ---------------------------------------------------------------------------
; START	OF FUNCTION CHUNK FOR CallFunctionObj_1

SetRndShip:				; CODE XREF: CallFunctionObj_1+8j
		ld	a, (cargoSlaves)
		ld	c, a
		ld	a, (cargoNarc)
		add	a, c
		add	a, a
		ld	c, a
		ld	a, (cargoFireams)
		add	a, c
		ld	c, a
		ld	a, (Mission2)
		and	a
		jr	nz, loc_BBFD
		call	IntRnd_A
		cp	1Eh
		jr	nc, loc_BBFD
		ld	a, (PilotExtWeapon) ; возможность выдать ASP для 2й миссии
		and	42h ; 'B'
		cp	2
		jr	nz, loc_BBFD
		dec	a		; ставим ASP для 2й миссии
		ld	(Mission2), a
		ld	a, 0Fh
		ld	hl, sub_BBF2
		push	hl
		push	af
		call	FindSlot4Obj
		ret	nz
		ld	a, 0Dh
		jr	loc_BC65
; END OF FUNCTION CHUNK	FOR CallFunctionObj_1

; =============== S U B	R O U T	I N E =======================================

; Attributes: noreturn



; === SetAtrViewScr === CODE XREF: j_SetAtrViewScrj
SetAtrViewScr:				; CODE XREF: j_SetAtrViewScrj
		ld	a, (CloakingDeviceOn)
		and	a
		jr	nz, FastSetAtrInViewScr
		ld	a, l
		and	111b
		ld	l, a
; End of function sub_70A9


; =============== S U B	R O U T	I N E =======================================


FastSetAtrInViewScr:			; CODE XREF: sub_70A9+2p sub_70A9+Bj
		ld	a, (PilotMission1)
		and	a
		jr	z, loc_70C4
		ld	a, l
		and	111010b
		ld	l, a

loc_70C4:				; CODE XREF: FastSetAtrInViewScr+4j
		halt
		ld	a, l
		ld	hl, 59FEh
		ld	b, 0Fh

loc_70CB:				; CODE XREF: FastSetAtrInViewScr+1Fj
		ld	(hl), a
		ld	d, h
		ld	e, l
		dec	de
		push	bc
		ld	bc, 1Dh
		lddr
		dec	hl
		dec	hl
		dec	hl
		pop	bc
		djnz	loc_70CB
		ret
; End of function FastSetAtrInViewScr

; ---------------------------------------------------------------------------
byte_70DC:	db 5Eh			; DATA XREF: ProgressMissionWaro
		db 43h
		db 0FCh
		db 0FFh
		db 0C3h
		db 66h
		db 73h


; === SetAttrFrom_DE === CODE XREF: RAM:E719p	sub_E87E+61p ...
SetAttrFrom_DE:				; CODE XREF: RAM:E719p	sub_E87E+61p ...
		ld	hl, 5841h

loc_D56B:				; CODE XREF: SetAttrFrom_DE+22j
		ld	a, (de)
		inc	de
		push	af
		and	0F8h ; '°'
		rrca
		rrca
		rrca
		ld	b, a
		push	hl
		ld	a, 5
		call	Fill_HL_A_Len_B
		pop	hl
		pop	af
		and	7
		jr	z, loc_D58C
		ld	b, a

loc_D581:				; CODE XREF: SetAttrFrom_DE:loc_D588j
		ld	a, 20h ; ' '
		add	a, l
		ld	l, a
		jr	nc, loc_D588
		inc	h

loc_D588:				; CODE XREF: SetAttrFrom_DE+1Dj
		djnz	loc_D581
		jr	loc_D56B
; ---------------------------------------------------------------------------

loc_D58C:				; CODE XREF: SetAttrFrom_DE+16j
		ld	hl, 5822h
		ld	a, 7
		ld	(hl), a
		inc	hl
		ld	(hl), a
		ret
; End of function SetAttrFrom_DE

; ---------------------------------------------------------------------------
; START	OF FUNCTION CHUNK FOR sub_D470

loc_D595:				; CODE XREF: sub_D470+1Bj
		sub	0D7h ; '╫'
		ld	hl, unk_FD99
		jr	loc_D5A1
; END OF FUNCTION CHUNK	FOR sub_D470
; ---------------------------------------------------------------------------
; START	OF FUNCTION CHUNK FOR PrintMessage1_A

loc_D59C:				; CODE XREF: PrintMessage1_A+15j
		sub	80h ; 'А'
		ld	hl, byte_FDB3

loc_D5A1:				; CODE XREF: sub_D470+12Aj
		add	a, a
		ld	d, 0
		ld	e, a
		add	hl, de
		ld	a, (hl)
		push	hl
		call	sub_D499
		pop	hl
		inc	hl
		ld	a, (hl)
		cp	3Fh ; '?'
		ret	z
		jp	sub_D499
; END OF FUNCTION CHUNK	FOR PrintMessage1_A

; =============== S U B	R O U T	I N E =======================================

; гибель минеральной жизни при недостаточном количестве
; Вых: z=1 есть	минеральная жизнь в трюме
;	=0 минеральная жизнь померла
;	   нет минеральной жизни



; === SetBit2ExtSlot_rndA === CODE XREF: objPrnObj:objPrnLaserp
SetBit2ExtSlot_rndA:			; CODE XREF: objPrnObj:objPrnLaserp
		ld	hl, (word_B452)
		ld	a, (hl)
		push	hl
		ld	hl, PlayerAggressor
		call	bit2A_equ_bit2HL
		pop	hl
		ld	(hl), a
		inc	hl
		ld	(hl), 0FFh
		jp	IntRnd_A
; End of function SetBit2ExtSlot_rndA


; =============== S U B	R O U T	I N E =======================================




; === SetCobra_EscPod === CODE XREF: CallFunctionObj_1+22j
SetCobra_EscPod:			; CODE XREF: CallFunctionObj_1+22j
		bit	0, c
		ld	a, 2
		jr	z, SetShipEPodCargo
		ld	a, 9
		ld	ix, SlotObj_05
		call	SetPlayerObjSlot
		call	RndDirectionObj
		ld	a, 80h ; 'А'
		ld	(ix+5),	a
		ld	a, 1Ch
		ld	(ix+3),	a
		res	6, (ix+20h)
		ld	a, 4Ch ; 'L'
		ld	(ix+6),	a
		ld	a, 0C0h	; '└'
		ld	(ix+1Eh), a
		ld	a, (HighDeltaSpeed)
		add	a, (ix+1Bh)
		ld	(ix+1Bh), a
		xor	a
		ret
; ---------------------------------------------------------------------------

SetShipEPodCargo:			; CODE XREF: CallFunctionObj_1+28Cj
					; CallFunctionObj_1+294j
		push	ix
		call	ShipSetObject
; ---------------------------------------------------------------------------
		ex	(sp), ix
		pop	hl
		ret	nz
		push	hl
		ex	(sp), ix
		res	6, (ix+20h)
		ld	a, 0Ah
		ld	(ix+26h), a
		ld	a, 80h ; 'А'
		ld	(ix+1Dh), a
		xor	a
		pop	ix
		ret
; ---------------------------------------------------------------------------

Set2Minerals:				; CODE XREF: CallFunctionObj_1+26j
		ld	a, 5
		call	SetShipMinAlloys
; ---------------------------------------------------------------------------
		ret	nz
		ld	a, 5
; END OF FUNCTION CHUNK	FOR CallFunctionObj_1

; =============== S U B	R O U T	I N E =======================================

; Attributes: noreturn

SetShipMinAlloys:			; CODE XREF: CallFunctionObj_1+280p
					; CallFunctionObj_1+286j ...
		push	ix
		call	ShipSetObject
; ---------------------------------------------------------------------------
		ex	(sp), ix
		pop	hl
		ret	nz
		push	hl
		ex	(sp), ix
		call	RotateSKOobj
		ld	a, 0Ah
		ld	(ix+26h), a
		res	6, (ix+20h)
		set	7, (ix+1Dh)
		pop	ix
		xor	a
		ret
; End of function SetShipMinAlloys


; =============== S U B	R O U T	I N E =======================================


SetShip4Intro:				; CODE XREF: sub_728D+132p
					; CallFunctionObj_1+2Aj
		ld	ix, objMissile
		ld	a, c
		add	a, 2
		call	SetObjInSlot_Intro
		ld	a, 0C0h	; '└'
		ld	(ix+1Dh), a
		ld	(ix+1Eh), a
		xor	a
		ld	(ix+6),	a
		ld	(ix+0),	a
		ld	(ix+1),	a
		ld	(ix+3),	a
		ld	(ix+4),	a
		ld	(ix+8),	a
		ld	(ix+1Bh), a
		res	6, (ix+20h)
		inc	a
		inc	a
		ld	(ix+7),	a
		push	ix
		pop	hl
		ld	de, 9
		ld	b, d
		ld	c, e
		add	hl, de
		ex	de, hl
		ld	hl, SKO4Intro

loc_BBB3:				; CODE XREF: SetShip4Intro+41j
		inc	de
		ldi
		jp	pe, loc_BBB3
		ret
; End of function SetShip4Intro

; ---------------------------------------------------------------------------
; START	OF FUNCTION CHUNK FOR CallFunctionObj_1

SetRndShip:				; CODE XREF: CallFunctionObj_1+8j
		ld	a, (cargoSlaves)
		ld	c, a
		ld	a, (cargoNarc)
		add	a, c
		add	a, a
		ld	c, a
		ld	a, (cargoFireams)
		add	a, c
		ld	c, a
		ld	a, (Mission2)
		and	a
		jr	nz, loc_BBFD
		call	IntRnd_A
		cp	1Eh
		jr	nc, loc_BBFD
		ld	a, (PilotExtWeapon) ; возможность выдать ASP для 2й миссии
		and	42h ; 'B'
		cp	2
		jr	nz, loc_BBFD
		dec	a		; ставим ASP для 2й миссии
		ld	(Mission2), a
		ld	a, 0Fh
		ld	hl, sub_BBF2
		push	hl
		push	af
		call	FindSlot4Obj
		ret	nz
		ld	a, 0Dh
		jr	loc_BC65
; END OF FUNCTION CHUNK	FOR CallFunctionObj_1

; =============== S U B	R O U T	I N E =======================================

; Attributes: noreturn



; === SetFlgPrn_xx1 === CODE XREF: sub_D86Ap	sub_D872p ...
SetFlgPrn_xx1:				; CODE XREF: sub_D86Ap	sub_D872p ...
		ld	a, 1
		ld	(byte_D0D3), a
		ret
; End of function SetFlgPrn_xx1

; ---------------------------------------------------------------------------
		call	SetFlgPrn_xx1
		call	sub_D5E1
		and	3

loc_D88D:				; CODE XREF: RAM:D8A7j
		push	af
		call	sub_D5E1
		and	3Eh ; '>'
		ld	hl, unk_FD9B
		ld	d, 0
		ld	e, a
		add	hl, de
		push	hl
		call	sub_D8F8
		pop	hl
		inc	hl
		call	sub_D8F8
		pop	af
		and	a
		ret	z
		dec	a
		jr	loc_D88D
; ---------------------------------------------------------------------------
		ld	a, 2
		jr	loc_D8B0
; ---------------------------------------------------------------------------
		ret
; ---------------------------------------------------------------------------
		ld	a, 1
; START	OF FUNCTION CHUNK FOR sub_D822

loc_D8B0:				; CODE XREF: sub_D822+13j RAM:D8ABj
		ld	(FlgPrnSym1), a
		ret
; END OF FUNCTION CHUNK	FOR sub_D822
; ---------------------------------------------------------------------------
		jp	PrnCrd_CRLF
; ---------------------------------------------------------------------------
		jp	sub_E1E8
; ---------------------------------------------------------------------------
		jp	ClsAndPrintRamk
; ---------------------------------------------------------------------------
		jp	sub_E2B1
; ---------------------------------------------------------------------------
		jp	locret_D55C
; ---------------------------------------------------------------------------
		jp	sub_E36B
; ---------------------------------------------------------------------------
		jp	sub_E35A
; ---------------------------------------------------------------------------
		ld	a, 0Bh
		jp	loc_E2B8

; =============== S U B	R O U T	I N E =======================================




; === SetFuelZero === CODE XREF: sub_D1E9+Dj sub_DCE9+196p
SetFuelZero:				; CODE XREF: sub_D1E9+Dj sub_DCE9+196p
		ld	a, (Pilot_Fuel)
		and	a
		ret	z
		ld	hl, NoFuel
		set	7, (hl)
		xor	a
; End of function SetFuelZero


; =============== S U B	R O U T	I N E =======================================


SetPilotFuel:				; CODE XREF: sub_DAB4+C1p
					; PrnCountdownJump+74p	...
		cp	47h ; 'G'
		jr	c, loc_DEA8
		ld	a, 46h ; 'F'

loc_DEA8:				; CODE XREF: SetPilotFuel+2j
		ld	(Pilot_Fuel), a
		sla	a
		ld	b, a
		srl	a
		srl	a
		srl	a
		add	a, b
		ld	(Fuel),	a
		ret
; End of function SetPilotFuel

; ---------------------------------------------------------------------------
byte_DEB9:	db 0			; DATA XREF: sub_DCE9+38o


; === SetIndStation === CODE XREF: sub_74C6+6p sub_96DC+5p
SetIndStation:				; CODE XREF: sub_74C6+6p sub_96DC+5p
		ld	a, (FlgCoriolisNear)
		and	a
		ret	z
		ld	a, (IndDanger)
		cp	63h ; 'c'
		ret	z
		cp	2
		ret	z
		ld	a, 6
		ld	(IndDanger), a
		ret
; End of function SetIndStation

; ---------------------------------------------------------------------------

loc_98A2:				; CODE XREF: PrnCocpit_Ifn+41Aj
		call	CopyScaner2Scr
		ld	bc, 5
		ld	a, 0EBh	; 'ы'
		ld	hl, loc_9912
		ld	(hl), a
		ld	a, 0E5h	; 'х'
		ld	hl, loc_9915
		ld	(hl), a
		add	hl, bc
		ld	(hl), a
		ld	a, 0E1h	; 'с'
		ld	hl, loc_9935
		ld	(hl), a
		inc	hl
		ld	a, 24h ; '$'
		ld	(hl), a
		xor	a
		ld	hl, loc_9940
		ld	(hl), a
		ld	a, 0EBh	; 'ы'
		ld	hl, loc_9939
		ld	(hl), a
		jr	loc_990C

; =============== S U B	R O U T	I N E =======================================


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




; === SetMaxTonnage === CODE XREF: sub_5EC5+1Bp
SetMaxTonnage:				; CODE XREF: sub_5EC5+1Bp
					; UseEscapePod+3Fp ...
		ld	a, (PilotLargeCargo) ; установка максимальной грузоподъемности
		ld	b, a
		dec	b
		ld	a, 5

loc_80C8:				; CODE XREF: RAM:80CBj
		add	a, 0Fh
		inc	b
		jr	nz, loc_80C8
		ld	(PilotFreeCargo), a
		ret
; ---------------------------------------------------------------------------
word_80D1:	dw 411Ah		; DATA XREF: sub_813Ew	sub_813E+18r

; =============== S U B	R O U T	I N E =======================================

; проверка нажатия (и соответствующий переход) клавиш 1, 2, 3, 4, I, O,	P, K, L, enter



; === SetObjInSlot === CODE XREF: sub_5EC5p
SetObjInSlot:				; CODE XREF: sub_5EC5p
					; ReCalcCrdObj_coriolis+86p ...
		call	sub_EF01	; a - номер объекта
		ld	a, b

loc_B89C:				; CODE XREF: SetObjInSlot_Intro+4j
		and	3Fh ; '?'
		add	a, a
		add	a, a
		add	a, l
		ld	l, a
		jr	nc, loc_B8A5
		inc	h

loc_B8A5:				; CODE XREF: SetObjInSlot+Aj
		inc	hl
		ld	e, (hl)
		inc	hl
		ld	d, (hl)
		push	de
		pop	iy
		ld	(ix+24h), d
		ld	(ix+23h), e
		push	ix
		pop	de
		ld	bc, 1Fh
		ld	hl, PatternObject
		ldir
		ld	a, (iy+7)
		ld	(ix+22h), a
		ld	a, (iy+12h)
		ld	c, a
		and	6Fh ; 'o'
		ld	(ix+21h), a
		call	sub_D60B
		and	7
		rla
		rl	c
		rra
		ld	(ix+20h), a
		ld	a, (iy+6)
		srl	a
		ld	(ix+1Bh), a
		ld	a, 7Eh ; '~'
		ld	(ix+25h), a
		xor	a
		ld	(ix+26h), a
		set	6, (ix+20h)
		bit	2, (ix+21h)
		jr	nz, loc_B903
		call	IntRnd_A
		and	7
		ld	(ix+1),	a
		call	IntRnd_A
		and	7
		ld	(ix+4),	a

loc_B903:				; CODE XREF: SetObjInSlot+59j
		call	IntRnd_A
		ld	(ix+0),	a
		call	IntRnd_A
		ld	(ix+3),	a
		call	IntRnd_A
		rra
		rr	(ix+2)
		rra
		rr	(ix+5)
		xor	a
		ret
; End of function SetObjInSlot


; =============== S U B	R O U T	I N E =======================================

; Attributes: noreturn

ShipSetObject:				; CODE XREF: CallFunctionObj_1+167p
					; CallFunctionObj_1+1A5p ...
		ld	c, a
		push	ix
		call	sub_FACA
; ---------------------------------------------------------------------------
		pop	hl
		ret	nz
		push	hl
		ld	a, c
		call	sub_B71C	; hl - адрес слота родиителя
; ---------------------------------------------------------------------------
		pop	hl
		push	ix
		pop	de
		ld	bc, 1Bh
		ldir
		xor	a
		ret
; End of function ShipSetObject


; =============== S U B	R O U T	I N E =======================================


FindSlot4Obj:				; CODE XREF: ReCalcCrdObj_coriolis+81p
					; SetPlayerObj+1p ...
		ld	ix, objCoriolis
		ld	de, 27h	; '''
		ld	b, 6

loc_B93F:				; CODE XREF: FindSlot4Obj+10j
		add	ix, de
		ld	a, (ix+22h)
		and	a
		ret	z
		djnz	loc_B93F
		inc	b
		ret
; End of function FindSlot4Obj


; =============== S U B	R O U T	I N E =======================================


SetPlayerObj:				; CODE XREF: SetPlayerAlloys:SetPlayerCargop
		ld	c, a
		call	FindSlot4Obj
		ret	nz
		ld	a, c
; End of function SetPlayerObj


; =============== S U B	R O U T	I N E =======================================


SetPlayerObjSlot:			; CODE XREF: sub_9391+18j
					; CallFunctionObj_1+29Cp
		call	sub_5EC5	; запуск ракеты	игроком
					; запуск спасательной капсулы (установка кобры)
		res	7, (ix+0Eh)
		res	7, (ix+16h)
		ld	(ix+0),	a
		ld	(ix+1),	a
		ld	(ix+4),	a
		ld	(ix+6),	a
		ld	(ix+7),	a
		ret
; End of function SetPlayerObjSlot

; ---------------------------------------------------------------------------
; START	OF FUNCTION CHUNK FOR CallFunctionObj_1

Set3_4Targons:				; CODE XREF: CallFunctionObj_1+Cj
		call	SetTargonStation ; Установка станции таргонов в	межпланетном пространстве для последней	подмиссии в войне с таргонами
		and	3
		add	a, 3
		ld	c, a

loc_B973:				; CODE XREF: CallFunctionObj_1+132j
		call	FindSlot4Obj
		ret	nz
		ld	a, 0Fh
		push	bc
		call	SetObjInSlot	; a - номер объекта для	установки
		set	0, (ix+25h)
		call	IntRnd_A
		and	0Fh
		or	4
		ld	(ix+7),	a
		rra
		rra
		rra
		and	1
		ld	(ix+1),	a
		xor	a
		ld	(ix+4),	a
		call	RndDirectionObj
		pop	bc
		dec	c
		jr	nz, loc_B973
		ret
; ---------------------------------------------------------------------------

SetObjMissile:				; CODE XREF: CallFunctionObj_1+10j
		ld	a, c		; Запуск ракеты
					; Вх:  с - цель	ракеты
					;	 =0 игрок (запустил противник)
					;	<>0 номер объекта по которому запустил ракету игрок
					; Вых: z=1 успешно
		and	a
		jr	z, loc_B9CF
		ld	ix, objCoriolis
		ld	de, 111h
		add	ix, de
		ld	a, (ix+22h)
		and	a
		jr	z, loc_B9B6
		call	FindSlot4Obj
		ret	nz

loc_B9B6:				; CODE XREF: CallFunctionObj_1+146j
		ld	a, 1
		push	bc
		call	sub_9391	; c = номер объекта по которому	запустил ракету	игрок
		pop	bc
		ld	a, 6Ah ; 'j'
		ld	(ix+6),	a
		ld	a, 29h ; ')'
		ld	(ix+3),	a
		ld	a, 80h ; 'А'
		ld	(ix+5),	a
		ld	a, c
		jr	loc_B9DE
; ---------------------------------------------------------------------------

loc_B9CF:				; CODE XREF: CallFunctionObj_1+137j
		ld	a, 1
		call	ShipSetObject
; ---------------------------------------------------------------------------
		ret	nz
		ld	a, (HighDeltaSpeed)
		add	a, (ix+1Bh)
		ld	(ix+1Bh), a

loc_B9DE:				; CODE XREF: CallFunctionObj_1+163j
		ld	(ix+25h), c
		set	0, (ix+26h)
		xor	a
		ret
; ---------------------------------------------------------------------------

ShipSetShip:				; CODE XREF: CallFunctionObj_1+16j
		ld	a, 8
		dec	c
		jr	z, loc_BA0D
		call	IntRnd_A
		and	1
		add	a, 9
		dec	c
		jr	z, loc_BA0D
		ld	a, 12h
		dec	c
		jr	z, loc_BA0D
		ld	a, 0Fh
		dec	c
		jr	z, loc_BA0D
		dec	c
		ret	nz
		call	IntRnd_A
		bit	0, a
		ld	a, 10h
		jr	z, loc_BA0D
		ld	a, 11h

loc_BA0D:				; CODE XREF: CallFunctionObj_1+180j
					; CallFunctionObj_1+18Aj ...
		push	ix
		call	ShipSetObject
; ---------------------------------------------------------------------------
		pop	iy
		ret	nz
		bit	2, (iy+26h)
		call	z, RotateSKOobj
		bit	2, (iy+21h)
		ret	z
		set	2, (ix+21h)
		xor	a
		ret
; END OF FUNCTION CHUNK	FOR CallFunctionObj_1

; =============== S U B	R O U T	I N E =======================================


RotateSKOobj:				; CODE XREF: CallFunctionObj_1+1AFp
					; SetShipMinAlloys+Cp

; FUNCTION CHUNK AT BA58 SIZE 0000002B BYTES

		ld	a, (byte_B866)
		xor	1
		ld	(byte_B866), a
		jr	z, loc_BA58
		push	ix
		call	sub_BA3F
		call	sub_BA3F
		call	sub_BA3F
		pop	ix
		ret
; End of function RotateSKOobj


; =============== S U B	R O U T	I N E =======================================




; === SetObjInSlot_Intro === CODE XREF: SetShip4Intro+7p
SetObjInSlot_Intro:			; CODE XREF: SetShip4Intro+7p
		ld	hl, AdrDataShips
		ld	b, a
		jp	loc_B89C
; End of function SetObjInSlot_Intro

; ---------------------------------------------------------------------------
byte_AE3C:	db 6, 7, 8, 0Bh, 0Ch, 0Dh, 11h,	0Eh, 14h, 0Fh, 12h, 13h
					; DATA XREF: sub_728D+12Ao
					; номера кораблей для интро
		db  7Eh	; ~
		db  22h	; "
		db 0D8h	; ╪
		db 0F3h	; є
		db  76h	; v
		db 0C9h	; ╔
		db 0C9h	; ╔
		db 0C9h	; ╔

; =============== S U B	R O U T	I N E =======================================

; Получение печатных координат вершины из таблицы по номеру
; Вх:  a - номер вершины
; Вых: de - печатные координаты	вершины
;      a - знаки координат
;      z =1 вершина печатается
;      hl- не определен

GetPrnCrdVertex:			; CODE XREF: PrintObject_2:loc_6FB5p
					; PrintObject_2+83p ...
		ld	b, a
		add	a, a
		ld	hl, TablCrdPrnVertex
		jr	nc, loc_AE58
		inc	h

loc_AE58:				; CODE XREF: GetPrnCrdVertex+5j
		add	a, b
		jr	nc, loc_AE5C
		inc	h

loc_AE5C:				; CODE XREF: GetPrnCrdVertex+9j
		add	a, l
		ld	l, a
		jr	nc, loc_AE61
		inc	h

loc_AE61:				; CODE XREF: GetPrnCrdVertex+Ej
		ld	d, (hl)
		inc	hl
		ld	e, (hl)
		inc	hl
		ld	a, (hl)
		bit	7, a
		ret
; End of function GetPrnCrdVertex


; =============== S U B	R O U T	I N E =======================================


CorrAxirY_SKO:				; CODE XREF: sub_9D40+ECp sub_A28D+17p
		push	ix
		push	ix
		ex	(sp), iy
		ld	de, 9
		add	ix, de
		ld	e, 0Fh
		add	iy, de
		call	ScalarMultVectors
		ld	e, a
		call	sub_AE8A
		call	sub_AE8A
		call	sub_AE8A
		pop	iy
		pop	ix
		ret
; End of function CorrAxirY_SKO


; =============== S U B	R O U T	I N E =======================================




; === SetObjbyShip === CODE XREF: CallFunction_Obj+4p
SetObjbyShip:				; CODE XREF: CallFunction_Obj+4p
		cp	2
		jp	nz, sub_B867
		call	sub_B867	; запускает ракету противник
; ---------------------------------------------------------------------------
		ret	nz
		ld	iy, (word_B452)
		ld	a, (iy+8)
		and	0Fh
		ld	(ix+25h), a
		xor	a
		ret
; End of function SetObjbyShip


; =============== S U B	R O U T	I N E =======================================




; === SetPilotFuel === CODE XREF: sub_DAB4+C1p
SetPilotFuel:				; CODE XREF: sub_DAB4+C1p
					; PrnCountdownJump+74p	...
		cp	47h ; 'G'
		jr	c, loc_DEA8
		ld	a, 46h ; 'F'

loc_DEA8:				; CODE XREF: SetPilotFuel+2j
		ld	(Pilot_Fuel), a
		sla	a
		ld	b, a
		srl	a
		srl	a
		srl	a
		add	a, b
		ld	(Fuel),	a
		ret
; End of function SetPilotFuel

; ---------------------------------------------------------------------------
byte_DEB9:	db 0			; DATA XREF: sub_DCE9+38o


; === SetPlayerAlloys === CODE XREF: CallFunctionObj_1+22Ep
SetPlayerAlloys:			; CODE XREF: CallFunctionObj_1+22Ep
		ld	a, 3

SetPlayerCargo:				; CODE XREF: CallFunctionObj_1+28Ej
		call	SetPlayerObj
		ret	nz
		call	RndDirectionObj
		res	6, (ix+20h)
		call	IntRnd_A
		and	3Fh ; '?'
		or	10h
		ld	(ix+0),	a
		ld	a, (ix+0Ah)
		and	80h ; 'А'
		xor	80h ; 'А'
		ld	(ix+2),	a
		call	IntRnd_A
		and	3Fh ; '?'
		or	10h
		ld	(ix+3),	a
		ld	a, (ix+0Ch)
		and	80h ; 'А'
		xor	80h ; 'А'
		ld	(ix+5),	a
		ld	a, 19h
		ld	(ix+6),	a
		call	IntRnd_A
		ld	(ix+1Dh), a
		call	IntRnd_A
		and	1Fh
		or	8
		ld	(ix+1Bh), a
		xor	a
		ret
; End of function SetPlayerAlloys

; ---------------------------------------------------------------------------
; START	OF FUNCTION CHUNK FOR CallFunctionObj_1

ShipSet2Alloys:				; CODE XREF: CallFunctionObj_1+22Cj
		ld	a, 3
		call	SetShipMinAlloys
; ---------------------------------------------------------------------------
		ret	nz
		ld	a, 3
		jr	SetShipMinAlloys
; ---------------------------------------------------------------------------

SetCargo:				; CODE XREF: CallFunctionObj_1+1Ej
		bit	0, c
		ld	a, 4
		jr	z, SetShipEPodCargo
		jr	SetPlayerCargo
; ---------------------------------------------------------------------------

SetCobra_EscPod:			; CODE XREF: CallFunctionObj_1+22j
		bit	0, c
		ld	a, 2
		jr	z, SetShipEPodCargo
		ld	a, 9
		ld	ix, SlotObj_05
		call	SetPlayerObjSlot
		call	RndDirectionObj
		ld	a, 80h ; 'А'
		ld	(ix+5),	a
		ld	a, 1Ch
		ld	(ix+3),	a
		res	6, (ix+20h)
		ld	a, 4Ch ; 'L'
		ld	(ix+6),	a
		ld	a, 0C0h	; '└'
		ld	(ix+1Eh), a
		ld	a, (HighDeltaSpeed)
		add	a, (ix+1Bh)
		ld	(ix+1Bh), a
		xor	a
		ret
; ---------------------------------------------------------------------------

SetShipEPodCargo:			; CODE XREF: CallFunctionObj_1+28Cj
					; CallFunctionObj_1+294j
		push	ix
		call	ShipSetObject
; ---------------------------------------------------------------------------
		ex	(sp), ix
		pop	hl
		ret	nz
		push	hl
		ex	(sp), ix
		res	6, (ix+20h)
		ld	a, 0Ah
		ld	(ix+26h), a
		ld	a, 80h ; 'А'
		ld	(ix+1Dh), a
		xor	a
		pop	ix
		ret
; ---------------------------------------------------------------------------

Set2Minerals:				; CODE XREF: CallFunctionObj_1+26j
		ld	a, 5
		call	SetShipMinAlloys
; ---------------------------------------------------------------------------
		ret	nz
		ld	a, 5
; END OF FUNCTION CHUNK	FOR CallFunctionObj_1

; =============== S U B	R O U T	I N E =======================================

; Attributes: noreturn

SetShipMinAlloys:			; CODE XREF: CallFunctionObj_1+280p
					; CallFunctionObj_1+286j ...
		push	ix
		call	ShipSetObject
; ---------------------------------------------------------------------------
		ex	(sp), ix
		pop	hl
		ret	nz
		push	hl
		ex	(sp), ix
		call	RotateSKOobj
		ld	a, 0Ah
		ld	(ix+26h), a
		res	6, (ix+20h)
		set	7, (ix+1Dh)
		pop	ix
		xor	a
		ret
; End of function SetShipMinAlloys


; =============== S U B	R O U T	I N E =======================================


SetShip4Intro:				; CODE XREF: sub_728D+132p
					; CallFunctionObj_1+2Aj
		ld	ix, objMissile
		ld	a, c
		add	a, 2
		call	SetObjInSlot_Intro
		ld	a, 0C0h	; '└'
		ld	(ix+1Dh), a
		ld	(ix+1Eh), a
		xor	a
		ld	(ix+6),	a
		ld	(ix+0),	a
		ld	(ix+1),	a
		ld	(ix+3),	a
		ld	(ix+4),	a
		ld	(ix+8),	a
		ld	(ix+1Bh), a
		res	6, (ix+20h)
		inc	a
		inc	a
		ld	(ix+7),	a
		push	ix
		pop	hl
		ld	de, 9
		ld	b, d
		ld	c, e
		add	hl, de
		ex	de, hl
		ld	hl, SKO4Intro

loc_BBB3:				; CODE XREF: SetShip4Intro+41j
		inc	de
		ldi
		jp	pe, loc_BBB3
		ret
; End of function SetShip4Intro

; ---------------------------------------------------------------------------
; START	OF FUNCTION CHUNK FOR CallFunctionObj_1

SetRndShip:				; CODE XREF: CallFunctionObj_1+8j
		ld	a, (cargoSlaves)
		ld	c, a
		ld	a, (cargoNarc)
		add	a, c
		add	a, a
		ld	c, a
		ld	a, (cargoFireams)
		add	a, c
		ld	c, a
		ld	a, (Mission2)
		and	a
		jr	nz, loc_BBFD
		call	IntRnd_A
		cp	1Eh
		jr	nc, loc_BBFD
		ld	a, (PilotExtWeapon) ; возможность выдать ASP для 2й миссии
		and	42h ; 'B'
		cp	2
		jr	nz, loc_BBFD
		dec	a		; ставим ASP для 2й миссии
		ld	(Mission2), a
		ld	a, 0Fh
		ld	hl, sub_BBF2
		push	hl
		push	af
		call	FindSlot4Obj
		ret	nz
		ld	a, 0Dh
		jr	loc_BC65
; END OF FUNCTION CHUNK	FOR CallFunctionObj_1

; =============== S U B	R O U T	I N E =======================================

; Attributes: noreturn



; === SetPlayerObj === CODE XREF: SetPlayerAlloys:SetPlayerCargop
SetPlayerObj:				; CODE XREF: SetPlayerAlloys:SetPlayerCargop
		ld	c, a
		call	FindSlot4Obj
		ret	nz
		ld	a, c
; End of function SetPlayerObj


; =============== S U B	R O U T	I N E =======================================


SetPlayerObjSlot:			; CODE XREF: sub_9391+18j
					; CallFunctionObj_1+29Cp
		call	sub_5EC5	; запуск ракеты	игроком
					; запуск спасательной капсулы (установка кобры)
		res	7, (ix+0Eh)
		res	7, (ix+16h)
		ld	(ix+0),	a
		ld	(ix+1),	a
		ld	(ix+4),	a
		ld	(ix+6),	a
		ld	(ix+7),	a
		ret
; End of function SetPlayerObjSlot

; ---------------------------------------------------------------------------
; START	OF FUNCTION CHUNK FOR CallFunctionObj_1

Set3_4Targons:				; CODE XREF: CallFunctionObj_1+Cj
		call	SetTargonStation ; Установка станции таргонов в	межпланетном пространстве для последней	подмиссии в войне с таргонами
		and	3
		add	a, 3
		ld	c, a

loc_B973:				; CODE XREF: CallFunctionObj_1+132j
		call	FindSlot4Obj
		ret	nz
		ld	a, 0Fh
		push	bc
		call	SetObjInSlot	; a - номер объекта для	установки
		set	0, (ix+25h)
		call	IntRnd_A
		and	0Fh
		or	4
		ld	(ix+7),	a
		rra
		rra
		rra
		and	1
		ld	(ix+1),	a
		xor	a
		ld	(ix+4),	a
		call	RndDirectionObj
		pop	bc
		dec	c
		jr	nz, loc_B973
		ret
; ---------------------------------------------------------------------------

SetObjMissile:				; CODE XREF: CallFunctionObj_1+10j
		ld	a, c		; Запуск ракеты
					; Вх:  с - цель	ракеты
					;	 =0 игрок (запустил противник)
					;	<>0 номер объекта по которому запустил ракету игрок
					; Вых: z=1 успешно
		and	a
		jr	z, loc_B9CF
		ld	ix, objCoriolis
		ld	de, 111h
		add	ix, de
		ld	a, (ix+22h)
		and	a
		jr	z, loc_B9B6
		call	FindSlot4Obj
		ret	nz

loc_B9B6:				; CODE XREF: CallFunctionObj_1+146j
		ld	a, 1
		push	bc
		call	sub_9391	; c = номер объекта по которому	запустил ракету	игрок
		pop	bc
		ld	a, 6Ah ; 'j'
		ld	(ix+6),	a
		ld	a, 29h ; ')'
		ld	(ix+3),	a
		ld	a, 80h ; 'А'
		ld	(ix+5),	a
		ld	a, c
		jr	loc_B9DE
; ---------------------------------------------------------------------------

loc_B9CF:				; CODE XREF: CallFunctionObj_1+137j
		ld	a, 1
		call	ShipSetObject
; ---------------------------------------------------------------------------
		ret	nz
		ld	a, (HighDeltaSpeed)
		add	a, (ix+1Bh)
		ld	(ix+1Bh), a

loc_B9DE:				; CODE XREF: CallFunctionObj_1+163j
		ld	(ix+25h), c
		set	0, (ix+26h)
		xor	a
		ret
; ---------------------------------------------------------------------------

ShipSetShip:				; CODE XREF: CallFunctionObj_1+16j
		ld	a, 8
		dec	c
		jr	z, loc_BA0D
		call	IntRnd_A
		and	1
		add	a, 9
		dec	c
		jr	z, loc_BA0D
		ld	a, 12h
		dec	c
		jr	z, loc_BA0D
		ld	a, 0Fh
		dec	c
		jr	z, loc_BA0D
		dec	c
		ret	nz
		call	IntRnd_A
		bit	0, a
		ld	a, 10h
		jr	z, loc_BA0D
		ld	a, 11h

loc_BA0D:				; CODE XREF: CallFunctionObj_1+180j
					; CallFunctionObj_1+18Aj ...
		push	ix
		call	ShipSetObject
; ---------------------------------------------------------------------------
		pop	iy
		ret	nz
		bit	2, (iy+26h)
		call	z, RotateSKOobj
		bit	2, (iy+21h)
		ret	z
		set	2, (ix+21h)
		xor	a
		ret
; END OF FUNCTION CHUNK	FOR CallFunctionObj_1

; =============== S U B	R O U T	I N E =======================================


RotateSKOobj:				; CODE XREF: CallFunctionObj_1+1AFp
					; SetShipMinAlloys+Cp

; FUNCTION CHUNK AT BA58 SIZE 0000002B BYTES

		ld	a, (byte_B866)
		xor	1
		ld	(byte_B866), a
		jr	z, loc_BA58
		push	ix
		call	sub_BA3F
		call	sub_BA3F
		call	sub_BA3F
		pop	ix
		ret
; End of function RotateSKOobj


; =============== S U B	R O U T	I N E =======================================




; === SetPlayerObjSlot === CODE XREF: sub_9391+18j
SetPlayerObjSlot:			; CODE XREF: sub_9391+18j
					; CallFunctionObj_1+29Cp
		call	sub_5EC5	; запуск ракеты	игроком
					; запуск спасательной капсулы (установка кобры)
		res	7, (ix+0Eh)
		res	7, (ix+16h)
		ld	(ix+0),	a
		ld	(ix+1),	a
		ld	(ix+4),	a
		ld	(ix+6),	a
		ld	(ix+7),	a
		ret
; End of function SetPlayerObjSlot

; ---------------------------------------------------------------------------
; START	OF FUNCTION CHUNK FOR CallFunctionObj_1

Set3_4Targons:				; CODE XREF: CallFunctionObj_1+Cj
		call	SetTargonStation ; Установка станции таргонов в	межпланетном пространстве для последней	подмиссии в войне с таргонами
		and	3
		add	a, 3
		ld	c, a

loc_B973:				; CODE XREF: CallFunctionObj_1+132j
		call	FindSlot4Obj
		ret	nz
		ld	a, 0Fh
		push	bc
		call	SetObjInSlot	; a - номер объекта для	установки
		set	0, (ix+25h)
		call	IntRnd_A
		and	0Fh
		or	4
		ld	(ix+7),	a
		rra
		rra
		rra
		and	1
		ld	(ix+1),	a
		xor	a
		ld	(ix+4),	a
		call	RndDirectionObj
		pop	bc
		dec	c
		jr	nz, loc_B973
		ret
; ---------------------------------------------------------------------------

SetObjMissile:				; CODE XREF: CallFunctionObj_1+10j
		ld	a, c		; Запуск ракеты
					; Вх:  с - цель	ракеты
					;	 =0 игрок (запустил противник)
					;	<>0 номер объекта по которому запустил ракету игрок
					; Вых: z=1 успешно
		and	a
		jr	z, loc_B9CF
		ld	ix, objCoriolis
		ld	de, 111h
		add	ix, de
		ld	a, (ix+22h)
		and	a
		jr	z, loc_B9B6
		call	FindSlot4Obj
		ret	nz

loc_B9B6:				; CODE XREF: CallFunctionObj_1+146j
		ld	a, 1
		push	bc
		call	sub_9391	; c = номер объекта по которому	запустил ракету	игрок
		pop	bc
		ld	a, 6Ah ; 'j'
		ld	(ix+6),	a
		ld	a, 29h ; ')'
		ld	(ix+3),	a
		ld	a, 80h ; 'А'
		ld	(ix+5),	a
		ld	a, c
		jr	loc_B9DE
; ---------------------------------------------------------------------------

loc_B9CF:				; CODE XREF: CallFunctionObj_1+137j
		ld	a, 1
		call	ShipSetObject
; ---------------------------------------------------------------------------
		ret	nz
		ld	a, (HighDeltaSpeed)
		add	a, (ix+1Bh)
		ld	(ix+1Bh), a

loc_B9DE:				; CODE XREF: CallFunctionObj_1+163j
		ld	(ix+25h), c
		set	0, (ix+26h)
		xor	a
		ret
; ---------------------------------------------------------------------------

ShipSetShip:				; CODE XREF: CallFunctionObj_1+16j
		ld	a, 8
		dec	c
		jr	z, loc_BA0D
		call	IntRnd_A
		and	1
		add	a, 9
		dec	c
		jr	z, loc_BA0D
		ld	a, 12h
		dec	c
		jr	z, loc_BA0D
		ld	a, 0Fh
		dec	c
		jr	z, loc_BA0D
		dec	c
		ret	nz
		call	IntRnd_A
		bit	0, a
		ld	a, 10h
		jr	z, loc_BA0D
		ld	a, 11h

loc_BA0D:				; CODE XREF: CallFunctionObj_1+180j
					; CallFunctionObj_1+18Aj ...
		push	ix
		call	ShipSetObject
; ---------------------------------------------------------------------------
		pop	iy
		ret	nz
		bit	2, (iy+26h)
		call	z, RotateSKOobj
		bit	2, (iy+21h)
		ret	z
		set	2, (ix+21h)
		xor	a
		ret
; END OF FUNCTION CHUNK	FOR CallFunctionObj_1

; =============== S U B	R O U T	I N E =======================================


RotateSKOobj:				; CODE XREF: CallFunctionObj_1+1AFp
					; SetShipMinAlloys+Cp

; FUNCTION CHUNK AT BA58 SIZE 0000002B BYTES

		ld	a, (byte_B866)
		xor	1
		ld	(byte_B866), a
		jr	z, loc_BA58
		push	ix
		call	sub_BA3F
		call	sub_BA3F
		call	sub_BA3F
		pop	ix
		ret
; End of function RotateSKOobj


; =============== S U B	R O U T	I N E =======================================




; === SetRndShip === CODE XREF: CallFunctionObj_1+8j
SetRndShip:				; CODE XREF: CallFunctionObj_1+8j
		ld	a, (cargoSlaves)
		ld	c, a
		ld	a, (cargoNarc)
		add	a, c
		add	a, a
		ld	c, a
		ld	a, (cargoFireams)
		add	a, c
		ld	c, a
		ld	a, (Mission2)
		and	a
		jr	nz, loc_BBFD
		call	IntRnd_A
		cp	1Eh
		jr	nc, loc_BBFD
		ld	a, (PilotExtWeapon) ; возможность выдать ASP для 2й миссии
		and	42h ; 'B'
		cp	2
		jr	nz, loc_BBFD
		dec	a		; ставим ASP для 2й миссии
		ld	(Mission2), a
		ld	a, 0Fh
		ld	hl, sub_BBF2
		push	hl
		push	af
		call	FindSlot4Obj
		ret	nz
		ld	a, 0Dh
		jr	loc_BC65
; END OF FUNCTION CHUNK	FOR CallFunctionObj_1

; =============== S U B	R O U T	I N E =======================================

; Attributes: noreturn



; === SetShip4Intro === CODE XREF: sub_728D+132p
SetShip4Intro:				; CODE XREF: sub_728D+132p
					; CallFunctionObj_1+2Aj
		ld	ix, objMissile
		ld	a, c
		add	a, 2
		call	SetObjInSlot_Intro
		ld	a, 0C0h	; '└'
		ld	(ix+1Dh), a
		ld	(ix+1Eh), a
		xor	a
		ld	(ix+6),	a
		ld	(ix+0),	a
		ld	(ix+1),	a
		ld	(ix+3),	a
		ld	(ix+4),	a
		ld	(ix+8),	a
		ld	(ix+1Bh), a
		res	6, (ix+20h)
		inc	a
		inc	a
		ld	(ix+7),	a
		push	ix
		pop	hl
		ld	de, 9
		ld	b, d
		ld	c, e
		add	hl, de
		ex	de, hl
		ld	hl, SKO4Intro

loc_BBB3:				; CODE XREF: SetShip4Intro+41j
		inc	de
		ldi
		jp	pe, loc_BBB3
		ret
; End of function SetShip4Intro

; ---------------------------------------------------------------------------
; START	OF FUNCTION CHUNK FOR CallFunctionObj_1

SetRndShip:				; CODE XREF: CallFunctionObj_1+8j
		ld	a, (cargoSlaves)
		ld	c, a
		ld	a, (cargoNarc)
		add	a, c
		add	a, a
		ld	c, a
		ld	a, (cargoFireams)
		add	a, c
		ld	c, a
		ld	a, (Mission2)
		and	a
		jr	nz, loc_BBFD
		call	IntRnd_A
		cp	1Eh
		jr	nc, loc_BBFD
		ld	a, (PilotExtWeapon) ; возможность выдать ASP для 2й миссии
		and	42h ; 'B'
		cp	2
		jr	nz, loc_BBFD
		dec	a		; ставим ASP для 2й миссии
		ld	(Mission2), a
		ld	a, 0Fh
		ld	hl, sub_BBF2
		push	hl
		push	af
		call	FindSlot4Obj
		ret	nz
		ld	a, 0Dh
		jr	loc_BC65
; END OF FUNCTION CHUNK	FOR CallFunctionObj_1

; =============== S U B	R O U T	I N E =======================================

; Attributes: noreturn



; === SetShipMinAlloys === CODE XREF: CallFunctionObj_1+280p
SetShipMinAlloys:			; CODE XREF: CallFunctionObj_1+280p
					; CallFunctionObj_1+286j ...
		push	ix
		call	ShipSetObject
; ---------------------------------------------------------------------------
		ex	(sp), ix
		pop	hl
		ret	nz
		push	hl
		ex	(sp), ix
		call	RotateSKOobj
		ld	a, 0Ah
		ld	(ix+26h), a
		res	6, (ix+20h)
		set	7, (ix+1Dh)
		pop	ix
		xor	a
		ret
; End of function SetShipMinAlloys


; =============== S U B	R O U T	I N E =======================================


SetShip4Intro:				; CODE XREF: sub_728D+132p
					; CallFunctionObj_1+2Aj
		ld	ix, objMissile
		ld	a, c
		add	a, 2
		call	SetObjInSlot_Intro
		ld	a, 0C0h	; '└'
		ld	(ix+1Dh), a
		ld	(ix+1Eh), a
		xor	a
		ld	(ix+6),	a
		ld	(ix+0),	a
		ld	(ix+1),	a
		ld	(ix+3),	a
		ld	(ix+4),	a
		ld	(ix+8),	a
		ld	(ix+1Bh), a
		res	6, (ix+20h)
		inc	a
		inc	a
		ld	(ix+7),	a
		push	ix
		pop	hl
		ld	de, 9
		ld	b, d
		ld	c, e
		add	hl, de
		ex	de, hl
		ld	hl, SKO4Intro

loc_BBB3:				; CODE XREF: SetShip4Intro+41j
		inc	de
		ldi
		jp	pe, loc_BBB3
		ret
; End of function SetShip4Intro

; ---------------------------------------------------------------------------
; START	OF FUNCTION CHUNK FOR CallFunctionObj_1

SetRndShip:				; CODE XREF: CallFunctionObj_1+8j
		ld	a, (cargoSlaves)
		ld	c, a
		ld	a, (cargoNarc)
		add	a, c
		add	a, a
		ld	c, a
		ld	a, (cargoFireams)
		add	a, c
		ld	c, a
		ld	a, (Mission2)
		and	a
		jr	nz, loc_BBFD
		call	IntRnd_A
		cp	1Eh
		jr	nc, loc_BBFD
		ld	a, (PilotExtWeapon) ; возможность выдать ASP для 2й миссии
		and	42h ; 'B'
		cp	2
		jr	nz, loc_BBFD
		dec	a		; ставим ASP для 2й миссии
		ld	(Mission2), a
		ld	a, 0Fh
		ld	hl, sub_BBF2
		push	hl
		push	af
		call	FindSlot4Obj
		ret	nz
		ld	a, 0Dh
		jr	loc_BC65
; END OF FUNCTION CHUNK	FOR CallFunctionObj_1

; =============== S U B	R O U T	I N E =======================================

; Attributes: noreturn



; === SetShipwreck === CODE XREF: RAM:752Cp
SetShipwreck:				; CODE XREF: RAM:752Cp
		jp	SetShipwreck2
; End of function SetShipwreck

; ---------------------------------------------------------------------------


; === SetShipwreck2 === CODE XREF: SetShipwreckj
SetShipwreck2:				; CODE XREF: SetShipwreckj
		ld	hl, nBeepFX
		set	2, (hl)
		xor	a
		ld	(WhatOnScreen),	a
		ld	ix, loc_6021
		ld	de, 27h	; '''
		ld	b, 8

loc_B62A:				; CODE XREF: SetShipwreck2+17j
		add	ix, de
		ld	(ix+22h), a
		djnz	loc_B62A
		ld	a, 16h
		call	sub_B867
; End of function SetShipwreck2

; ---------------------------------------------------------------------------
		ld	a, 15h
		call	sub_B867
; ---------------------------------------------------------------------------
		ret

; =============== S U B	R O U T	I N E =======================================


FireLaser:				; CODE XREF: RAM:B7F1p

; FUNCTION CHUNK AT B749 SIZE 00000050 BYTES

		ld	a, (byte_B4D7)
		ld	c, a
		xor	a
		ld	(byte_B4D7), a
		ld	a, (TempLaser)
		cp	2
		jr	nc, loc_B64D
		ld	a, 2

loc_B64D:				; CODE XREF: FireLaser+Dj
		sub	2
		ld	(TempLaser), a
		ld	hl, ControlKey3
		bit	0, (hl)
		ret	z
		res	0, (hl)
		cp	90h ; 'Р'
		ret	nc
		ld	b, a
		ld	a, (WhatOnScreen)
		ld	hl, PilotFrontLaser
		ld	d, 0
		ld	e, a
		add	hl, de
		ld	a, (hl)
		and	a
		ret	z
		ld	e, c
		ld	c, a
		dec	a
		jr	nz, loc_B672
		ld	d, 1

loc_B672:				; CODE XREF: FireLaser+32j
		sub	3
		jr	nz, loc_B678
		ld	d, 7

loc_B678:				; CODE XREF: FireLaser+38j
		ld	a, e
		and	e
		ld	a, 1
		ld	(byte_B4D7), a
		jr	z, loc_B686
		ld	a, (FrameCounter)
		and	d
		ret	nz

loc_B686:				; CODE XREF: FireLaser+43j
		ld	hl, DamageEnergyUnit
		inc	(hl)
		call	sub_9389
		ld	hl, nBeepFX
		set	4, (hl)
		ld	a, c
		sub	3
		ld	a, 0Fh
		jr	c, loc_B69F
		jr	z, loc_B69D
		add	a, 1Bh

loc_B69D:				; CODE XREF: FireLaser+5Dj
		add	a, 8

loc_B69F:				; CODE XREF: FireLaser+5Bj
		ld	c, a
		ld	a, d
		dec	a
		and	1
		rla
		add	a, 8
		add	a, b
		cp	0A0h ; 'а'
		jr	c, loc_B6AE
		ld	a, 0A0h	; 'а'

loc_B6AE:				; CODE XREF: FireLaser+6Ej
		ld	(TempLaser), a
		ld	a, (ObjInTarget)
		and	a
		jp	z, PrnLaser
		dec	a
		sub	9
		cpl
		ld	ix, objCoriolis
		ld	de, 27h	; '''

loc_B6C3:				; CODE XREF: FireLaser+8Cj
		dec	a
		jr	z, loc_B6CA
		add	ix, de
		jr	loc_B6C3
; ---------------------------------------------------------------------------

loc_B6CA:				; CODE XREF: FireLaser+88j
		call	sub_F9E1
		nop
		ld	hl, nBeepFX
		set	3, (hl)
		bit	2, (ix+26h)
		jr	nz, PrnLaser
		bit	5, (ix+20h)
		jr	nz, PrnLaser
		ld	a, (ix+25h)
		ld	b, a

loc_B6E3:
		sub	0Eh
		jr	nc, loc_B6EB
		and	1
		add	a, 6

loc_B6EB:				; CODE XREF: FireLaser+A9j
		rla
		sla	b
		rra
		ld	(ix+25h), a
		ld	a, (ix+22h)
		cp	0FFh
		jr	z, PrnLaser
		sub	c
		ld	(ix+22h), a
		jr	z, loc_B701
		jr	nc, PrnLaser

loc_B701:				; CODE XREF: FireLaser+C1j
		xor	a
		dec	a
		set	4, (ix+20h)
		ld	(ix+22h), a
		call	DestroyObj
; ---------------------------------------------------------------------------
		jr	PrnLaser
; End of function FireLaser

; [00000003 BYTES: COLLAPSED FUNCTION j_DestroyObj. PRESS KEYPAD "+" TO EXPAND]

; =============== S U B	R O U T	I N E =======================================


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



; === Sqr_DE_A === CODE XREF: CalcLenVectorCrd+29p
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




; === Sqr_HL_wordAdrIX === CODE XREF: CalcLenVector+2p
Sqr_HL_wordAdrIX:			; CODE XREF: CalcLenVector+2p
					; CalcLenVector+6p ...
		ld	b, (ix+1)
		ld	c, (ix+0)
		res	7, b
		push	ix
		ld	ix, 0
		call	Mult_HL_B_C
		xor	a
		ld	d, a
		add	hl, hl
		rl	d
		ld	e, h
		add	ix, de
		ld	h, l
		ld	l, a
		push	hl
		ld	h, 0BEh	; '╛'
		ld	l, b
		ld	e, (hl)
		inc	h
		ld	d, (hl)
		dec	h
		add	ix, de
		ld	l, c
		ld	e, (hl)
		inc	h
		ld	d, (hl)
		pop	hl
		add	hl, de
		jr	nc, loc_9EC7
		inc	ix

loc_9EC7:				; CODE XREF: Sqr_HL_wordAdrIX+2Bj
		ex	(sp), ix
		pop	hl
		inc	ix
		inc	ix
		ret
; End of function Sqr_HL_wordAdrIX


; =============== S U B	R O U T	I N E =======================================

; Attributes: noreturn

CaclNormalVector:			; CODE XREF: CalcLenVector+19p
					; CalcLenVector+1Cp ...
		ld	h, (ix+1)
		ld	l, (ix+0)
		push	hl
		res	7, h
		call	Div_BC_HLAdivDE_1
; ---------------------------------------------------------------------------
		srl	b
		rr	c
		pop	hl
		bit	7, h
		jr	z, loc_9EE6
		set	7, b

loc_9EE6:				; CODE XREF: CaclNormalVector+13j
		ld	(ix+0),	c
		ld	(ix+1),	b
		inc	ix
		inc	ix
		ret
; End of function CaclNormalVector


; =============== S U B	R O U T	I N E =======================================

; Attributes: noreturn

Sqrt_H_HL:				; CODE XREF: j_Sqrt_H_HLj
					; CalcLenVector+13p
		ld	a, h
		cp	10h
		jr	nc, sub_9F18
		and	a
		jr	z, loc_9F0A
		ld	b, 4

loc_9EFB:				; CODE XREF: Sqrt_H_HL+Bj
		add	hl, hl
		djnz	loc_9EFB
		call	sub_9F18
; ---------------------------------------------------------------------------
		srl	h
		rr	l
		srl	h
		rr	l
		ret
; ---------------------------------------------------------------------------

loc_9F0A:				; CODE XREF: Sqrt_H_HL+6j
		ld	h, l
		ld	l, a
		call	sub_9F18
; ---------------------------------------------------------------------------
		ld	b, 4

loc_9F11:				; CODE XREF: Sqrt_H_HL+24j
		srl	h
		rr	l
		djnz	loc_9F11
		ret
; End of function Sqrt_H_HL


; =============== S U B	R O U T	I N E =======================================

; Attributes: noreturn



; === Sqrt_H_HL === CODE XREF: j_Sqrt_H_HLj
Sqrt_H_HL:				; CODE XREF: j_Sqrt_H_HLj
					; CalcLenVector+13p
		ld	a, h
		cp	10h
		jr	nc, sub_9F18
		and	a
		jr	z, loc_9F0A
		ld	b, 4

loc_9EFB:				; CODE XREF: Sqrt_H_HL+Bj
		add	hl, hl
		djnz	loc_9EFB
		call	sub_9F18
; ---------------------------------------------------------------------------
		srl	h
		rr	l
		srl	h
		rr	l
		ret
; ---------------------------------------------------------------------------

loc_9F0A:				; CODE XREF: Sqrt_H_HL+6j
		ld	h, l
		ld	l, a
		call	sub_9F18
; ---------------------------------------------------------------------------
		ld	b, 4

loc_9F11:				; CODE XREF: Sqrt_H_HL+24j
		srl	h
		rr	l
		djnz	loc_9F11
		ret
; End of function Sqrt_H_HL


; =============== S U B	R O U T	I N E =======================================

; Attributes: noreturn



; === TestMainDiagonal === CODE XREF: ReCalcCrdObj_coriolis+6p
TestMainDiagonal:			; CODE XREF: ReCalcCrdObj_coriolis+6p
					; sub_7370+Fp
		ld	hl, Pilot_CrdYSystem
		ld	a, (hl)
		inc	hl
		inc	hl
		sub	(hl)
		jr	nc, loc_8D35
		cpl

loc_8D35:				; CODE XREF: TestMainDiagonal+7j
		and	0F8h ; '°'
		ret
; End of function TestMainDiagonal

; ---------------------------------------------------------------------------

loc_8D38:				; CODE XREF: RAM:EFA0p
		ld	hl, 306h
		ld	(CrdXPrnSymbol), hl
		ld	a, (byte_D08E)	; <>0 есть Cobra Mk4
		and	a
		ld	a, 0F9h	; '∙'
		call	nz, PrintMessage1_A
		ld	a, 4
		jp	loc_E2C8
; ---------------------------------------------------------------------------
; START	OF FUNCTION CHUNK FOR sub_8643

loc_8D4C:				; CODE XREF: sub_8643+6E6j
		jr	nz, loc_8D53
		call	sub_8DEE
		jr	loc_8D56
; ---------------------------------------------------------------------------

loc_8D53:				; CODE XREF: sub_8643:loc_8D4Cj
		call	loc_8E08

loc_8D56:				; CODE XREF: sub_8643+70Ej
		exx
		add	hl, de
		inc	e
		add	hl, de
		bit	7, h
		exx
		jr	nz, loc_8D6F
		srl	b
		jr	nc, loc_8D74

loc_8D63:				; CODE XREF: sub_8643+738j
		exx
		add	hl, bc
		inc	c
		ex	af, af'
		cp	c
		jr	z, loc_8D7D
		ex	af, af'
		add	hl, bc
		bit	7, h
		exx

loc_8D6F:				; CODE XREF: sub_8643+71Aj
		jp	nz, loc_8D17
		jr	loc_8D94
; ---------------------------------------------------------------------------

loc_8D74:				; CODE XREF: sub_8643+71Ej
		inc	e
		ld	a, l
		sub	e
		jr	z, loc_8D85
		ld	b, 7Fh ; ''
		jr	loc_8D63
; ---------------------------------------------------------------------------

loc_8D7D:				; CODE XREF: sub_8643+725j
		ex	af, af'
		jp	loc_8B69
; ---------------------------------------------------------------------------

loc_8D81:				; CODE XREF: sub_8643+75Cj
		ex	af, af'
		jp	loc_8BA9
; ---------------------------------------------------------------------------

loc_8D85:				; CODE XREF: sub_8643+734j
					; sub_8643+74Bj ...
		ld	sp, (word_8623)
		pop	af
		ret
; ---------------------------------------------------------------------------

loc_8D8B:				; CODE XREF: sub_8643+753j
		inc	e
		ld	a, l
		sub	e
		jr	z, loc_8D85
		ld	b, 7Fh ; ''
		jr	loc_8D98
; ---------------------------------------------------------------------------

loc_8D94:				; CODE XREF: sub_8643+72Fj
					; sub_8643+763j ...
		srl	b
		jr	nc, loc_8D8B

loc_8D98:				; CODE XREF: sub_8643+74Fj
		exx
		add	hl, bc
		inc	c
		jr	z, loc_8D85
		ex	af, af'
		cp	c
		jr	z, loc_8D81
		ex	af, af'
		add	hl, bc
		bit	7, h
		exx
		jr	z, loc_8D94
		exx
		add	hl, de
		inc	e
		add	hl, de
		exx
		ld	d, l
		ld	l, e
		ld	a, (hl)
		or	b
		ld	(hl), a
		ld	a, d
		sub	e
		dec	a
		srl	a
		jr	nc, loc_8DBC
		inc	l
		ld	(hl), 0FFh

loc_8DBC:				; CODE XREF: sub_8643+774j
		ld	l, d
		sub	11h
		cpl
		ld	(loc_8DC8+1), a
		ld	sp, 8FB6h
		pop	af
		ld	sp, hl

loc_8DC8:				; CODE XREF: sub_8643:loc_8DC8j
					; DATA XREF: sub_8643+77Dw
		jr	$
; END OF FUNCTION CHUNK	FOR sub_8643
; ---------------------------------------------------------------------------
		push	af
		push	af
		push	af
		push	af
		push	af
		push	af
		push	af
		push	af
		push	af
		push	af
		push	af
		push	af
		push	af
		push	af
		push	af
		push	af
		ld	a, (DirPrnCircle)
		and	a
		ld	sp, (word_8623)
		jr	nz, loc_8DE9
		call	sub_8DEE
		jr	loc_8D94
; ---------------------------------------------------------------------------

loc_8DE9:				; CODE XREF: RAM:8DE2j
		call	loc_8E08
		jr	loc_8D94

; =============== S U B	R O U T	I N E =======================================




; === TestObjInRadius === CODE XREF: DirectObject+14p
TestObjInRadius:			; CODE XREF: DirectObject+14p
					; DirectObject+21p
		ld	h, (ix+1)
		ld	l, (ix+0)
		and	a
		sbc	hl, de
		ret	nc
		ld	h, (ix+4)
		ld	l, (ix+3)
		and	a
		sbc	hl, de
		ret	nc
		ld	h, (ix+7)
		ld	l, (ix+6)
		inc	d
		inc	d
		and	a
		sbc	hl, de
		ret
; End of function TestObjInRadius


; =============== S U B	R O U T	I N E =======================================

; Attributes: noreturn

DirectObject:				; CODE XREF: ReactionObject+D2j
					; ReactionObject+117j
		call	GetDirectObj
		ret	nc
		jr	z, loc_A440
		ld	a, (PilotRating0)
		cp	28h ; '('
		jr	c, loc_A440
		set	2, (ix+21h)

loc_A440:				; CODE XREF: DirectObject+4j
					; DirectObject+Bj
		ld	de, 200h
		call	TestObjInRadius	; в радиусе реакции
		jr	c, MoveObjFromPlayer
		bit	5, (ix+26h)
		jr	z, loc_A455
		ld	d, 0Ah
		call	TestObjInRadius
		jr	c, MoveObjFromPlayer

loc_A455:				; CODE XREF: DirectObject+1Dj
		bit	2, (ix+21h)
		ret	z
		ld	a, (PilotMission3) ; =#06 при стыковке будет захват таргонами (вроде нереальная	ситуация)
					; =#04 в системе со танцией таргонов во	время войны
		cp	4
		jr	z, loc_A46F
		ld	a, (byte_606A)	; objCoriolis+oEnerg
		and	a
		jr	z, loc_A46F
		bit	3, (ix+21h)
		jr	z, MoveObjToPlayer
		jr	MoveObjFromPlayer
; ---------------------------------------------------------------------------

loc_A46F:				; CODE XREF: DirectObject+30j
					; DirectObject+36j
		ld	a, (ix+25h)
		and	7Fh ; ''
		cp	8
		jr	nc, MoveObjToPlayer
		set	7, (ix+25h)

MoveObjFromPlayer:			; CODE XREF: DirectObject+17j
					; DirectObject+24j ...
		push	ix
		call	HL_CrdNdiv4
		ld	(deltaCrdX), hl
		call	HL_CrdNdiv4
		ld	(deltaCrdY), hl
		call	HL_CrdNdiv4
		ld	(deltaCrdZ), hl
		pop	ix
		set	5, (ix+26h)
		jr	MoveObject1
; ---------------------------------------------------------------------------

MoveObjToPlayer:			; CODE XREF: DirectObject+3Cj
					; DirectObject+47j ...
		push	ix
		call	HL_CrdNdiv4_InvSign
		ld	(deltaCrdX), hl
		call	HL_CrdNdiv4_InvSign
		ld	(deltaCrdY), hl
		call	HL_CrdNdiv4_InvSign
		ld	(deltaCrdZ), hl
		pop	ix
		res	5, (ix+26h)

MoveObject1:				; CODE XREF: DirectObject+67j
					; MoveMissile+7Fj
		jp	MoveObject
; End of function DirectObject


; =============== S U B	R O U T	I N E =======================================


MoveMissile:				; CODE XREF: sub_9D40+DFp
		ld	a, (TimeWorkECM)
		and	a
		jr	z, loc_A4C4
		call	sub_7D3E
		ret	nz
		set	4, (ix+20h)
		ret
; ---------------------------------------------------------------------------

loc_A4C4:				; CODE XREF: MoveMissile+4j
		ld	a, c
		and	0Fh
		and	a
		jr	z, MoveObjToPlayer
		push	iy
		ld	iy, loc_6021
		ld	de, 27h	; '''
		sub	0Ah
		cpl
		ld	b, a

loc_A4D7:				; CODE XREF: MoveMissile+24j
		add	iy, de
		djnz	loc_A4D7
		push	ix
		push	iy
		call	sub_A53F
		ld	(deltaCrdX), hl
		call	sub_A53F
		ld	(deltaCrdY), hl
		call	sub_A53F
		ld	(deltaCrdZ), hl
		pop	iy
		pop	ix
		call	sub_A537
		jr	nz, loc_A532
		ld	hl, (deltaCrdY)
		call	sub_A537
		jr	nz, loc_A532
		ld	hl, (deltaCrdX)
		call	sub_F97E
		jr	nz, loc_A532
		bit	2, (iy+26h)
		jr	z, loc_A527
		ld	hl, PilotMission3 ; =#06 при стыковке будет захват таргонами (вроде нереальная ситуация)
					; =#04 в системе со танцией таргонов во	время войны
		call	sub_D6AA
		jr	nz, loc_A52B
		ld	hl, planetAlienItems
		xor	a
		ld	(hl), a
		ld	de, 0FFFFh
		call	j_IncBattleRating2
		set	7, (iy+26h)

loc_A527:				; CODE XREF: MoveMissile+59j
		set	4, (iy+20h)

loc_A52B:				; CODE XREF: MoveMissile+61j
		pop	iy
		ret
; ---------------------------------------------------------------------------

loc_A52E:				; CODE XREF: sub_9D40+1j
		ld	(hl), a
		jp	loc_9E5C
; ---------------------------------------------------------------------------

loc_A532:				; CODE XREF: MoveMissile+43j
					; MoveMissile+4Bj ...
		pop	iy
		jp	MoveObject1
; End of function MoveMissile


; =============== S U B	R O U T	I N E =======================================




; === TestVergesVisible === CODE XREF: PrintObject_2+5Ap
TestVergesVisible:			; CODE XREF: PrintObject_2+5Ap
					; MakeTablPrnCrdVertex+30p ...
		ld	b, a
		call	loc_AA20
		push	bc
		ld	b, a
		pop	af
		rrca
		rrca
		rrca
		rrca
		call	loc_AA20

loc_AA1E:				; DATA XREF: PrintObject_2+17w
					; PrintObject_2+A6w
		or	b
		ret
; ---------------------------------------------------------------------------

loc_AA20:				; CODE XREF: TestVergesVisible+1p
					; TestVergesVisible+Bp
		and	0Fh
		add	a, 84h ; 'Д'
		ld	(loc_AA27+1), a

loc_AA27:				; DATA XREF: TestVergesVisible+14w
					; RAM:D79Ew ...
		ld	a, (byte_A98E)
		ret
; End of function TestVergesVisible


; =============== S U B	R O U T	I N E =======================================




; === UpdateAllIndicators === CODE XREF: RAM:7505p
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




; === UseEnergyBomb === CODE XREF: j_UseEnergyBombj
UseEnergyBomb:				; CODE XREF: j_UseEnergyBombj
		ld	a, (ControlKey3)
		bit	5, a
		ret	z
		ld	hl, PilotEnergyBomb
		ld	a, (hl)
		and	a
		ret	z
		inc	(hl)
		xor	a
		inc	a
		ld	(EnergyBombUsed), a
		ld	hl, nBeepFX
		set	0, (hl)
		ld	c, 18h

loc_70A2:				; CODE XREF: UseEnergyBomb+1Dj
		call	sub_70A9
		dec	c
		jr	nz, loc_70A2
		ret
; End of function UseEnergyBomb


; =============== S U B	R O U T	I N E =======================================




; === UseEscapePod === CODE XREF: Process_MissBombEPod+Cp
UseEscapePod:				; CODE XREF: Process_MissBombEPod+Cp

; FUNCTION CHUNK AT 9C71 SIZE 0000000A BYTES

		ld	hl, byte_9C1B
		ld	a, (hl)
		and	a
		jr	z, Use_Q_EscPod
		dec	a
		ld	(hl), a
		jp	z, loc_71BA
		ret
; ---------------------------------------------------------------------------

Use_Q_EscPod:				; CODE XREF: UseEscapePod+5j
		ld	a, (ControlKey3)
		bit	6, a
		ret	z
		ld	a, (PilotEscapePod)
		and	a
		ret	z
		ld	a, (InFreeSpace)
		and	a
		ret	nz
		ld	a, 18h
		ld	(byte_9C1B), a
		ld	hl, 10h
		ld	(DeltaSpeed), hl
		xor	a
		ld	hl, PilotLegalStatus
		ld	(hl), a
		call	sub_848A	; hl = PilotEscapePod
		ld	l, 6Bh ; 'k'    ; hl = PilotCargo
		ld	b, 0Dh

loc_9C50:				; CODE XREF: UseEscapePod+36j
		ld	(hl), a
		inc	hl
		djnz	loc_9C50
		ld	l, 0B0h	; '░'   ; hl = PilotExtWeapon
		res	5, (hl)
		ld	l, 7Bh ; '{'    ; hl= cargoAlienItems
		ld	(hl), a
		call	SetMaxTonnage	; установка максимальной грузоподъемности
		ld	a, 17h
		call	CallFunctionObj
; ---------------------------------------------------------------------------
		jr	loc_9C71
; ---------------------------------------------------------------------------
		ld	hl, (word_E1E0)
		ld	a, (hl)
		nop
		ld	a, b
		cp	3
		dec	a
		ret	c
		ld	a, (hl)
		ret
; End of function UseEscapePod

; ---------------------------------------------------------------------------
; START	OF FUNCTION CHUNK FOR UseEscapePod

loc_9C71:				; CODE XREF: UseEscapePod+47j
		ld	(PilotDataOfSun+1), a
		ld	a, (Pilot_CrdYSystem)
		ld	(PilotDataOfSun+2), a
		ret
; END OF FUNCTION CHUNK	FOR UseEscapePod
; ---------------------------------------------------------------------------
		db    0
		db    0
		db    0
		db    0
		db    0

; =============== S U B	R O U T	I N E =======================================

; Attributes: noreturn thunk



; === Use_J_Drive === CODE XREF: sub_728D:loc_74B6p
Use_J_Drive:				; CODE XREF: sub_728D:loc_74B6p
		ld	a, (ControlKey2)
		bit	6, a
		push	af
		and	10111111b
		ld	(ControlKey2), a
		pop	af
		ret	z
		ld	a, (InFreeSpace)
		and	a
		jr	nz, loc_7EFB
		ld	hl, (J_Drive_Ready2)
		ld	a, (J_Drive_Coriolis)
		inc	a
		or	h
		or	l
		ret	nz

loc_7EFB:				; CODE XREF: Use_J_Drive+11j
		ld	a, (J_Drive_Ready4)
		and	a
		ret	nz
		ld	(Mission2), a
		ld	ix, objCoriolis
		call	ClearAroundObj

loc_7F0A:				; DATA XREF: Use_J_Drive+30w
		ld	a, 55h ; 'U'
		nop
		rrca
		ld	(loc_7F0A+1), a
		ld	a, 1
		jr	c, loc_7F18
		ld	(FrameCounter),	a

loc_7F18:				; CODE XREF: Use_J_Drive+35j
		ld	(StarsFastScroll), a
		ld	ix, SunCrd
		call	sub_7F2D
		ld	ix, PlanetCrd
		call	sub_7F2D
		ld	ix, objCoriolis
; End of function Use_J_Drive


; =============== S U B	R O U T	I N E =======================================




; === Use_Q_EscPod === CODE XREF: UseEscapePod+5j
Use_Q_EscPod:				; CODE XREF: UseEscapePod+5j
		ld	a, (ControlKey3)
		bit	6, a
		ret	z
		ld	a, (PilotEscapePod)
		and	a
		ret	z
		ld	a, (InFreeSpace)
		and	a
		ret	nz
		ld	a, 18h
		ld	(byte_9C1B), a
		ld	hl, 10h
		ld	(DeltaSpeed), hl
		xor	a
		ld	hl, PilotLegalStatus
		ld	(hl), a
		call	sub_848A	; hl = PilotEscapePod
		ld	l, 6Bh ; 'k'    ; hl = PilotCargo
		ld	b, 0Dh

loc_9C50:				; CODE XREF: UseEscapePod+36j
		ld	(hl), a
		inc	hl
		djnz	loc_9C50
		ld	l, 0B0h	; '░'   ; hl = PilotExtWeapon
		res	5, (hl)
		ld	l, 7Bh ; '{'    ; hl= cargoAlienItems
		ld	(hl), a
		call	SetMaxTonnage	; установка максимальной грузоподъемности
		ld	a, 17h
		call	CallFunctionObj
; ---------------------------------------------------------------------------
		jr	loc_9C71
; ---------------------------------------------------------------------------
		ld	hl, (word_E1E0)
		ld	a, (hl)
		nop
		ld	a, b
		cp	3
		dec	a
		ret	c
		ld	a, (hl)
		ret
; End of function UseEscapePod

; ---------------------------------------------------------------------------
; START	OF FUNCTION CHUNK FOR UseEscapePod

loc_9C71:				; CODE XREF: UseEscapePod+47j
		ld	(PilotDataOfSun+1), a
		ld	a, (Pilot_CrdYSystem)
		ld	(PilotDataOfSun+2), a
		ret
; END OF FUNCTION CHUNK	FOR UseEscapePod
; ---------------------------------------------------------------------------
		db    0
		db    0
		db    0
		db    0
		db    0

; =============== S U B	R O U T	I N E =======================================

; Attributes: noreturn thunk



; === Use_Y_Cloacking === CODE XREF: RAM:7509p
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




; === bit2A_equ_bit2HL === CODE XREF: SetBit2ExtSlot_rndA+8p
bit2A_equ_bit2HL:			; CODE XREF: SetBit2ExtSlot_rndA+8p
					; sub_7C60+88p	...
		bit	2, (hl)
		ret	nz
		set	2, a
		ret
; End of function bit2A_equ_bit2HL

; ---------------------------------------------------------------------------
		ld	de, (CrdXProjection)

; =============== S U B	R O U T	I N E =======================================


GetDirectObj:				; CODE XREF: DirectObjectp
		ld	hl, (word_B452)
		ld	de, 8
		add	hl, de
		ld	a, (hl)
		and	0Fh
		cp	1
		bit	1, (ix+21h)
		ret
; End of function GetDirectObj

; ---------------------------------------------------------------------------
		db    0
byte_B4D6:	db 0			; DATA XREF: FireLaser+114w
					; FireLaser+139r ...
byte_B4D7:	db 0			; DATA XREF: FireLaserr FireLaser+5w ...

; =============== S U B	R O U T	I N E =======================================


CollisionWithObj:			; CODE XREF: sub_9D40:loc_9E41p

; FUNCTION CHUNK AT D0E4 SIZE 00000003 BYTES
; FUNCTION CHUNK AT D183 SIZE 00000032 BYTES

		ld	a, (PlayerGameOver)
		and	a
		ret	nz
		ld	d, 0
		ld	e, (iy+1)
		bit	0, (ix+26h)
		jr	z, loc_B4F0
		ld	a, (ix+25h)
		and	a
		jr	nz, loc_B4F0
		ld	d, 1

loc_B4F0:				; CODE XREF: CollisionWithObj+Ej
					; CollisionWithObj+14j
		bit	1, (ix+26h)
		jr	z, loc_B4F8
		ld	e, 0B4h	; '┤'

loc_B4F8:				; CODE XREF: CollisionWithObj+1Cj
		ld	l, (ix+0)
		ld	h, (ix+1)
		sbc	hl, de
		ret	nc
		and	a
		ld	l, (ix+3)
		ld	h, (ix+4)
		sbc	hl, de
		ret	nc
		and	a
		ld	l, (ix+6)
		ld	h, (ix+7)
		sbc	hl, de
		ret	nc
		ld	a, (ix+8)
		add	a, a
		ret	nz
		bit	1, (ix+26h)
		jr	z, loc_B55D	; столкновение со станцией
		bit	7, (ix+5)
		jp	z, objCollision
		ld	a, (PilotFuelScoops)
		and	a
		jr	z, objCollision
		xor	a
		ld	(ix+22h), a
		call	OffTargetMissiles_2
		ld	a, (PilotFreeCargo)
		and	a
		jr	z, objCollision
		ld	a, (iy+0)
		and	0F0h ; 'Ё'
		jr	z, loc_B548
		rrca
		rrca
		rrca
		rrca
		inc	a
		jr	AddFoundCargo2
; ---------------------------------------------------------------------------

loc_B548:				; CODE XREF: CollisionWithObj+67j
		ld	a, (Mission2)
		sub	3
		jr	nz, loc_B555
		ld	(Mission2), a
		dec	a
		jr	AddFoundCargo2
; ---------------------------------------------------------------------------

loc_B555:				; CODE XREF: CollisionWithObj+75j
		call	IntRnd_A
		and	7

AddFoundCargo2:				; CODE XREF: CollisionWithObj+6Ej
					; CollisionWithObj+7Bj
		jp	AddFoundCargo
; ---------------------------------------------------------------------------

loc_B55D:				; CODE XREF: CollisionWithObj+46j
		bit	2, (ix+26h)	; столкновение со станцией
		jr	z, objCollision
		bit	7, (ix+0Eh)
		jr	z, killPlayer
		call	sub_8429
		jp	c, loc_B595
		bit	2, (ix+21h)
		jr	nz, killPlayer
		ld	a, (ix+0Ah)
		add	a, a
		cp	0DCh ; '▄'
		jr	nc, killPlayer
		ld	b, a
		ld	a, (ix+0Ch)
		add	a, a
		cp	0DCh ; '▄'
		jr	nc, killPlayer
		cp	3Ch ; '<'
		jr	nc, loc_B595
		ld	a, b
		cp	3Ch ; '<'
		jr	nc, loc_B595
		ld	a, 1
		ld	(ManualDocking), a
		ret
; ---------------------------------------------------------------------------

loc_B595:				; CODE XREF: CollisionWithObj+94j
					; CollisionWithObj+B0j	...
		ld	hl, nBeepFX
		set	6, (hl)
		ld	hl, HighDeltaSpeed
		ld	a, (DamageFrontShield)
		add	a, 0Ch
		add	a, (hl)
		add	a, (hl)
		ld	(DamageFrontShield), a
		ret
; ---------------------------------------------------------------------------

objCollision:				; CODE XREF: CollisionWithObj+4Cj
					; CollisionWithObj+53j	...
		xor	a
		call	sub_B743
		ld	b, (iy+1)
		bit	6, (ix+26h)
		jr	z, loc_B5BF
		ld	a, (Mission2)
		and	a
		jr	z, loc_B5BF
		xor	a
		ld	(Mission2), a

loc_B5BF:				; CODE XREF: CollisionWithObj+DBj
					; CollisionWithObj+E1j
		ld	hl, nBeepFX
		set	2, (hl)
		bit	7, (ix+8)
		jr	z, loc_B5D8
		ld	a, (iy+15h)
		or	b
		ld	(DamageRearShield), a
		jr	OffTargetMissiles
; ---------------------------------------------------------------------------

killPlayer:				; CODE XREF: CollisionWithObj+8Fj
					; CollisionWithObj+9Bj	...
		inc	a
		ld	(PlayerGameOver), a
		ret
; ---------------------------------------------------------------------------

loc_B5D8:				; CODE XREF: CollisionWithObj+F0j
		ld	a, (iy+15h)
		or	b
		ld	(DamageFrontShield), a
; End of function CollisionWithObj


; =============== S U B	R O U T	I N E =======================================


OffTargetMissiles:			; CODE XREF: CounterShipHyperJump+18p
					; sub_9D40+A5p	...
		ld	a, (InverseNumSlotIX)
		ld	h, a
		push	iy
		ld	iy, objCoriolis
		ld	de, 27h	; '''
		ld	b, 7

loc_B5EE:				; CODE XREF: OffTargetMissiles:loc_B600j
		add	iy, de
		ld	a, (iy+22h)
		and	a
		jr	z, loc_B600
		ld	a, (iy+25h)
		xor	h
		call	z, sub_8C29
		call	sub_8AA3

loc_B600:				; CODE XREF: OffTargetMissiles+15j
		djnz	loc_B5EE
		pop	iy
		ld	a, (Missile1)
		cp	2
		ret	nz
		ld	a, (nObjMissile)
		sub	h
		ret	nz
		ld	(Missile1), a
		ld	hl, UntargetMissile
		set	7, (hl)
		ret
; End of function OffTargetMissiles


; =============== S U B	R O U T	I N E =======================================

; Attributes: noreturn

SetShipwreck2:				; CODE XREF: SetShipwreckj
		ld	hl, nBeepFX
		set	2, (hl)
		xor	a
		ld	(WhatOnScreen),	a
		ld	ix, loc_6021
		ld	de, 27h	; '''
		ld	b, 8

loc_B62A:				; CODE XREF: SetShipwreck2+17j
		add	ix, de
		ld	(ix+22h), a
		djnz	loc_B62A
		ld	a, 16h
		call	sub_B867
; End of function SetShipwreck2

; ---------------------------------------------------------------------------
		ld	a, 15h
		call	sub_B867
; ---------------------------------------------------------------------------
		ret

; =============== S U B	R O U T	I N E =======================================


FireLaser:				; CODE XREF: RAM:B7F1p

; FUNCTION CHUNK AT B749 SIZE 00000050 BYTES

		ld	a, (byte_B4D7)
		ld	c, a
		xor	a
		ld	(byte_B4D7), a
		ld	a, (TempLaser)
		cp	2
		jr	nc, loc_B64D
		ld	a, 2

loc_B64D:				; CODE XREF: FireLaser+Dj
		sub	2
		ld	(TempLaser), a
		ld	hl, ControlKey3
		bit	0, (hl)
		ret	z
		res	0, (hl)
		cp	90h ; 'Р'
		ret	nc
		ld	b, a
		ld	a, (WhatOnScreen)
		ld	hl, PilotFrontLaser
		ld	d, 0
		ld	e, a
		add	hl, de
		ld	a, (hl)
		and	a
		ret	z
		ld	e, c
		ld	c, a
		dec	a
		jr	nz, loc_B672
		ld	d, 1

loc_B672:				; CODE XREF: FireLaser+32j
		sub	3
		jr	nz, loc_B678
		ld	d, 7

loc_B678:				; CODE XREF: FireLaser+38j
		ld	a, e
		and	e
		ld	a, 1
		ld	(byte_B4D7), a
		jr	z, loc_B686
		ld	a, (FrameCounter)
		and	d
		ret	nz

loc_B686:				; CODE XREF: FireLaser+43j
		ld	hl, DamageEnergyUnit
		inc	(hl)
		call	sub_9389
		ld	hl, nBeepFX
		set	4, (hl)
		ld	a, c
		sub	3
		ld	a, 0Fh
		jr	c, loc_B69F
		jr	z, loc_B69D
		add	a, 1Bh

loc_B69D:				; CODE XREF: FireLaser+5Dj
		add	a, 8

loc_B69F:				; CODE XREF: FireLaser+5Bj
		ld	c, a
		ld	a, d
		dec	a
		and	1
		rla
		add	a, 8
		add	a, b
		cp	0A0h ; 'а'
		jr	c, loc_B6AE
		ld	a, 0A0h	; 'а'

loc_B6AE:				; CODE XREF: FireLaser+6Ej
		ld	(TempLaser), a
		ld	a, (ObjInTarget)
		and	a
		jp	z, PrnLaser
		dec	a
		sub	9
		cpl
		ld	ix, objCoriolis
		ld	de, 27h	; '''

loc_B6C3:				; CODE XREF: FireLaser+8Cj
		dec	a
		jr	z, loc_B6CA
		add	ix, de
		jr	loc_B6C3
; ---------------------------------------------------------------------------

loc_B6CA:				; CODE XREF: FireLaser+88j
		call	sub_F9E1
		nop
		ld	hl, nBeepFX
		set	3, (hl)
		bit	2, (ix+26h)
		jr	nz, PrnLaser
		bit	5, (ix+20h)
		jr	nz, PrnLaser
		ld	a, (ix+25h)
		ld	b, a

loc_B6E3:
		sub	0Eh
		jr	nc, loc_B6EB
		and	1
		add	a, 6

loc_B6EB:				; CODE XREF: FireLaser+A9j
		rla
		sla	b
		rra
		ld	(ix+25h), a
		ld	a, (ix+22h)
		cp	0FFh
		jr	z, PrnLaser
		sub	c
		ld	(ix+22h), a
		jr	z, loc_B701
		jr	nc, PrnLaser

loc_B701:				; CODE XREF: FireLaser+C1j
		xor	a
		dec	a
		set	4, (ix+20h)
		ld	(ix+22h), a
		call	DestroyObj
; ---------------------------------------------------------------------------
		jr	PrnLaser
; End of function FireLaser

; [00000003 BYTES: COLLAPSED FUNCTION j_DestroyObj. PRESS KEYPAD "+" TO EXPAND]

; =============== S U B	R O U T	I N E =======================================


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



; === compDownScrAdr_HL === CODE XREF: sub_7FCD+Ap sub_7FCD+Ep ...
compDownScrAdr_HL:			; CODE XREF: sub_7FCD+Ap sub_7FCD+Ep ...
		ld	a, l
		add	a, 20h ; ' '
		ld	l, a
		ret	c
		ld	a, h
		sub	8
		ld	h, a
		ret
; End of function compDownScrAdr_HL


; =============== S U B	R O U T	I N E =======================================


compUpScrAdr_HL:			; CODE XREF: sub_9736+B3p
					; PrnSmallCircle+Dp
		ld	a, l
		sub	20h ; ' '
		ld	l, a
		ret	c
		ld	a, h
		add	a, 8
		ld	h, a
		ret
; End of function compUpScrAdr_HL

; ---------------------------------------------------------------------------
; START	OF FUNCTION CHUNK FOR PrnCocpit_Ifn

loc_8459:				; CODE XREF: PrnCocpit_Ifn+3Bj
		call	PrnCompass
		jp	loc_98A2
; END OF FUNCTION CHUNK	FOR PrnCocpit_Ifn
; ---------------------------------------------------------------------------
		db    1
		db 0FFh
		db  18h
		db 0EDh	; э
		db 0A0h	; а
		db 0EDh	; э
		db 0A0h	; а
		db 0EDh	; э
		db 0A0h	; а
		db  1Dh
		db  1Dh
		db  1Dh
		db 0EBh	; ы
		db  24h	; $
		db  7Ch	; |
		db 0E6h	; ц
		db    7
		db 0CCh	; ╠
		db  45h	; E
		db  84h	; Д
		db 0EBh	; ы
		db  10h
		db 0ECh	; ь
		db 0C9h	; ╔
		db    0
		db    0
		db  3Ch	; <
		db    0
		db  24h	; $
		db    0
		db  24h	; $
		db    0
		db  3Ch	; <
		db    0
; ---------------------------------------------------------------------------

loc_8481:				; CODE XREF: RAM:8487j
					; EnergyRecovery+4p
		push	af
		call	sub_9AE2
		pop	af
		inc	a
		jr	nz, loc_8481
		ret

; =============== S U B	R O U T	I N E =======================================

; hl = PilotEscapePod



; === compUpScrAdr_HL === CODE XREF: sub_9736+B3p
compUpScrAdr_HL:			; CODE XREF: sub_9736+B3p
					; PrnSmallCircle+Dp
		ld	a, l
		sub	20h ; ' '
		ld	l, a
		ret	c
		ld	a, h
		add	a, 8
		ld	h, a
		ret
; End of function compUpScrAdr_HL

; ---------------------------------------------------------------------------
; START	OF FUNCTION CHUNK FOR PrnCocpit_Ifn

loc_8459:				; CODE XREF: PrnCocpit_Ifn+3Bj
		call	PrnCompass
		jp	loc_98A2
; END OF FUNCTION CHUNK	FOR PrnCocpit_Ifn
; ---------------------------------------------------------------------------
		db    1
		db 0FFh
		db  18h
		db 0EDh	; э
		db 0A0h	; а
		db 0EDh	; э
		db 0A0h	; а
		db 0EDh	; э
		db 0A0h	; а
		db  1Dh
		db  1Dh
		db  1Dh
		db 0EBh	; ы
		db  24h	; $
		db  7Ch	; |
		db 0E6h	; ц
		db    7
		db 0CCh	; ╠
		db  45h	; E
		db  84h	; Д
		db 0EBh	; ы
		db  10h
		db 0ECh	; ь
		db 0C9h	; ╔
		db    0
		db    0
		db  3Ch	; <
		db    0
		db  24h	; $
		db    0
		db  24h	; $
		db    0
		db  3Ch	; <
		db    0
; ---------------------------------------------------------------------------

loc_8481:				; CODE XREF: RAM:8487j
					; EnergyRecovery+4p
		push	af
		call	sub_9AE2
		pop	af
		inc	a
		jr	nz, loc_8481
		ret

; =============== S U B	R O U T	I N E =======================================

; hl = PilotEscapePod



; === j_IncBattleRating2 === CODE XREF: IncBattleRating+9j
j_IncBattleRating2:			; CODE XREF: IncBattleRating+9j
					; MoveMissile+6Bp
		jp	IncBattleRating2
; End of function j_IncBattleRating2

; ---------------------------------------------------------------------------
; START	OF FUNCTION CHUNK FOR GoMission3

EndMission3:				; CODE XREF: GoMission3+Cj
		jp	sub_D14D
; END OF FUNCTION CHUNK	FOR GoMission3
; ---------------------------------------------------------------------------
		db    0
		db    0
		db    0

; =============== S U B	R O U T	I N E =======================================

; Attributes: thunk



; === j_SetAtrViewScr === CODE XREF: Use_Y_Cloacking:loc_7259j
j_SetAtrViewScr:			; CODE XREF: Use_Y_Cloacking:loc_7259j
					; RAM:7526p ...
		jp	SetAtrViewScr
; End of function j_SetAtrViewScr


; =============== S U B	R O U T	I N E =======================================




; === j_Sqrt_H_HL === CODE XREF: CalcLenVectorCrd:loc_8377p
j_Sqrt_H_HL:				; CODE XREF: CalcLenVectorCrd:loc_8377p
					; CalcLenVectorCrd:loc_8386p ...
		jp	Sqrt_H_HL
; End of function j_Sqrt_H_HL


; =============== S U B	R O U T	I N E =======================================

; Attributes: noreturn thunk

Div_BC_HLAdivDE:			; CODE XREF: CompasCrdXplanet:loc_82BBp
					; sub_96DC+57p	...
		jp	Div_BC_HLAdivDE_1
; End of function Div_BC_HLAdivDE

; ---------------------------------------------------------------------------
		db 0C3h	; ├
		db  44h	; D
		db 0B0h	; ░
		db 0C3h	; ├
		db 0B4h	; ┤
		db 0B7h	; ╖
		db    0


; === j_UseEnergyBomb === CODE XREF: Process_MissBombEPod+Fp
j_UseEnergyBomb:			; CODE XREF: Process_MissBombEPod+Fp
		jp	UseEnergyBomb
; End of function j_UseEnergyBomb

; ---------------------------------------------------------------------------
		jp	sub_70A9

; =============== S U B	R O U T	I N E =======================================

; Attributes: thunk



; === killPlayer === CODE XREF: CollisionWithObj+8Fj
killPlayer:				; CODE XREF: CollisionWithObj+8Fj
					; CollisionWithObj+9Bj	...
		inc	a
		ld	(PlayerGameOver), a
		ret
; ---------------------------------------------------------------------------

loc_B5D8:				; CODE XREF: CollisionWithObj+F0j
		ld	a, (iy+15h)
		or	b
		ld	(DamageFrontShield), a
; End of function CollisionWithObj


; =============== S U B	R O U T	I N E =======================================


OffTargetMissiles:			; CODE XREF: CounterShipHyperJump+18p
					; sub_9D40+A5p	...
		ld	a, (InverseNumSlotIX)
		ld	h, a
		push	iy
		ld	iy, objCoriolis
		ld	de, 27h	; '''
		ld	b, 7

loc_B5EE:				; CODE XREF: OffTargetMissiles:loc_B600j
		add	iy, de
		ld	a, (iy+22h)
		and	a
		jr	z, loc_B600
		ld	a, (iy+25h)
		xor	h
		call	z, sub_8C29
		call	sub_8AA3

loc_B600:				; CODE XREF: OffTargetMissiles+15j
		djnz	loc_B5EE
		pop	iy
		ld	a, (Missile1)
		cp	2
		ret	nz
		ld	a, (nObjMissile)
		sub	h
		ret	nz
		ld	(Missile1), a
		ld	hl, UntargetMissile
		set	7, (hl)
		ret
; End of function OffTargetMissiles


; =============== S U B	R O U T	I N E =======================================

; Attributes: noreturn

SetShipwreck2:				; CODE XREF: SetShipwreckj
		ld	hl, nBeepFX
		set	2, (hl)
		xor	a
		ld	(WhatOnScreen),	a
		ld	ix, loc_6021
		ld	de, 27h	; '''
		ld	b, 8

loc_B62A:				; CODE XREF: SetShipwreck2+17j
		add	ix, de
		ld	(ix+22h), a
		djnz	loc_B62A
		ld	a, 16h
		call	sub_B867
; End of function SetShipwreck2

; ---------------------------------------------------------------------------
		ld	a, 15h
		call	sub_B867
; ---------------------------------------------------------------------------
		ret

; =============== S U B	R O U T	I N E =======================================


FireLaser:				; CODE XREF: RAM:B7F1p

; FUNCTION CHUNK AT B749 SIZE 00000050 BYTES

		ld	a, (byte_B4D7)
		ld	c, a
		xor	a
		ld	(byte_B4D7), a
		ld	a, (TempLaser)
		cp	2
		jr	nc, loc_B64D
		ld	a, 2

loc_B64D:				; CODE XREF: FireLaser+Dj
		sub	2
		ld	(TempLaser), a
		ld	hl, ControlKey3
		bit	0, (hl)
		ret	z
		res	0, (hl)
		cp	90h ; 'Р'
		ret	nc
		ld	b, a
		ld	a, (WhatOnScreen)
		ld	hl, PilotFrontLaser
		ld	d, 0
		ld	e, a
		add	hl, de
		ld	a, (hl)
		and	a
		ret	z
		ld	e, c
		ld	c, a
		dec	a
		jr	nz, loc_B672
		ld	d, 1

loc_B672:				; CODE XREF: FireLaser+32j
		sub	3
		jr	nz, loc_B678
		ld	d, 7

loc_B678:				; CODE XREF: FireLaser+38j
		ld	a, e
		and	e
		ld	a, 1
		ld	(byte_B4D7), a
		jr	z, loc_B686
		ld	a, (FrameCounter)
		and	d
		ret	nz

loc_B686:				; CODE XREF: FireLaser+43j
		ld	hl, DamageEnergyUnit
		inc	(hl)
		call	sub_9389
		ld	hl, nBeepFX
		set	4, (hl)
		ld	a, c
		sub	3
		ld	a, 0Fh
		jr	c, loc_B69F
		jr	z, loc_B69D
		add	a, 1Bh

loc_B69D:				; CODE XREF: FireLaser+5Dj
		add	a, 8

loc_B69F:				; CODE XREF: FireLaser+5Bj
		ld	c, a
		ld	a, d
		dec	a
		and	1
		rla
		add	a, 8
		add	a, b
		cp	0A0h ; 'а'
		jr	c, loc_B6AE
		ld	a, 0A0h	; 'а'

loc_B6AE:				; CODE XREF: FireLaser+6Ej
		ld	(TempLaser), a
		ld	a, (ObjInTarget)
		and	a
		jp	z, PrnLaser
		dec	a
		sub	9
		cpl
		ld	ix, objCoriolis
		ld	de, 27h	; '''

loc_B6C3:				; CODE XREF: FireLaser+8Cj
		dec	a
		jr	z, loc_B6CA
		add	ix, de
		jr	loc_B6C3
; ---------------------------------------------------------------------------

loc_B6CA:				; CODE XREF: FireLaser+88j
		call	sub_F9E1
		nop
		ld	hl, nBeepFX
		set	3, (hl)
		bit	2, (ix+26h)
		jr	nz, PrnLaser
		bit	5, (ix+20h)
		jr	nz, PrnLaser
		ld	a, (ix+25h)
		ld	b, a

loc_B6E3:
		sub	0Eh
		jr	nc, loc_B6EB
		and	1
		add	a, 6

loc_B6EB:				; CODE XREF: FireLaser+A9j
		rla
		sla	b
		rra
		ld	(ix+25h), a
		ld	a, (ix+22h)
		cp	0FFh
		jr	z, PrnLaser
		sub	c
		ld	(ix+22h), a
		jr	z, loc_B701
		jr	nc, PrnLaser

loc_B701:				; CODE XREF: FireLaser+C1j
		xor	a
		dec	a
		set	4, (ix+20h)
		ld	(ix+22h), a
		call	DestroyObj
; ---------------------------------------------------------------------------
		jr	PrnLaser
; End of function FireLaser

; [00000003 BYTES: COLLAPSED FUNCTION j_DestroyObj. PRESS KEYPAD "+" TO EXPAND]

; =============== S U B	R O U T	I N E =======================================


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



; === locret_82D0 === CODE XREF: CompasCrdXplanet+1Cj
locret_82D0:				; CODE XREF: CompasCrdXplanet+1Cj
		ret
; End of function CompasCrdXplanet

; ---------------------------------------------------------------------------
; START	OF FUNCTION CHUNK FOR PrintCompass

loc_82D1:				; CODE XREF: PrintCompass+36j
					; PrintCompass+90j
		ld	a, 1
		ld	(PlayerGameOver), a
		ret
; END OF FUNCTION CHUNK	FOR PrintCompass

; =============== S U B	R O U T	I N E =======================================


CalcTempAltitude:			; CODE XREF: PrintCompass+47p
					; PrintCompass+94p
		push	hl
		add	hl, hl
		pop	de
		srl	d
		rr	e
		add	hl, de
		ld	a, (IndDanger)
		ld	b, a
		ld	a, h
		cp	50h ; 'P'
		jr	c, loc_82F8
		cp	78h ; 'x'
		jr	nc, loc_82F4
		ld	a, b
		cp	63h ; 'c'
		ret	z
		ld	a, 2
		jr	loc_82FE
; ---------------------------------------------------------------------------

loc_82F4:				; CODE XREF: CalcTempAltitude+13j
		ld	a, 63h ; 'c'
		jr	loc_82FE
; ---------------------------------------------------------------------------

loc_82F8:				; CODE XREF: CalcTempAltitude+Fj
		ld	a, b
		cp	4
		ret	nz
		ld	a, 6

loc_82FE:				; CODE XREF: CalcTempAltitude+1Bj
					; CalcTempAltitude+1Fj
		ld	(IndDanger), a
		ret
; End of function CalcTempAltitude


; =============== S U B	R O U T	I N E =======================================


GetCrdObject:				; CODE XREF: CalcLenVectorCrdHp
					; CalcLenVectorCrdp
		ld	e, (ix+0)
		ld	d, (ix+1)
		ld	c, (ix+3)
		ld	b, (ix+4)
		ld	l, (ix+6)
		ld	h, (ix+7)
		ret
; End of function GetCrdObject


; =============== S U B	R O U T	I N E =======================================

; Attributes: noreturn

CalcLenVectorCrdH:			; CODE XREF: PrintCompass+D7p
		call	GetCrdObject
		res	7, h
		res	7, d
		res	7, b
		ld	a, 1
		jr	loc_8327
; End of function CalcLenVectorCrdH


; =============== S U B	R O U T	I N E =======================================

; Attributes: noreturn

CalcLenVectorCrd:			; CODE XREF: PrintCompass+1Dp
					; PrintCompass+7Cp ...

; FUNCTION CHUNK AT 838F SIZE 00000001 BYTES

		call	GetCrdObject
		ld	a, 1

loc_8327:				; CODE XREF: CalcLenVectorCrdH+Bj
					; CalcLenVectorCrd+14j
		ex	de, hl
		add	hl, hl
		ex	de, hl
		jr	c, loc_8338
		sla	c
		rl	b
		jr	c, loc_833C
		add	hl, hl
		jr	c, loc_8342
		add	a, a
		jr	nc, loc_8327

loc_8338:				; CODE XREF: CalcLenVectorCrd+8j
		rr	d
		jr	loc_8348
; ---------------------------------------------------------------------------

loc_833C:				; CODE XREF: CalcLenVectorCrd+Ej
		rr	b
		srl	d
		jr	loc_8348
; ---------------------------------------------------------------------------

loc_8342:				; CODE XREF: CalcLenVectorCrd+11j
		rr	h
		srl	b
		srl	d

loc_8348:				; CODE XREF: CalcLenVectorCrd+18j
					; CalcLenVectorCrd+1Ej
		ld	c, d
		push	af
		ld	a, h
		call	Sqr_DE_A
		ex	de, hl
		ld	a, b
		ld	b, 0
		call	Sqr_DE_A
		add	hl, de
		jr	nc, loc_8359
		inc	b

loc_8359:				; CODE XREF: CalcLenVectorCrd+34j
		ld	a, c
		call	Sqr_DE_A
		add	hl, de
		jr	nc, loc_8361
		inc	b

loc_8361:				; CODE XREF: CalcLenVectorCrd+3Cj
		ld	a, b
		and	a
		jr	z, loc_8377
		srl	b
		rr	h
		rr	l
		srl	b
		rr	h
		rr	l
		pop	af
		srl	a
		jr	z, loc_8386
		push	af

loc_8377:				; CODE XREF: CalcLenVectorCrd+41j
		call	j_Sqrt_H_HL
; ---------------------------------------------------------------------------
		pop	af

loc_837B:				; CODE XREF: CalcLenVectorCrd+61j
		srl	a
		jr	z, locret_838F
		srl	h
		rr	l
		jp	loc_837B
; ---------------------------------------------------------------------------

loc_8386:				; CODE XREF: CalcLenVectorCrd+52j
		call	j_Sqrt_H_HL
; End of function CalcLenVectorCrd

; ---------------------------------------------------------------------------
		add	hl, hl
		jr	nc, locret_838F
		ld	hl, 0
; START	OF FUNCTION CHUNK FOR CalcLenVectorCrd

locret_838F:				; CODE XREF: CalcLenVectorCrd+5Bj
					; RAM:838Aj
		ret
; END OF FUNCTION CHUNK	FOR CalcLenVectorCrd
; ---------------------------------------------------------------------------
; START	OF FUNCTION CHUNK FOR PrintCompass

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



; === locret_838F === CODE XREF: CalcLenVectorCrd+5Bj
locret_838F:				; CODE XREF: CalcLenVectorCrd+5Bj
					; RAM:838Aj
		ret
; END OF FUNCTION CHUNK	FOR CalcLenVectorCrd
; ---------------------------------------------------------------------------
; START	OF FUNCTION CHUNK FOR PrintCompass

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



; === locret_A1E9 === CODE XREF: CalcNewCrdObj_Pl+51j
locret_A1E9:				; CODE XREF: CalcNewCrdObj_Pl+51j
					; CalcNewCrdObj_Pl+61j
		ret
; End of function CalcNewCrdObj_Pl


; =============== S U B	R O U T	I N E =======================================


CalcNewCrdNobj:				; CODE XREF: CalcNewCrdObj_Pl+Cp
					; CalcNewCrdObj_Pl+1Cp	...
		push	ix
		push	ix
		ex	(sp), iy
		add	ix, de
		ex	de, hl
		add	iy, de
		ld	a, (ix+2)
		ld	d, (ix+1)
		ld	e, (ix+0)
		ld	b, a
		xor	c
		and	80h ; 'А'
		push	af
		res	7, c
		res	7, b
		push	de
		call	Mult_HL_B_C
		pop	de
		push	hl
		ld	b, d
		push	de
		call	Mult_HL_B_C
		pop	de
		ld	b, e
		ld	a, l
		ld	l, h
		ld	h, 0
		pop	de
		add	hl, de
		ld	e, a
		push	hl
		call	Mult_A_hiB_C
		add	a, e
		pop	hl
		jr	nc, loc_A224
		inc	hl

loc_A224:				; CODE XREF: CalcNewCrdNobj+37j
		ld	b, a
		ld	a, h
		ld	h, l
		ld	l, b
		pop	bc
		ld	c, b
		ld	b, (iy+2)
		ld	d, (iy+1)
		ld	e, (iy+0)
		push	af
		ld	a, b
		xor	c
		rlc	c
		rlc	c
		rla
		rr	c
		rrc	c
		srl	a
		ld	b, a
		pop	af
		bit	6, c
		jr	nz, loc_A252
		add	a, a
		sla	c
		rra
		add	a, b
		add	hl, de
		jr	nc, loc_A273
		inc	a
		jr	loc_A273
; ---------------------------------------------------------------------------

loc_A252:				; CODE XREF: CalcNewCrdNobj+5Bj
		and	a
		ex	de, hl
		sbc	hl, de
		jr	nc, loc_A259
		inc	a

loc_A259:				; CODE XREF: CalcNewCrdNobj+6Cj
		ld	d, a
		ld	a, b
		sub	d
		ld	b, a
		jr	nc, loc_A26C
		cpl
		ld	b, a
		ld	a, c
		xor	80h ; 'А'
		ld	c, a
		ex	de, hl
		and	a
		ld	hl, 0
		sbc	hl, de

loc_A26C:				; CODE XREF: CalcNewCrdNobj+73j
		ld	a, b
		rla
		sla	c
		rra
		xor	80h ; 'А'

loc_A273:				; CODE XREF: CalcNewCrdNobj+63j
					; CalcNewCrdNobj+66j
		ld	(iy+2),	a
		ld	(iy+1),	h
		ld	(iy+0),	l
		pop	iy
		pop	ix
		ret
; End of function CalcNewCrdNobj


; =============== S U B	R O U T	I N E =======================================


Mult_Ax2_sign:				; CODE XREF: CalcNewCrdObj_Pl+14p
					; CalcNewCrdObj_Pl+22p
		ld	c, a
		rl	c
		rra
		rl	c
		rla
		rr	c
		ret
; End of function Mult_Ax2_sign

; ---------------------------------------------------------------------------
		db    0
		db    0

; =============== S U B	R O U T	I N E =======================================




; === locret_D55C === CODE XREF: RAM:D8C0j
locret_D55C:				; CODE XREF: RAM:D8C0j
		ret
; ---------------------------------------------------------------------------
		db    0
		db    0

; =============== S U B	R O U T	I N E =======================================


PrintHorLine:				; CODE XREF: ClsAndPrintRamk+2Dp
					; ClsAndPrintRamk+4Bp ...
		ld	a, 0FFh
		ld	b, 20h ; ' '
; End of function PrintHorLine


; =============== S U B	R O U T	I N E =======================================


Fill_HL_A_Len_B:			; CODE XREF: FillAttrLineFrom_HL+2p
					; Fill_HL_A_Len_B+2j ...
		ld	(hl), a
		inc	hl
		djnz	Fill_HL_A_Len_B
		ret
; End of function Fill_HL_A_Len_B


; =============== S U B	R O U T	I N E =======================================


SetAttrFrom_DE:				; CODE XREF: RAM:E719p	sub_E87E+61p ...
		ld	hl, 5841h

loc_D56B:				; CODE XREF: SetAttrFrom_DE+22j
		ld	a, (de)
		inc	de
		push	af
		and	0F8h ; '°'
		rrca
		rrca
		rrca
		ld	b, a
		push	hl
		ld	a, 5
		call	Fill_HL_A_Len_B
		pop	hl
		pop	af
		and	7
		jr	z, loc_D58C
		ld	b, a

loc_D581:				; CODE XREF: SetAttrFrom_DE:loc_D588j
		ld	a, 20h ; ' '
		add	a, l
		ld	l, a
		jr	nc, loc_D588
		inc	h

loc_D588:				; CODE XREF: SetAttrFrom_DE+1Dj
		djnz	loc_D581
		jr	loc_D56B
; ---------------------------------------------------------------------------

loc_D58C:				; CODE XREF: SetAttrFrom_DE+16j
		ld	hl, 5822h
		ld	a, 7
		ld	(hl), a
		inc	hl
		ld	(hl), a
		ret
; End of function SetAttrFrom_DE

; ---------------------------------------------------------------------------
; START	OF FUNCTION CHUNK FOR sub_D470

loc_D595:				; CODE XREF: sub_D470+1Bj
		sub	0D7h ; '╫'
		ld	hl, unk_FD99
		jr	loc_D5A1
; END OF FUNCTION CHUNK	FOR sub_D470
; ---------------------------------------------------------------------------
; START	OF FUNCTION CHUNK FOR PrintMessage1_A

loc_D59C:				; CODE XREF: PrintMessage1_A+15j
		sub	80h ; 'А'
		ld	hl, byte_FDB3

loc_D5A1:				; CODE XREF: sub_D470+12Aj
		add	a, a
		ld	d, 0
		ld	e, a
		add	hl, de
		ld	a, (hl)
		push	hl
		call	sub_D499
		pop	hl
		inc	hl
		ld	a, (hl)
		cp	3Fh ; '?'
		ret	z
		jp	sub_D499
; END OF FUNCTION CHUNK	FOR PrintMessage1_A

; =============== S U B	R O U T	I N E =======================================

; гибель минеральной жизни при недостаточном количестве
; Вых: z=1 есть	минеральная жизнь в трюме
;	=0 минеральная жизнь померла
;	   нет минеральной жизни



; === locret_DDEB === CODE XREF: sub_DD60+6Bj sub_DD60+6Ej
locret_DDEB:				; CODE XREF: sub_DD60+6Bj sub_DD60+6Ej
		ret
; End of function sub_DD60


; =============== S U B	R O U T	I N E =======================================




; === rndSet2_4Krait === CODE XREF: sub_BBF2+8j
rndSet2_4Krait:				; CODE XREF: sub_BBF2+8j
					; CallFunctionObj_1+41Cj
		call	IntRnd_A
		and	3
		rra
		adc	a, 2
		ld	c, a

loc_BCB4:				; CODE XREF: CallFunctionObj_1+45Bj
		call	FindSlot4Obj
		ret	nz
		push	bc
		call	IntRnd_A
		and	1
		add	a, 10h
		call	SetObjInSlot	; a - номер объекта для	установки
		pop	bc
		dec	c
		jr	nz, loc_BCB4
		ret
; END OF FUNCTION CHUNK	FOR CallFunctionObj_1
; ---------------------------------------------------------------------------


; === rndSetAstViper === CODE XREF: CallFunctionObj_1+42Ej
rndSetAstViper:				; CODE XREF: CallFunctionObj_1+42Ej
		ld	a, c
		rlca
		and	1
		add	a, 7
		jr	loc_BC64
; ---------------------------------------------------------------------------

rndSet2_4Krait:				; CODE XREF: sub_BBF2+8j
					; CallFunctionObj_1+41Cj
		call	IntRnd_A
		and	3
		rra
		adc	a, 2
		ld	c, a

loc_BCB4:				; CODE XREF: CallFunctionObj_1+45Bj
		call	FindSlot4Obj
		ret	nz
		push	bc
		call	IntRnd_A
		and	1
		add	a, 10h
		call	SetObjInSlot	; a - номер объекта для	установки
		pop	bc
		dec	c
		jr	nz, loc_BCB4
		ret
; END OF FUNCTION CHUNK	FOR CallFunctionObj_1
; ---------------------------------------------------------------------------


; === rndSetShip === CODE XREF: CallFunctionObj_1+3C3j
rndSetShip:				; CODE XREF: CallFunctionObj_1+3C3j
					; CallFunctionObj_1+3CAj
		call	FindSlot4Obj	; если находимся у станции таргонов, то	запуск таргона
		ret	nz
		ld	a, c
		jr	loc_BC64
; ---------------------------------------------------------------------------

loc_BC7F:				; CODE XREF: CallFunctionObj_1+3D9j
		call	IntRnd_A
		ld	b, a
		and	7
		cp	e
		jr	nc, rndSet2_4Krait
		rlc	b
		rlc	b
		rlc	b
		ld	c, b
		call	FindSlot4Obj
		ret	nz
		ld	a, c
		and	7
		cp	2
		jr	c, rndSetAstViper
		dec	a
		srl	a
		srl	a
		add	a, 9		; cobra1/python1
		jr	loc_BC64
; ---------------------------------------------------------------------------

rndSetAstViper:				; CODE XREF: CallFunctionObj_1+42Ej
		ld	a, c
		rlca
		and	1
		add	a, 7
		jr	loc_BC64
; ---------------------------------------------------------------------------

rndSet2_4Krait:				; CODE XREF: sub_BBF2+8j
					; CallFunctionObj_1+41Cj
		call	IntRnd_A
		and	3
		rra
		adc	a, 2
		ld	c, a

loc_BCB4:				; CODE XREF: CallFunctionObj_1+45Bj
		call	FindSlot4Obj
		ret	nz
		push	bc
		call	IntRnd_A
		and	1
		add	a, 10h
		call	SetObjInSlot	; a - номер объекта для	установки
		pop	bc
		dec	c
		jr	nz, loc_BCB4
		ret
; END OF FUNCTION CHUNK	FOR CallFunctionObj_1
; ---------------------------------------------------------------------------


; === sub_5D70 === CODE XREF: PrnCountdownJump:loc_DC5Fp
sub_5D70:				; CODE XREF: PrnCountdownJump:loc_DC5Fp
		ld	hl, PilotGyperdrive
		ld	a, (hl)
		and	a
		ret	z
		inc	(hl)
		ret
; End of function sub_5D70

; ---------------------------------------------------------------------------
		db 7Fh
		db  40h	; @
		db  40h	; @
		db  20h
		db  20h
		db  10h
		db  0Ch
		db    3
		db 0FEh	; ■
		db    2
		db    2
		db    4
		db    4
		db    8
		db  30h	; 0
		db 0C0h	; └
		db  7Fh	; 
		db  7Fh	; 
		db  7Fh	; 
		db  3Fh	; ?
		db  3Fh	; ?
		db  1Fh
		db  0Fh
		db    3
		db 0FEh	; ■
		db 0FEh	; ■
		db 0FEh	; ■
		db 0FCh	; №
		db 0FCh	; №
		db 0F8h	; °
		db 0F0h	; Ё
		db 0C0h	; └
		db    0
		db  7Eh	; ~
		db  42h	; B
		db  42h	; B
		db  42h	; B
		db  7Eh	; ~
		db    0
		db 0FFh
		db    0
		db 0C0h	; └
		db 0F0h	; Ё
		db 0F8h	; °
		db 0FCh	; №
		db 0FCh	; №
		db 0FEh	; ■
		db 0FEh	; ■
		db    0
		db    8
		db  10h
		db    0
		db    0
		db    0
		db    0
		db    0
		db    0
		db  10h
		db  20h
		db  20h
		db  20h
		db  20h
		db  10h
		db    0
		db    0
		db    8
		db    4
		db    4
		db    4
		db    4
		db    8
		db    0
		db    0
		db  10h
		db  10h
		db  10h
		db  10h
		db    0
		db  10h
		db    0

; =============== S U B	R O U T	I N E =======================================


CalcCrdCoriolis2:			; CODE XREF: sub_728D+226p

; FUNCTION CHUNK AT 7861 SIZE 00000037 BYTES

		ld	a, (InFreeSpace)
		and	a
		ret	nz
		jp	CalcCrdCoriolis
; End of function CalcCrdCoriolis2

; ---------------------------------------------------------------------------
		db    0
		db    0
		db    0
		db    0
		db    0
		db    8
		db    8
		db  10h
		db    0
		db    0
		db    0
		db    0
		db  3Eh	; >
		db    0
		db    0
		db    0
		db    0
		db    0
		db    0
		db    0
		db    0
		db  18h
		db  18h
		db    0
		db    0
		db    0
		db    2
		db    4
		db    8
		db  10h
		db  20h
		db    0
; ---------------------------------------------------------------------------
; START	OF FUNCTION CHUNK FOR sub_D499

loc_5DF0:				; CODE XREF: sub_D499+AAj
		pop	de
		ld	b, 8
		nop
		nop
		call	sub_D546
		nop
		nop
		ret
; END OF FUNCTION CHUNK	FOR sub_D499

; =============== S U B	R O U T	I N E =======================================




; === sub_5DFB === CODE XREF: sub_D499+59p
sub_5DFB:				; CODE XREF: sub_D499+59p
		ld	hl, 3D00h

loc_5DFE:
		cp	20h ; ' '
		ret	z
		cp	30h ; '0'
		jr	c, loc_5E16
		cp	3Bh ; ';'
		ret	c
		cp	41h ; 'A'
		jr	c, loc_5E16
		cp	5Bh ; '['
		ret	c
		cp	61h ; 'a'
		jr	c, loc_5E16
		cp	7Bh ; '{'
		ret	c

loc_5E16:				; CODE XREF: sub_5DFB+8j sub_5DFB+Fj ...
		ld	hl, 5D70h
		ret
; End of function sub_5DFB


; =============== S U B	R O U T	I N E =======================================




; === sub_5E1A === CODE XREF: RAM:E278p
sub_5E1A:				; CODE XREF: RAM:E278p
		push	af
		sub	0DCh ; '▄'
		call	z, sub_D5B4	; гибель минеральной жизни при недостаточном количестве
					; Вых: z=1 есть	минеральная жизнь в трюме
					;	=0 минеральная жизнь померла
					;	   нет минеральной жизни
		jr	z, loc_5E24
		ld	a, 0C9h	; '╔'

loc_5E24:				; CODE XREF: sub_5E1A+6j
		ld	(loc_7013), a
		ld	(loc_DE6A), a
		jr	z, loc_5E30
		pop	af
		jp	PrintMessage1_A
; ---------------------------------------------------------------------------

loc_5E30:				; CODE XREF: sub_5E1A+10j
		pop	af
		ld	a, 52h ; 'R'
		jp	PrintMessage2
; End of function sub_5E1A


; =============== S U B	R O U T	I N E =======================================




; === sub_5E36 === CODE XREF: sub_86A1:loc_8B25p
sub_5E36:				; CODE XREF: sub_86A1:loc_8B25p
					; sub_8672:loc_8C24p ...
		ld	l, d
		add	a, a
		jr	z, loc_5E50
		push	bc
		push	de
		push	hl
		ld	c, a
		ld	b, 0
		ld	a, (DataFillCircle)
		dec	hl
		ld	(hl), a
		dec	c
		jr	z, loc_5E4D
		ld	d, h
		ld	e, l
		dec	de
		lddr

loc_5E4D:				; CODE XREF: sub_5E36+10j
		pop	hl
		pop	de
		pop	bc

loc_5E50:				; CODE XREF: sub_5E36+2j
		ld	a, (DirPrnCircle)
		and	a
		ret
; End of function sub_5E36

; ---------------------------------------------------------------------------
; START	OF FUNCTION CHUNK FOR LaunchDocking

loc_5E55:				; CODE XREF: LaunchDocking+6Dj
		ld	hl, DemoMode	; =#00 при стыковке со станцией	таргонов
					;      после использования escape pod
					; =#01 включен демо режим
					; =#E0 после прыжка
		inc	(hl)
		dec	(hl)
		ld	(hl), 0
		jr	z, loc_5E65
		ld	hl, (word_E07B)
		inc	hl
		ld	(word_E07B), hl

loc_5E65:				; CODE XREF: LaunchDocking-3591j
		jp	sub_813E
; END OF FUNCTION CHUNK	FOR LaunchDocking
; ---------------------------------------------------------------------------
		db    0
		db  7Eh	; ~
		db  42h	; B
		db    6
		db    8
		db    0
		db    8
		db    0
		db    0
		db    0
		db    0
		db    0
		db    0
		db    0
		db    0
		db 0FFh

; =============== S U B	R O U T	I N E =======================================

; вызывается при увеличении 3го	байта рейтинга



; === sub_5E78 === CODE XREF: sub_B24E:loc_B258p
sub_5E78:				; CODE XREF: sub_B24E:loc_B258p
		ld	a, (PilotTargonWar) ; =#81 при сбросе миссий
					; 7,=1 начало войны, предолжение участвовать
					; 6-0, чем больше число	тем дальше от левого края карты	захват планет таргонами
					;
					; =#01 война закончена
		and	0FEh ; '■'
		jr	nz, loc_5EBE	; война	идёт
		nop
		nop
		ld	a, (PilotMission1)
		ld	l, 0AFh	; 'п'   ; hl,PilotMission3
		or	(hl)
		jr	nz, loc_5EBB	; hl,RatingRightOn+1 номер сообщения
		ld	l, 86h ; 'Ж'    ; hl,byte_D086
		ld	a, (hl)
		inc	hl
		or	(hl)
		jr	z, loc_5EB7	; hl,JumpCounter+1
		ld	a, (PilotTargonWar) ; =#81 при сбросе миссий
					; 7,=1 начало войны, предолжение участвовать
					; 6-0, чем больше число	тем дальше от левого края карты	захват планет таргонами
					;
					; =#01 война закончена
		and	a
		jr	z, loc_5E9C
		ld	l, 64h ; 'd'    ; hl,JumpCounter+1
		set	7, (hl)
		jr	loc_5EBE	; hl,PilotRating2
; ---------------------------------------------------------------------------

loc_5E9C:				; CODE XREF: sub_5E78+1Cj
		ld	a, (PilotRating2) ; >=#84 в трюме десант
					; 7,=1 война была или идёт
					; 6,=1 война закончилать
					; 3-1 -	ранг (=текущая стадия войны)
		cp	20h ; ' '
		jr	c, loc_5EB7	; hl,JumpCounter+1
		ld	l, 64h ; 'd'    ; hl,JumpCounter+1
		and	(hl)
		jr	z, loc_5EB7	; hl,JumpCounter+1
		ld	l, 0B0h	; '░'   ; hl = PilotExtWeapon
		ld	a, (hl)		; сброс	миссий
		and	h		; h=%11010000
		ld	(hl), a
		ld	a, 81h ; 'Б'
		ld	(PilotRating2),	a ; >=#84 в трюме десант
					; 7,=1 война была или идёт
					; 6,=1 война закончилать
					; 3-1 -	ранг (=текущая стадия войны)
		ld	(PilotTargonWar), a ; =#81 при сбросе миссий
					; 7,=1 начало войны, предолжение участвовать
					; 6-0, чем больше число	тем дальше от левого края карты	захват планет таргонами
					;
					; =#01 война закончена
		jr	loc_5EBB	; hl,RatingRightOn+1 номер сообщения
; ---------------------------------------------------------------------------

loc_5EB7:				; CODE XREF: sub_5E78+16j sub_5E78+29j ...
		ld	l, 64h ; 'd'    ; hl,JumpCounter+1
		set	7, (hl)

loc_5EBB:				; CODE XREF: sub_5E78+Fj sub_5E78+3Dj	...
		ld	l, 0C9h	; '╔'   ; hl,RatingRightOn+1 номер сообщения
		ret
; ---------------------------------------------------------------------------

loc_5EBE:				; CODE XREF: sub_5E78+5j sub_5E78+22j
		bit	0, (hl)		; hl,PilotRating2
		jr	nz, loc_5EBB	; hl,RatingRightOn+1 номер сообщения
		dec	(hl)
		jr	loc_5EBB	; hl,RatingRightOn+1 номер сообщения
; End of function sub_5E78


; =============== S U B	R O U T	I N E =======================================

; запуск ракеты	игроком
; запуск спасательной капсулы (установка кобры)



; === sub_5EC5 === CODE XREF: SetPlayerObjSlotp
sub_5EC5:				; CODE XREF: SetPlayerObjSlotp
		call	SetObjInSlot	; a - номер объекта для	установки
		ld	hl, PilotMissiles
		ld	a, (PilotTargonWar) ; =#81 при сбросе миссий
					; 7,=1 начало войны, предолжение участвовать
					; 6-0, чем больше число	тем дальше от левого края карты	захват планет таргонами
					;
					; =#01 война закончена
		and	0FEh ; '■'
		jr	z, loc_5EEC
		ld	a, (PilotExtWeapon) ; 7,=1 - ECM Jammer
					; 6,=1 - Cloaking device
					; 5,=1 - нельзя	купить товары(в	трюме беженцы или десант)
					; 4
					; 3 =1 3я миссия была выполнена
					; 2 =1 - третья	миссия выполнена
					; 1 =1 - второя	миссия выполнена
					; 0 =1 - первая	миссия выполнена
		and	20h ; ' '
		jr	z, loc_5EEC
		ld	a, (PilotRating2) ; >=#84 в трюме десант
					; 7,=1 война была или идёт
					; 6,=1 война закончилать
					; 3-1 -	ранг (=текущая стадия войны)
		cp	84h ; 'Д'
		jr	c, loc_5EEC
		call	SetMaxTonnage	; установка максимальной грузоподъемности
		ld	l, 0B0h	; '░'   ; hl = PilotExtWeapon
		res	5, (hl)
		ld	hl, dSidewinderH
		jr	loc_5F09
; ---------------------------------------------------------------------------

loc_5EEC:				; CODE XREF: sub_5EC5+Bj sub_5EC5+12j	...
		dec	(hl)
		ld	a, (PilotAlienComputer)	; 0,=1 есть Alien Computer
		and	1
		ret	z
		ld	a, (cargoAlienItems)
		and	a
		ret	z
		ld	a, 0FDh	; '¤'
		in	a, (0FEh)
		cpl
		and	4
		ret	z
		inc	(hl)
		ld	l, 7Bh ; '{'    ; hl = cargoAlienItems
		dec	(hl)
		inc	hl
		inc	(hl)
		ld	hl, dTargletH

loc_5F09:				; CODE XREF: sub_5EC5+25j
		ld	(ix+23h), l
		ld	(ix+24h), h
		xor	a
		ret
; End of function sub_5EC5


; =============== S U B	R O U T	I N E =======================================


ReCalcCrdObj_coriolis:			; CODE XREF: CalcCrdCoriolis2:loc_7874p
		call	j_CalcNewCrdObj_Pl
		call	ProgressMissionWar ; Увеличение/уменьшение прогресса таргонов, и выдача	сообщения о войне
					; Вх:  ---
					; Вых: hl,a - не определены
		call	TestMainDiagonal ; Определение главной диагонали
					; Вых: z=1 на главной диагонале
		jr	z, loc_5F3F
		call	sub_83FE	; расстояние до	планеты
		cp	7
		ret	nc
		call	TestPilotInZoneWar ; Определяем	находится ли пилот/планета в заданной зоне войны
					; Вх:  (sp) =#00 зона оккупированная таргонами
					;	    =#3C зона линии фронта (активные боевые действия)
					;	    =#78 прифронтовая зона
					; Вых: cy =1 координаты	в пределах проверяемой зоны
					;	  =0 пилот в другой галактике галактике
					;	     война окончена/не начиналась
					;	     координаты	слишком	далеко от левого края
					;      hl,de,b,a - не определены
; ---------------------------------------------------------------------------
		db 0
; ---------------------------------------------------------------------------
		ld	b, 0Fh		; Targon
		jr	c, loc_5F87
		ld	a, (PilotNumGalaxy)
		and	a
		ret	z
		ld	a, (PilotMission3) ; =#06 при стыковке будет захват таргонами (вроде нереальная	ситуация)
					; =#04 в системе со танцией таргонов во	время войны
		cp	4
		ret	z
		ld	a, (PilotLegalStatus)
		cp	7Fh ; ''
		ld	b, 8		; Viper
		jr	nc, loc_5F87
		ret
; ---------------------------------------------------------------------------

loc_5F3F:				; CODE XREF: ReCalcCrdObj_coriolis+9j
		call	sub_83FE+2	; 83fe	 PlanetCrd
					; 83fe+2 SunCrd
		ret	c		; далеко от солнца
		cp	0Ch
		ret	nc		; далеко от солнца
		cp	6
		ret	z		; близко к солнцу
		ld	b, 0Dh		; ASP
		jr	nc, loc_5F81	; предложение о	покупке	кобры мк4
		ld	a, (ix+49h)
		and	a
		ret	nz
		push	ix
		pop	hl
		ld	b, 3
		ld	de, SunCrd

loc_5F5A:				; CODE XREF: ReCalcCrdObj_coriolis+55j
		inc	hl
		inc	de
		ld	a, (de)
		ld	(hl), a
		inc	hl
		inc	de
		ld	a, (de)
		and	80h ; 'А'
		ld	(hl), a
		inc	hl
		inc	de
		djnz	loc_5F5A
		ld	l, 48h ; 'H'
		ld	de, DataOfAround
		ld	c, 27h ; '''
		res	2, (ix+21h)
		call	loc_7892
		ldir
		inc	hl
		ld	a, (hl)
		xor	4
		ld	(hl), a
		pop	hl
		jp	loc_7889
; ---------------------------------------------------------------------------

loc_5F81:				; CODE XREF: ReCalcCrdObj_coriolis+3Aj
		ld	hl, 0F480h	; предложение о	покупке	кобры мк4
		ld	(CargoFound), hl

loc_5F87:				; CODE XREF: ReCalcCrdObj_coriolis+17j
					; ReCalcCrdObj_coriolis+2Bj
		push	ix
		ld	a, (DataOfAround.field_22)
		push	af
		ld	(ix+49h), 7Eh ;	'~'
		push	bc
		call	FindSlot4Obj
		pop	bc
		ld	a, b
		call	z, SetObjInSlot	; a - номер объекта для	установки
		pop	af
		ld	(DataOfAround.field_22), a
		pop	ix
		ret
; End of function ReCalcCrdObj_coriolis


; =============== S U B	R O U T	I N E =======================================

; предложение покупки кобры МК-4
; Attributes: noreturn



; === sub_5FA1 === CODE XREF: LaunchDocking+65p
sub_5FA1:				; CODE XREF: LaunchDocking+65p
		call	GoMissionWar	; Выдача миссии/подмиссии: Война с таргонами, если доступно
					; Вх:  ---
					; Вых: hl,de,bc,a - не определены
		call	GoMission3
		ld	a, (PilotCash2hi)
		and	a
		ret	z
		ld	a, (DataOfAround.field_26)
		xor	0Ch
		ret	nz
		ld	a, (byte_D08E)	; <>0 есть Cobra Mk4
		and	a
		ret	nz
		call	ClsAndPrintRamk
		ld	a, 5Ch ; '\'
		call	PrintMessage2
		call	PrintSymbol_SP
; End of function sub_5FA1

; ---------------------------------------------------------------------------
		db 0F4h
; ---------------------------------------------------------------------------
		call	sub_E1F7
		cp	59h ; 'Y'
		ret	nz
		call	sub_E1E8
; ---------------------------------------------------------------------------
		db  21h	; !
		db  8Eh	; О
		db 0D0h	; ╨
		db  36h	; 6
		db 0FDh	; ¤
		db  2Eh	; .
		db  67h	; g
		db  35h	; 5
		db 0C9h	; ╔

; =============== S U B	R O U T	I N E =======================================




; === sub_5FD5 === CODE XREF: sub_7F2D:loc_7F61j
sub_5FD5:				; CODE XREF: sub_7F2D:loc_7F61j
					; CalcNewCrdObj_Pl+58j
		ld	a, xl		; вылет	за пределы системы
		cp	10h
		jr	nz, loc_6043
		ld	a, (PlanetCrdZ_hi) ; был обсчет	планеты
		xor	(ix+8)
		rlca
		jr	c, loc_6043
		pop	hl
		ld	hl, Pilot_CrdYSystem
		ld	a, (fsPlCrdZh)
		ld	b, a
		add	a, a
		ld	d, a
		cp	0Ch
		call	nc, SetCrdSunPlInFreeSpace ; Установка начальных координат "солнца" и "планеты" в межпланетном пространстве
		ld	a, (fsSunCrdZh)
		ld	c, a
		add	a, a
		cp	d
		ld	a, c
		jr	nc, loc_5FFF
		ld	l, 99h ; 'Щ'
		ld	a, b

loc_5FFF:				; CODE XREF: sub_5FD5+25j
		add	a, a
		inc	(hl)
		jr	c, loc_6005
		dec	(hl)
		dec	(hl)

loc_6005:				; CODE XREF: sub_5FD5+2Cj
		ld	hl, (Pilot_CrdXSystem)
		ld	a, (Pilot_CrdYSystem)
		ld	h, a
		ld	(Pilot_CrdXTarget), hl
		call	SearchPlanetNearCursor
		ld	hl, (MinDistance)
		ld	a, h
		or	l
		ld	(InFreeSpace), a
		jr	nz, loc_6028
		ld	hl, loc_DDFB
		ld	(hl), 0C9h ; '╔'

loc_6021:				; DATA XREF: AttackTargetObj+3o
					; MoveMissile+17o ...
		push	hl
		call	sub_DCE9	; Гиперпрыжок
		pop	hl
		ld	(hl), 28h ; '('

loc_6028:				; CODE XREF: sub_5FD5+45j
		call	SetTargonStation ; Установка станции таргонов в	межпланетном пространстве для последней	подмиссии в войне с таргонами
		ld	a, 48h ; 'H'    ; установка координат солнца/планеты/станции
		ld	de, 3
		ld	hl, SunCrdXhi
		call	sub_8217
		sub	l
		call	sub_8217
		ld	hl, byte_604A
		call	sub_8217
		jp	SetVars4War	; Установка переменных,	после влёта в систему своим ходом
					; Вх:  ---
					; Вых: hl,de,b,a - не определены
					; Вых: z=1 есть	минеральная жизнь в трюме
					;	=0 минеральная жизнь померла
					;	   нет минеральной жизни
; ---------------------------------------------------------------------------

loc_6043:				; CODE XREF: sub_5FD5+4j sub_5FD5+Dj
					; DATA XREF: ...
		ld	(ix+8),	0DFh ; '▀'
		ret
; End of function sub_5FD5

; ---------------------------------------------------------------------------


; === sub_6F29 === CODE XREF: sub_8442j
sub_6F29:				; CODE XREF: sub_8442j
		ld	de, 8
		ld	hl, CompassIndFill
		ld	a, (InFreeSpace)
		and	a
		ret	z
		ld	a, 2
		ld	(loc_FD09+1), a
		ld	a, (sub_FD02+1)
		add	hl, de
		rrca
		ret	nc
		add	hl, de
		ret
; End of function sub_6F29


; =============== S U B	R O U T	I N E =======================================


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


; === sub_6FFB === CODE XREF: RAM:EA54j
sub_6FFB:				; CODE XREF: RAM:EA54j

; FUNCTION CHUNK AT EA57 SIZE 00000009 BYTES
; FUNCTION CHUNK AT EA75 SIZE 00000007 BYTES
; FUNCTION CHUNK AT EB47 SIZE 0000000D BYTES

		cp	59h ; 'Y'
		jr	nz, loc_7005
		xor	a
		ld	(PilotMineralLife), a ;	<>0 в трюме есть минеральная жизнь
					; =#FF минеральная жизнь померла
		dec	a
		ld	(hl), a

loc_7005:				; CODE XREF: sub_6FFB+2j
		ld	l, 8Eh ; 'О'
		inc	a
		cp	(hl)
		jp	z, loc_EB47
		jp	loc_EA57
; End of function sub_6FFB


; =============== S U B	R O U T	I N E =======================================




; === sub_700F === CODE XREF: sub_E935+7p
sub_700F:				; CODE XREF: sub_E935+7p
		ld	hl, (word_E1E0)
		ld	a, (hl)

loc_7013:				; DATA XREF: sub_5E1A:loc_5E24w
		nop
		ld	a, b
		cp	3
		dec	a
		ret	c
		ld	a, (hl)
		ret
; End of function sub_700F

; ---------------------------------------------------------------------------


; === sub_70A9 === CODE XREF: RAM:6FF5j
sub_70A9:				; CODE XREF: RAM:6FF5j
					; UseEnergyBomb:loc_70A2p
		ld	l, 28h ; '('
		call	FastSetAtrInViewScr
		ld	l, 0Fh

SetAtrViewScr:				; CODE XREF: j_SetAtrViewScrj
		ld	a, (CloakingDeviceOn)
		and	a
		jr	nz, FastSetAtrInViewScr
		ld	a, l
		and	111b
		ld	l, a
; End of function sub_70A9


; =============== S U B	R O U T	I N E =======================================


FastSetAtrInViewScr:			; CODE XREF: sub_70A9+2p sub_70A9+Bj
		ld	a, (PilotMission1)
		and	a
		jr	z, loc_70C4
		ld	a, l
		and	111010b
		ld	l, a

loc_70C4:				; CODE XREF: FastSetAtrInViewScr+4j
		halt
		ld	a, l
		ld	hl, 59FEh
		ld	b, 0Fh

loc_70CB:				; CODE XREF: FastSetAtrInViewScr+1Fj
		ld	(hl), a
		ld	d, h
		ld	e, l
		dec	de
		push	bc
		ld	bc, 1Dh
		lddr
		dec	hl
		dec	hl
		dec	hl
		pop	bc
		djnz	loc_70CB
		ret
; End of function FastSetAtrInViewScr

; ---------------------------------------------------------------------------
byte_70DC:	db 5Eh			; DATA XREF: ProgressMissionWaro
		db 43h
		db 0FCh
		db 0FFh
		db 0C3h
		db 66h
		db 73h


; === sub_7119 === CODE XREF: Cmd_I_GlobalMap+2Fp
sub_7119:				; CODE XREF: Cmd_I_GlobalMap+2Fp
					; RAM:ECC7p
		jp	loc_8E29
; End of function sub_7119

; ---------------------------------------------------------------------------


; === sub_7125 === CODE XREF: sub_FF25+16p
sub_7125:				; CODE XREF: sub_FF25+16p
		jp	sub_9579
; End of function sub_7125

; ---------------------------------------------------------------------------


; === sub_71CE === CODE XREF: NewJumpDockingCrdObj+8p
sub_71CE:				; CODE XREF: NewJumpDockingCrdObj+8p
		call	sub_71E4
		add	hl, hl
		jr	nc, loc_71D6
		set	7, h

loc_71D6:				; CODE XREF: sub_71CE+4j
		ld	(iy+2),	h
		ld	(iy+1),	l
		ld	de, 3
		add	iy, de
		add	ix, de
		ret
; End of function sub_71CE


; =============== S U B	R O U T	I N E =======================================




; === sub_71E4 === CODE XREF: sub_71CEp
sub_71E4:				; CODE XREF: sub_71CEp
		ld	h, (iy+2)
		ld	l, (iy+1)
		ld	d, (ix+2)
		ld	e, (ix+1)
		jp	loc_9CA5
; End of function sub_71E4


; =============== S U B	R O U T	I N E =======================================


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




; === sub_725C === CODE XREF: PrnCocpit_Ifn+35p
sub_725C:				; CODE XREF: PrnCocpit_Ifn+35p
		ld	a, (PilotMission1)
		and	a

loc_7260:				; CODE XREF: sub_726C+Aj
		ld	l, 2
		jr	nz, loc_7259
		ret
; End of function sub_725C


; =============== S U B	R O U T	I N E =======================================




; === sub_7265 === CODE XREF: PrnCocpit_Ifn+38p
sub_7265:				; CODE XREF: PrnCocpit_Ifn+38p
		ld	a, (CloakingDeviceOn)
		and	a
		jr	nz, loc_7253
		ret
; End of function sub_7265


; =============== S U B	R O U T	I N E =======================================




; === sub_726C === CODE XREF: LaunchDocking+90p
sub_726C:				; CODE XREF: LaunchDocking+90p
		ld	a, (PilotMission1)
		and	a
		ret	z
		ld	a, 1
		ld	(PilotMission1_1), a
		jr	loc_7260
; End of function sub_726C


; =============== S U B	R O U T	I N E =======================================


GoMission3:				; CODE XREF: sub_5FA1+3p

; FUNCTION CHUNK AT D0DD SIZE 00000003 BYTES
; FUNCTION CHUNK AT D0ED SIZE 00000003 BYTES
; FUNCTION CHUNK AT D123 SIZE 00000025 BYTES

		ld	a, (PilotMission3) ; =#06 при стыковке будет захват таргонами (вроде нереальная	ситуация)
					; =#04 в системе со танцией таргонов во	время войны
		dec	a
		jr	z, loc_727F
		dec	a

loc_727F:				; CODE XREF: GoMission3+4j
		jp	z, GiveMission3
		sub	3
		jp	z, EndMission3
		dec	a
		ld	a, 0D4h	; '╘'
		jr	z, loc_7295
		ret
; End of function GoMission3


; =============== S U B	R O U T	I N E =======================================

; Attributes: noreturn



; === sub_72D4 === CODE XREF: ScanKeyInPause+1Fp
sub_72D4:				; CODE XREF: ScanKeyInPause+1Fp
		inc	hl
; End of function sub_72D4


; =============== S U B	R O U T	I N E =======================================




; === sub_72D5 === CODE XREF: ScanKeyInPause+1Ap
sub_72D5:				; CODE XREF: ScanKeyInPause+1Ap
		push	bc
		ld	c, a
		call	16DCh
		ld	a, c
		pop	bc
		ret
; End of function sub_72D5

; ---------------------------------------------------------------------------

loc_72DD:				; CODE XREF: RAM:7315j
		call	sub_7A84
		nop
		jr	loc_7340
; ---------------------------------------------------------------------------

loc_72E3:				; CODE XREF: RAM:7317j
		ld	a, (byte_D08E)	; <>0 есть Cobra Mk4
		and	a
		jr	z, loc_733E
		ld	hl, (PilotFrontLaser)
		ld	a, h
		ld	d, l
		ld	hl, (PilotLeftLaser)
		ld	(PilotFrontLaser), hl
		ld	h, d
		ld	l, a
		ld	(PilotLeftLaser), hl
		jr	loc_7340
; ---------------------------------------------------------------------------
unk_72FB:	db  42h	; B		; DATA XREF: ScanKeyInPause+17o
		db  10h
		db  44h	; D
		db    8
		db  46h	; F
		db  40h	; @
		db  52h	; R
		db    4
		db  59h	; Y
		db  20h
		db  53h	; S
		db  80h	; А
		db    0
		db  20h
		db  19h
		db  4Ch	; L
		db  15h
		db  41h	; A
		db  17h
		db  4Bh	; K
		db  28h	; (
		db  43h	; C
		db  39h	; 9
		db  51h	; Q
		db  1Ah
		db    0
; ---------------------------------------------------------------------------
		jr	loc_72DD
; ---------------------------------------------------------------------------
		jr	loc_72E3
; ---------------------------------------------------------------------------
		db 0AFh	; п
		db  3Eh	; >
		db    0
		db  2Eh	; .
		db    8
		db  77h	; w
		db  23h	; #
		db  77h	; w
		db  23h	; #
		db  77h	; w
		db  23h	; #
		db  77h	; w
		db  23h	; #
		db  77h	; w
		db 0C9h	; ╔
		db  57h	; W
		db 0E6h	; ц
		db    3
		db  3Ah	; :
		db 0A2h	; в
		db  72h	; r
		db  20h
		db    3
		db 0A7h	; з
		db  28h	; (
		db  0Dh
		db  3Ah	; :
		db 0A2h	; в
		db  72h	; r
		db 0AAh	; к
		db  18h
		db    6
		db  21h	; !
		db  0Ah
		db 0FDh	; ¤
		db 0CDh	; ═
		db 0EEh	; ю
; ---------------------------------------------------------------------------

loc_733E:				; CODE XREF: RAM:72E7j
		rst	18h
; START	OF FUNCTION CHUNK FOR ScanKeyInPause

loc_733F:				; CODE XREF: ScanKeyInPause+2Bj
		ld	(hl), a

loc_7340:				; CODE XREF: RAM:72E1j	RAM:72F9j
		ld	hl, loc_72AC
		push	hl

loc_7344:				; CODE XREF: InitSpeedAngle+15j
		ld	b, 6
		ld	e, 0

loc_7348:				; CODE XREF: ScanKeyInPause+C0j
		push	bc
		halt
		ld	c, 4

loc_734C:				; CODE XREF: ScanKeyInPause+BDj
		ld	a, e
		add	a, a
		add	a, 5
		ld	e, a
		ld	b, a
		ld	a, 110000b
		out	(0FEh),	a

loc_7356:				; CODE XREF: ScanKeyInPause:loc_7356j
		djnz	$
		ld	a, e
		ld	b, a
		xor	a
		out	(0FEh),	a

loc_735D:				; CODE XREF: ScanKeyInPause:loc_735Dj
		djnz	$
		dec	c
		jr	nz, loc_734C
		pop	bc
		djnz	loc_7348
		ret
; END OF FUNCTION CHUNK	FOR ScanKeyInPause
; ---------------------------------------------------------------------------
		ld	(word_7578), sp
		call	InitSpeedAngle2
		call	PrnCocpit_Ifn

; =============== S U B	R O U T	I N E =======================================

; определение возможности сбора	минеральной жизни



; === sub_7370 === CODE XREF: CollisionWithObj+1CCAp
sub_7370:				; CODE XREF: CollisionWithObj+1CCAp
		ld	a, (PilotMineralLife) ;	<>0 в трюме есть минеральная жизнь
					; =#FF минеральная жизнь померла
		and	a
		ret	nz
		push	hl
		call	sub_83FE	; 83fe	 PlanetCrd
					; 83fe+2 SunCrd
		pop	hl
		ret	c
		cp	38h ; '8'
		ret	c
		push	hl
		call	TestMainDiagonal ; Определение главной диагонали
					; Вых: z=1 на главной диагонале
		pop	hl
		ret	nz
		ld	a, (cargoMinerals)
		cp	2
		ret	c
		ld	(PilotMineralLife), a ;	<>0 в трюме есть минеральная жизнь
					; =#FF минеральная жизнь померла
		ret
; End of function sub_7370

; ---------------------------------------------------------------------------
unk_738E:	db    3			; DATA XREF: SetCrdSunPlInFreeSpace+4o
					; начальные координаты "солнца" и "планеты" в межпланетном пространстве
		db    0
		db    3
		db  83h	; Г
		db    0
		db    3
; ---------------------------------------------------------------------------

loc_7394:				; CODE XREF: RAM:74A2j
		xor	a
		ld	(WhatOnScreen),	a
		call	j_ClsAndPrintRamk
		call	PrnCocpit_Ifn
; START	OF FUNCTION CHUNK FOR sub_728D

loc_739E:				; CODE XREF: sub_728D+12j
					; sub_728D+11Dj ...
		ld	hl, byte_6065
		set	7, (hl)
		call	sub_7461
		ld	a, (PressKeyPrnSymbol)
		inc	a
		jr	nz, loc_739E
		ld	hl, unk_7470

loc_73AF:				; CODE XREF: sub_728D+128j
		inc	(hl)
		ld	a, (hl)
		and	0Fh
		cp	0Ch
		jr	nc, loc_73AF
		ld	hl, byte_AE3C	; номера кораблей для интро
		ld	e, a
		ld	d, 0
		add	hl, de
		ld	c, (hl)
		call	SetShip4Intro
		nop

loc_73C3:				; CODE XREF: sub_728D+165j
		call	ClearScrBuffer
		call	sub_7430
		call	sub_9CAF
		call	CopyBuf2Screen
		ld	bc, 0FF1Fh
		ld	hl, byte_FC0F
		halt
		jr	loc_73E2
; END OF FUNCTION CHUNK	FOR sub_728D
; ---------------------------------------------------------------------------
		db    0
		db    0
		db    0
		db    0
		db    0
		db    0
		db    0
		db    0
		db    0
		db    0
; ---------------------------------------------------------------------------
; START	OF FUNCTION CHUNK FOR sub_728D

loc_73E2:				; CODE XREF: sub_728D+149j
		ld	a, 2
		bit	1, (hl)
		jr	nz, loc_73F7
		inc	a
		bit	3, (hl)
		jr	nz, loc_73F7
		ld	a, (PressKeyPrnSymbol)
		sub	20h ; ' '
		jr	nz, loc_73C3
		ld	b, a
		jr	loc_73FD
; ---------------------------------------------------------------------------

loc_73F7:				; CODE XREF: sub_728D+159j
					; sub_728D+15Ej
		ld	(byte_72A2), a
		and	3
		ld	b, a

loc_73FD:				; CODE XREF: sub_728D+168j
		ld	hl, FlagsSetUp
		ld	a, (hl)
		and	0FCh ; '№'
		or	b
		ld	(hl), a
		call	j_InitGame
		xor	a
		ld	(InFreeSpace), a
		ld	(IndDanger), a
		ld	(PilotMission1_1), a
		ld	(FuelScoopsOn),	a
		ld	(CloakingDeviceOn), a
		ld	(SunRadiusNova), a
		call	SaveCocpitInBuffer
		call	j_Cmd_L_PilotInfo
		call	InitSpeedAngle2
		jr	GameMain
; END OF FUNCTION CHUNK	FOR sub_728D
; ---------------------------------------------------------------------------
		call	sub_7447
		ld	hl, 0E01h
		ld	a, 6
		jr	loc_7438

; =============== S U B	R O U T	I N E =======================================




; === sub_7430 === CODE XREF: sub_728D+139p
sub_7430:				; CODE XREF: sub_728D+139p
		call	sub_7447
		ld	hl, 0D04h
		ld	a, 7

loc_7438:				; CODE XREF: RAM:742Ej
		ld	(CrdXPrnSymbol), hl
		ld	hl, (word_D0D5)
		inc	hl
		inc	hl
		set	7, (hl)
		call	j_PrintMessage2
		jr	loc_7459
; End of function sub_7430


; =============== S U B	R O U T	I N E =======================================




; === sub_7447 === CODE XREF: RAM:7426p	sub_7430p
sub_7447:				; CODE XREF: RAM:7426p	sub_7430p
		ld	hl, 105h
		ld	(CrdXPrnSymbol), hl
		ld	a, 1Dh
		ld	hl, (word_D0D5)
		inc	hl
		inc	hl
		set	7, (hl)
		call	j_PrintMessage1_A

loc_7459:				; CODE XREF: sub_7430+15j
		ld	hl, (word_D0D5)
		inc	hl
		inc	hl
		res	7, (hl)
		ret
; End of function sub_7447


; =============== S U B	R O U T	I N E =======================================




; === sub_7461 === CODE XREF: sub_728D+116p
sub_7461:				; CODE XREF: sub_728D+116p
		ld	hl, 5820h
		call	loc_7467

loc_7467:
		ld	b, 0F0h	; 'Ё'
		ld	a, 7

loc_746B:				; CODE XREF: sub_7461+Cj
		ld	(hl), a
		inc	hl
		djnz	loc_746B
		ret
; End of function sub_7461

; ---------------------------------------------------------------------------
unk_7470:	db  22h	; "             ; DATA XREF: sub_728D+11Fo
; ---------------------------------------------------------------------------
; START	OF FUNCTION CHUNK FOR sub_728D

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




; === sub_77A0 === CODE XREF: RAM:7615p	RAM:7637p ...
sub_77A0:				; CODE XREF: RAM:7615p	RAM:7637p ...

; FUNCTION CHUNK AT 7799 SIZE 00000007 BYTES

		push	hl
		exx
		dec	c
		exx
		and	a
		sbc	hl, de
		jp	c, loc_77ED
		jp	z, loc_77DD

loc_77AD:				; CODE XREF: sub_77A0+45j
		pop	hl
		ld	a, h
		and	a
		jr	z, loc_7799
		ld	a, 8

loc_77B4:				; CODE XREF: sub_77A0+22j
		sla	c
		adc	hl, hl
		jp	c, loc_780A
		sla	b
		rl	e
		rl	d
		dec	a
		jr	nz, loc_77B4

loc_77C4:				; CODE XREF: sub_77A0-3j
		ld	a, 8

loc_77C6:				; CODE XREF: sub_77A0+2Ej
		add	hl, hl
		jp	c, loc_780A
		ex	de, hl
		add	hl, hl
		ex	de, hl
		dec	a
		jr	nz, loc_77C6
		ld	a, 8

loc_77D2:				; CODE XREF: sub_77A0+3Aj
		sla	h
		jp	c, loc_780A
		sla	d
		dec	a
		jr	nz, loc_77D2
		ret
; ---------------------------------------------------------------------------

loc_77DD:				; CODE XREF: sub_77A0+Aj
		push	bc
		ld	a, c
		sub	b
		pop	bc
		jr	c, loc_77ED
		jr	z, loc_77ED
		jp	loc_77AD
; ---------------------------------------------------------------------------

loc_77E8:				; CODE XREF: sub_77A0+4Fj
		set	2, b
		jp	loc_77F3
; ---------------------------------------------------------------------------

loc_77ED:				; CODE XREF: sub_77A0+7j sub_77A0+41j	...
		exx
		dec	c
		jr	nz, loc_77E8
		set	3, b

loc_77F3:				; CODE XREF: sub_77A0+4Aj
		inc	c
		exx
		pop	hl
		ld	a, 17h

loc_77F8:				; CODE XREF: sub_77A0+67j
		sla	c
		adc	hl, hl
		push	af
		sla	b
		rl	e
		rl	d
		jr	c, loc_7831
		pop	af
		dec	a
		jr	nz, loc_77F8
		ret
; ---------------------------------------------------------------------------

loc_780A:				; CODE XREF: sub_77A0+18j sub_77A0+27j ...
		sla	b
		rl	e
		rl	d
		push	af
		scf
		rr	h
		rr	l
		pop	af
		jr	c, loc_7820
		push	hl
		and	a
		sbc	hl, de
		jr	nc, loc_7828
		pop	hl

loc_7820:				; CODE XREF: sub_77A0+77j
		scf
		rr	d
		and	a
		sbc	hl, de
		jr	loc_7839
; ---------------------------------------------------------------------------

loc_7828:				; CODE XREF: sub_77A0+7Dj
		pop	hl
		pop	af

loc_782A:				; CODE XREF: RAM:75F2j	RAM:7663j ...
		xor	a
		ret
; ---------------------------------------------------------------------------

loc_782C:				; CODE XREF: sub_77A0+92j
		scf
		rr	h
		jr	loc_7836
; ---------------------------------------------------------------------------

loc_7831:				; CODE XREF: sub_77A0+63j
		pop	af
		jr	c, loc_782C
		rr	h

loc_7836:				; CODE XREF: sub_77A0+8Fj
		scf
		rr	d

loc_7839:				; CODE XREF: sub_77A0+86j
		ld	a, h
		ld	b, d

loc_783B:				; CODE XREF: RAM:768Ep	RAM:7769p
		ld	e, 0

loc_783D:				; CODE XREF: RAM:7682p	RAM:775Dp
		ld	d, 0FEh	; '■'

loc_783F:				; CODE XREF: sub_77A0+AAj sub_77A0+B1j
		sla	e
		rla
		jr	c, loc_784C
		cp	b
		jr	nc, loc_784C
		sla	d
		ret	nc
		jr	loc_783F
; ---------------------------------------------------------------------------

loc_784C:				; CODE XREF: sub_77A0+A2j sub_77A0+A5j
		sub	b
		scf
		rl	d
		ret	nc
		jr	loc_783F
; End of function sub_77A0


; =============== S U B	R O U T	I N E =======================================


CalcCrdSunPlanet:			; CODE XREF: sub_728D+22Cp
					; LaunchDocking+F1p
		ld	ix, SunCrd
		call	j_CalcNewCrdObj_Pl
		ld	ix, PlanetCrd
		jp	CalcCrdSunPlanet_2
; End of function CalcCrdSunPlanet

; ---------------------------------------------------------------------------
; START	OF FUNCTION CHUNK FOR CalcCrdCoriolis2

CalcCrdCoriolis:			; CODE XREF: CalcCrdCoriolis2+5j
		ld	a, (J_Drive_Coriolis)
		and	a
		ld	ix, objCoriolis
		jr	nz, loc_7874
		ld	a, (ix+22h)
		and	a
		ret	nz
		dec	a
		ld	(J_Drive_Coriolis), a

loc_7874:				; CODE XREF: CalcCrdCoriolis2+1AA1j
		call	ReCalcCrdObj_coriolis
		ld	a, (ix+8)
		and	7Fh ; ''
		ret	nz
		ld	a, (ix+2)
		and	7Fh ; ''
		ret	nz
		ld	a, (ix+5)
		and	7Fh ; ''
		ret	nz

loc_7889:				; CODE XREF: ReCalcCrdObj_coriolis+6Dj
		bit	7, (ix+26h)
		ret	nz
		xor	a
		ld	(J_Drive_Coriolis), a

loc_7892:				; CODE XREF: ReCalcCrdObj_coriolis+62p
		ld	a, 0F0h	; 'Ё'
		ld	(ix+22h), a
		ret
; END OF FUNCTION CHUNK	FOR CalcCrdCoriolis2

; =============== S U B	R O U T	I N E =======================================


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




; === sub_78B1 === CODE XREF: PrintStars+4j
sub_78B1:				; CODE XREF: PrintStars+4j
					; PrintStars+Ep
		ld	a, (WhatOnScreen)

loc_78B4:				; DATA XREF: sub_78B1+5w
		cp	0
		ld	(loc_78B4+1), a
		jr	z, loc_78C5
		ld	b, 20h ; ' '
		ld	hl, byte_7A39

loc_78C0:				; CODE XREF: sub_78B1+12j
		ld	(hl), 0FFh
		inc	hl
		djnz	loc_78C0

loc_78C5:				; CODE XREF: sub_78B1+8j
		ld	b, 0
		cp	2
		jr	nc, loc_78D7
		push	af
		rrca
		ld	b, a
		ld	a, (HighDeltaSpeed)
		srl	a
		inc	a
		or	b
		ld	b, a
		pop	af

loc_78D7:				; CODE XREF: sub_78B1+18j
		ld	hl, HighAngleLR
		jr	c, loc_78DD
		inc	hl

loc_78DD:				; CODE XREF: sub_78B1+29j
		push	af
		ld	a, (hl)
		jr	c, loc_78E9
		and	80h ; 'А'
		ld	c, a
		ld	a, (hl)
		rrca
		and	3Fh ; '?'
		or	c

loc_78E9:				; CODE XREF: sub_78B1+2Ej
		ld	c, a
		pop	af
		push	af
		rrca
		and	80h ; 'А'
		push	af
		xor	c
		ld	c, a
		ld	a, 1
		xor	l
		ld	l, a
		pop	de
		push	de
		rrca
		cpl
		and	80h ; 'А'
		xor	d
		xor	(hl)
		ld	d, a
		pop	af
		ld	l, 0E6h	; 'ц'
		xor	(hl)
		ld	e, a
		inc	e
		pop	af
		cp	2
		jr	nc, loc_790C
		ld	e, 0

loc_790C:				; CODE XREF: sub_78B1+57j
		ld	a, (InFreeSpace)
		and	a
		ld	a, 10h
		jr	z, loc_7916
		ld	a, 4

loc_7916:				; CODE XREF: sub_78B1+61j
		push	af
		ld	hl, byte_7A39

loc_791A:				; CODE XREF: sub_78B1+F6j
		push	af
		ld	a, (hl)
		inc	hl
		and	(hl)
		inc	a
		jp	z, loc_79A4
		push	de
		dec	hl
		ld	d, (hl)
		inc	hl
		ld	e, (hl)
		push	hl
		ld	a, d
		add	a, a
		ld	l, a
		ld	a, c
		call	sub_9924
		ld	d, a
		ld	a, e
		add	a, a
		ld	l, a
		ld	a, c
		call	sub_9924
		ld	e, a
		pop	hl
		ld	a, (hl)
		cpl
		xor	c
		and	80h ; 'А'
		or	e
		ex	af, af'
		dec	hl
		ld	a, (hl)
		ex	af, af'
		call	sub_7A59
		inc	hl
		ex	af, af'
		xor	c
		and	80h ; 'А'
		or	d
		call	sub_7A59
		ld	a, b
		and	a
		jr	z, loc_797B
		dec	hl
		ld	a, (hl)
		add	a, a
		push	hl
		ld	l, a
		ld	a, b
		call	sub_9924
		inc	a
		ld	d, a
		pop	hl
		ld	a, (hl)
		xor	b
		and	80h ; 'А'
		or	d
		call	sub_7A59
		inc	hl
		ld	a, (hl)
		push	hl
		add	a, a
		ld	l, a
		ld	a, b
		call	sub_9924
		inc	a
		pop	hl
		ld	e, a
		ld	a, (hl)
		xor	b
		and	80h ; 'А'
		or	e
		call	sub_7A59

loc_797B:				; CODE XREF: sub_78B1+A0j
		pop	de
		ld	a, e
		call	sub_7A59
		dec	hl
		ld	a, d
		call	sub_7A59
		ld	a, b
		rlca
		ld	a, (hl)
		inc	hl
		jr	nc, loc_7992
		or	(hl)
		and	70h ; 'p'
		jr	nz, loc_7992
		ld	(hl), 0FFh

loc_7992:				; CODE XREF: sub_78B1+D8j sub_78B1+DDj
		ld	a, r
		rrca
		and	81h ; 'Б'
		call	sub_7A59
		dec	hl
		ld	a, r
		rrca
		and	81h ; 'Б'
		call	sub_7A59
		inc	hl

loc_79A4:				; CODE XREF: sub_78B1+6Ej
		inc	hl
		pop	af
		dec	a
		jp	nz, loc_791A
		pop	af
		push	af

loc_79AC:				; CODE XREF: sub_78B1+119j
		push	af
		dec	hl
		ld	a, (hl)
		dec	hl
		or	80h ; 'А'
		inc	a
		jr	z, loc_79BA
		ld	a, (hl)
		add	a, a
		add	a, a
		jr	nc, loc_79C8

loc_79BA:				; CODE XREF: sub_78B1+102j
		inc	hl
		call	IntRnd_A
		rrca
		ld	(hl), a
		call	IntRnd_A
		rrca
		and	0BFh ; '┐'
		dec	hl
		ld	(hl), a

loc_79C8:				; CODE XREF: sub_78B1+107j
		pop	af
		dec	a
		jr	nz, loc_79AC
		pop	bc

loc_79CD:				; CODE XREF: sub_78B1+145j
		push	bc
		ld	a, (hl)
		bit	6, a
		inc	hl
		jr	nz, loc_79F4
		dec	hl
		add	a, a
		ld	a, 3Fh ; '?'
		jr	c, loc_79DD
		sub	(hl)
		jr	loc_79DE
; ---------------------------------------------------------------------------

loc_79DD:				; CODE XREF: sub_78B1+127j
		adc	a, (hl)

loc_79DE:				; CODE XREF: sub_78B1+12Aj
		and	7Fh ; ''
		ld	b, a
		inc	hl
		ld	a, (hl)
		add	a, a
		ld	a, 77h ; 'w'
		jr	c, loc_79EB
		sub	(hl)
		jr	loc_79EE
; ---------------------------------------------------------------------------

loc_79EB:				; CODE XREF: sub_78B1+135j
		adc	a, (hl)
		or	80h ; 'А'

loc_79EE:				; CODE XREF: sub_78B1+138j
		ld	c, a
		push	hl
		call	objPrnPixel
		pop	hl

loc_79F4:				; CODE XREF: sub_78B1+121j
		inc	hl
		pop	bc
		djnz	loc_79CD
		ret
; End of function sub_78B1


; =============== S U B	R O U T	I N E =======================================

; опрос	клвиши во время	подготовки к прыжку



; === sub_79F9 === CODE XREF: PrnCountdownJump+3Fp
sub_79F9:				; CODE XREF: PrnCountdownJump+3Fp
		ld	bc, 1FFh
		ld	a, 0F7h	; 'ў'
		call	sub_8B2A
		jr	nc, loc_7A0E
		ld	a, 0EFh	; 'я'
		call	sub_8B2A
		jr	c, loc_7A0D
		ld	a, 0Eh
		sub	c

loc_7A0D:				; CODE XREF: sub_79F9+Fj
		ld	c, a

loc_7A0E:				; CODE XREF: sub_79F9+8j
		ld	a, c
		sub	(hl)
		jr	nc, loc_7A14
		add	a, 8

loc_7A14:				; CODE XREF: sub_79F9+17j
		ld	b, a
		ld	(hl), c
		ld	a, b
		or	b
		ret	z

loc_7A19:				; CODE XREF: sub_79F9+2Bj
		ld	b, 3
		ld	hl, PilotGalaxyData0

loc_7A1E:				; CODE XREF: sub_79F9+28j
		call	NextGalaxyData
		djnz	loc_7A1E
		dec	a
		jr	nz, loc_7A19
		ret
; End of function sub_79F9


; =============== S U B	R O U T	I N E =======================================




; === sub_7A27 === CODE XREF: RAM:loc_F0F7p
sub_7A27:				; CODE XREF: RAM:loc_F0F7p
		ld	a, (PilotTargonWar) ; =#81 при сбросе миссий
					; 7,=1 начало войны, предолжение участвовать
					; 6-0, чем больше число	тем дальше от левого края карты	захват планет таргонами
					;
					; =#01 война закончена
		cp	2
		jr	c, loc_7A33
		ld	a, (PilotRating2) ; >=#84 в трюме десант
					; 7,=1 война была или идёт
					; 6,=1 война закончилать
					; 3-1 -	ранг (=текущая стадия войны)
		cp	84h ; 'Д'

loc_7A33:				; CODE XREF: sub_7A27+5j
		ld	a, 0Ch
		ret	c
		ld	a, 0BCh	; '╝'
		ret
; End of function sub_7A27

; ---------------------------------------------------------------------------
byte_7A39:	db 3Ah			; DATA XREF: sub_78B1+Co sub_78B1+66o
		db 1Dh
		db 30h
		db 0E8h
		db 27h
		db 9Ch
		db 0BBh
		db    1
		db  22h	; "
		db  62h	; b
		db  96h	; Ц
		db 0AAh	; к
		db 0ACh	; м
		db    6
		db  9Ch	; Ь
		db 0F7h	; ў
		db  2Fh	; /
		db    6
		db  2Dh	; -
		db  76h	; v
		db  2Fh	; /
		db 0FEh	; ■
		db  28h	; (
		db  12h
		db  12h
		db 0B2h	; ▓
		db  8Eh	; О
		db  25h	; %
		db 0BFh	; ┐
		db 0D9h	; ┘
		db 0B6h	; ╢
		db 0DEh	; ▐

; =============== S U B	R O U T	I N E =======================================




; === sub_7A59 === CODE XREF: sub_78B1+92p sub_78B1+9Bp ...
sub_7A59:				; CODE XREF: sub_78B1+92p sub_78B1+9Bp ...
		push	bc
		ld	b, a
		xor	(hl)
		rlca
		ld	a, b
		jr	c, loc_7A74
		and	7Fh ; ''
		ld	c, a
		ld	a, (hl)
		and	7Fh ; ''
		add	a, c
		bit	7, a
		jr	z, loc_7A6C
		sbc	a, a

loc_7A6C:				; CODE XREF: sub_7A59+10j sub_7A59+26j ...
		ld	c, a
		ld	a, b
		and	80h ; 'А'
		or	c
		ld	(hl), a
		pop	bc
		ret
; ---------------------------------------------------------------------------

loc_7A74:				; CODE XREF: sub_7A59+5j
		and	7Fh ; ''
		ld	c, a
		ld	a, (hl)
		and	7Fh ; ''
		sub	c
		jr	nc, loc_7A81
		neg
		jr	loc_7A6C
; ---------------------------------------------------------------------------

loc_7A81:				; CODE XREF: sub_7A59+22j
		ld	b, (hl)
		jr	loc_7A6C
; End of function sub_7A59


; =============== S U B	R O U T	I N E =======================================




; === sub_7A84 === CODE XREF: RAM:loc_72DDp
sub_7A84:				; CODE XREF: RAM:loc_72DDp
		ld	hl, loc_972F
		ld	a, (hl)
		xor	90h ; 'Р'
		ld	(hl), a
		ld	hl,  CopyLocator2Scr+1
		ld	a, (hl)
		xor	7
		ld	(hl), a
		ld	a, (WhatOnScreen)
		cp	4
		ret	nc
		ld	a, 1
		ld	(FlgSaveScaner), a
		ret
; End of function sub_7A84


; =============== S U B	R O U T	I N E =======================================

; hl - адрес допслота



; === sub_7A9E === CODE XREF: sub_B71C+1Cp sub_F916+12p
sub_7A9E:				; CODE XREF: sub_B71C+1Cp sub_F916+12p
		push	hl
		ld	de, 7
		add	hl, de
		ld	(hl), 18h	; +7 =#18
		inc	hl
		ld	(hl), d		; +8 =#00
		inc	hl
		ld	(hl), d		; +9 =#00
		ld	a, (byte_606A)	; coriolis+22
		and	a
		jr	z, loc_7AB1
		set	7, (hl)		; +9 =#80

loc_7AB1:				; CODE XREF: sub_7A9E+Fj
		ld	e, (ix+23h)
		ld	d, (ix+24h)
		ld	hl, AdrDataShips

loc_7ABA:				; CODE XREF: sub_7A9E+2Ej sub_7A9E+31j
		ld	c, (hl)
		inc	c
		dec	c
		jr	nz, loc_7AC5
		ld	sp, 4120h
		jp	loc_74F4
; ---------------------------------------------------------------------------

loc_7AC5:				; CODE XREF: sub_7A9E+1Fj
		inc	hl
		ld	a, (hl)
		inc	hl
		cp	e
		ld	a, (hl)
		inc	hl
		inc	hl
		jr	nz, loc_7ABA
		cp	d
		jr	nz, loc_7ABA
		pop	hl
		ld	a, c
		cp	0C0h ; '└'
		jr	z, loc_7ADD
		ld	a, (byte_B454)
		and	3
		or	c

loc_7ADD:				; CODE XREF: sub_7A9E+37j
		ld	(hl), a
		bit	6, a
		ret	z
		bit	3, c
		jr	z, loc_7AF7
		res	3, (hl)
		ld	a, (PilotExtWeapon) ; 7,=1 - ECM Jammer
					; 6,=1 - Cloaking device
					; 5,=1 - нельзя	купить товары(в	трюме беженцы или десант)
					; 4
					; 3 =1 3я миссия была выполнена
					; 2 =1 - третья	миссия выполнена
					; 1 =1 - второя	миссия выполнена
					; 0 =1 - первая	миссия выполнена
		bit	6, a
		jr	z, loc_7AF7
		call	IntRnd_A
		and	70h ; 'p'
		jr	nz, loc_7AF7
		set	3, (hl)

loc_7AF7:				; CODE XREF: sub_7A9E+45j sub_7A9E+4Ej ...
		bit	4, (hl)
		jr	z, loc_7B03
		call	IntRnd_A
		and	(hl)
		jr	nz, loc_7B03
		res	4, (hl)

loc_7B03:				; CODE XREF: sub_7A9E+5Bj sub_7A9E+61j
		bit	5, (hl)
		ret	z
		ld	a, (byte_606A)
		and	a
		ret	nz
		call	IntRnd_A
		and	6
		ret	z
		res	5, (hl)
		ret
; End of function sub_7A9E


; =============== S U B	R O U T	I N E =======================================

; номер	вершины	с лазером

SetPrnCrdInExtSlot:			; CODE XREF: PrintObject+3j
		ld	a, (iy+14h)
		call	GetPrnCrdVertex	; Получение печатных координат вершины из таблицы по номеру
					; Вх:  a - номер вершины
					; Вых: de - печатные координаты	вершины
					;      a - знаки координат
					;      z =1 вершина печатается
					;      hl- не определен
		ld	hl, (word_B452)
		inc	hl
		ld	(hl), a
		inc	hl
		ld	(hl), d
		inc	hl
		ld	(hl), e
		push	hl
		ld	a, 3
		call	GetPrnCrdVertex	; Получение печатных координат вершины из таблицы по номеру
					; Вх:  a - номер вершины
					; Вых: de - печатные координаты	вершины
					;      a - знаки координат
					;      z =1 вершина печатается
					;      hl- не определен
		pop	hl
		inc	hl
		ld	(hl), a
		inc	hl
		ld	a, d
		xor	3
		ld	(hl), a
		inc	hl
		ld	a, e
		xor	3
		ld	(hl), a
		ret
; End of function SetPrnCrdInExtSlot


; =============== S U B	R O U T	I N E =======================================


SetBit2ExtSlot_rndA:			; CODE XREF: objPrnObj:objPrnLaserp
		ld	hl, (word_B452)
		ld	a, (hl)
		push	hl
		ld	hl, PlayerAggressor
		call	bit2A_equ_bit2HL
		pop	hl
		ld	(hl), a
		inc	hl
		ld	(hl), 0FFh
		jp	IntRnd_A
; End of function SetBit2ExtSlot_rndA


; =============== S U B	R O U T	I N E =======================================




; === sub_7B4A === CODE XREF: AttackTargetObj+5Bp
sub_7B4A:				; CODE XREF: AttackTargetObj+5Bp
					; AttackTargetObj+5Ep ...
		ld	de, 3
		add	ix, de
		add	iy, de
		ld	a, (ix+0FFh)
		xor	(iy+0FFh)
		ld	e, (ix+0FEh)
		ld	a, (iy+0FEh)
		jr	z, loc_7B63
		add	a, e

loc_7B60:				; CODE XREF: sub_7B4A+1Aj sub_7B4A+1Ej
		cp	24h ; '$'
		ret
; ---------------------------------------------------------------------------

loc_7B63:				; CODE XREF: sub_7B4A+13j
		sub	e
		jr	nc, loc_7B60
		neg
		jr	loc_7B60
; End of function sub_7B4A


; =============== S U B	R O U T	I N E =======================================


AttackTargetObj:			; CODE XREF: sub_7C13+17p sub_7C13+1Dp ...
		and	0Fh
		ret	z
		ld	hl, loc_6021
		ld	iy,  loc_B44C+1

loc_7B74:				; CODE XREF: AttackTargetObj+15j
		ld	de, 27h	; '''
		add	hl, de
		ld	e, 0Ah
		add	iy, de
		inc	a
		cp	9
		jr	nz, loc_7B74

AttackTargetObjIY:			; CODE XREF: sub_7C13+36p
		ld	a, (iy+0)
		and	a
		bit	7, a
		ret	z
		bit	6, a
		jr	nz, loc_7BA7
		bit	2, a
		ret	z
		push	ix
		push	hl
		pop	ix
		ld	a, (byte_B455)
		xor	(ix+25h)
		and	0Fh
		pop	ix
		ret	nz
		call	IntRnd_A
		and	0Fh
		ret	nz
		jr	loc_7BBE
; ---------------------------------------------------------------------------

loc_7BA7:				; CODE XREF: AttackTargetObj+20j
		ld	e, a
		or	c
		and	3
		ret	z
		ld	a, e
		xor	c
		and	3
		ret	z
		bit	0, (ix+26h)
		jr	nz, loc_7BBE
		bit	2, e
		jr	nz, loc_7BBE
		bit	2, c
		ret	z

loc_7BBE:				; CODE XREF: AttackTargetObj+3Bj
					; AttackTargetObj+4Bj ...
		push	iy
		push	hl
		pop	iy
		push	ix
		call	sub_7B4A
		call	c, sub_7B4A
		call	c, sub_7B4A
		pop	ix
		pop	iy
		ret	nc
		ld	a, (byte_B455)
		ld	e, a
		ld	a, (iy+8)
		add	a, a
		add	a, a
		add	a, a
		add	a, a
		or	e
		ld	(iy+8),	a
		push	hl
		ex	(sp), iy
		ld	a, (ix+22h)
		rrca
		rrca
		and	3Fh ; '?'
		xor	0Fh
		ld	e, a
		ld	a, (iy+22h)
		sub	e
		jr	nc, loc_7C0C
		set	4, (iy+20h)
		push	hl
		push	ix
		push	iy
		pop	ix
		push	ix
		call	j_DestroyObj
; ---------------------------------------------------------------------------
		db 0FDh	; ¤
		db 0E1h	; с
		db 0DDh	; ▌
		db 0E1h	; с
		db 0E1h	; с
		db  3Eh	; >
		db    1
; ---------------------------------------------------------------------------

loc_7C0C:				; CODE XREF: AttackTargetObj+89j
		ld	(iy+22h), a
		pop	iy
		scf
		ret
; End of function AttackTargetObj


; =============== S U B	R O U T	I N E =======================================




; === sub_7C13 === CODE XREF: sub_7C60+7Ap
sub_7C13:				; CODE XREF: sub_7C60+7Ap
		ld	c, (iy+0)
		ld	b, (iy+8)
		push	hl
		ld	a, (ix+23h)
		cp	6Ah ; 'j'       ; dTargletH
		jr	nz, loc_7C2F
		bit	0, (ix+26h)
		jr	z, loc_7C2F
		ld	a, (ix+25h)
		call	AttackTargetObj
		jr	c, loc_7C5A

loc_7C2F:				; CODE XREF: sub_7C13+Cj sub_7C13+12j
		ld	a, b
		call	AttackTargetObj
		jr	c, loc_7C5A
		ld	a, b
		rrca
		rrca
		rrca
		rrca
		call	AttackTargetObj
		jr	c, loc_7C5A
		ld	hl, DataOfAround
		ld	iy, byte_B461
		ld	a, 7

loc_7C48:				; CODE XREF: sub_7C13+45j
		push	af
		call	AttackTargetObjIY
		jr	c, loc_7C5C
		ld	de, 27h	; '''
		add	hl, de
		ld	e, 0Ah
		add	iy, de
		pop	af
		dec	a
		jr	nz, loc_7C48

loc_7C5A:				; CODE XREF: sub_7C13+1Aj sub_7C13+20j ...
		pop	hl
		ret
; ---------------------------------------------------------------------------

loc_7C5C:				; CODE XREF: sub_7C13+39j
		pop	af
		pop	hl
		scf
		ret
; End of function sub_7C13


; =============== S U B	R O U T	I N E =======================================




; === sub_7C60 === CODE XREF: MoveAllObjAI+1p
sub_7C60:				; CODE XREF: MoveAllObjAI+1p
		ld	(EnergyBombUsed), a
		ld	hl, byte_B454
		inc	(hl)
		ld	a, (hl)
		and	3
		jr	nz, loc_7C6D
		inc	(hl)

loc_7C6D:				; CODE XREF: sub_7C60+Aj
		ld	ix, objCoriolis
		ld	hl, byte_B457	; 5,+0 =0 - объектом был запущен корабль
					; +1,+2,+3 - печатные координаты вершины с лазером
					; +1,+2,+3 - печатные координаты вершины №3 xor	3
					; +8 биты 0-3,=0 можно увеличить рейтинг
		ld	a, 8

loc_7C76:				; CODE XREF: sub_7C60:loc_7CE1j
		push	af
		ld	(byte_B455), a
		ld	a, (hl)
		and	0C0h ; '└'
		xor	0C0h ; '└'
		push	hl
		pop	iy
		jr	z, loc_7C88
		bit	5, (hl)
		jr	z, loc_7CDF

loc_7C88:				; CODE XREF: sub_7C60+22j
		dec	(iy+7)
		jr	nz, loc_7CDF
		ld	a, (ix+16h)
		cpl
		rrca
		rrca
		rrca
		and	1Fh
		or	3
		ld	(iy+7),	a
		bit	6, (hl)
		jr	z, loc_7CD6
		ld	a, (ix+20h)
		and	a
		jr	z, loc_7CDF
		bit	5, (hl)
		jr	z, loc_7CD8
		ld	a, (hl)
		and	3
		xor	3
		jr	z, loc_7CB8
		ld	a, (byte_606A)
		and	a
		ld	e, 7
		jr	nz, loc_7CD2

loc_7CB8:				; CODE XREF: sub_7C60+4Ej
		ld	a, (ix+22h)
		ld	e, 3Fh ; '?'
		cp	e
		jr	c, loc_7CD2
		ld	a, (iy+8)
		and	a
		jr	z, loc_7CCC
		call	IntRnd_A
		cp	e
		jr	c, loc_7CD8

loc_7CCC:				; CODE XREF: sub_7C60+64j
		call	IntRnd_A
		cp	e
		jr	z, loc_7CD8

loc_7CD2:				; CODE XREF: sub_7C60+56j sub_7C60+5Ej
		call	IntRnd_A
		and	e

loc_7CD6:				; CODE XREF: sub_7C60+3Dj
		jr	z, loc_7D37

loc_7CD8:				; CODE XREF: sub_7C60+47j sub_7C60+6Aj ...
		res	7, (hl)
		call	sub_7C13
		set	7, (hl)

loc_7CDF:				; CODE XREF: sub_7C60+26j sub_7C60+2Bj ...
		jr	nc, loc_7D29

loc_7CE1:				; CODE XREF: sub_7C60+D4j
		jr	nc, loc_7C76
		ld	a, (hl)
		push	hl
		push	iy
		pop	hl
		call	bit2A_equ_bit2HL
		pop	hl
		ld	(hl), a
		ld	c, (iy+4)
		ld	d, (iy+5)
		ld	e, (iy+6)
		push	hl
		push	hl
		pop	iy
		ld	b, (iy+1)
		ld	h, (iy+2)
		ld	l, (iy+3)
		ld	a, b
		and	c
		inc	a
		jr	z, loc_7D28
		ld	a, b
		inc	a
		jr	nz, loc_7D10
		ld	a, b
		ld	b, c
		ld	c, a
		ex	de, hl

loc_7D10:				; CODE XREF: sub_7C60+AAj
		ld	a, c
		inc	a
		jr	nz, loc_7D1C
		call	IntRnd_A
		ld	c, a
		or	0C0h ; '└'
		ld	d, a
		ld	e, a

loc_7D1C:				; CODE XREF: sub_7C60+B2j
		ld	a, b
		rrca
		rrca
		rl	c
		rlca
		rlca
		rl	c
		call	PrnLineInBuf_clip

loc_7D28:				; CODE XREF: sub_7C60+A6j
		pop	hl

loc_7D29:				; CODE XREF: sub_7C60:loc_7CDFj
					; sub_7C60+DCj
		ld	de, 27h	; '''
		add	ix, de
		ld	e, 0Ah
		add	hl, de
		pop	af
		dec	a
		and	a
		jr	nz, loc_7CE1
		ret
; ---------------------------------------------------------------------------

loc_7D37:				; CODE XREF: sub_7C60:loc_7CD6j
		ld	(hl), 0A0h ; 'а'
		ld	(iy+7),	h
		jr	loc_7D29
; End of function sub_7C60


; =============== S U B	R O U T	I N E =======================================




; === sub_7D3E === CODE XREF: MoveMissile+6p
sub_7D3E:				; CODE XREF: MoveMissile+6p
		ld	a, (ix+23h)
		cp	0E3h ; 'у'
		ret	nz
		res	6, (ix+20h)
		set	4, (ix+20h)
		ret
; End of function sub_7D3E


; =============== S U B	R O U T	I N E =======================================

; установка объекта кораблем противника
; Attributes: noreturn

SetObjbyShip:				; CODE XREF: CallFunction_Obj+4p
		cp	2
		jp	nz, sub_B867
		call	sub_B867	; запускает ракету противник
; ---------------------------------------------------------------------------
		ret	nz
		ld	iy, (word_B452)
		ld	a, (iy+8)
		and	0Fh
		ld	(ix+25h), a
		xor	a
		ret
; End of function SetObjbyShip


; =============== S U B	R O U T	I N E =======================================




; === sub_7D64 === CODE XREF: sub_F9E1+3j
sub_7D64:				; CODE XREF: sub_F9E1+3j
		ld	hl, byte_B457	; 5,+0 =0 - объектом был запущен корабль
					; +1,+2,+3 - печатные координаты вершины с лазером
					; +1,+2,+3 - печатные координаты вершины №3 xor	3
					; +8 биты 0-3,=0 можно увеличить рейтинг
		ld	a, (ObjInTarget)
		sub	0Ah
		cpl
		ld	de, 0Ah

loc_7D70:				; CODE XREF: sub_7D64+10j
		dec	a
		jr	z, loc_7D76
		add	hl, de
		jr	loc_7D70
; ---------------------------------------------------------------------------

loc_7D76:				; CODE XREF: sub_7D64+Dj
		bit	6, (hl)
		jr	nz, loc_7D7F
		ld	a, (hl)
		or	11111100b
		inc	a
		ret	z

loc_7D7F:				; CODE XREF: sub_7D64+14j
		ld	a, (PlayerAggressor)
		call	bit2A_equ_bit2HL
		ld	(PlayerAggressor), a
		ld	e, 8
		add	hl, de
		ex	af, af'
		ld	a, (hl)
		and	0Fh
		ret	z
		ld	a, (hl)
		add	a, a
		add	a, a
		add	a, a
		add	a, a
		ld	(hl), a
		ex	af, af'
		and	100b
		ret	z
		ld	hl, byte_606A
		ld	a, (hl)
		and	a
		ret
; End of function sub_7D64


; =============== S U B	R O U T	I N E =======================================




; === sub_7E02 === CODE XREF: PrnObjIfVisible+F7j
sub_7E02:				; CODE XREF: PrnObjIfVisible+F7j
		jr	z, ShipHyperJump

objDestroyed:				; получение координат и	знаков проекции	объекта
		call	GetCrdsProjection
		call	SetCrdCDEinExtSlot
		jp	objDestroyed2
; ---------------------------------------------------------------------------

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


; === sub_7F2D === CODE XREF: Use_J_Drive+41p
sub_7F2D:				; CODE XREF: Use_J_Drive+41p
					; Use_J_Drive+48p
		ld	bc, 20h	; ' '
		ld	h, (ix+8)
		ld	l, (ix+7)
		ld	d, l
		bit	7, h
		jr	nz, loc_7F42
		and	a
		sbc	hl, bc
		jr	c, loc_7F4C
		jr	loc_7F45
; ---------------------------------------------------------------------------

loc_7F42:				; CODE XREF: sub_7F2D+Cj
		add	hl, bc
		jr	c, loc_7F61

loc_7F45:				; CODE XREF: sub_7F2D+13j
		ld	(ix+8),	h
		ld	(ix+7),	l
		ret
; ---------------------------------------------------------------------------

loc_7F4C:				; CODE XREF: sub_7F2D+11j
		ld	hl, 2000h
		ld	e, (ix+6)
		and	a
		sbc	hl, de
		ld	a, 80h ; 'А'
		ld	(ix+8),	a
		ld	(ix+7),	h
		ld	(ix+6),	l
		ret
; ---------------------------------------------------------------------------

loc_7F61:				; CODE XREF: sub_7F2D+16j
		jp	sub_5FD5
; End of function sub_7F2D


; =============== S U B	R O U T	I N E =======================================

; Установка станции таргонов в межпланетном пространстве для последней подмиссии в войне с таргонами

SetTargonStation:			; CODE XREF: sub_5FD5:loc_6028p
					; CallFunctionObj_1:Set3_4Targonsp
		call	TestTargonStation ; определение	возможности установки станции таргонов в межпланетном пространстве для последней подмиссии в войне с таргонами
					; Вх:  ---
					; Вых: cy =1 станция установлена
					;      hl,bc,a - не определены
		jp	nz, IntRnd_A
		ld	hl, objCoriolis
		ld	b, 8

loc_7F6F:				; CODE XREF: SetTargonStation+Ej
		ld	(hl), 0
		inc	hl
		djnz	loc_7F6F
		ld	(hl), 1
		ld	l, 6Ah ; 'j'
		ld	(hl), 0F0h ; 'Ё'
		ld	a, 4
		ld	(PilotMission3), a ; =#06 при стыковке будет захват таргонами (вроде нереальная	ситуация)
					; =#04 в системе со танцией таргонов во	время войны
		pop	hl
		ret
; End of function SetTargonStation


; =============== S U B	R O U T	I N E =======================================




; === sub_7F81 === CODE XREF: CalcNewCrdObj_Plp
sub_7F81:				; CODE XREF: CalcNewCrdObj_Plp
		ld	a, (byte_D08E)	; <>0 есть Cobra Mk4
		and	a
		jr	z, loc_7F9D
		ld	a, 0BFh	; '┐'
		in	a, (0FEh)
		bit	3, a
		jr	nz, loc_7F9D
		ld	hl, HighAngleLR

loc_7F92:				; CODE XREF: sub_7F81+1Aj
		ld	a, (hl)
		add	a, a
		or	(hl)
		and	0BFh ; '┐'
		ld	(hl), a
		bit	1, l
		inc	hl
		jr	z, loc_7F92

loc_7F9D:				; CODE XREF: sub_7F81+4j sub_7F81+Cj
		ld	a, (HighAngleDU)
		ret
; End of function sub_7F81


; =============== S U B	R O U T	I N E =======================================




; === sub_7FA1 === CODE XREF: sub_E87E+39p
sub_7FA1:				; CODE XREF: sub_E87E+39p
		bit	5, (hl)
		ret	z
		ld	a, (PilotTargonWar) ; =#81 при сбросе миссий
					; 7,=1 начало войны, предолжение участвовать
					; 6-0, чем больше число	тем дальше от левого края карты	захват планет таргонами
					;
					; =#01 война закончена
		sub	2
		ret	c
		ld	a, (PilotRating2) ; >=#84 в трюме десант
					; 7,=1 война была или идёт
					; 6,=1 война закончилать
					; 3-1 -	ранг (=текущая стадия войны)
		cp	84h ; 'Д'
		ccf
		push	hl
		call	nc, TestPilotInZoneCombat ; Определяем находится ли пилот в зоне #3C (активных боёв)
					; Вх:  ---
					; Вых: cy =1 пилот в зоне #3C
					;	  =0 пилот в другой галактике галактике
					;	     война окончена/не начиналась
					;	     пилот в другой зоне
					;      hl,de,b,a - не определены
		pop	hl
		jr	nc, loc_7FB9
		xor	a
		ret
; ---------------------------------------------------------------------------

loc_7FB9:				; CODE XREF: sub_7FA1+14j
		set	0, (hl)
		inc	h
		dec	h
		ret
; End of function sub_7FA1


; =============== S U B	R O U T	I N E =======================================


CopyBuf2Screen:				; CODE XREF: sub_728D+13Fp RAM:755Ap ...
		ld	hl, 0C000h
		ld	de, 4020h
		call	sub_7FCD
		ld	hl, 0C100h
		ld	de, 4120h
; End of function CopyBuf2Screen


; =============== S U B	R O U T	I N E =======================================




; === sub_7FCD === CODE XREF: CopyBuf2Screen+6p
sub_7FCD:				; CODE XREF: CopyBuf2Screen+6p
		ld	bc, 1E0h
		ld	a, 0Fh

loc_7FD2:				; CODE XREF: sub_7FCD+13j
		ex	af, af'
		call	sub_B2A0
		ex	de, hl
		call	compDownScrAdr_HL
		ex	de, hl
		call	compDownScrAdr_HL
		ex	af, af'
		dec	a
		jr	nz, loc_7FD2
		ret
; End of function sub_7FCD

; ---------------------------------------------------------------------------
		ex	de, hl
		call	sub_7FE7

; =============== S U B	R O U T	I N E =======================================




; === sub_7FE7 === CODE XREF: PrnCompass+1Dp
sub_7FE7:				; CODE XREF: PrnCompass+1Dp
					; sub_DA2F+13p
		inc	h
		ld	a, h
		and	7
		call	z, compDownScrAdr_HL
		ret
; End of function sub_7FE7

; ---------------------------------------------------------------------------
		db 0C9h

; =============== S U B	R O U T	I N E =======================================

; a - номер объекта



; === sub_7FF0 === CODE XREF: sub_EF01p
sub_7FF0:				; CODE XREF: sub_EF01p
		ld	b, a
		cp	9		; cobra1
		ret	z
		cp	7
		ret	c
		cp	0Fh		; targon
		ret	z
		cp	12h
		ret	nc
		call	sub_8E64	; a=7,8,A,B,C,D,E,10,11
		jr	c, loc_8005
		ld	b, 6		; ;asteroid
		ret
; ---------------------------------------------------------------------------

loc_8005:				; CODE XREF: sub_7FF0+10j
		push	bc
		call	TestPilotInZoneWar ; Определяем	находится ли пилот/планета в заданной зоне войны
					; Вх:  (sp) =#00 зона оккупированная таргонами
					;	    =#3C зона линии фронта (активные боевые действия)
					;	    =#78 прифронтовая зона
					; Вых: cy =1 координаты	в пределах проверяемой зоны
					;	  =0 пилот в другой галактике галактике
					;	     война окончена/не начиналась
					;	     координаты	слишком	далеко от левого края
					;      hl,de,b,a - не определены
		inc	a		; db #3C
		jr	nc, loc_801E
		call	TestPilotInZoneCombat ;	Определяем находится ли	пилот в	зоне #3C (активных боёв)
					; Вх:  ---
					; Вых: cy =1 пилот в зоне #3C
					;	  =0 пилот в другой галактике галактике
					;	     война окончена/не начиналась
					;	     пилот в другой зоне
					;      hl,de,b,a - не определены
		jr	nc, loc_801A
		ld	a, (CoriolisNear)
		rrca
		call	nc, IntRnd_A
		jr	c, loc_801E

loc_801A:				; CODE XREF: sub_7FF0+1Fj
		pop	bc
		ld	b, 0Fh		; targon
		ret
; ---------------------------------------------------------------------------

loc_801E:				; CODE XREF: sub_7FF0+1Aj sub_7FF0+28j
		pop	bc
		ret
; End of function sub_7FF0


; =============== S U B	R O U T	I N E =======================================


CalcDistanceToCursor:			; CODE XREF: SearchPlanetNearCursor+13p
		push	af
		ld	hl, Pilot_CrdYTarget
		xor	(hl)
		jr	nz, loc_802C
		dec	hl
		ld	a, b
		xor	(hl)
		jr	z, loc_8033

loc_802C:				; CODE XREF: CalcDistanceToCursor+5j
		pop	af
		call	CalcDistanceToCursor2
		set	0, l
		ret
; ---------------------------------------------------------------------------

loc_8033:				; CODE XREF: CalcDistanceToCursor+Aj
		ld	hl, 0
		ld	(MinDistance), hl
		call	SavePlanetData
		pop	af
		pop	hl
		pop	bc
		jp	loc_DFA2
; End of function CalcDistanceToCursor


; =============== S U B	R O U T	I N E =======================================


PrnCocpit_Ifn:				; CODE XREF: GoDocking+3Cp sub_728D+Bp ...

; FUNCTION CHUNK AT 8459 SIZE 00000006 BYTES

		ld	de, 4020h
		ld	a, (de)
		inc	a
		ret	nz
		call	loc_94FE
		call	j_ClsAndPrintRamk
		ld	de, 1Fh
		ld	b, 17h
		ld	a, 0
		ld	hl, 5800h
		ld	(hl), a

loc_8059:				; CODE XREF: PrnCocpit_Ifn+1Bj
		add	hl, de
		ld	(hl), a
		inc	hl
		ld	(hl), a
		djnz	loc_8059
		add	hl, de
		ld	(hl), a
		ld	de, 5000h
		ld	hl, DataCocpit
		ld	bc, 800h
		ldir
		ld	de, 5A00h
		ld	hl, 5B00h
		ld	bc, 100h
		ldir
		call	sub_725C
		call	sub_7265
		jp	loc_8459
; End of function PrnCocpit_Ifn


; =============== S U B	R O U T	I N E =======================================


ClearScrBuffer:				; CODE XREF: sub_728D:loc_73C3p
					; sub_728D+25Ap ...

; FUNCTION CHUNK AT DA5F SIZE 00000005 BYTES

		di
		ld	hl, 0CFE0h
		ld	de, 0C020h
		ld	bc, 5720h

loc_808A:				; CODE XREF: ClearScrBuffer+10j
		dec	l
		dec	e
		dec	c
		ld	a, (bc)
		ld	(hl), a
		ld	(de), a
		jr	nz, loc_808A
		ld	(loc_DA5F+1), sp
		ld	sp, hl
		ex	de, hl
		add	hl, hl
		ld	e, l
		ld	d, l
		ld	b, 7Dh ; '}'

loc_809D:				; CODE XREF: ClearScrBuffer+2Fj
		inc	e
		push	de
		dec	e
		push	de
		push	de
		push	de
		push	de
		push	de
		push	de
		push	de
		push	de
		push	de
		push	de
		push	de
		push	de
		push	de
		push	de
		push	hl
		djnz	loc_809D
		jp	loc_DA5F
; End of function ClearScrBuffer

; ---------------------------------------------------------------------------

loc_80B4:				; CODE XREF: sub_EB2F+4p
		cp	0Eh
		jr	nc, loc_80BC
		ld	b, a
		add	a, a
		add	a, b
		ret
; ---------------------------------------------------------------------------

loc_80BC:				; CODE XREF: RAM:80B6j
		ld	hl, 667h
		xor	a
		ret
; ---------------------------------------------------------------------------

SetMaxTonnage:				; CODE XREF: sub_5EC5+1Bp
					; UseEscapePod+3Fp ...
		ld	a, (PilotLargeCargo) ; установка максимальной грузоподъемности
		ld	b, a
		dec	b
		ld	a, 5

loc_80C8:				; CODE XREF: RAM:80CBj
		add	a, 0Fh
		inc	b
		jr	nz, loc_80C8
		ld	(PilotFreeCargo), a
		ret
; ---------------------------------------------------------------------------
word_80D1:	dw 411Ah		; DATA XREF: sub_813Ew	sub_813E+18r

; =============== S U B	R O U T	I N E =======================================

; проверка нажатия (и соответствующий переход) клавиш 1, 2, 3, 4, I, O,	P, K, L, enter



; === sub_80D3 === CODE XREF: sub_728D:loc_747Ap
sub_80D3:				; CODE XREF: sub_728D:loc_747Ap
		ld	a, (FlgJumpDocking) ; =1 запрет	на обработку нажатий клавиш
					;   включено при: стыковке
					;   работает п/п отрисовки концентрический кругов
					;
					; 0,Flg_Interrupts
		and	a
		ret	nz
		ld	a, (ControlKey2)
		and	0Fh
		jr	z, loc_8119
		ld	c, 0
		srl	a
		jr	c, loc_8109
		inc	c
		srl	a
		jr	c, loc_80F0
		inc	c
		srl	a
		jr	c, loc_80F0
		inc	c

loc_80F0:				; CODE XREF: sub_80D3+15j sub_80D3+1Aj
		ld	a, (IndDanger)
		and	a
		jr	nz, loc_8104
		ld	a, 3
		add	a, c
		ld	c, a
		call	sub_815D
		sub	4
		call	sub_813E
		jr	loc_8136
; ---------------------------------------------------------------------------

loc_8104:				; CODE XREF: sub_80D3+21j
		call	sub_815D
		jr	loc_8136
; ---------------------------------------------------------------------------

loc_8109:				; CODE XREF: sub_80D3+10j
		ld	a, (IndDanger)
		and	a
		jr	nz, loc_8114
		call	LaunchDocking
; ---------------------------------------------------------------------------
		jr	loc_8136
; ---------------------------------------------------------------------------

loc_8114:				; CODE XREF: sub_80D3+3Aj
		call	sub_815D
		jr	loc_8136
; ---------------------------------------------------------------------------

loc_8119:				; CODE XREF: sub_80D3+Aj
		ld	a, (ControlKey1)
		and	3Fh ; '?'
		add	a, a
		ret	z
		add	a, a
		ld	bc, 607h

loc_8124:				; CODE XREF: sub_80D3+55j
		add	a, a
		jr	c, loc_812A
		inc	c
		djnz	loc_8124

loc_812A:				; CODE XREF: sub_80D3+52j
		call	sub_815D
		cp	7
		jr	c, loc_8136
		sub	4
		call	sub_813E

loc_8136:				; CODE XREF: sub_80D3+2Fj sub_80D3+34j ...
		xor	a
		ld	(ControlKey2), a
		ld	(ControlKey1), a
		ret
; End of function sub_80D3


; =============== S U B	R O U T	I N E =======================================




; === sub_813E === CODE XREF: LaunchDocking:loc_5E65j
sub_813E:				; CODE XREF: LaunchDocking:loc_5E65j
					; ControlKeyIfMapOn+23p ...
		ld	(word_80D1), sp
		nop
		nop
		nop
		ld	b, a
		add	a, a
		add	a, b
		ld	hl, FunctionKernel
		add	a, l
		ld	l, a
		jr	nc, loc_8150
		inc	h

loc_8150:				; CODE XREF: sub_813E+Fj
		ld	(loc_8153+1), hl

loc_8153:				; DATA XREF: sub_813E:loc_8150w
		call	sub_D021
		ld	sp, (word_80D1)
		jp	loc_94FE
; End of function sub_813E


; =============== S U B	R O U T	I N E =======================================




; === sub_815D === CODE XREF: sub_80D3+27p
sub_815D:				; CODE XREF: sub_80D3+27p
					; sub_80D3:loc_8104p ...

; FUNCTION CHUNK AT 819A SIZE 00000009 BYTES

		ld	a, (WhatOnScreen)
		ld	b, a
		cp	c
		jr	nz, loc_816C
		ld	a, c
		cp	7
		jr	c, loc_816C
		pop	hl
		jr	loc_8136
; ---------------------------------------------------------------------------

loc_816C:				; CODE XREF: sub_815D+5j sub_815D+Aj
		ld	a, (JumpDocking) ; =1 межпланетный прыжок/стыковка
					; =2 межгалактический прыжок
		and	a
		ld	a, c
		jr	nz, loc_819A

loc_8173:				; CODE XREF: sub_815D+3Fj
		ld	(WhatOnScreen),	a
		ld	a, b
		cp	4
		ld	a, c
		ret	nc
		cp	4
		ret	c
; End of function sub_815D


; =============== S U B	R O U T	I N E =======================================


SaveCocpitInBuffer:			; CODE XREF: sub_728D+18Ep
					; LaunchDocking+5Fp
		push	af
		call	CopyScaner2Scr
		pop	af
		ld	de, DataCocpit
		ld	hl, 5000h
		ld	bc, 800h
		ldir
		ld	de, 5B00h
		ld	hl, 5A00h
		ld	bc, 100h
		ldir
		ret
; End of function SaveCocpitInBuffer

; ---------------------------------------------------------------------------
; START	OF FUNCTION CHUNK FOR sub_815D

loc_819A:				; CODE XREF: sub_815D+14j
		cp	4
		jr	nc, loc_8173
		xor	a
		ld	(WhatOnScreen),	a
		ret
; END OF FUNCTION CHUNK	FOR sub_815D
; ---------------------------------------------------------------------------
		db    0


; === sub_8217 === CODE XREF: sub_5FD5+5Ep sub_5FD5+62p ...
sub_8217:				; CODE XREF: sub_5FD5+5Ep sub_5FD5+62p ...
		ld	(hl), d
		add	hl, de
		ld	(hl), d
		add	hl, de
		ld	(hl), a
		add	hl, de
		ret
; End of function sub_8217

; ---------------------------------------------------------------------------
; START	OF FUNCTION CHUNK FOR PrintCompass

loc_821E:				; CODE XREF: PrintCompass+6Cj
		ld	a, 1
		ld	(J_Drive_Ready3), a
		jr	nz, loc_8246
		call	CalcLenVectorCrd
; ---------------------------------------------------------------------------
		ld	a, h
		or	l
		jr	z, loc_8246
		ex	de, hl
		ld	hl, 0
		sbc	hl, de
		ex	de, hl
		ld	hl, 4000h
		and	a
		sbc	hl, de
		jp	c, loc_82D1
		ex	de, hl
		call	CalcTempAltitude
		ld	a, 0A0h	; 'а'
		sub	h
		ld	(Altitude), a

loc_8246:				; CODE XREF: PrintCompass+7Aj
					; PrintCompass+81j
		ld	a, 1
		ld	(CoriolisNear),	a
		ld	a, (J_Drive_Coriolis)
		inc	a
		ld	ix, objCoriolis
		jr	z, loc_827E
		call	CalcLenVectorCrd
; ---------------------------------------------------------------------------
		ld	a, h
		or	l
		jr	nz, loc_825D
		dec	hl

loc_825D:				; CODE XREF: PrintCompass+B1j
		ex	de, hl
		push	de
		call	CompasCrdNnear
; ---------------------------------------------------------------------------
		ld	(CompasPointerCrdX), a
		inc	ix
		inc	ix
		inc	ix
		pop	de
		call	CompasCrdNnear
; ---------------------------------------------------------------------------
		jr	loc_829C
; END OF FUNCTION CHUNK	FOR PrintCompass

; =============== S U B	R O U T	I N E =======================================

; Attributes: noreturn

CompasCrdNnear:				; CODE XREF: PrintCompass+B6p
					; PrintCompass+C3p ...
		xor	a
		ld	l, (ix+0)
		ld	h, (ix+1)
		jr	loc_82BB
; End of function CompasCrdNnear

; ---------------------------------------------------------------------------
; START	OF FUNCTION CHUNK FOR PrintCompass

loc_827A:				; CODE XREF: PrintCompass+5Aj
					; PrintCompass+69j
		nop
		call	sub_FD02

loc_827E:				; CODE XREF: PrintCompass+AAj
		inc	ix
		call	CalcLenVectorCrdH
; ---------------------------------------------------------------------------
		ld	a, h
		or	l
		jr	nz, loc_8288
		dec	hl

loc_8288:				; CODE XREF: PrintCompass+DCj
		ex	de, hl
		dec	ix
		push	de
		call	CompasCrdXplanet
; ---------------------------------------------------------------------------
		ld	(CompasPointerCrdX), a
		inc	ix
		inc	ix
		inc	ix
		pop	de
		call	CompasCrdXplanet
; ---------------------------------------------------------------------------

loc_829C:				; CODE XREF: PrintCompass+C6j
		ld	(CompasPointerCrdY), a
		ld	a, (ix+5)
		and	80h ; 'А'
		ld	a, 0
		jr	z, loc_82AA
		ld	a, 1

loc_82AA:				; CODE XREF: PrintCompass+FDj
		ld	(CompasPointerType), a
		jp	RenewCompass
; END OF FUNCTION CHUNK	FOR PrintCompass

; =============== S U B	R O U T	I N E =======================================

; Attributes: noreturn

CompasCrdXplanet:			; CODE XREF: PrintCompass+E3p
					; PrintCompass+F0p
		ld	a, (ix+0)
		ld	l, (ix+1)
		ld	h, (ix+2)
		res	7, h

loc_82BB:				; CODE XREF: CompasCrdNnear+7j
		call	Div_BC_HLAdivDE
; ---------------------------------------------------------------------------
		rl	b
		rla
		rl	b
		rla
		rl	b
		rla
		ld	h, (ix+2)
		bit	7, h
		jr	z, locret_82D0
		or	80h ; 'А'

locret_82D0:				; CODE XREF: CompasCrdXplanet+1Cj
		ret
; End of function CompasCrdXplanet

; ---------------------------------------------------------------------------
; START	OF FUNCTION CHUNK FOR PrintCompass

loc_82D1:				; CODE XREF: PrintCompass+36j
					; PrintCompass+90j
		ld	a, 1
		ld	(PlayerGameOver), a
		ret
; END OF FUNCTION CHUNK	FOR PrintCompass

; =============== S U B	R O U T	I N E =======================================


CalcTempAltitude:			; CODE XREF: PrintCompass+47p
					; PrintCompass+94p
		push	hl
		add	hl, hl
		pop	de
		srl	d
		rr	e
		add	hl, de
		ld	a, (IndDanger)
		ld	b, a
		ld	a, h
		cp	50h ; 'P'
		jr	c, loc_82F8
		cp	78h ; 'x'
		jr	nc, loc_82F4
		ld	a, b
		cp	63h ; 'c'
		ret	z
		ld	a, 2
		jr	loc_82FE
; ---------------------------------------------------------------------------

loc_82F4:				; CODE XREF: CalcTempAltitude+13j
		ld	a, 63h ; 'c'
		jr	loc_82FE
; ---------------------------------------------------------------------------

loc_82F8:				; CODE XREF: CalcTempAltitude+Fj
		ld	a, b
		cp	4
		ret	nz
		ld	a, 6

loc_82FE:				; CODE XREF: CalcTempAltitude+1Bj
					; CalcTempAltitude+1Fj
		ld	(IndDanger), a
		ret
; End of function CalcTempAltitude


; =============== S U B	R O U T	I N E =======================================


GetCrdObject:				; CODE XREF: CalcLenVectorCrdHp
					; CalcLenVectorCrdp
		ld	e, (ix+0)
		ld	d, (ix+1)
		ld	c, (ix+3)
		ld	b, (ix+4)
		ld	l, (ix+6)
		ld	h, (ix+7)
		ret
; End of function GetCrdObject


; =============== S U B	R O U T	I N E =======================================

; Attributes: noreturn

CalcLenVectorCrdH:			; CODE XREF: PrintCompass+D7p
		call	GetCrdObject
		res	7, h
		res	7, d
		res	7, b
		ld	a, 1
		jr	loc_8327
; End of function CalcLenVectorCrdH


; =============== S U B	R O U T	I N E =======================================

; Attributes: noreturn

CalcLenVectorCrd:			; CODE XREF: PrintCompass+1Dp
					; PrintCompass+7Cp ...

; FUNCTION CHUNK AT 838F SIZE 00000001 BYTES

		call	GetCrdObject
		ld	a, 1

loc_8327:				; CODE XREF: CalcLenVectorCrdH+Bj
					; CalcLenVectorCrd+14j
		ex	de, hl
		add	hl, hl
		ex	de, hl
		jr	c, loc_8338
		sla	c
		rl	b
		jr	c, loc_833C
		add	hl, hl
		jr	c, loc_8342
		add	a, a
		jr	nc, loc_8327

loc_8338:				; CODE XREF: CalcLenVectorCrd+8j
		rr	d
		jr	loc_8348
; ---------------------------------------------------------------------------

loc_833C:				; CODE XREF: CalcLenVectorCrd+Ej
		rr	b
		srl	d
		jr	loc_8348
; ---------------------------------------------------------------------------

loc_8342:				; CODE XREF: CalcLenVectorCrd+11j
		rr	h
		srl	b
		srl	d

loc_8348:				; CODE XREF: CalcLenVectorCrd+18j
					; CalcLenVectorCrd+1Ej
		ld	c, d
		push	af
		ld	a, h
		call	Sqr_DE_A
		ex	de, hl
		ld	a, b
		ld	b, 0
		call	Sqr_DE_A
		add	hl, de
		jr	nc, loc_8359
		inc	b

loc_8359:				; CODE XREF: CalcLenVectorCrd+34j
		ld	a, c
		call	Sqr_DE_A
		add	hl, de
		jr	nc, loc_8361
		inc	b

loc_8361:				; CODE XREF: CalcLenVectorCrd+3Cj
		ld	a, b
		and	a
		jr	z, loc_8377
		srl	b
		rr	h
		rr	l
		srl	b
		rr	h
		rr	l
		pop	af
		srl	a
		jr	z, loc_8386
		push	af

loc_8377:				; CODE XREF: CalcLenVectorCrd+41j
		call	j_Sqrt_H_HL
; ---------------------------------------------------------------------------
		pop	af

loc_837B:				; CODE XREF: CalcLenVectorCrd+61j
		srl	a
		jr	z, locret_838F
		srl	h
		rr	l
		jp	loc_837B
; ---------------------------------------------------------------------------

loc_8386:				; CODE XREF: CalcLenVectorCrd+52j
		call	j_Sqrt_H_HL
; End of function CalcLenVectorCrd

; ---------------------------------------------------------------------------
		add	hl, hl
		jr	nc, locret_838F
		ld	hl, 0
; START	OF FUNCTION CHUNK FOR CalcLenVectorCrd

locret_838F:				; CODE XREF: CalcLenVectorCrd+5Bj
					; RAM:838Aj
		ret
; END OF FUNCTION CHUNK	FOR CalcLenVectorCrd
; ---------------------------------------------------------------------------
; START	OF FUNCTION CHUNK FOR PrintCompass

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



; === sub_83FE === CODE XREF: ReCalcCrdObj_coriolis+Bp
sub_83FE:				; CODE XREF: ReCalcCrdObj_coriolis+Bp
					; sub_7370+6p ...
		ld	hl, 2E10h
		rlca
		ld	h, 71h ; 'q'    ; hl=SunCrd
		push	ix
		push	hl
		pop	ix
		ld	a, (ix+8)
		ld	l, (ix+5)
		ld	h, (ix+2)
		pop	ix
		add	hl, hl
		add	a, a
		add	a, h
		ret	c
		add	a, l
		ret
; End of function sub_83FE


; =============== S U B	R O U T	I N E =======================================




; === sub_841A === CODE XREF: sub_728D+3p
sub_841A:				; CODE XREF: sub_728D+3p
		ld	a, (PilotMission3) ; =#06 при стыковке будет захват таргонами (вроде нереальная	ситуация)
					; =#04 в системе со танцией таргонов во	время войны
		cp	4
		ld	a, 0D4h	; '╘'
		ret	z
		bit	7, (hl)
		ld	a, 0Dh
		ret	z
		pop	hl
		ret
; End of function sub_841A


; =============== S U B	R O U T	I N E =======================================




; === sub_8429 === CODE XREF: CollisionWithObj+91p
sub_8429:				; CODE XREF: CollisionWithObj+91p
		ld	a, (JumpDocking) ; =1 межпланетный прыжок/стыковка
					; =2 межгалактический прыжок
		rrca
		ret	c
		ld	a, (ix+1Ah)
		add	a, a
		cp	4
		ret	c
		ld	a, (ix+0)
		and	0C0h ; '└'
		or	(ix+3)
		and	0F0h ; 'Ё'
		ret	z
		scf
		ret
; End of function sub_8429


; =============== S U B	R O U T	I N E =======================================

; Attributes: thunk



; === sub_8442 === CODE XREF: PrintCompass+1F0p
sub_8442:				; CODE XREF: PrintCompass+1F0p
		jp	sub_6F29
; End of function sub_8442


; =============== S U B	R O U T	I N E =======================================


compDownScrAdr_HL:			; CODE XREF: sub_7FCD+Ap sub_7FCD+Ep ...
		ld	a, l
		add	a, 20h ; ' '
		ld	l, a
		ret	c
		ld	a, h
		sub	8
		ld	h, a
		ret
; End of function compDownScrAdr_HL


; =============== S U B	R O U T	I N E =======================================


compUpScrAdr_HL:			; CODE XREF: sub_9736+B3p
					; PrnSmallCircle+Dp
		ld	a, l
		sub	20h ; ' '
		ld	l, a
		ret	c
		ld	a, h
		add	a, 8
		ld	h, a
		ret
; End of function compUpScrAdr_HL

; ---------------------------------------------------------------------------
; START	OF FUNCTION CHUNK FOR PrnCocpit_Ifn

loc_8459:				; CODE XREF: PrnCocpit_Ifn+3Bj
		call	PrnCompass
		jp	loc_98A2
; END OF FUNCTION CHUNK	FOR PrnCocpit_Ifn
; ---------------------------------------------------------------------------
		db    1
		db 0FFh
		db  18h
		db 0EDh	; э
		db 0A0h	; а
		db 0EDh	; э
		db 0A0h	; а
		db 0EDh	; э
		db 0A0h	; а
		db  1Dh
		db  1Dh
		db  1Dh
		db 0EBh	; ы
		db  24h	; $
		db  7Ch	; |
		db 0E6h	; ц
		db    7
		db 0CCh	; ╠
		db  45h	; E
		db  84h	; Д
		db 0EBh	; ы
		db  10h
		db 0ECh	; ь
		db 0C9h	; ╔
		db    0
		db    0
		db  3Ch	; <
		db    0
		db  24h	; $
		db    0
		db  24h	; $
		db    0
		db  3Ch	; <
		db    0
; ---------------------------------------------------------------------------

loc_8481:				; CODE XREF: RAM:8487j
					; EnergyRecovery+4p
		push	af
		call	sub_9AE2
		pop	af
		inc	a
		jr	nz, loc_8481
		ret

; =============== S U B	R O U T	I N E =======================================

; hl = PilotEscapePod



; === sub_848A === CODE XREF: UseEscapePod+2Dp
sub_848A:				; CODE XREF: UseEscapePod+2Dp
		ld	l, 89h ; 'Й'
		inc	(hl)
		ld	(DemoMode), a	; =#00 при стыковке со станцией	таргонов
					;      после использования escape pod
					; =#01 включен демо режим
					; =#E0 после прыжка
		ret
; End of function sub_848A

; ---------------------------------------------------------------------------


; === sub_8540 === CODE XREF: UpdateAllIndicators+36p
sub_8540:				; CODE XREF: UpdateAllIndicators+36p
		ld	b, a
		xor	a
		ld	(loc_8596+1), a
		ld	(loc_85B8+1), a
		ld	a, 3
		ld	(loc_856C+1), a
		jr	loc_855B
; End of function sub_8540


; =============== S U B	R O U T	I N E =======================================




; === sub_854F === CODE XREF: UpdateAllIndicators+16p
sub_854F:				; CODE XREF: UpdateAllIndicators+16p
		ld	b, a
		ld	a, 0FFh
		ld	(loc_856C+1), a
		ld	(loc_8596+1), a
		ld	(loc_85B8+1), a

loc_855B:				; CODE XREF: sub_8540+Dj
		ld	a, (de)
		ld	l, a
		inc	de
		ld	a, (de)
		ld	h, a
		exx
		ld	b, 5
		exx
		ld	a, b
		and	a
		jp	z, loc_85E1
		ld	d, a
		and	7

loc_856C:				; DATA XREF: sub_8540+Aw sub_854F+3w
		ld	c, 0FFh
		ld	b, a
		ld	a, d
		jr	z, loc_8582
		ld	a, (byte_8523)
		dec	a
		jp	z, loc_85F5
		xor	a

loc_857A:				; CODE XREF: sub_854F+2Dj
		scf
		rra
		djnz	loc_857A

loc_857E:				; CODE XREF: sub_854F:loc_8603j
		ld	c, a
		ld	a, d
		add	a, 8

loc_8582:				; CODE XREF: sub_854F+21j
		and	0F8h ; '°'
		srl	a
		srl	a
		srl	a
		ld	b, a
		ld	a, 5
		sub	b
		ld	d, a
		jr	z, loc_85B5
		dec	b
		jr	z, loc_85CC
		ld	e, b

loc_8595:				; CODE XREF: sub_854F+64j
		push	hl

loc_8596:				; DATA XREF: sub_8540+2w sub_854F+6w
		ld	a, 0FFh

loc_8598:				; CODE XREF: sub_854F+4Bj
		ld	(hl), a
		inc	l
		djnz	loc_8598
		call	sub_8606
		inc	l
		ld	b, d
		dec	b
		jr	z, loc_85A9
		xor	a

loc_85A5:				; CODE XREF: sub_854F+58j
		ld	(hl), a
		inc	l
		djnz	loc_85A5

loc_85A9:				; CODE XREF: sub_854F+53j
		ld	(hl), 1
		inc	l
		pop	hl
		exx
		dec	b
		exx
		ret	z
		inc	h
		ld	b, e
		jr	loc_8595
; ---------------------------------------------------------------------------

loc_85B5:				; CODE XREF: sub_854F+40j
		dec	b
		ld	e, b

loc_85B7:				; CODE XREF: sub_854F+7Bj
		push	hl

loc_85B8:				; DATA XREF: sub_8540+5w sub_854F+9w
		ld	a, 0FFh

loc_85BA:				; CODE XREF: sub_854F+6Dj
		ld	(hl), a
		inc	l
		djnz	loc_85BA
		set	0, c
		call	sub_8606
		pop	hl
		exx
		dec	b
		exx
		ret	z
		inc	h
		ld	b, e
		jr	loc_85B7
; ---------------------------------------------------------------------------

loc_85CC:				; CODE XREF: sub_854F+43j
		ld	b, 5

loc_85CE:				; CODE XREF: sub_854F+8Fj
		ld	a, c
		push	hl
		ld	(hl), a
		inc	l
		xor	a
		ld	(hl), a
		inc	l
		ld	(hl), a
		inc	l
		ld	(hl), a
		inc	l
		or	1
		ld	(hl), a
		pop	hl
		inc	h
		djnz	loc_85CE
		ret
; ---------------------------------------------------------------------------

loc_85E1:				; CODE XREF: sub_854F+17j
		ld	b, 5

loc_85E3:				; CODE XREF: sub_854F+A3j
		xor	a
		push	hl
		ld	(hl), a
		inc	l
		ld	(hl), a
		inc	l
		ld	(hl), a
		inc	l
		ld	(hl), a
		inc	l
		or	1
		ld	(hl), a
		pop	hl
		inc	h
		djnz	loc_85E3
		ret
; ---------------------------------------------------------------------------

loc_85F5:				; CODE XREF: sub_854F+27j
		ld	a, 80h ; 'А'
		dec	b
		jr	z, loc_8603
		scf
		rra
		dec	b
		jr	z, loc_8603

loc_85FF:				; CODE XREF: sub_854F+B2j
		srl	a
		djnz	loc_85FF

loc_8603:				; CODE XREF: sub_854F+A9j sub_854F+AEj
		jp	loc_857E
; End of function sub_854F


; =============== S U B	R O U T	I N E =======================================




; === sub_8606 === CODE XREF: sub_854F+4Dp sub_854F+71p
sub_8606:				; CODE XREF: sub_854F+4Dp sub_854F+71p
		ld	(hl), c
		ld	a, (byte_8523)
		dec	a
		ret	nz
		ld	a, c
		and	0FEh ; '■'
		cp	80h ; 'А'
		jr	z, loc_8614
		ret
; ---------------------------------------------------------------------------

loc_8614:				; CODE XREF: sub_8606+Bj
		dec	l
		ld	(hl), 1
		inc	l
		ret
; End of function sub_8606

; ---------------------------------------------------------------------------
		db 0


; === sub_862B === CODE XREF: PrnCircle_Clip+14Ep
sub_862B:				; CODE XREF: PrnCircle_Clip+14Ep
					; PrnCircle_Clip+185p ...
		push	af
		exx
		ld	hl, 0
		ld	d, h
		ld	b, 0FFh
		exx
		ld	a, 1
		ld	(DirPrnCircle),	a
		ld	a, (FillCircle)
		and	a
		jp	nz, loc_9060
		jp	loc_8CF8
; End of function sub_862B


; =============== S U B	R O U T	I N E =======================================




; === sub_8643 === CODE XREF: PrnCircle_Clip+133p
sub_8643:				; CODE XREF: PrnCircle_Clip+133p
					; PrnCircle_Clip+19Cp ...

; FUNCTION CHUNK AT 8CF8 SIZE 00000033 BYTES
; FUNCTION CHUNK AT 8D4C SIZE 0000007E BYTES
; FUNCTION CHUNK AT 9060 SIZE 0000009B BYTES
; FUNCTION CHUNK AT 916B SIZE 00000089 BYTES

		push	af
		exx
		ld	hl, 0
		ld	d, h
		ld	b, 0FFh
		exx
		xor	a
		ld	(DirPrnCircle),	a
		ld	a, (FillCircle)
		and	a
		jp	nz, loc_9060
		jp	loc_8CF8
; End of function sub_8643


; =============== S U B	R O U T	I N E =======================================




; === sub_865A === CODE XREF: PrnCircle_Clip+14Bp
sub_865A:				; CODE XREF: PrnCircle_Clip+14Bp
					; PrnCircle_Clip+182p ...
		push	af
		exx
		ld	hl, 0
		ld	d, h
		ld	b, 0FFh
		exx
		ld	a, 1
		ld	(DirPrnCircle),	a
		ld	a, (FillCircle)
		and	a
		jp	nz, loc_91F4
		jp	loc_8BF8
; End of function sub_865A


; =============== S U B	R O U T	I N E =======================================




; === sub_8672 === CODE XREF: PrnCircle_Clip+130p
sub_8672:				; CODE XREF: PrnCircle_Clip+130p
					; PrnCircle_Clip+199p ...

; FUNCTION CHUNK AT 8BF8 SIZE 00000031 BYTES
; FUNCTION CHUNK AT 8C4A SIZE 0000008A BYTES
; FUNCTION CHUNK AT 91F4 SIZE 00000097 BYTES

		push	af
		exx
		ld	hl, 0
		ld	d, h
		ld	b, 0FFh
		exx
		xor	a
		ld	(DirPrnCircle),	a
		ld	a, (FillCircle)
		and	a
		jp	nz, loc_91F4
		jp	loc_8BF8
; End of function sub_8672


; =============== S U B	R O U T	I N E =======================================




; === sub_8689 === CODE XREF: PrnCircle_Clip+E3j
sub_8689:				; CODE XREF: PrnCircle_Clip+E3j
					; PrnCircle_Clip+290p
		push	af
		exx
		ld	hl, 0
		ld	d, h
		ld	b, 0FFh
		exx
		ld	a, 1
		ld	(DirPrnCircle),	a
		ld	a, (FillCircle)
		and	a
		jp	nz, loc_90FB
		jp	loc_8AF7
; End of function sub_8689


; =============== S U B	R O U T	I N E =======================================




; === sub_86A1 === CODE XREF: PrnCircle_Clip+DDp
sub_86A1:				; CODE XREF: PrnCircle_Clip+DDp
					; PrnCircle_Clip+293p

; FUNCTION CHUNK AT 8AF7 SIZE 00000033 BYTES
; FUNCTION CHUNK AT 8B4B SIZE 00000089 BYTES
; FUNCTION CHUNK AT 8BE4 SIZE 00000014 BYTES
; FUNCTION CHUNK AT 90FB SIZE 00000070 BYTES

		push	af
		exx
		ld	hl, 0
		ld	d, h
		ld	b, 0FFh
		exx
		xor	a
		ld	(DirPrnCircle),	a
		ld	a, (FillCircle)
		and	a
		jp	nz, loc_90FB
		jp	loc_8AF7
; End of function sub_86A1


; =============== S U B	R O U T	I N E =======================================




; === sub_86B8 === CODE XREF: PrnCircle_Clip+122p
sub_86B8:				; CODE XREF: PrnCircle_Clip+122p
					; PrnCircle_Clip+18Bp

; FUNCTION CHUNK AT 8A37 SIZE 00000017 BYTES
; FUNCTION CHUNK AT 9004 SIZE 0000002E BYTES

		ld	c, a
		ld	a, (FillCircle)
		and	a
		ld	a, c
		jp	nz, loc_9004
		jp	loc_8A37
; End of function sub_86B8


; =============== S U B	R O U T	I N E =======================================




; === sub_86C4 === CODE XREF: PrnCircle_Clip+13Dp
sub_86C4:				; CODE XREF: PrnCircle_Clip+13Dp
					; PrnCircle_Clip+174p

; FUNCTION CHUNK AT 8A4E SIZE 00000055 BYTES
; FUNCTION CHUNK AT 9032 SIZE 0000002E BYTES

		ld	c, a
		ld	a, (FillCircle)
		and	a
		ld	a, c
		jp	nz, loc_9032
		jp	loc_8A4E
; End of function sub_86C4


; =============== S U B	R O U T	I N E =======================================


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




; === sub_87D4 === CODE XREF: PrnCircle_Clip:loc_87C8p
sub_87D4:				; CODE XREF: PrnCircle_Clip:loc_87C8p
					; PrnCircle_Clip+E0p
		ld	bc, (word_8620)
		ld	de, (CircleFlg)
		ld	a, d
		neg
		exx
		ld	c, a
		ld	e, 0
		exx
		ld	a, b
		sub	d
		call	sub_89D4
		ld	a, (word_8620)
		ld	c, a
		ld	a, (word_8620+1)
		add	a, d
		jp	sub_89EF
; End of function sub_87D4

; ---------------------------------------------------------------------------
; START	OF FUNCTION CHUNK FOR PrnCircle_Clip

loc_87F4:				; CODE XREF: PrnCircle_Clip+6Aj
		ld	a, b
		add	a, 78h ; 'x'
		ld	b, a
		ld	(word_861E+1), a
		xor	a
		jr	loc_8802
; ---------------------------------------------------------------------------

loc_87FE:				; CODE XREF: PrnCircle_Clip+5Aj
					; DATA XREF: RAM:8ED7o
		ld	a, 88h ; 'И'
		add	a, b
		inc	a

loc_8802:				; CODE XREF: PrnCircle_Clip+10Ej
		ex	af, af'

loc_8803:				; DATA XREF: RAM:8ED9o
		ld	a, 78h ; 'x'
		add	a, b
		call	sub_8ACD
		ld	(byte_8622), a
		ld	b, a
		ld	a, (word_8620)
		call	sub_86B8
		call	sub_8A16
		call	sub_8A25
		ld	a, (CircleFlg)
		bit	0, a
		call	z, sub_8672
		call	nz, sub_8643
		ld	a, (byte_8622)
		ld	b, a
		ld	a, (word_8620)
		call	sub_86C4
		call	sub_8A16
		call	sub_8A25
		ld	a, (CircleFlg)
		bit	0, a
		call	z, sub_865A
		call	nz, sub_862B
		ret
; ---------------------------------------------------------------------------

loc_8840:				; CODE XREF: PrnCircle_Clip+CBj
		ld	a, b
		add	a, 78h ; 'x'
		ld	b, a
		ld	(word_861E+1), a
		xor	a
		jr	loc_884D
; ---------------------------------------------------------------------------

loc_884A:				; CODE XREF: PrnCircle_Clip+9Aj
		ld	a, b
		sub	77h ; 'w'

loc_884D:				; CODE XREF: PrnCircle_Clip+15Aj
		ex	af, af'
		ld	de, (CircleFlg)
		ld	a, 78h ; 'x'
		add	a, b
		call	sub_8ACD
		ld	(byte_8622), a
		sub	c
		ld	b, a
		bit	1, e
		jr	z, loc_8877
		xor	a
		call	sub_86C4
		call	sub_8A16
		call	sub_8A25
		ld	a, (CircleFlg)
		bit	0, a
		call	z, sub_865A
		call	nz, sub_862B
		ret
; ---------------------------------------------------------------------------

loc_8877:				; CODE XREF: PrnCircle_Clip+171j
		ld	a, 7Fh ; ''
		call	sub_86B8
		call	sub_8A16
		call	sub_8A25
		ld	a, (CircleFlg)
		bit	0, a
		call	z, sub_8672
		call	nz, sub_8643
		ret
; ---------------------------------------------------------------------------

loc_888E:				; CODE XREF: PrnCircle_Clip+6Dj
		ld	a, b
		add	a, 78h ; 'x'
		ld	(word_861E+1), a
		ld	b, a
		xor	a
		jr	loc_889B
; ---------------------------------------------------------------------------

loc_8898:				; CODE XREF: PrnCircle_Clip:loc_8745j
		ld	a, b

loc_8899:				; DATA XREF: RAM:8EDBo
		sub	77h ; 'w'

loc_889B:				; CODE XREF: PrnCircle_Clip+1A8j
		ex	af, af'
		bit	0, e
		jr	z, loc_88AC
		call	sub_8985
		call	sub_862B
		call	sub_8985
		jp	sub_8643
; ---------------------------------------------------------------------------

loc_88AC:				; CODE XREF: PrnCircle_Clip+1B0j
		call	sub_89A1
		call	sub_865A
		call	sub_89A1
		jp	sub_8672
; ---------------------------------------------------------------------------

loc_88B8:				; CODE XREF: PrnCircle_Clip:loc_87BCj
		ld	a, b
		add	a, 78h ; 'x'
		ld	(word_861E+1), a
		ld	b, a
		xor	a
		jr	loc_88C5
; ---------------------------------------------------------------------------

loc_88C2:				; CODE XREF: PrnCircle_Clip+9Dj
		ld	a, b
		sub	77h ; 'w'

loc_88C5:				; CODE XREF: PrnCircle_Clip+1D2j
		ex	af, af'
		ld	de, (CircleFlg)
		ld	a, c
		call	sub_8ACD
		ld	(byte_8622), a
		ld	a, c
		exx
		ld	e, a
		ld	a, (byte_8622)
		neg
		ld	c, a
		exx
		bit	0, e
		jr	z, loc_8910
		ld	a, (byte_8622)
		cpl
		add	a, 81h ; 'Б'
		add	a, b
		ld	c, 0
		bit	1, e
		jr	nz, loc_88EE
		ld	c, 7Fh ; ''

loc_88EE:				; CODE XREF: PrnCircle_Clip+1FCj
		call	sub_89D4
		ld	a, (FillCircle)
		and	a
		jp	nz, loc_8FB8
		ld	a, e
		and	1Fh
		cp	1Fh
		ret	z
		ld	a, e
		or	1Fh
		ld	l, a
		ld	c, 0
		ld	a, (CircleFlg)
		bit	1, a
		call	nz, sub_862B
		call	z, sub_8643
		ret
; ---------------------------------------------------------------------------

loc_8910:				; CODE XREF: PrnCircle_Clip+1EFj
		ld	a, (byte_8622)
		add	a, 80h ; 'А'
		sub	b
		cp	8
		ret	c
		ld	c, 0
		bit	1, e
		jr	nz, loc_8921
		ld	c, 7Fh ; ''

loc_8921:				; CODE XREF: PrnCircle_Clip+22Fj
		call	sub_89EF
		ld	a, l
		and	0E0h ; 'р'
		or	1
		ld	e, a
		ld	a, (FillCircle)
		and	a
		jp	nz, loc_8FCC
		ld	b, 0FFh
		ld	a, (CircleFlg)
		bit	1, a
		call	nz, sub_865A
		call	z, sub_8672
		ret
; ---------------------------------------------------------------------------

loc_893F:				; CODE XREF: PrnCircle_Clip+8Ej
		ld	de, (CircleFlg)
		ld	a, c
		call	sub_8ACD
		ld	(byte_8622), a
		ld	a, c
		exx
		ld	e, a
		ld	a, (byte_8622)
		neg
		ld	c, a
		exx
		xor	a
		bit	1, e
		jr	nz, loc_895B
		ld	a, 7Fh ; ''

loc_895B:				; CODE XREF: PrnCircle_Clip+269j
		ld	(word_8620), a
		ld	c, a
		ld	a, (byte_8622)
		ld	l, a
		ld	a, (word_8620+1)
		sub	l
		call	sub_89D4
		ld	a, (word_8620)
		ld	c, a
		ld	a, (byte_8622)
		ld	l, a
		ld	a, (word_8620+1)
		add	a, l
		call	sub_89EF
		ld	a, (CircleFlg)
		bit	1, a
		call	nz, sub_8689
		call	z, sub_86A1
		ret
; END OF FUNCTION CHUNK	FOR PrnCircle_Clip

; =============== S U B	R O U T	I N E =======================================




; === sub_8985 === CODE XREF: PrnCircle_Clip+1B2p
sub_8985:				; CODE XREF: PrnCircle_Clip+1B2p
					; PrnCircle_Clip+1B8p

; FUNCTION CHUNK AT 8FDA SIZE 00000009 BYTES

		call	sub_89C3
		ld	a, (word_8620)
		ld	c, a
		ld	a, 80h ; 'А'
		add	a, b
		sub	d
		call	sub_89D4
		ld	a, (FillCircle)
		and	a
		jp	nz, loc_8FDA
		ld	a, e
		or	1Fh
		ld	l, a
		ld	c, 0
		ret
; End of function sub_8985


; =============== S U B	R O U T	I N E =======================================




; === sub_89A1 === CODE XREF: PrnCircle_Clip:loc_88ACp
sub_89A1:				; CODE XREF: PrnCircle_Clip:loc_88ACp
					; PrnCircle_Clip+1C4p
		call	sub_89C3
		ld	a, (word_8620)
		ld	c, a
		ld	a, 80h ; 'А'
		add	a, d
		sub	b
		call	sub_89EF
		ld	a, l
		and	0E0h ; 'р'

loc_89B2:				; DATA XREF: RAM:8EDFo
		or	1
		ld	e, a
		ld	b, a
		ld	a, (FillCircle)
		and	a
		ld	a, b
		jr	nz, loc_89C0
		ld	b, 0FFh
		ret
; ---------------------------------------------------------------------------

loc_89C0:				; CODE XREF: sub_89A1+1Aj
		ld	b, 80h ; 'А'
		ret
; End of function sub_89A1


; =============== S U B	R O U T	I N E =======================================




; === sub_89C3 === CODE XREF: sub_8985p	sub_89A1p
sub_89C3:				; CODE XREF: sub_8985p	sub_89A1p
		ld	bc, (word_861E)
		ld	de, (CircleFlg)
		ld	a, d
		neg
		exx
		ld	c, a
		ld	e, 0
		exx
		ret
; End of function sub_89C3


; =============== S U B	R O U T	I N E =======================================




; === sub_89D4 === CODE XREF: sub_87D4+12p
sub_89D4:				; CODE XREF: sub_87D4+12p
					; PrnCircle_Clip:loc_88EEp ...

; FUNCTION CHUNK AT 8FE3 SIZE 0000000A BYTES

		ld	b, c
		ld	c, a
		ld	a, b
		call	22B0h

loc_89DA:				; DATA XREF: RAM:8EE1o	sub_8F88w ...
		set	7, h
		ld	b, a
		ld	a, (FillCircle)
		and	a
		ld	a, b
		jp	nz, loc_8FE3
		ld	b, 0FFh
		inc	a

loc_89E8:				; CODE XREF: sub_89D4+17j
		srl	b
		dec	a
		jr	nz, loc_89E8
		ld	e, l
		ret
; End of function sub_89D4


; =============== S U B	R O U T	I N E =======================================




; === sub_89EF === CODE XREF: sub_87D4+1Dj
sub_89EF:				; CODE XREF: sub_87D4+1Dj
					; PrnCircle_Clip:loc_8921p ...

; FUNCTION CHUNK AT 8FED SIZE 0000000C BYTES

		push	af
		ld	a, b
		ld	(byte_8627), a
		pop	af
		ld	b, c
		ld	c, a
		ld	a, b
		call	22B0h

loc_89FB:				; DATA XREF: RAM:8EE5o	sub_8F88+3w ...
		set	7, h
		sub	8
		neg
		ld	c, a
		ld	a, (FillCircle)
		and	a
		ld	a, c
		jp	nz, loc_8FED
		ld	c, 0FFh

loc_8A0C:				; CODE XREF: sub_89EF+20j
		sla	c
		dec	a
		jr	nz, loc_8A0C
		ld	a, (byte_8627)
		ld	b, a
		ret
; End of function sub_89EF


; =============== S U B	R O U T	I N E =======================================




; === sub_8A16 === CODE XREF: PrnCircle_Clip+125p
sub_8A16:				; CODE XREF: PrnCircle_Clip+125p
					; PrnCircle_Clip+140p ...
		ld	bc, (word_861E)

loc_8A1A:				; DATA XREF: RAM:8EE9o
		ld	a, 88h ; 'И'
		sub	b
		exx
		ld	c, a
		ld	a, (byte_8622)
		ld	e, a
		exx
		ret
; End of function sub_8A16


; =============== S U B	R O U T	I N E =======================================




; === sub_8A25 === CODE XREF: PrnCircle_Clip+128p
sub_8A25:				; CODE XREF: PrnCircle_Clip+128p
					; PrnCircle_Clip+143p ...

; FUNCTION CHUNK AT 8FF9 SIZE 0000000B BYTES

		ld	a, (FillCircle)
		and	a
		jp	nz, loc_8FF9
		ld	a, l
		and	0E0h ; 'р'
		or	1
		ld	e, a
		ld	c, 0
		ld	b, 0FFh
		ret
; End of function sub_8A25

; ---------------------------------------------------------------------------
; START	OF FUNCTION CHUNK FOR sub_86B8

loc_8A37:				; CODE XREF: sub_86B8+9j
		ld	hl, 257Ch
		ld	(loc_8A85), hl
		ld	hl, 20D6h
		ld	(loc_8A92), hl
		ld	hl, 8C6h
		ld	(loc_8A98), hl

loc_8A49:				; DATA XREF: PrnExplodeObj+7w
					; PrnExplodeObj+69w
		ld	hl, 30C0h
		jr	loc_8A63
; END OF FUNCTION CHUNK	FOR sub_86B8
; ---------------------------------------------------------------------------
; START	OF FUNCTION CHUNK FOR sub_86C4

loc_8A4E:				; CODE XREF: sub_86C4+9j
		ld	hl, 7C24h
		ld	(loc_8A85), hl
		ld	hl, 20C6h
		ld	(loc_8A92), hl
		ld	hl, 8D6h
		ld	(loc_8A98), hl

loc_8A60:				; DATA XREF: sub_8F88+Cw
		ld	hl, 38D0h

loc_8A63:				; CODE XREF: sub_86B8+394j
		ld	(loc_8A9D+1), hl
		push	de
		push	af
		push	bc
		ld	c, 0F8h	; '°'
		call	22B0h

loc_8A6E:				; DATA XREF: sub_8F88+6w
		set	7, h
		pop	bc
		push	bc
		dec	b
		inc	b
		jr	z, loc_8A8D

loc_8A76:				; CODE XREF: sub_86C4:loc_8A8Bj
		push	hl
		dec	hl
		ld	(hl), 0FFh
		ld	d, h
		ld	e, l
		dec	de
		push	bc
		ld	bc, 1Dh
		lddr
		pop	bc
		pop	hl

loc_8A85:				; DATA XREF: sub_86B8+382w
					; sub_86C4+38Dw
		nop
		nop
		and	7
		jr	z, loc_8A91

loc_8A8B:				; CODE XREF: sub_86C4+3D7j
					; sub_86C4+3DBj
		djnz	loc_8A76

loc_8A8D:				; CODE XREF: sub_86C4+3B0j
					; sub_86C4+3DDj
		pop	bc
		pop	af
		pop	de
		ret
; ---------------------------------------------------------------------------

loc_8A91:				; CODE XREF: sub_86C4+3C5j
		ld	a, l

loc_8A92:				; DATA XREF: sub_86B8+388w
					; sub_86C4+393w
		nop
		nop
		ld	l, a
		ld	a, h
		jr	c, loc_8A9D

loc_8A98:				; DATA XREF: sub_86B8+38Ew
					; sub_86C4+399w
		add	a, 8
		ld	h, a
		jr	loc_8A8B
; ---------------------------------------------------------------------------

loc_8A9D:				; CODE XREF: sub_86C4+3D2j
					; DATA XREF: sub_86C4:loc_8A63w
		cp	0
		jr	nc, loc_8A8B
		jr	loc_8A8D
; END OF FUNCTION CHUNK	FOR sub_86C4

; =============== S U B	R O U T	I N E =======================================




; === sub_8AA3 === CODE XREF: OffTargetMissiles+1Ep
sub_8AA3:				; CODE XREF: OffTargetMissiles+1Ep
		ld	a, (ix+23h)
		cp	25h ; '%'
		ret	nz
		push	hl
		ld	hl, PilotAlienComputer ; 0,=1 есть Alien Computer
		inc	(hl)
		inc	(hl)
		pop	hl
		ret	nz
		ld	a, 4		; cargo
		call	SetObjInSlot	; a - номер объекта для	установки
		dec	(ix+26h)
		ret
; End of function sub_8AA3

; ---------------------------------------------------------------------------


; === sub_8ACD === CODE XREF: PrnCircle_Clip+118p
sub_8ACD:				; CODE XREF: PrnCircle_Clip+118p
					; PrnCircle_Clip+167p ...
		push	bc
		push	de
		push	hl
		push	af
		ld	a, d
		call	Sqr_DE_A
		ld	h, d
		ld	l, e
		pop	af
		call	Sqr_DE_A
		sbc	hl, de
		ld	b, 80h ; 'А'
		xor	a
		ld	(word_8625), hl

loc_8AE3:				; CODE XREF: sub_8ACD+24j
		ld	hl, (word_8625)
		add	a, b
		call	Sqr_DE_A
		sbc	hl, de
		jr	nc, loc_8AEF
		sub	b

loc_8AEF:				; CODE XREF: sub_8ACD+1Fj
		srl	b
		jr	nc, loc_8AE3
		pop	hl
		pop	de
		pop	bc
		ret
; End of function sub_8ACD

; ---------------------------------------------------------------------------
; START	OF FUNCTION CHUNK FOR sub_86A1

loc_8AF7:				; CODE XREF: sub_8689+15j sub_86A1+14j
		ld	a, e
		sub	l
		jr	nc, loc_8B0D
		ld	a, (DirPrnCircle)
		and	a
		jr	nz, loc_8B08

loc_8B01:				; DATA XREF: sub_8F88+22w
		ld	a, 0BFh	; '┐'
		cp	h
		jr	c, loc_8B0F
		jr	loc_8B0D
; ---------------------------------------------------------------------------

loc_8B08:				; CODE XREF: sub_86A1+45Ej
					; DATA XREF: sub_8F88+17w
		ld	a, 0CFh	; '╧'
		cp	h
		jr	nc, loc_8B0F

loc_8B0D:				; CODE XREF: sub_86A1+458j
					; sub_86A1+465j
		pop	af
		ret
; ---------------------------------------------------------------------------

loc_8B0F:				; CODE XREF: sub_86A1+463j
					; sub_86A1+46Aj
		ld	(word_8623), sp

loc_8B13:				; CODE XREF: sub_86A1:loc_8B6Dj
		ld	a, (hl)
		or	c
		ld	(hl), a
		ld	d, l
		ld	l, e
		ld	a, (hl)
		or	b
		ld	(hl), a
		ld	a, d
		sub	e
		dec	a
		srl	a
		jr	nc, loc_8B25
		inc	l
		ld	(hl), 0FFh

loc_8B25:				; CODE XREF: sub_86A1+47Fj
		call	sub_5E36
		jr	loc_8B4B
; END OF FUNCTION CHUNK	FOR sub_86A1

; =============== S U B	R O U T	I N E =======================================




; === sub_8B2A === CODE XREF: sub_79F9+5p sub_79F9+Cp
sub_8B2A:				; CODE XREF: sub_79F9+5p sub_79F9+Cp
		ld	e, 5
		ex	af, af'
		ld	a, (byte_D08E)	; <>0 есть Cobra Mk4
		cp	e
		jr	c, loc_8B3C
		ex	af, af'
		in	a, (0FEh)

loc_8B36:				; CODE XREF: sub_8B2A+10j
		inc	c
		rrca
		ret	nc
		dec	e
		jr	nz, loc_8B36

loc_8B3C:				; CODE XREF: sub_8B2A+7j
		ld	a, (hl)
		inc	a
		ret
; End of function sub_8B2A


; =============== S U B	R O U T	I N E =======================================




; === sub_8B3F === CODE XREF: PrintCompass+4Bp
sub_8B3F:				; CODE XREF: PrintCompass+4Bp
		ld	a, (InFreeSpace)
		and	a
		ld	a, h
		jr	z, loc_8B47
		xor	a

loc_8B47:				; CODE XREF: sub_8B3F+5j
		ld	(Temperature), a
		ret
; End of function sub_8B3F

; ---------------------------------------------------------------------------
; START	OF FUNCTION CHUNK FOR sub_86A1

loc_8B4B:				; CODE XREF: sub_86A1+487j
		jr	nz, loc_8B52
		call	sub_8DEE
		jr	loc_8B55
; ---------------------------------------------------------------------------

loc_8B52:				; CODE XREF: sub_86A1:loc_8B4Bj
		call	loc_8E08

loc_8B55:				; CODE XREF: sub_86A1+4AFj
		exx
		add	hl, de
		inc	e
		add	hl, de
		bit	7, h
		exx
		jr	nz, loc_8B6D
		srl	b
		jr	nc, loc_8B72

loc_8B62:				; CODE XREF: sub_86A1+4D8j
		sla	c
		jr	nc, loc_8B7B

loc_8B66:				; CODE XREF: sub_86A1+4E1j
		exx
		add	hl, bc
		inc	c

loc_8B69:				; CODE XREF: sub_8672+611j
					; sub_8643+73Bj
		add	hl, bc
		bit	7, h
		exx

loc_8B6D:				; CODE XREF: sub_86A1+4BBj
		jp	nz, loc_8B13
		jr	loc_8B9C
; ---------------------------------------------------------------------------

loc_8B72:				; CODE XREF: sub_86A1+4BFj
		inc	e
		ld	a, l
		sub	e
		jr	z, loc_8B84
		ld	b, 7Fh ; ''
		jr	loc_8B62
; ---------------------------------------------------------------------------

loc_8B7B:				; CODE XREF: sub_86A1+4C3j
		dec	l
		ld	a, l
		sub	e
		jr	z, loc_8B84
		ld	c, 0FEh	; '■'
		jr	loc_8B66
; ---------------------------------------------------------------------------

loc_8B84:				; CODE XREF: sub_86A1+4D4j
					; sub_86A1+4DDj ...
		ld	sp, (word_8623)
		pop	af
		ret
; ---------------------------------------------------------------------------

loc_8B8A:				; CODE XREF: sub_86A1+4FDj
		inc	e
		ld	a, l
		sub	e
		jr	z, loc_8B84
		ld	b, 7Fh ; ''
		jr	loc_8BA0
; ---------------------------------------------------------------------------

loc_8B93:				; CODE XREF: sub_86A1+501j
		dec	l
		ld	a, l
		sub	e
		jr	z, loc_8B84
		ld	c, 0FEh	; '■'
		jr	loc_8BA4
; ---------------------------------------------------------------------------

loc_8B9C:				; CODE XREF: sub_86A1+4CFj
					; sub_86A1+50Cj ...
		srl	b
		jr	nc, loc_8B8A

loc_8BA0:				; CODE XREF: sub_86A1+4F0j
		sla	c
		jr	nc, loc_8B93

loc_8BA4:				; CODE XREF: sub_86A1+4F9j
		exx
		add	hl, bc
		inc	c
		jr	z, loc_8B84

loc_8BA9:				; CODE XREF: sub_8672+61Bj
					; sub_8643+73Fj
		add	hl, bc
		bit	7, h
		exx
		jr	z, loc_8B9C
		exx
		add	hl, de
		inc	e
		add	hl, de
		exx
		ld	a, (hl)
		or	c
		ld	(hl), a
		ld	d, l
		ld	l, e
		ld	a, (hl)
		or	b
		ld	(hl), a
		ld	a, d
		sub	e
		dec	a
		srl	a
		jr	nc, loc_8BC6
		inc	l
		ld	(hl), 0FFh

loc_8BC6:				; CODE XREF: sub_86A1+520j
		ld	l, d
		sub	11h
		cpl
		ld	(loc_8BD2+1), a
		ld	sp, DataFillCircle
		pop	af
		ld	sp, hl

loc_8BD2:				; DATA XREF: sub_86A1+529w
		jr	loc_8BE4
; END OF FUNCTION CHUNK	FOR sub_86A1
; ---------------------------------------------------------------------------
		push	af
		push	af
		push	af
		push	af
		push	af
		push	af
		push	af
		push	af
		push	af
		push	af
		push	af
		push	af
		push	af
		push	af
		push	af
		push	af
; START	OF FUNCTION CHUNK FOR sub_86A1

loc_8BE4:				; CODE XREF: sub_86A1:loc_8BD2j
		ld	a, (DirPrnCircle)
		and	a
		ld	sp, (word_8623)
		jr	nz, loc_8BF3
		call	sub_8DEE
		jr	loc_8B9C
; ---------------------------------------------------------------------------

loc_8BF3:				; CODE XREF: sub_86A1+54Bj
		call	loc_8E08
		jr	loc_8B9C
; END OF FUNCTION CHUNK	FOR sub_86A1
; ---------------------------------------------------------------------------
; START	OF FUNCTION CHUNK FOR sub_8672

loc_8BF8:				; CODE XREF: sub_865A+15j sub_8672+14j
		ld	a, l
		and	1Fh
		cp	20h ; ' '
		jr	nc, loc_8C11
		ld	a, (DirPrnCircle)
		and	a
		jr	nz, loc_8C0C

loc_8C05:				; DATA XREF: sub_8F88+25w
		ld	a, 0BFh	; '┐'
		cp	h
		jr	c, loc_8C13
		jr	loc_8C11
; ---------------------------------------------------------------------------

loc_8C0C:				; CODE XREF: sub_8672+591j
					; DATA XREF: sub_8F88+1Aw
		ld	a, 0CFh	; '╧'
		cp	h
		jr	nc, loc_8C13

loc_8C11:				; CODE XREF: sub_8672+58Bj
					; sub_8672+598j
		pop	af
		ret
; ---------------------------------------------------------------------------

loc_8C13:				; CODE XREF: sub_8672+596j
					; sub_8672+59Dj
		ld	(word_8623), sp

loc_8C17:				; CODE XREF: sub_8672:loc_8C6Dj
		ld	a, (hl)
		or	c
		ld	(hl), a
		ld	d, l
		ld	l, e
		ld	a, d
		sub	e
		srl	a
		jr	nc, loc_8C24
		ld	(hl), 0FFh

loc_8C24:				; CODE XREF: sub_8672+5AEj
		call	sub_5E36
		jr	loc_8C4A
; END OF FUNCTION CHUNK	FOR sub_8672

; =============== S U B	R O U T	I N E =======================================




; === sub_8C29 === CODE XREF: OffTargetMissiles+1Bp
sub_8C29:				; CODE XREF: OffTargetMissiles+1Bp
		bit	0, (iy+23h)
		jr	z, loc_8C34
		set	4, (iy+20h)
		ret
; ---------------------------------------------------------------------------

loc_8C34:				; CODE XREF: sub_8C29+4j
		ld	(iy+26h), 3
		ld	(iy+25h), a
		ret
; End of function sub_8C29

; ---------------------------------------------------------------------------

loc_8C3C:				; CODE XREF: PrnCountdownJump+90p
		call	TestPilotInZoneWar ; шанс перехвата таргонами
; ---------------------------------------------------------------------------
		db 78h
; ---------------------------------------------------------------------------
		call	c, IntRnd_A
		ld	a, (RndWord)
		ret	nc
		ld	a, 0F9h	; '∙'
		ret
; ---------------------------------------------------------------------------
; START	OF FUNCTION CHUNK FOR sub_8672

loc_8C4A:				; CODE XREF: sub_8672+5B5j
		jr	nz, loc_8C51
		call	sub_8DEE
		jr	loc_8C54
; ---------------------------------------------------------------------------

loc_8C51:				; CODE XREF: sub_8672:loc_8C4Aj
		call	loc_8E08

loc_8C54:				; CODE XREF: sub_8672+5DDj
		exx
		add	hl, de
		inc	e
		add	hl, de
		bit	7, h
		exx
		jr	nz, loc_8C6D
		sla	c
		jr	nc, loc_8C72

loc_8C61:				; CODE XREF: sub_8672+608j
		exx
		add	hl, bc
		inc	c
		ex	af, af'
		cp	c
		jr	z, loc_8C7C
		ex	af, af'
		add	hl, bc
		bit	7, h
		exx

loc_8C6D:				; CODE XREF: sub_8672+5E9j
		jp	nz, loc_8C17
		jr	loc_8CA0
; ---------------------------------------------------------------------------

loc_8C72:				; CODE XREF: sub_8672+5EDj
		dec	l
		ld	a, l
		sub	e
		inc	a
		jr	z, loc_8C90
		ld	c, 0FEh	; '■'
		jr	loc_8C61
; ---------------------------------------------------------------------------

loc_8C7C:				; CODE XREF: sub_8672+5F4j
		ex	af, af'
		exx
		ld	a, l
		sub	e
		exx
		jr	z, loc_8C90
		jp	loc_8B69
; ---------------------------------------------------------------------------

loc_8C86:				; CODE XREF: sub_8672+639j
		ex	af, af'
		exx
		ld	a, l
		sub	e
		exx
		jr	z, loc_8C90
		jp	loc_8BA9
; ---------------------------------------------------------------------------

loc_8C90:				; CODE XREF: sub_8672+604j
					; sub_8672+60Fj ...
		ld	sp, (word_8623)
		pop	af
		ret
; ---------------------------------------------------------------------------

loc_8C96:				; CODE XREF: sub_8672+630j
		dec	l
		ld	a, l
		sub	e
		inc	a
		jr	z, loc_8C90
		ld	c, 0FEh	; '■'
		jr	loc_8CA4
; ---------------------------------------------------------------------------

loc_8CA0:				; CODE XREF: sub_8672+5FEj
					; sub_8672+640j ...
		sla	c
		jr	nc, loc_8C96

loc_8CA4:				; CODE XREF: sub_8672+62Cj
		exx
		add	hl, bc
		inc	c
		jr	z, loc_8C90
		ex	af, af'
		cp	c
		jr	z, loc_8C86
		ex	af, af'
		add	hl, bc
		bit	7, h
		exx
		jr	z, loc_8CA0
		exx
		add	hl, de
		inc	e
		add	hl, de
		exx
		ld	a, (hl)
		or	c
		ld	(hl), a
		ld	d, l
		ld	l, e
		ld	a, d
		sub	e
		srl	a
		jr	nc, loc_8CC6
		ld	(hl), 0FFh

loc_8CC6:				; CODE XREF: sub_8672+650j
		ld	l, d
		sub	11h
		cpl
		ld	(loc_8CD2+1), a
		ld	sp, DataFillCircle
		pop	af
		ld	sp, hl

loc_8CD2:				; CODE XREF: sub_8672:loc_8CD2j
					; DATA XREF: sub_8672+658w
		jr	$
; END OF FUNCTION CHUNK	FOR sub_8672
; ---------------------------------------------------------------------------
		push	af
		push	af
		push	af
		push	af
		push	af
		push	af
		push	af
		push	af
		push	af
		push	af
		push	af
		push	af
		push	af
		push	af
		push	af
		push	af
		ld	a, (DirPrnCircle)
		and	a
		ld	sp, (word_8623)
		jr	nz, loc_8CF3
		call	sub_8DEE
		jr	loc_8CA0
; ---------------------------------------------------------------------------

loc_8CF3:				; CODE XREF: RAM:8CECj
		call	loc_8E08
		jr	loc_8CA0
; ---------------------------------------------------------------------------
; START	OF FUNCTION CHUNK FOR sub_8643

loc_8CF8:				; CODE XREF: sub_862B+15j sub_8643+14j
		ld	a, e
		and	1Fh
		cp	20h ; ' '
		jr	nc, loc_8D11
		ld	a, (DirPrnCircle)
		and	a
		jr	nz, loc_8D0C

loc_8D05:				; DATA XREF: sub_8F88+1Fw
		ld	a, 0BFh	; '┐'
		cp	h
		jr	c, loc_8D13
		jr	loc_8D11
; ---------------------------------------------------------------------------

loc_8D0C:				; CODE XREF: sub_8643+6C0j
					; DATA XREF: sub_8F88+14w
		ld	a, 0CFh	; '╧'
		cp	h
		jr	nc, loc_8D13

loc_8D11:				; CODE XREF: sub_8643+6BAj
					; sub_8643+6C7j
		pop	af
		ret
; ---------------------------------------------------------------------------

loc_8D13:				; CODE XREF: sub_8643+6C5j
					; sub_8643+6CCj
		ld	(word_8623), sp

loc_8D17:				; CODE XREF: sub_8643:loc_8D6Fj
		ld	d, l
		ld	l, e
		ld	a, (hl)
		or	b
		ld	(hl), a
		ld	a, d
		sub	e
		dec	a
		srl	a
		jr	nc, loc_8D26
		inc	l
		ld	(hl), 0FFh

loc_8D26:				; CODE XREF: sub_8643+6DEj
		call	sub_5E36
		jr	loc_8D4C
; END OF FUNCTION CHUNK	FOR sub_8643

; =============== S U B	R O U T	I N E =======================================

; Определение главной диагонали
; Вых: z=1 на главной диагонале

TestMainDiagonal:			; CODE XREF: ReCalcCrdObj_coriolis+6p
					; sub_7370+Fp
		ld	hl, Pilot_CrdYSystem
		ld	a, (hl)
		inc	hl
		inc	hl
		sub	(hl)
		jr	nc, loc_8D35
		cpl

loc_8D35:				; CODE XREF: TestMainDiagonal+7j
		and	0F8h ; '°'
		ret
; End of function TestMainDiagonal

; ---------------------------------------------------------------------------

loc_8D38:				; CODE XREF: RAM:EFA0p
		ld	hl, 306h
		ld	(CrdXPrnSymbol), hl
		ld	a, (byte_D08E)	; <>0 есть Cobra Mk4
		and	a
		ld	a, 0F9h	; '∙'
		call	nz, PrintMessage1_A
		ld	a, 4
		jp	loc_E2C8
; ---------------------------------------------------------------------------
; START	OF FUNCTION CHUNK FOR sub_8643

loc_8D4C:				; CODE XREF: sub_8643+6E6j
		jr	nz, loc_8D53
		call	sub_8DEE
		jr	loc_8D56
; ---------------------------------------------------------------------------

loc_8D53:				; CODE XREF: sub_8643:loc_8D4Cj
		call	loc_8E08

loc_8D56:				; CODE XREF: sub_8643+70Ej
		exx
		add	hl, de
		inc	e
		add	hl, de
		bit	7, h
		exx
		jr	nz, loc_8D6F
		srl	b
		jr	nc, loc_8D74

loc_8D63:				; CODE XREF: sub_8643+738j
		exx
		add	hl, bc
		inc	c
		ex	af, af'
		cp	c
		jr	z, loc_8D7D
		ex	af, af'
		add	hl, bc
		bit	7, h
		exx

loc_8D6F:				; CODE XREF: sub_8643+71Aj
		jp	nz, loc_8D17
		jr	loc_8D94
; ---------------------------------------------------------------------------

loc_8D74:				; CODE XREF: sub_8643+71Ej
		inc	e
		ld	a, l
		sub	e
		jr	z, loc_8D85
		ld	b, 7Fh ; ''
		jr	loc_8D63
; ---------------------------------------------------------------------------

loc_8D7D:				; CODE XREF: sub_8643+725j
		ex	af, af'
		jp	loc_8B69
; ---------------------------------------------------------------------------

loc_8D81:				; CODE XREF: sub_8643+75Cj
		ex	af, af'
		jp	loc_8BA9
; ---------------------------------------------------------------------------

loc_8D85:				; CODE XREF: sub_8643+734j
					; sub_8643+74Bj ...
		ld	sp, (word_8623)
		pop	af
		ret
; ---------------------------------------------------------------------------

loc_8D8B:				; CODE XREF: sub_8643+753j
		inc	e
		ld	a, l
		sub	e
		jr	z, loc_8D85
		ld	b, 7Fh ; ''
		jr	loc_8D98
; ---------------------------------------------------------------------------

loc_8D94:				; CODE XREF: sub_8643+72Fj
					; sub_8643+763j ...
		srl	b
		jr	nc, loc_8D8B

loc_8D98:				; CODE XREF: sub_8643+74Fj
		exx
		add	hl, bc
		inc	c
		jr	z, loc_8D85
		ex	af, af'
		cp	c
		jr	z, loc_8D81
		ex	af, af'
		add	hl, bc
		bit	7, h
		exx
		jr	z, loc_8D94
		exx
		add	hl, de
		inc	e
		add	hl, de
		exx
		ld	d, l
		ld	l, e
		ld	a, (hl)
		or	b
		ld	(hl), a
		ld	a, d
		sub	e
		dec	a
		srl	a
		jr	nc, loc_8DBC
		inc	l
		ld	(hl), 0FFh

loc_8DBC:				; CODE XREF: sub_8643+774j
		ld	l, d
		sub	11h
		cpl
		ld	(loc_8DC8+1), a
		ld	sp, 8FB6h
		pop	af
		ld	sp, hl

loc_8DC8:				; CODE XREF: sub_8643:loc_8DC8j
					; DATA XREF: sub_8643+77Dw
		jr	$
; END OF FUNCTION CHUNK	FOR sub_8643
; ---------------------------------------------------------------------------
		push	af
		push	af
		push	af
		push	af
		push	af
		push	af
		push	af
		push	af
		push	af
		push	af
		push	af
		push	af
		push	af
		push	af
		push	af
		push	af
		ld	a, (DirPrnCircle)
		and	a
		ld	sp, (word_8623)
		jr	nz, loc_8DE9
		call	sub_8DEE
		jr	loc_8D94
; ---------------------------------------------------------------------------

loc_8DE9:				; CODE XREF: RAM:8DE2j
		call	loc_8E08
		jr	loc_8D94

; =============== S U B	R O U T	I N E =======================================




; === sub_8DEE === CODE XREF: sub_86A1+4ACp
sub_8DEE:				; CODE XREF: sub_86A1+4ACp
					; sub_86A1+54Dp ...
		ld	a, h
		dec	h
		and	7
		ret	nz

loc_8DF3:				; CODE XREF: sub_86A1+A89p
		ld	a, e
		sub	20h ; ' '
		ld	e, a
		ld	a, l
		sub	20h ; ' '
		ld	l, a
		jr	nc, loc_8E03

loc_8DFD:				; DATA XREF: RAM:8F0Bo	sub_8F88+28w
		ld	a, 0BFh	; '┐'
		cp	h
		jr	z, loc_8E22
		ret
; ---------------------------------------------------------------------------

loc_8E03:				; CODE XREF: sub_8DEE+Dj
		ld	a, h
		add	a, 8
		ld	h, a
		ret
; ---------------------------------------------------------------------------

loc_8E08:				; CODE XREF: sub_86A1:loc_8B52p
					; sub_86A1:loc_8BF3p ...
		inc	h
		ld	a, h
		and	7
		ret	nz

loc_8E0D:				; CODE XREF: sub_86A1+A92p
		ld	a, e
		add	a, 20h ; ' '
		ld	e, a
		ld	a, l
		add	a, 20h ; ' '
		ld	l, a
		jr	nc, loc_8E1D

loc_8E17:				; DATA XREF: RAM:8F09o	sub_8F88+Fw
		ld	a, 0D0h	; '╨'
		cp	h
		jr	z, loc_8E22
		ret
; ---------------------------------------------------------------------------

loc_8E1D:				; CODE XREF: sub_8DEE+27j
		ld	a, h
		sub	8
		ld	h, a
		ret
; ---------------------------------------------------------------------------

loc_8E22:				; CODE XREF: sub_8DEE+12j sub_8DEE+2Cj
		pop	af
		ld	sp, (word_8623)
		pop	af
		ret
; End of function sub_8DEE

; ---------------------------------------------------------------------------

loc_8E29:				; CODE XREF: sub_7119j
		ld	b, 1100b
		ld	a, (PrnCrdXCursor)
		ld	c, a
		sub	80h ; 'А'
		jr	c, loc_8E37
		set	0, b
		jr	loc_8E3A
; ---------------------------------------------------------------------------

loc_8E37:				; CODE XREF: RAM:8E31j
		ld	a, 80h ; 'А'
		sub	c

loc_8E3A:				; CODE XREF: RAM:8E35j
		ld	(CircleCrdY+1),	a
		ld	a, (PrnCrdYCursor)
		ld	c, a
		sub	60h ; '`'
		jr	nc, loc_8E4A
		set	1, b
		ld	a, 60h ; '`'
		sub	c

loc_8E4A:				; CODE XREF: RAM:8E43j
		ld	(CircleCrdY), a
		ld	a, b
		ld	(CircleFlg), a
		ld	a, (RadiusCircle)
		ld	(CircleRadius),	a
		di
		ld	hl, byte_8E8D
		call	sub_8E77
		call	PrnCircle_NoFill
		di
		jr	loc_8E74

; =============== S U B	R O U T	I N E =======================================




; === sub_8E64 === CODE XREF: sub_7FF0+Dp
sub_8E64:				; CODE XREF: sub_7FF0+Dp
		call	sub_83FE+2	; 83fe	 PlanetCrd
					; 83fe+2 SunCrd
		jr	nc, loc_8E6C
		cp	10h
		ret	c

loc_8E6C:				; CODE XREF: sub_8E64+3j
		call	sub_83FE	; 83fe	 PlanetCrd
					; 83fe+2 SunCrd
		ccf
		ret	nc
		cp	35h ; '5'
		ret
; End of function sub_8E64

; ---------------------------------------------------------------------------

loc_8E74:				; CODE XREF: RAM:8E62j
		ld	hl, byte_8EAD

; =============== S U B	R O U T	I N E =======================================




; === sub_8E77 === CODE XREF: RAM:8E5Bp
sub_8E77:				; CODE XREF: RAM:8E5Bp
		ld	(word_8623), sp
		ld	a, (byte_862A)
		ld	sp, off_8ECD

loc_8E81:				; CODE XREF: sub_8E77+Ej
		pop	de
		ldi
		dec	a
		jr	nz, loc_8E81
		ld	sp, (word_8623)
		ei
		ret
; End of function sub_8E77

; ---------------------------------------------------------------------------
byte_8E8D:	db 60h,	60h		; DATA XREF: RAM:8E58o
		db 80h,	80h
		db 80h,	80h
		db 80h,	7Fh
		db 1Fh,	0
		db 0, 0
		db 0, 0
		db 80h,	0
		db 0FFh, 0
		db 0, 28h
		db 0FFh, 0
		db 0, 58h
		db 57h,	3Fh
		db 3Fh,	57h
		db 57h,	3Fh
		db 58h,	3Fh
byte_8EAD:	db 40h,	40h		; DATA XREF: RAM:loc_8E74o
		db 78h,	78h
		db 78h,	88h
		db 78h,	77h
		db 1Eh,	1
		db 0CBh, 0FCh
		db 0CBh, 0FCh
		db 88h,	1
		db 0F7h, 0CBh
		db 0FCh, 0C0h
		db 0F7h, 0CBh
		db 0FCh, 0D0h
		db 0CFh, 0BFh
		db 0BFh, 0CFh
		db 0CFh, 0BFh
		db 0D0h, 0BFh
off_8ECD:	dw loc_86FA+1		; DATA XREF: sub_8E77+7o
		dw loc_8713+1
		dw loc_8719+1
		dw loc_873A+1
		dw loc_8743+1
		dw loc_87FE+1
		dw loc_8803+1
		dw loc_8899+1
		dw loc_8FDD+1
		dw loc_89B2+1
		dw loc_89DA
		dw loc_89DA+1
		dw loc_89FB
		dw loc_89FB+1
		dw loc_8A1A+1
		dw loc_8FFC+1
		dw loc_9007+1
		dw loc_900C
		dw loc_900C+1
		dw loc_902C+1
		dw loc_9035+1
		dw loc_903A
		dw loc_903A+1
		dw loc_905A+1
		dw loc_9074+1
		dw loc_906D+1
		dw loc_9105+1
		dw loc_910C+1
		dw loc_9208+1
		dw loc_9201+1
		dw loc_8E17+1
		dw loc_8DFD+1

; =============== S U B	R O U T	I N E =======================================

; визуальный эффект,взрыв корабля от энергобомбы?
; прямая печать	на экран

PrnExplodeObj:				; CODE XREF: sub_ED60p
		ld	a, (FlgObjNear)
		and	a
		ret	z
		ld	a, 40h ; '@'
		ld	(loc_8A49+1), a
		ld	a, 4Fh ; 'O'
		ld	(loc_8F9A+1), a
		ld	a, 3Fh ; '?'
		ld	(loc_8FA5+1), a
		ld	hl, 0
		ld	a, 50h ; 'P'
		call	sub_8F88
		ld	ix, objCoriolis
		ld	b, 8

loc_8F2F:				; CODE XREF: PrnExplodeObj+65j
		push	bc
		ld	a, (ix+20h)
		ld	c, a
		and	30h ; '0'
		cp	30h ; '0'
		jr	nz, loc_8F6C
		ld	l, 0Ah
		call	j_SetAtrViewScr
		ld	h, (ix+1Ch)
		ld	l, (ix+1Dh)
		ld	(CircleCrdY), hl
		ld	l, (ix+1Eh)
		ld	h, (ix+1Fh)
		ld	(CircleFlg), hl
		ld	c, 0Fh

loc_8F53:				; CODE XREF: PrnExplodeObj+54j
		push	bc
		call	sub_F8D7
		pop	bc
		add	a, 3
		ld	(CircleRadius),	a
		ld	a, c
		sub	4
		ld	c, a
		jr	nc, loc_8F53
		ld	l, 0Fh
		call	j_SetAtrViewScr
		res	4, (ix+20h)

loc_8F6C:				; CODE XREF: PrnExplodeObj+2Bj
		ld	de, 27h	; '''
		add	ix, de
		pop	bc
		djnz	loc_8F2F

loc_8F74:				; CODE XREF: sub_F8D7p
		ld	a, 0C0h	; '└'
		ld	(loc_8A49+1), a
		ld	a, 0CFh	; '╧'
		ld	(loc_8F9A+1), a
		ld	a, 0BFh	; '┐'
		ld	(loc_8FA5+1), a
		ld	hl, 0FCCBh
		ld	a, 0D0h	; '╨'
; End of function PrnExplodeObj


; =============== S U B	R O U T	I N E =======================================




; === sub_8F88 === CODE XREF: PrnExplodeObj+19p
sub_8F88:				; CODE XREF: PrnExplodeObj+19p
		ld	(loc_89DA), hl
		ld	(loc_89FB), hl
		ld	(loc_8A6E), hl
		nop
		nop
		nop
		ld	(loc_8A60+1), a
		ld	(loc_8E17+1), a

loc_8F9A:				; DATA XREF: PrnExplodeObj+Cw
					; PrnExplodeObj+6Ew
		ld	a, 0CFh	; '╧'
		ld	(loc_8D0C+1), a
		ld	(loc_8B08+1), a
		ld	(loc_8C0C+1), a

loc_8FA5:				; DATA XREF: PrnExplodeObj+11w
					; PrnExplodeObj+73w
		ld	a, 0BFh	; '┐'
		ld	(loc_8D05+1), a
		ld	(loc_8B01+1), a
		ld	(loc_8C05+1), a
		ld	(loc_8DFD+1), a
		ret
; End of function sub_8F88

; ---------------------------------------------------------------------------
		db 0
		db 0


; === sub_9296 === CODE XREF: RAM:74FAp
sub_9296:				; CODE XREF: RAM:74FAp

; FUNCTION CHUNK AT 937B SIZE 00000006 BYTES
; FUNCTION CHUNK AT 93C0 SIZE 00000010 BYTES
; FUNCTION CHUNK AT 9532 SIZE 0000000B BYTES

		ld	a, (FlgJumpDocking) ; =1 запрет	на обработку нажатий клавиш
					;   включено при: стыковке
					;   работает п/п отрисовки концентрический кругов
					;
					; 0,Flg_Interrupts
		and	a
		ret	z
		call	InitSpeedAngle
		call	ClearAroundObj
		ld	a, (InFreeSpace)
		and	a
		jr	z, loc_92B3
		ld	ix, objCoriolis
		xor	a
		ld	(ix+22h), a
		inc	a
		call	CallFunctionObj
; ---------------------------------------------------------------------------

loc_92B3:				; CODE XREF: sub_9296+Fj
		call	ClearScrBuffer
		call	CopyBuf2Screen
		ld	a, 42h ; 'B'
		ld	(byte_93D2), a
		call	sub_953D
		ld	hl, 93D3h
		ld	(loc_92F9+1), hl
		ld	a, 10h
		ld	(byte_93D0), a
		ld	a, 1
		ld	(loc_9325+1), a
		ld	a, 15h
		ld	(loc_932A+1), a

loc_92D6:				; CODE XREF: PrnDockingTonnel+8j
		ld	a, (FlgJumpDocking) ; =1 запрет	на обработку нажатий клавиш
					;   включено при: стыковке
					;   работает п/п отрисовки концентрический кругов
					;
					; 0,Flg_Interrupts
		and	a

loc_92DA:
		jr	nz, loc_92E3
		ld	a, 1Eh
		ld	(loc_9317+1), a
		jr	loc_92E8
; ---------------------------------------------------------------------------

loc_92E3:				; CODE XREF: sub_9296:loc_92DAj
		ld	a, 5
		ld	(loc_9317+1), a

loc_92E8:				; CODE XREF: sub_9296+4Bj
		ld	a, 0Fh
		ld	(CircleFlg), a
		ld	hl, 0
		ld	(CircleCrdY), hl
		ld	a, (byte_93D0)
		ld	(byte_93D1), a

loc_92F9:				; CODE XREF: sub_9296+9Fj
					; DATA XREF: sub_9296+2Ew ...
		ld	hl, unk_93E3

loc_92FC:				; CODE XREF: sub_9296+86j
		ld	a, (hl)
		dec	a
		ld	(hl), a
		inc	hl
		ld	b, a
		ld	a, 1
		call	sub_93AC
; ---------------------------------------------------------------------------
		ld	a, c
		cp	4
		jr	c, loc_9313
		ld	(CircleRadius),	a
		push	hl
		call	loc_86E4
		pop	hl

loc_9313:				; CODE XREF: sub_9296+73j
		ld	a, (byte_93D1)
		dec	a

loc_9317:				; DATA XREF: sub_9296+48w sub_9296+4Fw
		jr	z, loc_9337
		ld	(byte_93D1), a
		jr	loc_92FC
; ---------------------------------------------------------------------------
		call	CopyBuf2Screen
		ld	a, (byte_93D2)
		dec	a

loc_9325:				; DATA XREF: sub_9296+38w
					; LaunchDocking+12w
		cp	30h ; '0'
		ld	(byte_93D2), a

loc_932A:				; DATA XREF: sub_9296+3Dw
					; LaunchDocking+28w ...
		jr	c, loc_935E
		ld	a, (byte_93D0)
		ld	(byte_93D1), a
		call	ClearScrBuffer
		jr	loc_92F9
; ---------------------------------------------------------------------------

loc_9337:				; CODE XREF: sub_9296:loc_9317j
		ld	a, (byte_93D2)
		dec	a
		ld	(byte_93D2), a
		jr	z, loc_9369
		ret
; ---------------------------------------------------------------------------
		xor	a
		ld	(FlgJumpDocking), a ; =1 запрет	на обработку нажатий клавиш
					;   включено при: стыковке
					;   работает п/п отрисовки концентрический кругов
					;
					; 0,Flg_Interrupts
		ld	(JumpDocking), a ; =1 межпланетный прыжок/стыковка
					; =2 межгалактический прыжок
		ld	a, 10h
		ld	b, a
		ld	hl, unk_93D3

loc_934E:				; CODE XREF: sub_9296+BDj
		ld	a, (hl)
		sub	0BEh ; '╛'
		ld	(hl), a
		inc	hl
		djnz	loc_934E
		call	ClearScrBuffer
		call	CopyBuf2Screen
		jp	loc_93C0
; ---------------------------------------------------------------------------

loc_935E:				; CODE XREF: sub_9296:loc_932Aj
		xor	a
		ld	(FlgJumpDocking), a ; =1 запрет	на обработку нажатий клавиш
					;   включено при: стыковке
					;   работает п/п отрисовки концентрический кругов
					;
					; 0,Flg_Interrupts
		jr	loc_937B
; ---------------------------------------------------------------------------
		xor	a
		ld	(FlgJumpDocking), a ; =1 запрет	на обработку нажатий клавиш
					;   включено при: стыковке
					;   работает п/п отрисовки концентрический кругов
					;
					; 0,Flg_Interrupts
		ret
; ---------------------------------------------------------------------------

loc_9369:				; CODE XREF: sub_9296+A8j
		xor	a
		ld	(JumpDocking), a ; =1 межпланетный прыжок/стыковка
					; =2 межгалактический прыжок
		ld	a, 8
		ld	b, a
		ld	hl, 93E3h

loc_9373:				; CODE XREF: sub_9296+E2j
		ld	a, (hl)
		sub	0C0h ; '└'
		ld	(hl), a
		inc	hl
		djnz	loc_9373
		ret
; End of function sub_9296

; ---------------------------------------------------------------------------
; START	OF FUNCTION CHUNK FOR sub_9296

loc_937B:				; CODE XREF: sub_9296+CCj
		ld	a, 7
		ld	(HighDeltaSpeed), a
		ret
; END OF FUNCTION CHUNK	FOR sub_9296

; =============== S U B	R O U T	I N E =======================================




; === sub_9381 === CODE XREF: RAM:7554p
sub_9381:				; CODE XREF: RAM:7554p
		push	af
		call	sub_F9AB
		pop	af
		jp	j_PrintMessage2
; End of function sub_9381


; =============== S U B	R O U T	I N E =======================================




; === sub_9389 === CODE XREF: sub_9391p	FireLaser+4Ep ...
sub_9389:				; CODE XREF: sub_9391p	FireLaser+4Ep ...
		push	hl
		ld	hl,  loc_FAB9+1
		set	7, (hl)
		pop	hl
		ret
; End of function sub_9389


; =============== S U B	R O U T	I N E =======================================

; c = номер объекта по которому	запустил ракету	игрок



; === sub_9391 === CODE XREF: CallFunctionObj_1+14Fp
sub_9391:				; CODE XREF: CallFunctionObj_1+14Fp
		call	sub_9389
		ld	hl, byte_B4A7
		ld	de, 0FFF6h	; = -#0a
		ld	b, c

loc_939B:				; CODE XREF: sub_9391+Bj
		add	hl, de
		djnz	loc_939B
		ld	a, (PlayerAggressor)
		call	bit2A_equ_bit2HL
		ld	(PlayerAggressor), a
		ld	a, 1
		jp	SetPlayerObjSlot
; End of function sub_9391


; =============== S U B	R O U T	I N E =======================================

; Attributes: noreturn



; === sub_93AC === CODE XREF: sub_9296+6Dp
sub_93AC:				; CODE XREF: sub_9296+6Dp
		ld	c, 0FEh	; '■'

loc_93AE:				; CODE XREF: sub_93AC+Bj sub_93AC+12j
		add	a, a
		jr	c, loc_93B9
		cp	b
		jr	nc, loc_93B9
		sla	c
		ret	nc
		jr	loc_93AE
; ---------------------------------------------------------------------------

loc_93B9:				; CODE XREF: sub_93AC+3j sub_93AC+6j
		sub	b
		scf
		rl	c
		ret	nc
		jr	loc_93AE
; End of function sub_93AC

; ---------------------------------------------------------------------------
; START	OF FUNCTION CHUNK FOR sub_9296

loc_93C0:				; CODE XREF: sub_9296+C5j
		ld	a, (PilotMission1)
		and	a
		ld	l, 7
		call	z, j_SetAtrViewScr
		xor	a
		ld	(SunRadiusNova), a
		jp	loc_9532
; END OF FUNCTION CHUNK	FOR sub_9296
; ---------------------------------------------------------------------------
byte_93D0:	db 8			; DATA XREF: sub_9296+33w sub_9296+5Dr ...
byte_93D1:	db 1			; DATA XREF: sub_9296+60w
					; sub_9296:loc_9313r ...
byte_93D2:	db 0			; DATA XREF: sub_9296+25w sub_9296+8Br ...
unk_93D3:	db  1Eh			; DATA XREF: sub_9296+B5o
		db  21h	; !
		db  23h	; #
		db  26h	; &
		db  28h	; (
		db  2Bh	; +
		db  2Dh	; -
		db  30h	; 0
		db  32h	; 2
		db  35h	; 5
		db  37h	; 7
		db  3Ah	; :
		db  3Ch	; <
		db  3Fh	; ?
		db  41h	; A
		db  44h	; D
unk_93E3:	db  10h			; DATA XREF: sub_9296:loc_92F9o
					; LaunchDocking+5o
		db  10h
		db  10h
		db  18h
		db  20h
		db  28h	; (
		db  30h	; 0
		db  38h	; 8
		db    0
		db    0

; =============== S U B	R O U T	I N E =======================================

; Attributes: noreturn

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



; === sub_953D === CODE XREF: sub_9296+28p
sub_953D:				; CODE XREF: sub_9296+28p
		ld	a, (FlagsSetUp)
		bit	7, a
		ret	z
		call	sub_96B7
		jr	loc_9551
; End of function sub_953D


; =============== S U B	R O U T	I N E =======================================




; === sub_9548 === CODE XREF: RAM:752Fp
sub_9548:				; CODE XREF: RAM:752Fp
		ld	a, (FlagsSetUp)
		bit	7, a
		ret	z
		call	loc_95CF

loc_9551:				; CODE XREF: FX_Launch+9j
					; sub_9296+2A5j ...
		ld	a, (byte_9522)
		and	a
		jr	z, loc_955C
		call	loc_95A6
		jr	loc_9551
; ---------------------------------------------------------------------------

loc_955C:				; CODE XREF: sub_9548+Dj
		ld	(nBeepFX), a
		ei
		ret
; End of function sub_9548

; ---------------------------------------------------------------------------

loc_9561:				; DATA XREF: sub_9579+1Do
		jp	loc_9678
; ---------------------------------------------------------------------------
		jp	loc_9674
; ---------------------------------------------------------------------------
		jp	loc_9692
; ---------------------------------------------------------------------------
		jp	loc_965E
; ---------------------------------------------------------------------------
		jp	loc_9641
; ---------------------------------------------------------------------------
		jp	loc_9648
; ---------------------------------------------------------------------------
		jp	loc_9657
; ---------------------------------------------------------------------------
		jp	loc_9657

; =============== S U B	R O U T	I N E =======================================




; === sub_9579 === CODE XREF: sub_7125j
sub_9579:				; CODE XREF: sub_7125j
		ld	a, (FlagsSetUp)
		bit	7, a
		ret	z
		ld	a, (byte_9522)
		and	a
		jr	nz, loc_959F
		ld	a, (nBeepFX)
		and	a
		ret	z
		ld	b, 0FFh

loc_958C:				; CODE XREF: sub_9579+15j
		inc	b
		rra
		jr	nc, loc_958C
		ld	a, b
		ld	(byte_96BF), a
		add	a, a
		add	a, b
		ld	hl, loc_9561
		add	a, l
		ld	l, a
		jr	nc, loc_959E
		inc	h

loc_959E:				; CODE XREF: sub_9579+22j
		jp	(hl)
; ---------------------------------------------------------------------------

loc_959F:				; CODE XREF: sub_9579+Aj
		dec	a
		jp	z, loc_9624
		dec	a
		jr	z, loc_95E7

loc_95A6:				; CODE XREF: sub_9548+Fp
		ld	hl, (word_9525)
		ld	de, (word_9523)

loc_95AD:				; CODE XREF: sub_969B-Cj
					; DATA XREF: sub_9579+5Dw ...
		ld	c, 50h ; 'P'

loc_95AF:				; CODE XREF: sub_9579+40j
		ld	a, (hl)
		and	18h
		out	(0FEh),	a
		ld	b, d

loc_95B5:				; CODE XREF: sub_9579:loc_95B5j
		djnz	$
		inc	hl
		dec	c
		jr	nz, loc_95AF
		ld	a, d

loc_95BC:				; DATA XREF: sub_969B-13w
		add	a, 2
		ld	d, a
		dec	e
		ld	(word_9523), de
		ld	(word_9525), hl
		ret	nz
		ld	a, (byte_9522)
		bit	7, a
		jr	z, loc_9610

loc_95CF:				; CODE XREF: sub_9548+6p
		ld	a, 3
		ld	(byte_9522), a
		ld	a, 0A0h	; 'а'
		ld	(loc_95AD+1), a
		ld	hl, 127h
		ld	de, 4002h
		ld	(word_9523), de
		ld	(word_9525), hl
		ret
; ---------------------------------------------------------------------------

loc_95E7:				; CODE XREF: sub_9579+2Bj
		ld	hl, (word_9525)
		ld	de, (word_9523)

loc_95EE:				; CODE XREF: RAM:9671j
		ld	c, 64h ; 'd'

loc_95F0:				; CODE XREF: sub_9579+8Cj
		ld	a, (hl)
		inc	l

loc_95F2:				; DATA XREF: RAM:9666w
		srl	a
		srl	a
		ld	e, a
		ld	b, a
		ld	a, 30h ; '0'
		out	(0FEh),	a

loc_95FC:				; CODE XREF: sub_9579:loc_95FCj
		djnz	$
		ld	b, e
		xor	a
		out	(0FEh),	a

loc_9602:				; CODE XREF: sub_9579:loc_9602j
		djnz	$
		dec	c
		jr	nz, loc_95F0

loc_9607:				; CODE XREF: sub_9579+C6j
		dec	d
		ld	(word_9523), de
		ld	(word_9525), hl
		ret	nz

loc_9610:				; CODE XREF: sub_9579+54j
		xor	a
		ld	(byte_9522), a
		ld	a, (byte_96BF)
		inc	a
		ld	b, a
		ld	a, 80h ; 'А'

loc_961B:				; CODE XREF: sub_9579+A3j
		rlca
		djnz	loc_961B
		ld	hl, nBeepFX
		xor	(hl)
		ld	(hl), a
		ret
; ---------------------------------------------------------------------------

loc_9624:				; CODE XREF: sub_9579+27j
		ld	de, (word_9523)

loc_9628:				; CODE XREF: RAM:9655j
		ld	a, e
		add	a, a

loc_962A:				; DATA XREF: RAM:loc_964Dw
		add	a, 5
		ld	e, a
		ld	c, 0Fh

loc_962F:				; CODE XREF: sub_9579+C4j
		ld	a, 30h ; '0'
		ld	b, e
		out	(0FEh),	a

loc_9634:				; CODE XREF: sub_9579:loc_9634j
		djnz	$
		xor	a
		ld	b, e
		out	(0FEh),	a

loc_963A:				; CODE XREF: sub_9579:loc_963Aj
		djnz	$
		dec	c
		jr	nz, loc_962F
		jr	loc_9607
; End of function sub_9579

; ---------------------------------------------------------------------------

loc_9641:				; CODE XREF: RAM:956Dj
		ld	a, 0FDh	; '¤'
		ld	de, 200h
		jr	loc_964D
; ---------------------------------------------------------------------------

loc_9648:				; CODE XREF: RAM:9570j
		ld	a, 0D3h	; '╙'
		ld	de, 800h

loc_964D:				; CODE XREF: RAM:9646j
		ld	(loc_962A+1), a
		ld	a, 1
		ld	(byte_9522), a
		jr	loc_9628
; ---------------------------------------------------------------------------

loc_9657:				; CODE XREF: RAM:9573j	RAM:9576j
		ld	a, 1Ah
		ld	hl, 10C6h
		jr	loc_9663
; ---------------------------------------------------------------------------

loc_965E:				; CODE XREF: RAM:956Aj
		ld	a, 3
		ld	hl, 3FCBh

loc_9663:				; CODE XREF: RAM:965Cj
		ld	de, 100h
		ld	(loc_95F2), hl
		ld	h, a
		ld	l, 1Ah
		ld	a, 2
		ld	(byte_9522), a
		jp	loc_95EE
; ---------------------------------------------------------------------------

loc_9674:				; CODE XREF: RAM:9564j
		ld	c, 0E4h	; 'ф'
		jr	loc_967A
; ---------------------------------------------------------------------------

loc_9678:				; CODE XREF: RAM:loc_9561j
		ld	c, 0FEh	; '■'

loc_967A:				; CODE XREF: RAM:9676j
		ld	a, 3
		ld	(byte_9522), a

loc_967F:				; CODE XREF: RAM:9699j
		ld	b, 40h ; '@'
		ld	de, 4008h
; START	OF FUNCTION CHUNK FOR sub_969B

loc_9684:				; CODE XREF: sub_969B+Cj sub_96A9+Cj
		ld	hl, 127h
		ld	a, c
		ld	(loc_95BC+1), a
		ld	a, b
		ld	(loc_95AD+1), a
		jp	loc_95AD
; END OF FUNCTION CHUNK	FOR sub_969B
; ---------------------------------------------------------------------------

loc_9692:				; CODE XREF: RAM:9567j
		ld	a, 83h ; 'Г'
		ld	(byte_9522), a
		ld	c, 2
		jr	loc_967F

; =============== S U B	R O U T	I N E =======================================




; === sub_969B === CODE XREF: FX_Launch+6p
sub_969B:				; CODE XREF: FX_Launch+6p

; FUNCTION CHUNK AT 9684 SIZE 0000000E BYTES

		ld	a, 3
		ld	(byte_9522), a
		di
		ld	bc, 5002h
		ld	de, 4020h
		jr	loc_9684
; End of function sub_969B


; =============== S U B	R O U T	I N E =======================================




; === sub_96A9 === CODE XREF: sub_9296+2A2p
sub_96A9:				; CODE XREF: sub_9296+2A2p
		ld	bc, 0C0F0h
		ld	de, 0C00Ch

loc_96AF:				; CODE XREF: sub_96B7+6j
		di
		ld	a, 3
		ld	(byte_9522), a
		jr	loc_9684
; End of function sub_96A9


; =============== S U B	R O U T	I N E =======================================




; === sub_96B7 === CODE XREF: sub_953D+6p
sub_96B7:				; CODE XREF: sub_953D+6p
		ld	bc, 0C010h
		ld	de, 100Ah
		jr	loc_96AF
; End of function sub_96B7

; ---------------------------------------------------------------------------
byte_96BF:	db 2			; DATA XREF: sub_9579+18w sub_9579+9Br


; === sub_96C5 === CODE XREF: sub_9736+1Cp sub_9736+3Cp
sub_96C5:				; CODE XREF: sub_9736+1Cp sub_9736+3Cp
		srl	b
		rr	c
; End of function sub_96C5


; =============== S U B	R O U T	I N E =======================================




; === sub_96C9 === CODE XREF: sub_9736p
sub_96C9:				; CODE XREF: sub_9736p
		srl	b
		rr	c
; End of function sub_96C9


; =============== S U B	R O U T	I N E =======================================




; === sub_96CD === CODE XREF: sub_9736+21p sub_9736+41p
sub_96CD:				; CODE XREF: sub_9736+21p sub_9736+41p
		srl	b
		rr	c
; End of function sub_96CD


; =============== S U B	R O U T	I N E =======================================




; === sub_96D1 === CODE XREF: sub_9736+5p
sub_96D1:				; CODE XREF: sub_9736+5p
		srl	b
		rr	c
		ret
; End of function sub_96D1

; ---------------------------------------------------------------------------
; START	OF FUNCTION CHUNK FOR sub_96DC

loc_96D6:				; CODE XREF: sub_96DC+15j sub_96DC+1Cj ...
		jp	loc_982E
; END OF FUNCTION CHUNK	FOR sub_96DC
; ---------------------------------------------------------------------------
; START	OF FUNCTION CHUNK FOR sub_ED60

PrintObjScaner:				; CODE XREF: sub_ED60+6j
		call	CopyLocator2Scr
; END OF FUNCTION CHUNK	FOR sub_ED60

; =============== S U B	R O U T	I N E =======================================




; === sub_9852 === CODE XREF: sub_9736+EDj
sub_9852:				; CODE XREF: sub_9736+EDj
		dec	l
		ld	a, (hl)
		or	3
		ld	(hl), a
		inc	l
		ld	a, (hl)
		or	80h ; 'А'
		ld	(hl), a
		jr	loc_982E
; End of function sub_9852

; ---------------------------------------------------------------------------
; START	OF FUNCTION CHUNK FOR sub_9736

loc_985E:				; CODE XREF: sub_9736+F1j
		dec	l
		ld	a, (hl)
		or	1
		ld	(hl), a
		inc	l
		ld	a, (hl)
		or	0C0h ; '└'
		ld	(hl), a
		jr	loc_982E
; END OF FUNCTION CHUNK	FOR sub_9736

; =============== S U B	R O U T	I N E =======================================




; === sub_986A === CODE XREF: sub_9736+E6p
sub_986A:				; CODE XREF: sub_9736+E6p
		ld	a, l
		add	a, 20h ; ' '
		ld	l, a
		jr	nc, loc_9877
		ld	a, 58h ; 'X'
		cp	h
		ret	nz
		pop	af
		jr	loc_982E
; ---------------------------------------------------------------------------

loc_9877:				; CODE XREF: sub_986A+4j
		ld	a, h
		sub	8
		ld	h, a
		ret
; End of function sub_986A


; =============== S U B	R O U T	I N E =======================================




; === sub_987C === CODE XREF: sub_9736+D8p
sub_987C:				; CODE XREF: sub_9736+D8p
		ld	a, l
		cp	0Fh
		jr	nz, loc_9885
		pop	af
		jp	loc_982E
; ---------------------------------------------------------------------------

loc_9885:				; CODE XREF: sub_987C+3j
		sub	20h ; ' '
		ld	l, a
		ret	c
		ld	a, h
		add	a, 8
		ld	h, a
		ret
; End of function sub_987C


; =============== S U B	R O U T	I N E =======================================


SetIndStation:				; CODE XREF: sub_74C6+6p sub_96DC+5p
		ld	a, (FlgCoriolisNear)
		and	a
		ret	z
		ld	a, (IndDanger)
		cp	63h ; 'c'
		ret	z
		cp	2
		ret	z
		ld	a, 6
		ld	(IndDanger), a
		ret
; End of function SetIndStation

; ---------------------------------------------------------------------------

loc_98A2:				; CODE XREF: PrnCocpit_Ifn+41Aj
		call	CopyScaner2Scr
		ld	bc, 5
		ld	a, 0EBh	; 'ы'
		ld	hl, loc_9912
		ld	(hl), a
		ld	a, 0E5h	; 'х'
		ld	hl, loc_9915
		ld	(hl), a
		add	hl, bc
		ld	(hl), a
		ld	a, 0E1h	; 'с'
		ld	hl, loc_9935
		ld	(hl), a
		inc	hl
		ld	a, 24h ; '$'
		ld	(hl), a
		xor	a
		ld	hl, loc_9940
		ld	(hl), a
		ld	a, 0EBh	; 'ы'
		ld	hl, loc_9939
		ld	(hl), a
		jr	loc_990C

; =============== S U B	R O U T	I N E =======================================


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




; === sub_9924 === CODE XREF: sub_78B1+7Bp sub_78B1+83p ...
sub_9924:				; CODE XREF: sub_78B1+7Bp sub_78B1+83p ...

; FUNCTION CHUNK AT B286 SIZE 00000019 BYTES

		ld	h, 0
		add	a, a
		and	a
		jp	loc_B286
; End of function sub_9924

; ---------------------------------------------------------------------------
byte_992B:	db 21h			; DATA XREF: CopyLocator2Scr+Do
		db    6
		db 0ACh	; м
		db    6
		db  21h	; !
		db    0
		db 0EDh	; э
		db 0A0h	; а
		db 0EDh	; э
		db 0A0h	; а
; ---------------------------------------------------------------------------
; START	OF FUNCTION CHUNK FOR CopyScaner2Scr

loc_9935:				; CODE XREF: CopyScaner2Scr+55j
					; DATA XREF: RAM:98B8o	...
		pop	de
		inc	d
		djnz	loc_991A

loc_9939:				; DATA XREF: RAM:98C7o
					; CopyScaner2Scr+3Bo
		nop
		pop	de
		ex	de, hl
		ld	bc, 20h	; ' '
		add	hl, bc

loc_9940:				; DATA XREF: RAM:98C1o
					; CopyScaner2Scr+36o
		ex	de, hl
		dec	a
		jr	nz, loc_9915
		ret
; END OF FUNCTION CHUNK	FOR CopyScaner2Scr
; ---------------------------------------------------------------------------
		db    0

; =============== S U B	R O U T	I N E =======================================




; === sub_9946 === CODE XREF: LaunchDocking+8Dp
sub_9946:				; CODE XREF: LaunchDocking+8Dp
		call	j_PrintMissiles
		call	j_ResetBattleMessages
		xor	a
		ld	(Missile1), a
		ld	(TempLaser), a
		ld	(Temperature), a
		ld	(AutoDocking), a
		ld	(TimeWorkECM), a
		ld	(byte_9C1B), a
		ld	a, 0A0h	; 'а'
		ld	(ShieldFront), a
		ld	(ShieldRear), a
		ld	(Altitude), a
		ld	hl, ShieldUnit1
		ld	b, 4

loc_996F:				; CODE XREF: sub_9946+2Cj
		ld	(hl), a
		inc	hl
		inc	hl
		djnz	loc_996F

loc_9974:				; CODE XREF: RAM:loc_712Dj
		ld	hl, 5A23h
		ld	a, (WhatOnScreen)
		cp	4
		jr	c, loc_997F
		inc	h

loc_997F:				; CODE XREF: sub_9946+36j
		ld	c, 17h
		ld	a, (PilotEscapePod)
		and	a
		jr	z, loc_9989
		ld	c, 1Fh

loc_9989:				; CODE XREF: sub_9946+3Fj
		ld	de, 1Bh
		push	hl
		call	sub_9995
		pop	hl
		ld	a, l
		add	a, 15h
		ld	l, a
; End of function sub_9946


; =============== S U B	R O U T	I N E =======================================




; === sub_9995 === CODE XREF: sub_9946+47p
sub_9995:				; CODE XREF: sub_9946+47p
		ld	b, 7

loc_9997:				; CODE XREF: RAM:99A0j
		push	bc

loc_9998:
		ld	b, 5
; End of function sub_9995


loc_999A:				; CODE XREF: RAM:999Cj
		ld	(hl), c
		inc	hl
		djnz	loc_999A
		pop	bc
		add	hl, de
		djnz	loc_9997
		ret
; ---------------------------------------------------------------------------
unk_99A3:	db  44h	; D		; DATA XREF: sub_99B5+Do
; ---------------------------------------------------------------------------

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




; === sub_9A3A === CODE XREF: sub_99B5+2Ep
sub_9A3A:				; CODE XREF: sub_99B5+2Ep
		ld	a, 4
		ld	(IndDanger), a
		ret
; End of function sub_9A3A


; =============== S U B	R O U T	I N E =======================================

; Печать индикаторов, запуск ракеты, бомбы

Process_MissBombEPod:			; CODE XREF: RAM:750Cp
		ld	a, (WhatOnScreen)
		cp	4
		ret	nc
		call	PrnDangerEcmDock
		call	UseMissile
		call	UseEscapePod
		call	j_UseEnergyBomb
		ld	hl, ControlKey3
		ld	a, (hl)
		and	10010001b
		ld	(hl), a
; End of function Process_MissBombEPod


; =============== S U B	R O U T	I N E =======================================


Damage2Pilot:				; CODE XREF: sub_74C6+9p
		ld	hl, IndDanger
		xor	a
		cp	(hl)
		ret	z
		ld	a, (ShieldUnit4)
		cp	0A0h ; 'а'
		jr	z, loc_9A6B
		ld	a, 63h ; 'c'
		ld	(IndDanger), a

loc_9A6B:				; CODE XREF: Damage2Pilot+Bj
		call	EnergyRecovery
		ld	hl, ShieldFront
		ld	a, (DamageFrontShield)
		call	sub_9AA7
		ld	hl, ShieldRear
		ld	a, (DamageRearShield)
		call	sub_9AA7
		ld	a, (CloakingDeviceOn)
		and	a
		ld	a, (DamageEnergyUnit)
		jr	z, loc_9A8A
		inc	a

loc_9A8A:				; CODE XREF: Damage2Pilot+2Ej
		ld	c, a
		ld	b, 4
; End of function Damage2Pilot


; =============== S U B	R O U T	I N E =======================================




; === sub_9A8D === CODE XREF: sub_9AA7+23p
sub_9A8D:				; CODE XREF: sub_9AA7+23p
		ld	hl, ShieldUnit1
		ld	d, 4

loc_9A92:				; CODE XREF: sub_9A8D+Ej sub_9A8D+17j
		ld	a, (hl)
		sub	c
		jr	nc, loc_9AA3
		ld	(hl), 0
		inc	hl
		inc	hl
		dec	d
		jr	nz, loc_9A92
		ld	a, 1
		ld	(PlayerGameOver), a
		ret
; ---------------------------------------------------------------------------

loc_9AA3:				; CODE XREF: sub_9A8D+7j
		ld	(hl), a
		djnz	loc_9A92
		ret
; End of function sub_9A8D


; =============== S U B	R O U T	I N E =======================================




; === sub_9AA7 === CODE XREF: Damage2Pilot+1Bp
sub_9AA7:				; CODE XREF: Damage2Pilot+1Bp
					; Damage2Pilot+24p
		ld	c, a
		and	a
		jr	nz, loc_9AB3
		ld	hl, unk_9ADA
		ld	a, (hl)
		and	a
		ret	z
		dec	(hl)
		ret
; ---------------------------------------------------------------------------

loc_9AB3:				; CODE XREF: sub_9AA7+2j
		push	bc
		push	hl
		cp	46h ; 'F'
		call	nc, sub_D0F3
		pop	hl
		pop	bc
		ld	b, 4

loc_9ABE:				; CODE XREF: sub_9AA7+1Cj
		ld	a, (hl)
		sub	c
		jr	c, loc_9AC6
		ld	(hl), a
		djnz	loc_9ABE
		ret
; ---------------------------------------------------------------------------

loc_9AC6:				; CODE XREF: sub_9AA7+19j
		ld	(hl), 0
		dec	b
		ret	z
		call	sub_9A8D
		ld	hl, 9ADAh
		ld	a, (hl)
		add	a, 5
		ld	(hl), a
		cp	0Dh
		ret	c
		jp	sub_D0F3
; End of function sub_9AA7

; ---------------------------------------------------------------------------
unk_9ADA:	db    0			; DATA XREF: sub_9AA7+4o

; =============== S U B	R O U T	I N E =======================================


EnergyRecovery:				; CODE XREF: Damage2Pilot:loc_9A6Bp
		ld	a, (byte_D08B)
		and	a
		call	nz, loc_8481
; End of function EnergyRecovery


; =============== S U B	R O U T	I N E =======================================




; === sub_9AE2 === CODE XREF: RAM:8482p
sub_9AE2:				; CODE XREF: RAM:8482p
		ld	hl, ShieldUnit4
		ld	b, 4

loc_9AE7:				; CODE XREF: sub_9AE2+Ej
		ld	a, (hl)
		cp	0A0h ; 'а'
		jr	z, loc_9AEE
		inc	(hl)
		ret
; ---------------------------------------------------------------------------

loc_9AEE:				; CODE XREF: sub_9AE2+8j
		dec	hl
		dec	hl
		djnz	loc_9AE7
		ld	hl, ShieldFront
		call	sub_9AFB
		ld	hl, ShieldRear
; End of function sub_9AE2


; =============== S U B	R O U T	I N E =======================================




; === sub_9AFB === CODE XREF: sub_9AE2+13p
sub_9AFB:				; CODE XREF: sub_9AE2+13p
		ld	a, (hl)
		cp	0A0h ; 'а'
		ret	z
		inc	(hl)
		ret
; End of function sub_9AFB


; =============== S U B	R O U T	I N E =======================================


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




; === sub_9B14 === CODE XREF: PrnDangerEcmDock+Cp
sub_9B14:				; CODE XREF: PrnDangerEcmDock+Cp
		and	a
		jr	z, loc_9B19
		ld	a, 7

loc_9B19:				; CODE XREF: sub_9B14+1j
		ld	(hl), a
		ret
; End of function sub_9B14


; =============== S U B	R O U T	I N E =======================================


PrintMissiles:				; CODE XREF: j_PrintMissilesj
		ld	hl, 1702h
		ld	(CrdXPrnSymbol), hl
		ld	hl, (word_D04B)
		ld	(hl), 0
		push	hl
		ld	a, (PilotMissiles)
		ld	c, a
		ld	b, 4

loc_9B2D:				; CODE XREF: PrintMissiles+20j
		dec	c
		push	bc
		bit	7, c
		ld	a, 25h ; '%'
		jr	z, loc_9B37
		ld	a, 40h ; '@'

loc_9B37:				; CODE XREF: PrintMissiles+18j
		call	PrintMessage1_A
		pop	bc
		djnz	loc_9B2D
		pop	hl
		ld	(hl), 0A9h ; 'й'
		ret
; End of function PrintMissiles


; =============== S U B	R O U T	I N E =======================================

; Увеличение/уменьшение	прогресса таргонов, и выдача сообщения о войне
; Вх:  ---
; Вых: hl,a - не определены

ProgressMissionWar:			; CODE XREF: ReCalcCrdObj_coriolis+3p
		ld	hl, byte_70DC
		ld	a, (PilotTargonWar) ; =#81 при сбросе миссий
					; 7,=1 начало войны, предолжение участвовать
					; 6-0, чем больше число	тем дальше от левого края карты	захват планет таргонами
					;
					; =#01 война закончена
		cpl
		add	a, (hl)
		ld	(hl), a
		ret	nc
		inc	hl
		inc	(hl)
		ret	nz
		ld	hl, PilotTargonWar ; =#81 при сбросе миссий
					; 7,=1 начало войны, предолжение участвовать
					; 6-0, чем больше число	тем дальше от левого края карты	захват планет таргонами
					;
					; =#01 война закончена
		ld	a, (hl)
		cp	2
		ret	c
		rlca
		jr	c, loc_9B5A
		dec	(hl)
		ret
; ---------------------------------------------------------------------------

loc_9B5A:				; CODE XREF: ProgressMissionWar+15j
		inc	(hl)
		jr	z, loc_9B65
		ld	l, 0C7h	; '╟'   ; hl = CargoFound
		ld	(hl), 82h ; 'В' ; сообщение о вторжении в 1ю галактику
		inc	hl
		ld	(hl), 0F0h ; 'Ё'
		ret
; ---------------------------------------------------------------------------

loc_9B65:				; CODE XREF: ProgressMissionWar+1Aj
		ld	(hl), 7Fh ; ''
		ret
; End of function ProgressMissionWar


; =============== S U B	R O U T	I N E =======================================


CalcCrdSunPlanet_2:			; CODE XREF: CalcCrdSunPlanet+Bj
		call	j_CalcNewCrdObj_Pl
		ld	ix, fsSunCrdXl	; координаты "солнца" и "планеты" в межпланетном пространстве
		ld	hl, loc_A1AD
		ld	(hl), 0C9h ; '╔'
		push	hl
		call	j_CalcNewCrdObj_Pl
		ld	ix, fsPlCrdXl
		call	j_CalcNewCrdObj_Pl
		pop	hl
		ld	(hl), 3Ah ; ':'
		ret
; End of function CalcCrdSunPlanet_2


; =============== S U B	R O U T	I N E =======================================




; === sub_9B83 === CODE XREF: GenerateAnotherPricep
sub_9B83:				; CODE XREF: GenerateAnotherPricep
		ld	hl, Pilot_Fuel
		ld	a, 46h ; 'F'
		sub	(hl)
		jr	z, loc_9B9D
		ld	b, a
		ld	a, (PilotRnd)
		ld	l, 96h ; 'Ц'    ; hl = PilotPlanetData
		or	(hl)
		or	80h ; 'А'
		ld	e, a
		ld	d, 0
		ld	h, d
		ld	l, d

loc_9B99:				; CODE XREF: sub_9B83+17j
		add	hl, de
		djnz	loc_9B99
		ld	a, h

loc_9B9D:				; CODE XREF: sub_9B83+6j
		ld	(byte_7060), a
		ld	hl, PilotPlanetProduct
		ret
; End of function sub_9B83

; ---------------------------------------------------------------------------
		db  19h
		db 0C9h	; ╔
		db 0D4h	; ╘

; =============== S U B	R O U T	I N E =======================================


UseMissile:				; CODE XREF: Process_MissBombEPod+9p
		ld	a, (PilotMissiles)
		and	a
		ret	z
		ld	a, (Missile1)
		cp	1
		jr	nz, loc_9BCF
		ld	hl, nObjMissile
		ld	a, (hl)
		and	a
		jr	nz, loc_9BCF
		ld	a, (ObjInTarget)
		and	a
		jr	z, loc_9BCF
		ld	(hl), a
		ld	hl, Missile1
		ld	(hl), 2
		ld	hl, 70h	; 'p'
		ld	de, 60h	; '`'
		call	3B5h

loc_9BCF:				; CODE XREF: UseMissile+Aj
					; UseMissile+11j ...
		ld	hl, ControlKey3
		ld	a, (hl)
		ld	hl, Missile1
		srl	a
		srl	a
		jr	c, MissileTarget
		srl	a
		jr	c, MissileFire
		srl	a
		jr	c, MissileUntarget
		ret
; ---------------------------------------------------------------------------

MissileTarget:				; CODE XREF: UseMissile+33j
		ld	a, (hl)
		cp	2
		ret	z
		ld	(hl), 1
		xor	a
		ld	(nObjMissile), a
		ret
; ---------------------------------------------------------------------------

MissileUntarget:			; CODE XREF: UseMissile+3Bj
		ld	(hl), 0
		ret
; ---------------------------------------------------------------------------

MissileFire:				; CODE XREF: UseMissile+37j
		ld	a, (hl)
		cp	2
		ret	nz
		ld	a, (nObjMissile)
		add	a, a
		add	a, a
		add	a, a
		add	a, a
		or	2
		push	hl
		call	CallFunctionObj
; End of function UseMissile

; ---------------------------------------------------------------------------
		pop	hl
		jr	z, loc_9C0D
		ld	hl, MissileJammed
		set	7, (hl)
		ret
; ---------------------------------------------------------------------------

loc_9C0D:				; CODE XREF: RAM:9C05j
		ld	(hl), 0
		ld	hl, nBeepFX
		set	1, (hl)
		nop
		nop
		nop
		nop

; =============== S U B	R O U T	I N E =======================================

; Attributes: thunk



; === sub_9CAF === CODE XREF: sub_728D+13Cp
sub_9CAF:				; CODE XREF: sub_728D+13Cp
		jp	sub_A28D
; End of function sub_9CAF

; ---------------------------------------------------------------------------


; === sub_9F18 === CODE XREF: Sqrt_H_HL+3j Sqrt_H_HL+Dp ...
sub_9F18:				; CODE XREF: Sqrt_H_HL+3j Sqrt_H_HL+Dp ...
		xor	a
		push	hl
		push	hl
		srl	h
		rr	l
		rra
		ld	de, 8000h
		call	loc_9F35
; ---------------------------------------------------------------------------
		pop	de
		call	loc_9F2B
; ---------------------------------------------------------------------------
		pop	de

loc_9F2B:				; CODE XREF: sub_9F18+Fp
		rr	h
		rr	l
		ex	de, hl
		srl	h
		rr	l
		rra

loc_9F35:				; CODE XREF: sub_9F18+Bp
		push	de
		call	Div_BC_HLAdivDE_1
; ---------------------------------------------------------------------------
		srl	b
		rr	c
		xor	a
		pop	hl
		add	hl, bc
		ret
; End of function sub_9F18


; =============== S U B	R O U T	I N E =======================================


CorrAxisVector:				; CODE XREF: sub_9D40+F6p sub_9D40+F9p ...
		ld	h, 0BFh	; '┐'
		ld	l, (ix+1)
		sla	l
		ld	a, (hl)
		ld	l, (ix+3)
		sla	l
		add	a, (hl)
		ld	l, (ix+5)
		sla	l
		add	a, (hl)
		sra	a
		sra	a
		neg
		xor	80h ; 'А'
		ld	b, a
		call	sub_9F68
		call	sub_9F68
		call	sub_9F68
		ret
; End of function CorrAxisVector


; =============== S U B	R O U T	I N E =======================================




; === sub_9F68 === CODE XREF: CorrAxisVector+1Dp
sub_9F68:				; CODE XREF: CorrAxisVector+1Dp
					; CorrAxisVector+20p ...
		ld	c, (ix+1)
		sla	c
		call	Mult_HL_B_C
		sla	h
		sla	(ix+1)
		rr	h
		ld	(ix+1),	h
		inc	ix
		inc	ix
		ret
; End of function sub_9F68

; ---------------------------------------------------------------------------


; === sub_A076 === CODE XREF: CalcNewCrdObjSpeed+7p
sub_A076:				; CODE XREF: CalcNewCrdObjSpeed+7p
		sla	d
		rl	c
		rl	c
		srl	d
		ld	hl, 0
		ld	b, 6

loc_A083:				; CODE XREF: sub_A076:loc_A08Cj
		srl	d
		rr	e
		sla	c
		jr	nc, loc_A08C
		add	hl, de

loc_A08C:				; CODE XREF: sub_A076+13j
		djnz	loc_A083
		ld	de, 0FFh
		add	hl, de
		ld	e, h
		sla	c
		rr	d
		ret
; End of function sub_A076


; =============== S U B	R O U T	I N E =======================================




; === sub_A098 === CODE XREF: CalcNewCrdObjSpeed+13p
sub_A098:				; CODE XREF: CalcNewCrdObjSpeed+13p
		ld	b, a
		ld	c, d
		xor	c
		res	7, d
		jr	nz, loc_A0A1
		add	hl, de
		ret
; ---------------------------------------------------------------------------

loc_A0A1:				; CODE XREF: sub_A098+5j
		sbc	hl, de
		ret	nc
		add	hl, de
		ex	de, hl
		and	a
		sbc	hl, de
		ld	b, c
		ret
; End of function sub_A098


; =============== S U B	R O U T	I N E =======================================


RotateAxisSCO:				; CODE XREF: ReCalcCrdObj+33p
					; ReCalcCrdObj+36p ...
		ld	a, (AngleLR4Calc+1)
		xor	80h ; 'А'
		sra	a
		res	6, a
		ld	c, a
		ld	b, (ix+3)
		ld	d, (ix+5)
		call	sub_A0FD
		ld	(ix+5),	a
		ld	a, (AngleLR4Calc)
		xor	80h ; 'А'
		ld	c, a
		ld	b, (ix+3)
		ld	d, (ix+1)
		call	sub_A0FD
		ld	(ix+1),	a
		ld	a, (AngleLR4Calc)
		ld	c, a
		ld	b, (ix+1)
		ld	d, (ix+3)
		call	sub_A0FD
		ld	(ix+3),	a
		ld	a, (AngleLR4Calc+1)
		sra	a
		res	6, a
		ld	c, a
		ld	b, (ix+5)
		ld	d, (ix+3)
		call	sub_A0FD
		ld	(ix+3),	a
		ld	de, 6
		add	ix, de
		ret
; End of function RotateAxisSCO


; =============== S U B	R O U T	I N E =======================================




; === sub_A0FD === CODE XREF: RotateAxisSCO+10p
sub_A0FD:				; CODE XREF: RotateAxisSCO+10p
					; RotateAxisSCO+22p ...
		push	de
		xor	b
		sla	b
		res	7, c
		ld	e, a
		call	Mult_A_hiB_C
		pop	bc
		ld	c, a
		ld	a, e
		xor	b
		sla	b
		rr	e
		sla	c
		rla
		ld	a, b
		jr	c, loc_A11C
		add	a, c
		jr	nc, loc_A123
		ld	a, 0FFh
		jr	loc_A123
; ---------------------------------------------------------------------------

loc_A11C:				; CODE XREF: sub_A0FD+16j
		sub	c
		jr	nc, loc_A123
		neg
		rl	e

loc_A123:				; CODE XREF: sub_A0FD+19j sub_A0FD+1Dj ...
		rl	e
		rra
		ret
; End of function sub_A0FD


; =============== S U B	R O U T	I N E =======================================


RotateCrdNSCO:				; CODE XREF: DemoRotateObj+5p
					; DemoRotateObj+8p ...
		ld	a, (AngleLR4Calc+1)
		xor	80h ; 'А'
		ld	c, a
		ld	a, c
		ld	b, (ix+10h)
		ld	d, (ix+0Ah)
		call	sub_A0FD
		ld	(ix+0Ah), a
		ld	a, (AngleLR4Calc)
		xor	80h ; 'А'
		ld	c, a
		ld	a, c
		ld	b, (ix+10h)
		ld	d, (ix+16h)
		call	sub_A0FD
		ld	(ix+16h), a
		ld	a, (AngleLR4Calc)
		ld	c, a
		ld	b, (ix+16h)
		ld	d, (ix+10h)
		call	sub_A0FD
		ld	(ix+10h), a
		ld	a, (AngleLR4Calc+1)
		ld	c, a
		ld	a, c
		ld	b, (ix+0Ah)
		ld	d, (ix+10h)
		call	sub_A0FD
		ld	(ix+10h), a
		inc	ix
		inc	ix
		ret
; End of function RotateCrdNSCO


; =============== S U B	R O U T	I N E =======================================


CalcNewCrdObj_Pl:			; CODE XREF: j_CalcNewCrdObj_Plj
					; ReCalcCrdObjp
		call	sub_7F81
		xor	80h ; 'А'
		ld	c, a
		ld	de, 3
		ld	hl, 6
		call	CalcNewCrdNobj
		ld	a, (HighAngleLR)
		xor	80h ; 'А'
		call	Mult_Ax2_sign
		ld	de, 3
		ld	h, d
		ld	l, d
		call	CalcNewCrdNobj
		ld	a, (HighAngleLR)
		call	Mult_Ax2_sign
		ld	hl, 3
		ld	d, h
		ld	e, h
		call	CalcNewCrdNobj
		ld	a, (HighAngleDU)
		ld	c, a
		ld	hl, 3
		ld	de, 6
		call	CalcNewCrdNobj

loc_A1AD:				; DATA XREF: CalcCrdSunPlanet_2+7o
		ld	a, (HighDeltaSpeed)
		inc	a
		add	a, a
		ld	b, a
		ld	h, (ix+8)
		ld	l, (ix+7)
		ld	a, (ix+6)
		bit	7, h
		jr	z, loc_A1D0
		add	a, b
		ld	(ix+6),	a
		jr	nc, locret_A1E9
		inc	hl
		bit	7, h
		jr	nz, loc_A1E3
		jp	sub_5FD5
; ---------------------------------------------------------------------------
		db  18h
		db  0Eh
; ---------------------------------------------------------------------------

loc_A1D0:				; CODE XREF: CalcNewCrdObj_Pl+4Bj
		sub	b
		ld	(ix+6),	a
		jr	nc, locret_A1E9
		dec	hl
		bit	7, h
		jr	z, loc_A1E3
		ld	hl, 8000h
		neg
		ld	(ix+6),	a

loc_A1E3:				; CODE XREF: CalcNewCrdObj_Pl+56j
					; CalcNewCrdObj_Pl+66j
		ld	(ix+8),	h
		ld	(ix+7),	l

locret_A1E9:				; CODE XREF: CalcNewCrdObj_Pl+51j
					; CalcNewCrdObj_Pl+61j
		ret
; End of function CalcNewCrdObj_Pl


; =============== S U B	R O U T	I N E =======================================


CalcNewCrdNobj:				; CODE XREF: CalcNewCrdObj_Pl+Cp
					; CalcNewCrdObj_Pl+1Cp	...
		push	ix
		push	ix
		ex	(sp), iy
		add	ix, de
		ex	de, hl
		add	iy, de
		ld	a, (ix+2)
		ld	d, (ix+1)
		ld	e, (ix+0)
		ld	b, a
		xor	c
		and	80h ; 'А'
		push	af
		res	7, c
		res	7, b
		push	de
		call	Mult_HL_B_C
		pop	de
		push	hl
		ld	b, d
		push	de
		call	Mult_HL_B_C
		pop	de
		ld	b, e
		ld	a, l
		ld	l, h
		ld	h, 0
		pop	de
		add	hl, de
		ld	e, a
		push	hl
		call	Mult_A_hiB_C
		add	a, e
		pop	hl
		jr	nc, loc_A224
		inc	hl

loc_A224:				; CODE XREF: CalcNewCrdNobj+37j
		ld	b, a
		ld	a, h
		ld	h, l
		ld	l, b
		pop	bc
		ld	c, b
		ld	b, (iy+2)
		ld	d, (iy+1)
		ld	e, (iy+0)
		push	af
		ld	a, b
		xor	c
		rlc	c
		rlc	c
		rla
		rr	c
		rrc	c
		srl	a
		ld	b, a
		pop	af
		bit	6, c
		jr	nz, loc_A252
		add	a, a
		sla	c
		rra
		add	a, b
		add	hl, de
		jr	nc, loc_A273
		inc	a
		jr	loc_A273
; ---------------------------------------------------------------------------

loc_A252:				; CODE XREF: CalcNewCrdNobj+5Bj
		and	a
		ex	de, hl
		sbc	hl, de
		jr	nc, loc_A259
		inc	a

loc_A259:				; CODE XREF: CalcNewCrdNobj+6Cj
		ld	d, a
		ld	a, b
		sub	d
		ld	b, a
		jr	nc, loc_A26C
		cpl
		ld	b, a
		ld	a, c
		xor	80h ; 'А'
		ld	c, a
		ex	de, hl
		and	a
		ld	hl, 0
		sbc	hl, de

loc_A26C:				; CODE XREF: CalcNewCrdNobj+73j
		ld	a, b
		rla
		sla	c
		rra
		xor	80h ; 'А'

loc_A273:				; CODE XREF: CalcNewCrdNobj+63j
					; CalcNewCrdNobj+66j
		ld	(iy+2),	a
		ld	(iy+1),	h
		ld	(iy+0),	l
		pop	iy
		pop	ix
		ret
; End of function CalcNewCrdNobj


; =============== S U B	R O U T	I N E =======================================


Mult_Ax2_sign:				; CODE XREF: CalcNewCrdObj_Pl+14p
					; CalcNewCrdObj_Pl+22p
		ld	c, a
		rl	c
		rra
		rl	c
		rla
		rr	c
		ret
; End of function Mult_Ax2_sign

; ---------------------------------------------------------------------------
		db    0
		db    0

; =============== S U B	R O U T	I N E =======================================




; === sub_A28D === CODE XREF: sub_9CAFj
sub_A28D:				; CODE XREF: sub_9CAFj
		ld	ix, objMissile
		ld	h, (ix+24h)
		ld	l, (ix+23h)
		push	hl
		pop	iy
		ld	a, 94h ; 'Ф'
		ld	(AngleLR4Calc),	a
		ld	a, 8Ch ; 'М'
		call	DemoRotateObj
		call	CorrAxirY_SKO
		ld	ix, objMissile.field_9
		call	CorrAxisVector
		call	CorrAxisVector
		ld	ix, objMissile
		jp	PrnObjIfVisible
; End of function sub_A28D

; ---------------------------------------------------------------------------


; === sub_A537 === CODE XREF: MoveMissile+40p
sub_A537:				; CODE XREF: MoveMissile+40p
					; MoveMissile+48p ...
		add	hl, hl
		ld	a, h
		and	a
		ret	nz
		ld	a, l
		and	0C0h ; '└'
		ret
; End of function sub_A537


; =============== S U B	R O U T	I N E =======================================




; === sub_A53F === CODE XREF: MoveMissile+2Ap
sub_A53F:				; CODE XREF: MoveMissile+2Ap
					; MoveMissile+30p ...
		ld	l, (iy+0)
		ld	h, (iy+1)
		ld	c, (iy+2)
		ld	e, (ix+0)
		ld	d, (ix+1)
		ld	b, (ix+2)
		sla	c
		rr	h
		rr	l
		sla	b
		rr	d
		rr	e
		call	sub_AFC9
		ld	de, 3
		add	ix, de
		add	iy, de
		ret
; End of function sub_A53F


; =============== S U B	R O U T	I N E =======================================


HL_CrdNdiv4_InvSign:			; CODE XREF: DirectObject+6Bp
					; DirectObject+71p ...
		ld	a, (ix+2)
		xor	80h ; 'А'
		jr	loc_A572
; End of function HL_CrdNdiv4_InvSign


; =============== S U B	R O U T	I N E =======================================


HL_CrdNdiv4:				; CODE XREF: DirectObject+4Fp
					; DirectObject+55p ...
		ld	a, (ix+2)

loc_A572:				; CODE XREF: HL_CrdNdiv4_InvSign+5j
		ld	l, (ix+0)
		ld	h, (ix+1)
		srl	h
		rr	l
		rla
		rr	h
		rr	l
		ld	de, 3
		add	ix, de
		ret
; End of function HL_CrdNdiv4


; =============== S U B	R O U T	I N E =======================================

; Attributes: noreturn

MoveObject:				; CODE XREF: DirectObject:MoveObject1j
		push	iy
		ld	iy, deltaCrdX
		push	ix
		ld	ix, deltaCrdX
		call	CalcLenVector
; ---------------------------------------------------------------------------
		pop	ix
		ld	(LenVector), de
		push	ix
		ld	de, 9
		add	ix, de
		call	ScalarMultVectors
		ld	de, 6
		add	ix, de
		ld	h, a
		push	hl
		call	ScalarMultVectors
		pop	hl
		ld	l, a
		push	hl
		ld	de, 6
		add	ix, de
		call	ScalarMultVectors
		pop	hl
		pop	ix
		pop	iy
		ld	d, a
		and	7Fh ; ''
		jr	z, loc_A5E7
		ld	c, a
		srl	c
		srl	c
		srl	c
		adc	a, c
		srl	a
		srl	a
		cp	0Fh
		jr	c, loc_A5D7
		ld	a, 0Fh

loc_A5D7:				; CODE XREF: MoveObject+4Cj
		ld	b, a
		ld	a, d
		xor	l
		and	80h ; 'А'
		rra
		or	b
		ld	(ix+1Dh), a
		and	3Fh ; '?'
		cp	4
		jr	nc, loc_A605

loc_A5E7:				; CODE XREF: MoveObject+3Cj
		ld	a, h
		bit	7, a
		jr	z, loc_A5FB
		and	7Fh ; ''
		add	a, 50h ; 'P'
		srl	a
		cpl
		add	a, 68h ; 'h'
		srl	a
		cp	40h ; '@'
		jr	c, loc_A5FD

loc_A5FB:				; CODE XREF: MoveObject+63j
		ld	a, 3Fh ; '?'

loc_A5FD:				; CODE XREF: MoveObject+72j
		sla	l
		rra
		srl	a
		ld	(ix+1Eh), a

loc_A605:				; CODE XREF: MoveObject+5Ej
		xor	a
		bit	7, (ix+25h)
		jr	z, loc_A60E
		ld	a, 2

loc_A60E:				; CODE XREF: MoveObject+83j
		ld	(ix+1Ch), a
		ret	nz
		ld	de, (LenVector)
		ld	a, d
		sla	h
		jr	nc, loc_A686
		ld	b, 0Ah
		bit	0, (ix+26h)
		jr	z, loc_A625
		ld	b, 2

loc_A625:				; CODE XREF: MoveObject+9Aj
		cp	b
		jr	c, loc_A632
		ld	b, 1
		cp	10h
		jr	nc, loc_A62F
		inc	b

loc_A62F:				; CODE XREF: MoveObject+A5j
		ld	(ix+1Ch), b

loc_A632:				; CODE XREF: MoveObject+9Fj
		bit	2, (ix+21h)
		ret	z
		ld	a, (ix+8)
		xor	(ix+0Eh)
		and	80h ; 'А'
		ret	z
		ld	b, (iy+2)
		ld	a, (PilotRating2) ; >=#84 в трюме десант
					; 7,=1 война была или идёт
					; 6,=1 война закончилать
					; 3-1 -	ранг (=текущая стадия войны)
		and	a
		jr	nz, loc_A64B
		srl	b

loc_A64B:				; CODE XREF: MoveObject+C0j
		call	IntRnd_A
		cp	b
		ret	nc
		ld	a, (ix+26h)
		and	0Fh
		ret	nz
		ld	a, d
		cp	0Ch
		ret	nc
		sla	h
		ret	nc
		call	sub_FA84
		ret	nc
		ld	a, d
		cp	4
		jr	c, loc_A669
		sla	h
		ret	nc

loc_A669:				; CODE XREF: MoveObject+DDj
		ld	hl, nBeepFX
		set	6, (hl)
		bit	7, (ix+8)
		jr	z, loc_A67D
		ld	a, (DamageRearShield)
		add	a, 7
		ld	(DamageRearShield), a
		ret
; ---------------------------------------------------------------------------

loc_A67D:				; CODE XREF: MoveObject+EBj
		ld	a, (DamageFrontShield)
		add	a, 7
		ld	(DamageFrontShield), a
		ret
; ---------------------------------------------------------------------------

loc_A686:				; CODE XREF: MoveObject+92j
		ld	b, 0Bh
		bit	0, (ix+26h)
		jr	z, loc_A690
		ld	b, 3

loc_A690:				; CODE XREF: MoveObject+105j
		cp	b
		ret	c
		ld	b, 0FFh
		cp	12h
		jr	c, loc_A699
		dec	b

loc_A699:				; CODE XREF: MoveObject+10Fj
		ld	(ix+1Ch), b
		ret
; End of function MoveObject


; =============== S U B	R O U T	I N E =======================================

; Умножение hl = -1 * high((a*2) * (b*2))
; Вх:  b - первый множитель, 7,b - знак
;      a - второй множитель, 7,a - знак
; Вых: l,a = high((a*2)	* (b*2))
;	l,a = high((a*2) * (b*2))
;	7,h - знак

Mult_HL_A2_B2:				; CODE XREF: ScalarMultVectors+6p
					; ScalarMultVectors+10p ...
		ld	c, a
		xor	b
		xor	80h ; 'А'
		sla	b
		sla	c
		push	af
		call	Mult_A_hiB_C
		pop	hl
		ld	l, a
		ret
; End of function Mult_HL_A2_B2


; =============== S U B	R O U T	I N E =======================================


ScalarMultVectors:			; CODE XREF: MoveObject+1Cp
					; MoveObject+26p ...
		ld	b, (ix+1)
		ld	a, (iy+1)
		call	Mult_HL_A2_B2	; Умножение hl = -1 * high((a*2) * (b*2))
					; Вх:  b - первый множитель, 7,b - знак
					;      a - второй множитель, 7,a - знак
					; Вых: l,a = high((a*2)	* (b*2))
					;	l,a = high((a*2) * (b*2))
					;	7,h - знак
		push	hl
		ld	b, (ix+3)
		ld	a, (iy+3)
		call	Mult_HL_A2_B2	; Умножение hl = -1 * high((a*2) * (b*2))
					; Вх:  b - первый множитель, 7,b - знак
					;      a - второй множитель, 7,a - знак
					; Вых: l,a = high((a*2)	* (b*2))
					;	l,a = high((a*2) * (b*2))
					;	7,h - знак
		push	hl
		ld	b, (ix+5)
		ld	a, (iy+5)
		call	Mult_HL_A2_B2	; Умножение hl = -1 * high((a*2) * (b*2))
; End of function ScalarMultVectors	; Вх:  b - первый множитель, 7,b - знак
					;      a - второй множитель, 7,a - знак
					; Вых: l,a = high((a*2)	* (b*2))
					;	l,a = high((a*2) * (b*2))
					;	7,h - знак

; =============== S U B	R O U T	I N E =======================================




; === sub_A7D9 === CODE XREF: sub_7E02+17j
sub_A7D9:				; CODE XREF: sub_7E02+17j
					; sub_7E02:loc_7E9Dj

; FUNCTION CHUNK AT A8DE SIZE 0000000E BYTES

		ld	a, (Mission2)
		dec	a
		jr	nz, loc_A7E6
		bit	6, (ix+26h)
		jp	nz, objPrintLaser ; de - координаты откуда стреляет

loc_A7E6:				; CODE XREF: sub_A7D9+4j
		ld	hl, (DistanceToObj)
		ld	a, h
		cp	(iy+0Bh)
		jr	c, objPrnObj
		ld	a, (CrdXProjection)
		cp	7Fh ; ''
		jp	nc, objPrintLaser ; de - координаты откуда стреляет
		ld	c, a
		ld	a, (CrdYProjection)
		cp	3Fh ; '?'
		jp	nc, objPrintLaser ; de - координаты откуда стреляет
		ld	b, a
		ld	a, (WhatOnScreen)
		bit	0, a
		ld	a, (CrdXYsign)
		jr	z, loc_A80D
		xor	2

loc_A80D:				; CODE XREF: sub_A7D9+30j
		ld	d, a
		bit	1, d
		jr	nz, loc_A818
		ld	a, c
		add	a, 7Fh ; ''
		ld	c, a
		jr	loc_A81C
; ---------------------------------------------------------------------------

loc_A818:				; CODE XREF: sub_A7D9+37j
		ld	a, 7Fh ; ''
		sub	c
		ld	c, a

loc_A81C:				; CODE XREF: sub_A7D9+3Dj
		bit	0, d
		jr	z, loc_A826
		ld	a, b
		add	a, 3Fh ; '?'
		ld	b, a
		jr	loc_A82A
; ---------------------------------------------------------------------------

loc_A826:				; CODE XREF: sub_A7D9+45j
		ld	a, 3Fh ; '?'
		sub	b
		ld	b, a

loc_A82A:				; CODE XREF: sub_A7D9+4Bj
		call	objPrnPixel
		inc	b
		call	objPrnPixel
		inc	c
		call	objPrnPixel
		dec	b
		call	objPrnPixel
		jp	objPrintLaser	; de - координаты откуда стреляет
; End of function sub_A7D9


; =============== S U B	R O U T	I N E =======================================


objPrnPixel:				; CODE XREF: sub_78B1+13Fp
					; sub_A7D9:loc_A82Ap ...
		ld	a, b
		call	22B1h
		ld	d, a
		inc	d
		ld	a, 1

loc_A844:				; CODE XREF: objPrnPixel+Aj
		rrca
		dec	d
		jr	nz, loc_A844

loc_A848:				; DATA XREF: RAM:B35Dw	RAM:B38Fw
		set	7, h
		xor	(hl)
		ld	(hl), a
		ret
; End of function objPrnPixel


; =============== S U B	R O U T	I N E =======================================


objPrnObj:				; CODE XREF: sub_A7D9+14j
		push	iy
		call	CalcMatrixRotate
		pop	iy
		ld	hl, (DistanceToObj)
		add	hl, hl
		ld	b, 1Fh
		jr	c, loc_A861
		ld	a, h
		cp	b
		jr	nc, loc_A861
		ld	b, h

loc_A861:				; CODE XREF: objPrnObj+Dj
					; objPrnObj+11j
		ld	a, b
		sub	2
		jr	nc, loc_A867
		xor	a

loc_A867:				; CODE XREF: objPrnObj+17j
		ld	(DistanceToObj), a
		ld	a, 0FDh	; '¤'
		ld	(PointTablCrdPrn), a
		call	CalcVisibleVerges
		call	MakeTablPrnCrdVertex
		call	PrintObject

loc_A878:				; CODE XREF: sub_7E02:loc_7EA6j
		bit	3, (ix+20h)
		ret	z		; нет выстрела из лазера
		ld	a, (iy+14h)	; вершина с лазером
		call	GetPrnCrdVertex	; Получение печатных координат вершины из таблицы по номеру
					; Вх:  a - номер вершины
					; Вых: de - печатные координаты	вершины
					;      a - знаки координат
					;      z =1 вершина печатается
					;      hl- не определен
		ld	c, a

objPrnLaser:				; CODE XREF: sub_A7D9+110j
		call	SetBit2ExtSlot_rndA
		ld	l, a
		sub	0FFh
		neg
		ld	h, a
		ld	a, c
		cpl
		and	3
		sla	c
		sla	c
		or	c
		ld	c, a
		call	PrnLineInBuf_clip
		jr	loc_A8A7
; ---------------------------------------------------------------------------
unk_A89C:	db  68h	; h		; DATA XREF: CopyLocator2Scr+3Fo
		db  94h	; Ф
		db  95h	; Х
		db 0B4h	; ┤
		db 0B5h	; ╡
		db  87h	; З
		db  88h	; И
		db 0A7h	; з
		db 0A8h	; и
		db 0A8h	; и
		db 0FFh
; ---------------------------------------------------------------------------

loc_A8A7:				; CODE XREF: objPrnObj+4Dj
		res	3, (ix+20h)
		ret
; End of function objPrnObj


; =============== S U B	R O U T	I N E =======================================


objDestroyed2:				; CODE XREF: sub_7E02+8j
		ld	a, (CrdXProjection)
		ld	(ix+1Ch), a
		ld	a, (CrdYProjection)
		ld	(ix+1Dh), a
		ld	a, (RadiusBlast)
		ld	(ix+1Fh), a
		ld	a, (WhatOnScreen)
		bit	0, a
		ld	a, (CrdXYsign)
		jr	z, loc_A8CA
		xor	2

loc_A8CA:				; CODE XREF: objDestroyed2+1Aj
		ld	c, a
		xor	a
		rr	c
		rla
		rr	c
		rla
		xor	3
		add	a, 0Ch
		ld	(ix+1Eh), a
		set	5, (ix+20h)
		ret
; End of function objDestroyed2

; ---------------------------------------------------------------------------
; START	OF FUNCTION CHUNK FOR sub_A7D9

objPrintLaser:				; CODE XREF: PrnObjIfVisible+9Fj
					; PrnObjIfVisible+ABj ...
		call	GetCrdsProjection ; de - координаты откуда стреляет
		bit	3, (ix+20h)
		call	SetCrdCDEinExtSlot
		ret	z
		jp	objPrnLaser
; END OF FUNCTION CHUNK	FOR sub_A7D9

; =============== S U B	R O U T	I N E =======================================


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


; === sub_AA2B === CODE XREF: MakeTablPrnCrdVertex+1Ep
sub_AA2B:				; CODE XREF: MakeTablPrnCrdVertex+1Ep
		ld	hl, CrdNVertex	; координата N текущей вершины
		ld	a, (hl)
		dec	hl
		ld	(hl), a
; End of function sub_AA2B


; =============== S U B	R O U T	I N E =======================================




; === sub_AA31 === CODE XREF: CalcVisibleVerges+2Dp
sub_AA31:				; CODE XREF: CalcVisibleVerges+2Dp
		ld	hl, (TempAdr)	; адрес	текущей	вершины
		ld	a, (hl)
		ld	(CrdNVertex), a	; координата N текущей вершины
		or	1
		ret
; End of function sub_AA31

; ---------------------------------------------------------------------------
		db  9Ch	; Ь
		db 0B8h	; ╕

; =============== S U B	R O U T	I N E =======================================


RotateCrdN_Vertex:			; CODE XREF: MakeTablPrnCrdVertex+2FBp
					; MakeTablPrnCrdVertex+304p ...
		ld	a, (VisiblePrevVertex)
		and	a
		jr	nz, loc_AA8B
		ld	a, (CrdNVertex)	; координата N текущей вершины
		rrca
		jr	c, loc_AA8B
		ld	hl, CrdNVertex	; координата N текущей вершины
		ld	a, (hl)
		dec	hl
		xor	(hl)
		ld	b, a
		ld	l, (iy+12h)
		ld	h, (iy+13h)
		ld	e, (iy+14h)
		ld	d, (iy+15h)
		xor	h
		and	80h ; 'А'
		ld	h, a
		ld	(iy+13h), h
		rl	b
		ld	a, b
		xor	d
		and	80h ; 'А'
		ld	d, a
		ld	(iy+15h), d
		call	AddSign_L_E
		ld	l, a
		ld	e, (iy+16h)
		ld	d, (iy+17h)
		ld	a, b
		rla
		xor	d
		and	80h ; 'А'
		ld	d, a
		ld	(iy+17h), d
		call	AddSign_L_E
		ld	l, a
		ld	de, 6
		add	iy, de
		jr	loc_AAE3
; ---------------------------------------------------------------------------

loc_AA8B:				; CODE XREF: RotateCrdN_Vertex+4j
					; RotateCrdN_Vertex+Aj
		ld	b, (ix+0)
		call	sub_AB1E
		ld	a, (CrdNVertex)	; координата N текущей вершины
		xor	(iy+1)
		rla
		rr	h
		ld	(iy+12h), l
		ld	(iy+13h), h
		ex	de, hl
		inc	iy
		inc	iy
		ld	b, (ix+1)
		call	sub_AB1E
		ld	a, (CrdNVertex)	; координата N текущей вершины
		rla
		xor	(iy+1)
		rla
		rr	h
		ld	(iy+12h), l
		ld	(iy+13h), h
		call	AddSign_L_E
		ld	l, a
		ex	de, hl
		inc	iy
		inc	iy
		ld	b, (ix+2)
		call	sub_AB1E
		ld	a, (CrdNVertex)	; координата N текущей вершины
		rla
		rla
		xor	(iy+1)
		rla
		rr	h
		ld	(iy+12h), l
		ld	(iy+13h), h
		call	AddSign_L_E
		ld	l, a
		inc	iy
		inc	iy

loc_AAE3:				; CODE XREF: RotateCrdN_Vertex+4Cj
		ld	a, (VertexIncFactor)
		ld	b, a
		xor	a
		ld	c, h
		res	7, h

loc_AAEB:				; CODE XREF: RotateCrdN_Vertex+B3j
		srl	h
		rr	l
		rra
		djnz	loc_AAEB
		jr	nc, loc_AAF5
		inc	hl

loc_AAF5:				; CODE XREF: RotateCrdN_Vertex+B5j
		bit	7, c
		ret	z
		set	7, h
		ret
; End of function RotateCrdN_Vertex


; =============== S U B	R O U T	I N E =======================================


AddSign_L_E:				; CODE XREF: sub_A6C9+1p sub_A6C9+Ap ...
		ld	a, h
		xor	d
		rla
		jr	c, loc_AB03
		ld	a, l
		add	a, e
		ret
; ---------------------------------------------------------------------------

loc_AB03:				; CODE XREF: AddSign_L_E+3j
		ld	a, l
		sub	e
		ret	nc
		neg
		and	a
		ld	h, d
		ret
; End of function AddSign_L_E


; =============== S U B	R O U T	I N E =======================================


Mult_A_hiB_C:				; CODE XREF: sub_A0FD+7p
					; CalcNewCrdNobj+32p ...
		ld	a, b
		sub	c
		jr	nc, loc_AB11
		neg

loc_AB11:				; CODE XREF: Mult_A_hiB_C+2j
		srl	a
		ld	h, 0BFh	; '┐'
		ld	l, a
		ld	d, (hl)
		ld	a, b
		add	a, c
		rra
		ld	l, a
		ld	a, (hl)
		sub	d
		ret
; End of function Mult_A_hiB_C


; =============== S U B	R O U T	I N E =======================================




; === sub_AB1E === CODE XREF: RotateCrdN_Vertex+51p
sub_AB1E:				; CODE XREF: RotateCrdN_Vertex+51p
					; RotateCrdN_Vertex+6Bp ...
		ld	a, (VertexIncFactor)

loc_AB21:				; CODE XREF: sub_AB1E+6j
		sla	b
		dec	a
		jr	nz, loc_AB21
		ld	c, (iy+1)
		res	7, c
		push	de
		call	Mult_A_hiB_C
		ld	l, a
		ld	h, 0
		pop	de
		ret
; End of function sub_AB1E


; =============== S U B	R O U T	I N E =======================================


Mult_HL_B_C:				; CODE XREF: Sqr_HL_wordAdrIX+Ep
					; sub_9F68+5p ...
		ld	a, b
		sub	c
		jr	nc, loc_AB3A
		neg

loc_AB3A:				; CODE XREF: Mult_HL_B_C+2j
		srl	a
		ld	h, 0BEh	; '╛'
		ld	l, a
		ld	e, (hl)
		inc	h
		ld	d, (hl)
		dec	h
		ld	a, b
		add	a, c
		rra
		ld	l, a
		ld	a, (hl)
		inc	h
		ld	h, (hl)
		ld	l, a
		and	a
		sbc	hl, de
		ret
; End of function Mult_HL_B_C


; =============== S U B	R O U T	I N E =======================================


AddCrdNobj_CrdNvertex:			; CODE XREF: MakeTablPrnCrdVertex+332p
					; MakeTablPrnCrdVertex+351p ...
		ld	c, d
		rl	e
		rl	d
		ld	a, b
		xor	c
		jr	nz, loc_AB5B
		add	hl, de
		ld	a, b
		ret
; ---------------------------------------------------------------------------

loc_AB5B:				; CODE XREF: AddCrdNobj_CrdNvertex+7j
		sbc	hl, de
		jr	nc, loc_AB66
		add	hl, de
		ex	de, hl
		and	a
		sbc	hl, de
		ld	a, c
		ret
; ---------------------------------------------------------------------------

loc_AB66:				; CODE XREF: AddCrdNobj_CrdNvertex+Ej
		ld	a, h
		or	l
		ld	a, c
		ret	z
		pop	de
		pop	af
		neg
		push	af
		push	de
		dec	hl
		ld	a, b
		ret
; End of function AddCrdNobj_CrdNvertex

; ---------------------------------------------------------------------------
; START	OF FUNCTION CHUNK FOR MakeTablPrnCrdVertex

RotateCrdVertex:			; CODE XREF: MakeTablPrnCrdVertex+2E0j
					; MakeTablPrnCrdVertex+2F8j
		ld	a, (VisiblePrevVertex)
		and	a
		jr	nz, loc_AB90
		ld	a, (CrdNVertex)	; координата N текущей вершины
		rrca
		jr	c, loc_AB90
		call	sub_ABC0
		ld	(VertexCrdX), hl
		call	sub_ABC0
		ld	(VertexCrdY), hl
		call	sub_ABC0
		jr	loc_AB9F
; ---------------------------------------------------------------------------

loc_AB90:				; CODE XREF: MakeTablPrnCrdVertex+1AEj
					; MakeTablPrnCrdVertex+1B4j
		call	RotateCrdN
		ld	(VertexCrdX), hl
		call	RotateCrdN
		ld	(VertexCrdY), hl
		call	RotateCrdN

loc_AB9F:				; CODE XREF: MakeTablPrnCrdVertex+1C5j
		ld	(VertexCrdZ), hl
		xor	a
		ld	(VertexCrdXlow), a
		ld	(VertexCrdYlow), a
		ld	(VertexCrdZlow), a
		jp	loc_ACDF
; END OF FUNCTION CHUNK	FOR MakeTablPrnCrdVertex

; =============== S U B	R O U T	I N E =======================================




; === sub_ABAF === CODE XREF: GenerateProductPricep
sub_ABAF:				; CODE XREF: GenerateProductPricep
		ld	hl, RndWord
		ld	a, (Pilot_CrdYSystem)
		add	a, (hl)
		ret
; End of function sub_ABAF

; ---------------------------------------------------------------------------
		db 0C9h	; ╔
		db  21h	; !
		db    5
		db  71h	; q
		db  3Ah	; :
		db  97h	; Ч
		db 0D0h	; ╨
		db  86h	; Ж
		db 0C9h	; ╔

; =============== S U B	R O U T	I N E =======================================




; === sub_ABC0 === CODE XREF: MakeTablPrnCrdVertex+1B6p
sub_ABC0:				; CODE XREF: MakeTablPrnCrdVertex+1B6p
					; MakeTablPrnCrdVertex+1BCp ...
		ld	hl, 0AA0Fh
		ld	a, (hl)
		dec	hl
		xor	(hl)
		ld	b, a
		ld	l, (iy+12h)
		ld	h, (iy+13h)
		ld	e, (iy+14h)
		ld	d, (iy+15h)
		xor	h
		and	80h ; 'А'
		ld	h, a
		ld	(iy+13h), h
		rl	b
		ld	a, b
		xor	d
		and	80h ; 'А'
		ld	d, a
		ld	(iy+15h), d
		call	AddSign_L_E
		ld	l, a
		ld	e, (iy+16h)
		ld	d, (iy+17h)
		ld	a, b
		rla
		xor	d
		and	80h ; 'А'
		ld	d, a
		ld	(iy+17h), d
		call	AddSign_L_E
		ld	l, a
		ld	de, 6
		add	iy, de
		ret
; End of function sub_ABC0


; =============== S U B	R O U T	I N E =======================================


RotateCrdN:				; CODE XREF: MakeTablPrnCrdVertex:loc_AB90p
					; MakeTablPrnCrdVertex+1CDp ...
		ld	b, (ix+0)
		ld	c, (iy+1)
		res	7, c
		call	Mult_A_hiB_C
		ld	l, a
		ld	a, (CrdNVertex)	; координата N текущей вершины
		xor	(iy+1)
		and	80h ; 'А'
		ld	h, a
		ld	(iy+12h), l
		ld	(iy+13h), h
		ex	de, hl

loc_AC1D:
		ld	b, (ix+1)
		ld	c, (iy+3)
		res	7, c
		push	de
		call	Mult_A_hiB_C
		pop	de
		ld	l, a
		ld	a, (CrdNVertex)	; координата N текущей вершины
		rla
		xor	(iy+3)
		and	80h ; 'А'
		ld	h, a
		ld	(iy+14h), l
		ld	(iy+15h), h
		call	AddSign_L_E
		ld	l, a
		ex	de, hl
		ld	b, (ix+2)
		ld	c, (iy+5)
		res	7, c
		push	de
		call	Mult_A_hiB_C
		pop	de
		ld	l, a
		ld	a, (CrdNVertex)	; координата N текущей вершины
		rla
		rla
		xor	(iy+5)
		and	80h ; 'А'
		ld	h, a
		ld	(iy+16h), l
		ld	(iy+17h), h
		call	AddSign_L_E
		ld	l, a
		ld	de, 6
		add	iy, de
		ret
; End of function RotateCrdN


; =============== S U B	R O U T	I N E =======================================

; Attributes: noreturn

Div_C_DEc_HLa:				; CODE XREF: PrnObjIfVisible:loc_A71Ep
					; PrnObjIfVisible+4Fp ...
		ld	b, 9

loc_AC6B:				; CODE XREF: Div_C_DEc_HLa+10j
		dec	b
		jr	z, loc_AC84
		rla
		adc	hl, hl
		jr	c, loc_AC80
		rl	c
		rl	e
		rl	d
		jr	nc, loc_AC6B
		rr	d
		rr	e
		scf

loc_AC80:				; CODE XREF: Div_C_DEc_HLa+8j
		rr	h
		rr	l

loc_AC84:				; CODE XREF: Div_C_DEc_HLa+3j
		ld	a, d
		ld	c, 0FFh
		cp	h
		ret	nc
		ld	b, h
		dec	c

loc_AC8B:				; CODE XREF: Div_C_DEc_HLa+2Dj
					; Div_C_DEc_HLa+34j
		sla	e
		rla
		jr	c, loc_AC98
		cp	b
		jr	nc, loc_AC98
		sla	c
		ret	nc
		jr	loc_AC8B
; ---------------------------------------------------------------------------

loc_AC98:				; CODE XREF: Div_C_DEc_HLa+25j
					; Div_C_DEc_HLa+28j
		sub	b
		scf
		rl	c
		ret	nc
		jr	loc_AC8B
; End of function Div_C_DEc_HLa

; ---------------------------------------------------------------------------


; === sub_AE01 === CODE XREF: MakeTablPrnCrdVertex+Cp
sub_AE01:				; CODE XREF: MakeTablPrnCrdVertex+Cp
		ld	e, (iy+0Ch)
		add	ix, de
		ld	e, a
		push	af
		add	a, a
		add	a, e
		ld	e, a
		pop	af
		jr	loc_AE16
; End of function sub_AE01


; =============== S U B	R O U T	I N E =======================================




; === sub_AE0E === CODE XREF: CalcVisibleVerges+Fp
sub_AE0E:				; CODE XREF: CalcVisibleVerges+Fp
		ld	e, (ix+10h)
		add	ix, de
		ld	e, (iy+5)

loc_AE16:				; CODE XREF: sub_AE01+Bj
		ld	(TempAdr), ix	; адрес	текущей	вершины
		ld	d, 0
		ret
; End of function sub_AE0E


; =============== S U B	R O U T	I N E =======================================




; === sub_AE1D === CODE XREF: MakeTablPrnCrdVertex:loc_ADF2p
sub_AE1D:				; CODE XREF: MakeTablPrnCrdVertex:loc_ADF2p
		ld	bc, 3
		ld	hl, (TempAdr)	; адрес	текущей	вершины
		add	hl, bc
		jr	loc_AE2D
; End of function sub_AE1D


; =============== S U B	R O U T	I N E =======================================




; === sub_AE26 === CODE XREF: CalcVisibleVerges+7Dp
sub_AE26:				; CODE XREF: CalcVisibleVerges+7Dp
		ld	bc, 3
		ld	hl, (TempAdr)	; адрес	текущей	вершины
		inc	hl

loc_AE2D:				; CODE XREF: sub_AE1D+7j
		ld	(TempAdr), hl	; адрес	текущей	вершины
		bit	0, (hl)
		ret	nz
		ld	c, b
		ret
; End of function sub_AE26


; =============== S U B	R O U T	I N E =======================================


SetObjInSlot_Intro:			; CODE XREF: SetShip4Intro+7p
		ld	hl, AdrDataShips
		ld	b, a
		jp	loc_B89C
; End of function SetObjInSlot_Intro

; ---------------------------------------------------------------------------
byte_AE3C:	db 6, 7, 8, 0Bh, 0Ch, 0Dh, 11h,	0Eh, 14h, 0Fh, 12h, 13h
					; DATA XREF: sub_728D+12Ao
					; номера кораблей для интро
		db  7Eh	; ~
		db  22h	; "
		db 0D8h	; ╪
		db 0F3h	; є
		db  76h	; v
		db 0C9h	; ╔
		db 0C9h	; ╔
		db 0C9h	; ╔

; =============== S U B	R O U T	I N E =======================================

; Получение печатных координат вершины из таблицы по номеру
; Вх:  a - номер вершины
; Вых: de - печатные координаты	вершины
;      a - знаки координат
;      z =1 вершина печатается
;      hl- не определен

GetPrnCrdVertex:			; CODE XREF: PrintObject_2:loc_6FB5p
					; PrintObject_2+83p ...
		ld	b, a
		add	a, a
		ld	hl, TablCrdPrnVertex
		jr	nc, loc_AE58
		inc	h

loc_AE58:				; CODE XREF: GetPrnCrdVertex+5j
		add	a, b
		jr	nc, loc_AE5C
		inc	h

loc_AE5C:				; CODE XREF: GetPrnCrdVertex+9j
		add	a, l
		ld	l, a
		jr	nc, loc_AE61
		inc	h

loc_AE61:				; CODE XREF: GetPrnCrdVertex+Ej
		ld	d, (hl)
		inc	hl
		ld	e, (hl)
		inc	hl
		ld	a, (hl)
		bit	7, a
		ret
; End of function GetPrnCrdVertex


; =============== S U B	R O U T	I N E =======================================


CorrAxirY_SKO:				; CODE XREF: sub_9D40+ECp sub_A28D+17p
		push	ix
		push	ix
		ex	(sp), iy
		ld	de, 9
		add	ix, de
		ld	e, 0Fh
		add	iy, de
		call	ScalarMultVectors
		ld	e, a
		call	sub_AE8A
		call	sub_AE8A
		call	sub_AE8A
		pop	iy
		pop	ix
		ret
; End of function CorrAxirY_SKO


; =============== S U B	R O U T	I N E =======================================




; === sub_AE8A === CODE XREF: CorrAxirY_SKO+13p
sub_AE8A:				; CODE XREF: CorrAxirY_SKO+13p
					; CorrAxirY_SKO+16p ...
		ld	c, (ix+1)
		ld	b, e
		sla	b
		sla	c
		call	Mult_A_hiB_C
		ld	c, a
		ld	a, e
		xor	(ix+1)
		rla
		rr	c
		ld	b, (iy+1)
		ld	a, b
		xor	c
		sla	c
		rl	h
		sla	b
		rl	h
		rla
		ld	a, b
		jr	c, loc_AEB6
		add	a, c
		jr	nc, loc_AEBD
		ld	a, 0FFh
		jp	loc_AEBD
; ---------------------------------------------------------------------------

loc_AEB6:				; CODE XREF: sub_AE8A+22j
		sub	c
		jr	nc, loc_AEBD
		neg
		rr	h

loc_AEBD:				; CODE XREF: sub_AE8A+25j sub_AE8A+29j ...
		rr	h
		rra
		ld	(iy+1),	a
		inc	ix
		inc	ix
		inc	iy
		inc	iy
		ret
; End of function sub_AE8A


; =============== S U B	R O U T	I N E =======================================


CalcMatrixRotate:			; CODE XREF: objPrnObj+2p
		ld	iy, MatrixRotate
		ld	b, (ix+0Ch)
		ld	c, (ix+14h)
		ld	d, (ix+0Eh)
		ld	e, (ix+12h)
		call	MultCrdsVectors
		ld	(ix+16h), h
		ld	(iy+1),	h
		ld	b, (ix+0Eh)
		ld	(iy+11h), b
		ld	c, (ix+10h)
		ld	(iy+3),	c
		ld	d, (ix+0Ah)
		ld	e, (ix+14h)
		ld	(iy+0Fh), e

loc_AEFA:
		call	MultCrdsVectors
		ld	(ix+18h), h
		ld	(iy+7),	h
		ld	b, (ix+0Ah)
		ld	(iy+5),	b
		ld	c, (ix+12h)
		ld	(iy+9),	c
		ld	d, (ix+0Ch)
		ld	(iy+0Bh), d
		ld	e, (ix+10h)
		call	MultCrdsVectors
		ld	(ix+1Ah), h
		ld	(iy+0Dh), h
		ret
; End of function CalcMatrixRotate


; =============== S U B	R O U T	I N E =======================================


MultCrdsVectors:			; CODE XREF: CalcMatrixRotate+10p
					; CalcMatrixRotate:loc_AEFAp ...
		sla	b
		rra
		xor	c
		sla	c
		and	80h ; 'А'
		ld	h, a
		sla	d
		rra
		xor	e
		sla	e
		and	80h ; 'А'
		xor	h
		sla	h
		rra
		xor	80h ; 'А'
		push	af
		push	de
		call	Mult_HL_B_C
		pop	bc
		push	hl
		call	Mult_HL_B_C
		pop	de
		pop	af
		bit	6, a
		jr	z, loc_AF51
		add	hl, de
		jr	nc, loc_AF5E
		ld	hl, 0FFFFh
		jr	loc_AF5E
; ---------------------------------------------------------------------------

loc_AF51:				; CODE XREF: MultCrdsVectors+25j
		ex	de, hl
		and	a
		sbc	hl, de
		jr	nc, loc_AF5E
		add	hl, de
		ex	de, hl
		and	a
		sbc	hl, de
		xor	80h ; 'А'

loc_AF5E:				; CODE XREF: MultCrdsVectors+28j
					; MultCrdsVectors+2Dj ...
		rla
		rr	h
		rr	l
		ret
; End of function MultCrdsVectors

; ---------------------------------------------------------------------------
		db 0DDh	; ▌
		db 0E5h	; х
		db 0DDh	; ▌
		db  21h	; !
		db    0
		db    0
		db  47h	; G
		db 0C5h	; ┼
		db 0D5h	; ╒
		db 0C5h	; ┼
		db 0D5h	; ╒
		db 0C5h	; ┼
		db 0D5h	; ╒
		db  43h	; C
		db 0CDh	; ═
		db  34h	; 4
		db 0ABh	; л
		db 0EBh	; ы
		db 0DDh	; ▌
		db  19h
		db 0D1h	; ╤
		db 0C1h	; ┴
		db  42h	; B
		db 0CDh	; ═
		db  34h	; 4
		db 0ABh	; л
		db  1Eh
		db    0
		db  55h	; U
		db 0DDh	; ▌
		db  19h
		db  6Ch	; l
		db  63h	; c
		db  30h	; 0
		db    1
		db  23h	; #
		db 0C1h	; ┴
		db 0D1h	; ╤
		db 0E5h	; х
		db  42h	; B
		db 0CDh	; ═
		db  34h	; 4
		db 0ABh	; л
		db  1Eh
		db    0
		db  55h	; U
		db 0DDh	; ▌
		db  19h
		db 0D1h	; ╤
		db  6Ch	; l
		db  26h	; &
		db    0
		db  30h	; 0
		db    1
		db  23h	; #
		db  19h
		db 0C1h	; ┴
		db 0D1h	; ╤
		db  4Ah	; J
		db 0E5h	; х
		db 0CDh	; ═
		db  34h	; 4
		db 0ABh	; л
		db 0D1h	; ╤
		db  19h
		db 0DDh	; ▌
		db 0E1h	; с
		db 0C9h	; ╔

; =============== S U B	R O U T	I N E =======================================

; Attributes: noreturn

Div_BC_HLAdivDE_1:			; CODE XREF: Div_BC_HLAdivDEj
					; CaclNormalVector+9p ...
		ld	bc, 0FFFEh

loc_AFAB:				; CODE XREF: Div_BC_HLAdivDE_1+14j
					; Div_BC_HLAdivDE_1+1Fj
		add	a, a
		rl	l
		rl	h
		jr	c, loc_AFBE
		sbc	hl, de
		jr	nc, loc_AFC1
		add	hl, de
		sla	c
		rl	b
		ret	nc
		jr	loc_AFAB
; ---------------------------------------------------------------------------

loc_AFBE:				; CODE XREF: Div_BC_HLAdivDE_1+8j
		and	a
		sbc	hl, de

loc_AFC1:				; CODE XREF: Div_BC_HLAdivDE_1+Cj
		scf
		rl	c
		rl	b
		ret	nc
		jr	loc_AFAB
; End of function Div_BC_HLAdivDE_1


; =============== S U B	R O U T	I N E =======================================




; === sub_AFC9 === CODE XREF: RAM:loc_9CA5j
sub_AFC9:				; CODE XREF: RAM:loc_9CA5j
					; sub_A53F+1Ep
		ld	a, d
		xor	80h ; 'А'
		ld	d, a
; End of function sub_AFC9


; =============== S U B	R O U T	I N E =======================================


AddSign_HL_DE:				; CODE XREF: ScalarMultVectors16_8+27p
					; ScalarMultVectors16_8+33j
		ld	a, h
		xor	d
		ld	b, h
		ld	c, d
		res	7, h
		res	7, d
		and	80h ; 'А'
		jr	nz, loc_AFDC
		add	hl, de
		jr	loc_AFE6
; ---------------------------------------------------------------------------

loc_AFDC:				; CODE XREF: AddSign_HL_DE+Aj
		sbc	hl, de
		jr	nc, loc_AFE6
		add	hl, de
		ex	de, hl
		and	a
		sbc	hl, de
		ld	b, c

loc_AFE6:				; CODE XREF: AddSign_HL_DE+Dj
					; AddSign_HL_DE+11j
		srl	h
		rr	l
		bit	7, b
		ret	z
		set	7, h
		ret
; End of function AddSign_HL_DE

; ---------------------------------------------------------------------------
		db    0
		db    4
		db    2
		db    3
		db    0
		db    4
		db    6
		db    5
		db    0
		db    8
		db    2
		db    1
		db    0
		db    8
		db    6
		db    7
		db    0
		db    0
		db    0
		db    2
		db    1
		db    2
		db    0
		db    0
		db    0
		db    0
		db    0
		db    2
		db    2
		db    1
		db    2
		db    2
		db    0
		db    0
		db    0
		db    0
		db    0
		db    2
		db    1
		db    2
		db    2
		db    2
		db    0
		db    0
		db    0
		db    2
		db    2
		db    1
		db    1
		db    2
		db    0
		db    0
		db    0
		db    0
		db    0
		db    2
		db    2
		db    1
		db    2
		db    2
		db    0
		db    0
		db    0
		db    2
		db    0
		db    2
		db    1
		db    2
		db    0
		db    0
		db    0
		db    0
		db    0
		db    2
		db    2
		db    1
		db    2
		db    2
		db    0
		db    0
		db    0
		db    0
		db    0
		db    0

; =============== S U B	R O U T	I N E =======================================


PrnLineInBuf_clip:			; CODE XREF: PrintObject_2+91p
					; sub_7C60+C5p	...

; FUNCTION CHUNK AT B0B6 SIZE 00000082 BYTES
; FUNCTION CHUNK AT B162 SIZE 00000029 BYTES

		ld	a, d
		and	l
		inc	a
		ret	z
		ld	a, e
		and	h
		inc	a
		ret	z
		ld	a, c
		add	a, a
		add	a, a
		xor	c
		cpl
		ld	c, a
		call	sub_B095
		ld	a, d
		sub	78h ; 'x'
		jr	c, loc_B065
		ld	b, 14h
		call	loc_B21B
		ld	e, a
		ld	b, 0
		call	sub_B09C

loc_B065:				; CODE XREF: PrnLineInBuf_clip+14j
		ld	a, e
		sub	3Fh ; '?'
		jr	c, loc_B073
		ld	b, 28h ; '('
		call	loc_B1D7
		ld	d, a
		call	sub_B095

loc_B073:				; CODE XREF: PrnLineInBuf_clip+24j
		ld	a, h
		sub	78h ; 'x'
		jr	c, loc_B085
		ex	de, hl
		ld	b, 5
		call	loc_B21B
		ld	e, a
		ex	de, hl
		ld	b, 0
		call	loc_B0A6

loc_B085:				; CODE XREF: PrnLineInBuf_clip+32j
		ld	a, l
		sub	3Fh ; '?'
		jr	c, loc_B0B6
		ex	de, hl
		ld	b, 0Ah
		call	loc_B1D7
		ld	d, a
		ex	de, hl
		jp	loc_B0B6
; End of function PrnLineInBuf_clip


; =============== S U B	R O U T	I N E =======================================




; === sub_B095 === CODE XREF: PrnLineInBuf_clip+Ep
sub_B095:				; CODE XREF: PrnLineInBuf_clip+Ep
					; PrnLineInBuf_clip+2Cp
		ld	b, 0
		ld	a, 77h ; 'w'
		cp	d
		rl	b
; End of function sub_B095


; =============== S U B	R O U T	I N E =======================================




; === sub_B09C === CODE XREF: PrnLineInBuf_clip+1Ep
sub_B09C:				; CODE XREF: PrnLineInBuf_clip+1Ep
		ld	a, 3Eh ; '>'
		cp	e
		rl	b
		ld	a, 77h ; 'w'
		cp	h
		rl	b
; End of function sub_B09C


loc_B0A6:				; CODE XREF: PrnLineInBuf_clip+3Ep
		ld	a, 3Eh ; '>'
		cp	l
		rl	b
		jr	z, loc_B0B3
		ld	a, b
		rlca
		rlca
		and	b
		and	c
		ret	z

loc_B0B3:				; CODE XREF: RAM:B0ABj
		inc	sp
		inc	sp
		ret	nz
; START	OF FUNCTION CHUNK FOR PrnLineInBuf_clip

loc_B0B6:				; CODE XREF: PrnLineInBuf_clip+44j
					; PrnLineInBuf_clip+4Ej
		ld	a, 7Fh ; ''
		bit	5, c
		jr	nz, loc_B0BD+1
		sub	d

loc_B0BD:				; CODE XREF: PrnLineInBuf_clip+76j
		cp	82h ; 'В'
		ld	d, a
		ld	a, 3Fh ; '?'
		bit	4, c
		jr	z, loc_B0C7+1
		sub	e

loc_B0C7:				; CODE XREF: PrnLineInBuf_clip+80j
		cp	83h ; 'Г'
		ld	e, a
		ld	a, 7Fh ; ''
		bit	1, c
		jr	nz, loc_B0D1+1
		sub	h

loc_B0D1:				; CODE XREF: PrnLineInBuf_clip+8Aj
		cp	84h ; 'Д'
		ld	h, a
		ld	a, 3Fh ; '?'
		bit	0, c
		jr	z, loc_B0DB+1
		sub	l

loc_B0DB:				; CODE XREF: PrnLineInBuf_clip+94j
		cp	85h ; 'Е'
		ld	l, a
		cp	e
		jr	nc, loc_B0E2
		ex	de, hl

loc_B0E2:				; CODE XREF: PrnLineInBuf_clip+9Bj
		push	hl
		ld	a, d
		and	7
		ld	hl, 0B15Ah
		add	a, l
		ld	l, a
		ld	c, (hl)
		ld	a, e
		and	a
		rra
		scf
		rra
		scf
		rra
		ld	l, a
		xor	e
		and	0F8h ; '°'
		xor	e
		ld	h, a
		ld	a, d
		xor	l
		and	0F8h ; '°'
		xor	l
		rrca
		rrca
		rrca
		ld	l, a
		ex	(sp), hl
		ld	a, l
		sub	e
		ld	e, a
		ld	a, h
		sub	d
		push	af
		jr	nc, loc_B10D
		neg

loc_B10D:				; CODE XREF: PrnLineInBuf_clip+C5j
		ld	d, a
		cp	e
		jp	nc, loc_B162
		pop	af
		pop	hl
		ld	a, 1
		jr	c, loc_B11A
		ld	a, 9

loc_B11A:				; CODE XREF: PrnLineInBuf_clip+D2j
		ld	(unk_B152), a
		ld	a, 2Ch ; ','
		adc	a, 0
		ld	(unk_B156), a
		ld	a, h
		and	7
		neg
		add	a, 8
		ld	b, a
		xor	a
		sub	e
		ld	(unk_B150), a
		inc	e
		add	a, d
		jr	nc, loc_B136
		sub	d

loc_B136:				; CODE XREF: PrnLineInBuf_clip+EFj
		ex	af, af'
		ld	a, c
; END OF FUNCTION CHUNK	FOR PrnLineInBuf_clip
; ---------------------------------------------------------------------------
unk_B138:	db 0B6h	; ╢		; DATA XREF: MoveAllObjects:loc_9CD1w
					; j_MoveAllObjects+1B16r
					; тип печати OR/XOR
		db  77h	; w
		db  1Dh
		db 0C8h	; ╚
		db  24h	; $
		db  10h
		db  0Bh
		db    6
		db    8
		db  7Dh	; }
		db 0C6h	; ╞
		db  20h
		db  6Fh	; o
		db  38h	; 8
		db    3
		db  7Ch	; |
		db  90h	; Р
		db  67h	; g
		db    8
		db  82h	; В
		db 0D2h	; ╥
		db  36h	; 6
		db 0B1h	; ▒
		db 0C6h	; ╞
unk_B150:	db 0BDh	; ╜		; DATA XREF: PrnLineInBuf_clip+EAw
		db 0CBh	; ╦
unk_B152:	db    9			; DATA XREF: PrnLineInBuf_clip:loc_B11Aw
		db 0D2h	; ╥
		db  36h	; 6
		db 0B1h	; ▒
unk_B156:	db  2Ch	; ,		; DATA XREF: PrnLineInBuf_clip+DDw
		db 0C3h	; ├
		db  36h	; 6
		db 0B1h	; ▒
		db  80h	; А
		db  40h	; @
		db  20h
		db  10h
		db    8
		db    4
		db    2
		db    1
; ---------------------------------------------------------------------------
; START	OF FUNCTION CHUNK FOR PrnLineInBuf_clip

loc_B162:				; CODE XREF: PrnLineInBuf_clip+CBj
		pop	af
		pop	hl
		ld	a, 1
		jr	c, loc_B16A

loc_B168:				; CODE XREF: RAM:loc_B1E1j
		ld	a, 9

loc_B16A:				; CODE XREF: PrnLineInBuf_clip+122j
		ld	(loc_B19A+1), a

loc_B16D:
		ld	a, 2Ch ; ','
		adc	a, 0
		ld	(loc_B191), a
		ld	a, h
		and	7

loc_B177:				; CODE XREF: RAM:loc_B1F2j
		neg
		add	a, 8
		ld	b, a
		xor	a
		sub	d
		ld	(loc_B1A3+1), a
		inc	d
		add	a, e
		jr	nc, loc_B186
		sub	e

loc_B186:				; CODE XREF: PrnLineInBuf_clip+13Fj
		ex	af, af'
		ld	a, c
		call	loc_B198
; END OF FUNCTION CHUNK	FOR PrnLineInBuf_clip

loc_B18B:				; DATA XREF: MoveAllObjects+19w
		or	(hl)
		ld	(hl), a
		ret
; ---------------------------------------------------------------------------

loc_B18E:				; CODE XREF: RAM:B19Cj
					; DATA XREF: MoveAllObjects+1Cw
		or	(hl)
		ld	(hl), a
		xor	a

loc_B191:				; DATA XREF: PrnLineInBuf_clip+12Dw
		inc	l
		ex	af, af'
		add	a, e
		jr	c, loc_B1A3

loc_B196:				; CODE XREF: RAM:B1A0j
		ex	af, af'
		or	c

loc_B198:				; CODE XREF: PrnLineInBuf_clip+144p
					; RAM:loc_B1AAj ...
		dec	d
		ret	z

loc_B19A:				; DATA XREF: PrnLineInBuf_clip:loc_B16Aw
		rrc	c
		jr	c, loc_B18E
		ex	af, af'
		add	a, e
		jp	nc, loc_B196

loc_B1A3:				; CODE XREF: RAM:B194j
					; DATA XREF: PrnLineInBuf_clip+13Aw
		add	a, 0EAh	; 'ъ'
		ex	af, af'

loc_B1A6:				; DATA XREF: MoveAllObjects+1Fw
		or	(hl)
		ld	(hl), a
		ld	a, c
		inc	h

loc_B1AA:
		djnz	loc_B198
		ld	b, 8
		ld	a, l
		add	a, 20h ; ' '
		ld	l, a
		ld	a, c
		jr	c, loc_B198
		ld	a, h
		sub	b
		ld	h, a
		ld	a, c
		jp	loc_B198

; =============== S U B	R O U T	I N E =======================================




; === sub_B1BC === CODE XREF: RAM:loc_B1F4p
sub_B1BC:				; CODE XREF: RAM:loc_B1F4p
					; RAM:loc_B238p
		exx
		ld	hl, 0
		ld	e, l
		rra
		rr	e
		ld	d, a
		ex	af, af'

loc_B1C6:				; CODE XREF: sub_B1BC+17j
		srl	d
		rr	e
		sub	b
		jr	c, loc_B1D0
		add	hl, de
		ret	z
; ---------------------------------------------------------------------------
		db 0FEh
; ---------------------------------------------------------------------------

loc_B1D0:				; CODE XREF: sub_B1BC+Fj
		add	a, b
		srl	b
		jp	nz, loc_B1C6
		ret
; End of function sub_B1BC

; ---------------------------------------------------------------------------

loc_B1D7:				; CODE XREF: PrnLineInBuf_clip+28p
					; PrnLineInBuf_clip+49p
		inc	a
		ex	af, af'
		ld	a, e
		ld	e, 3Eh ; '>'
		bit	2, c
		jr	z, loc_B1E1+1
		sub	l

loc_B1E1:				; CODE XREF: RAM:B1DEj
		jr	c, loc_B168
		exx
		rra
		ld	b, a
		exx
		ld	a, d
		bit	3, c
		jr	z, loc_B1F2+1
		sub	h
		push	af
		jr	nc, loc_B1F4
		ld	a, h
		sub	d

loc_B1F2:				; CODE XREF: RAM:B1EAj
		jr	z, loc_B177+1

loc_B1F4:				; CODE XREF: RAM:B1EEj
		call	sub_B1BC
		add	hl, hl
		ld	a, h
		ex	af, af'
		exx
		bit	3, c
		jr	z, loc_B205
		pop	af
		jr	nc, loc_B205
		ex	af, af'
		add	a, d
		ret
; ---------------------------------------------------------------------------

loc_B205:				; CODE XREF: RAM:B1FDj	RAM:B200j
		ld	a, b
		ld	b, d

loc_B207:				; CODE XREF: RAM:B24Bj
		ld	(loc_B210+1), a
		ex	af, af'
		jr	nc, loc_B215
		add	a, b

loc_B20E:				; CODE XREF: RAM:B216j
		ex	af, af'
		ld	a, c

loc_B210:				; DATA XREF: RAM:loc_B207w
		xor	28h ; '('
		ld	c, a
		ex	af, af'
		ret
; ---------------------------------------------------------------------------

loc_B215:				; CODE XREF: RAM:B20Bj
		sub	b
		jr	nc, loc_B20E
		neg
		ret
; ---------------------------------------------------------------------------

loc_B21B:				; CODE XREF: PrnLineInBuf_clip+18p
					; PrnLineInBuf_clip+37p
		inc	a
		ex	af, af'
		ld	a, d
		ld	d, 77h ; 'w'
		bit	3, c
		jr	z, loc_B226
		sub	h
; ---------------------------------------------------------------------------
		db 38h
; ---------------------------------------------------------------------------

loc_B226:				; CODE XREF: RAM:B222j
		add	a, h
		exx
		rra
		ld	b, a
		exx
		ld	a, e
		bit	2, c
		jr	z, loc_B237
		sub	l
		push	af
		jr	nc, loc_B238
		ld	a, l
		sub	e
; ---------------------------------------------------------------------------
		db 28h
; ---------------------------------------------------------------------------

loc_B237:				; CODE XREF: RAM:B22Ej
		add	a, l

loc_B238:				; CODE XREF: RAM:B232j
		call	sub_B1BC
		add	hl, hl
		ld	a, h
		ex	af, af'
		exx
		bit	2, c
		jr	z, loc_B249
		pop	af
		jr	nc, loc_B249
		ex	af, af'
		add	a, e
		ret
; ---------------------------------------------------------------------------

loc_B249:				; CODE XREF: RAM:B241j	RAM:B244j
		ld	a, b
		ld	b, e
		jp	loc_B207

; =============== S U B	R O U T	I N E =======================================

; вызывается при увеличении 3го	байта рейтинга



; === sub_B24E === CODE XREF: IncBattleRating2+Cp
sub_B24E:				; CODE XREF: IncBattleRating2+Cp
		ld	a, (hl)
		cp	0C0h ; '└'      ; hl,PilotRating2
		jr	nc, loc_B256
		inc	(hl)
		jr	loc_B258
; ---------------------------------------------------------------------------

loc_B256:				; CODE XREF: sub_B24E+3j
		res	0, (hl)

loc_B258:				; CODE XREF: sub_B24E+6j
		call	sub_5E78	; вызывается при увеличении 3го	байта рейтинга
		ret
; End of function sub_B24E


; =============== S U B	R O U T	I N E =======================================




; === sub_B25C === CODE XREF: sub_D14Dp
sub_B25C:				; CODE XREF: sub_D14Dp
		ld	a, (hl)
		cp	0C0h ; '└'
		jr	nc, loc_B263
		inc	(hl)
		ret
; ---------------------------------------------------------------------------

loc_B263:				; CODE XREF: sub_B25C+3j
		res	0, (hl)
		ret
; End of function sub_B25C

; ---------------------------------------------------------------------------
		db 0F3h	; є
		db 0F3h	; є
		db 0F3h	; є
		db 0F3h	; є
		db 0F3h	; є
		db 0F3h	; є
		db 0F3h	; є
		db 0F3h	; є
		db 0F3h	; є
		db 0F3h	; є
		db 0F3h	; є
		db 0F3h	; є
		db 0F3h	; є
		db 0F3h	; є
		db 0F3h	; є
		db 0F3h	; є
		db 0F3h	; є
		db 0F3h	; є
		db 0F3h	; є
		db 0F3h	; є
		db 0F3h	; є
		db 0F3h	; є
		db 0F3h	; є
		db 0F3h	; є
		db 0F3h	; є
		db 0F3h	; є
		db 0F3h	; є
		db 0F3h	; є
		db 0F3h	; є
		db 0F3h	; є
		db 0F3h	; є
		db  76h	; v
; ---------------------------------------------------------------------------
; START	OF FUNCTION CHUNK FOR sub_9924

loc_B286:				; CODE XREF: sub_9924+4j
		push	de
		ld	de, 0
		jr	loc_B28F
; ---------------------------------------------------------------------------

loc_B28C:				; CODE XREF: sub_9924+1974j
		and	a
		jr	z, loc_B29C

loc_B28F:				; CODE XREF: sub_9924+1966j
		rra
		jr	nc, loc_B297
		ex	de, hl
		add	hl, de
		jr	c, loc_B29A
		ex	de, hl

loc_B297:				; CODE XREF: sub_9924+196Cj
		add	hl, hl
		jr	loc_B28C
; ---------------------------------------------------------------------------

loc_B29A:				; CODE XREF: sub_9924+1970j
		sbc	a, a
; ---------------------------------------------------------------------------
		db 0FEh
; ---------------------------------------------------------------------------

loc_B29C:				; CODE XREF: sub_9924+1969j
		ld	a, d
		pop	de
		ret
; END OF FUNCTION CHUNK	FOR sub_9924
; ---------------------------------------------------------------------------
		db    0

; =============== S U B	R O U T	I N E =======================================




; === sub_B2A0 === CODE XREF: sub_7FCD+6p
sub_B2A0:				; CODE XREF: sub_7FCD+6p
		ld	a, 4

loc_B2A2:				; CODE XREF: sub_B2A0+49j
		ldi
		ldi
		ldi
		ldi
		ldi
		ldi
		ldi
		ldi
		ldi
		ldi
		ldi
		ldi
		ldi
		ldi
		ldi
		ldi
		ldi
		ldi
		ldi
		ldi
		ldi
		ldi
		ldi
		ldi
		ldi
		ldi
		ldi
		ldi
		ldi
		ldi
		ldi
		ldi
		ld	c, 0E0h	; 'р'
		ex	de, hl
		add	hl, bc
		ex	de, hl
		add	hl, bc
		dec	a
		jp	nz, loc_B2A2
		ret
; End of function sub_B2A0

; ---------------------------------------------------------------------------
		db 0C1h	; ┴
		db  18h
; ---------------------------------------------------------------------------
; START	OF FUNCTION CHUNK FOR sub_96DC

loc_B2EF:				; CODE XREF: sub_96DC+54j
		push	ix
		call	CalcLenVectorCrd
; END OF FUNCTION CHUNK	FOR sub_96DC
; ---------------------------------------------------------------------------
		ld	a, h
		or	l
		jr	nz, loc_B2F9
		dec	hl

loc_B2F9:				; CODE XREF: RAM:B2F6j
		ex	de, hl
		push	de
		call	CompasCrdNnear
; ---------------------------------------------------------------------------
		pop	de
		push	af
		inc	ix
		inc	ix
		inc	ix
		push	de
		call	CompasCrdNnear
; ---------------------------------------------------------------------------
		pop	de
		add	a, a
		rr	e
		ld	b, a
		srl	b
		add	a, b
		ld	b, a
		rl	e
		ld	a, 20h ; ' '
		jr	c, loc_B31C
		sub	b
		jr	loc_B31D
; ---------------------------------------------------------------------------

loc_B31C:				; CODE XREF: RAM:B317j
		add	a, b

loc_B31D:				; CODE XREF: RAM:B31Aj
		add	a, 3
		ld	b, a
		pop	af
		add	a, a
		rr	e
		ld	c, a
		srl	c
		add	a, c
		ld	c, a
		rl	e
		ld	a, 20h ; ' '
		jr	c, loc_B332
		add	a, c
		jr	loc_B333
; ---------------------------------------------------------------------------

loc_B332:				; CODE XREF: RAM:B32Dj
		sub	c

loc_B333:				; CODE XREF: RAM:B330j
		add	a, 5Ch ; '\'
		ld	c, a
		pop	ix
		ld	a, d
		and	70h ; 'p'
		ld	e, 0
		jr	nz, loc_B342
		dec	c
		ld	e, 4

loc_B342:				; CODE XREF: RAM:B33Dj
		ld	a, d
		and	0Ch
		jr	nz, loc_B349
		set	1, e

loc_B349:				; CODE XREF: RAM:B345j
		ld	a, d
		and	3
		jr	nz, loc_B34F
		inc	e

loc_B34F:				; CODE XREF: RAM:B34Cj
		inc	e
		ld	a, xl
		rrca
		jr	nc, loc_B35A
		dec	b
		rrca
		jr	nc, loc_B35A
		dec	b

loc_B35A:				; CODE XREF: RAM:B353j	RAM:B357j
		ld	hl, 0B6E4h
		ld	(loc_A848+1), hl
		call	objPrnPixel
		ld	a, (ix+8)
		rlca
		ld	hl, 4
		jr	c, loc_B37B
		call	sub_B440
		inc	l
		call	sub_B440
		ld	l, 0Ch
		call	sub_B440
		inc	l
		jr	loc_B389
; ---------------------------------------------------------------------------

loc_B37B:				; CODE XREF: RAM:B36Aj
		ld	h, 0Ch
		call	sub_B440
		inc	h
		call	sub_B440
		inc	l
		call	sub_B440
		dec	h

loc_B389:				; CODE XREF: RAM:B379j
		call	sub_B440

loc_B38C:				; DATA XREF: CopyLocator2Scr+15o
		ld	hl, 0AEFCh
		ld	(loc_A848+1), hl
		jp	loc_982E
; ---------------------------------------------------------------------------
		db    0
		db    0
		db  80h	; А
		db    0
		db    0
		db    0
		db    0
		db    0
		db    2
		db    0
		db    0
		db  80h	; А
		db    0
		db 0FFh
		db 0FFh
		db 0FEh	; ■
		db    0
		db    2
		db    0
		db    0
		db  80h	; А
		db    0
		db  80h	; А
		db    0
		db    2
		db    0
		db    2
		db    0
		db    0

; =============== S U B	R O U T	I N E =======================================

; Attributes: noreturn

CopyLocator2Scr:			; CODE XREF: sub_ED60:PrintObjScanerp
					; DATA XREF: sub_7A84+7o
		ld	a, 7
		and	a
		jr	nz, loc_B3EA
		ld	a, (FlgSaveScaner)
		dec	a
		ret	nz
		ld	de, 502Ah
		ld	hl, byte_992B

loc_B3C2:				; CODE XREF: CopyLocator2Scr+36j
		ld	a, (hl)
		and	a
		jr	z, loc_B3EF
		push	hl
		ld	hl, loc_B38C
		ld	bc, 9

loc_B3CD:				; CODE XREF: CopyLocator2Scr+1Dj
		add	hl, bc
		rra
		jr	nc, loc_B3CD

loc_B3D1:				; CODE XREF: CopyLocator2Scr+32j
		push	hl
		push	de
		ld	c, 0Bh
		ldir
		pop	hl
		inc	h
		ex	af, af'
		ld	a, h
		and	7
		call	z, compDownScrAdr_HL
		ex	de, hl
		pop	hl
		ex	af, af'
		dec	a
		jr	nz, loc_B3D1
		pop	hl
		inc	hl
		jr	loc_B3C2
; ---------------------------------------------------------------------------

loc_B3EA:				; CODE XREF: CopyLocator2Scr+3j
		push	af
		call	CopyScaner2Scr
		pop	af

loc_B3EF:				; CODE XREF: CopyLocator2Scr+12j
		ld	d, 5Ah ; 'Z'
		ld	hl, unk_A89C

loc_B3F4:				; CODE XREF: CopyLocator2Scr+47j
		ld	e, (hl)
		inc	hl
		inc	e
		ret	z
		ld	(de), a
		jr	loc_B3F4
; End of function CopyLocator2Scr

; ---------------------------------------------------------------------------
		db  44h	; D
		db 0CBh	; ╦
		db  50h	; P
		db  20h
		db    2
		db 0FFh
		db  80h	; А
		db 0C0h	; └
		db 0E0h	; р
		db 0F0h	; Ё
		db 0F8h	; °
		db 0FCh	; №
		db 0FEh	; ■
		db  7Fh	; 
		db  40h	; @
		db  60h	; `
		db  70h	; p
		db  78h	; x
		db  7Ch	; |
		db  7Eh	; ~
		db  7Fh	; 
		db  3Fh	; ?
		db  20h
		db  30h	; 0
		db  38h	; 8
		db  3Ch	; <
		db  3Eh	; >
		db  3Fh	; ?
		db  3Fh	; ?
		db  1Fh
		db  10h
		db  18h
		db  1Ch
		db  1Eh
		db  1Fh
		db  1Fh
		db  1Fh
		db  0Fh
		db    8
		db  0Ch
		db  0Eh
		db  0Fh
		db  0Fh
		db  0Fh
		db  0Fh
		db    7
		db    4
		db    6
		db    7
		db    7
		db    7
		db    7
		db    7
		db    3
		db    2
		db    3
		dw 303h
		db    3
		db    3
		db    3
		db    1
		db    1
		db    1
		db    1
		db    1
		db    1
		db    1
		db    1

; =============== S U B	R O U T	I N E =======================================




; === sub_B440 === CODE XREF: RAM:B36Cp	RAM:B370p ...
sub_B440:				; CODE XREF: RAM:B36Cp	RAM:B370p ...
		push	bc
		push	de
		push	hl
		ld	(loc_B446), hl

loc_B446:				; CODE XREF: sub_B440:loc_B44Cj
					; DATA XREF: sub_B440+3w
		dec	b
		inc	c
		call	objPrnPixel
		dec	e

loc_B44C:				; DATA XREF: AttackTargetObj+6o
		jr	nz, loc_B446
		pop	hl
		pop	de
		pop	bc
		ret
; End of function sub_B440

; ---------------------------------------------------------------------------
word_B452:	dw 0B4A7h		; DATA XREF: SetPrnCrdInExtSlot+6r
					; SetBit2ExtSlot_rndAr	...
byte_B454:	db 8Ah			; DATA XREF: sub_7A9E+39r sub_7C60+3o	...
byte_B455:	db 1			; DATA XREF: AttackTargetObj+2Ar
					; AttackTargetObj+69r ...


; === sub_B4B1 === CODE XREF: MoveAllObjects+7Fp
sub_B4B1:				; CODE XREF: MoveAllObjects+7Fp
		ld	(InverseNumSlotIX), a
		ld	hl, (word_B452)
		res	7, (hl)
		ret
; End of function sub_B4B1


; =============== S U B	R O U T	I N E =======================================


bit2A_equ_bit2HL:			; CODE XREF: SetBit2ExtSlot_rndA+8p
					; sub_7C60+88p	...
		bit	2, (hl)
		ret	nz
		set	2, a
		ret
; End of function bit2A_equ_bit2HL

; ---------------------------------------------------------------------------
		ld	de, (CrdXProjection)

; =============== S U B	R O U T	I N E =======================================


GetDirectObj:				; CODE XREF: DirectObjectp
		ld	hl, (word_B452)
		ld	de, 8
		add	hl, de
		ld	a, (hl)
		and	0Fh
		cp	1
		bit	1, (ix+21h)
		ret
; End of function GetDirectObj

; ---------------------------------------------------------------------------
		db    0
byte_B4D6:	db 0			; DATA XREF: FireLaser+114w
					; FireLaser+139r ...
byte_B4D7:	db 0			; DATA XREF: FireLaserr FireLaser+5w ...

; =============== S U B	R O U T	I N E =======================================


CollisionWithObj:			; CODE XREF: sub_9D40:loc_9E41p

; FUNCTION CHUNK AT D0E4 SIZE 00000003 BYTES
; FUNCTION CHUNK AT D183 SIZE 00000032 BYTES

		ld	a, (PlayerGameOver)
		and	a
		ret	nz
		ld	d, 0
		ld	e, (iy+1)
		bit	0, (ix+26h)
		jr	z, loc_B4F0
		ld	a, (ix+25h)
		and	a
		jr	nz, loc_B4F0
		ld	d, 1

loc_B4F0:				; CODE XREF: CollisionWithObj+Ej
					; CollisionWithObj+14j
		bit	1, (ix+26h)
		jr	z, loc_B4F8
		ld	e, 0B4h	; '┤'

loc_B4F8:				; CODE XREF: CollisionWithObj+1Cj
		ld	l, (ix+0)
		ld	h, (ix+1)
		sbc	hl, de
		ret	nc
		and	a
		ld	l, (ix+3)
		ld	h, (ix+4)
		sbc	hl, de
		ret	nc
		and	a
		ld	l, (ix+6)
		ld	h, (ix+7)
		sbc	hl, de
		ret	nc
		ld	a, (ix+8)
		add	a, a
		ret	nz
		bit	1, (ix+26h)
		jr	z, loc_B55D	; столкновение со станцией
		bit	7, (ix+5)
		jp	z, objCollision
		ld	a, (PilotFuelScoops)
		and	a
		jr	z, objCollision
		xor	a
		ld	(ix+22h), a
		call	OffTargetMissiles_2
		ld	a, (PilotFreeCargo)
		and	a
		jr	z, objCollision
		ld	a, (iy+0)
		and	0F0h ; 'Ё'
		jr	z, loc_B548
		rrca
		rrca
		rrca
		rrca
		inc	a
		jr	AddFoundCargo2
; ---------------------------------------------------------------------------

loc_B548:				; CODE XREF: CollisionWithObj+67j
		ld	a, (Mission2)
		sub	3
		jr	nz, loc_B555
		ld	(Mission2), a
		dec	a
		jr	AddFoundCargo2
; ---------------------------------------------------------------------------

loc_B555:				; CODE XREF: CollisionWithObj+75j
		call	IntRnd_A
		and	7

AddFoundCargo2:				; CODE XREF: CollisionWithObj+6Ej
					; CollisionWithObj+7Bj
		jp	AddFoundCargo
; ---------------------------------------------------------------------------

loc_B55D:				; CODE XREF: CollisionWithObj+46j
		bit	2, (ix+26h)	; столкновение со станцией
		jr	z, objCollision
		bit	7, (ix+0Eh)
		jr	z, killPlayer
		call	sub_8429
		jp	c, loc_B595
		bit	2, (ix+21h)
		jr	nz, killPlayer
		ld	a, (ix+0Ah)
		add	a, a
		cp	0DCh ; '▄'
		jr	nc, killPlayer
		ld	b, a
		ld	a, (ix+0Ch)
		add	a, a
		cp	0DCh ; '▄'
		jr	nc, killPlayer
		cp	3Ch ; '<'
		jr	nc, loc_B595
		ld	a, b
		cp	3Ch ; '<'
		jr	nc, loc_B595
		ld	a, 1
		ld	(ManualDocking), a
		ret
; ---------------------------------------------------------------------------

loc_B595:				; CODE XREF: CollisionWithObj+94j
					; CollisionWithObj+B0j	...
		ld	hl, nBeepFX
		set	6, (hl)
		ld	hl, HighDeltaSpeed
		ld	a, (DamageFrontShield)
		add	a, 0Ch
		add	a, (hl)
		add	a, (hl)
		ld	(DamageFrontShield), a
		ret
; ---------------------------------------------------------------------------

objCollision:				; CODE XREF: CollisionWithObj+4Cj
					; CollisionWithObj+53j	...
		xor	a
		call	sub_B743
		ld	b, (iy+1)
		bit	6, (ix+26h)
		jr	z, loc_B5BF
		ld	a, (Mission2)
		and	a
		jr	z, loc_B5BF
		xor	a
		ld	(Mission2), a

loc_B5BF:				; CODE XREF: CollisionWithObj+DBj
					; CollisionWithObj+E1j
		ld	hl, nBeepFX
		set	2, (hl)
		bit	7, (ix+8)
		jr	z, loc_B5D8
		ld	a, (iy+15h)
		or	b
		ld	(DamageRearShield), a
		jr	OffTargetMissiles
; ---------------------------------------------------------------------------

killPlayer:				; CODE XREF: CollisionWithObj+8Fj
					; CollisionWithObj+9Bj	...
		inc	a
		ld	(PlayerGameOver), a
		ret
; ---------------------------------------------------------------------------

loc_B5D8:				; CODE XREF: CollisionWithObj+F0j
		ld	a, (iy+15h)
		or	b
		ld	(DamageFrontShield), a
; End of function CollisionWithObj


; =============== S U B	R O U T	I N E =======================================


OffTargetMissiles:			; CODE XREF: CounterShipHyperJump+18p
					; sub_9D40+A5p	...
		ld	a, (InverseNumSlotIX)
		ld	h, a
		push	iy
		ld	iy, objCoriolis
		ld	de, 27h	; '''
		ld	b, 7

loc_B5EE:				; CODE XREF: OffTargetMissiles:loc_B600j
		add	iy, de
		ld	a, (iy+22h)
		and	a
		jr	z, loc_B600
		ld	a, (iy+25h)
		xor	h
		call	z, sub_8C29
		call	sub_8AA3

loc_B600:				; CODE XREF: OffTargetMissiles+15j
		djnz	loc_B5EE
		pop	iy
		ld	a, (Missile1)
		cp	2
		ret	nz
		ld	a, (nObjMissile)
		sub	h
		ret	nz
		ld	(Missile1), a
		ld	hl, UntargetMissile
		set	7, (hl)
		ret
; End of function OffTargetMissiles


; =============== S U B	R O U T	I N E =======================================

; Attributes: noreturn

SetShipwreck2:				; CODE XREF: SetShipwreckj
		ld	hl, nBeepFX
		set	2, (hl)
		xor	a
		ld	(WhatOnScreen),	a
		ld	ix, loc_6021
		ld	de, 27h	; '''
		ld	b, 8

loc_B62A:				; CODE XREF: SetShipwreck2+17j
		add	ix, de
		ld	(ix+22h), a
		djnz	loc_B62A
		ld	a, 16h
		call	sub_B867
; End of function SetShipwreck2

; ---------------------------------------------------------------------------
		ld	a, 15h
		call	sub_B867
; ---------------------------------------------------------------------------
		ret

; =============== S U B	R O U T	I N E =======================================


FireLaser:				; CODE XREF: RAM:B7F1p

; FUNCTION CHUNK AT B749 SIZE 00000050 BYTES

		ld	a, (byte_B4D7)
		ld	c, a
		xor	a
		ld	(byte_B4D7), a
		ld	a, (TempLaser)
		cp	2
		jr	nc, loc_B64D
		ld	a, 2

loc_B64D:				; CODE XREF: FireLaser+Dj
		sub	2
		ld	(TempLaser), a
		ld	hl, ControlKey3
		bit	0, (hl)
		ret	z
		res	0, (hl)
		cp	90h ; 'Р'
		ret	nc
		ld	b, a
		ld	a, (WhatOnScreen)
		ld	hl, PilotFrontLaser
		ld	d, 0
		ld	e, a
		add	hl, de
		ld	a, (hl)
		and	a
		ret	z
		ld	e, c
		ld	c, a
		dec	a
		jr	nz, loc_B672
		ld	d, 1

loc_B672:				; CODE XREF: FireLaser+32j
		sub	3
		jr	nz, loc_B678
		ld	d, 7

loc_B678:				; CODE XREF: FireLaser+38j
		ld	a, e
		and	e
		ld	a, 1
		ld	(byte_B4D7), a
		jr	z, loc_B686
		ld	a, (FrameCounter)
		and	d
		ret	nz

loc_B686:				; CODE XREF: FireLaser+43j
		ld	hl, DamageEnergyUnit
		inc	(hl)
		call	sub_9389
		ld	hl, nBeepFX
		set	4, (hl)
		ld	a, c
		sub	3
		ld	a, 0Fh
		jr	c, loc_B69F
		jr	z, loc_B69D
		add	a, 1Bh

loc_B69D:				; CODE XREF: FireLaser+5Dj
		add	a, 8

loc_B69F:				; CODE XREF: FireLaser+5Bj
		ld	c, a
		ld	a, d
		dec	a
		and	1
		rla
		add	a, 8
		add	a, b
		cp	0A0h ; 'а'
		jr	c, loc_B6AE
		ld	a, 0A0h	; 'а'

loc_B6AE:				; CODE XREF: FireLaser+6Ej
		ld	(TempLaser), a
		ld	a, (ObjInTarget)
		and	a
		jp	z, PrnLaser
		dec	a
		sub	9
		cpl
		ld	ix, objCoriolis
		ld	de, 27h	; '''

loc_B6C3:				; CODE XREF: FireLaser+8Cj
		dec	a
		jr	z, loc_B6CA
		add	ix, de
		jr	loc_B6C3
; ---------------------------------------------------------------------------

loc_B6CA:				; CODE XREF: FireLaser+88j
		call	sub_F9E1
		nop
		ld	hl, nBeepFX
		set	3, (hl)
		bit	2, (ix+26h)
		jr	nz, PrnLaser
		bit	5, (ix+20h)
		jr	nz, PrnLaser
		ld	a, (ix+25h)
		ld	b, a

loc_B6E3:
		sub	0Eh
		jr	nc, loc_B6EB
		and	1
		add	a, 6

loc_B6EB:				; CODE XREF: FireLaser+A9j
		rla
		sla	b
		rra
		ld	(ix+25h), a
		ld	a, (ix+22h)
		cp	0FFh
		jr	z, PrnLaser
		sub	c
		ld	(ix+22h), a
		jr	z, loc_B701
		jr	nc, PrnLaser

loc_B701:				; CODE XREF: FireLaser+C1j
		xor	a
		dec	a
		set	4, (ix+20h)
		ld	(ix+22h), a
		call	DestroyObj
; ---------------------------------------------------------------------------
		jr	PrnLaser
; End of function FireLaser

; [00000003 BYTES: COLLAPSED FUNCTION j_DestroyObj. PRESS KEYPAD "+" TO EXPAND]

; =============== S U B	R O U T	I N E =======================================


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



; === sub_B71C === CODE XREF: ShipSetObject+Ap
sub_B71C:				; CODE XREF: ShipSetObject+Ap
		push	ix
		call	FindObjInExtSlot ; поиск объекта в дополнительном слоте
					; hl - адрес слота объекта
; ---------------------------------------------------------------------------
		ld	a, (ix+0)
		ld	(byte_B454), a
		ld	a, c
		pop	ix
		push	ix
		call	SetObjInSlot	; a - номер объекта для	установки
		pop	hl
		ret	nz
		push	hl
		call	FindObjInExtSlot ; поиск объекта в дополнительном слоте
					; hl - адрес слота объекта
; ---------------------------------------------------------------------------
		ex	(sp), ix
		pop	hl
		call	sub_7A9E	; hl - адрес допслота
		xor	a
		ret
; End of function sub_B71C


; =============== S U B	R O U T	I N E =======================================


OffTargetMissiles_2:			; CODE XREF: CollisionWithObj+59p
		call	sub_9389
		jp	OffTargetMissiles
; End of function OffTargetMissiles_2


; =============== S U B	R O U T	I N E =======================================




; === sub_B743 === CODE XREF: CollisionWithObj+D1p
sub_B743:				; CODE XREF: CollisionWithObj+D1p
		ld	(ix+22h), a
		jp	sub_9389
; End of function sub_B743

; ---------------------------------------------------------------------------
; START	OF FUNCTION CHUNK FOR FireLaser

PrnLaser:				; CODE XREF: FireLaser+79j
					; FireLaser+9Bj ...
		call	IntRnd_A
		ld	b, a
		and	0Ch
		ld	c, a
		ld	(byte_B4D6), a
		ld	a, b
		and	3
		or	1
		sub	1
		ld	d, a
		rlc	b
		rlc	b
		ld	a, b
		and	3
		or	1
		sub	1
		ld	e, a
		push	de
		ld	a, c
		or	3
		ld	c, a
		ld	hl, 543Fh
		push	hl
		call	PrnLineInBuf_clip
		pop	hl
		pop	de
		ld	a, (byte_B4D6)
		or	1
		ld	c, a
		push	de
		call	PrnLineInBuf_clip
		pop	de
		ld	hl, 403Fh
		ld	a, (byte_B4D6)
		or	3
		ld	c, a
		push	hl
		push	de
		call	PrnLineInBuf_clip
		pop	de
		pop	hl
		ld	a, (byte_B4D6)
		or	1
		ld	c, a
		jp	PrnLineInBuf_clip
; END OF FUNCTION CHUNK	FOR FireLaser

; =============== S U B	R O U T	I N E =======================================


IntRnd_A:				; CODE XREF: sub_78B1+10Ap
					; sub_78B1+10Fp ...
		push	hl
		push	bc
		ld	hl, (word_B7AA)
		ld	a, h
		adc	a, a
		ld	b, a
		adc	a, l
		ld	h, a
		ld	l, b
		ld	(word_B7AA), hl
		pop	bc
		pop	hl
		ret
; End of function IntRnd_A

; ---------------------------------------------------------------------------
word_B7AA:	dw 0A26Ch		; DATA XREF: IntRnd_A+2r IntRnd_A+Bw

; =============== S U B	R O U T	I N E =======================================


Process_LaserECM2:			; CODE XREF: j_MoveAllObjects+1B03j
		xor	a
		ld	(TableBasePrice+28h), a	; +#28 Fireams
		ld	(TableBasePrice+29h), a	; +#28 Fireams
		ret
; End of function Process_LaserECM2

; ---------------------------------------------------------------------------
; START	OF FUNCTION CHUNK FOR j_MoveAllObjects

Processing_LaserECM:			; CODE XREF: RAM:9CB8j
		ld	a, (PlayerGameOver)
		and	a
		jr	nz, Process_LaserECM2
		ld	a, (WhatOnScreen)
		cp	4
		ret	nc
		ld	hl, PilotFrontLaser
		ld	d, 0
		ld	e, a
		add	hl, de
		ld	a, (hl)
		and	a
		jr	z, sub_B7F4	; ;проверка 3й миссии
		ld	a, (unk_B138)	; тип печати OR/XOR
		ld	(loc_B7DB), a
		ld	(loc_B7EA), a
		ld	hl, unk_C7EE
		ld	b, 2

loc_B7D9:				; CODE XREF: RAM:B7E0j
		ld	a, 0FEh	; '■'
; END OF FUNCTION CHUNK	FOR j_MoveAllObjects

loc_B7DB:				; DATA XREF: j_MoveAllObjects+1B19w
		or	(hl)
		ld	(hl), a
		ld	l, 0F1h	; 'ё'
		ld	c, b
		djnz	loc_B7D9
		ld	de, unk_CF2F
		ld	l, d
		ld	b, 0Eh

loc_B7E8:				; CODE XREF: RAM:B7EDj
		ld	a, c
		dec	h

loc_B7EA:				; DATA XREF: j_MoveAllObjects+1B1Cw
		or	(hl)
		ld	(hl), a
		ex	de, hl
		djnz	loc_B7E8
		ld	b, 0
		call	FireLaser

; =============== S U B	R O U T	I N E =======================================

; ;проверка 3й миссии



; === sub_B7F4 === CODE XREF: j_MoveAllObjects+1B14j
sub_B7F4:				; CODE XREF: j_MoveAllObjects+1B14j
		ld	a, (PilotExtWeapon) ; 7,=1 - ECM Jammer
					; 6,=1 - Cloaking device
					; 5,=1 - нельзя	купить товары(в	трюме беженцы или десант)
					; 4
					; 3 =1 3я миссия была выполнена
					; 2 =1 - третья	миссия выполнена
					; 1 =1 - второя	миссия выполнена
					; 0 =1 - первая	миссия выполнена
		bit	7, a
		jr	z, loc_B801	; ;счётчик ECM
		ld	a, (PilotMission3) ; =#06 при стыковке будет захват таргонами (вроде нереальная	ситуация)
					; =#04 в системе со танцией таргонов во	время войны
		cp	4
		ret	z

loc_B801:				; CODE XREF: sub_B7F4+5j
		ld	hl, ControlKey3	; ;счётчик ECM
		ld	b, (hl)
		res	4, (hl)
		ld	a, (PilotECMSystem)
		ld	c, a
		ld	a, (TimeWorkECM)
		and	a
		jr	z, ECM_On
		dec	a
		ld	(TimeWorkECM), a
		jr	nz, loc_B819
		res	7, c

loc_B819:				; CODE XREF: sub_B7F4+21j
		ld	hl, nBeepFX
		set	5, (hl)
		ld	a, c
		and	a
		ret	z
		ld	(PilotECMSystem), a
		ld	a, (DamageEnergyUnit)
		inc	a
		ld	(DamageEnergyUnit), a
		ret
; ---------------------------------------------------------------------------

ECM_On:					; CODE XREF: sub_B7F4+1Bj
		ld	a, c
		and	a
		ret	z
		bit	4, b
		ret	z
		or	80h ; 'А'
		ld	(PilotECMSystem), a
		ld	a, 0Ah
		ld	(TimeWorkECM), a
		ret
; End of function sub_B7F4

; ---------------------------------------------------------------------------
		db    0


; === sub_B867 === CODE XREF: SetObjbyShip+2j
sub_B867:				; CODE XREF: SetObjbyShip+2j
					; SetObjbyShip+5p ...
		jp	sub_DA51
; End of function sub_B867


; =============== S U B	R O U T	I N E =======================================

; Attributes: noreturn

CallFunctionObj_1:			; CODE XREF: sub_DA51+Bp

; FUNCTION CHUNK AT B96B SIZE 000000BC BYTES
; FUNCTION CHUNK AT BA94 SIZE 00000008 BYTES
; FUNCTION CHUNK AT BAE8 SIZE 0000006D BYTES
; FUNCTION CHUNK AT BBBA SIZE 00000038 BYTES
; FUNCTION CHUNK AT BBFD SIZE 000000CB BYTES

		srl	c
		srl	c
		srl	c
		and	0Fh
		jp	z, SetRndShip
		dec	a
		jp	z, Set3_4Targons
		dec	a
		jp	z, SetObjMissile
		dec	a
		ret	z
		dec	a
		jp	z, ShipSetShip
		dec	a
		jp	z, Set2Alloys
		dec	a
		jp	z, SetCargo
		dec	a
		jp	z, SetCobra_EscPod
		dec	a
		jp	z, Set2Minerals
		dec	a
		jp	z, SetShip4Intro
		ret
; End of function CallFunctionObj_1


; =============== S U B	R O U T	I N E =======================================

; a - номер объекта для	установки

SetObjInSlot:				; CODE XREF: sub_5EC5p
					; ReCalcCrdObj_coriolis+86p ...
		call	sub_EF01	; a - номер объекта
		ld	a, b

loc_B89C:				; CODE XREF: SetObjInSlot_Intro+4j
		and	3Fh ; '?'
		add	a, a
		add	a, a
		add	a, l
		ld	l, a
		jr	nc, loc_B8A5
		inc	h

loc_B8A5:				; CODE XREF: SetObjInSlot+Aj
		inc	hl
		ld	e, (hl)
		inc	hl
		ld	d, (hl)
		push	de
		pop	iy
		ld	(ix+24h), d
		ld	(ix+23h), e
		push	ix
		pop	de
		ld	bc, 1Fh
		ld	hl, PatternObject
		ldir
		ld	a, (iy+7)
		ld	(ix+22h), a
		ld	a, (iy+12h)
		ld	c, a
		and	6Fh ; 'o'
		ld	(ix+21h), a
		call	sub_D60B
		and	7
		rla
		rl	c
		rra
		ld	(ix+20h), a
		ld	a, (iy+6)
		srl	a
		ld	(ix+1Bh), a
		ld	a, 7Eh ; '~'
		ld	(ix+25h), a
		xor	a
		ld	(ix+26h), a
		set	6, (ix+20h)
		bit	2, (ix+21h)
		jr	nz, loc_B903
		call	IntRnd_A
		and	7
		ld	(ix+1),	a
		call	IntRnd_A
		and	7
		ld	(ix+4),	a

loc_B903:				; CODE XREF: SetObjInSlot+59j
		call	IntRnd_A
		ld	(ix+0),	a
		call	IntRnd_A
		ld	(ix+3),	a
		call	IntRnd_A
		rra
		rr	(ix+2)
		rra
		rr	(ix+5)
		xor	a
		ret
; End of function SetObjInSlot


; =============== S U B	R O U T	I N E =======================================

; Attributes: noreturn

ShipSetObject:				; CODE XREF: CallFunctionObj_1+167p
					; CallFunctionObj_1+1A5p ...
		ld	c, a
		push	ix
		call	sub_FACA
; ---------------------------------------------------------------------------
		pop	hl
		ret	nz
		push	hl
		ld	a, c
		call	sub_B71C	; hl - адрес слота родиителя
; ---------------------------------------------------------------------------
		pop	hl
		push	ix
		pop	de
		ld	bc, 1Bh
		ldir
		xor	a
		ret
; End of function ShipSetObject


; =============== S U B	R O U T	I N E =======================================


FindSlot4Obj:				; CODE XREF: ReCalcCrdObj_coriolis+81p
					; SetPlayerObj+1p ...
		ld	ix, objCoriolis
		ld	de, 27h	; '''
		ld	b, 6

loc_B93F:				; CODE XREF: FindSlot4Obj+10j
		add	ix, de
		ld	a, (ix+22h)
		and	a
		ret	z
		djnz	loc_B93F
		inc	b
		ret
; End of function FindSlot4Obj


; =============== S U B	R O U T	I N E =======================================


SetPlayerObj:				; CODE XREF: SetPlayerAlloys:SetPlayerCargop
		ld	c, a
		call	FindSlot4Obj
		ret	nz
		ld	a, c
; End of function SetPlayerObj


; =============== S U B	R O U T	I N E =======================================


SetPlayerObjSlot:			; CODE XREF: sub_9391+18j
					; CallFunctionObj_1+29Cp
		call	sub_5EC5	; запуск ракеты	игроком
					; запуск спасательной капсулы (установка кобры)
		res	7, (ix+0Eh)
		res	7, (ix+16h)
		ld	(ix+0),	a
		ld	(ix+1),	a
		ld	(ix+4),	a
		ld	(ix+6),	a
		ld	(ix+7),	a
		ret
; End of function SetPlayerObjSlot

; ---------------------------------------------------------------------------
; START	OF FUNCTION CHUNK FOR CallFunctionObj_1

Set3_4Targons:				; CODE XREF: CallFunctionObj_1+Cj
		call	SetTargonStation ; Установка станции таргонов в	межпланетном пространстве для последней	подмиссии в войне с таргонами
		and	3
		add	a, 3
		ld	c, a

loc_B973:				; CODE XREF: CallFunctionObj_1+132j
		call	FindSlot4Obj
		ret	nz
		ld	a, 0Fh
		push	bc
		call	SetObjInSlot	; a - номер объекта для	установки
		set	0, (ix+25h)
		call	IntRnd_A
		and	0Fh
		or	4
		ld	(ix+7),	a
		rra
		rra
		rra
		and	1
		ld	(ix+1),	a
		xor	a
		ld	(ix+4),	a
		call	RndDirectionObj
		pop	bc
		dec	c
		jr	nz, loc_B973
		ret
; ---------------------------------------------------------------------------

SetObjMissile:				; CODE XREF: CallFunctionObj_1+10j
		ld	a, c		; Запуск ракеты
					; Вх:  с - цель	ракеты
					;	 =0 игрок (запустил противник)
					;	<>0 номер объекта по которому запустил ракету игрок
					; Вых: z=1 успешно
		and	a
		jr	z, loc_B9CF
		ld	ix, objCoriolis
		ld	de, 111h
		add	ix, de
		ld	a, (ix+22h)
		and	a
		jr	z, loc_B9B6
		call	FindSlot4Obj
		ret	nz

loc_B9B6:				; CODE XREF: CallFunctionObj_1+146j
		ld	a, 1
		push	bc
		call	sub_9391	; c = номер объекта по которому	запустил ракету	игрок
		pop	bc
		ld	a, 6Ah ; 'j'
		ld	(ix+6),	a
		ld	a, 29h ; ')'
		ld	(ix+3),	a
		ld	a, 80h ; 'А'
		ld	(ix+5),	a
		ld	a, c
		jr	loc_B9DE
; ---------------------------------------------------------------------------

loc_B9CF:				; CODE XREF: CallFunctionObj_1+137j
		ld	a, 1
		call	ShipSetObject
; ---------------------------------------------------------------------------
		ret	nz
		ld	a, (HighDeltaSpeed)
		add	a, (ix+1Bh)
		ld	(ix+1Bh), a

loc_B9DE:				; CODE XREF: CallFunctionObj_1+163j
		ld	(ix+25h), c
		set	0, (ix+26h)
		xor	a
		ret
; ---------------------------------------------------------------------------

ShipSetShip:				; CODE XREF: CallFunctionObj_1+16j
		ld	a, 8
		dec	c
		jr	z, loc_BA0D
		call	IntRnd_A
		and	1
		add	a, 9
		dec	c
		jr	z, loc_BA0D
		ld	a, 12h
		dec	c
		jr	z, loc_BA0D
		ld	a, 0Fh
		dec	c
		jr	z, loc_BA0D
		dec	c
		ret	nz
		call	IntRnd_A
		bit	0, a
		ld	a, 10h
		jr	z, loc_BA0D
		ld	a, 11h

loc_BA0D:				; CODE XREF: CallFunctionObj_1+180j
					; CallFunctionObj_1+18Aj ...
		push	ix
		call	ShipSetObject
; ---------------------------------------------------------------------------
		pop	iy
		ret	nz
		bit	2, (iy+26h)
		call	z, RotateSKOobj
		bit	2, (iy+21h)
		ret	z
		set	2, (ix+21h)
		xor	a
		ret
; END OF FUNCTION CHUNK	FOR CallFunctionObj_1

; =============== S U B	R O U T	I N E =======================================


RotateSKOobj:				; CODE XREF: CallFunctionObj_1+1AFp
					; SetShipMinAlloys+Cp

; FUNCTION CHUNK AT BA58 SIZE 0000002B BYTES

		ld	a, (byte_B866)
		xor	1
		ld	(byte_B866), a
		jr	z, loc_BA58
		push	ix
		call	sub_BA3F
		call	sub_BA3F
		call	sub_BA3F
		pop	ix
		ret
; End of function RotateSKOobj


; =============== S U B	R O U T	I N E =======================================




; === sub_BA3F === CODE XREF: RotateSKOobj+Cp
sub_BA3F:				; CODE XREF: RotateSKOobj+Cp
					; RotateSKOobj+Fp ...
		ld	a, (ix+0Ah)
		ld	b, (ix+10h)
		ld	(ix+10h), a
		ld	(ix+0Ah), b
		ld	a, (ix+16h)
		xor	80h ; 'А'
		ld	(ix+16h), a
		inc	ix
		inc	ix
		ret
; End of function sub_BA3F

; ---------------------------------------------------------------------------
; START	OF FUNCTION CHUNK FOR RotateSKOobj

loc_BA58:				; CODE XREF: RotateSKOobj+8j
		ld	a, (ix+10h)
		xor	80h ; 'А'
		ld	b, (ix+0Ah)
		ld	(ix+0Ah), a
		ld	(ix+10h), b
		ld	a, (ix+12h)
		xor	80h ; 'А'
		ld	b, (ix+0Ch)
		ld	(ix+0Ch), a
		ld	(ix+12h), b
		ld	a, (ix+14h)
		xor	80h ; 'А'
		ld	b, (ix+0Eh)
		ld	(ix+0Eh), a
		ld	(ix+14h), b
		ret
; END OF FUNCTION CHUNK	FOR RotateSKOobj

; =============== S U B	R O U T	I N E =======================================


RndDirectionObj:			; CODE XREF: CallFunctionObj_1+12Dp
					; SetPlayerAlloys+6p ...
		call	IntRnd_A
		and	87h ; 'З'
		ld	(ix+0Ah), a
		call	IntRnd_A
		and	87h ; 'З'
		ld	(ix+0Ch), a
		ret
; End of function RndDirectionObj

; ---------------------------------------------------------------------------
; START	OF FUNCTION CHUNK FOR CallFunctionObj_1

Set2Alloys:				; CODE XREF: CallFunctionObj_1+1Aj
		bit	0, c
		jr	z, ShipSet2Alloys
		call	SetPlayerAlloys
		ret	nz
; END OF FUNCTION CHUNK	FOR CallFunctionObj_1

; =============== S U B	R O U T	I N E =======================================


SetPlayerAlloys:			; CODE XREF: CallFunctionObj_1+22Ep
		ld	a, 3

SetPlayerCargo:				; CODE XREF: CallFunctionObj_1+28Ej
		call	SetPlayerObj
		ret	nz
		call	RndDirectionObj
		res	6, (ix+20h)
		call	IntRnd_A
		and	3Fh ; '?'
		or	10h
		ld	(ix+0),	a
		ld	a, (ix+0Ah)
		and	80h ; 'А'
		xor	80h ; 'А'
		ld	(ix+2),	a
		call	IntRnd_A
		and	3Fh ; '?'
		or	10h
		ld	(ix+3),	a
		ld	a, (ix+0Ch)
		and	80h ; 'А'
		xor	80h ; 'А'
		ld	(ix+5),	a
		ld	a, 19h
		ld	(ix+6),	a
		call	IntRnd_A
		ld	(ix+1Dh), a
		call	IntRnd_A
		and	1Fh
		or	8
		ld	(ix+1Bh), a
		xor	a
		ret
; End of function SetPlayerAlloys

; ---------------------------------------------------------------------------
; START	OF FUNCTION CHUNK FOR CallFunctionObj_1

ShipSet2Alloys:				; CODE XREF: CallFunctionObj_1+22Cj
		ld	a, 3
		call	SetShipMinAlloys
; ---------------------------------------------------------------------------
		ret	nz
		ld	a, 3
		jr	SetShipMinAlloys
; ---------------------------------------------------------------------------

SetCargo:				; CODE XREF: CallFunctionObj_1+1Ej
		bit	0, c
		ld	a, 4
		jr	z, SetShipEPodCargo
		jr	SetPlayerCargo
; ---------------------------------------------------------------------------

SetCobra_EscPod:			; CODE XREF: CallFunctionObj_1+22j
		bit	0, c
		ld	a, 2
		jr	z, SetShipEPodCargo
		ld	a, 9
		ld	ix, SlotObj_05
		call	SetPlayerObjSlot
		call	RndDirectionObj
		ld	a, 80h ; 'А'
		ld	(ix+5),	a
		ld	a, 1Ch
		ld	(ix+3),	a
		res	6, (ix+20h)
		ld	a, 4Ch ; 'L'
		ld	(ix+6),	a
		ld	a, 0C0h	; '└'
		ld	(ix+1Eh), a
		ld	a, (HighDeltaSpeed)
		add	a, (ix+1Bh)
		ld	(ix+1Bh), a
		xor	a
		ret
; ---------------------------------------------------------------------------

SetShipEPodCargo:			; CODE XREF: CallFunctionObj_1+28Cj
					; CallFunctionObj_1+294j
		push	ix
		call	ShipSetObject
; ---------------------------------------------------------------------------
		ex	(sp), ix
		pop	hl
		ret	nz
		push	hl
		ex	(sp), ix
		res	6, (ix+20h)
		ld	a, 0Ah
		ld	(ix+26h), a
		ld	a, 80h ; 'А'
		ld	(ix+1Dh), a
		xor	a
		pop	ix
		ret
; ---------------------------------------------------------------------------

Set2Minerals:				; CODE XREF: CallFunctionObj_1+26j
		ld	a, 5
		call	SetShipMinAlloys
; ---------------------------------------------------------------------------
		ret	nz
		ld	a, 5
; END OF FUNCTION CHUNK	FOR CallFunctionObj_1

; =============== S U B	R O U T	I N E =======================================

; Attributes: noreturn

SetShipMinAlloys:			; CODE XREF: CallFunctionObj_1+280p
					; CallFunctionObj_1+286j ...
		push	ix
		call	ShipSetObject
; ---------------------------------------------------------------------------
		ex	(sp), ix
		pop	hl
		ret	nz
		push	hl
		ex	(sp), ix
		call	RotateSKOobj
		ld	a, 0Ah
		ld	(ix+26h), a
		res	6, (ix+20h)
		set	7, (ix+1Dh)
		pop	ix
		xor	a
		ret
; End of function SetShipMinAlloys


; =============== S U B	R O U T	I N E =======================================


SetShip4Intro:				; CODE XREF: sub_728D+132p
					; CallFunctionObj_1+2Aj
		ld	ix, objMissile
		ld	a, c
		add	a, 2
		call	SetObjInSlot_Intro
		ld	a, 0C0h	; '└'
		ld	(ix+1Dh), a
		ld	(ix+1Eh), a
		xor	a
		ld	(ix+6),	a
		ld	(ix+0),	a
		ld	(ix+1),	a
		ld	(ix+3),	a
		ld	(ix+4),	a
		ld	(ix+8),	a
		ld	(ix+1Bh), a
		res	6, (ix+20h)
		inc	a
		inc	a
		ld	(ix+7),	a
		push	ix
		pop	hl
		ld	de, 9
		ld	b, d
		ld	c, e
		add	hl, de
		ex	de, hl
		ld	hl, SKO4Intro

loc_BBB3:				; CODE XREF: SetShip4Intro+41j
		inc	de
		ldi
		jp	pe, loc_BBB3
		ret
; End of function SetShip4Intro

; ---------------------------------------------------------------------------
; START	OF FUNCTION CHUNK FOR CallFunctionObj_1

SetRndShip:				; CODE XREF: CallFunctionObj_1+8j
		ld	a, (cargoSlaves)
		ld	c, a
		ld	a, (cargoNarc)
		add	a, c
		add	a, a
		ld	c, a
		ld	a, (cargoFireams)
		add	a, c
		ld	c, a
		ld	a, (Mission2)
		and	a
		jr	nz, loc_BBFD
		call	IntRnd_A
		cp	1Eh
		jr	nc, loc_BBFD
		ld	a, (PilotExtWeapon) ; возможность выдать ASP для 2й миссии
		and	42h ; 'B'
		cp	2
		jr	nz, loc_BBFD
		dec	a		; ставим ASP для 2й миссии
		ld	(Mission2), a
		ld	a, 0Fh
		ld	hl, sub_BBF2
		push	hl
		push	af
		call	FindSlot4Obj
		ret	nz
		ld	a, 0Dh
		jr	loc_BC65
; END OF FUNCTION CHUNK	FOR CallFunctionObj_1

; =============== S U B	R O U T	I N E =======================================

; Attributes: noreturn



; === sub_BBF2 === DATA XREF: CallFunctionObj_1+37Bo
sub_BBF2:				; DATA XREF: CallFunctionObj_1+37Bo
		set	6, (ix+26h)
		set	2, (ix+20h)
		jp	rndSet2_4Krait
; End of function sub_BBF2

; ---------------------------------------------------------------------------
; START	OF FUNCTION CHUNK FOR CallFunctionObj_1

loc_BBFD:				; CODE XREF: CallFunctionObj_1+363j
					; CallFunctionObj_1+36Aj ...
		call	IntRnd_A
		sub	c
		jr	c, loc_BC21
		cp	5Ah ; 'Z'
		ret	nc
		cp	50h ; 'P'
		jr	c, loc_BC21
		call	FindSlot4Obj	; установка простого астероида
		ret	nz
		ld	a, 6
		call	SetObjInSlot	; a - номер объекта для	установки
		set	7, (ix+0Eh)
		res	6, (ix+20h)
		ld	a, 0C0h	; '└'
		ld	(ix+1Dh), a
		ret
; ---------------------------------------------------------------------------

loc_BC21:				; CODE XREF: CallFunctionObj_1+397j
					; CallFunctionObj_1+39Ej
		ld	b, a
		and	7
		ld	e, a
		ld	a, (PilotMission3) ; =#06 при стыковке будет захват таргонами (вроде нереальная	ситуация)
					; =#04 в системе со танцией таргонов во	время войны
		cp	4
		ld	d, a
		ld	c, 0Fh
		jr	z, rndSetShip	; если находимся у станции таргонов, то	запуск таргона
		ld	a, d
		cp	1
		ld	c, 0Ah
		jr	z, rndSetShip	; если пора выдавать 3ю	миссию,	то запуск питона1
		ld	a, (PilotPlanetGov)
		and	7
		jr	z, loc_BC40	; если анархия
		inc	e
		cp	e
		ret	nc

loc_BC40:				; CODE XREF: CallFunctionObj_1+3D1j
		ld	c, a		; анархия на планете
		bit	3, b
		jp	z, loc_BC7F
		call	FindSlot4Obj
		ret	nz
		call	IntRnd_A
		and	3Fh ; '?'
		ld	b, 0Fh		; Targon
		jr	z, loc_BC63
		dec	b		; Fer-De-Lance
		sub	0Ch
		jr	c, loc_BC63
		dec	b		; Asp
		sub	12h
		jr	c, loc_BC63
		dec	b		; Cobra2
		sub	10h
		jr	c, loc_BC63
		dec	b		; Python2

loc_BC63:				; CODE XREF: CallFunctionObj_1+3E7j
					; CallFunctionObj_1+3ECj ...
		ld	a, b

loc_BC64:				; CODE XREF: CallFunctionObj_1+413j
					; CallFunctionObj_1+437j ...
		push	af

loc_BC65:				; CODE XREF: CallFunctionObj_1+386j
		call	SetObjInSlot	; a - номер объекта для	установки
		pop	af
		cp	0Fh
		jr	z, loc_BC73
		call	IntRnd_A
		cp	33h ; '3'
		ret	nc

loc_BC73:				; CODE XREF: CallFunctionObj_1+401j
		set	0, (ix+25h)
		ret
; ---------------------------------------------------------------------------

rndSetShip:				; CODE XREF: CallFunctionObj_1+3C3j
					; CallFunctionObj_1+3CAj
		call	FindSlot4Obj	; если находимся у станции таргонов, то	запуск таргона
		ret	nz
		ld	a, c
		jr	loc_BC64
; ---------------------------------------------------------------------------

loc_BC7F:				; CODE XREF: CallFunctionObj_1+3D9j
		call	IntRnd_A
		ld	b, a
		and	7
		cp	e
		jr	nc, rndSet2_4Krait
		rlc	b
		rlc	b
		rlc	b
		ld	c, b
		call	FindSlot4Obj
		ret	nz
		ld	a, c
		and	7
		cp	2
		jr	c, rndSetAstViper
		dec	a
		srl	a
		srl	a
		add	a, 9		; cobra1/python1
		jr	loc_BC64
; ---------------------------------------------------------------------------

rndSetAstViper:				; CODE XREF: CallFunctionObj_1+42Ej
		ld	a, c
		rlca
		and	1
		add	a, 7
		jr	loc_BC64
; ---------------------------------------------------------------------------

rndSet2_4Krait:				; CODE XREF: sub_BBF2+8j
					; CallFunctionObj_1+41Cj
		call	IntRnd_A
		and	3
		rra
		adc	a, 2
		ld	c, a

loc_BCB4:				; CODE XREF: CallFunctionObj_1+45Bj
		call	FindSlot4Obj
		ret	nz
		push	bc
		call	IntRnd_A
		and	1
		add	a, 10h
		call	SetObjInSlot	; a - номер объекта для	установки
		pop	bc
		dec	c
		jr	nz, loc_BCB4
		ret
; END OF FUNCTION CHUNK	FOR CallFunctionObj_1
; ---------------------------------------------------------------------------


; === sub_D021 === CODE XREF: sub_813E:loc_8153p
sub_D021:				; CODE XREF: sub_813E:loc_8153p
		jp	loc_E4E6
; End of function sub_D021

; ---------------------------------------------------------------------------
		jp	loc_E4D4
; ---------------------------------------------------------------------------
		jp	loc_E51A
; ---------------------------------------------------------------------------
		jp	loc_E45B
; ---------------------------------------------------------------------------
		jp	loc_E398
; ---------------------------------------------------------------------------
		jp	loc_DF03

; =============== S U B	R O U T	I N E =======================================

; Attributes: thunk



; === sub_D0F3 === CODE XREF: sub_9AA7+10p sub_9AA7+30j
sub_D0F3:				; CODE XREF: sub_9AA7+10p sub_9AA7+30j
		jp	sub_D1E9
; End of function sub_D0F3

; ---------------------------------------------------------------------------
; START	OF FUNCTION CHUNK FOR sub_FC33

loc_D0F6:				; CODE XREF: sub_FC33:loc_D0D7j
		push	af
		ld	de, 0B17h
		ld	hl, 703h
		call	sub_D162
		pop	af
		call	PrintMessage2
		ld	a, 5Dh ; ']'
		call	PrintMessage2
		ld	hl, 0B04h
		ld	(CrdXPrnSymbol), hl
		ld	a, 5Ah ; 'Z'
		call	PrintMessage2
; END OF FUNCTION CHUNK	FOR sub_FC33

loc_D114:				; CODE XREF: sub_D148+3j
		call	sub_E1E8
; ---------------------------------------------------------------------------
		call	sub_E1F7
		call	ClsAndPrintRamk
		call	Pause_24Int
		jp	ResetPressedKeys
; ---------------------------------------------------------------------------
; START	OF FUNCTION CHUNK FOR GoMission3

loc_D123:				; CODE XREF: GoMission3:GiveMission3j
		ld	de, 581Fh
		ld	hl, 405h
		call	sub_D162
		call	sub_E1F7
		cp	59h ; 'Y'
		jr	z, loc_D138
		xor	a
		ld	(PilotMission3), a ; =#06 при стыковке будет захват таргонами (вроде нереальная	ситуация)
					; =#04 в системе со танцией таргонов во	время войны
		ret
; ---------------------------------------------------------------------------

loc_D138:				; CODE XREF: GoMission3+5EB9j
		ld	hl, PilotExtWeapon ; 7,=1 - ECM	Jammer
					; 6,=1 - Cloaking device
					; 5,=1 - нельзя	купить товары(в	трюме беженцы или десант)
					; 4
					; 3 =1 3я миссия была выполнена
					; 2 =1 - третья	миссия выполнена
					; 1 =1 - второя	миссия выполнена
					; 0 =1 - первая	миссия выполнена
		set	7, (hl)
		ld	a, 3
		ld	(PilotMission3), a ; =#06 при стыковке будет захват таргонами (вроде нереальная	ситуация)
					; =#04 в системе со танцией таргонов во	время войны
		ld	de, 5917h

loc_D145:				; CODE XREF: GoMissionWar+53j
		ld	hl, 200h
; END OF FUNCTION CHUNK	FOR GoMission3

; =============== S U B	R O U T	I N E =======================================




; === sub_D148 === CODE XREF: sub_D14D+13j sub_E87E+54p
sub_D148:				; CODE XREF: sub_D14D+13j sub_E87E+54p
		call	sub_D162
		jr	loc_D114
; End of function sub_D148


; =============== S U B	R O U T	I N E =======================================




; === sub_D14D === CODE XREF: GoMission3:EndMission3j
sub_D14D:				; CODE XREF: GoMission3:EndMission3j
		call	sub_B25C
		nop
		xor	a
		ld	(PilotMission3), a ; =#06 при стыковке будет захват таргонами (вроде нереальная	ситуация)
					; =#04 в системе со танцией таргонов во	время войны
		ld	hl, PilotExtWeapon ; 7,=1 - ECM	Jammer
					; 6,=1 - Cloaking device
					; 5,=1 - нельзя	купить товары(в	трюме беженцы или десант)
					; 4
					; 3 =1 3я миссия была выполнена
					; 2 =1 - третья	миссия выполнена
					; 1 =1 - второя	миссия выполнена
					; 0 =1 - первая	миссия выполнена
		set	3, (hl)
		ld	de, 0A0Fh
		ld	hl, 300h
		jr	sub_D148
; End of function sub_D14D


; =============== S U B	R O U T	I N E =======================================




; === sub_D162 === CODE XREF: sub_FC33-2B36p
sub_D162:				; CODE XREF: sub_FC33-2B36p
					; GoMission3+5EB1p ...
		push	hl
		push	de
		call	ClsAndPrintRamk
		ld	a, 5Ch ; '\'
		call	PrintMessage2
		pop	de
		pop	hl
		ld	(CrdXPrnSymbol), hl
		ld	hl, 5820h
		ld	bc, 2E0h

loc_D177:				; CODE XREF: sub_D162+18j sub_D162+1Bj
		ld	(hl), e
		inc	hl
		dec	c
		jr	nz, loc_D177
		dec	b
		jr	nz, loc_D177
		ld	a, d
		jp	PrintMessage2
; End of function sub_D162

; ---------------------------------------------------------------------------
; START	OF FUNCTION CHUNK FOR CollisionWithObj

loc_D183:				; CODE XREF: CollisionWithObj+1CBDj
		bit	7, (ix+26h)
		jr	z, loc_D19F	; minerals
		ld	l, 86h ; 'Ж'    ; hl = byte_D086
		inc	(hl)		; найден Alien Computer
		ld	a, 0F3h	; 'є'
		jr	loc_D1AD	; hl = CargoFound
; ---------------------------------------------------------------------------

AddFoundCargo3:				; CODE XREF: CollisionWithObj:AddFoundCargoj
		ld	hl, PilotFreeCargo
		cp	0FFh
		jr	nz, loc_D183
		ld	l, 0B0h	; '░'   ; hl = PilotExtWeapon
		set	6, (hl)
		ld	a, 7Dh ; '}'
		jr	loc_D1AD	; hl = CargoFound
; ---------------------------------------------------------------------------

loc_D19F:				; CODE XREF: CollisionWithObj+1CAFj
		cp	0Ch		; minerals
		push	af
		call	z, sub_7370	; минеральная жизнь???
		pop	af
		dec	(hl)
		add	a, 6Bh ; 'k'
		ld	l, a
		inc	(hl)
		add	a, 65h ; 'e'

loc_D1AD:				; CODE XREF: CollisionWithObj+1CB6j
					; CollisionWithObj+1CC5j
		ld	l, 0C7h	; '╟'   ; hl = CargoFound
		ld	(hl), 8Bh ; 'Л'
		inc	hl
		ld	(hl), a
		sub	0D0h ; '╨'
; END OF FUNCTION CHUNK	FOR CollisionWithObj

; =============== S U B	R O U T	I N E =======================================




; === sub_D1B5 === CODE XREF: RAM:E813p
sub_D1B5:				; CODE XREF: RAM:E813p
		cp	3
		jr	z, loc_D1BC
		cp	6
		ret	nz

loc_D1BC:				; CODE XREF: sub_D1B5+2j
		ld	hl, PilotLegalStatus
		ld	a, (hl)
		add	a, 2
		jr	nc, loc_D1C5
		sbc	a, a

loc_D1C5:				; CODE XREF: sub_D1B5+Dj
		ld	(hl), a
		ret
; End of function sub_D1B5


; =============== S U B	R O U T	I N E =======================================


IncBattleRating2:			; CODE XREF: j_IncBattleRating2j
					; sub_D6AA+49p
		ld	hl, PilotRating0
		ld	a, (hl)
		add	a, e
		ld	(hl), a
		inc	hl
		ld	a, (hl)
		adc	a, d
		ld	(hl), a
		ret	nc
		inc	hl
		call	sub_B24E	; вызывается при увеличении 3го	байта рейтинга
		nop
		jr	loc_D1E6
; ---------------------------------------------------------------------------

AddCashObj2:				; CODE XREF: RAM:AddCashObj3j
		ld	a, d
		or	e
		ret	z
		ld	(LastBonusCash), de
		call	IncCASH
		ld	hl, CashBonus

loc_D1E6:				; CODE XREF: IncBattleRating2+10j
		set	7, (hl)
		ret
; End of function IncBattleRating2


; =============== S U B	R O U T	I N E =======================================




; === sub_D1E9 === CODE XREF: sub_D0F3j
sub_D1E9:				; CODE XREF: sub_D0F3j
		ld	a, (msgNumber)
		cp	0FDh ; '¤'
		ret	z
		ld	a, (RndWord)
		rrca
		rrca
		and	3Fh ; '?'
		jp	z, SetFuelZero
		cp	11h
		ret	nc
		dec	a
		cp	6
		jr	nc, loc_D21F
		and	a
		jr	z, loc_D206
		add	a, 2

loc_D206:				; CODE XREF: sub_D1E9+19j
		add	a, 85h ; 'Е'
		ld	l, a
		ld	h, 0D0h	; '╨'
		inc	(hl)
		dec	(hl)
		ret	z
		set	7, (hl)
		inc	(hl)
		add	a, 0E7h	; 'ч'

loc_D213:				; CODE XREF: sub_D1E9+47j
		ld	(nMsgDamage), a
		call	loc_712D
		ld	hl, DestDeviceCargo
		set	7, (hl)
		ret
; ---------------------------------------------------------------------------

loc_D21F:				; CODE XREF: sub_D1E9+16j
		ld	hl, PilotCash2hi
		call	sub_EB3E
		ld	a, (hl)
		and	a
		ret	z
		dec	(hl)
		ld	hl, PilotFreeCargo
		inc	(hl)
		ld	a, 0CCh	; '╠'
		add	a, e
		jr	loc_D213
; End of function sub_D1E9

; ---------------------------------------------------------------------------
aJameson_1:	.ascii 'JAMESON   ',0   ; DATA XREF: InitGame+3o
		db    0
		db    0
		db    0
		db    0
		db    0
		db    0
		db    0
		db    0
		db    0
		db 0E8h	; ш
		db    3
		db    0
		db    0
		db    0
		db    0
		db    0
		db    0
		db    0
		db    0
		db    0
		db    0
		db    0
		db    0
		db    0
		db    0
		db    0
		db    0
		db    0
		db    0
		db  14h
		db    1
		db    0
		db    0
		db    0
		db    0
		db  46h	; F
		db    3
		db    0
		db    0
		db    0
		db    0
		db    0
		db    0
		db    0
		db    0
		db    0
		db    0
		db    0
		db    0
		db  4Ah	; J
		db  5Ah	; Z
		db  48h	; H
		db    2
		db  53h	; S
		db 0B7h	; ╖
		db  38h	; 8
		db 0ADh	; н
		db  9Ch	; Ь
		db  14h
		db  1Dh
		db  15h
		db    5
		db    3
		db  11h
		db  12h
		db  1Ah
		db  0Eh
		db  27h	; '
		db    8
		db    0
		db    0
		db  0Ah
		db  19h
		db    0
		db    0
		db  3Dh	; =
		db  0Eh
		db  11h
		db  0Ch
		db    0
		db    0
		db    0
unk_D28F:	db 0F0h	; Ё		; DATA XREF: InitGame+34o
		db  52h	; R
		db    5
		db  33h	; 3
		db  98h	; Ш
		db    7
		db 0B6h	; ╢
		db 0FAh	; ·
		db  8Bh	; Л

; =============== S U B	R O U T	I N E =======================================


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




; === sub_D2DE === CODE XREF: sub_D822+21p RAM:EE29p
sub_D2DE:				; CODE XREF: sub_D822+21p RAM:EE29p
		scf
		jr	loc_D2E2
; End of function sub_D2DE


; =============== S U B	R O U T	I N E =======================================


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




; === sub_D32D === CODE XREF: sub_D822+5p
sub_D32D:				; CODE XREF: sub_D822+5p
		ld	hl, (PilotCash2low)
		ld	de, (PilotCash1)
; End of function sub_D32D


; =============== S U B	R O U T	I N E =======================================


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




; === sub_D357 === CODE XREF: RAM:D2F6p	RAM:D2FCp ...
sub_D357:				; CODE XREF: RAM:D2F6p	RAM:D2FCp ...

; FUNCTION CHUNK AT D376 SIZE 00000007 BYTES

		ld	hl, (word_D2DC)
		xor	a
		and	a

loc_D35C:				; CODE XREF: sub_D357+Bj
		sbc	hl, de
		jp	c, loc_D364
		inc	a
		jr	loc_D35C
; ---------------------------------------------------------------------------

loc_D364:				; CODE XREF: sub_D357+7j
		add	hl, de
		ld	(word_D2DC), hl
		and	a
		jr	z, loc_D376
		ld	b, a
		xor	a
		ld	(DigitsInNumber), a
		ld	a, b
; End of function sub_D357


; =============== S U B	R O U T	I N E =======================================




; === sub_D371 === CODE XREF: RAM:D316p	RAM:D321p ...
sub_D371:				; CODE XREF: RAM:D316p	RAM:D321p ...
		add	a, 30h ; '0'
		jp	PrintMessage1_A
; End of function sub_D371

; ---------------------------------------------------------------------------
; START	OF FUNCTION CHUNK FOR sub_D357

loc_D376:				; CODE XREF: sub_D357+12j
		ld	a, (DigitsInNumber)
		and	a
		jr	z, sub_D371
		ret
; END OF FUNCTION CHUNK	FOR sub_D357

; =============== S U B	R O U T	I N E =======================================




; === sub_D37D === CODE XREF: RAM:D2E7p
sub_D37D:				; CODE XREF: RAM:D2E7p
		ld	b, 1
		jr	nc, loc_D38B
		push	hl
		and	a
		ld	de, 0Ah
		sbc	hl, de
		pop	hl
		jr	c, loc_D3AF

loc_D38B:				; CODE XREF: sub_D37D+2j
		and	a
		ld	de, 0Ah
		call	sub_D3CE
		jr	c, loc_D3B0
		ld	de, 64h	; 'd'
		call	sub_D3CE
		jr	c, loc_D3AF
		ld	de, 3E8h
		call	sub_D3CE
		jr	c, loc_D3AE
		ld	de, 2710h
		call	sub_D3CE
		jr	c, loc_D3AD
		inc	b

loc_D3AD:				; CODE XREF: sub_D37D+2Dj
		inc	b

loc_D3AE:				; CODE XREF: sub_D37D+25j
		inc	b

loc_D3AF:				; CODE XREF: sub_D37D+Cj sub_D37D+1Dj
		inc	b

loc_D3B0:				; CODE XREF: sub_D37D+15j
		ld	a, (byte_D0CE)
		cp	b
		ret	z
		ret	c
		sub	b
		ld	b, a

loc_D3B8:				; CODE XREF: sub_D37D+4Ej
		push	bc
		ld	a, (DigitsInNumber)
		and	a
		jp	z, loc_D3C5
		ld	a, 20h ; ' '
		jp	loc_D3C7
; ---------------------------------------------------------------------------

loc_D3C5:				; CODE XREF: sub_D37D+40j
		ld	a, 30h ; '0'

loc_D3C7:				; CODE XREF: sub_D37D+45j
		call	PrintMessage1_A
		pop	bc
		djnz	loc_D3B8
		ret
; End of function sub_D37D


; =============== S U B	R O U T	I N E =======================================




; === sub_D3CE === CODE XREF: sub_D37D+12p sub_D37D+1Ap ...
sub_D3CE:				; CODE XREF: sub_D37D+12p sub_D37D+1Ap ...
		push	hl
		sbc	hl, de
		pop	hl
		ret
; End of function sub_D3CE


; =============== S U B	R O U T	I N E =======================================


IncCASH:				; CODE XREF: IncBattleRating2+19p
					; sub_E935+25p	...
		ld	hl, (PilotCash1)
		add	hl, de
		ld	(PilotCash1), hl
		and	a
		ld	de, 10000
		sbc	hl, de
		ret	c
		ld	(PilotCash1), hl
		ld	de, 1

loc_D3E7:				; CODE XREF: RAM:EB11p
		ld	hl, (PilotCash2low)
		add	hl, de
		ld	(PilotCash2low), hl
		ret
; End of function IncCASH


; =============== S U B	R O U T	I N E =======================================




; === sub_D3EF === CODE XREF: RAM:EA15p
sub_D3EF:				; CODE XREF: RAM:EA15p
		ld	hl, (PilotCash2low)
		sbc	hl, de
		jr	c, loc_D3FB
		ld	(PilotCash2low), hl
		and	a
		ret
; ---------------------------------------------------------------------------

loc_D3FB:				; CODE XREF: sub_D3EF+5j
		scf
		ret
; End of function sub_D3EF


; =============== S U B	R O U T	I N E =======================================




; === sub_D3FD === CODE XREF: RAM:E7F0p	RAM:EA1Fp
sub_D3FD:				; CODE XREF: RAM:E7F0p	RAM:EA1Fp
		ld	hl, (PilotCash1)
		and	a
		sbc	hl, de
		jr	nc, loc_D41B
		ld	hl, (PilotCash2low)
		ld	a, l
		or	h
		scf
		ret	z
		dec	hl
		ld	(PilotCash2low), hl
		ld	hl, 2710h
		and	a
		sbc	hl, de
		ld	de, (PilotCash1)
		add	hl, de

loc_D41B:				; CODE XREF: sub_D3FD+6j
		ld	(PilotCash1), hl
		and	a
		ret
; End of function sub_D3FD

; ---------------------------------------------------------------------------
byte_D420:	db 1			; DATA XREF: sub_D470+1r RAM:loc_D8EBw

; =============== S U B	R O U T	I N E =======================================


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




; === sub_D470 === CODE XREF: PrintMessage2+33p
sub_D470:				; CODE XREF: PrintMessage2+33p

; FUNCTION CHUNK AT D595 SIZE 00000007 BYTES

		ld	b, a
		ld	a, (byte_D420)
		cp	0
		ld	a, b
		jr	z, PrintMessage1_A
		cp	20h ; ' '
		jp	c, loc_D7E2+2
		cp	5Bh ; '['
		jr	c, sub_D499
		nop
		nop
		nop
		nop
		nop
		cp	0D7h ; '╫'
		jr	c, PrintMessage2
		jp	loc_D595
; End of function sub_D470

; ---------------------------------------------------------------------------
; START	OF FUNCTION CHUNK FOR PrintMessage2

loc_D48E:				; CODE XREF: PrintMessage2+5j
					; PrintMessage2+38j
		ld	a, (hl)
		and	a
		ret	z
		push	hl
		call	sub_D470
		pop	hl
		inc	hl
		jr	loc_D48E
; END OF FUNCTION CHUNK	FOR PrintMessage2

; =============== S U B	R O U T	I N E =======================================




; === sub_D499 === CODE XREF: PrintMessage1_A+Cj
sub_D499:				; CODE XREF: PrintMessage1_A+Cj
					; sub_D470+10j	...

; FUNCTION CHUNK AT 5DF0 SIZE 0000000B BYTES

		cp	41h ; 'A'
		jr	c, loc_D4D2
		cp	5Bh ; '['
		jr	nc, loc_D4D2
		push	af
		ld	a, (FlgPrnSym1)
		cp	1
		jr	z, loc_D4D8
		ld	a, (byte_D0D3)
		and	a
		jr	z, loc_D4B8
		xor	a
		ld	(byte_D0D3), a
		ld	(byte_D0D2), a
		jr	loc_D4D8
; ---------------------------------------------------------------------------

loc_D4B8:				; CODE XREF: sub_D499+14j
		ld	a, (byte_D0D2)
		cp	1
		jr	nz, loc_D4CC
		xor	a
		ld	(byte_D0D2), a
		ld	a, (FlgPrnSym1)
		cp	2
		jr	z, loc_D4CC
		jr	loc_D4D8
; ---------------------------------------------------------------------------

loc_D4CC:				; CODE XREF: sub_D499+24j sub_D499+2Fj
		pop	af
		add	a, 20h ; ' '
		push	af
		jr	loc_D4D8
; ---------------------------------------------------------------------------

loc_D4D2:				; CODE XREF: sub_D499+2j sub_D499+6j
		push	af
		ld	a, 1
		ld	(byte_D0D2), a

loc_D4D8:				; CODE XREF: sub_D499+Ej sub_D499+1Dj	...
		ld	a, (byte_D0D4)
		cp	1
		nop
		nop
		nop
		pop	af
		push	af
		ld	hl, CrdXPrnSymbol
		inc	(hl)
		ld	a, (hl)
		cp	20h ; ' '
		jr	c, loc_D4F1
		ld	a, 1
		ld	(hl), a
		call	sub_E2B1

loc_D4F1:				; CODE XREF: sub_D499+50j
		pop	af
		call	sub_5DFB
		sub	20h ; ' '
		ld	c, a
		ld	b, 0
		sla	c
		rl	b
		sla	c
		rl	b
		sla	c
		rl	b
		add	hl, bc
		push	hl

loc_D508:				; DATA XREF: PrnTxtOnViewScr:PrnNameViewScrInBufo
					; PrnTxtOnViewScr+34o
		ld	hl, 4000h
		ld	a, (CrdXPrnSymbol+1)
		cp	10h
		jp	nc, loc_D51C
		cp	8
		jp	nc, loc_D526
		ld	b, a
		jp	loc_D52D
; ---------------------------------------------------------------------------

loc_D51C:				; CODE XREF: sub_D499+77j
		sub	10h
		ld	b, a
		ld	de, 1000h
		add	hl, de
		jp	loc_D52D
; ---------------------------------------------------------------------------

loc_D526:				; CODE XREF: sub_D499+7Cj
		sub	8
		ld	b, a
		ld	de, 800h
		add	hl, de

loc_D52D:				; CODE XREF: sub_D499+80j sub_D499+8Aj
		ld	c, 0
		srl	b
		rr	c
		srl	b
		rr	c
		srl	b
		rr	c
		add	hl, bc
		ld	b, 0
		ld	a, (CrdXPrnSymbol)
		ld	c, a
		add	hl, bc
		jp	loc_5DF0
; End of function sub_D499


; =============== S U B	R O U T	I N E =======================================




; === sub_D546 === CODE XREF: sub_D499-76A4p
sub_D546:				; CODE XREF: sub_D499-76A4p
					; sub_D546+6j
		ld	a, (de)
		ld	c, (hl)

loc_D548:				; DATA XREF: PrnTxtOnViewScr+Bo
					; PrnTxtOnViewScr+13o ...
		xor	c
		ld	(hl), a
		inc	h
		inc	de
		djnz	sub_D546
		ret
; End of function sub_D546


; =============== S U B	R O U T	I N E =======================================


FillAttr_21line:			; CODE XREF: RAM:E71Cp	sub_E87E+64j ...
		ld	hl, 5AA1h
		ld	a, 5
; End of function FillAttr_21line


; =============== S U B	R O U T	I N E =======================================


FillAttrLineFrom_HL:			; CODE XREF: RAM:F0E4p
		ld	b, 1Eh
		call	Fill_HL_A_Len_B
		inc	hl
		inc	hl
		ret
; End of function FillAttrLineFrom_HL

; ---------------------------------------------------------------------------

locret_D55C:				; CODE XREF: RAM:D8C0j
		ret
; ---------------------------------------------------------------------------
		db    0
		db    0

; =============== S U B	R O U T	I N E =======================================


PrintHorLine:				; CODE XREF: ClsAndPrintRamk+2Dp
					; ClsAndPrintRamk+4Bp ...
		ld	a, 0FFh
		ld	b, 20h ; ' '
; End of function PrintHorLine


; =============== S U B	R O U T	I N E =======================================


Fill_HL_A_Len_B:			; CODE XREF: FillAttrLineFrom_HL+2p
					; Fill_HL_A_Len_B+2j ...
		ld	(hl), a
		inc	hl
		djnz	Fill_HL_A_Len_B
		ret
; End of function Fill_HL_A_Len_B


; =============== S U B	R O U T	I N E =======================================


SetAttrFrom_DE:				; CODE XREF: RAM:E719p	sub_E87E+61p ...
		ld	hl, 5841h

loc_D56B:				; CODE XREF: SetAttrFrom_DE+22j
		ld	a, (de)
		inc	de
		push	af
		and	0F8h ; '°'
		rrca
		rrca
		rrca
		ld	b, a
		push	hl
		ld	a, 5
		call	Fill_HL_A_Len_B
		pop	hl
		pop	af
		and	7
		jr	z, loc_D58C
		ld	b, a

loc_D581:				; CODE XREF: SetAttrFrom_DE:loc_D588j
		ld	a, 20h ; ' '
		add	a, l
		ld	l, a
		jr	nc, loc_D588
		inc	h

loc_D588:				; CODE XREF: SetAttrFrom_DE+1Dj
		djnz	loc_D581
		jr	loc_D56B
; ---------------------------------------------------------------------------

loc_D58C:				; CODE XREF: SetAttrFrom_DE+16j
		ld	hl, 5822h
		ld	a, 7
		ld	(hl), a
		inc	hl
		ld	(hl), a
		ret
; End of function SetAttrFrom_DE

; ---------------------------------------------------------------------------
; START	OF FUNCTION CHUNK FOR sub_D470

loc_D595:				; CODE XREF: sub_D470+1Bj
		sub	0D7h ; '╫'
		ld	hl, unk_FD99
		jr	loc_D5A1
; END OF FUNCTION CHUNK	FOR sub_D470
; ---------------------------------------------------------------------------
; START	OF FUNCTION CHUNK FOR PrintMessage1_A

loc_D59C:				; CODE XREF: PrintMessage1_A+15j
		sub	80h ; 'А'
		ld	hl, byte_FDB3

loc_D5A1:				; CODE XREF: sub_D470+12Aj
		add	a, a
		ld	d, 0
		ld	e, a
		add	hl, de
		ld	a, (hl)
		push	hl
		call	sub_D499
		pop	hl
		inc	hl
		ld	a, (hl)
		cp	3Fh ; '?'
		ret	z
		jp	sub_D499
; END OF FUNCTION CHUNK	FOR PrintMessage1_A

; =============== S U B	R O U T	I N E =======================================

; гибель минеральной жизни при недостаточном количестве
; Вых: z=1 есть	минеральная жизнь в трюме
;	=0 минеральная жизнь померла
;	   нет минеральной жизни



; === sub_D5B4 === CODE XREF: sub_5E1A+3p
sub_D5B4:				; CODE XREF: sub_5E1A+3p
					; SetVars4War:loc_EEF9p
		ld	a, (PilotMineralLife) ;	<>0 в трюме есть минеральная жизнь
					; =#FF минеральная жизнь померла
		rlc	a
		ret	c
		jr	z, loc_D5C9
		ld	a, (cargoMinerals)
		cp	3
		jr	c, loc_D5C5
		xor	a
		ret
; ---------------------------------------------------------------------------

loc_D5C5:				; CODE XREF: sub_D5B4+Dj
		sbc	a, a
		ld	(PilotMineralLife), a ;	<>0 в трюме есть минеральная жизнь
					; =#FF минеральная жизнь померла

loc_D5C9:				; CODE XREF: sub_D5B4+6j
		dec	a
		ret
; End of function sub_D5B4


; =============== S U B	R O U T	I N E =======================================

; Установка начальных координат	"солнца" и "планеты" в межпланетном пространстве

SetCrdSunPlInFreeSpace:			; CODE XREF: sub_5FD5+1Bp
		exx
		ld	bc, 600h
		ld	de, unk_738E	; начальные координаты "солнца" и "планеты" в межпланетном пространстве
		ld	hl, fsSunCrdXl	; координаты "солнца" и "планеты" в межпланетном пространстве

loc_D5D5:				; CODE XREF: SetCrdSunPlInFreeSpace+12j
		ld	a, (de)
		inc	de
		ld	(hl), c
		inc	hl
		ld	(hl), c
		inc	hl
		ld	(hl), a
		inc	hl
		djnz	loc_D5D5
		exx
		ret
; End of function SetCrdSunPlInFreeSpace


; =============== S U B	R O U T	I N E =======================================




; === sub_D5E1 === CODE XREF: RAM:D888p	RAM:D88Ep
sub_D5E1:				; CODE XREF: RAM:D888p	RAM:D88Ep
		ld	hl, (DescrPlanetData)
		sla	l
		ex	de, hl
		ld	hl, (DescrPlanetData+2)
		adc	hl, de
		ld	a, h
		ld	(DescrPlanetData), hl
		ex	de, hl
		ld	(DescrPlanetData+2), hl
		ret
; End of function sub_D5E1


; =============== S U B	R O U T	I N E =======================================




; === sub_D5F5 === CODE XREF: sub_E935+12p
sub_D5F5:				; CODE XREF: sub_E935+12p
		push	af
		cp	0Ah
		jr	z, loc_D5FD
		and	a
		jr	nz, loc_D609

loc_D5FD:				; CODE XREF: sub_D5F5+3j
		push	bc
		call	TestPilotInZoneCombat ;	Определяем находится ли	пилот в	зоне #3C (активных боёв)
					; Вх:  ---
					; Вых: cy =1 пилот в зоне #3C
					;	  =0 пилот в другой галактике галактике
					;	     война окончена/не начиналась
					;	     пилот в другой зоне
					;      hl,de,b,a - не определены
		pop	bc
		jr	nc, loc_D609
		ld	hl, PilotExtWeapon ; 7,=1 - ECM	Jammer
					; 6,=1 - Cloaking device
					; 5,=1 - нельзя	купить товары(в	трюме беженцы или десант)
					; 4
					; 3 =1 3я миссия была выполнена
					; 2 =1 - третья	миссия выполнена
					; 1 =1 - второя	миссия выполнена
					; 0 =1 - первая	миссия выполнена
		set	1, (hl)

loc_D609:				; CODE XREF: sub_D5F5+6j sub_D5F5+Dj
		pop	af
		ret
; End of function sub_D5F5


; =============== S U B	R O U T	I N E =======================================




; === sub_D60B === CODE XREF: SetObjInSlot+34p
sub_D60B:				; CODE XREF: SetObjInSlot+34p
		call	IntRnd_A
		and	3
		ret	z
		ld	a, (PilotRating1)
		or	(ix+27h)
		and	(iy+0Ah)
		ret
; End of function sub_D60B

; ---------------------------------------------------------------------------
		db 0F8h	; °
		db 0CFh	; ╧

; =============== S U B	R O U T	I N E =======================================


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




; === sub_D6AA === CODE XREF: MoveMissile+5Ep
sub_D6AA:				; CODE XREF: MoveMissile+5Ep
		ld	a, (ix+23h)
		cp	3Ch ; '<'       ; dSidewinderH
		jr	z, loc_D6C5	; запуск sidewinder во время войны
		xor	a
		set	3, (hl)
		bit	2, (hl)
		jr	nz, loc_D6BD
		ld	(hl), a
		dec	a
		ld	(PilotLegalStatus), a

loc_D6BD:				; CODE XREF: sub_D6AA+Cj
		inc	hl
		bit	7, (hl)
		jr	nz, loc_D6C3
		scf

loc_D6C3:				; CODE XREF: sub_D6AA+16j
		sbc	a, a
		ret
; ---------------------------------------------------------------------------

loc_D6C5:				; CODE XREF: sub_D6AA+5j
		xor	a		; запуск sidewinder во время войны
		ld	(ix+22h), a
		call	TestPilotInZoneWar ; Определяем	находится ли пилот/планета в заданной зоне войны
					; Вх:  (sp) =#00 зона оккупированная таргонами
					;	    =#3C зона линии фронта (активные боевые действия)
					;	    =#78 прифронтовая зона
					; Вых: cy =1 координаты	в пределах проверяемой зоны
					;	  =0 пилот в другой галактике галактике
					;	     война окончена/не начиналась
					;	     координаты	слишком	далеко от левого края
					;      hl,de,b,a - не определены
; ---------------------------------------------------------------------------
		db 0
; ---------------------------------------------------------------------------
		ld	hl, PilotMission3 ; =#06 при стыковке будет захват таргонами (вроде нереальная ситуация)
					; =#04 в системе со танцией таргонов во	время войны
		inc	l		; PilotExtWeapon
		ret	nc
		set	2, (hl)

loc_D6D4:				; CODE XREF: sub_D6AA+34j
		ld	hl, PilotTargonWar ; уменьшение	территории захвата таргогнами
		res	7, (hl)
		dec	(hl)
		call	TestPilotInZoneWar ; Определяем	находится ли пилот/планета в заданной зоне войны
					; Вх:  (sp) =#00 зона оккупированная таргонами
					;	    =#3C зона линии фронта (активные боевые действия)
					;	    =#78 прифронтовая зона
					; Вых: cy =1 координаты	в пределах проверяемой зоны
					;	  =0 пилот в другой галактике галактике
					;	     война окончена/не начиналась
					;	     координаты	слишком	далеко от левого края
					;      hl,de,b,a - не определены
; ---------------------------------------------------------------------------
		db 0
; ---------------------------------------------------------------------------
		jr	c, loc_D6D4	; уменьшение территории	захвата	таргогнами
		ld	hl, byte_6069	; станция +#22
		xor	a
		ld	(hl), a
		ld	(PlayerAggressor), a
		ld	hl, PilotMission3 ; =#06 при стыковке будет захват таргонами (вроде нереальная ситуация)
					; =#04 в системе со танцией таргонов во	время войны
		ld	(hl), a
		ld	l, 8Fh ; 'П'    ; hl = byte_D08F
		res	7, (hl)
		ld	de, 0FFFFh
		call	IncBattleRating2
		xor	a
		inc	a
		ret
; End of function sub_D6AA


; =============== S U B	R O U T	I N E =======================================

; Выдача миссии/подмиссии: Война с таргонами, если доступно
; Вх:  ---
; Вых: hl,de,bc,a - не определены

GoMissionWar:				; CODE XREF: sub_5FA1p
		call	TestPilotInZoneWar ; Определяем	находится ли пилот/планета в заданной зоне войны
					; Вх:  (sp) =#00 зона оккупированная таргонами
					;	    =#3C зона линии фронта (активные боевые действия)
					;	    =#78 прифронтовая зона
					; Вых: cy =1 координаты	в пределах проверяемой зоны
					;	  =0 пилот в другой галактике галактике
					;	     война окончена/не начиналась
					;	     координаты	слишком	далеко от левого края
					;      hl,de,b,a - не определены
; ---------------------------------------------------------------------------
		db 0
; ---------------------------------------------------------------------------
		jr	c, loc_D74F
		ld	a, (PilotTargonWar) ; =#81 при сбросе миссий
					; 7,=1 начало войны, предолжение участвовать
					; 6-0, чем больше число	тем дальше от левого края карты	захват планет таргонами
					;
					; =#01 война закончена
		dec	a
		jr	nz, loc_D715
		ld	l, 62h ; 'b'    ; PilotRating2
		bit	6, (hl)
		ret	nz
		ld	a, (hl)
		cp	82h ; 'В'
		ret	c
		set	6, (hl)
		inc	(hl)
		ld	d, 0B8h	; '╕'
		jr	loc_D74A
; ---------------------------------------------------------------------------

loc_D715:				; CODE XREF: GoMissionWar+Aj
		call	TestPilotInZoneCombat ;	Определяем находится ли	пилот в	зоне #3C (активных боёв)
					; Вх:  ---
					; Вых: cy =1 пилот в зоне #3C
					;	  =0 пилот в другой галактике галактике
					;	     война окончена/не начиналась
					;	     пилот в другой зоне
					;      hl,de,b,a - не определены
		ret	nc
		ld	hl, PilotRating2 ; >=#84 в трюме десант
					; 7,=1 война была или идёт
					; 6,=1 война закончилать
					; 3-1 -	ранг (=текущая стадия войны)
		ld	a, (hl)
		rrca
		ret	nc
		and	7
		jr	z, loc_D730	; предложение участвовать в войне
		ld	b, a		; b=ранг 1..7
		ld	d, 0B4h	; '┤'
		ld	a, (PilotExtWeapon) ; 7,=1 - ECM Jammer
					; 6,=1 - Cloaking device
					; 5,=1 - нельзя	купить товары(в	трюме беженцы или десант)
					; 4
					; 3 =1 3я миссия была выполнена
					; 2 =1 - третья	миссия выполнена
					; 1 =1 - второя	миссия выполнена
					; 0 =1 - первая	миссия выполнена

loc_D729:				; CODE XREF: GoMissionWar+32j
		rrca
		inc	d
		djnz	loc_D729
		ret	nc
		jr	loc_D749
; ---------------------------------------------------------------------------

loc_D730:				; CODE XREF: GoMissionWar+28j
		ld	a, (PilotExtWeapon) ; 7,=1 - ECM Jammer
					; 6,=1 - Cloaking device
					; 5,=1 - нельзя	купить товары(в	трюме беженцы или десант)
					; 4
					; 3 =1 3я миссия была выполнена
					; 2 =1 - третья	миссия выполнена
					; 1 =1 - второя	миссия выполнена
					; 0 =1 - первая	миссия выполнена
		bit	5, a
		ret	nz
		push	hl
		ld	de, 0AC1Fh
		ld	hl, 405h
		call	sub_D162
		call	sub_E1F7
		pop	hl		; hl=PilotRating2
		cp	59h ; 'Y'
		ret	nz
		ld	d, 0ADh	; 'н'

loc_D749:				; CODE XREF: GoMissionWar+35j
		inc	(hl)

loc_D74A:				; CODE XREF: GoMissionWar+1Aj
		ld	e, 17h
		jp	loc_D145
; ---------------------------------------------------------------------------

loc_D74F:				; CODE XREF: GoMissionWar+4j
		ld	a, 4
		ld	(PilotMission3), a ; =#06 при стыковке будет захват таргонами (вроде нереальная	ситуация)
					; =#04 в системе со танцией таргонов во	время войны
		ret
; End of function GoMissionWar


; =============== S U B	R O U T	I N E =======================================

; печать рейтинга и, если нужно	ранга



; === sub_D755 === CODE XREF: RAM:EFFDp
sub_D755:				; CODE XREF: RAM:EFFDp
		call	PrnMsg_A_CRLF
		ld	a, (PilotTargonWar) ; =#81 при сбросе миссий
					; 7,=1 начало войны, предолжение участвовать
					; 6-0, чем больше число	тем дальше от левого края карты	захват планет таргонами
					;
					; =#01 война закончена
		and	a
		jr	z, loc_D785
		ld	a, (PilotRating2) ; >=#84 в трюме десант
					; 7,=1 война была или идёт
					; 6,=1 война закончилать
					; 3-1 -	ранг (=текущая стадия войны)
		bit	7, a
		jr	z, loc_D785
		and	0Eh		; %00001110
		jr	z, loc_D785	; война	только началась, нет еще ранга
		push	af
		call	PrintSymbol_SP
; ---------------------------------------------------------------------------
		db 0F1h
; ---------------------------------------------------------------------------
		pop	af
		rrca
		add	a, 9Fh ; 'Я'
		call	PrintMessage2
		ld	a, (PilotRating2) ; >=#84 в трюме десант
					; 7,=1 война была или идёт
					; 6,=1 война закончилать
					; 3-1 -	ранг (=текущая стадия войны)
		and	0FEh ; '■'
		nop
		nop
		nop
		nop
		bit	6, a
		ld	a, 0A5h	; 'е'
		call	nz, PrintMessage2

loc_D785:				; CODE XREF: sub_D755+7j sub_D755+Ej ...
		jp	PrnCrd_CRLF
; End of function sub_D755

; ---------------------------------------------------------------------------
		jr	z, loc_D78E
		ld	ix,  loc_D632+1

loc_D78E:				; CODE XREF: RAM:D788j
		xor	a
		ld	(CoriolisNear),	a
		ret
; ---------------------------------------------------------------------------
		ret	nz
		ld	c, b
		ret
; ---------------------------------------------------------------------------
		ld	c, b
		ret
; ---------------------------------------------------------------------------
		nop
		ret
; ---------------------------------------------------------------------------
		ld	hl, (loc_AA27+2)
		ld	a, (hl)
		ld	(loc_AA27+1), a
		or	1
		ret
; ---------------------------------------------------------------------------
		db 2Ah
		db 29h
		db 0AAh
		db 23h
		db 22h
		db 29h

; =============== S U B	R O U T	I N E =======================================


ResetSignInExtSlot:			; CODE XREF: PrnObjIfVisiblep
		ld	hl, (word_B452)
		inc	hl
		ld	(hl), 0FFh
		inc	hl
		inc	hl
		inc	hl
		ld	(hl), 0FFh
		ld	a, (WhatOnScreen)
		ret
; End of function ResetSignInExtSlot


; =============== S U B	R O U T	I N E =======================================


CalcPrnCrdSun:				; CODE XREF: PrnSunPlanet+Fp
					; PrnSunPlanet+34p
		call	loc_75E1
		and	a
		ret	z
		push	ix
		ld	ix, CircleCrdZlow
		ld	a, (WhatOnScreen)
		cp	2
		jr	c, loc_D7CE
		ld	xl, 7Ah	; 'z'

loc_D7CE:				; CODE XREF: CalcPrnCrdSun+10j
		push	iy
		ld	iy,  loc_7575+2
		call	sub_FBE0
		call	nz, sub_FBE0
		call	nz, sub_FBE0
		pop	iy
		pop	ix
		ret
; End of function CalcPrnCrdSun

; ---------------------------------------------------------------------------
; START	OF FUNCTION CHUNK FOR PrintMessage1_A

loc_D7E2:				; CODE XREF: PrintMessage1_A+2j
					; sub_D470+Bj
		ld	hl, 2EF2h
		nop
		dec	l
		ld	h, 0D7h	; '╫'
		ld	d, 0
		ld	e, a
		add	hl, de
		ld	l, (hl)
		ld	h, 0D8h	; '╪'
		jp	(hl)
; END OF FUNCTION CHUNK	FOR PrintMessage1_A
; ---------------------------------------------------------------------------
		db 84h
		db 4Bh
		db  6Ah	; j
		db  72h	; r
		db 2Eh
		db  37h	; 7
		db 0AEh	; о
		db  22h	; "
		db  33h	; 3
		db 0B4h	; ┤
		db 0E5h	; х
		db  57h	; W
		db  84h	; Д
		db  84h	; Д
		db  84h	; Д
		db 0AEh	; о
		db  33h	; 3
		db  72h	; r
		db  2Eh	; .
		db 0E5h	; х
		db 0E9h	; щ
		db 0B7h	; ╖
		db  7Bh	; {
		db 0BAh	; ║
		db 0BDh	; ╜
		db 0C0h	; └
		db 0B4h	; ┤
		db 0A9h	; й
		db  84h	; Д
		db  84h	; Д
		db  57h	; W
		db  84h	; Д
		db  85h	; Е
		db  7Fh	; 
		db  84h	; Д
		db 0C3h	; ├
		db 0C6h	; ╞
		db 0F3h	; є
		db  1Fh
		db  1Fh
		db 0C9h	; ╔
		db  84h	; Д
		db  84h	; Д
		db 0EFh	; я
		db  84h	; Д
		db  84h	; Д
		db 0C3h	; ├
		db  5Eh	; ^
		db 0E3h	; у

; =============== S U B	R O U T	I N E =======================================




; === sub_D8CE === CODE XREF: RAM:EA01p
sub_D8CE:				; CODE XREF: RAM:EA01p
		cp	0Fh

loc_D8D0:				; CODE XREF: sub_D8CE+Cj
		ld	a, (byte_EB5E)
		ret	nz
		ld	a, (PilotPlanetTechLvl)
		inc	a
		cp	0Fh
		jr	c, loc_D8D0
		ret
; End of function sub_D8CE


; =============== S U B	R O U T	I N E =======================================




; === sub_D8DD === CODE XREF: sub_DCE9+4p
sub_D8DD:				; CODE XREF: sub_DCE9+4p
		ld	hl, DemoMode	; =#00 при стыковке со станцией	таргонов
					;      после использования escape pod
					; =#01 включен демо режим
					; =#E0 после прыжка
		ld	(hl), h
		ld	hl, PlanetData
		ret
; End of function sub_D8DD

; ---------------------------------------------------------------------------
		ld	a, 1
		jr	loc_D8EB
; ---------------------------------------------------------------------------
		ld	a, 0

loc_D8EB:				; CODE XREF: RAM:D8E7j
		ld	(byte_D420), a
		ret
; ---------------------------------------------------------------------------
		ld	a, 6
		jr	loc_D8F5
; ---------------------------------------------------------------------------
		ld	a, 0Ah

loc_D8F5:				; CODE XREF: RAM:D8F1j
		jp	loc_E2C8

; =============== S U B	R O U T	I N E =======================================




; === sub_D8F8 === CODE XREF: RAM:D89Bp	RAM:D8A0p
sub_D8F8:				; CODE XREF: RAM:D89Bp	RAM:D8A0p
		ld	a, (hl)
		cp	3Fh ; '?'
		ret	z
		jp	PrintMessage1_A
; End of function sub_D8F8

; ---------------------------------------------------------------------------
		db    0
		db    0
		db    0
		db  0Ch
		db  12h
		db  12h
		db  13h
		db  80h	; А

; =============== S U B	R O U T	I N E =======================================

; Attributes: noreturn

MenuSaveLoad2:				; CODE XREF: MenuSaveLoadj
		di
		ld	hl, DataCocpit
		call	sub_D91A
		call	byte_C800
		ei
		push	af
		call	j_FX_Beep
; End of function MenuSaveLoad2

; ---------------------------------------------------------------------------
		db 0F1h	; ё
		db 0C3h	; ├
		db    5
		db 0E2h	; т

; =============== S U B	R O U T	I N E =======================================




; === sub_D91A === CODE XREF: MenuSaveLoad2+4p
sub_D91A:				; CODE XREF: MenuSaveLoad2+4p
					; DemoModeOn+18p
		ld	a, 0D0h	; '╨'

loc_D91C:				; CODE XREF: DemoModeOn+6p
		ld	de, 1110h
		ld	bc, 7FFDh
		exx
		ld	hl, sub_D934
		ld	de, 4700h
		ld	bc, 10h
		push	de
		ldir
		ld	c, a
		ld	de, byte_C800
		ret
; End of function sub_D91A


; =============== S U B	R O U T	I N E =======================================




; === sub_D934 === DATA XREF: sub_D91A+9o
sub_D934:				; DATA XREF: sub_D91A+9o
		exx
		out	(c), d
		ld	a, (hl)
		inc	hl
		out	(c), e
		exx
		ld	(de), a
		inc	de
		ld	a, d
		cp	c
		jp	c, 4700h
		ret
; End of function sub_D934


; =============== S U B	R O U T	I N E =======================================




; === sub_DA2F === CODE XREF: sub_E306+3p
sub_DA2F:				; CODE XREF: sub_E306+3p
		jr	loc_DA31

loc_DA31:
		ld	c, 1Eh
		ld	hl, 4521h
		ld	de, 4141h

loc_DA39:				; CODE XREF: sub_DA2F+1Dj
		push	de
		push	hl
		ld	b, 6

loc_DA3D:				; CODE XREF: sub_DA2F+16j
		ld	a, (de)
		ld	(hl), a
		xor	a
		ld	(de), a
		inc	d
		call	sub_7FE7
		djnz	loc_DA3D
		pop	hl
		pop	de
		inc	e
		inc	l
		dec	c
		jr	nz, loc_DA39
		jp	_PrintIndDanger
; End of function sub_DA2F


; =============== S U B	R O U T	I N E =======================================

; Attributes: noreturn



; === sub_DA51 === CODE XREF: sub_B867j
sub_DA51:				; CODE XREF: sub_B867j
		ld	c, a
		srl	c
		ld	(loc_DA5F+1), sp
		di
		ld	sp, 4020h
		call	CallFunctionObj_1
; End of function sub_DA51

; ---------------------------------------------------------------------------
; START	OF FUNCTION CHUNK FOR ClearScrBuffer

loc_DA5F:				; CODE XREF: ClearScrBuffer+31j
					; DATA XREF: ClearScrBuffer+12w ...
		ld	sp, 411Eh
		ei
		ret
; END OF FUNCTION CHUNK	FOR ClearScrBuffer
; ---------------------------------------------------------------------------
		db 0
		db 0
		db 0
		db 0
		db 0
		db 0
		db 0
		db 0
		db 0
		db 0
		db 0FFh
		db 0FFh
		db 0FFh
		db 0FFh
byte_DA72:	db 1Dh			; DATA XREF: sub_DAB4+8Do


; === sub_DAB4 === CODE XREF: PrnTxtOnViewScr+10p
sub_DAB4:				; CODE XREF: PrnTxtOnViewScr+10p
					; PrnTxtOnViewScr+31p

; FUNCTION CHUNK AT DAEF SIZE 000000A2 BYTES

		call	PrnCountdownJump
		call	_PrintIndDanger
		ld	hl, msgTimeLife
		ld	a, (hl)
		and	a
		jr	nz, loc_DAEF
		ld	hl, byte_DA74	; цикл мигания сообщения в бою
		inc	(hl)
		bit	0, (hl)
		jr	nz, loc_DB2E
		ld	b, 0Ah
		ld	hl, MissileJammed

loc_DACE:				; CODE XREF: sub_DAB4+20j
		bit	7, (hl)
		jr	nz, loc_DAD7
		inc	hl
		inc	hl
		djnz	loc_DACE
		ret
; ---------------------------------------------------------------------------

loc_DAD7:				; CODE XREF: sub_DAB4+1Cj
		res	7, (hl)
		ld	c, (hl)
		inc	hl
		ld	b, (hl)

loc_DADC:				; CODE XREF: GoJump+65j
		push	bc
		call	sub_E1E8
; ---------------------------------------------------------------------------
		db 0C1h	; ┴
; ---------------------------------------------------------------------------

loc_DAE1:				; CODE XREF: sub_DAB4:loc_DB8Ej
		ld	a, 0Eh
		ld	(msgTimeLife), a
; End of function sub_DAB4


; =============== S U B	R O U T	I N E =======================================




; === sub_DAE6 === CODE XREF: GoJump+51p
sub_DAE6:				; CODE XREF: GoJump+51p
		ld	a, c
		ld	(msgCrdY), a
		ld	a, b
		ld	(msgNumber), a
		ret
; End of function sub_DAE6

; ---------------------------------------------------------------------------
; START	OF FUNCTION CHUNK FOR sub_DAB4

loc_DAEF:				; CODE XREF: sub_DAB4+Bj
		dec	(hl)
		cp	4
		ret	c
		ld	a, (WhatOnScreen)
		cp	4
		ret	nc
		ld	a, (msgCrdY)
		ld	l, a
		ld	h, 0Dh
		ld	(CrdXPrnSymbol), hl
		ld	a, (msgNumber)
		cp	0FDh ; '¤'
		jr	z, loc_DB13
		cp	0E2h ; 'т'
		jr	z, loc_DB25

loc_DB0D:				; CODE XREF: sub_DAB4+6Fj sub_DAB4+78j
		ld	a, (msgNumber)
		jp	PrnMsg_A_CRLF
; ---------------------------------------------------------------------------

loc_DB13:				; CODE XREF: sub_DAB4+53j
		ld	a, (nMsgDamage)
		cp	73h ; 's'
		jr	z, loc_DB20
		ld	hl, CrdXPrnSymbol
		inc	(hl)
		inc	(hl)
		inc	(hl)

loc_DB20:				; CODE XREF: sub_DAB4+64j
		call	PrintMessage1_A
		jr	loc_DB0D
; ---------------------------------------------------------------------------

loc_DB25:				; CODE XREF: sub_DAB4+57j
		ld	hl, (LastBonusCash)
		scf
		call	PrintNumber_HL
		jr	loc_DB0D
; ---------------------------------------------------------------------------

loc_DB2E:				; CODE XREF: sub_DAB4+13j
		ld	a, (ShieldUnit4)
		cp	9Fh ; 'Я'
		jr	c, loc_DB7D
		ld	a, (FuelScoopsOn)
		and	a
		jr	nz, loc_DB6C
		ld	a, (AutoDocking)
		and	a
		jr	nz, loc_DB67
		ld	hl, byte_DA72
		ld	a, (hl)
		and	a
		jr	nz, loc_DB60
		ld	a, 46h ; 'F'
		ld	(hl), a
		ld	a, (PilotMission3) ; =#06 при стыковке будет захват таргонами (вроде нереальная	ситуация)
					; =#04 в системе со танцией таргонов во	время войны
		dec	a
		jr	z, loc_DB62
		ld	a, (PilotMission1)
		and	a
		ret	z
		ld	a, (PilotMission1_1)
		and	a
		ret	nz
		ld	bc, 0C202h
		jr	loc_DB8E
; ---------------------------------------------------------------------------

loc_DB60:				; CODE XREF: sub_DAB4+92j
		dec	(hl)
		ret
; ---------------------------------------------------------------------------

loc_DB62:				; CODE XREF: sub_DAB4+9Bj
		ld	bc, 0F03h
		jr	loc_DB8E
; ---------------------------------------------------------------------------

loc_DB67:				; CODE XREF: sub_DAB4+8Bj
		ld	bc, 7B05h
		jr	loc_DB8E
; ---------------------------------------------------------------------------

loc_DB6C:				; CODE XREF: sub_DAB4+85j
		ld	a, (PilotFuelScoops)
		and	a
		ret	z
		ld	a, (Pilot_Fuel)
		inc	a
		call	SetPilotFuel
		ld	bc, 0A008h
		jr	loc_DB8E
; ---------------------------------------------------------------------------

loc_DB7D:				; CODE XREF: sub_DAB4+7Fj
		xor	a
		ld	(CloakingDeviceOn), a
		ld	l, 7
		ld	a, (WhatOnScreen)
		cp	4
		call	c, j_SetAtrViewScr
		ld	bc, 640Ah

loc_DB8E:				; CODE XREF: sub_DAB4+AAj sub_DAB4+B1j ...
		jp	loc_DAE1
; END OF FUNCTION CHUNK	FOR sub_DAB4

; =============== S U B	R O U T	I N E =======================================


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


; === sub_DCE9 === CODE XREF: sub_5FD5+4Dp
sub_DCE9:				; CODE XREF: sub_5FD5+4Dp
					; PrnCountdownJump+58j	...

; FUNCTION CHUNK AT DDF7 SIZE 00000055 BYTES
; FUNCTION CHUNK AT DE6F SIZE 00000028 BYTES

		xor	a
		ld	(InFreeSpace), a
		call	sub_D8DD
		ld	de, PilotPlanetData
		ld	bc, 6
		ldir
		ld	hl, PlanetName
		ld	de, PlanetPresent
		ld	bc, 8
		ldir
		ld	a, (PlanetTechLevel)
		ld	(PilotPlanetTechLvl), a
		ld	a, (PlanetGovernment)
		ld	(PilotPlanetGov), a
		call	GenerateProductPrice
		ld	b, 12h
		xor	a
		ld	hl, SunCrd

loc_DD18:				; CODE XREF: sub_DCE9+31j
		ld	(hl), a
		inc	hl
		djnz	loc_DD18
		ld	a, 0FFh
		ld	(J_Drive_Coriolis), a
		ld	hl, byte_DEB9	; PlanetData-1
		ld	de, byte_7106	; SunCrd-1
		ld	bc, 3
		call	sub_DD48
		call	sub_DD48
		call	sub_DD53
		inc	a
		or	81h ; 'Б'
		ld	(de), a
		call	sub_DD48
		call	sub_DD48
		call	sub_DD53
		ld	(de), a
		ld	c, (hl)
		call	sub_DD60
		jp	loc_DDF7
; End of function sub_DCE9


; =============== S U B	R O U T	I N E =======================================




; === sub_DD48 === CODE XREF: sub_DCE9+41p sub_DCE9+44p ...
sub_DD48:				; CODE XREF: sub_DCE9+41p sub_DCE9+44p ...
		inc	hl
		ex	de, hl
		add	hl, bc
		ex	de, hl
		ld	a, (hl)
		and	83h ; 'Г'
		or	1
		ld	(de), a
		ret
; End of function sub_DD48


; =============== S U B	R O U T	I N E =======================================




; === sub_DD53 === CODE XREF: sub_DCE9+47p sub_DCE9+54p
sub_DD53:				; CODE XREF: sub_DCE9+47p sub_DCE9+54p
		ex	de, hl
		add	hl, bc
		ex	de, hl
		ld	a, (hl)
		inc	hl
		or	(hl)
		and	7
		add	a, c
		ret
; End of function sub_DD53

; ---------------------------------------------------------------------------
		ld	(PlanetCrdZ_hi), a

; =============== S U B	R O U T	I N E =======================================




; === sub_DD60 === CODE XREF: sub_DCE9+59p
sub_DD60:				; CODE XREF: sub_DCE9+59p
		push	bc
		ld	bc, 9
		ld	hl, PlanetCrd
		ld	de, objCoriolis
		ldir
		pop	bc
		ld	a, (PlanetCrdZ_hi)
		ld	hl, byte_604F
		srl	c
		jr	c, loc_DD7D
		ld	a, (PlanetCrdX_hi)
		ld	hl, byte_6049

loc_DD7D:				; CODE XREF: sub_DD60+15j
		set	7, (hl)
		ld	b, a
		and	7Fh ; ''
		srl	c
		jr	c, loc_DD96
		cp	1
		jr	nz, loc_DD92
		ld	d, a
		ld	a, b
		xor	80h ; 'А'
		ld	b, a
		ld	a, d
		jr	loc_DD93
; ---------------------------------------------------------------------------

loc_DD92:				; CODE XREF: sub_DD60+28j
		dec	a

loc_DD93:				; CODE XREF: sub_DD60+30j
		dec	a
		jr	loc_DD97
; ---------------------------------------------------------------------------

loc_DD96:				; CODE XREF: sub_DD60+24j
		inc	a

loc_DD97:				; CODE XREF: sub_DD60+34j
		sla	a
		sla	b
		rr	a
		inc	hl
		ld	(hl), a
		xor	a
		ld	hl, byte_6051
		ld	b, 12h

loc_DDA5:				; CODE XREF: sub_DD60+47j
		ld	(hl), a
		inc	hl
		djnz	loc_DDA5
		ld	hl, PlanetCrdZ_med
		ld	a, (byte_604F)
		cp	(hl)
		ld	a, 7Fh ; ''
		ld	(byte_605A), a
		jr	nz, loc_DDD0
		ld	bc, byte_6062
		ld	de, byte_6052
		ld	(bc), a
		ld	(de), a
		ld	hl, PlanetCrdX_hi
		ld	a, (byte_604A)
		cp	(hl)
		ld	a, 0FFh
		jr	c, loc_DDCD
		ld	(de), a
		jr	locret_DDEB
; ---------------------------------------------------------------------------

loc_DDCD:				; CODE XREF: sub_DD60+68j
		ld	(bc), a
		jr	locret_DDEB
; ---------------------------------------------------------------------------

loc_DDD0:				; CODE XREF: sub_DD60+55j
		ld	bc, byte_605E
		ld	de, byte_6056
		ld	hl, PlanetCrdZ_hi
		ld	a, (byte_6050)
		cp	(hl)
		jr	c, loc_DDE7
		ld	a, 0FFh
		jr	loc_DDE9
; ---------------------------------------------------------------------------

loc_DDE3:				; CODE XREF: sub_DCE9+11Bj
		ld	a, 5
		jr	loc_DE22
; ---------------------------------------------------------------------------

loc_DDE7:				; CODE XREF: sub_DD60+7Dj
		ld	a, 7Fh ; ''

loc_DDE9:				; CODE XREF: sub_DD60+81j
		ld	(bc), a
		ld	(de), a

locret_DDEB:				; CODE XREF: sub_DD60+6Bj sub_DD60+6Ej
		ret
; End of function sub_DD60


; =============== S U B	R O U T	I N E =======================================




; === sub_DDEC === CODE XREF: RAM:EA91p
sub_DDEC:				; CODE XREF: RAM:EA91p
		cp	0Fh
		ld	a, 42h ; 'B'
		jr	z, loc_DDF3
		ld	a, b

loc_DDF3:				; CODE XREF: sub_DDEC+4j
		ld	(byte_EB62), a
		ret
; End of function sub_DDEC

; ---------------------------------------------------------------------------
; START	OF FUNCTION CHUNK FOR sub_DCE9

loc_DDF7:				; CODE XREF: sub_DCE9+5Cj
		ld	a, (PilotNumGalaxy) ; ;---- до сюда после вылета в систему своим ходом
		and	a

loc_DDFB:				; DATA XREF: sub_5FD5+47o
		jr	z, loc_DE4C
		ld	hl, byte_606E	; Flg4 coriolis
		bit	7, (hl)		; была уничтожена станция таргонов
		res	7, (hl)
		jr	nz, loc_DDE3
		ld	hl, PilotMission3 ; =#06 при стыковке будет захват таргонами (вроде нереальная ситуация)
					; =#04 в системе со танцией таргонов во	время войны
		ld	a, (hl)
		and	a
		jr	z, loc_DE25
		dec	a
		jr	nz, loc_DE13
		inc	(hl)
		jr	loc_DE18
; ---------------------------------------------------------------------------

loc_DE13:				; CODE XREF: sub_DCE9+125j
		cp	3
		jr	nz, loc_DE18
		ld	(hl), a

loc_DE18:				; CODE XREF: sub_DCE9+128j
					; sub_DCE9+12Cj
		ld	hl, PilotPlanetData
		call	CompareGalaxyData
		jr	c, loc_DE25
		ld	a, 4

loc_DE22:				; CODE XREF: sub_DD60+85j
		ld	(PilotMission3), a ; =#06 при стыковке будет захват таргонами (вроде нереальная	ситуация)
					; =#04 в системе со танцией таргонов во	время войны

loc_DE25:				; CODE XREF: sub_DCE9+122j
					; sub_DCE9+135j
		ld	hl, PilotRating2 ; >=#84 в трюме десант
					; 7,=1 война была или идёт
					; 6,=1 война закончилать
					; 3-1 -	ранг (=текущая стадия войны)
		ld	a, (hl)
		inc	hl		; hl=JumpCounter
		inc	(hl)
		inc	hl		; hl=JumpCounter+1
		jr	nz, loc_DE2F
		inc	(hl)

loc_DE2F:				; CODE XREF: sub_DCE9+143j
		bit	7, (hl)
		res	7, (hl)
		jr	z, loc_DE39	; захват пиратами
		ld	l, 0B0h	; '░'   ; hl,PilotExtWeapon
		jr	loc_DE6F	; стандартные миссии
; ---------------------------------------------------------------------------

loc_DE39:				; CODE XREF: sub_DCE9+14Aj
					; sub_DCE9+1A3j
		ld	hl, RndWord	; захват пиратами
		ld	a, (PilotPlanetGov)
		xor	0FFh
		and	7
		rra
		cp	(hl)
		jr	c, loc_DE4C
		ld	hl, byte_6065
		res	7, (hl)
; END OF FUNCTION CHUNK	FOR sub_DCE9
; START	OF FUNCTION CHUNK FOR PrnCountdownJump

loc_DE4C:				; CODE XREF: PrnCountdownJump+BDj
					; sub_DCE9:loc_DDFBj ...
		ld	a, 1
		ld	(FlgJumpDocking), a ; =1 запрет	на обработку нажатий клавиш
					;   включено при: стыковке
					;   работает п/п отрисовки концентрический кругов
					;
					; 0,Flg_Interrupts
		xor	a
		ld	(WhatOnScreen),	a
		ld	(Temperature), a
		ld	(FuelScoopsOn),	a
		ld	(Missile1), a
		ld	(Mission2), a
		ld	a, 0A0h	; 'а'
		ld	(Altitude), a
		ret
; END OF FUNCTION CHUNK	FOR PrnCountdownJump

; =============== S U B	R O U T	I N E =======================================




; === sub_DE67 === CODE XREF: sub_E935+22p
sub_DE67:				; CODE XREF: sub_E935+22p
		ld	e, (hl)
		inc	hl
		ld	d, (hl)

loc_DE6A:				; DATA XREF: sub_5E1A+Dw
		nop
		ld	de, 10000
		ret
; End of function sub_DE67

; ---------------------------------------------------------------------------
; START	OF FUNCTION CHUNK FOR sub_DCE9

loc_DE6F:				; CODE XREF: sub_DCE9+14Ej
		and	(hl)
		rrca
		jr	c, loc_DE84
		set	0, (hl)		; выдача миссии	1
		ld	a, 1
		ld	(PilotMission1), a
		ld	l, 2
		call	j_SetAtrViewScr
		call	SetFuelZero
		jr	loc_DE4C
; ---------------------------------------------------------------------------

loc_DE84:				; CODE XREF: sub_DCE9+188j
		rrca
		jr	c, loc_DE8B
		set	1, (hl)		; выдача миссии	2
		jr	loc_DE4C
; ---------------------------------------------------------------------------

loc_DE8B:				; CODE XREF: sub_DCE9+19Cj
		rrca
		jr	c, loc_DE39	; захват пиратами
		set	2, (hl)		; выдача миссии	3
		ld	a, 1
		ld	(PilotMission3), a ; =#06 при стыковке будет захват таргонами (вроде нереальная	ситуация)
					; =#04 в системе со танцией таргонов во	время войны
		jr	loc_DE4C
; END OF FUNCTION CHUNK	FOR sub_DCE9

; =============== S U B	R O U T	I N E =======================================


SetFuelZero:				; CODE XREF: sub_D1E9+Dj sub_DCE9+196p
		ld	a, (Pilot_Fuel)
		and	a
		ret	z
		ld	hl, NoFuel
		set	7, (hl)
		xor	a
; End of function SetFuelZero


; =============== S U B	R O U T	I N E =======================================


SetPilotFuel:				; CODE XREF: sub_DAB4+C1p
					; PrnCountdownJump+74p	...
		cp	47h ; 'G'
		jr	c, loc_DEA8
		ld	a, 46h ; 'F'

loc_DEA8:				; CODE XREF: SetPilotFuel+2j
		ld	(Pilot_Fuel), a
		sla	a
		ld	b, a
		srl	a
		srl	a
		srl	a
		add	a, b
		ld	(Fuel),	a
		ret
; End of function SetPilotFuel

; ---------------------------------------------------------------------------
byte_DEB9:	db 0			; DATA XREF: sub_DCE9+38o


; === sub_DF1A === CODE XREF: RAM:DF0Ep	RAM:DF14p
sub_DF1A:				; CODE XREF: RAM:DF0Ep	RAM:DF14p
		ld	e, (hl)
		rlc	e
		ld	(hl), e
		inc	hl
		ld	d, (hl)
		rlc	d
		ld	(hl), d
		ret
; End of function sub_DF1A


; =============== S U B	R O U T	I N E =======================================




; === sub_DF24 === CODE XREF: RAM:E3A9p
sub_DF24:				; CODE XREF: RAM:E3A9p
		call	CopyGalaxyToPlanet
		xor	a
		ld	(byte_DF00), a

loc_DF2B:				; CODE XREF: sub_DF24+23j
		call	SavePlanetData
		call	GeneratePlanetName
		ld	hl, byte_DED0
		ld	de, TempName	; "LAVE"

loc_DF37:				; CODE XREF: sub_DF24+1Dj
		ld	b, (hl)
		ld	a, (de)
		cp	b
		jr	nz, loc_DF43
		and	a
		jr	z, loc_DF4C
		inc	hl
		inc	de
		jr	loc_DF37
; ---------------------------------------------------------------------------

loc_DF43:				; CODE XREF: sub_DF24+16j
		ld	hl, byte_DF00
		inc	(hl)
		jp	nz, loc_DF2B
		scf
		ret
; ---------------------------------------------------------------------------

loc_DF4C:				; CODE XREF: sub_DF24+19j
		call	RestorePlanetData
		call	PrintSmallCursOnMap
		call	GenerateInfoPlanet
		call	PrintSmallCursOnMap
		call	CopyPlanetName
		jp	CalcFuelForJump
; End of function sub_DF24

; ---------------------------------------------------------------------------


; === sub_E03D === CODE XREF: GeneratePlanetName+15p
sub_E03D:				; CODE XREF: GeneratePlanetName+15p
					; GeneratePlanetName+18p ...
		ld	hl, (DataOfPlanet2)
		ld	a, h
		and	1Fh
		and	a
		jr	z, ModifyPlanetData
		ld	hl, byte_FDB3
		add	a, a
		ld	d, 0
		ld	e, a
		add	hl, de
		ld	a, (hl)
		inc	hl
		push	hl
		ld	hl, (word_DF01)
		ld	(hl), a
		inc	hl
		ex	de, hl
		pop	hl
		ld	a, (hl)
		ex	de, hl
		cp	3Fh ; '?'
		jr	z, loc_E060
		ld	(hl), a
		inc	hl

loc_E060:				; CODE XREF: sub_E03D+1Fj
		ld	(word_DF01), hl
; End of function sub_E03D


; =============== S U B	R O U T	I N E =======================================


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


; === sub_E1E8 === CODE XREF: RAM:5FC9p	RAM:loc_D114p ...
sub_E1E8:				; CODE XREF: RAM:5FC9p	RAM:loc_D114p ...
		ld	a, (FlagsSetUp)
		bit	7, a
		ret	z
		ld	hl, 80h	; 'А'
		ld	de, 60h	; '`'
		jp	3B5h
; End of function sub_E1E8


; =============== S U B	R O U T	I N E =======================================




; === sub_E1F7 === CODE XREF: RAM:5FC3p	RAM:D117p ...
sub_E1F7:				; CODE XREF: RAM:5FC3p	RAM:D117p ...
		call	sub_E35A
		call	ResetPressedKeys

loc_E1FD:				; CODE XREF: sub_E1F7+Bj
		ld	a, (PressKeyPrnSymbol)
		cp	0FFh
		jr	z, loc_E1FD
		ret
; End of function sub_E1F7


; =============== S U B	R O U T	I N E =======================================


ResetPressedKeys:			; CODE XREF: RAM:D120j	sub_E1F7+3p ...
		di
		ld	hl, ControlKey1
		ld	b, 4

loc_E20B:				; CODE XREF: ResetPressedKeys+9j
		ld	(hl), 0
		inc	hl
		djnz	loc_E20B
		ei
		ret
; End of function ResetPressedKeys


; =============== S U B	R O U T	I N E =======================================




; === sub_E212 === CODE XREF: RAM:E72Fp
sub_E212:				; CODE XREF: RAM:E72Fp
					; sub_EF48:loc_EF79p

; FUNCTION CHUNK AT E28C SIZE 00000004 BYTES

		ld	a, (byte_E1DF)
		add	a, 0D0h	; '╨'
		call	PrintMessage1_A
		call	SetCrdX_SP
; ---------------------------------------------------------------------------
		db 0Dh
; ---------------------------------------------------------------------------
		call	sub_E31E
		call	SetCrdX_SP
; ---------------------------------------------------------------------------
		db 0Fh
; ---------------------------------------------------------------------------
		ld	hl, (word_E1E2)
		inc	hl
		inc	hl
		ld	(word_E1E2), hl
		ld	e, (hl)
		inc	hl
		ld	d, (hl)
		ex	de, hl
		scf
		ld	a, 4
		ld	(byte_D0CE), a
		call	PrintNumber_HL
		call	SetCrdX_SP
; ---------------------------------------------------------------------------
		db 17h
; ---------------------------------------------------------------------------
		ld	hl, (word_E1E4)
		inc	hl
		ld	a, (hl)
		ld	(word_E1E4), hl
		and	a
		jr	nz, loc_E254
		call	PrintSymbol_SP
; ---------------------------------------------------------------------------
		db 20h
; ---------------------------------------------------------------------------
		ld	a, 2Dh ; '-'
		call	PrnMsg_A_CRLF
		scf
		ret
; ---------------------------------------------------------------------------

loc_E254:				; CODE XREF: sub_E212+35j
		ld	a, 2
		ld	(byte_D0CE), a
		ld	hl, (word_E1E4)
		ld	a, (hl)
		call	PrintNumber_A
		call	sub_E31E
		call	PrnCrd_CRLF
		and	a
		ret
; End of function sub_E212

; ---------------------------------------------------------------------------

loc_E268:				; CODE XREF: sub_E87E+83p
					; RAM:loc_F107p
		ld	hl, (word_E1E0)
		inc	hl
		ld	(word_E1E0), hl
		ld	a, (hl)
		and	a
		jr	z, loc_E290
		ld	a, (byte_E1DF)
		add	a, 0D0h	; '╨'
		call	sub_5E1A
		call	SetCrdX_SP
; ---------------------------------------------------------------------------
		db 0Dh
; ---------------------------------------------------------------------------
		ld	a, 3
		ld	(byte_D0CE), a
		ld	hl, (word_E1E0)
		ld	a, (hl)
		call	PrintNumber_A
; ---------------------------------------------------------------------------
		db 0CDh	; ═
; ---------------------------------------------------------------------------
; START	OF FUNCTION CHUNK FOR sub_E212
		ld	e, 0E3h	; 'у'
		and	a
		ret
; END OF FUNCTION CHUNK	FOR sub_E212
; ---------------------------------------------------------------------------

loc_E290:				; CODE XREF: RAM:E271j
		scf
		ret

; =============== S U B	R O U T	I N E =======================================




; === sub_E292 === CODE XREF: RAM:loc_EE23p RAM:EE43p ...
sub_E292:				; CODE XREF: RAM:loc_EE23p RAM:EE43p ...
		call	PrintMessage2
		jr	loc_E29A
; ---------------------------------------------------------------------------

loc_E297:				; CODE XREF: sub_D822+2p sub_D822+17p	...
		call	PrintMessage1_A

loc_E29A:				; CODE XREF: sub_E292+3j
		ld	a, 3Ah ; ':'
		jr	loc_E2A3
; End of function sub_E292


; =============== S U B	R O U T	I N E =======================================


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




; === sub_E2B1 === CODE XREF: sub_D499+55p RAM:D8BDj
sub_E2B1:				; CODE XREF: sub_D499+55p RAM:D8BDj
		ld	a, (CrdXPrnSymbol+1)
		inc	a
		jr	loc_E2C8
; ---------------------------------------------------------------------------

loc_E2B7:				; CODE XREF: PrnCrd_CRLF+5p
					; sub_E2B1+22j	...
		xor	a

loc_E2B8:				; CODE XREF: RAM:D8CBj
		ld	(CrdXPrnSymbol), a
		ret
; ---------------------------------------------------------------------------

SetCrdX_SP:				; CODE XREF: RAM:6EFAp	RAM:6F02p ...
		pop	hl
		ld	a, (hl)
		inc	hl
		push	hl

loc_E2C0:				; CODE XREF: RAM:EA9Fp	sub_F072+5j
		ld	(CrdXPrnSymbol), a
		cp	20h ; ' '
		ret	c
		jr	PrnCrd_CRLF
; ---------------------------------------------------------------------------

loc_E2C8:				; CODE XREF: RAM:8D49j	RAM:loc_D8F5j ...
		ld	(CrdXPrnSymbol+1), a
		cp	18h
		ret	c
		ld	a, 1
		ld	(CrdXPrnSymbol+1), a
		jr	loc_E2B7
; End of function sub_E2B1


; =============== S U B	R O U T	I N E =======================================




; === sub_E2D5 === CODE XREF: RAM:E439p
sub_E2D5:				; CODE XREF: RAM:E439p
					; sub_E776:loc_E76Ep
		ld	hl, CrdXPrnSymbol
		dec	(hl)
		push	hl
		ld	hl, loc_D548
		push	hl
		ld	a, (hl)
		push	af
		ld	(hl), 0
		call	PrintSymbol_SP
; ---------------------------------------------------------------------------
		db 20h
; ---------------------------------------------------------------------------
		pop	af
		pop	hl
		ld	(hl), a
		pop	hl
		dec	(hl)
		ret
; End of function sub_E2D5

; ---------------------------------------------------------------------------
		db    0
		db    0
		db    0
		db    0
		db    0
		db    0
		db    0
		db    0
		db    0
		db    0
		db    0
		db    0

; =============== S U B	R O U T	I N E =======================================




; === sub_E2F8 === CODE XREF: sub_E306p	RAM:E463p ...
sub_E2F8:				; CODE XREF: sub_E306p	RAM:E463p ...
		push	af
		call	PrintSymbol_SP
; ---------------------------------------------------------------------------
		db 6
; ---------------------------------------------------------------------------
		pop	af
		call	PrintMessage1_A
		ld	a, 8
		jp	PrintMessage1_A
; End of function sub_E2F8


; =============== S U B	R O U T	I N E =======================================




; === sub_E306 === CODE XREF: sub_E87E+75p
sub_E306:				; CODE XREF: sub_E87E+75p
					; PrintHeaderMap+6p ...
		call	sub_E2F8
		call	sub_DA2F
		ld	a, 9
		jp	PrintMessage1_A
; End of function sub_E306


; =============== S U B	R O U T	I N E =======================================

; Attributes: noreturn



; === sub_E311 === CODE XREF: sub_D822:loc_D878j
sub_E311:				; CODE XREF: sub_D822:loc_D878j
					; sub_E311+Bj ...
		ld	a, (hl)
		cp	0
		scf
		ret	z
		inc	hl
		push	hl
		call	PrintMessage1_A
		pop	hl
		jr	sub_E311
; End of function sub_E311


; =============== S U B	R O U T	I N E =======================================




; === sub_E31E === CODE XREF: sub_E212+Cp sub_E212+4Ep	...
sub_E31E:				; CODE XREF: sub_E212+Cp sub_E212+4Ep	...
		ld	a, 0
		ld	(byte_D0D2), a
		ld	a, (byte_E1DF)
		cp	0Dh
		jr	c, loc_E336
		cp	0Fh
		jr	c, loc_E33A
		jr	z, loc_E33F
		cp	11h
		jr	c, loc_E336
		jr	loc_E343
; ---------------------------------------------------------------------------

loc_E336:				; CODE XREF: sub_E31E+Aj sub_E31E+14j
		ld	a, 54h ; 'T'
		jr	loc_E349
; ---------------------------------------------------------------------------

loc_E33A:				; CODE XREF: sub_E31E+Ej
		ld	a, 4Bh ; 'K'
		call	PrintMessage1_A

loc_E33F:				; CODE XREF: sub_E31E+10j
		ld	a, 47h ; 'G'
		jr	loc_E349
; ---------------------------------------------------------------------------

loc_E343:				; CODE XREF: sub_E31E+16j
		call	PrintSymbol_SP
; ---------------------------------------------------------------------------
		db 4Bh
; ---------------------------------------------------------------------------
		ld	a, 4Dh ; 'M'

loc_E349:				; CODE XREF: sub_E31E+1Aj sub_E31E+23j
		jp	PrintMessage1_A
; End of function sub_E31E


; =============== S U B	R O U T	I N E =======================================




; === sub_E34C === CODE XREF: RAM:E7F7p	sub_E935+Fp
sub_E34C:				; CODE XREF: RAM:E7F7p	sub_E935+Fp
		ld	a, (byte_E1DF)
		cp	0Dh
		ret	c
		cp	10h
		jr	nz, loc_E358
		scf
		ret
; ---------------------------------------------------------------------------

loc_E358:				; CODE XREF: sub_E34C+8j
		and	a
		ret
; End of function sub_E34C


; =============== S U B	R O U T	I N E =======================================




; === sub_E35A === CODE XREF: RAM:D8C6j	sub_E1F7p ...
sub_E35A:				; CODE XREF: RAM:D8C6j	sub_E1F7p ...
		ld	b, 14h
		jr	loc_E360
; End of function sub_E35A


; =============== S U B	R O U T	I N E =======================================


Pause_24Int:				; CODE XREF: RAM:D11Dp	RAM:E835p ...
		ld	b, 24h ; '$'

loc_E360:				; CODE XREF: sub_E35A+2j
					; Pause_24Int+3j
		halt
		djnz	loc_E360
		ret
; End of function Pause_24Int


; =============== S U B	R O U T	I N E =======================================




; === sub_E364 === CODE XREF: RAM:E734p	sub_E87E+88p
sub_E364:				; CODE XREF: RAM:E734p	sub_E87E+88p
		ld	hl, (CrdXPrnSymbol)
		ld	(word_E1E6), hl
		ret
; End of function sub_E364


; =============== S U B	R O U T	I N E =======================================




; === sub_E36B === CODE XREF: RAM:D8C3j	RAM:E3A6j ...
sub_E36B:				; CODE XREF: RAM:D8C3j	RAM:E3A6j ...
		ld	a, 15h
		call	loc_E2C8
		call	loc_E2B7
		xor	a
		ld	hl, 50A1h
		call	sub_E37D
		ld	hl, 50C1h
; End of function sub_E36B


; =============== S U B	R O U T	I N E =======================================




; === sub_E37D === CODE XREF: sub_E36B+Cp
sub_E37D:				; CODE XREF: sub_E36B+Cp
		ld	b, 8

loc_E37F:				; CODE XREF: sub_E37D+Dj
		push	bc
		ld	b, 1Eh
		push	hl

loc_E383:				; CODE XREF: sub_E37D+8j
		ld	(hl), a
		inc	hl
		djnz	loc_E383
		pop	hl
		inc	h
		pop	bc
		djnz	loc_E37F
		ret
; End of function sub_E37D


; =============== S U B	R O U T	I N E =======================================




; === sub_E38D === CODE XREF: RAM:loc_E74Fp
sub_E38D:				; CODE XREF: RAM:loc_E74Fp
					; sub_E87E:loc_E917p
		ld	hl, (word_E1E6)
		ld	(CrdXPrnSymbol), hl
		ret
; End of function sub_E38D

; ---------------------------------------------------------------------------
byte_E394:	db 0			; DATA XREF: RAM:E3A2r	sub_E3CC+11w ...
word_E395:	dw 0			; DATA XREF: sub_E3CC+Dw RAM:E41Br ...
byte_E397:	db 0			; DATA XREF: RAM:E3F0w	RAM:E3FBr
; ---------------------------------------------------------------------------

loc_E398:				; CODE XREF: RAM:D02Dj
		ld	a, 0Eh
		ld	b, 8
		ld	hl, byte_DED0
		call	sub_E3CC
		ld	a, (byte_E394)
		and	a
		jp	z, sub_E36B
		call	sub_DF24
		jr	nc, loc_E3C2

; =============== S U B	R O U T	I N E =======================================

; Attributes: noreturn



; === sub_E3CC === CODE XREF: RAM:E39Fp
sub_E3CC:				; CODE XREF: RAM:E39Fp
		ld	(byte_E3C9), a
		ld	a, b
		ld	(loc_E412+1), a
		ld	(word_E3CA), hl
		ld	hl, (word_E3CA)
		ld	(word_E395), hl
		xor	a
		ld	(byte_E394), a
		call	sub_E36B
		call	PrintSymbol_SP
		ld	b, 3Ah ; ':'
		ret
; End of function sub_E3CC

; ---------------------------------------------------------------------------
		ex	(sp), hl
		call	PrintMessage2

loc_E3ED:				; CODE XREF: RAM:E404j	RAM:E40Cj ...
		call	sub_E1F7
		ld	(byte_E397), a
		cp	0Dh
		jr	z, loc_E446
		cp	0FEh ; '■'
		jr	z, loc_E428
		ld	a, (byte_E397)
		cp	30h ; '0'
		jr	c, loc_E446
		cp	5Bh ; '['
		jr	nc, loc_E3ED
		cp	3Ah ; ':'
		jr	c, loc_E40E
		cp	40h ; '@'
		jr	c, loc_E3ED

loc_E40E:				; CODE XREF: RAM:E408j
		ld	d, a
		ld	a, (byte_E394)

loc_E412:				; DATA XREF: sub_E3CC+4w
		cp	0Ah
		jr	nc, loc_E446
		inc	a
		ld	(byte_E394), a
		ld	a, d
		ld	hl, (word_E395)
		ld	(hl), a
		inc	hl
		ld	(word_E395), hl
		call	PrintMessage1_A
		jr	loc_E3ED
; ---------------------------------------------------------------------------

loc_E428:				; CODE XREF: RAM:E3F9j
		ld	a, (byte_E394)
		and	a
		jr	z, loc_E3ED
		dec	a
		ld	(byte_E394), a
		ld	hl, (word_E395)
		dec	hl
		ld	(word_E395), hl
		call	sub_E2D5
		ld	b, 6

loc_E43E:				; CODE XREF: RAM:E43Fj
		halt
		djnz	loc_E43E
		call	ResetPressedKeys
		jr	loc_E3ED
; ---------------------------------------------------------------------------

loc_E446:				; CODE XREF: RAM:E3F5j	RAM:E400j ...
		ld	hl, (word_E395)
		ld	(hl), 0
		call	PrnCrd_CRLF
		call	sub_E35A

loc_E451:				; CODE XREF: RAM:E456j
		ld	a, (PressKeyPrnSymbol)
		cp	0Dh
		jr	z, loc_E451
		jp	ResetPressedKeys
; ---------------------------------------------------------------------------

loc_E45B:				; CODE XREF: RAM:D02Aj
		call	SearchNearPlanet

loc_E45E:				; CODE XREF: RAM:E3C6j
		call	sub_E36B
		ld	a, 3
		call	sub_E2F8
		call	PrnCrd_CRLF
		call	loc_E107
		jp	sub_E1E8

; =============== S U B	R O U T	I N E =======================================


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


; === sub_E611 === CODE XREF: PrintSmallCursOnMap:loc_E5B4p
sub_E611:				; CODE XREF: PrintSmallCursOnMap:loc_E5B4p
					; PrintSmallCursOnMap:loc_E5C0p ...
		xor	a

loc_E612:				; CODE XREF: sub_E611+4j
		scf
		rra
		dec	e
		jr	nz, loc_E612
		ld	e, a
		ret
; End of function sub_E611


; =============== S U B	R O U T	I N E =======================================




; === sub_E619 === CODE XREF: PrintSmallCursOnMap+148p
sub_E619:				; CODE XREF: PrintSmallCursOnMap+148p
					; PrintSmallCursOnMap:loc_E5C7p ...
		xor	a

loc_E61A:				; CODE XREF: sub_E619+4j
		scf
		rla
		dec	d
		jr	nz, loc_E61A
		ld	d, a
		ret
; End of function sub_E619


; =============== S U B	R O U T	I N E =======================================




; === sub_E621 === CODE XREF: PrintSmallCursOnMap+D3p
sub_E621:				; CODE XREF: PrintSmallCursOnMap+D3p
					; PrintSmallCursOnMap+105p ...
		rra
		rra
		rra
		and	1Fh
		ld	l, a
		ld	a, c
		and	38h ; '8'
		add	a, a
		add	a, a
		or	l
		ld	l, a
		ld	a, c
		and	7
		or	40h ; '@'
		ld	h, a
		ld	a, c
		and	0C0h ; '└'
		rra
		rra
		rra
		and	1Fh
		or	h
		ld	h, a
		ret
; End of function sub_E621

; ---------------------------------------------------------------------------
; START	OF FUNCTION CHUNK FOR PrintSmallCursOnMap

loc_E63F:				; CODE XREF: PrintSmallCursOnMap:loc_E5E1j
		ld	a, (hl)
		xor	e
		ld	(hl), a
		jr	loc_E647
; ---------------------------------------------------------------------------

loc_E644:				; CODE XREF: PrintSmallCursOnMap+15Dj
		ld	a, (hl)
		xor	d
		ld	(hl), a

loc_E647:				; CODE XREF: PrintSmallCursOnMap+1A0j
					; PrintSmallCursOnMap+1D3j
		ld	a, (CursorWidth)
		srl	a
		ld	b, a
		ld	a, (byte_E52C)
		ld	c, a
		ld	a, (PrnCrdYCursor)
		sub	b
		jr	c, loc_E674
		cp	c
		jr	c, loc_E674
		ld	c, a
		ld	a, (PrnCrdXCursor)
		call	sub_E621
		ld	a, (CrdYDownGalaxyMap)
		ld	c, a
		ld	a, (PrnCrdYCursor)
		add	a, b
		sub	c
		ld	c, a
		ld	a, (CursorWidth)
		jr	c, loc_E671
		sub	c

loc_E671:				; CODE XREF: PrintSmallCursOnMap+1FFj
		ld	b, a
		jr	loc_E683
; ---------------------------------------------------------------------------

loc_E674:				; CODE XREF: PrintSmallCursOnMap+1E6j
					; PrintSmallCursOnMap+1E9j
		ld	d, a
		ld	a, c
		sub	d
		ld	d, a
		ld	a, (CursorWidth)
		sub	d
		ld	b, a
		ld	a, (PrnCrdXCursor)
		call	sub_E621

loc_E683:				; CODE XREF: PrintSmallCursOnMap+203j
		ld	a, (PrnCrdXCursor)
		and	7
		inc	a
		ld	c, a
		xor	a
		scf
		rra
		dec	c
		jr	z, loc_E695

loc_E690:				; CODE XREF: PrintSmallCursOnMap+224j
		srl	a
		dec	c
		jr	nz, loc_E690

loc_E695:				; CODE XREF: PrintSmallCursOnMap+21Fj
		ld	e, a

loc_E696:				; CODE XREF: PrintSmallCursOnMap+231j
		ld	a, (hl)
		xor	e
		ld	(hl), a
		inc	h
		ld	a, h
		and	7
		call	z, sub_E6A3
		djnz	loc_E696
		ret
; END OF FUNCTION CHUNK	FOR PrintSmallCursOnMap

; =============== S U B	R O U T	I N E =======================================




; === sub_E6A3 === CODE XREF: PrintSmallCursOnMap+22Ep
sub_E6A3:				; CODE XREF: PrintSmallCursOnMap+22Ep
		ld	a, l
		add	a, 20h ; ' '
		ld	l, a
		ret	c
		ld	a, h
		sub	8
		ld	h, a
		ret
; End of function sub_E6A3

; ---------------------------------------------------------------------------
		db    0

; =============== S U B	R O U T	I N E =======================================


Cmd_2_BuyProducts:			; CODE XREF: RAM:FunctionKernelj

; FUNCTION CHUNK AT E707 SIZE 00000009 BYTES

		ld	a, (PilotExtWeapon) ; 7,=1 - ECM Jammer
					; 6,=1 - Cloaking device
					; 5,=1 - нельзя	купить товары(в	трюме беженцы или десант)
					; 4
					; 3 =1 3я миссия была выполнена
					; 2 =1 - третья	миссия выполнена
					; 1 =1 - второя	миссия выполнена
					; 0 =1 - первая	миссия выполнена
		bit	5, a
		jr	nz, loc_E6F1+1
		ld	de, 5B17h
		ld	hl, 201h
		call	GoMission1
		nop
		jr	nc, loc_E707
		call	sub_D162
		jr	sub_E6D6
; End of function Cmd_2_BuyProducts


; =============== S U B	R O U T	I N E =======================================




; === sub_E6C6 === CODE XREF: TestPilotInZoneWar+1Ap
sub_E6C6:				; CODE XREF: TestPilotInZoneWar+1Ap
					; TestPilotInZoneWar+24p
		push	af
		ld	a, 5Ch ; '\'
		call	ABS_A_sub_B
		ld	d, a
		call	Mult_HL_D_A
		pop	bc
		push	hl
		xor	a
		jp	loc_DFDE
; End of function sub_E6C6


; =============== S U B	R O U T	I N E =======================================




; === sub_E6D6 === CODE XREF: Cmd_2_BuyProducts+16j
sub_E6D6:				; CODE XREF: Cmd_2_BuyProducts+16j
		call	sub_E1F7
		cp	59h ; 'Y'
		jr	nz, loc_E6EB
		xor	a
		ld	hl, PilotCargo
		ld	b, 12h

loc_E6E3:				; CODE XREF: sub_E6D6+Fj
		ld	(hl), a
		inc	hl
		djnz	loc_E6E3
		ld	l, 0B0h	; '░'
		set	5, (hl)

loc_E6EB:				; CODE XREF: sub_E6D6+5j
		jp	Cmd_Ent_Inventory
; End of function sub_E6D6


; =============== S U B	R O U T	I N E =======================================




; === sub_E6EE === CODE XREF: LaunchDocking+82p
sub_E6EE:				; CODE XREF: LaunchDocking+82p
		ld	(CloakingDeviceOn), a
; End of function sub_E6EE


loc_E6F1:				; CODE XREF: Cmd_2_BuyProducts+5j
		ld	a, (PilotRnd)
		ld	(RndWord), a
		ret
; ---------------------------------------------------------------------------
unk_E6F8:	db 0F1h	; ё		; DATA XREF: RAM:E716o
		db 0F0h	; Ё

; =============== S U B	R O U T	I N E =======================================




; === sub_E6FA === CODE XREF: Cmd_2_BuyProducts:loc_E707p
sub_E6FA:				; CODE XREF: Cmd_2_BuyProducts:loc_E707p
					; sub_EF48p
		ld	hl,  ProductPrice+15h
		ld	a, (hl)
		and	a
		ret	nz
		ld	(hl), 2
		dec	hl
		ld	a, 0C0h	; '└'
		ld	(hl), a
		ret
; End of function sub_E6FA

; ---------------------------------------------------------------------------
; START	OF FUNCTION CHUNK FOR Cmd_2_BuyProducts

loc_E707:				; CODE XREF: Cmd_2_BuyProducts+11j
		call	sub_E6FA
		call	ClsAndPrintRamk
		call	SetCrdX_SP
; END OF FUNCTION CHUNK	FOR Cmd_2_BuyProducts
; ---------------------------------------------------------------------------
		db 10h
; ---------------------------------------------------------------------------
		ld	a, 0FFh
		call	PrnMsg_A_CRLF
		ld	de, unk_E6F8
		call	SetAttrFrom_DE
		call	FillAttr_21line
		call	sub_E962
		ld	hl, PilotPlanetGov
		ld	(word_E1E4), hl

loc_E728:				; CODE XREF: RAM:E759j
		ld	hl, (word_E1E0)
		inc	hl
		ld	(word_E1E0), hl
		call	sub_E212
		jr	c, loc_E752
		call	sub_E364

loc_E737:				; CODE XREF: RAM:E744j
		call	sub_E87E
		ld	hl, loc_E7DB
		call	sub_E776
		jr	nc, loc_E74C
		cp	1
		jr	z, loc_E737
		cp	2
		jr	z, loc_E74F
		jr	loc_E75B
; ---------------------------------------------------------------------------

loc_E74C:				; CODE XREF: RAM:E740j
		call	sub_E7CF
; ---------------------------------------------------------------------------

loc_E74F:				; CODE XREF: RAM:E748j
		call	sub_E38D

loc_E752:				; CODE XREF: RAM:E732j
		ld	hl, byte_E1DF
		inc	(hl)
		ld	a, (hl)
		cp	11h
		jr	c, loc_E728

loc_E75B:				; CODE XREF: RAM:E74Aj
		jp	Cmd_Ent_Inventory
; ---------------------------------------------------------------------------
		ld	hl, 5820h
		ld	b, 0E0h	; 'р'
		call	sub_E769
		call	sub_E769

; =============== S U B	R O U T	I N E =======================================




; === sub_E769 === CODE XREF: RAM:E763p	sub_E769+2j
sub_E769:				; CODE XREF: RAM:E763p	sub_E769+2j
		ld	(hl), a
		inc	hl
		djnz	sub_E769
		ret
; End of function sub_E769

; ---------------------------------------------------------------------------
; START	OF FUNCTION CHUNK FOR sub_E776

loc_E76E:				; CODE XREF: sub_E776+38j
		call	sub_E2D5
		call	sub_E85D
		jr	loc_E779
; END OF FUNCTION CHUNK	FOR sub_E776

; =============== S U B	R O U T	I N E =======================================




; === sub_E776 === CODE XREF: RAM:E73Dp	sub_E87E+91p ...
sub_E776:				; CODE XREF: RAM:E73Dp	sub_E87E+91p ...

; FUNCTION CHUNK AT E76E SIZE 00000008 BYTES

		ld	(word_E8AC), hl

loc_E779:				; CODE XREF: sub_E776-2j
		xor	a
		ld	(byte_E8AB), a

loc_E77D:				; CODE XREF: sub_E776+10j sub_E776+14j
		call	sub_E1F7
		cp	0Dh
		jr	z, loc_E7CB
		cp	31h ; '1'
		jr	c, loc_E77D
		cp	0FEh ; '■'
		jr	z, loc_E77D
		cp	3Ah ; ':'
		jr	nc, loc_E7CD
		ld	b, a
		sub	30h ; '0'
		ld	(byte_E8AB), a
		ld	a, b
		call	PrintMessage1_A
		ld	hl, (word_E8AC)
		call	sub_E7DA
; ---------------------------------------------------------------------------
		jr	c, loc_E7CD

loc_E7A2:				; CODE XREF: sub_E776+34j
		call	sub_E1F7
		cp	0Dh
		ret	z
		cp	30h ; '0'
		jr	c, loc_E7A2
		cp	0FEh ; '■'
		jr	z, loc_E76E
		cp	3Ah ; ':'
		jr	nc, loc_E7CD
		ld	d, a
		sub	30h ; '0'
		ld	b, a
		ld	hl, byte_E8AB
		call	loc_712A
		call	sub_E85D
		ld	hl, (word_E8AC)
		call	sub_E7DA
; ---------------------------------------------------------------------------
		jr	c, loc_E7CD
		and	a
		ret
; ---------------------------------------------------------------------------

loc_E7CB:				; CODE XREF: sub_E776+Cj
		ld	a, 2

loc_E7CD:				; CODE XREF: sub_E776+18j sub_E776+2Aj ...
		scf
		ret
; End of function sub_E776


; =============== S U B	R O U T	I N E =======================================

; Attributes: noreturn



; === sub_E7CF === CODE XREF: RAM:loc_E74Cp
sub_E7CF:				; CODE XREF: RAM:loc_E74Cp
					; sub_E87E+96p	...
		call	SetCrdX_SP
; ---------------------------------------------------------------------------
		db 3
; ---------------------------------------------------------------------------
		call	PrintSymbol_SP
; ---------------------------------------------------------------------------
		db 7
; ---------------------------------------------------------------------------
		jp	j_FX_Beep
; End of function sub_E7CF


; =============== S U B	R O U T	I N E =======================================

; Attributes: noreturn



; === sub_E7DA === CODE XREF: sub_E776+27p sub_E776+4Ep
sub_E7DA:				; CODE XREF: sub_E776+27p sub_E776+4Ep
		jp	(hl)
; End of function sub_E7DA

; ---------------------------------------------------------------------------

loc_E7DB:				; DATA XREF: RAM:E73Ao
		call	sub_E83C
		ld	a, (byte_E8AB)
		ld	b, a
		ld	hl, (word_E1E4)
		ld	a, (hl)
		sub	b
		jr	c, loc_E818
		ld	(hl), a

loc_E7EA:				; CODE XREF: RAM:E7F5j
		ld	hl, (word_E1E2)
		ld	e, (hl)
		inc	hl
		ld	d, (hl)
		call	sub_D3FD
		jr	c, sub_E81C
		djnz	loc_E7EA
		call	sub_E34C
		ld	a, (byte_E8AB)
		ld	b, a
		jr	nc, loc_E808
		ld	hl, PilotFreeCargo
		ld	a, (hl)
		sub	b
		jr	c, loc_E820
		ld	(hl), a

loc_E808:				; CODE XREF: RAM:E7FEj
		ld	hl, (word_E1E0)
		ld	a, (hl)
		add	a, b
		jr	c, loc_E820
		ld	(hl), a
		ld	a, (byte_E1DF)
		call	sub_D1B5
		and	a
		ret
; ---------------------------------------------------------------------------
; START	OF FUNCTION CHUNK FOR sub_E935

loc_E818:				; CODE XREF: RAM:E7E7j	sub_E935+Bj
		ld	a, 0B0h	; '░'
		jr	loc_E822
; END OF FUNCTION CHUNK	FOR sub_E935

; =============== S U B	R O U T	I N E =======================================

; Attributes: noreturn



; === sub_E81C === CODE XREF: RAM:E7F3j	RAM:loc_EB27p
sub_E81C:				; CODE XREF: RAM:E7F3j	RAM:loc_EB27p
		ld	a, 0C5h	; '┼'
		jr	loc_E822
; ---------------------------------------------------------------------------

loc_E820:				; CODE XREF: RAM:E805j	RAM:E80Dj
		ld	a, 0CEh	; '╬'

loc_E822:				; CODE XREF: sub_E935-11Bj sub_E81C+2j
		push	af
		call	SetCrdX_SP
; ---------------------------------------------------------------------------
		db 3
; ---------------------------------------------------------------------------
		pop	af
		call	PrintMessage1_A
		call	PrintSymbol_SP
; ---------------------------------------------------------------------------
		db 3Fh
; ---------------------------------------------------------------------------
		call	sub_E85D
; End of function sub_E81C


; =============== S U B	R O U T	I N E =======================================

; Attributes: noreturn thunk



; === sub_E83C === CODE XREF: RAM:loc_E7DBp sub_E935p ...
sub_E83C:				; CODE XREF: RAM:loc_E7DBp sub_E935p ...
		ld	hl, (PilotCash2low)
		ld	(word_E8A6), hl
		ld	hl, (PilotCash1)
		ld	(word_E8A4), hl
		ld	hl, (word_E1E4)
		ld	a, (hl)
		ld	(byte_E8A9), a
		ld	hl, (word_E1E0)
		ld	a, (hl)
		ld	(byte_E8A8), a
		ld	a, (PilotFreeCargo)
		ld	(byte_E8AA), a
		ret
; End of function sub_E83C


; =============== S U B	R O U T	I N E =======================================




; === sub_E85D === CODE XREF: sub_E776-5p sub_E776+48p	...
sub_E85D:				; CODE XREF: sub_E776-5p sub_E776+48p	...
		ld	hl, (word_E8A6)
		ld	(PilotCash2low), hl
		ld	hl, (word_E8A4)
		ld	(PilotCash1), hl
		ld	hl, (word_E1E4)
		ld	a, (byte_E8A9)
		ld	(hl), a
		ld	hl, (word_E1E0)
		ld	a, (byte_E8A8)
		ld	(hl), a
		ld	a, (byte_E8AA)
		ld	(PilotFreeCargo), a
		ret
; End of function sub_E85D


; =============== S U B	R O U T	I N E =======================================




; === sub_E87E === CODE XREF: RAM:loc_E737p
sub_E87E:				; CODE XREF: RAM:loc_E737p
					; sub_E87E:loc_E909p

; FUNCTION CHUNK AT E8EA SIZE 0000004B BYTES
; FUNCTION CHUNK AT F0B8 SIZE 0000001B BYTES

		call	sub_E36B
		ld	a, 1
		ld	(byte_D0D2), a
		call	PrintSymbol_SP
; ---------------------------------------------------------------------------
		db 0CCh
; ---------------------------------------------------------------------------
		ld	a, 0D0h	; '╨'
		ld	b, a
		ld	a, (byte_E1DF)
		add	a, b
		call	PrintMessage1_A
		call	PrintSymbol_SP
; ---------------------------------------------------------------------------
		db 2Fh
; ---------------------------------------------------------------------------
		call	sub_E31E
		call	PrintSymbol_SP
; ---------------------------------------------------------------------------
		db 20h
; ---------------------------------------------------------------------------
		ld	a, 3Fh ; '?'
		jp	PrnMsg_A_CRLF
; ---------------------------------------------------------------------------
word_E8A4:	dw 0			; DATA XREF: sub_E83C+9w sub_E85D+6r
word_E8A6:	dw 0			; DATA XREF: sub_E83C+3w sub_E85Dr
byte_E8A8:	db 0			; DATA XREF: sub_E83C+17w sub_E85D+16r
byte_E8A9:	db 0			; DATA XREF: sub_E83C+10w sub_E85D+Fr
byte_E8AA:	db 0			; DATA XREF: sub_E83C+1Dw sub_E85D+1Ar
byte_E8AB:	db 0			; DATA XREF: sub_E776+4w sub_E776+1Dw	...
word_E8AC:	dw 0			; DATA XREF: sub_E776w	sub_E776+24r ...
; ---------------------------------------------------------------------------

Cmd_3_SellProducrs:			; CODE XREF: RAM:D003j
		ld	a, (PilotMission1)
		and	a
		jr	nz, loc_E8EA
		ld	hl, PilotExtWeapon ; 7,=1 - ECM	Jammer
					; 6,=1 - Cloaking device
					; 5,=1 - нельзя	купить товары(в	трюме беженцы или десант)
					; 4
					; 3 =1 3я миссия была выполнена
					; 2 =1 - третья	миссия выполнена
					; 1 =1 - второя	миссия выполнена
					; 0 =1 - первая	миссия выполнена
		call	sub_7FA1
		jr	z, loc_E8EA
		res	5, (hl)
		xor	a
		ld	(PilotMineralLife), a ;	<>0 в трюме есть минеральная жизнь
					; =#FF минеральная жизнь померла
		ld	a, 100
		ld	(cargoGemStones), a
		call	SetMaxTonnage	; установка максимальной грузоподъемности
		nop
		nop
		ld	de, 0F0Fh
		ld	hl, 700h
		call	sub_D148
		jp	Cmd_Ent_Inventory
; ---------------------------------------------------------------------------
unk_E8D8:	db 0F0h	; Ё		; DATA XREF: sub_E87E+5Eo
; ---------------------------------------------------------------------------

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




; === sub_E935 === DATA XREF: sub_E87E+8Eo
sub_E935:				; DATA XREF: sub_E87E+8Eo

; FUNCTION CHUNK AT E818 SIZE 00000004 BYTES

		call	sub_E83C
		ld	a, (byte_E8AB)
		ld	b, a
		call	sub_700F
		sub	b
		jp	c, loc_E818
		ld	(hl), a
		call	sub_E34C
		call	sub_D5F5
		jr	nc, loc_E953
		ld	a, (PilotFreeCargo)
		add	a, b
		ld	(PilotFreeCargo), a

loc_E953:				; CODE XREF: sub_E935+15j sub_E935+29j
		push	bc
		ld	hl, (word_E1E2)
		call	sub_DE67
		call	IncCASH
		pop	bc
		djnz	loc_E953
		and	a
		ret
; End of function sub_E935


; =============== S U B	R O U T	I N E =======================================




; === sub_E962 === CODE XREF: RAM:E71Fp	sub_E87E+78p
sub_E962:				; CODE XREF: RAM:E71Fp	sub_E87E+78p
		ld	hl, PilotRnd
		ld	(word_E1E0), hl
		ld	hl, byte_E1BB
		ld	(word_E1E2), hl
		xor	a
		ld	(byte_E1DF), a
		ret
; End of function sub_E962

; ---------------------------------------------------------------------------

Cmd_4_EquipShip:			; CODE XREF: RAM:D006j	RAM:EA38j ...
		call	PrintMenuRamk
		call	PrintSymbol_SP
; ---------------------------------------------------------------------------
		db 6
; ---------------------------------------------------------------------------
		ld	a, 0CFh	; '╧'
		call	PrintMessage1_A_Space
		call	PrintSymbol_SP
; ---------------------------------------------------------------------------
		db 0B9h
; ---------------------------------------------------------------------------
		call	PrintSymbol_SP
; ---------------------------------------------------------------------------
		db 8
; ---------------------------------------------------------------------------
		call	DubleCRLF
		ld	hl, unk_705F
		ld	(word_EB60), hl
		ld	a, (PilotPlanetTechLvl)
		add	a, 3
		cp	0Ch
		jr	c, loc_E99B
		ld	a, 0Eh

loc_E99B:				; CODE XREF: RAM:E997j
		ld	(byte_EB5E), a
		xor	a

loc_E99F:				; CODE XREF: RAM:E9DDj
		ld	(byte_EB5F), a
		ld	a, 3
		ld	(byte_D0CE), a
		ld	a, (byte_EB5F)
		inc	a
		call	PrintNumber_A
		call	SetCrdX_SP
; ---------------------------------------------------------------------------
		db 4
; ---------------------------------------------------------------------------
		ld	b, 69h ; 'i'
		ld	a, (byte_EB5F)
		add	a, b
		call	PrintMessage1_A
		call	SetCrdX_SP
; ---------------------------------------------------------------------------
		db 16h
; ---------------------------------------------------------------------------
		ld	hl, (word_EB60)
		ld	a, (hl)
		inc	hl
		ld	e, (hl)
		inc	hl
		ld	d, (hl)
		inc	hl
		ld	(word_EB60), hl
		ld	h, 0
		ld	l, a
		call	PrintNumber_HLDE
		call	PrnCrd_CRLF
		ld	a, (byte_EB5E)
		ld	b, a
		ld	a, (byte_EB5F)
		inc	a
		cp	b
		jr	nz, loc_E99F

loc_E9DF:				; CODE XREF: RAM:EA7Cj	RAM:EB2Aj
		call	loc_6EE8
		call	PrintSymbol_SP
; ---------------------------------------------------------------------------
		db 7Fh
; ---------------------------------------------------------------------------
		ld	a, 3Fh ; '?'
		call	PrintMessage1_A_Space
		call	sub_E83C
		ld	hl, word_EB2D
		call	sub_E776
		jr	c, loc_EA05
		call	PrnCrd_CRLF
		ld	a, (byte_E8AB)
		and	a
		jp	z, Cmd_L_PilotInfo
		ld	b, a
		call	sub_D8CE
		cp	b

loc_EA05:				; CODE XREF: RAM:E9F4j
		jp	c, Cmd_L_PilotInfo
		call	sub_E83C
		ld	a, (byte_E8AB)
		call	sub_EB2F
		ld	d, 0
		ld	e, (hl)
		push	hl
		call	sub_D3EF
		pop	hl
		jr	c, loc_EA22
		inc	hl
		ld	e, (hl)
		inc	hl
		ld	d, (hl)
		call	sub_D3FD

loc_EA22:				; CODE XREF: RAM:EA19j
		jp	c, loc_EB27
		ld	a, (byte_E8AB)
		cp	1
		jr	nz, loc_EA3B
		xor	a
		ld	(byte_7060), a
		call	sub_E7CF
; ---------------------------------------------------------------------------
		ld	a, 46h ; 'F'
		call	SetPilotFuel
		jp	Cmd_4_EquipShip
; ---------------------------------------------------------------------------

loc_EA3B:				; CODE XREF: RAM:EA2Aj
		cp	2
		jr	nz, loc_EA4C
		ld	hl, PilotMissiles
		ld	a, (hl)
		cp	4
		jp	nc, loc_EB43
		inc	a
		ld	(hl), a
		jr	loc_EA79
; ---------------------------------------------------------------------------

loc_EA4C:				; CODE XREF: RAM:EA3Dj
		cp	3
		jr	nz, loc_EA60
		ld	hl, PilotLargeCargo
		ld	a, (hl)
		jp	sub_6FFB
; ---------------------------------------------------------------------------
; START	OF FUNCTION CHUNK FOR sub_6FFB

loc_EA57:				; CODE XREF: sub_6FFB+11j
		ld	hl, PilotFreeCargo
		ld	a, (hl)
		add	a, 0Fh
		ld	(hl), a
		jr	loc_EA75
; END OF FUNCTION CHUNK	FOR sub_6FFB
; ---------------------------------------------------------------------------

loc_EA60:				; CODE XREF: RAM:EA4Ej
		cp	4
		jr	z, loc_EA6C
		cp	7
		jr	c, loc_EA7F
		cp	0Dh
		jr	nc, loc_EA7F

loc_EA6C:				; CODE XREF: RAM:EA62j
		call	sub_EB38
		ld	a, (hl)
		or	80h ; 'А'
		call	sub_EC4B
; START	OF FUNCTION CHUNK FOR sub_6FFB

loc_EA75:				; CODE XREF: sub_6FFB+7A63j
		call	sub_EB38
		dec	(hl)

loc_EA79:				; CODE XREF: RAM:EA4Aj
		call	sub_E7CF
; END OF FUNCTION CHUNK	FOR sub_6FFB
; ---------------------------------------------------------------------------
		jp	loc_E9DF
; ---------------------------------------------------------------------------

loc_EA7F:				; CODE XREF: RAM:EA66j	RAM:EA6Aj
		ld	b, 1
		cp	5
		jr	z, loc_EA90
		inc	b
		cp	6
		jr	z, loc_EA90
		inc	b
		cp	0Dh
		jr	z, loc_EA90
		inc	b

loc_EA90:				; CODE XREF: RAM:EA83j	RAM:EA88j ...
		nop
		call	sub_DDEC
		push	bc
		call	ClsAndPrintRamk
		call	FillAttr_21line
		pop	bc
		ld	a, 4
		add	a, b
		call	loc_E2C0
		ld	a, 0Eh
		call	loc_E2C8
		ld	a, (byte_E8AB)
		ld	b, 68h ; 'h'
		add	a, b
		call	PrintMessage1_A
		ld	a, 10h
		call	loc_E2C8
		xor	a

loc_EAB6:				; CODE XREF: RAM:EAD3j
		ld	(byte_EB5F), a
		call	SetCrdX_SP
; ---------------------------------------------------------------------------
		db 0Ah
; ---------------------------------------------------------------------------
		ld	a, (byte_EB5F)
		add	a, 30h ; '0'
		call	PrintMessage1_A_Space
		ld	a, (byte_EB5F)
		add	a, 60h ; '`'
		call	PrnMsg_A_CRLF
		ld	a, (byte_EB5F)
		inc	a
		cp	4
		jr	c, loc_EAB6
		call	sub_E36B
		call	PrintSymbol_SP
; ---------------------------------------------------------------------------
		db 0AFh
; ---------------------------------------------------------------------------
		call	PrintSymbol_SP
; ---------------------------------------------------------------------------
		db 3Fh
; ---------------------------------------------------------------------------
		call	sub_E1F7
		cp	30h ; '0'
		jp	c, loc_EB22
		cp	34h ; '4'
		jp	nc, loc_EB22
		push	af
		call	PrnMsg_A_CRLF
		pop	af
		sub	30h ; '0'
		ld	hl, PilotFrontLaser
		call	sub_EB3E
		ld	b, (hl)
		ld	a, (byte_EB62)
		ld	(hl), a
		ld	a, b
		and	a
		jr	z, loc_EB1C
		ld	hl,  loc_FD87+2
		call	sub_EB3E
		ld	a, (hl)
		call	sub_EB2F
		ld	d, 0
		ld	e, (hl)
		push	hl
		call	loc_D3E7
		pop	hl
		inc	hl
		ld	e, (hl)
		inc	hl
		ld	d, (hl)
		call	IncCASH

loc_EB1C:				; CODE XREF: RAM:EB01j
		call	sub_E7CF
; ---------------------------------------------------------------------------
		jp	Cmd_4_EquipShip
; ---------------------------------------------------------------------------

loc_EB22:				; CODE XREF: RAM:EAE5j	RAM:EAEAj
		call	sub_E85D
		jr	loc_EB5B
; ---------------------------------------------------------------------------

loc_EB27:				; CODE XREF: RAM:loc_EA22j
		call	sub_E81C
; ---------------------------------------------------------------------------
		jp	loc_E9DF
; ---------------------------------------------------------------------------
word_EB2D:	dw 0C9A7h		; DATA XREF: RAM:E9EEo

; =============== S U B	R O U T	I N E =======================================




; === sub_EB2F === CODE XREF: RAM:EA0Ep	RAM:EB0Ap
sub_EB2F:				; CODE XREF: RAM:EA0Ep	RAM:EB0Ap
		dec	a
		ld	hl, unk_705F
		call	loc_80B4
		jr	sub_EB3E
; End of function sub_EB2F


; =============== S U B	R O U T	I N E =======================================




; === sub_EB38 === CODE XREF: RAM:loc_EA6Cp
sub_EB38:				; CODE XREF: RAM:loc_EA6Cp
					; sub_6FFB:loc_EA75p
		ld	hl, PilotMission1
		ld	a, (byte_E8AB)
; End of function sub_EB38


; =============== S U B	R O U T	I N E =======================================




; === sub_EB3E === CODE XREF: sub_D1E9+39p RAM:EAF7p ...
sub_EB3E:				; CODE XREF: sub_D1E9+39p RAM:EAF7p ...
		ld	d, 0
		ld	e, a
		add	hl, de
		ret
; End of function sub_EB3E

; ---------------------------------------------------------------------------

loc_EB43:				; CODE XREF: RAM:EA45j
		ld	a, 7Ch ; '|'
		jr	loc_EB4E
; ---------------------------------------------------------------------------
; START	OF FUNCTION CHUNK FOR sub_6FFB

loc_EB47:				; CODE XREF: sub_6FFB+Ej
					; DATA XREF: sub_EC4B+8o
		ld	a, 68h ; 'h'
		ld	b, a
		ld	a, (byte_E8AB)
		add	a, b

loc_EB4E:				; CODE XREF: RAM:EB45j
		call	PrintMessage1_A_Space
		call	PrintSymbol_SP
; END OF FUNCTION CHUNK	FOR sub_6FFB
; ---------------------------------------------------------------------------
		db 1Eh
; ---------------------------------------------------------------------------
		call	j_FX_Beep
; ---------------------------------------------------------------------------
		db 0CDh	; ═
		db  5Dh	; ]
		db 0E8h	; ш
; ---------------------------------------------------------------------------

loc_EB5B:				; CODE XREF: RAM:EB25j
		jp	Cmd_L_PilotInfo
; ---------------------------------------------------------------------------
byte_EB5E:	db 0			; DATA XREF: sub_D8CE:loc_D8D0r
					; RAM:loc_E99Bw ...
byte_EB5F:	db 0			; DATA XREF: RAM:loc_E99Fw RAM:E9A7r ...
word_EB60:	dw 0			; DATA XREF: RAM:E98Dw	RAM:E9BFr ...
byte_EB62:	db 0			; DATA XREF: sub_DDEC:loc_DDF3w
					; RAM:EAFBr

; =============== S U B	R O U T	I N E =======================================


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




; === sub_EBEE === CODE XREF: Cmd_I_GlobalMap+37p
sub_EBEE:				; CODE XREF: Cmd_I_GlobalMap+37p
					; Cmd_I_GlobalMap+3Fp
		ld	e, 8

loc_EBF0:				; CODE XREF: sub_EBEE+11j
		push	hl
		ld	d, a

loc_EBF2:				; CODE XREF: sub_EBEE:loc_EBFAj
		ld	bc, 2000h

loc_EBF5:				; CODE XREF: sub_EBEE+9j
		dec	hl
		ld	(hl), c
		djnz	loc_EBF5
		dec	d

loc_EBFA:
		jr	nz, loc_EBF2
		pop	hl
		inc	h
		dec	e
		jr	nz, loc_EBF0
		ret
; End of function sub_EBEE

; ---------------------------------------------------------------------------
		db  10h
		db 0FDh	; ¤
		db  3Dh	; =
		db  20h
		db 0F8h	; °
		db  24h	; $
		db  0Dh
		db  20h
		db 0F0h	; Ё
		db 0EDh	; э
		db  7Bh	; {
		db  12h
		db 0ECh	; ь
		db 0FBh	; √
		db 0C9h	; ╔
		db    0
		dw 0

; =============== S U B	R O U T	I N E =======================================


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




; === sub_EC3B === CODE XREF: RAM:EC72p	RAM:EC88p
sub_EC3B:				; CODE XREF: RAM:EC72p	RAM:EC88p
		xor	a
		ret
; End of function sub_EC3B


; =============== S U B	R O U T	I N E =======================================




; === sub_EC3D === CODE XREF: RAM:EC7Dp	RAM:EC93p
sub_EC3D:				; CODE XREF: RAM:EC7Dp	RAM:EC93p
		ld	a, 0FFh
		ret
; End of function sub_EC3D


; =============== S U B	R O U T	I N E =======================================

; Определяем находится ли пилот	в зоне #3C (активных боёв)
; Вх:  ---
; Вых: cy =1 пилот в зоне #3C
;	  =0 пилот в другой галактике галактике
;	     война окончена/не начиналась
;	     пилот в другой зоне
;      hl,de,b,a - не определены

TestPilotInZoneCombat:			; CODE XREF: sub_7FA1+10p sub_7FF0+1Cp ...
		call	TestPilotInZoneWar ; Определяем	находится ли пилот/планета в заданной зоне войны
					; Вх:  (sp) =#00 зона оккупированная таргонами
					;	    =#3C зона линии фронта (активные боевые действия)
					;	    =#78 прифронтовая зона
					; Вых: cy =1 координаты	в пределах проверяемой зоны
					;	  =0 пилот в другой галактике галактике
					;	     война окончена/не начиналась
					;	     координаты	слишком	далеко от левого края
					;      hl,de,b,a - не определены
; ---------------------------------------------------------------------------
		db 3Ch
; ---------------------------------------------------------------------------
		ret	nc
		call	TestPilotInZoneWar ; Определяем	находится ли пилот/планета в заданной зоне войны
					; Вх:  (sp) =#00 зона оккупированная таргонами
					;	    =#3C зона линии фронта (активные боевые действия)
					;	    =#78 прифронтовая зона
					; Вых: cy =1 координаты	в пределах проверяемой зоны
					;	  =0 пилот в другой галактике галактике
					;	     война окончена/не начиналась
					;	     координаты	слишком	далеко от левого края
					;      hl,de,b,a - не определены
; ---------------------------------------------------------------------------
		db 0
; ---------------------------------------------------------------------------
		ccf
		ret
; End of function TestPilotInZoneCombat


; =============== S U B	R O U T	I N E =======================================




; === sub_EC4B === CODE XREF: RAM:EA72p
sub_EC4B:				; CODE XREF: RAM:EA72p
		push	hl
		ld	l, 8Eh ; 'О'
		inc	a
		cp	(hl)
		pop	hl
		ret	nz
		ex	(sp), hl
		ld	hl, loc_EB47
		ex	(sp), hl
		ret
; End of function sub_EC4B

; ---------------------------------------------------------------------------

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




; === sub_ECFD === CODE XREF: PrnPlanetOnSmallMapp
sub_ECFD:				; CODE XREF: PrnPlanetOnSmallMapp
					; sub_ED78p
		ld	c, 61h ; 'a'
		ld	a, (Pilot_CrdXSystem)
		ld	b, a
		ld	a, (DataOfPlanet1+1)
		sub	b
		sla	a
		sla	a
		add	a, c
		add	a, 1
		ld	c, a
		ld	d, 60h ; '`'
		ld	a, (Pilot_CrdYSystem)
		ld	b, a
		ld	a, (DataOfPlanet+1)
		sub	b
		sla	a
		neg
		add	a, d
		sub	15h
		ld	b, a
		push	bc
		call	22AAh
		pop	bc
		ret
; End of function sub_ECFD


; =============== S U B	R O U T	I N E =======================================


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




; === sub_ED60 === CODE XREF: RAM:loc_7502p
sub_ED60:				; CODE XREF: RAM:loc_7502p

; FUNCTION CHUNK AT 96D9 SIZE 00000003 BYTES

		call	PrnExplodeObj	; визуальный эффект,взрыв корабля от энергобомбы?
					; прямая печать	на экран
		call	CopyBuf2Screen
		jp	PrintObjScaner
; End of function sub_ED60


; =============== S U B	R O U T	I N E =======================================

; Attributes: noreturn



; === sub_ED69 === CODE XREF: MoveAllObjects+82p
sub_ED69:				; CODE XREF: MoveAllObjects+82p
		ld	a, xh
		sub	60h ; '`'
		cp	2
		ld	a, (ix+22h)
		ret	c

loc_ED73:				; CODE XREF: sub_ED69:loc_ED73j
		jr	$
; End of function sub_ED69

; ---------------------------------------------------------------------------
		nop
		nop

loc_ED77:				; DATA XREF: sub_ED78+14w
		rlca

; =============== S U B	R O U T	I N E =======================================

; Attributes: noreturn



; === sub_ED78 === CODE XREF: RAM:loc_ECF1p
sub_ED78:				; CODE XREF: RAM:loc_ECF1p
					; DATA XREF: RAM:ECBBo
		call	sub_ECFD
		push	bc
		call	GeneratePlanetName
		ld	bc, 9
		ld	hl, TempName	; "LAVE"
		xor	a
		cpir
		ld	a, c
		xor	7
		inc	a
		ld	(loc_ED77), a
		pop	bc
		ld	a, c
		rra
		rra
		rra
		and	1Fh
		inc	a
		ld	(CrdXPrnSymbol), a
		ld	a, b
		srl	a
		srl	a
		srl	a
		jr	nc, loc_EDA4
		inc	a

loc_EDA4:				; CODE XREF: sub_ED78+29j
		neg
		add	a, 15h
		ld	(CrdXPrnSymbol+1), a
		call	sub_EDC5
; End of function sub_ED78

; ---------------------------------------------------------------------------
		ret	c
		ld	hl,  CrdXPrnSymbol+1
		dec	(hl)
		ld	a, (hl)
		cp	3
		jr	c, loc_EDBC
		call	sub_EDC5
; ---------------------------------------------------------------------------
		db 0D8h	; ╪
; ---------------------------------------------------------------------------

loc_EDBC:				; CODE XREF: RAM:EDB6j
		ld	hl,  CrdXPrnSymbol+1
		inc	(hl)
		inc	(hl)
		ld	a, (hl)
		cp	17h
		ret	nc

; =============== S U B	R O U T	I N E =======================================

; Attributes: noreturn



; === sub_EDC5 === CODE XREF: sub_ED78+33p RAM:EDB8p
sub_EDC5:				; CODE XREF: sub_ED78+33p RAM:EDB8p
		ld	a, (CrdXPrnSymbol+1)
		ld	hl, BufferCrdY
		call	sub_EB3E
		ld	a, (hl)
		and	a
		ret	nz
		ld	a, 0FFh
		ld	(hl), a
		ld	a, 1
		ld	(byte_D0D2), a
		ld	hl, TempName	; "LAVE"
		jp	sub_E311
; End of function sub_EDC5

; ---------------------------------------------------------------------------

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




; === sub_EE9D === CODE XREF: RAM:EE0Ap	RAM:EE2Ep ...
sub_EE9D:				; CODE XREF: RAM:EE0Ap	RAM:EE2Ep ...
		call	PrnMsg_A_CRLF
		jr	loc_EEA5
; End of function sub_EE9D


; =============== S U B	R O U T	I N E =======================================


DubleCRLF:				; CODE XREF: RAM:E987p	RAM:EE1Ep ...
		call	PrnCrd_CRLF

loc_EEA5:				; CODE XREF: sub_EE9D+3j
		jp	PrnCrd_CRLF
; End of function DubleCRLF


; =============== S U B	R O U T	I N E =======================================




; === sub_EEA8 === CODE XREF: RAM:EDFBp
sub_EEA8:				; CODE XREF: RAM:EDFBp
		ld	a, 70h ; 'p'
		call	sub_E292
		ld	a, (PlanetEconomy)
		cp	4
		jr	z, loc_EEC8
		jr	c, loc_EEB8
		sub	5

loc_EEB8:				; CODE XREF: sub_EEA8+Cj
		add	a, 0AAh	; 'к'
		call	PrintMessage1_A

loc_EEBD:				; CODE XREF: sub_EEA8+24j
		ld	a, (PlanetEconomy)
		srl	a
		srl	a
		add	a, 0A8h	; 'и'
		jr	sub_EE9D
; ---------------------------------------------------------------------------

loc_EEC8:				; CODE XREF: sub_EEA8+Aj
		call	PrintSymbol_SP
; ---------------------------------------------------------------------------
		db 0ADh
; ---------------------------------------------------------------------------
		jr	loc_EEBD
; End of function sub_EEA8


; =============== S U B	R O U T	I N E =======================================




; === sub_EECE === CODE XREF: RAM:EE35p
sub_EECE:				; CODE XREF: RAM:EE35p
		ld	a, (PlanetData2)
		rlca
		ld	a, 0BCh	; '╝'
		jr	nc, loc_EED8
		ld	a, 0F7h	; 'ў'

loc_EED8:				; CODE XREF: sub_EECE+6j
		jp	PrintMessage1_A
; End of function sub_EECE


; =============== S U B	R O U T	I N E =======================================

; Установка переменных,	после влёта в систему своим ходом
; Вх:  ---
; Вых: hl,de,b,a - не определены
; Вых: z=1 есть	минеральная жизнь в трюме
;	=0 минеральная жизнь померла
;	   нет минеральной жизни

SetVars4War:				; CODE XREF: sub_5FD5+6Bj
					; InitSpeedAngle+3p
		ld	a, (PilotNumGalaxy)
		and	a
		jr	nz, loc_EEF9
		call	TestPilotInZoneCombat ;	Определяем находится ли	пилот в	зоне #3C (активных боёв)
					; Вх:  ---
					; Вых: cy =1 пилот в зоне #3C
					;	  =0 пилот в другой галактике галактике
					;	     война окончена/не начиналась
					;	     пилот в другой зоне
					;      hl,de,b,a - не определены
		jr	nc, loc_EEED
		xor	a
		ld	(planetTextiles), a
		ld	(planetFireams), a

loc_EEED:				; CODE XREF: SetVars4War+9j
		call	TestPlaneInZoneWar ; Определяем	находится ли пилот/планета в заданной зоне войны
					; Вх:  (sp) =#00 зона оккупированная таргонами
					;	    =#3C зона линии фронта (активные боевые действия)
					;	    =#78 прифронтовая зона
					; Вых: cy =1 координаты	в пределах проверяемой зоны
					;	  =0 пилот в другой галактике галактике
					;	     война окончена/не начиналась
					;	     координаты	слишком	далеко от левого края
					;      hl,de,b,a - не определены
; ---------------------------------------------------------------------------
		db 0
; ---------------------------------------------------------------------------
		ld	a, 4
		jr	c, loc_EEF6
		xor	a

loc_EEF6:				; CODE XREF: SetVars4War+18j
		ld	(PilotMission3), a ; =#06 при стыковке будет захват таргонами (вроде нереальная	ситуация)
					; =#04 в системе со танцией таргонов во	время войны

loc_EEF9:				; CODE XREF: SetVars4War+4j
		call	sub_D5B4	; гибель минеральной жизни при недостаточном количестве
					; Вых: z=1 есть	минеральная жизнь в трюме
					;	=0 минеральная жизнь померла
					;	   нет минеральной жизни
		ret	nz
		ld	(planetMinerals), a
		ret
; End of function SetVars4War


; =============== S U B	R O U T	I N E =======================================

; a - номер объекта



; === sub_EF01 === CODE XREF: SetObjInSlotp
sub_EF01:				; CODE XREF: SetObjInSlotp
		call	sub_7FF0	; a - номер объекта
		ld	hl, AdrDataShips
		ld	a, b
		cp	0Dh
		ret	c
		cp	12h
		ret	nc
		call	IntRnd_A
		cp	50h ; 'P'
		ret	nc
		ld	a, b
		cp	0Fh
		ld	b, 13h		; targon6
		ret	z
		inc	b
		ret	c
		inc	b
		rrca
		ret	nc
		inc	b
		ret
; End of function sub_EF01


; =============== S U B	R O U T	I N E =======================================




; === sub_EF21 === CODE XREF: RAM:EE67p
sub_EF21:				; CODE XREF: RAM:EE67p
		ld	hl, (PlanetData2)
		ld	a, h
		and	0Fh
		add	a, 0Bh
		ld	hl, (PlanetData1)
		ld	l, h
		ld	h, a
		ld	a, 5
		ld	(byte_D0CE), a
		and	a
		jp	PrintNumber_HL
; End of function sub_EF21


; =============== S U B	R O U T	I N E =======================================


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




; === sub_EF48 === CODE XREF: RAM:D012j
sub_EF48:				; CODE XREF: RAM:D012j
		call	sub_E6FA
		call	PrintRamkMenu
		ld	a, 0A7h	; 'з'
		call	sub_E306
		ld	hl, 310h
		ld	(CrdXPrnSymbol), hl
		ld	a, 0FFh
		call	PrnMsg_A_CRLF
		ld	de, unk_FDFB
		call	SetAttrFrom_DE
		ld	a, (InFreeSpace)
		and	a
		ret	nz
		xor	a
		ld	(byte_E1DF), a
		ld	hl, byte_E1BB
		ld	(word_E1E2), hl
		ld	hl, PilotPlanetGov
		ld	(word_E1E4), hl

loc_EF79:				; CODE XREF: sub_EF48+3Bj
		call	sub_E212
		ld	hl, byte_E1DF
		inc	(hl)
		ld	a, (hl)
		cp	11h
		jr	c, loc_EF79
		ret
; End of function sub_EF48

; ---------------------------------------------------------------------------
		db    3
; ---------------------------------------------------------------------------

Cmd_L_PilotInfo:			; CODE XREF: j_Cmd_L_PilotInfoj
					; RAM:E9FDj ...
		ld	a, 0Bh
		ld	(WhatOnScreen),	a
		call	PrintRamkMenu
		call	_PrintIndDanger
		ld	de, unk_FD8E
		call	SetAttrFrom_DE
		call	sub_F90E
		nop
		call	PrintSymbol_SP
; ---------------------------------------------------------------------------
		db 7Eh
; ---------------------------------------------------------------------------
		call	loc_8D38
		nop
		nop
		ld	a, 2
		call	loc_F0A1
		ld	a, 3
		call	loc_F0A1
		ld	a, 6
		call	loc_E2C8
		call	PrintSymbol_SP
; ---------------------------------------------------------------------------
		db 5
; ---------------------------------------------------------------------------
		call	PrintSymbol_SP
; ---------------------------------------------------------------------------
		db 7
; ---------------------------------------------------------------------------
		ld	a, 71h ; 'q'
		call	sub_E292
		call	PrintSymbol_SP
; ---------------------------------------------------------------------------
		db 20h
; ---------------------------------------------------------------------------
		ld	b, 12h
		ld	a, (PilotLegalStatus)
		and	a

loc_EFCB:
		jr	z, loc_EFD3
		cp	32h ; '2'
		jr	c, loc_EFD2
		inc	b

loc_EFD2:				; CODE XREF: RAM:EFCFj
		inc	b

loc_EFD3:				; CODE XREF: RAM:loc_EFCBj
		ld	a, b
		call	PrnMsg_A_CRLF
		ld	a, 72h ; 'r'
		call	sub_E292
		call	PrintSymbol_SP
; ---------------------------------------------------------------------------
		db 20h
; ---------------------------------------------------------------------------
		ld	hl, PilotRating2 ; >=#84 в трюме десант
					; 7,=1 война была или идёт
					; 6,=1 война закончилать
					; 3-1 -	ранг (=текущая стадия войны)
		ld	a, (hl)
		and	a
		jr	nz, loc_EFEF
		dec	hl
		ld	de, byte_F0AE
		ld	b, 15h
		jr	loc_EFF4
; ---------------------------------------------------------------------------

loc_EFEF:				; CODE XREF: RAM:EFE5j
		ld	de, byte_F0B4
		ld	b, 1Ah

loc_EFF4:				; CODE XREF: RAM:EFEDj	RAM:EFFAj
		ld	a, (de)
		cp	(hl)
		jr	nc, loc_EFFC
		inc	b
		inc	de
		jr	loc_EFF4
; ---------------------------------------------------------------------------

loc_EFFC:				; CODE XREF: RAM:EFF6j
		ld	a, b
		call	sub_D755	; печать рейтинга и, если нужно	ранга
		ld	a, 11h
		call	sub_F06F
		ld	bc, 6FEh
		ld	hl,  loc_F098+2

loc_F00B:				; CODE XREF: RAM:F041j
		inc	c
		inc	hl
		push	bc
		push	hl
		ld	l, (hl)
		ld	h, 0D0h	; '╨'
		ld	a, (PilotExtWeapon) ; 7,=1 - ECM Jammer
					; 6,=1 - Cloaking device
					; 5,=1 - нельзя	купить товары(в	трюме беженцы или десант)
					; 4
					; 3 =1 3я миссия была выполнена
					; 2 =1 - третья	миссия выполнена
					; 1 =1 - второя	миссия выполнена
					; 0 =1 - первая	миссия выполнена
		and	c
		rlca
		ld	a, l
		push	af
		ld	a, (hl)
		and	a
		jr	z, loc_F03E
		ld	a, (byte_D08E)	; <>0 есть Cobra Mk4
		and	a
		jr	z, loc_F02C
		ld	b, (hl)
		set	7, b
		ld	a, 30h ; '0'
		sub	b
		call	PrintMessage1_A_Space

loc_F02C:				; CODE XREF: RAM:F021j
		pop	af
		push	af
		jr	nc, loc_F039
		ld	a, 0CFh	; '╧'
		call	PrintMessage2
		call	sub_F072
		xor	a

loc_F039:				; CODE XREF: RAM:F02Ej
		sub	19h
		call	nc, sub_F06F

loc_F03E:				; CODE XREF: RAM:F01Bj
		pop	af
		pop	hl
		pop	bc
		djnz	loc_F00B
		ld	hl, PilotFrontLaser

loc_F046:				; CODE XREF: RAM:F04Fj
		push	bc
		push	hl
		call	sub_F07A
		pop	hl
		pop	bc
		inc	hl
		dec	c
		jr	nz, loc_F046
		ld	a, (PilotAlienComputer)	; 0,=1 есть Alien Computer
		rrca
		ld	a, 0F3h	; 'є'
		call	c, sub_F06F
		ld	a, (PilotExtWeapon) ; 7,=1 - ECM Jammer
					; 6,=1 - Cloaking device
					; 5,=1 - нельзя	купить товары(в	трюме беженцы или десант)
					; 4
					; 3 =1 3я миссия была выполнена
					; 2 =1 - третья	миссия выполнена
					; 1 =1 - второя	миссия выполнена
					; 0 =1 - первая	миссия выполнена
		and	40h ; '@'
		call	sub_F068
; ---------------------------------------------------------------------------
		db 7Dh
; ---------------------------------------------------------------------------
		jp	loc_D646
; ---------------------------------------------------------------------------
		db  46h	; F
		db 0D6h	; ╓

; =============== S U B	R O U T	I N E =======================================




; === sub_F068 === CODE XREF: RAM:F05Fp
sub_F068:				; CODE XREF: RAM:F05Fp
		pop	hl
		ld	b, (hl)
		inc	hl
		push	hl
		and	a
		ret	z
		ld	a, b
; End of function sub_F068


; =============== S U B	R O U T	I N E =======================================




; === sub_F06F === CODE XREF: RAM:F002p	RAM:F03Bp ...
sub_F06F:				; CODE XREF: RAM:F002p	RAM:F03Bp ...
		call	PrintMessage1_A
; End of function sub_F06F


; =============== S U B	R O U T	I N E =======================================




; === sub_F072 === CODE XREF: RAM:F035p
sub_F072:				; CODE XREF: RAM:F035p
		call	PrnCrd_CRLF
		ld	a, 5
		jp	loc_E2C0
; End of function sub_F072


; =============== S U B	R O U T	I N E =======================================




; === sub_F07A === CODE XREF: RAM:F048p
sub_F07A:				; CODE XREF: RAM:F048p
		ld	a, (hl)
		and	a
		ret	z
		push	af
		ld	a, 4
		sub	c
		add	a, 60h ; '`'
		call	PrintMessage1_A_Space
		pop	af
		cp	5
		jr	nc, loc_F096
		ld	b, 72h ; 'r'
		cp	3
		jr	nc, loc_F093
		ld	b, 66h ; 'f'

loc_F093:				; CODE XREF: sub_F07A+15j
		add	a, b
		jr	loc_F098
; ---------------------------------------------------------------------------

loc_F096:				; CODE XREF: sub_F07A+Fj
		ld	a, 0F6h	; 'Ў'

loc_F098:				; CODE XREF: sub_F07A+1Aj
					; DATA XREF: RAM:F008o
		jp	sub_F06F
; End of function sub_F07A

; ---------------------------------------------------------------------------
		db  85h	; Е
		db  88h	; И
		db  8Ah	; К
		db  8Bh	; Л
		db  8Ch	; М
		db  8Dh	; Н
; ---------------------------------------------------------------------------

loc_F0A1:				; CODE XREF: RAM:EFA7p	RAM:EFACp
		ex	af, af'
		call	SetCrdX_SP
; ---------------------------------------------------------------------------
		db 15h
; ---------------------------------------------------------------------------
		call	PrintSymbol_SP
; ---------------------------------------------------------------------------
		db 3Ah
; ---------------------------------------------------------------------------
		ex	af, af'
		jp	PrnMsg_A_CRLF
; ---------------------------------------------------------------------------
byte_F0AE:	db 8			; DATA XREF: RAM:EFE8o
		db 10h
		db 20h
		db 40h
		db 80h
		db 0FFh
byte_F0B4:	db 2			; DATA XREF: RAM:loc_EFEFo
		db 0Ah
		db 19h
		db 0FFh
; ---------------------------------------------------------------------------
; START	OF FUNCTION CHUNK FOR sub_E87E

Cmd_Ent_Inventory:			; CODE XREF: RAM:D018j
					; sub_E6D6:loc_E6EBj ...
		ld	a, 0Ch
		ld	(WhatOnScreen),	a
		call	PrintRamkMenu
		ld	a, 0A4h	; 'д'
		call	sub_E306
		ld	de, unk_FD96
		call	SetAttrFrom_DE
		ld	a, 3
		call	loc_E2C8

loc_F0D0:				; fuel
		call	PrintSymbol_SP
; END OF FUNCTION CHUNK	FOR sub_E87E
; ---------------------------------------------------------------------------
		db 5
; ---------------------------------------------------------------------------
		ld	a, 7		; cash
		call	PrnMsg_A_CRLF
		ld	a, (PilotLargeCargo)
		and	a
		jr	z, loc_F0EB
		ld	hl, 58C1h
		ld	a, 5
		call	FillAttrLineFrom_HL
		call	PrintSymbol_SP
; ---------------------------------------------------------------------------
		db 6Bh
; ---------------------------------------------------------------------------

loc_F0EB:				; CODE XREF: RAM:F0DDj
		call	PrnCrd_CRLF
		ld	a, (PilotExtWeapon) ; 7,=1 - ECM Jammer
					; 6,=1 - Cloaking device
					; 5,=1 - нельзя	купить товары(в	трюме беженцы или десант)
					; 4
					; 3 =1 3я миссия была выполнена
					; 2 =1 - третья	миссия выполнена
					; 1 =1 - второя	миссия выполнена
					; 0 =1 - первая	миссия выполнена

loc_F0F1:
		bit	5, a
		jr	z, loc_F0FD
		ld	a, 0Ch

loc_F0F7:
		call	sub_7A27
		call	PrintMessage2

loc_F0FD:				; CODE XREF: RAM:F0F3j
		xor	a
		ld	(byte_E1DF), a
		ld	hl, PilotRnd
		ld	(word_E1E0), hl

loc_F107:				; CODE XREF: RAM:F114j
		call	loc_E268
		call	nc, PrnCrd_CRLF
		ld	hl, byte_E1DF
		inc	(hl)
		ld	a, (hl)
		cp	11h
		jr	c, loc_F107
		ret
; ---------------------------------------------------------------------------
		db    0
		db    0
		db    0
word_F11A:	dw 0F11Eh		; DATA XREF: PrintMessage1_A:loc_D443r
word_F11C:	dw 0F4D0h		; DATA XREF: PrintMessage2r
		db  6Fh	; o
		db  10h
		db    0
		db  20h
		db  43h	; C
		db  48h	; H
		db  8Ah	; К
		db  54h	; T
		db    0
		db  47h	; G
		db  4Fh	; O
		db  96h	; Ц
		db  52h	; R
		db  4Eh	; N
		db  4Dh	; M
		db  92h	; Т
		db  54h	; T
		db    0
		db    6
		db  44h	; D
		db  91h	; С
		db  41h	; A
		db  10h
		db    3
		db    8
		db    0
		db    6
		db  8Ch	; М
		db  96h	; Ц
		db  4Eh	; N
		db  54h	; T
		db  99h	; Щ
		db  59h	; Y
		db    8
		db    0
		db  53h	; S
		db  59h	; Y
		db  53h	; S
		db  9Ch	; Ь
		db  4Dh	; M
		db    0
		db  50h	; P
		db  9Eh	; Ю
		db  85h	; Е
		db    0
		db    2
		db  20h
		db  8Bh	; Л
		db  52h	; R
		db  4Bh	; K
		db  45h	; E
		db  54h	; T
		db  20h
		db 0A6h	; ж
		db  53h	; S
		db    0
		db  8Ch	; М
		db  44h	; D
		db  88h	; И
		db  54h	; T
		db  9Eh	; Ю
		db  80h	; А
		db    0
		db  41h	; A
		db  47h	; G
		db  9Eh	; Ю
		db  43h	; C
		db  55h	; U
		db  4Ch	; L
		db  54h	; T
		db  55h	; U
		db  94h	; Ф
		db  4Ch	; L
		db    0
		db  9Eh	; Ю
		db  43h	; C
		db  48h	; H
		db  20h
		db    0
		db  41h	; A
		db  96h	; Ц
		db  94h	; Ф
		db  83h	; Г
		db  20h
		db    0
		db  50h	; P
		db  4Fh	; O
		db  99h	; Щ
		db  20h
		db    0
		db  8Bh	; Л
		db  8Ch	; М
		db  4Ch	; L
		db  59h	; Y
		db  20h
		db    0
		db  55h	; U
		db  4Eh	; N
		db  49h	; I
		db  54h	; T
		db    0
		db  20h
		db  56h	; V
		db  49h	; I
		db  45h	; E
		db  57h	; W
		db  20h
		db    0
		db  9Ah	; Ъ
		db  9Bh	; Ы
		db  97h	; Ч
		db  54h	; T
		db  59h	; Y
		db    0
		db  9Bh	; Ы
		db  8Ah	; К
		db  43h	; C
		db  48h	; H
		db  59h	; Y
		db    0
		db  46h	; F
		db  45h	; E
		db  55h	; U
		db  44h	; D
		db  80h	; А
		db    0
		db  4Dh	; M
		db  55h	; U
		db  4Ch	; L
		db  97h	; Ч
		db  2Dh	; -
		db 0A2h	; в
		db    0
		db  8Dh	; Н
		db  43h	; C
		db  54h	; T
		db  91h	; С
		db  99h	; Щ
		db 0B9h	; ╣
		db    0
		db 0FBh	; √
		db  4Dh	; M
		db  55h	; U
		db  4Eh	; N
		db  9Dh	; Э
		db  54h	; T
		db    0
		db  43h	; C
		db  9Fh	; Я
		db  46h	; F
		db  98h	; Ш
		db  90h	; Р
		db  41h	; A
		db  43h	; C
		db  59h	; Y
		db    0
		db  44h	; D
		db  45h	; E
		db  4Dh	; M
		db  4Fh	; O
		db  43h	; C
		db  94h	; Ф
		db  43h	; C
		db  59h	; Y
		db    0
		db  43h	; C
		db  99h	; Щ
		db  50h	; P
		db  99h	; Щ
		db  91h	; С
		db  45h	; E
		db  20h
		db 0CBh	; ╦
		db  91h	; С
		db  45h	; E
		db    0
		db  53h	; S
		db  48h	; H
		db  49h	; I
		db  50h	; P
		db    0
		db  50h	; P
		db 0FEh	; ■
		db  44h	; D
		db  55h	; U
		db  43h	; C
		db  54h	; T
		db    0
		db  20h
		db  95h	; Х
		db  53h	; S
		db  90h	; Р
		db    0
		db  48h	; H
		db  55h	; U
		db  4Dh	; M
		db  9Bh	; Ы
		db  20h
		db  43h	; C
		db  4Fh	; O
		db  4Ch	; L
		db  9Fh	; Я
		db  49h	; I
		db  80h	; А
		db    0
		db  48h	; H
		db  59h	; Y
		db  50h	; P
		db  90h	; Р
		db  53h	; S
		db  50h	; P
		db  41h	; A
		db  85h	; Е
		db    0
		db    6
		db  53h	; S
		db  48h	; H
		db  99h	; Щ
		db  54h	; T
		db  20h
		db 0CAh	; ╩
		db 0A1h	; б
		db    8
		db    0
		db  8Dh	; Н
		db 0CBh	; ╦
		db  9Bh	; Ы
		db  85h	; Е
		db    0
		db 0BDh	; ╜
		db  20h
		db  2Dh	; -
		db  2Dh	; -
		db  20h
		db    3
		db    0
		db 0BDh	; ╜
		db  20h
		db 0CAh	; ╩
		db  20h
		db  3Fh	; ?
		db    0
		db 0F8h	; °
		db  43h	; C
		db  99h	; Щ
		db  49h	; I
		db  4Fh	; O
		db  4Ch	; L
		db  49h	; I
		db  53h	; S
		db  20h
		db  8Ch	; М
		db  20h
		db  44h	; D
		db  41h	; A
		db  4Eh	; N
		db  83h	; Г
		db  52h	; R
		db  2Ah	; *
		db    0
		db  20h
		db  4Ch	; L
		db  49h	; I
		db  47h	; G
		db  48h	; H
		db  54h	; T
		db  20h
		db  59h	; Y
		db  45h	; E
		db  8Ah	; К
		db  53h	; S
		db    0
		db  9Ch	; Ь
		db  43h	; C
		db  48h	; H
		db  2Eh	; .
		db  81h	; Б
		db  96h	; Ц
		db  4Ch	; L
		db    0
		db  43h	; C
		db  41h	; A
		db  53h	; S
		db  48h	; H
		db    0
		db  20h
		db  86h	; Ж
		db  0Eh
		db  49h	; I
		db  9Fh	; Я
		db    0
		db    6
		db  7Ah	; z
		db 0A1h	; б
		db    8
		db    1
		db    0
		db  54h	; T
		db  8Ah	; К
		db  83h	; Г
		db  54h	; T
		db  20h
		db  4Ch	; L
		db  4Fh	; O
		db 0CBh	; ╦
		db    0
		db  6Ah	; j
		db  20h
		db  4Ah	; J
		db  41h	; A
		db  4Dh	; M
		db  4Dh	; M
		db  98h	; Ш
		db    0
		db  52h	; R
		db  9Bh	; Ы
		db  83h	; Г
		db    0
		db  53h	; S
		db  54h	; T
		db    0
		db 0B0h	; ░
		db  20h
		db  4Fh	; O
		db  46h	; F
		db  20h
		db    0
		db  53h	; S
		db  45h	; E
		db  0Eh
		db    0
		db  20h
		db  43h	; C
		db  8Ah	; К
		db  47h	; G
		db  4Fh	; O
		db    0
		db  45h	; E
		db  9Ah	; Ъ
		db  49h	; I
		db  50h	; P
		db    0
		db  46h	; F
		db  4Fh	; O
		db  4Fh	; O
		db  44h	; D
		db    0
		db  9Ch	; Ь
		db  58h	; X
		db  97h	; Ч
		db  4Ch	; L
		db  89h	; Й
		db    0
		db  94h	; Ф
		db  8Dh	; Н
		db  4Fh	; O
		db  41h	; A
		db  43h	; C
		db  97h	; Ч
		db  96h	; Ц
		db  53h	; S
		db    0
		db  53h	; S
		db  95h	; Х
		db  96h	; Ц
		db  53h	; S
		db    0
		db  4Ch	; L
		db  49h	; I
		db  9Ah	; Ъ
		db  99h	; Щ
		db  2Fh	; /
		db  57h	; W
		db  8Ch	; М
		db  89h	; Й
		db    0
		db  4Ch	; L
		db  55h	; U
		db  58h	; X
		db  55h	; U
		db  9Eh	; Ю
		db  89h	; Й
		db    0
		db  4Eh	; N
		db  8Ah	; К
		db  43h	; C
		db  4Fh	; O
		db  97h	; Ч
		db  43h	; C
		db  53h	; S
		db    0
		db 0EEh	; ю
		db  53h	; S
		db    0
		db  8Bh	; Л
		db  43h	; C
		db  48h	; H
		db  8Ch	; М
		db  90h	; Р
		db  59h	; Y
		db    0
		db  41h	; A
		db  0Eh
		db  4Fh	; O
		db  59h	; Y
		db  53h	; S
		db    0
		db  46h	; F
		db  49h	; I
		db  8Eh	; О
		db  8Ah	; К
		db  4Dh	; M
		db  53h	; S
		db    0
		db  46h	; F
		db  55h	; U
		db  52h	; R
		db  53h	; S
		db    0
		db  4Dh	; M
		db  8Ch	; М
		db  90h	; Р
		db  80h	; А
		db  53h	; S
		db    0
		db  47h	; G
		db  4Fh	; O
		db  4Ch	; L
		db  44h	; D
		db    0
		db  50h	; P
		db  4Ch	; L
		db  91h	; С
		db  8Ch	; М
		db  55h	; U
		db  4Dh	; M
		db    0
		db  83h	; Г
		db  4Dh	; M
		db  2Dh	; -
		db 0CBh	; ╦
		db  9Fh	; Я
		db  89h	; Й
		db    0
		db 0F2h	; Є
		db  7Fh	; 
		db  53h	; S
		db    0
		db  20h
		db  4Eh	; N
		db  4Fh	; O
		db  54h	; T
		db  20h
		db  46h	; F
		db  4Fh	; O
		db  55h	; U
		db  4Eh	; N
		db  44h	; D
		db    0
		db  20h
		db  43h	; C
		db  52h	; R
		db    0
		db  4Ch	; L
		db  8Ah	; К
		db  83h	; Г
		db    0
		db    0
		db    0
		db    0
		db    0
		db    0
		db    0
		db    0
		db    0
		db  44h	; D
		db  89h	; Й
		db  54h	; T
		db 0FEh	; ■
		db  59h	; Y
		db    0
		db  20h
		db  9Bh	; Ы
		db  44h	; D
		db  20h
		db    0
		db 0FBh	; √
		db  50h	; P
		db  55h	; U
		db  54h	; T
		db  90h	; Р
		db    0
		db  20h
		db  8Ch	; М
		db  56h	; V
		db  41h	; A
		db  44h	; D
		db  98h	; Ш
		db  2Ah	; *
		db    0
		db 0F8h	; °
		db  7Ah	; z
		db  20h
		db  31h	; 1
		db 0EFh	; я
		db    0
		db  94h	; Ф
		db  4Eh	; N
		db  4Bh	; K
		db  3Ah	; :
		db  20h
		db    0
		db  80h	; А
		db  49h	; I
		db  92h	; Т
		db  20h
		db    0
		db 0F2h	; Є
		db 0EEh	; ю
		db    0
		db  44h	; D
		db  4Fh	; O
		db  20h
		db  59h	; Y
		db  4Fh	; O
		db  55h	; U
		db  20h
		db  57h	; W
		db  9Bh	; Ы
		db  54h	; T
		db  20h
		db  54h	; T
		db  4Fh	; O
		db  20h
		db  42h	; B
		db  55h	; U
		db  59h	; Y
		db  20h
		db 0F9h	; ∙
		db  20h
		db  3Fh	; ?
		db    0
		db  53h	; S
		db  55h	; U
		db  50h	; P
		db  90h	; Р
		db    0
		db 0F5h	; ї
		db 0BBh	; ╗
		db    0
		db  48h	; H
		db  55h	; U
		db  4Dh	; M
		db  9Bh	; Ы
		db  4Fh	; O
		db  49h	; I
		db  44h	; D
		db    0
		db    6
		db  4Dh	; M
		db  41h	; A
		db  59h	; Y
		db  44h	; D
		db  41h	; A
		db  59h	; Y
		db    8
		db  2Ah	; *
		db  20h
		db    0
		db    6
		db  43h	; C
		db  4Fh	; O
		db  42h	; B
		db  94h	; Ф
		db  20h
		db  4Dh	; M
		db  4Bh	; K
		db  34h	; 4
		db    8
		db    0
		db 0ABh	; л
		db  94h	; Ф
		db  8Dh	; Н
		db  88h	; И
		db    0
		db  43h	; C
		db  4Fh	; O
		db  4Dh	; M
		db    0
		db 0FBh	; √
		db  4Dh	; M
		db  9Bh	; Ы
		db  44h	; D
		db  90h	; Р
		db    0
		db  20h
		db 0ECh	; ь
		db  98h	; Ш
		db    0
		db  52h	; R
		db  4Fh	; O
		db    0
		db    6
		db 0AEh	; о
		db  20h
		db  20h
		db 0B0h	; ░
		db    9
		db 0BAh	; ║
		db  20h
		db  20h
		db  20h
		db  20h
		db 0AEh	; о
		db  20h
		db 0A6h	; ж
		db  20h
		db  46h	; F
		db  99h	; Щ
		db  20h
		db  53h	; S
		db  41h	; A
		db  81h	; Б
		db    8
		db    0
		db  46h	; F
		db  52h	; R
		db  9Fh	; Я
		db  54h	; T
		db    0
		db  8Eh	; О
		db  8Ah	; К
		db    0
		db  81h	; Б
		db  46h	; F
		db  54h	; T
		db    0
		db  9Eh	; Ю
		db  47h	; G
		db  48h	; H
		db  54h	; T
		db    0
		db  79h	; y
		db  4Ch	; L
		db  4Fh	; O
		db  57h	; W
		db    0
		db  63h	; c
		db  10h
		db 0FCh	; №
		db  2Ah	; *
		db    0
		db  45h	; E
		db  58h	; X
		db  54h	; T
		db  94h	; Ф
		db  20h
		db    0
		db  50h	; P
		db  55h	; U
		db  4Ch	; L
		db  53h	; S
		db  45h	; E
		db 0BBh	; ╗
		db    0
		db  93h	; У
		db  41h	; A
		db  4Dh	; M
		db 0BBh	; ╗
		db    0
		db  46h	; F
		db  55h	; U
		db  45h	; E
		db  4Ch	; L
		db    0
		db  4Dh	; M
		db  9Dh	; Э
		db  53h	; S
		db  49h	; I
		db  81h	; Б
		db    0
		db 0E3h	; у
		db 0CEh	; ╬
		db  20h
		db  42h	; B
		db  41h	; A
		db  59h	; Y
		db    0
		db  45h	; E
		db  2Eh	; .
		db  43h	; C
		db  2Eh	; .
		db  4Dh	; M
		db  2Eh	; .
		db 0A5h	; е
		db    0
		db  66h	; f
		db  67h	; g
		db  53h	; S
		db    0
		db  66h	; f
		db  68h	; h
		db  53h	; S
		db    0
		db  69h	; i
		db  20h
		db  53h	; S
		db  43h	; C
		db  4Fh	; O
		db  4Fh	; O
		db  50h	; P
		db  53h	; S
		db    0
		db  89h	; Й
		db  43h	; C
		db  41h	; A
		db  50h	; P
		db  45h	; E
		db  20h
		db  50h	; P
		db  4Fh	; O
		db  44h	; D
		db    0
		db  79h	; y
		db  42h	; B
		db  4Fh	; O
		db  4Dh	; M
		db  42h	; B
		db    0
		db  79h	; y
		db 0AEh	; о
		db    0
		db  44h	; D
		db  4Fh	; O
		db  43h	; C
		db  4Bh	; K
		db  8Ch	; М
		db  47h	; G
		db  20h
		db 0EEh	; ю
		db    0
		db  7Ah	; z
		db  20h
		db 0BDh	; ╜
		db    0
		db  4Dh	; M
		db  49h	; I
		db  4Ch	; L
		db  49h	; I
		db  54h	; T
		db  8Ah	; К
		db  59h	; Y
		db 0BBh	; ╗
		db    0
		db  4Dh	; M
		db  8Ch	; М
		db  8Ch	; М
		db  47h	; G
		db 0BBh	; ╗
		db    0
		db    9
		db    9
		db    9
		db    0
		db  8Ch	; М
		db 0FBh	; √
		db  8Ch	; М
		db  47h	; G
		db  20h
		db  6Ah	; j
		db    0
		db  92h	; Т
		db  90h	; Р
		db  47h	; G
		db  59h	; Y
		db  20h
		db    0
		db  47h	; G
		db  41h	; A
		db  95h	; Х
		db  43h	; C
		db  97h	; Ч
		db  43h	; C
		db    0
		db  73h	; s
		db  20h
		db  4Fh	; O
		db  4Eh	; N
		db    0
		db  41h	; A
		db  0Eh
		db    0
		db  43h	; C
		db  4Ch	; L
		db  4Fh	; O
		db  41h	; A
		db  4Bh	; K
		db  8Ch	; М
		db  47h	; G
		db  20h
		db  44h	; D
		db  45h	; E
		db  56h	; V
		db  49h	; I
		db  85h	; Е
		db    0
		db    6
		db 0FCh	; №
		db  20h
		db    4
		db    8
		db    9
		db    9
		db  50h	; P
		db  8Eh	; О
		db  53h	; S
		db  92h	; Т
		db  54h	; T
		db  20h
		db 0A5h	; е
		db    9
		db 0BDh	; ╜
		db  20h
		db 0A5h	; е
		db    9
		db    0
		db  49h	; I
		db  9Ch	; Ь
		db  4Dh	; M
		db    0
		db  4Ch	; L
		db  4Ch	; L
		db    0
		db 0F8h	; °
		db  0Bh
		db 0EFh	; я
		db    0
		db  20h
		db  9Fh	; Я
		db  20h
		db    0
		db    6
		db 0CFh	; ╧
		db  4Dh	; M
		db  92h	; Т
		db  54h	; T
		db    8
		db  3Ah	; :
		db    0
		db  43h	; C
		db  81h	; Б
		db  9Bh	; Ы
		db    0
		db  4Fh	; O
		db  46h	; F
		db  46h	; F
		db  92h	; Т
		db  44h	; D
		db  90h	; Р
		db    0
		db  46h	; F
		db  55h	; U
		db  47h	; G
		db  49h	; I
		db  97h	; Ч
		db  96h	; Ц
		db    0
		db  48h	; H
		db  8Ah	; К
		db  4Dh	; M
		db  81h	; Б
		db  53h	; S
		db  53h	; S
		db    0
		db  4Dh	; M
		db  4Fh	; O
		db 0CBh	; ╦
		db  4Ch	; L
		db  59h	; Y
		db  20h
		db  15h
		db    0
		db 0ACh	; м
		db    0
		db 0ABh	; л
		db    0
		db  41h	; A
		db  42h	; B
		db  4Fh	; O
		db  96h	; Ц
		db  20h
		db 0ABh	; л
		db    0
		db 0FBh	; √
		db  50h	; P
		db  45h	; E
		db  54h	; T
		db  92h	; Т
		db  54h	; T
		db    0
		db  44h	; D
		db  9Bh	; Ы
		db  83h	; Г
		db 0FEh	; ■
		db  88h	; И
		db    0
		db  44h	; D
		db  45h	; E
		db  41h	; A
		db  44h	; D
		db  4Ch	; L
		db  59h	; Y
		db    0
		db  2Dh	; -
		db  2Dh	; -
		db  2Dh	; -
		db  2Dh	; -
		db  20h
		db  45h	; E
		db  20h
		db  4Ch	; L
		db  20h
		db  49h	; I
		db  20h
		db  54h	; T
		db  20h
		db  45h	; E
		db  20h
		db  2Dh	; -
		db  2Dh	; -
		db  2Dh	; -
		db  2Dh	; -
		db    0
		db  50h	; P
		db  8Eh	; О
		db  53h	; S
		db  92h	; Т
		db  54h	; T
		db    0
		db  69h	; i
		db  20h
		db  81h	; Б
		db  41h	; A
		db  4Bh	; K
		db  2Ah	; *
		db    0
		db    0
		db    0
		db    0
		db    0
		db 0B0h	; ░
		db    3
		db 0CAh	; ╩
		db  6Eh	; n
		db 0B1h	; ▒
		db    0
		db    0
		db  50h	; P
		db 0F2h	; Є
		db  53h	; S
		db  53h	; S
		db  20h
		db 0D1h	; ╤
		db  41h	; A
		db 0E9h	; щ
		db  2Ch	; ,
		db  20h
		db  9Ah	; Ъ
		db    0
		db    0
		db  20h
		db  47h	; G
		db  41h	; A
		db  4Dh	; M
		db  45h	; E
		db  20h
		db  4Fh	; O
		db 0FAh	; ·
		db  52h	; R
		db    0
		db 0B0h	; ░
		db  20h
		db 0C1h	; ┴
		db  53h	; S
		db  20h
		db  4Fh	; O
		db  46h	; F
		db  20h
		db  13h
		db  10h
		db  20h
		db  41h	; A
		db  57h	; W
		db 0EEh	; ю
		db  44h	; D
		db  20h
		db 0B3h	; │
		db  20h
		db 0E2h	; т
		db  45h	; E
		db  49h	; I
		db  52h	; R
		db  20h
		db  48h	; H
		db  49h	; I
		db  47h	; G
		db  48h	; H
		db  45h	; E
		db 0DEh	; ▐
		db  20h
		db  48h	; H
		db 0DFh	; ▀
		db 0D9h	; ┘
		db  52h	; R
		db  20h
		db  2Dh	; -
		db  0Ch
		db  93h	; У
		db  12h
		db  2Dh	; -
		db  12h
		db 0B1h	; ▒
		db 0D3h
		db    1
		db 4Ch
		db 49h
		db 0FAh
		db  20h
		db 0E0h	; р
		db  4Eh	; N
		db  47h	; G
		db 0B2h	; ▓
		db  50h	; P
		db  52h	; R
		db  4Fh	; O
		db 0D1h	; ╤
		db 0F4h	; Ї
		db  2Ah	; *
		db    2
		db    0
unk_F52E:	db    2			; CODE XREF: RAM:F553j
		db  53h	; S
		db  48h	; H
		db  49h	; I
		db  50h	; P
		db  20h
		db  42h	; B
		db  4Fh	; O
		db 0EEh	; ю
		db  44h	; D
		db 0C4h	; ─
		db 0A6h	; ж
		db    1
		db    0
		db 0A8h	; и
		db    0
		db  20h
		db  50h	; P
		db  49h	; I
		db  52h	; R
unk_F542:	db 0F5h	; ї		; CODE XREF: RAM:F55Ej
		db  45h	; E
		db    0
		db  15h
		db  91h	; С
		db  20h
		db 0C8h	; ╚
		db  20h
		db    0
word_F54B:	dw 0B302h
; ---------------------------------------------------------------------------
		jr	nz, unk_F597
		ld	b, c

loc_F550:
		jp	m, 44CBh
		jr	nz, unk_F52E
		ld	d, d
		jr	nz, unk_F5A4
		ld	c, c
		jp	m, 2A53h
		out	(18h), a
		jr	unk_F542
; ---------------------------------------------------------------------------
		db 0FFh
		db  4Bh	; K
		db  20h
		db 0B3h	; │
		db  2Ch	; ,
		db  20h
		db  9Ah	; Ъ
		db  2Ah	; *
		db    0
		db    0
		db    0
		db    0
		db    0
		db    0
		db    0
		db    0
		db    0
		db    0
		db    0
		db    0
		db    0
		db    0
		db    0
		db    0
		db    0
		db    0
		db    0
		db    0
		db    0
		db    0
		db    0
		db    0
		db    0
		db    0
		db    0
		db    0
		db    0
		db    0
		db    0
		db    0
		db    0
		db    0
		db    0
		db    0
		db    0
		db    0
		db    0
		db    0
		db    0
		db    0
		db    0
		db    0
		db    0
		db    0
		db    0
unk_F597:	db    0			; CODE XREF: RAM:F54Dj
		db    0
		db    0
		db    0
		db    0
		db    0
		db    0
		db    0
		db    0
		db    0
		db    0
		db    0
		db    0
unk_F5A4:	db    0			; CODE XREF: RAM:F556j
		db    0
		db    0
		db    0
		db    0
		db    0
		db    0
		db  4Dh	; M
		db 0F0h	; Ё
		db 0F4h	; Ї
		db 0E4h	; ф
		db  20h
		db  4Ch	; L
		db  49h	; I
		db  46h	; F
		db  45h	; E
		db  20h
		db    0
		db    0
		db    0
		db    0
		db 0B0h	; ░
		db    1
		db    3
		db 0CAh	; ╩
		db 0BDh	; ╜
		db  0Ch
		db 0A6h	; ж
		db 0D4h	; ╘
		db  53h	; S
		db  2Ah	; *
		db    2
		db  0Fh
		db    0
		db 0B0h	; ░
		db    1
		db    3
		db 0CAh	; ╩
		db 0A9h	; й
		db  0Ch
		db  41h	; A
		db 0D4h	; ╘
		db  20h
		db 0A7h	; з
		db  2Ah	; *
		db    2
		db  0Fh
		db    0
		db  97h	; Ч
		db 0D1h	; ╤
		db  45h	; E
		db  43h	; C
		db  49h	; I
		db 0E4h	; ф
		db 0C5h	; ┼
		db 0B4h	; ┤
		db    0
		db 0B0h	; ░
		db  93h	; У
		db 0D4h	; ╘
		db  53h	; S
		db 0C2h	; ┬
		db 0ECh	; ь
		db 0C3h	; ├
		db  93h	; У
		db  0Ch
		db 0C0h	; └
		db 0F5h	; ї
		db  20h
		db  13h
		db  10h
		db  20h
		db  41h	; A
		db  53h	; S
		db 0D0h	; ╨
		db  42h	; B
		db  41h	; A
		db 0DAh	; ┌
		db  92h	; Т
		db 0E2h	; т
		db  45h	; E
		db  49h	; I
		db  52h	; R
		db  20h
		db 0A7h	; з
		db 0BBh	; ╗
		db 0AEh	; о
		db  93h	; У
		db 0C0h	; └
		db  0Ch
		db 0F5h	; ї
		db  20h
		db  10h
		db 0CCh	; ╠
		db 0D3h	; ╙
		db  0Dh
		db  9Ch	; Ь
		db  20h
		db  94h	; Ф
		db 0CFh	; ╧
		db 0D6h	; ╓
		db    0
		db 0E2h	; т
		db  45h	; E
		db  59h	; Y
		db  20h
		db  53h	; S
		db  48h	; H
		db  4Fh	; O
		db  57h	; W
		db  20h
		db 0E3h	; у
		db  20h
		db  4Dh	; M
		db 0F4h	; Ї
		db  43h	; C
		db  59h	; Y
		db    0
		db    2
		db  93h	; У
		db  53h	; S
		db  55h	; U
		db  4Eh	; N
		db 0CAh	; ╩
		db  47h	; G
		db  4Fh	; O
		db 0C3h	; ├
		db 0E3h	; у
		db  56h	; V
		db  41h	; A
		db  2Ah	; *
		db  9Dh	; Э
		db    0
		db  13h
		db    0
		db  53h	; S
		db    2
		db  2Ah	; *
		db  18h
		db  18h
		db    0
		db    0
		db    0
		db    0
		db    0
		db    0
		db    0
		db    0
		db    0
		db    0
		db    0
		db    0
		db    0
		db    0
		db    0
		db    0
		db    0
		db    0
		db  47h	; G
		db  52h	; R
		db  4Fh	; O
		db  53h	; S
		db  53h	; S
		db  20h
		db    6
		db 0BAh	; ║
		db  0Ah
		db  49h	; I
		db  56h	; V
		db 0DBh	; █
		db  59h	; Y
		db    0
		db  45h	; E
		db  43h	; C
		db  4Fh	; O
		db 0E3h	; у
		db  4Dh	; M
		db  59h	; Y
		db    0
		db 0E5h	; х
		db  47h	; G
		db 0E4h	; ф
		db  20h
		db 0DEh	; ▐
		db 0F5h	; ї
		db 0ECh	; ь
		db    0
		db 0F8h	; °
		db 0FBh	; √
		db  4Eh	; N
		db  47h	; G
		db    0
		db    0
		db    0
		db    0
		db    0
		db    0
		db    0
		db    0
		db    0
		db    0
		db    0
		db    0
		db    0
		db    0
		db    0
		db    0
		db    0
		db    0
		db    0
		db    0
		db    0
		db    0
		db    0
		db    0
		db    0
		db    0
		db    0
		db    0
		db    0
		db    0
		db    0
		db  50h	; P
		db  4Ch	; L
		db 0FFh
		db 0DDh	; ▌
		db    0
		db  20h
		db  46h	; F
		db 0FDh	; ¤
		db  20h
		db    0
		db 0E2h	; т
		db  45h	; E
		db  20h
		db    0
		db 0E2h	; т
		db  98h	; Ш
		db  20h
		db    0
		db    0
		db    9
		db  0Bh
		db    1
		db    8
		db    0
		db    1
		db  1Ah
		db  57h	; W
		db 0FFh
		db  54h	; T
		db 0C4h	; ─
		db  0Ch
		db 0D3h	; ╙
		db  20h
		db  20h
		db  20h
		db  20h
		db 0D0h	; ╨
		db  42h	; B
		db 0F8h	; °
		db 0FAh	; ·
		db  20h
		db  9Ah	; Ъ
		db  20h
		db 0D3h	; ╙
		db  20h
		db  20h
		db  92h	; Т
		db  41h	; A
		db  20h
		db    0
		db  49h	; I
		db  53h	; S
		db    0
		db  49h	; I
		db 0FFh
		db    0
		db  13h
		db    6
		db 0FCh	; №
		db  0Ah
		db    0
		db    0
		db  54h	; T
		db  41h	; A
		db  4Bh	; K
		db  45h	; E
		db    0
		db  9Fh	; Я
		db 0CBh	; ╦
		db 0A8h	; и
		db 0CEh	; ╬
		db    0
		db  9Fh	; Я
		db  20h
		db  9Ch	; Ь
		db 0BCh	; ╝
		db 0CEh	; ╬
		db    0
		db 0D3h	; ╙
		db  57h	; W
		db 0DCh	; ▄
		db  4Ch	; L
		db  20h
		db 0B3h	; │
		db    0
		db  4Ch	; L
		db  49h	; I
		db  45h	; E
		db  55h	; U
		db  54h	; T
		db  45h	; E
		dw 0FF4Eh
		db  54h	; T
		db    0
		db  43h	; C
		db  41h	; A
		db  50h	; P
		db  54h	; T
		db  41h	; A
		db 0F0h	; Ё
		db    0
		db 0EFh	; я
		db  4Ah	; J
		db 0FDh	; ¤
		db    0
		db  43h	; C
		db  4Fh	; O
		db  4Ch	; L
		db 0DFh	; ▀
		db  45h	; E
		db  4Ch	; L
		db    0
		db 0E7h	; ч
		db  4Eh	; N
		db 0F4h	; Ї
		db 0E4h	; ф
		db    0
		db  20h
		db  28h	; (
		db  0Dh
		db 0F0h	; Ё
		db  20h
		db 0F2h	; Є
		db 0DAh	; ┌
		db  52h	; R
		db 0FAh	; ·
		db  29h	; )
		db    0
		db  42h	; B
		db  59h	; Y
		db  20h
		db    0
		db 0F0h	; Ё
		db  56h	; V
		db  41h	; A
		db  53h	; S
		db  49h	; I
		db 0DFh	; ▀
		db  20h
		db    0
		db  20h
		db 0F2h	; Є
		db  46h	; F
		db  55h	; U
		db 0E7h	; ч
		db 0EDh	; э
		db  20h
		db    0
		db 0F2h	; Є
		db  53h	; S
		db  49h	; I
		db 0DEh	; ▐
		db 0C3h	; ├
		db    0
		db    0
		db    0
		db  97h	; Ч
		db  46h	; F
		db  49h	; I
		db  47h	; G
		db  48h	; H
		db  54h	; T
		db 0C3h	; ├
		db  53h	; S
		db 0F4h	; Ї
		db  56h	; V
		db  49h	; I
		db 0E9h	; щ
		db 0B4h	; ┤
		db    0
		db 0BAh	; ║
		db 0A0h	; а
		db 0BBh	; ╗
		db 0CBh	; ╦
		db 0A8h	; и
		db  20h
		db  46h	; F
		db  52h	; R
		db  4Fh	; O
		db  4Dh	; M
		db  20h
		db  0Ch
		db  93h	; У
		db 0A9h	; й
		db  91h	; С
		db  53h	; S
		db 0D6h	; ╓
		db    0
		db    6
		db 0ECh	; ь
		db  0Ah
		db  20h
		db    0
		db 0DBh	; █
		db  53h	; S
		db  20h
		db    0
		db  0Dh
		db  13h
		db    0
		db  2Eh	; .
		db  0Ch
		db    0
		db  20h
		db 0FFh
		db  44h	; D
		db  20h
		db    0
		db  59h	; Y
		db 0D9h	; ┘
		db    0
		db    2
		db 0D3h	; ╙
		db 0D3h	; ╙
		db  0Dh
		db  13h
		db 0C2h	; ┬
		db 0B3h	; │
		db  20h
		db 0F0h	; Ё
		db  54h	; T
		db  45h	; E
		db 0F2h	; Є
		db 0DEh	; ▐
		db 0C4h	; ─
		db 0CEh	; ╬
		db    0
		db 0BAh	; ║
		db 0A1h	; б
		db 0BBh	; ╗
		db  50h	; P
		db  52h	; R
		db  4Fh	; O
		db  56h	; V
		db  49h	; I
		db  44h	; D
		db  45h	; E
		db  20h
		db 0A9h	; й
		db  91h	; С
		db  53h	; S
		db  0Ch
		db  57h	; W
		db  49h	; I
		db  54h	; T
		db  48h	; H
		db  20h
		db    6
		db 0DAh	; ┌
		db 0EDh	; э
		db 0D0h	; ╨
		db  0Ah
		db 0D6h	; ╓
		db    0
		db 0BAh	; ║
		db 0A2h	; в
		db 0BBh	; ╗
		db  54h	; T
		db 0F8h	; °
		db  4Eh	; N
		db 0D1h	; ╤
		db 0FDh	; ¤
		db  54h	; T
		db  20h
		db 0BCh	; ╝
		db 0C9h	; ╔
		db  93h	; У
		db  0Ch
		db 0BDh	; ╜
		db  91h	; С
		db  53h	; S
		db 0D6h	; ╓
		db    0
		db 0BAh	; ║
		db 0A3h	; г
		db 0BBh	; ╗
		db 0DAh	; ┌
		db 0EEh	; ю
		db  43h	; C
		db  48h	; H
		db 0B2h	; ▓
		db 0AEh	; о
		db  48h	; H
		db  49h	; I
		db  44h	; D
		db  44h	; D
		db 0F6h	; Ў
		db  20h
		db 0D4h	; ╘
		db  27h	; '
		db  53h	; S
		db  20h
		db  42h	; B
		db  41h	; A
		db 0DAh	; ┌
		db  20h
		db  4Eh	; N
		db  45h	; E
		db 0EEh	; ю
		db 0A6h	; ж
		db  10h
		db 0D6h	; ╓
		db    0


; === sub_F8D7 === CODE XREF: PrnExplodeObj+47p
sub_F8D7:				; CODE XREF: PrnExplodeObj+47p
		call	loc_8F74
		call	PrnCircle_Fill
		call	CopyBuf2Screen
		ld	a, (CircleRadius)
		ret
; End of function sub_F8D7


; =============== S U B	R O U T	I N E =======================================




; === sub_F8E4 === CODE XREF: sub_9D40+123j
sub_F8E4:				; CODE XREF: sub_9D40+123j
		jp	nz, loc_9D38
		ld	(loc_F909+1), sp
		di
		ld	hl, 40F0h
		ld	de, unk_FFD0
		ld	bc, 30h	; '0'
		ldir
		ld	sp, 4120h
		call	MoveAllObjAI
		di
		ld	hl, unk_FFD0
		ld	bc, 30h	; '0'
		ld	de, 40F0h
		ldir

loc_F909:				; DATA XREF: sub_F8E4+3w
		ld	sp, 411Ah
		ei
		ret
; End of function sub_F8E4


; =============== S U B	R O U T	I N E =======================================




; === sub_F90E === CODE XREF: RAM:EF98p
sub_F90E:				; CODE XREF: RAM:EF98p
		ld	hl, 206h
		ld	(CrdXPrnSymbol), hl
		ret
; End of function sub_F90E

; ---------------------------------------------------------------------------
		db    0

; =============== S U B	R O U T	I N E =======================================




; === sub_F916 === CODE XREF: sub_9D40+4p
sub_F916:				; CODE XREF: sub_9D40+4p
		ld	(FlgObjNear), a
		inc	(hl)
		dec	(hl)
		set	7, (hl)
		jr	z, loc_F927
		bit	6, (hl)
		ret	z
		bit	1, (ix+26h)
		ret	z

loc_F927:				; CODE XREF: sub_F916+7j
		push	af
		call	sub_7A9E	; hl - адрес допслота
		pop	af
		ret
; End of function sub_F916


; =============== S U B	R O U T	I N E =======================================


SetCrdCDEinExtSlot:			; CODE XREF: sub_7E02+5p sub_7E02+1Dp	...
		push	af
		ld	hl, (word_B452)
		inc	hl
		ld	(hl), c
		inc	hl
		ld	(hl), d
		inc	hl
		ld	(hl), e
		inc	hl
		ld	(hl), c
		inc	hl
		ld	a, d
		xor	1
		ld	(hl), a
		inc	hl
		ld	a, e
		xor	1
		ld	(hl), a
		pop	af
		ret
; End of function SetCrdCDEinExtSlot


; =============== S U B	R O U T	I N E =======================================

; получение координат и	знаков проекции	объекта

GetCrdsProjection:			; CODE XREF: sub_7E02:objDestroyedp
					; sub_7E02+1Ap	...
		ld	de, (CrdXProjection)
		ld	a, d
		ld	d, e
		ld	e, a
		ld	a, (WhatOnScreen)
		bit	0, a
		ld	a, (CrdXYsign)
		jr	z, loc_F958
		xor	2

loc_F958:				; CODE XREF: GetCrdsProjection+Fj
		ld	c, a
		ret
; End of function GetCrdsProjection


; =============== S U B	R O U T	I N E =======================================

; проверка есть	ли ракета нацеленная на	текущий	объект



; === sub_F95A === CODE XREF: ReactionObject+FEp
sub_F95A:				; CODE XREF: ReactionObject+FEp
		ld	ix, objMissile
		ld	a, (InverseNumSlotIX)
		ld	b, 7

loc_F963:				; CODE XREF: sub_F95A+20j
		bit	0, (ix+26h)
		jr	z, loc_F975
		ld	d, (ix+22h)
		inc	d
		dec	d
		jr	z, loc_F975
		cp	(ix+25h)
		scf
		ret	z

loc_F975:				; CODE XREF: sub_F95A+Dj sub_F95A+14j
		ld	de, 0FFD9h
		add	ix, de
		djnz	loc_F963
		and	a
		ret
; End of function sub_F95A


; =============== S U B	R O U T	I N E =======================================




; === sub_F97E === CODE XREF: MoveMissile+50p
sub_F97E:				; CODE XREF: MoveMissile+50p
		bit	0, (ix+26h)
		jp	nz, sub_A537
		push	hl
		ld	hl, (word_B452)
		ld	a, (hl)
		pop	hl
		and	0D0h ; '╨'
		xor	0D0h ; '╨'
		ret	nz
		ld	a, (iy+26h)
		cp	0Ch
		jr	z, loc_F9A0
		cpl
		and	2
		ret	nz
		ld	(iy+22h), a
		inc	a
		ret
; ---------------------------------------------------------------------------

loc_F9A0:				; CODE XREF: sub_F97E+17j
		xor	a
		ld	(ix+22h), a
		inc	a
		ret
; End of function sub_F97E

; ---------------------------------------------------------------------------
		db  3Ch	; <
		db 0C9h	; ╔

; =============== S U B	R O U T	I N E =======================================




; === sub_F9A8 === CODE XREF: PrintStars+6p
sub_F9A8:				; CODE XREF: PrintStars+6p
		ld	(StarsFastScroll), a
; End of function sub_F9A8


; =============== S U B	R O U T	I N E =======================================




; === sub_F9AB === CODE XREF: sub_9381+1p
sub_F9AB:				; CODE XREF: sub_9381+1p
					; InitSpeedAngle2+2p
		ld	hl, PlayerAggressor
		ld	(hl), 0
		ld	a, (PilotLegalStatus)
		cp	38h ; '8'
		jr	c, loc_F9B9	; очистка доп слотов
		set	2, (hl)

loc_F9B9:				; CODE XREF: sub_F9AB+Aj
		inc	hl		; очистка доп слотов
		ld	de,  byte_B457+1 ; 5,+0	=0 - объектом был запущен корабль
					; +1,+2,+3 - печатные координаты вершины с лазером
					; +1,+2,+3 - печатные координаты вершины №3 xor	3
					; +8 биты 0-3,=0 можно увеличить рейтинг
		ld	bc, 4Fh	; 'O'
		ld	(hl), b
		ldir
		ld	b, 5
		ld	hl, AngleLeftRight
		ret
; End of function sub_F9AB


; =============== S U B	R O U T	I N E =======================================

; установка агрессии при попадании из лазера

SetAgrFireLaser:			; CODE XREF: sub_9D40:loc_9D94p
					; sub_F9E1p
		ld	a, (PilotMission3) ; =#06 при стыковке будет захват таргонами (вроде нереальная	ситуация)
					; =#04 в системе со танцией таргонов во	время войны
		cp	4
		jr	z, loc_F9DC
		ld	a, (PlayerAggressor)
		and	100b
		jr	nz, loc_F9DC
		bit	3, (ix+26h)
		ret	nz

loc_F9DC:				; CODE XREF: SetAgrFireLaser+5j
					; SetAgrFireLaser+Cj
		set	2, (ix+21h)
		ret
; End of function SetAgrFireLaser


; =============== S U B	R O U T	I N E =======================================




; === sub_F9E1 === CODE XREF: FireLaser:loc_B6CAp
sub_F9E1:				; CODE XREF: FireLaser:loc_B6CAp
		call	SetAgrFireLaser	; установка агрессии при попадании из лазера
		jp	sub_7D64
; End of function sub_F9E1


; =============== S U B	R O U T	I N E =======================================




; === sub_F9E7 === CODE XREF: sub_FA0D+1Dp sub_FA0D+26p
sub_F9E7:				; CODE XREF: sub_FA0D+1Dp sub_FA0D+26p
		scf
; End of function sub_F9E7


; =============== S U B	R O U T	I N E =======================================




; === sub_F9E8 === CODE XREF: sub_FA0D+30p
sub_F9E8:				; CODE XREF: sub_FA0D+30p
		push	af
		ex	af, af'
		pop	af
		and	0Fh
		ret	z
		push	hl
		ld	hl, loc_6043
		ld	b, a
		push	de
		ld	de, 27h	; '''

loc_F9F7:				; CODE XREF: sub_F9E8+13j
		add	hl, de
		inc	a
		cp	9
		jr	c, loc_F9F7
		pop	de
		ld	a, (hl)
		and	a
		dec	hl
		dec	hl
		ld	a, (hl)
		pop	hl
		ret	z
		ex	af, af'
		jr	nc, loc_FA0B
		ex	af, af'
		and	a
		ret	z

loc_FA0B:				; CODE XREF: sub_F9E8+1Ej
		ld	c, b
		ret
; End of function sub_F9E8


; =============== S U B	R O U T	I N E =======================================




; === sub_FA0D === CODE XREF: sub_9D40+D9p
sub_FA0D:				; CODE XREF: sub_9D40+D9p
		ld	c, a
		ld	b, (ix+25h)
		bit	0, (ix+26h)
		jr	nz, loc_FA81
		ld	a, (ix+20h)
		and	a
		ret	z
		ld	hl, (word_B452)
		ld	a, (hl)
		ld	de, 7
		add	hl, de
		ld	e, a
		bit	0, (hl)
		inc	hl
		ld	a, (hl)
		ret	z
		call	sub_F9E7
		ret	nz
		ld	a, (hl)
		rrca
		rrca
		rrca
		rrca
		call	sub_F9E7
		ret	nz
		inc	hl
		ld	a, (hl)
		and	0Fh
		jr	z, loc_FA48
		call	sub_F9E8
		jr	z, loc_FA47
		dec	hl
		dec	hl
		bit	0, (hl)
		ret
; ---------------------------------------------------------------------------

loc_FA47:				; CODE XREF: sub_FA0D+33j
		ld	(hl), a

loc_FA48:				; CODE XREF: sub_FA0D+2Ej
		bit	4, e
		ret	z
		bit	6, e
		ret	z
		push	hl
		ld	hl, byte_B461
		ld	b, 7

loc_FA54:				; CODE XREF: sub_FA0D+53j
		ld	a, (hl)
		and	0F0h ; 'Ё'
		cp	90h ; 'Р'
		jr	z, loc_FA7B
		push	de
		ld	e, 0Ah
		add	hl, de
		pop	de
		djnz	loc_FA54
		pop	hl
		bit	2, e
		jr	nz, loc_FA79
		bit	7, (hl)
		jr	nz, loc_FA79
		ld	a, (byte_606A)
		and	a
		ld	b, 8
		jr	z, loc_FA79
		call	IntRnd_A
		and	b
		jr	nz, loc_FA7C

loc_FA79:				; CODE XREF: sub_FA0D+58j sub_FA0D+5Cj ...
		xor	a
		ret
; ---------------------------------------------------------------------------

loc_FA7B:				; CODE XREF: sub_FA0D+4Cj
		pop	hl

loc_FA7C:				; CODE XREF: sub_FA0D+6Aj
		ld	a, (hl)
		and	80h ; 'А'
		or	b
		ld	(hl), a

loc_FA81:				; CODE XREF: sub_FA0D+8j
		ld	c, b
		inc	b
		ret
; End of function sub_FA0D


; =============== S U B	R O U T	I N E =======================================




; === sub_FA84 === CODE XREF: MoveObject+D6p
sub_FA84:				; CODE XREF: MoveObject+D6p
		push	iy
		ld	iy, (word_B452)
		ld	a, (iy+0)
		and	3
		jr	nz, loc_FA98
		ld	a, (PlayerAggressor)
		bit	2, a
		jr	z, loc_FAA9

loc_FA98:				; CODE XREF: sub_FA84+Bj
		ld	a, (iy+9)
		and	0Fh
		or	(iy+8)
		jr	z, loc_FAA8
		bit	1, (iy+7)
		jr	nz, loc_FAA9

loc_FAA8:				; CODE XREF: sub_FA84+1Cj
		scf

loc_FAA9:				; CODE XREF: sub_FA84+12j sub_FA84+22j
		pop	iy
		ret	nc
		ld	hl,  loc_FAB9+1
		ld	a, (CloakingDeviceOn)
		and	a
		scf
		jr	z, loc_FABB
		call	IntRnd_A

loc_FAB9:				; DATA XREF: sub_9389+1o sub_FA84+28o
		cp	20h ; ' '

loc_FABB:				; CODE XREF: sub_FA84+30j
		jr	nc, loc_FABF
		set	7, (hl)

loc_FABF:				; CODE XREF: sub_FA84:loc_FABBj
		push	af
		srl	(hl)
		set	5, (hl)
		pop	af
		set	3, (ix+20h)
		ret
; End of function sub_FA84


; =============== S U B	R O U T	I N E =======================================

; Attributes: noreturn



; === sub_FACA === CODE XREF: ShipSetObject+3p
sub_FACA:				; CODE XREF: ShipSetObject+3p
		push	ix
		call	FindSlot4Obj
		pop	hl
		ret	nz
		ld	a, l
		cp	48h ; 'H'       ; это кориолис?
		jr	nz, loc_FAE9	; нет
		ld	a, c
		cp	0Ah		; Python1
		jr	nz, loc_FAE9
		call	IntRnd_A
		rlca
		jr	c, loc_FAE2
		dec	c		; cobra1

loc_FAE2:				; CODE XREF: sub_FACA+15j
		and	c
		jr	nz, loc_FAE9
		ld	a, c
		add	a, 4
		ld	c, a		; asp/ferdelance

loc_FAE9:				; CODE XREF: sub_FACA+Aj sub_FACA+Fj ...
		ld	a, c
		dec	a
		ret	z		; ракета
		push	ix
		call	FindObjInExtSlot ; поиск объекта в дополнительном слоте
					; hl - адрес слота объекта
; ---------------------------------------------------------------------------
		res	5, (ix+0)
		pop	ix

loc_FAF7:				; DATA XREF: sub_FB03+Ao sub_FB03+23o
		xor	a
		ret
; End of function sub_FACA

; ---------------------------------------------------------------------------
		db  81h	; Б
		db 0C1h	; ┴
		db 0C2h	; ┬
		db 0E2h	; т
		db 0E3h	; у
		db 0F3h	; є
		db 0F4h	; Ї

; =============== S U B	R O U T	I N E =======================================




; === sub_FB00 === CODE XREF: PrnSmallCircle+3Bp
sub_FB00:				; CODE XREF: PrnSmallCircle+3Bp
					; PrnSmallCircle+40p ...
		or	(hl)
		ld	(hl), a
		ret
; End of function sub_FB00


; =============== S U B	R O U T	I N E =======================================

; подготовка для печати	маленькой окружности
; a = радиус окружности	2..8



; === sub_FB03 === CODE XREF: PrnPlanetOnSmallMap+19p
sub_FB03:				; CODE XREF: PrnPlanetOnSmallMap+19p
					; PrnSmallCircle_NoFill+11p
		ld	e, a
		ld	d, 0
		ld	hl,  loc_ED56+1
		ex	af, af'
		and	a
		jr	z, loc_FB10
		ld	hl, loc_FAF7

loc_FB10:				; CODE XREF: sub_FB03+8j
		add	hl, de
		ld	a, (hl)
		and	7
		ld	b, a
		ld	a, (hl)
		and	0F8h ; '°'
		ld	hl, TablCrdPrnVertex

loc_FB1B:				; CODE XREF: sub_FB03+1Cj
		ld	(hl), a
		inc	hl
		sra	a
		djnz	loc_FB1B
		ld	b, e

loc_FB22:				; CODE XREF: sub_FB03+21j
		ld	(hl), a
		inc	hl
		djnz	loc_FB22
		ld	hl, loc_FAF7
		add	hl, de
		ld	a, (hl)
		and	0F8h ; '°'
		ld	(TablCrdPrnVertex), a
		ret
; End of function sub_FB03

; ---------------------------------------------------------------------------


; === sub_FBE0 === CODE XREF: CalcPrnCrdSun+1Bp
sub_FBE0:				; CODE XREF: CalcPrnCrdSun+1Bp
					; CalcPrnCrdSun+1Ep ...
		ld	de, 3
		add	iy, de
		ld	a, (iy+2)
		and	7Fh ; ''
		ld	d, a
		ld	a, (ix+2)
		and	7Fh ; ''
		cp	5
		jr	c, loc_FC00
		cp	d
		jr	c, loc_FC03
		ret	nz
		ld	a, (ix+1)
		cp	(iy+1)
		jr	c, loc_FC03

loc_FC00:				; CODE XREF: sub_FBE0+12j
		or	1
		ret
; ---------------------------------------------------------------------------

loc_FC03:				; CODE XREF: sub_FBE0+15j sub_FBE0+1Ej
		xor	a
		ret
; End of function sub_FBE0

; ---------------------------------------------------------------------------
		db 0AFh	; п
		db 0C9h	; ╔
		db 0F8h	; °


; === sub_FC25 === CODE XREF: sub_9D40+C9p
sub_FC25:				; CODE XREF: sub_9D40+C9p
		ld	a, (ix+23h)
		cp	84h ; 'Д'
		ld	a, 40h ; '@'
		jr	nz, loc_FC30
		ld	a, 7Fh ; ''

loc_FC30:				; CODE XREF: sub_FC25+7j
		or	(hl)
		ld	(hl), a
		ret
; End of function sub_FC25


; =============== S U B	R O U T	I N E =======================================




; === sub_FC33 === CODE XREF: sub_728D:loc_7295p
sub_FC33:				; CODE XREF: sub_728D:loc_7295p

; FUNCTION CHUNK AT D0D7 SIZE 00000003 BYTES
; FUNCTION CHUNK AT D0F6 SIZE 0000001E BYTES

		ld	hl, DemoMode	; =#00 при стыковке со станцией	таргонов
					;      после использования escape pod
					; =#01 включен демо режим
					; =#E0 после прыжка
		ld	(hl), 0
		jp	loc_D0D7
; End of function sub_FC33

; ---------------------------------------------------------------------------
		db  63h	; c
		db 0FCh	; №
; ---------------------------------------------------------------------------
; START	OF FUNCTION CHUNK FOR ScanKeys

loc_FC3D:				; CODE XREF: ScanKeys-146j
		dec	a
		ld	a, (byte_FC0F)
		rra
		ld	d, a
		jr	z, loc_FC57
		ld	d, 0
		rl	d
		rra
		rra
		rl	d
		sla	d
		rra
		rr	d
		rra
		rl	d
		rl	d

loc_FC57:				; CODE XREF: ScanKeys-126j
		ld	hl, ControlKey1
		bit	6, (hl)
		jr	z, loc_FC63
		ld	hl, ControlKey3
		set	0, (hl)

loc_FC63:				; CODE XREF: ScanKeys-149j
					; ScanKeys-10Dj
		bit	4, e
		jr	z, loc_FC74
		ld	a, d
		and	5
		sla	a
		ld	c, a
		ld	a, d
		and	0Ah
		srl	a
		or	c
		ld	d, a

loc_FC74:				; CODE XREF: ScanKeys-104j
		bit	5, e
		jr	z, loc_FC89
		ld	a, d
		and	3
		ld	b, a
		ld	a, d
		and	0Ch
		ld	c, a
		srl	c
		sla	a
		or	c
		and	0Ch
		or	b
		ld	d, a

loc_FC89:				; CODE XREF: ScanKeys-F3j
		ld	hl, ControlKeyMove
		ld	a, (FlagsSetUp)
		and	3
		jr	nz, loc_FCA1
		ld	a, (hl)
		and	a
		jr	z, loc_FCA1
		inc	l
		res	7, (hl)
		inc	l
		res	2, (hl)
		res	7, (hl)
		dec	l
		dec	l

loc_FCA1:				; CODE XREF: ScanKeys-D8j ScanKeys-D4j
		ld	a, (hl)
		and	0F0h ; 'Ё'
		or	d
		ld	(hl), a
		ld	hl, 0FC08h
		bit	6, (hl)
		ret	z
		res	6, (hl)
		ld	a, (ControlKey2)
		and	80h ; 'А'
		ret	z
		ld	a, 0FEh	; '■'
		ld	(PressKeyPrnSymbol), a
		ret
; END OF FUNCTION CHUNK	FOR ScanKeys

; =============== S U B	R O U T	I N E =======================================




; === sub_FCBA === CODE XREF: ScanKeys+18p
sub_FCBA:				; CODE XREF: ScanKeys+18p
		push	bc
		push	af
		ld	a, 5
		sub	e
		sla	a
		sla	a
		sla	a
		add	a, d
		dec	a
		ld	b, a
		sla	b
		ld	hl, unk_FD41
		add	a, l
		jr	nc, loc_FCD1
		inc	h

loc_FCD1:				; CODE XREF: sub_FCBA+14j
		ld	l, a
		ld	a, (hl)
		ld	(PressKeyPrnSymbol), a
		ld	hl, unk_FF58
		ld	a, l
		add	a, b
		ld	l, a
		ld	a, (hl)
		inc	hl
		ld	b, (hl)
		ld	hl, ControlKey1
		add	a, l
		ld	l, a
		ld	a, (hl)
		or	b
		ld	(hl), a
		pop	af
		pop	bc
		ret
; End of function sub_FCBA


; =============== S U B	R O U T	I N E =======================================

; определение возможности установки станции таргонов в межпланетном пространстве для последней подмиссии в войне с таргонами
; Вх:  ---
; Вых: cy =1 станция установлена
;      hl,bc,a - не определены

TestTargonStation:			; CODE XREF: SetTargonStationp
		ld	a, (PilotRating2) ; >=#84 в трюме десант
					; 7,=1 война была или идёт
					; 6,=1 война закончилать
					; 3-1 -	ранг (=текущая стадия войны)
		and	40h ; '@'
		ret	nz
		call	TestPilotInZoneWar ; Определяем	находится ли пилот/планета в заданной зоне войны
					; Вх:  (sp) =#00 зона оккупированная таргонами
					;	    =#3C зона линии фронта (активные боевые действия)
					;	    =#78 прифронтовая зона
					; Вых: cy =1 координаты	в пределах проверяемой зоны
					;	  =0 пилот в другой галактике галактике
					;	     война окончена/не начиналась
					;	     координаты	слишком	далеко от левого края
					;      hl,de,b,a - не определены
; ---------------------------------------------------------------------------
		db 3Ch
; ---------------------------------------------------------------------------
		ld	a, (Pilot_CrdYSystem)
		inc	a
		ret	nc
		xor	5Dh ; ']'
		ret	nz
		ld	a, (Pilot_CrdXSystem)
		xor	3
		ret
; End of function TestTargonStation


; =============== S U B	R O U T	I N E =======================================




; === sub_FD02 === CODE XREF: PrintCompass+D2p
sub_FD02:				; CODE XREF: PrintCompass+D2p
					; DATA XREF: sub_6F29+10r ...
		ld	a, 9
		xor	9
		ld	(sub_FD02+1), a

loc_FD09:				; DATA XREF: sub_6F29+Dw
		jr	z, loc_FD0B

loc_FD0B:
		ld	a, 9
		ld	xl, 7
		ld	e, a
		ld	a, (InFreeSpace)
		and	a
		jr	z, loc_FD1B
		ld	ix, fsSunCrdXl	; координаты "солнца" и "планеты" в межпланетном пространстве

loc_FD1B:				; CODE XREF: sub_FD02+13j
		xor	a
		ld	d, a
		add	ix, de
		ld	(CoriolisNear),	a
		ret
; End of function sub_FD02


; =============== S U B	R O U T	I N E =======================================


GoMission1:				; CODE XREF: Cmd_2_BuyProducts+Dp
		ld	a, (PilotMission1)
		and	a
		scf
		ret	nz
		push	hl
		push	de
		call	TestPilotInZoneCombat ;	Определяем находится ли	пилот в	зоне #3C (активных боёв)
					; Вх:  ---
					; Вых: cy =1 пилот в зоне #3C
					;	  =0 пилот в другой галактике галактике
					;	     война окончена/не начиналась
					;	     пилот в другой зоне
					;      hl,de,b,a - не определены
		pop	de
		pop	hl
		ret	nc
		ld	d, 9Dh ; 'Э'
		ld	a, (PilotRating2) ; >=#84 в трюме десант
					; 7,=1 война была или идёт
					; 6,=1 война закончилать
					; 3-1 -	ранг (=текущая стадия войны)
		cp	84h ; 'Д'
		ret	c
		inc	d
		and	8Eh ; 'О'
		xor	86h ; 'Ж'
		ret	nz
		scf
		ret
; End of function GoMission1

; ---------------------------------------------------------------------------
unk_FD41:	db  20h			; DATA XREF: sub_FCBA+10o
		db  0Dh
		db  50h	; P
		db  30h	; 0
		db  31h	; 1
		db  51h	; Q
		db  41h	; A
		db 0FFh
		db 0FFh
		db  4Ch	; L
		db  4Fh	; O
		db  39h	; 9
		db  32h	; 2
		db  57h	; W
		db  53h	; S
		db  5Ah	; Z
		db  4Dh	; M
		db  4Bh	; K
		db  49h	; I
		db  38h	; 8
		db  33h	; 3
		db  45h	; E
		db  44h	; D
		db  58h	; X
		db  4Eh	; N
		db  4Ah	; J
		db  55h	; U
		db  37h	; 7
		db  34h	; 4
		db  52h	; R
		db  46h	; F
		db  43h	; C
		db  42h	; B
		db  48h	; H
		db  59h	; Y
		db  36h	; 6
		db  35h	; 5
		db  54h	; T
		db  47h	; G
		db  56h	; V

; =============== S U B	R O U T	I N E =======================================


ScanKeys:				; CODE XREF: sub_FF25+10p

; FUNCTION CHUNK AT FC10 SIZE 00000015 BYTES
; FUNCTION CHUNK AT FC3D SIZE 0000007D BYTES

		xor	a
		ld	(byte_FC0F), a
		dec	a
		ld	(PressKeyPrnSymbol), a
		ld	bc, 7FFEh
		ld	d, 8

loc_FD76:				; CODE XREF: ScanKeys-158j
		rlc	b
		in	a, (c)
		and	1Fh
		xor	1Fh
		ld	e, 5

loc_FD80:				; CODE XREF: ScanKeys+1Cj
		rra
		call	c, sub_FCBA
		dec	e
		jr	nz, loc_FD80

loc_FD87:				; DATA XREF: RAM:EB03o
		jp	loc_FC10
; End of function ScanKeys

; ---------------------------------------------------------------------------
		db    5
		db    6
		db  0Dh
		db  0Eh
unk_FD8E:	db 0CAh	; ╩		; DATA XREF: RAM:EF92o
		db  71h	; q
		db  89h	; Й
		db  21h	; !
		db  21h	; !
		db  61h	; a
		db  31h	; 1
		db  20h
unk_FD96:	db  79h	; y		; DATA XREF: sub_E87E+847o
		db  21h	; !
		db  20h
unk_FD99:	db  40h	; @		; DATA XREF: sub_D470+127o
		db  40h	; @
unk_FD9B:	db  41h	; A		; DATA XREF: RAM:D893o
		db  42h	; B
		db  4Fh	; O
		db  55h	; U
		db  53h	; S
		db  45h	; E
		db  49h	; I
		db  54h	; T
		db  49h	; I
		db  4Ch	; L
		db  45h	; E
		db  54h	; T
		db  53h	; S
		db  54h	; T
		db  4Fh	; O
		db  4Eh	; N
		db  4Ch	; L
		db  4Fh	; O
		db  4Eh	; N
		db  55h	; U
		db  54h	; T
		db  48h	; H
		db  4Eh	; N
		db  4Fh	; O
byte_FDB3:	db 41h			; DATA XREF: PrintMessage1_A+179o
					; sub_E03D+9o
		db 4Ch
		db 4Ch
		db 45h
		db 58h
		db 45h
		db 47h
		db  45h	; E
		db  5Ah	; Z
		db  41h	; A
		db  43h	; C
		db  45h	; E
		db  42h	; B
		db  49h	; I
		db  53h	; S
		db  4Fh	; O
		db  55h	; U
		db  53h	; S
		db  45h	; E
		db  53h	; S
		db  41h	; A
		db  52h	; R
		db  4Dh	; M
		db  41h	; A
		db  49h	; I
		db  4Eh	; N
		db  44h	; D
		db  49h	; I
		db  52h	; R
		db  45h	; E
		db  41h	; A
		db  3Fh	; ?
		db  45h	; E
		db  52h	; R
		db  41h	; A
		db  54h	; T
		db  45h	; E
		db  4Eh	; N
		db  42h	; B
		db  45h	; E
		db  52h	; R
		db  41h	; A
		db  4Ch	; L
		db  41h	; A
		db  56h	; V
		db  45h	; E
		db  54h	; T
		db  49h	; I
		db  45h	; E
		db  44h	; D
		db  4Fh	; O
		db  52h	; R
		db  51h	; Q
		db  55h	; U
		db  41h	; A
		db  4Eh	; N
		db  54h	; T
		db  45h	; E
		db  49h	; I
		db  53h	; S
		db  52h	; R
		db  49h	; I
		db  4Fh	; O
		db  4Eh	; N
unk_FDF3:	db 0B1h	; ▒		; DATA XREF: RAM:EDE5o
		db  42h	; B
		db  3Ah	; :
		db  52h	; R
		db  52h	; R
		db  54h	; T
		db  92h	; Т
		db  70h	; p
unk_FDFB:	db 0D9h	; ┘		; DATA XREF: sub_EF48+16o
		db 0F1h	; ё
		db 0F0h	; Ё
		db 0FEh	; ■
		db 0FEh	; ■
		db 0FEh	; ■

; =============== S U B	R O U T	I N E =======================================




; === sub_FE01 === CODE XREF: sub_FF25+13p
sub_FE01:				; CODE XREF: sub_FF25+13p
		ld	a, (WhatOnScreen)
		cp	4
		ret	nc
		ld	a, (ControlKeyMove)
		ld	b, a
		ld	a, (FlagsSetUp)
		ld	c, a
		ld	hl, (AngleLeftRight)
		bit	0, b
		jr	nz, loc_FE41
		bit	1, b
		jr	nz, loc_FE2E
		bit	3, c
		jr	z, loc_FE7A
		ld	de, 3Ch	; '<'
		and	a
		sbc	hl, de
		bit	6, h
		jr	z, loc_FE7A
		ld	hl, 0
		jp	loc_FE7A
; ---------------------------------------------------------------------------

loc_FE2E:				; CODE XREF: sub_FE01+17j
		ld	a, h
		and	7Fh ; ''
		or	l
		jr	nz, loc_FE3A
		ld	hl, 0FFh
		jp	loc_FE7A
; ---------------------------------------------------------------------------

loc_FE3A:				; CODE XREF: sub_FE01+31j
		bit	7, h
		jr	z, loc_FE55
		jp	loc_FE63
; ---------------------------------------------------------------------------

loc_FE41:				; CODE XREF: sub_FE01+13j
		bit	1, b
		jr	nz, loc_FE7A
		ld	a, h
		and	7Fh ; ''
		or	l
		jr	nz, loc_FE51
		ld	hl, 80FFh
		jp	loc_FE7A
; ---------------------------------------------------------------------------

loc_FE51:				; CODE XREF: sub_FE01+48j
		bit	7, h
		jr	z, loc_FE63

loc_FE55:				; CODE XREF: sub_FE01+3Bj
		ld	de, 0FFh
		add	hl, de
		bit	4, h
		jr	z, loc_FE7A
		and	a
		sbc	hl, de
		jp	loc_FE7A
; ---------------------------------------------------------------------------

loc_FE63:				; CODE XREF: sub_FE01+3Dj sub_FE01+52j
		bit	2, c
		jr	z, loc_FE6D
		ld	hl, 0
		jp	loc_FE7A
; ---------------------------------------------------------------------------

loc_FE6D:				; CODE XREF: sub_FE01+64j
		ld	de, 0FFh
		and	a
		sbc	hl, de
		bit	6, h
		jr	z, loc_FE7A
		ld	hl, 0

loc_FE7A:				; CODE XREF: sub_FE01+1Bj sub_FE01+25j ...
		ld	(AngleLeftRight), hl
		ld	hl, (byte_FF0A)
		bit	2, b
		jr	nz, loc_FEAF
		bit	3, b
		jr	nz, loc_FE9C
		bit	3, c
		jr	z, loc_FEE8
		ld	de, 24h	; '$'
		and	a
		sbc	hl, de
		bit	6, h
		jr	z, loc_FEE8
		ld	hl, 0
		jp	loc_FEE8
; ---------------------------------------------------------------------------

loc_FE9C:				; CODE XREF: sub_FE01+85j
		ld	a, h
		and	7Fh ; ''
		or	l
		jr	nz, loc_FEA8
		ld	hl, 66h	; 'f'
		jp	loc_FEE8
; ---------------------------------------------------------------------------

loc_FEA8:				; CODE XREF: sub_FE01+9Fj
		bit	7, h
		jr	z, loc_FEC3
		jp	loc_FED1
; ---------------------------------------------------------------------------

loc_FEAF:				; CODE XREF: sub_FE01+81j
		bit	3, b
		jr	nz, loc_FEE8
		ld	a, h
		and	7Fh ; ''
		or	l
		jr	nz, loc_FEBF
		ld	hl, 8066h
		jp	loc_FEE8
; ---------------------------------------------------------------------------

loc_FEBF:				; CODE XREF: sub_FE01+B6j
		bit	7, h
		jr	z, loc_FED1

loc_FEC3:				; CODE XREF: sub_FE01+A9j
		ld	de, 66h	; 'f'
		add	hl, de
		bit	4, h
		jr	z, loc_FEE8
		and	a
		sbc	hl, de
		jp	loc_FEE8
; ---------------------------------------------------------------------------

loc_FED1:				; CODE XREF: sub_FE01+ABj sub_FE01+C0j
		bit	2, c
		jr	z, loc_FEDB
		ld	hl, 0
		jp	loc_FEE8
; ---------------------------------------------------------------------------

loc_FEDB:				; CODE XREF: sub_FE01+D2j
		ld	de, 66h	; 'f'
		and	a
		sbc	hl, de
		bit	6, h
		jr	z, loc_FEE8
		ld	hl, 0

loc_FEE8:				; CODE XREF: sub_FE01+89j sub_FE01+93j ...
		ld	(byte_FF0A), hl
		ld	hl, (DeltaSpeed)
		ld	de, 0A0h ; 'а'
		bit	4, b
		jr	nz, loc_FF00
		bit	5, b
		jr	z, loc_FF1D
		and	a
		sbc	hl, de
		jr	loc_FF0E
; ---------------------------------------------------------------------------

loc_FEFE:				; DATA XREF: sub_9D40+5B7Dw
					; sub_9D40+5B90w
		jr	sub_FF25
; ---------------------------------------------------------------------------

loc_FF00:				; CODE XREF: sub_FE01+F2j
		bit	5, b
		jr	nz, loc_FF1D
		add	hl, de
		jp	loc_FF16
; ---------------------------------------------------------------------------


; === sub_FF25 === CODE XREF: sub_FE01:loc_FEFEj
sub_FF25:				; CODE XREF: sub_FE01:loc_FEFEj
		di
		ld	(word_FF56), sp
		ld	sp, 4020h
		push	hl
		push	de
		push	bc
		push	af
		push	ix
		push	iy
		call	ScanKeys	; опрос	клавиш
		call	sub_FE01
		call	sub_7125	; звук
		pop	iy
		pop	ix
		pop	af
		pop	bc
		pop	de
		pop	hl
		ld	sp, (word_FF56)
		ei
		ret
; End of function sub_FF25

; ---------------------------------------------------------------------------
		db  3Eh	; >
		db 0FDh	; ¤
		db 0EDh	; э
		db  47h	; G
		db 0EDh	; э
		db  5Eh	; ^
		db 0C9h	; ╔
		db 0EDh	; э
		db  56h	; V
		db 0C9h	; ╔
word_FF56:	dw 411Ch		; DATA XREF: sub_FF25+1w sub_FF25+21r
unk_FF58:	db    1			; DATA XREF: sub_FCBA+1Co
		db  10h
		db    0
		db    1
		db    0
		db    8
		db    0
		db  40h	; @
		db    2
		db    1
		db    3
		db  40h	; @
		db    3
		db    1
		db    2
		db  80h	; А
		db    1
		db  20h
		db    0
		db    2
		db    0
		db  10h
		db    7
		db    1
		db    2
		db    2
		db    3
		db  20h
		db    1
		db    8
		db    0
		db    0
		db    1
		db    2
		db    0
		db    4
		db    0
		db  20h
		db    7
		db    4
		db    2
		db    4
		db    3
		db  10h
		db    4
		db  20h
		db    1
		db    4
		db    1
		db    1
		db    2
		db  40h	; @
		db    3
		db    8
		db    7
		db  10h
		db    2
		db    8
		db    4
		db  40h	; @
		db    3
		db    4
		db    3
		db  80h	; А
		db    4
		db  10h
		db    2
		db  10h
		db    0
		db  80h	; А
		db    7
		db    8
		db    7
		db    2
		db    3
		db    2
		db    2
		db  20h
		db  3Ch	; <
		db    0
		db    0
		db  44h	; D
		db  48h	; H
		db    1
		db    0
		db 0FEh	; ■
		db 0BFh	; ┐
		db  84h	; Д
		db 0FDh	; ¤
		db  3Eh	; >
		db 0FFh
		db  19h
		db  6Ah	; j
		db 0D4h	; ╘
		db    3
		db    8
		db  15h
		db    0
		db  0Ch
		db  20h
		db    0
		db  0Dh
		db 0FFh
		db    0
		db    0
		db  42h	; B
		db  62h	; b
		db  52h	; R
		db  4Ah	; J
		db  46h	; F
		db  42h	; B
		db    0
		db    0
		db  3Ch	; <
		db  42h	; B
		db  42h	; B
		db  42h	; B
		db  42h	; B
		db  3Ch	; <
		db    0
unk_FFD0:	db    0			; DATA XREF: sub_F8E4+Bo sub_F8E4+1Ao
		db    0
		db    0
		db    0
		db    0
		db    0
		db    0
		db    0
		db    0
		db    0
		db    0
		db    0
		db    0
		db    0
		db    1
		db    0
		db  4Bh	; K
		db 0D5h	; ╒
		db 0F8h	; °
		db  5Dh	; ]
		db  3Eh	; >
		db 0ABh	; л
		db 0A9h	; й
		db 0A6h	; ж
		db    4
		db 0FCh	; №
		db 0C9h	; ╔
		db 0A6h	; ж
		db 0CDh	; ═
		db 0A6h	; ж
		db  95h	; Х
		db 0AEh	; о
		db  70h	; p
		db  9Fh	; Я
		db  67h	; g
		db  9Fh	; Я
		db  3Fh	; ?
		db  9Eh	; Ю
a@aUDu:		.ascii '@Э`Ю╕ЬDu',0
		db    1
; end of 'RAM'


		end


