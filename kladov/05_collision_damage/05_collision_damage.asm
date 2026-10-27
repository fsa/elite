; 05_collision_damage - разобрано из kladov.asm
; Количество подпрограмм: 7

; === CollisionWithObj === CODE XREF: sub_9D40:loc_9E41p
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



; === Damage2Pilot === CODE XREF: sub_74C6+9p
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




; === DestroyObj === CODE XREF: FireLaser+CEp
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




; === PrnExplodeObj === CODE XREF: sub_ED60p
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




; === objCollision === CODE XREF: CollisionWithObj+4Cj
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



; === objDestroyed === получение координат и	знаков проекции	объекта
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


; === objDestroyed2 === CODE XREF: sub_7E02+8j
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


