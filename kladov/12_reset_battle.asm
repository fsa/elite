;27630: loc_DB8E:				; CODE XREF: sub_DAB4+AAj sub_DAB4+B1j ...
;27631: 		jp	loc_DAE1
;27632: ; END OF FUNCTION CHUNK	FOR sub_DAB4
;27633: 
;27634: ; =============== S U B	R O U T	I N E =======================================
;27635: 
;27636: 
;27637: ResetBattleMessages:			; CODE XREF: j_ResetBattleMessagesj
;27638: 					; InitGamep ...
;27639: 		ld	b, 0Ah
;27640: 		ld	hl, MissileJammed
;27641: 
;27642: loc_DB96:				; CODE XREF: ResetBattleMessages+9j
;27643: 		res	7, (hl)
;27644: 		inc	hl
;27645: 		inc	hl
;27646: 		djnz	loc_DB96
;27647: 		xor	a
;27648: 		ld	(msgNumber), a
;27649: 		ret
;27650: ; End of function ResetBattleMessages
;27651: 
;27652: ; ---------------------------------------------------------------------------
;27653: JumpCountdown:	db 0			; DATA XREF: InitGame+40w GoJump+Cr ...
;27654: 
;27655: ; =============== S U B	R O U T	I N E =======================================
;27656: 
;27657: 
;27658: GoJump:					; CODE XREF: j_GoJumpj
;27659: 		ld	hl, ControlKey2
;27660: 		ld	a, (hl)
;27661: 		and	30h ; '0'
;27662: 		ret	z
;27663: 		ld	b, a
;27664: 		ld	a, (hl)
;27665: 		and	0CFh ; '╧'
;27666: 		ld	(hl), a
;27667: 		ld	a, (JumpCountdown)
;27668: 		and	a
;27669: 		ret	nz
;27670: 		ld	a, (IndDanger)
;27671: 		and	a
;27672: 		jr	z, JumpDocked
;27673: 		bit	5, b
;27674: 		jr	z, loc_DBCF
;27675: 		bit	4, b
;27676: 		ret	z
;27677: 		ld	a, (PilotGyperdrive)
;27678: 		and	a
;27679: 		ret	z
;27680: 		ld	a, 2
;27681: 		ld	(JumpDocking), a ; =1 межпланетный прыжок/стыковка
;27682: 					; =2 межгалактический прыжок
;27683: 		ld	bc, 7406h
;27684: 		jr	loc_DBEB
;27685: ; ---------------------------------------------------------------------------
;27686: 
;27687: loc_DBCF:				; CODE XREF: GoJump+19j
;27688: 		call	SearchNearPlanet
;27689: 		ld	hl, (FuelToPlanet)
;27690: 		ld	a, h
;27691: 		and	a
;27692: 		jr	nz, loc_DC04
;27693: 		ld	a, l
;27694: 		and	a
;27695: 		ret	z
;27696: 		ld	b, a
;27697: 		ld	a, (Pilot_Fuel)
;27698: 		sub	b
;27699: 		jr	c, loc_DC04
;27700: 		ld	a, 1
;27701: 		ld	(JumpDocking), a ; =1 межпланетный прыжок/стыковка
;27702: 					; =2 межгалактический прыжок
;27703: 		ld	bc, 0C005h
;27704: 
;27705: loc_DBEB:				; CODE XREF: GoJump+2Bj
;27706: 		ld	a, 15h
;27707: 		ld	(JumpCountdown), a
;27708: 		ld	(msgTimeLife), a
;27709: 		call	sub_DAE6
;27710: 		call	sub_E1E8
;27711: ; ---------------------------------------------------------------------------
;27712: 		db  3Ah	; :
;27713: 		db 0E3h	; у
;27714: 		db  70h	; p
;27715: 		db 0FEh	; ■
;27716: 		db    4
;27717: 		db 0D0h	; ╨
;27718: 		db 0AFh	; п
;27719: 		db  32h	; 2
;27720: 		db 0E3h	; у
;27721: 		db  70h	; p
;27722: 		db 0C9h	; ╔
;27723: ; ---------------------------------------------------------------------------
;27724: 
;27725: loc_DC04:				; CODE XREF: GoJump+35j GoJump+3Fj
;27726: 		ld	bc, byte_C106
;27727: 		jp	loc_DADC
;27728: ; ---------------------------------------------------------------------------
;27729: 
;27730: JumpDocked:				; CODE XREF: GoJump+15j
;27731: 		call	ClsAndPrintRamk
;27732: 		ld	hl, 70Bh
;27733: 		ld	(CrdXPrnSymbol), hl
;27734: 		ld	a, 1
;27735: 		ld	(FlgPrnSym1), a
;27736: 		ld	a, 0CDh	; '═'
;27737: 		call	PrintMessage2
;27738: 		call	sub_E1E8
;27739: ; End of function GoJump
;27740: 
;27741: ; ---------------------------------------------------------------------------
;27742: 		db 0CDh	; ═
;27743: 		db  5Eh	; ^
;27744: 		db 0E3h	; у
;27745: 		db 0CDh	; ═
;27746: 		db    5
;27747: 		db 0E2h	; т
;27748: 		db 0C3h	; ├
;27749: 		db  87h	; З
;27750: 		db 0EFh	; я
