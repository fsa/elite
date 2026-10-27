; 04_movement_physics - разобрано из kladov.asm
; Количество подпрограмм: 18

; === CalcMatrixRotate === CODE XREF: objPrnObj+2p
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




; === DemoRotateObj === CODE XREF: ReCalcCrdObj+6Aj
DemoRotateObj:				; CODE XREF: ReCalcCrdObj+6Aj
					; ReCalcCrdObj+77j ...
		ld	(AngleLR4Calc+1), a
		push	ix
		call	RotateCrdNSCO
		call	RotateCrdNSCO
		call	RotateCrdNSCO
		pop	ix
		ld	a, (ix+1Ch)
		and	a
		jr	z, loc_A032
		ld	b, (ix+1Bh)
		add	a, b
		bit	7, a
		jr	z, loc_A01E
		xor	a
		jr	loc_A02F
; ---------------------------------------------------------------------------

loc_A01E:				; CODE XREF: DemoRotateObj+1Cj
		ld	c, (iy+6)
		bit	0, (ix+21h)
		jr	z, loc_A02B
		sla	c
		sla	c

loc_A02B:				; CODE XREF: DemoRotateObj+28j
		cp	c
		jr	c, loc_A02F
		ld	a, c

loc_A02F:				; CODE XREF: DemoRotateObj+1Fj
					; DemoRotateObj+2Fj
		ld	(ix+1Bh), a

loc_A032:				; CODE XREF: DemoRotateObj+14j
		push	ix
		push	iy
		ld	c, (ix+1Bh)
		push	ix
		pop	iy
		call	CalcNewCrdObjSpeed
		call	CalcNewCrdObjSpeed
		call	CalcNewCrdObjSpeed
		pop	iy
		pop	ix
		ret
; End of function DemoRotateObj


; =============== S U B	R O U T	I N E =======================================


CalcNewCrdObjSpeed:			; CODE XREF: DemoRotateObj+40p
					; DemoRotateObj+43p ...
		push	bc
		ld	d, (ix+0Ah)
		ld	e, (ix+9)
		call	sub_A076
		ld	a, (iy+2)
		ld	h, (iy+1)
		ld	l, (iy+0)
		call	sub_A098
		ld	(iy+2),	b
		ld	(iy+1),	h
		ld	(iy+0),	l
		inc	iy
		inc	iy
		inc	iy
		inc	ix
		inc	ix
		pop	bc
		ret
; End of function CalcNewCrdObjSpeed


; =============== S U B	R O U T	I N E =======================================




; === DirectObject === CODE XREF: ReactionObject+D2j
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




; === MoveAllObjAI === CODE XREF: sub_F8E4+16p
MoveAllObjAI:				; CODE XREF: sub_F8E4+16p
		xor	a
		call	sub_7C60
		ld	a, (J_Drive_Ready4)
		and	a
		ret	nz
		ld	(PlayerPirate),	a
		ret
; End of function MoveAllObjAI


; =============== S U B	R O U T	I N E =======================================

; Attributes: noreturn

CalcLenVector:				; CODE XREF: MoveObject+Cp
		push	ix
		call	Sqr_HL_wordAdrIX
		push	hl
		call	Sqr_HL_wordAdrIX
		push	hl
		call	Sqr_HL_wordAdrIX
		pop	de
		add	hl, de
		pop	de
		add	hl, de
		pop	ix
		call	Sqrt_H_HL
; ---------------------------------------------------------------------------
		ex	de, hl
		push	ix
		call	CaclNormalVector
; ---------------------------------------------------------------------------
		call	CaclNormalVector
; ---------------------------------------------------------------------------
		call	CaclNormalVector
; ---------------------------------------------------------------------------
		pop	ix
		ret
; End of function CalcLenVector


; =============== S U B	R O U T	I N E =======================================


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



