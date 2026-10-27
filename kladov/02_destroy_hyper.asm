; 6980: 
; 6981: ; Attributes: noreturn
; 6982: 
; 6983: DestroyObj:				; CODE XREF: FireLaser+CEp
; 6984: 					; j_DestroyObjj
; 6985: 		bit	6, (ix+26h)
; 6986: 		jr	z, loc_7DDD
; 6987: 		ld	a, (Mission2)
; 6988: 		and	a
; 6989: 		jr	z, loc_7DDD
; 6990: 
; 6991: loc_7DD1:
; 6992: 		cp	3
; 6993: 		jr	z, loc_7DDD
; 6994: 		ld	a, 3
; 6995: 		ld	(Mission2), a
; 6996: 		add	a, a
; 6997: 		jr	loc_7DFF
; 6998: ; ---------------------------------------------------------------------------
; 6999: 
; 7000: loc_7DDD:				; CODE XREF: DestroyObj+4j
; 7001: 					; DestroyObj+Aj ...
; 7002: 		ld	h, (ix+24h)
; 7003: 		ld	l, (ix+23h)
; 7004: 		ld	a, (hl)
; 7005: 		and	0Fh
; 7006: 		ld	a, 6
; 7007: 		jr	nz, loc_7DFF
; 7008: 		ld	de, 7
; 7009: 		add	hl, de
; 7010: 		ld	a, (hl)
; 7011: 		cp	55h ; 'U'
; 7012: 		ld	a, 5
; 7013: 		jr	nc, loc_7DFF
; 7014: 		ld	a, (hl)
; 7015: 		cp	3Ch ; '<'
; 7016: 		ret	nz
; 7017: 		ld	a, c
; 7018: 		cp	32h ; '2'
; 7019: 		ret	nz
; 7020: 		ld	a, 8
; 7021: 
; 7022: loc_7DFF:				; CODE XREF: DestroyObj+16j
; 7023: 					; DestroyObj+23j ...
; 7024: 		jp	sub_B867
; 7025: ; End of function DestroyObj
; 7026: 
; 7027: 
; 7028: ; =============== S U B	R O U T	I N E =======================================
; 7029: 
; 7030: 
; 7031: sub_7E02:				; CODE XREF: PrnObjIfVisible+F7j
; 7032: 		jr	z, ShipHyperJump
; 7033: 
; 7034: objDestroyed:				; получение координат и	знаков проекции	объекта
; 7035: 		call	GetCrdsProjection
; 7036: 		call	SetCrdCDEinExtSlot
; 7037: 		jp	objDestroyed2
; 7038: ; ---------------------------------------------------------------------------
; 7039: 
; 7040: ShipHyperJump:				; CODE XREF: sub_7E02j
; 7041: 		ld	hl, (word_B452)
; 7042: 		ld	a, (hl)
; 7043: 		and	0F8h ; '°'
; 7044: 		cp	0A0h ; 'а'
; 7045: 		jr	z, loc_7E4E	; уход в гипертоннель
; 7046: 		bit	3, a
; 7047: 		jp	z, sub_A7D9	; если включена	невидимость и это ASP из 2й миссии, то печать только луча лазера
; 7048: 		call	GetCrdsProjection ; получение координат	и знаков проекции объекта
; 7049: 		call	SetCrdCDEinExtSlot
; 7050: 		inc	hl
; 7051: 		ld	a, (ix+20h)
; 7052: 		and	a
; 7053: 		jr	z, loc_7E9D
; 7054: 		ld	a, (ix+22h)
; 7055: 		ld	e, 78h ; 'x'
; 7056: 		cp	e
; 7057: 		jr	nc, loc_7E38
; 7058: 		ld	e, 38h ; '8'
; 7059: 		cp	e
; 7060: 		jr	nc, loc_7E38
; 7061: 		ld	e, 0Ch
; 7062: 
; 7063: loc_7E38:				; CODE XREF: sub_7E02+2Dj sub_7E02+32j
; 7064: 		ld	a, (hl)
; 7065: 
; 7066: loc_7E39:				; CODE XREF: sub_7E02+3Cj
; 7067: 		rrc	e
; 7068: 		jr	c, loc_7E40
; 7069: 		rrca
; 7070: 		jr	loc_7E39
; 7071: ; ---------------------------------------------------------------------------
; 7072: 
; 7073: loc_7E40:				; CODE XREF: sub_7E02+39j
; 7074: 		rrca
; 7075: 		inc	(ix+22h)
; 7076: 		jr	c, loc_7E9D
; 7077: 		dec	(ix+22h)
; 7078: 		dec	(ix+22h)
; 7079: 		jr	loc_7EA6
; 7080: ; ---------------------------------------------------------------------------
; 7081: 
; 7082: loc_7E4E:				; CODE XREF: sub_7E02+13j
; 7083: 		ld	a, (hl)
; 7084: 		and	111b
; 7085: 		ld	b, a
; 7086: 		ld	a, (RadiusBlast)
; 7087: 		sub	b
; 7088: 		jr	c, loc_7E5C
; 7089: 		cp	4
; 7090: 		jr	nc, loc_7E5E
; 7091: 
; 7092: loc_7E5C:				; CODE XREF: sub_7E02+54j
; 7093: 		ld	a, 4
; 7094: 
; 7095: loc_7E5E:				; CODE XREF: sub_7E02+58j
; 7096: 		ld	h, a
; 7097: 		ld	a, (WhatOnScreen)
; 7098: 		rrca
; 7099: 		ld	a, (CrdXYsign)
; 7100: 		jr	c, loc_7E6A
; 7101: 		xor	2
; 7102: 
; 7103: loc_7E6A:				; CODE XREF: sub_7E02+64j
; 7104: 		ld	c, a
; 7105: 		xor	a
; 7106: 		rr	c
; 7107: 		rla
; 7108: 		rr	c
; 7109: 		rla
; 7110: 		xor	0Eh
; 7111: 		ld	l, a
; 7112: 		ld	(CircleFlg), hl
; 7113: 		ld	hl, (CrdXProjection)
; 7114: 		ex	de, hl
; 7115: 		ld	l, d
; 7116: 		ld	h, e
; 7117: 		ld	(CircleCrdY), hl
; 7118: 		ld	c, b
; 7119: 		ld	a, b
; 7120: 		bit	2, a
; 7121: 		jr	z, loc_7E89
; 7122: 		xor	7
; 7123: 
; 7124: loc_7E89:				; CODE XREF: sub_7E02+83j
; 7125: 		ld	b, a
; 7126: 		inc	b
; 7127: 		inc	b
; 7128: 
; 7129: loc_7E8C:				; CODE XREF: sub_7E02+96j
; 7130: 		push	bc
; 7131: 		call	PrnCircle_NoFill
; 7132: 		ld	hl, CircleRadius
; 7133: 		ld	a, (hl)
; 7134: 		pop	bc
; 7135: 		add	a, b
; 7136: 		add	a, b
; 7137: 		ld	(hl), a
; 7138: 		djnz	loc_7E8C
; 7139: 		ld	a, c
; 7140: 		cp	3
; 7141: 
; 7142: loc_7E9D:				; CODE XREF: sub_7E02+25j sub_7E02+42j
; 7143: 		jp	c, sub_A7D9	; если включена	невидимость и это ASP из 2й миссии, то печать только луча лазера
; 7144: 		call	GetCrdsProjection ; получение координат	и знаков проекции объекта
; 7145: 		call	SetCrdCDEinExtSlot
; 7146: 
; 7147: loc_7EA6:				; CODE XREF: sub_7E02+4Aj
; 7148: 		jp	loc_A878
; 7149: ; End of function sub_7E02
; 7150: 
; 7151: 
; 7152: ; =============== S U B	R O U T	I N E =======================================
; 7153: 
; 7154: 
; 7155: CounterShipHyperJump:			; CODE XREF: sub_9D40:loc_9D50p
; 7156: 		ld	a, (FrameCounter)
; 7157: 		ld	c, a
; 7158: 		ld	hl, (word_B452)
; 7159: 		ld	a, (hl)
; 7160: 		and	0F8h ; '°'
; 7161: 		cp	0A0h ; 'а'
; 7162: 		ret	nz
; 7163: 		inc	(hl)
; 7164: 		ld	a, (hl)
; 7165: 		and	7
; 7166: 		ret	nz
; 7167: 		ld	(hl), a
; 7168: 		ld	(ix+22h), a
; 7169: 		pop	hl
; 7170: 		push	bc
; 7171: 		call	OffTargetMissiles
; 7172: 		pop	bc
; 7173: 		jp	loc_9E5C
; 7174: ; End of function CounterShipHyperJump
; 7175: 
; 7176: 
; 7177: ; =============== S U B	R O U T	I N E =======================================
; 7178: 
; 7179: ; поиск	объекта	в дополнительном слоте
; 7180: ; hl - адрес слота объекта
