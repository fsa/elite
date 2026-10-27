;13430: ; =============== S U B	R O U T	I N E =======================================
;13431: 
;13432: ; Печать индикаторов, запуск ракеты, бомбы
;13433: 
;13434: Process_MissBombEPod:			; CODE XREF: RAM:750Cp
;13435: 		ld	a, (WhatOnScreen)
;13436: 		cp	4
;13437: 		ret	nc
;13438: 		call	PrnDangerEcmDock
;13439: 		call	UseMissile
;13440: 		call	UseEscapePod
;13441: 		call	j_UseEnergyBomb
;13442: 		ld	hl, ControlKey3
;13443: 		ld	a, (hl)
;13444: 		and	10010001b
;13445: 		ld	(hl), a
;13446: ; End of function Process_MissBombEPod
;13447: 
;13448: 
;13449: ; =============== S U B	R O U T	I N E =======================================
;13450: 
;13451: 
;13452: Damage2Pilot:				; CODE XREF: sub_74C6+9p
;13453: 		ld	hl, IndDanger
;13454: 		xor	a
;13455: 		cp	(hl)
;13456: 		ret	z
;13457: 		ld	a, (ShieldUnit4)
;13458: 		cp	0A0h ; 'а'
;13459: 		jr	z, loc_9A6B
;13460: 		ld	a, 63h ; 'c'
;13461: 		ld	(IndDanger), a
;13462: 
;13463: loc_9A6B:				; CODE XREF: Damage2Pilot+Bj
;13464: 		call	EnergyRecovery
;13465: 		ld	hl, ShieldFront
;13466: 		ld	a, (DamageFrontShield)
;13467: 		call	sub_9AA7
;13468: 		ld	hl, ShieldRear
;13469: 		ld	a, (DamageRearShield)
;13470: 		call	sub_9AA7
;13471: 		ld	a, (CloakingDeviceOn)
;13472: 		and	a
;13473: 		ld	a, (DamageEnergyUnit)
;13474: 		jr	z, loc_9A8A
;13475: 		inc	a
;13476: 
;13477: loc_9A8A:				; CODE XREF: Damage2Pilot+2Ej
;13478: 		ld	c, a
;13479: 		ld	b, 4
;13480: ; End of function Damage2Pilot
;13481: 
;13482: 
;13483: ; =============== S U B	R O U T	I N E =======================================
;13484: 
;13485: 
;13486: sub_9A8D:				; CODE XREF: sub_9AA7+23p
;13487: 		ld	hl, ShieldUnit1
;13488: 		ld	d, 4
;13489: 
;13490: loc_9A92:				; CODE XREF: sub_9A8D+Ej sub_9A8D+17j
;13491: 		ld	a, (hl)
;13492: 		sub	c
;13493: 		jr	nc, loc_9AA3
;13494: 		ld	(hl), 0
;13495: 		inc	hl
;13496: 		inc	hl
;13497: 		dec	d
;13498: 		jr	nz, loc_9A92
;13499: 		ld	a, 1
;13500: 		ld	(PlayerGameOver), a
;13501: 		ret
;13502: ; ---------------------------------------------------------------------------
;13503: 
;13504: loc_9AA3:				; CODE XREF: sub_9A8D+7j
;13505: 		ld	(hl), a
;13506: 		djnz	loc_9A92
;13507: 		ret
;13508: ; End of function sub_9A8D
;13509: 
;13510: 
;13511: ; =============== S U B	R O U T	I N E =======================================
;13512: 
;13513: 
;13514: sub_9AA7:				; CODE XREF: Damage2Pilot+1Bp
;13515: 					; Damage2Pilot+24p
;13516: 		ld	c, a
;13517: 		and	a
;13518: 		jr	nz, loc_9AB3
;13519: 		ld	hl, unk_9ADA
;13520: 		ld	a, (hl)
;13521: 		and	a
;13522: 		ret	z
;13523: 		dec	(hl)
;13524: 		ret
;13525: ; ---------------------------------------------------------------------------
;13526: 
;13527: loc_9AB3:				; CODE XREF: sub_9AA7+2j
;13528: 		push	bc
;13529: 		push	hl
;13530: 		cp	46h ; 'F'
;13531: 		call	nc, sub_D0F3
;13532: 		pop	hl
;13533: 		pop	bc
;13534: 		ld	b, 4
;13535: 
;13536: loc_9ABE:				; CODE XREF: sub_9AA7+1Cj
;13537: 		ld	a, (hl)
;13538: 		sub	c
;13539: 		jr	c, loc_9AC6
;13540: 		ld	(hl), a
;13541: 		djnz	loc_9ABE
;13542: 		ret
;13543: ; ---------------------------------------------------------------------------
;13544: 
;13545: loc_9AC6:				; CODE XREF: sub_9AA7+19j
;13546: 		ld	(hl), 0
;13547: 		dec	b
;13548: 		ret	z
;13549: 		call	sub_9A8D
;13550: 		ld	hl, 9ADAh
;13551: 		ld	a, (hl)
;13552: 		add	a, 5
;13553: 		ld	(hl), a
;13554: 		cp	0Dh
;13555: 		ret	c
;13556: 		jp	sub_D0F3
;13557: ; End of function sub_9AA7
;13558: 
;13559: ; ---------------------------------------------------------------------------
;13560: unk_9ADA:	db    0			; DATA XREF: sub_9AA7+4o
;13561: 
;13562: ; =============== S U B	R O U T	I N E =======================================
;13563: 
;13564: 
;13565: EnergyRecovery:				; CODE XREF: Damage2Pilot:loc_9A6Bp
;13566: 		ld	a, (byte_D08B)
;13567: 		and	a
;13568: 		call	nz, loc_8481
;13569: ; End of function EnergyRecovery
;13570: 
;13571: 
;13572: ; =============== S U B	R O U T	I N E =======================================
;13573: 
;13574: 
;13575: sub_9AE2:				; CODE XREF: RAM:8482p
;13576: 		ld	hl, ShieldUnit4
;13577: 		ld	b, 4
;13578: 
;13579: loc_9AE7:				; CODE XREF: sub_9AE2+Ej
;13580: 		ld	a, (hl)
;13581: 		cp	0A0h ; 'а'
;13582: 		jr	z, loc_9AEE
;13583: 		inc	(hl)
;13584: 		ret
;13585: ; ---------------------------------------------------------------------------
;13586: 
;13587: loc_9AEE:				; CODE XREF: sub_9AE2+8j
;13588: 		dec	hl
;13589: 		dec	hl
;13590: 		djnz	loc_9AE7
;13591: 		ld	hl, ShieldFront
;13592: 		call	sub_9AFB
;13593: 		ld	hl, ShieldRear
;13594: ; End of function sub_9AE2
;13595: 
;13596: 
;13597: ; =============== S U B	R O U T	I N E =======================================
;13598: 
;13599: 
;13600: sub_9AFB:				; CODE XREF: sub_9AE2+13p
;13601: 		ld	a, (hl)
;13602: 		cp	0A0h ; 'а'
;13603: 		ret	z
;13604: 		inc	(hl)
;13605: 		ret
;13606: ; End of function sub_9AFB
;13607: 
;13608: 
;13609: ; =============== S U B	R O U T	I N E =======================================
;13610: 
;13611: 
;13612: PrnDangerEcmDock:			; CODE XREF: Process_MissBombEPod+6p
;13613: 		ld	a, (WhatOnScreen)
;13614: 		cp	4
;13615: 		ret	nc
;13616: 		ld	hl, 5AD5h
;13617: 		ld	a, (TimeWorkECM)
;13618: 		call	sub_9B14
;13619: 		inc	hl
;13620: 		ld	a, (CoriolisNear)
;13621: ; End of function PrnDangerEcmDock
;13622: 
;13623: 
;13624: ; =============== S U B	R O U T	I N E =======================================
;13625: 
;13626: 
;13627: sub_9B14:				; CODE XREF: PrnDangerEcmDock+Cp
;13628: 		and	a
;13629: 		jr	z, loc_9B19
;13630: 		ld	a, 7
;13631: 
;13632: loc_9B19:				; CODE XREF: sub_9B14+1j
;13633: 		ld	(hl), a
;13634: 		ret
;13635: ; End of function sub_9B14
;13636: 
;13637: 
;13638: ; =============== S U B	R O U T	I N E =======================================
;13639: 
;13640: 
;13641: PrintMissiles:				; CODE XREF: j_PrintMissilesj
;13642: 		ld	hl, 1702h
;13643: 		ld	(CrdXPrnSymbol), hl
;13644: 		ld	hl, (word_D04B)
;13645: 		ld	(hl), 0
;13646: 		push	hl
;13647: 		ld	a, (PilotMissiles)
;13648: 		ld	c, a
;13649: 		ld	b, 4
;13650: 