; === MoveAllObjects === CODE XREF: j_MoveAllObjectsp
MoveAllObjects:				; CODE XREF: j_MoveAllObjectsp
		ld	a, (PilotMission3) ; =#06 при стыковке будет захват таргонами (вроде нереальная	ситуация)
					; =#04 в системе со танцией таргонов во	время войны
		cp	4
		jr	nz, loc_9CC7
		ld	hl, byte_6069
		set	2, (hl)

loc_9CC7:				; CODE XREF: MoveAllObjects+5j
		ld	a, (TypeOfPrint)
		and	a
		ld	a, 0B6h	; '╢'
		jr	z, loc_9CD1
		ld	a, 0AEh	; 'о'

loc_9CD1:				; CODE XREF: MoveAllObjects+12j
		ld	(unk_B138), a	; тип печати OR/XOR
		ld	(loc_B18B), a
		ld	(loc_B18E), a
		ld	(loc_B1A6), a
		ld	a, (FrameCounter)
		dec	a
		ld	(FrameCounter),	a
		ld	b, a
		ld	a, (CoriolisNear)
		and	a
		jr	nz, loc_9CFD
		ld	a, (InFreeSpace)
		and	a
		jr	nz, loc_9CFD
		ld	a, (nMissileObj)
		cp	4
		jr	nc, loc_9CFD
		ld	a, b
		and	a
		call	z, sub_B867	; CallFunctionObj  set rnd ship
; ---------------------------------------------------------------------------

loc_9CFD:				; CODE XREF: MoveAllObjects+2Ej
					; MoveAllObjects+34j ...
		xor	a
		ld	(DamageEnergyUnit), a
		ld	(DamageRearShield), a
		ld	(DamageFrontShield), a
		ld	(ObjInTarget), a
		ld	(FlgCoriolisNear), a
		ld	(FlgObjNear), a
		ld	(J_Drive_Ready4), a
		ld	c, a
		ld	ix, SlotObj_05
		ld	de, 0FFD9h
		ld	b, 6

loc_9D1D:				; CODE XREF: MoveAllObjects+71j
		ld	a, (ix+22h)
		and	a
		jr	z, loc_9D2A
		bit	6, (ix+20h)
		jr	z, loc_9D2A
		inc	c

loc_9D2A:				; CODE XREF: MoveAllObjects+66j
					; MoveAllObjects+6Cj
		add	ix, de
		djnz	loc_9D1D
		ld	a, c		; a,c кол-во ракет в пространстве
		call	SetFirstExtSlot	; установка адреса первого допслота
		ld	a, d
		ld	(PrintObject_2+1), a ; a=0
		ld	a, 8

loc_9D38:				; CODE XREF: sub_F8E4j
		push	af
		ld	b, a
		call	sub_B4B1	; допслот 7,+0 =0
		call	sub_ED69	; a,(ix+22h)
; End of function MoveAllObjects


; =============== S U B	R O U T	I N E =======================================




; === MoveObject === CODE XREF: DirectObject:MoveObject1j
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




; === MoveObject1 === CODE XREF: DirectObject+67j
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




; === ReactionObject === CODE XREF: ReactionObject2+7p
ReactionObject:				; CODE XREF: ReactionObject2+7p
		ld	a, (iy+7)
		ld	c, (ix+22h)
		inc	c
		cp	0F0h ; 'Ё'
		jr	c, loc_A2CE
		inc	c
		inc	c
		inc	c

loc_A2CE:				; CODE XREF: ReactionObject+9j
		cp	c
		jr	c, loc_A2D2
		ld	a, c

loc_A2D2:				; CODE XREF: ReactionObject+Fj
		ld	b, a
		ld	a, (CloakingDeviceOn)
		and	a
		nop
		ld	hl, Mission2
		ld	a, b
		cp	50h ; 'P'
		ld	a, (hl)
		jr	c, loc_A2E6
		cp	2
		jr	nz, loc_A2E6
		dec	(hl)

loc_A2E6:				; CODE XREF: ReactionObject+1Fj
					; ReactionObject+23j
		cp	1
		jr	nz, loc_A2F7
		ld	a, b
		cp	34h ; '4'
		jr	nc, loc_A2F4	; энергия >=#34
		ld	a, 2
		ld	(hl), a
		jr	loc_A2F7
