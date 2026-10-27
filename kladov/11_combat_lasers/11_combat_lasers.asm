; 11_combat_lasers - разобрано из kladov.asm
; Количество подпрограмм: 17

; === FireLaser === CODE XREF: RAM:B7F1p
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



; === MissileFire === CODE XREF: UseMissile+37j
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



; === MissileTarget === CODE XREF: UseMissile+33j
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



; === MissileUntarget === CODE XREF: UseMissile+3Bj
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



; === MoveMissile === CODE XREF: sub_9D40+DFp
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




; === OffTargetMissiles === CODE XREF: CounterShipHyperJump+18p
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



; === OffTargetMissiles_2 === CODE XREF: CollisionWithObj+59p
OffTargetMissiles_2:			; CODE XREF: CollisionWithObj+59p
		call	sub_9389
		jp	OffTargetMissiles
; End of function OffTargetMissiles_2


; =============== S U B	R O U T	I N E =======================================




; === PrintMissiles === CODE XREF: j_PrintMissilesj
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




; === PrnLaser === CODE XREF: FireLaser+79j
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



; === Process_LaserECM2 === CODE XREF: j_MoveAllObjects+1B03j
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



; === Processing_LaserECM === CODE XREF: RAM:9CB8j
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



; === SetAgrFireLaser === CODE XREF: sub_9D40:loc_9D94p
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




; === SetObjMissile === CODE XREF: CallFunctionObj_1+10j
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




; === UseMissile === CODE XREF: Process_MissBombEPod+9p
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



; === j_PrintMissiles === CODE XREF: sub_9946p
j_PrintMissiles:			; CODE XREF: sub_9946p
		jp	PrintMissiles
; End of function j_PrintMissiles

; ---------------------------------------------------------------------------
byte_9C1B:	db 0			; DATA XREF: sub_9946+16w
					; UseEscapePodo ...

; =============== S U B	R O U T	I N E =======================================


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



; === objPrintLaser === CODE XREF: PrnObjIfVisible+9Fj
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


; === objPrnLaser === CODE XREF: sub_A7D9+110j
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


