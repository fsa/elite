; 15_trading_equip - разобрано из kladov.asm
; Количество подпрограмм: 10

; === AddFoundCargo === CODE XREF: CollisionWithObj:AddFoundCargo2j
AddFoundCargo:				; CODE XREF: CollisionWithObj:AddFoundCargo2j
		jp	AddFoundCargo3
; END OF FUNCTION CHUNK	FOR CollisionWithObj
; ---------------------------------------------------------------------------

AddCashObj3:				; CODE XREF: AddCashObj+Ej
		jp	AddCashObj2

; =============== S U B	R O U T	I N E =======================================

; Attributes: thunk



; === AddFoundCargo2 === CODE XREF: CollisionWithObj+6Ej
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



; === AddFoundCargo3 === CODE XREF: CollisionWithObj:AddFoundCargoj
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




; === Cmd_4_EquipShip === CODE XREF: RAM:D006j	RAM:EA38j ...
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




; === Cmd_Ent_Inventory === CODE XREF: RAM:D018j
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


; === GenerateAnotherPrice === CODE XREF: InitGame+23p
GenerateAnotherPrice:			; CODE XREF: InitGame+23p
		call	sub_9B83
		ld	de, ProductPrice
		exx
		ld	hl, TableBasePrice ; +#28 Fireams
		ld	b, 11h

loc_E154:				; CODE XREF: GenerateAnotherPrice+6Cj
		push	bc
		ld	e, (hl)
		inc	hl
		ld	a, (hl)
		ld	d, a
		push	de
		and	1Fh
		jr	z, loc_E167
		ld	b, a
		ld	a, (PlanetEconomy)
		ld	c, a
		xor	a

loc_E164:				; CODE XREF: GenerateAnotherPrice+1Dj
		add	a, c
		djnz	loc_E164

loc_E167:				; CODE XREF: GenerateAnotherPrice+14j
		ld	e, a
		inc	hl
		ld	b, (hl)
		inc	hl
		ld	c, (hl)
		inc	hl
		ld	a, (PilotRnd)
		and	c
		add	a, b
		ld	b, a
		ld	a, e
		bit	7, d
		jr	nz, loc_E17B
		inc	a
		neg

loc_E17B:				; CODE XREF: GenerateAnotherPrice+2Ej
		add	a, b
		bit	7, a
		jr	z, loc_E181
		xor	a

loc_E181:				; CODE XREF: GenerateAnotherPrice+36j
		and	3Fh ; '?'
		ld	b, e
		pop	de
		push	af
		ld	a, (PilotRnd)
		and	c
		add	a, e
		ld	e, 0
		rl	e
		ld	c, a
		ld	a, b
		bit	7, d
		jr	z, loc_E19A
		ld	d, a
		ld	a, c
		sub	d
		jr	loc_E19E
; ---------------------------------------------------------------------------

loc_E19A:				; CODE XREF: GenerateAnotherPrice+4Bj
		add	a, c
		jr	nc, loc_E19E
		inc	e

loc_E19E:				; CODE XREF: GenerateAnotherPrice+50j
					; GenerateAnotherPrice+53j
		ld	b, e
		add	a, a
		rl	b
		add	a, a
		rl	b
		ld	c, a
		push	bc
		exx
		pop	bc
		ld	a, c
		ld	(de), a
		inc	de
		ld	a, b
		ld	(de), a
		inc	de
		pop	af

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


; === GenerateProductPrice === CODE XREF: sub_DCE9+26p
GenerateProductPrice:			; CODE XREF: sub_DCE9+26p
		call	sub_ABAF
		ld	(PilotRnd), a
; End of function GenerateProductPrice


; =============== S U B	R O U T	I N E =======================================


GenerateAnotherPrice:			; CODE XREF: InitGame+23p
		call	sub_9B83
		ld	de, ProductPrice
		exx
		ld	hl, TableBasePrice ; +#28 Fireams
		ld	b, 11h

loc_E154:				; CODE XREF: GenerateAnotherPrice+6Cj
		push	bc
		ld	e, (hl)
		inc	hl
		ld	a, (hl)
		ld	d, a
		push	de
		and	1Fh
		jr	z, loc_E167
		ld	b, a
		ld	a, (PlanetEconomy)
		ld	c, a
		xor	a

loc_E164:				; CODE XREF: GenerateAnotherPrice+1Dj
		add	a, c
		djnz	loc_E164

loc_E167:				; CODE XREF: GenerateAnotherPrice+14j
		ld	e, a
		inc	hl
		ld	b, (hl)
		inc	hl
		ld	c, (hl)
		inc	hl
		ld	a, (PilotRnd)
		and	c
		add	a, b
		ld	b, a
		ld	a, e
		bit	7, d
		jr	nz, loc_E17B
		inc	a
		neg

loc_E17B:				; CODE XREF: GenerateAnotherPrice+2Ej
		add	a, b
		bit	7, a
		jr	z, loc_E181
		xor	a

loc_E181:				; CODE XREF: GenerateAnotherPrice+36j
		and	3Fh ; '?'
		ld	b, e
		pop	de
		push	af
		ld	a, (PilotRnd)
		and	c
		add	a, e
		ld	e, 0
		rl	e
		ld	c, a
		ld	a, b
		bit	7, d
		jr	z, loc_E19A
		ld	d, a
		ld	a, c
		sub	d
		jr	loc_E19E
; ---------------------------------------------------------------------------

loc_E19A:				; CODE XREF: GenerateAnotherPrice+4Bj
		add	a, c
		jr	nc, loc_E19E
		inc	e

loc_E19E:				; CODE XREF: GenerateAnotherPrice+50j
					; GenerateAnotherPrice+53j
		ld	b, e
		add	a, a
		rl	b
		add	a, a
		rl	b
		ld	c, a
		push	bc
		exx
		pop	bc
		ld	a, c
		ld	(de), a
		inc	de
		ld	a, b
		ld	(de), a
		inc	de
		pop	af

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


; === SetCargo === CODE XREF: CallFunctionObj_1+1Ej
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



; === SetPlayerCargo === CODE XREF: CallFunctionObj_1+28Ej
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



; === SetShipEPodCargo === CODE XREF: CallFunctionObj_1+28Cj
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