; ---------------------------------------------------------------------------

loc_A2F4:				; CODE XREF: ReactionObject+2Dj
		sub	2
		ld	b, a

loc_A2F7:				; CODE XREF: ReactionObject+28j
					; ReactionObject+32j
		ld	a, b
		ld	(ix+22h), a
		ld	b, (iy+7)
		add	a, a
		jr	c, loc_A36C	; энергии от 50% до 100%
		cp	b
		jr	nc, loc_A36C	; энергии от 50% до 100%
		add	a, a
		jr	c, loc_A30D	; энергии от 13% до 49%
					; попытка запуска ракеты/корабля
		add	a, a
		jr	c, loc_A30D	; энергии от 13% до 49%
					; попытка запуска ракеты/корабля
		cp	b
		jr	c, loc_A343	; критический уровень энергии. менее 13%
					; попытка запуска объектом спасательной	капсулы/сброс груза

loc_A30D:				; CODE XREF: ReactionObject+45j
					; ReactionObject+48j
		ld	a, (ix+20h)	; энергии от 13% до 49%
					; попытка запуска ракеты/корабля
		and	7
		jr	z, loc_A36C	; энергии от 50% до 100%
		call	IntRnd_A
		cp	80h ; 'А'
		jr	nc, loc_A36C	; энергии от 50% до 100%
		ld	a, (TimeWorkECM)
		and	a
		jr	nz, loc_A36C	; энергии от 50% до 100%
		ld	a, (iy+0Ah)
		and	0F0h ; 'Ё'
		jr	nz, loc_A32A
		or	6

loc_A32A:				; CODE XREF: ReactionObject+66j
		xor	4
		ld	b, a
		push	bc
		call	CallFunction_Obj
; ---------------------------------------------------------------------------
		pop	bc
		jr	nz, loc_A36C	; энергии от 50% до 100%
		dec	(ix+20h)
		ld	a, b
		cp	2
		jr	nz, loc_A36C	; энергии от 50% до 100%
		ld	hl, ShipLaunchMisile
		set	7, (hl)
		jr	loc_A36C	; энергии от 50% до 100%
; ---------------------------------------------------------------------------

loc_A343:				; CODE XREF: ReactionObject+4Bj
		call	IntRnd_A	; критический уровень энергии. менее 13%
					; попытка запуска объектом спасательной	капсулы/сброс груза
		cp	(ix+25h)
		jr	c, loc_A3A2	; (шанс	запуска	EscapePod не сработал/это не тарглет)
					; попытка включить ECM
		bit	7, (ix+20h)
		jr	z, loc_A361	; сброс	грузового отсека
		ld	a, 7
		call	CallFunction_Obj
; ---------------------------------------------------------------------------
		jr	nz, loc_A36C	; энергии от 50% до 100%
		res	7, (ix+20h)
		res	6, (ix+20h)
		ret
; ---------------------------------------------------------------------------

loc_A361:				; CODE XREF: ReactionObject+8Fj
		ld	a, (iy+0)	; сброс	грузового отсека
		and	a
		jr	z, loc_A36C	; энергии от 50% до 100%
		ld	a, 6
		call	CallFunction_Obj
; ---------------------------------------------------------------------------

loc_A36C:				; CODE XREF: ReactionObject+3Fj
					; ReactionObject+42j ...
		ld	a, (iy+12h)	; энергии от 50% до 100%
		cp	4
		jr	nz, loc_A3A2	; (шанс	запуска	EscapePod не сработал/это не тарглет)
					; попытка включить ECM
		push	ix		; обработка тарглета
		ld	ix, objCoriolis
		ld	de, 27h	; '''
		ld	b, 6

loc_A37E:				; CODE XREF: ReactionObject:loc_A38Dj
		add	ix, de
		ld	a, (ix+22h)
		and	a
		jr	z, loc_A38D
		ld	a, (ix+21h)
		sub	0Ch
		jr	z, loc_A390

loc_A38D:				; CODE XREF: ReactionObject+C4j
		djnz	loc_A37E
		inc	a

loc_A390:				; CODE XREF: ReactionObject+CBj
		pop	ix
		jp	z, DirectObject
		res	6, (ix+20h)
		set	1, (ix+26h)
		srl	(ix+1Bh)
		ret
; ---------------------------------------------------------------------------

loc_A3A2:				; CODE XREF: ReactionObject+89j
					; ReactionObject+B1j
		bit	0, (ix+25h)	; (шанс	запуска	EscapePod не сработал/это не тарглет)
					; попытка включить ECM
		jr	z, loc_A3D3
		ld	a, (PilotMission3) ; =#06 при стыковке будет захват таргонами (вроде нереальная	ситуация)
					; =#04 в системе со танцией таргонов во	время войны
		cp	4
		jr	nz, loc_A3B6
		ld	a, (PilotExtWeapon) ; 7,=1 - ECM Jammer
					; 6,=1 - Cloaking device
					; 5,=1 - нельзя	купить товары(в	трюме беженцы или десант)
					; 4
					; 3 =1 3я миссия была выполнена
					; 2 =1 - третья	миссия выполнена
					; 1 =1 - второя	миссия выполнена
					; 0 =1 - первая	миссия выполнена
		bit	7, a
		jr	nz, loc_A3D3

loc_A3B6:				; CODE XREF: ReactionObject+EDj
		ld	a, (TimeWorkECM)
		and	a
		jr	nz, loc_A3D3
		push	ix
		call	sub_F95A	; проверка есть	ли ракета нацеленная на	текущий	объект
		ld	a, (ix+23h)
		pop	ix
		jr	nc, loc_A3D1
		cp	0E3h ; 'у'
		jr	nz, loc_A3D1
		ld	a, 8
		ld	(TimeWorkECM), a

loc_A3D1:				; CODE XREF: ReactionObject+106j
					; ReactionObject+10Aj
		nop
		nop

loc_A3D3:				; CODE XREF: ReactionObject+E6j
					; ReactionObject+F4j ...
		bit	2, (ix+26h)
		jr	z, DirectObject
		ld	a, (nMissileObj)
		cp	4
		ret	nc
		call	IntRnd_A
		bit	2, (ix+21h)
		jr	z, loc_A3FE
		cp	24h ; '$'
		ret	nc
		ld	a, (PilotMission3) ; =#06 при стыковке будет захват таргонами (вроде нереальная	ситуация)
					; =#04 в системе со танцией таргонов во	время войны
		cp	4
		ld	a, 14h
		jr	nz, CallFunction_Obj
		call	IntRnd_A
		cp	88h ; 'И'
		ret	nc
		ld	a, 44h ; 'D'
		jr	CallFunction_Obj
; ---------------------------------------------------------------------------

loc_A3FE:				; CODE XREF: ReactionObject+126j
		cp	2
		ret	nc
		ld	a, 24h ; '$'
; End of function ReactionObject


; =============== S U B	R O U T	I N E =======================================

; Attributes: noreturn

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




; === ReactionObject2 === CODE XREF: sub_9D40+E6p
ReactionObject2:			; CODE XREF: sub_9D40+E6p
		ld	(loc_DA2B+1), sp
		ld	sp, 4220h
		call	ReactionObject

loc_DA2B:				; DATA XREF: ReactionObject2w
		ld	sp, 4118h
		ret
; End of function ReactionObject2


; =============== S U B	R O U T	I N E =======================================




; === RotateAxisSCO === CODE XREF: ReCalcCrdObj+33p
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




; === RotateCrdN === CODE XREF: MakeTablPrnCrdVertex:loc_AB90p
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


; === RotateCrdNSCO === CODE XREF: DemoRotateObj+5p
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




; === RotateCrdN_Normal === CODE XREF: CalcVisibleVerges+47p
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


; === RotateCrdN_Vertex === CODE XREF: MakeTablPrnCrdVertex+2FBp
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




; === RotateCrdVertex === CODE XREF: MakeTablPrnCrdVertex+2E0j
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




; === RotateSKOobj === CODE XREF: CallFunctionObj_1+1AFp
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




; === j_MoveAllObjects === CODE XREF: j_j_MoveAllObjectsj
j_MoveAllObjects:			; CODE XREF: j_j_MoveAllObjectsj

; FUNCTION CHUNK AT B7B4 SIZE 00000027 BYTES

		call	MoveAllObjects
; End of function j_MoveAllObjects

; ---------------------------------------------------------------------------
		jp	Processing_LaserECM

; =============== S U B	R O U T	I N E =======================================

; Attributes: noreturn

MoveAllObjects:				; CODE XREF: j_MoveAllObjectsp
		ld	a, (PilotMission3) ; =#06 при стыковке будет захват таргонами (вроде нереальная	ситуация)
					; =#04 в системе со танцией таргонов во	время войны
		cp	4
		jr	nz, loc_9CC7
		ld	hl, byte_6069
		set	2, (hl)

loc_9CC7:				; CODE XREF: MoveAllObjects+5j
		ld	a, (TypeOfPrint)
		and	a
		ld	a, 0B6h	; '╢'
		jr	z, loc_9CD1
		ld	a, 0AEh	; 'о'

loc_9CD1:				; CODE XREF: MoveAllObjects+12j
		ld	(unk_B138), a	; тип печати OR/XOR
		ld	(loc_B18B), a
		ld	(loc_B18E), a
		ld	(loc_B1A6), a
		ld	a, (FrameCounter)
		dec	a
		ld	(FrameCounter),	a
		ld	b, a
		ld	a, (CoriolisNear)
		and	a
		jr	nz, loc_9CFD
		ld	a, (InFreeSpace)
		and	a
		jr	nz, loc_9CFD
		ld	a, (nMissileObj)
		cp	4
		jr	nc, loc_9CFD
		ld	a, b
		and	a
		call	z, sub_B867	; CallFunctionObj  set rnd ship
; ---------------------------------------------------------------------------

loc_9CFD:				; CODE XREF: MoveAllObjects+2Ej
					; MoveAllObjects+34j ...
		xor	a
		ld	(DamageEnergyUnit), a
		ld	(DamageRearShield), a
		ld	(DamageFrontShield), a
		ld	(ObjInTarget), a
		ld	(FlgCoriolisNear), a
		ld	(FlgObjNear), a
		ld	(J_Drive_Ready4), a
		ld	c, a
		ld	ix, SlotObj_05
		ld	de, 0FFD9h
		ld	b, 6

loc_9D1D:				; CODE XREF: MoveAllObjects+71j
		ld	a, (ix+22h)
		and	a
		jr	z, loc_9D2A
		bit	6, (ix+20h)
		jr	z, loc_9D2A
		inc	c

loc_9D2A:				; CODE XREF: MoveAllObjects+66j
					; MoveAllObjects+6Cj
		add	ix, de
		djnz	loc_9D1D
		ld	a, c		; a,c кол-во ракет в пространстве
		call	SetFirstExtSlot	; установка адреса первого допслота
		ld	a, d
		ld	(PrintObject_2+1), a ; a=0
		ld	a, 8

loc_9D38:				; CODE XREF: sub_F8E4j
		push	af
		ld	b, a
		call	sub_B4B1	; допслот 7,+0 =0
		call	sub_ED69	; a,(ix+22h)
; End of function MoveAllObjects


; =============== S U B	R O U T	I N E =======================================




; === j_j_MoveAllObjects === CODE XREF: sub_728D+236p
j_j_MoveAllObjects:			; CODE XREF: sub_728D+236p
					; sub_728D+26Ap ...
		jp	j_MoveAllObjects
; End of function j_j_MoveAllObjects


; =============== S U B	R O U T	I N E =======================================

; Attributes: thunk



