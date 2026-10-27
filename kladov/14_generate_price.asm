;28830: ; End of function GenerateProductPrice
;28831: 
;28832: 
;28833: ; =============== S U B	R O U T	I N E =======================================
;28834: 
;28835: 
;28836: GenerateAnotherPrice:			; CODE XREF: InitGame+23p
;28837: 		call	sub_9B83
;28838: 		ld	de, ProductPrice
;28839: 		exx
;28840: 		ld	hl, TableBasePrice ; +#28 Fireams
;28841: 		ld	b, 11h
;28842: 
;28843: loc_E154:				; CODE XREF: GenerateAnotherPrice+6Cj
;28844: 		push	bc
;28845: 		ld	e, (hl)
;28846: 		inc	hl
;28847: 		ld	a, (hl)
;28848: 		ld	d, a
;28849: 		push	de
;28850: 		and	1Fh
;28851: 		jr	z, loc_E167
;28852: 		ld	b, a
;28853: 		ld	a, (PlanetEconomy)
;28854: 		ld	c, a
;28855: 		xor	a
;28856: 
;28857: loc_E164:				; CODE XREF: GenerateAnotherPrice+1Dj
;28858: 		add	a, c
;28859: 		djnz	loc_E164
;28860: 
;28861: loc_E167:				; CODE XREF: GenerateAnotherPrice+14j
;28862: 		ld	e, a
;28863: 		inc	hl
;28864: 		ld	b, (hl)
;28865: 		inc	hl
;28866: 		ld	c, (hl)
;28867: 		inc	hl
;28868: 		ld	a, (PilotRnd)
;28869: 		and	c
;28870: 		add	a, b
;28871: 		ld	b, a
;28872: 		ld	a, e
;28873: 		bit	7, d
;28874: 		jr	nz, loc_E17B
;28875: 		inc	a
;28876: 		neg
;28877: 
;28878: loc_E17B:				; CODE XREF: GenerateAnotherPrice+2Ej
;28879: 		add	a, b
;28880: 		bit	7, a
;28881: 		jr	z, loc_E181
;28882: 		xor	a
;28883: 
;28884: loc_E181:				; CODE XREF: GenerateAnotherPrice+36j
;28885: 		and	3Fh ; '?'
;28886: 		ld	b, e
;28887: 		pop	de
;28888: 		push	af
;28889: 		ld	a, (PilotRnd)
;28890: 		and	c
;28891: 		add	a, e
;28892: 		ld	e, 0
;28893: 		rl	e
;28894: 		ld	c, a
;28895: 		ld	a, b
;28896: 		bit	7, d
;28897: 		jr	z, loc_E19A
;28898: 		ld	d, a
;28899: 		ld	a, c
;28900: 		sub	d
;28901: 		jr	loc_E19E
;28902: ; ---------------------------------------------------------------------------
;28903: 
;28904: loc_E19A:				; CODE XREF: GenerateAnotherPrice+4Bj
;28905: 		add	a, c
;28906: 		jr	nc, loc_E19E
;28907: 		inc	e
;28908: 
;28909: loc_E19E:				; CODE XREF: GenerateAnotherPrice+50j
;28910: 					; GenerateAnotherPrice+53j
;28911: 		ld	b, e
;28912: 		add	a, a
;28913: 		rl	b
;28914: 		add	a, a
;28915: 		rl	b
;28916: 		ld	c, a
;28917: 		push	bc
;28918: 		exx
;28919: 		pop	bc
;28920: 		ld	a, c
;28921: 		ld	(de), a
;28922: 		inc	de
;28923: 		ld	a, b
;28924: 		ld	(de), a
;28925: 		inc	de
;28926: 		pop	af
;28927: 
;28928: Amount:					; DATA XREF: InitGame+1Do
;28929: 		ld	(hl), a
;28930: 		inc	hl
;28931: 		exx
;28932: 		pop	bc
;28933: 		djnz	loc_E154
;28934: 		ld	hl, planetAlienItems
;28935: 		ld	(hl), b
;28936: 		ret
;28937: ; End of function GenerateAnotherPrice
;28938: 
;28939: ; ---------------------------------------------------------------------------
;28940: byte_E1BB:	db 0			; DATA XREF: sub_E962+6o sub_EF48+25o
;28941: 		db 0
;28942: ProductPrice:	dw  24h			; DATA XREF: GenerateAnotherPrice+3o
;28943: 		dw  3Ch
;28944: 		dw 0C8h
;28945: 		dw  3Ch
;28946: 		dw 0E8h
;28947: 		dw 3B0h
;28948: 		dw 5F0h
;28949: 		dw 380h
;28950: 		dw 24Ch
;28951: 		dw 14Ch
;28952: 		dw    0
;28953: 		dw 20Ch
;28954: 		dw  6Ch
;28955: 		dw 170h
;28956: 		dw 284h
;28957: 		dw 0A0h
;28958: 		dw 200h
;28959: byte_E1DF:	db 0			; DATA XREF: sub_E212r	RAM:E273r ...
;28960: word_E1E0:	dw 0			; DATA XREF: sub_700Fr
;28961: 					; UseEscapePod+49r ...
;28962: word_E1E2:	dw 0			; DATA XREF: sub_E212+13r sub_E212+18w ...
;28963: word_E1E4:	dw 0			; DATA XREF: sub_E212+2Cr sub_E212+31w ...
;28964: word_E1E6:	dw 0			; DATA XREF: sub_E364+3w sub_E38Dr
;28965: 
;28966: ; =============== S U B	R O U T	I N E =======================================
;28967: 
;28968: ; Attributes: noreturn
;28969: 
;28970: sub_E1E8:				; CODE XREF: RAM:5FC9p	RAM:loc_D114p ...
;28971: 		ld	a, (FlagsSetUp)
;28972: 		bit	7, a
;28973: 		ret	z
;28974: 		ld	hl, 80h	; 'А'
;28975: 		ld	de, 60h	; '`'
;28976: 		jp	3B5h
;28977: ; End of function sub_E1E8
;28978: 
;28979: 
;28980: ; =============== S U B	R O U T	I N E =======================================
;28981: 
;28982: 
;28983: sub_E1F7:				; CODE XREF: RAM:5FC3p	RAM:D117p ...
;28984: 		call	sub_E35A
;28985: 		call	ResetPressedKeys
;28986: 
;28987: loc_E1FD:				; CODE XREF: sub_E1F7+Bj
;28988: 		ld	a, (PressKeyPrnSymbol)
;28989: 		cp	0FFh
;28990: 		jr	z, loc_E1FD
;28991: 		ret
;28992: ; End of function sub_E1F7
;28993: 
;28994: 
;28995: ; =============== S U B	R O U T	I N E =======================================
;28996: 
;28997: 
;28998: ResetPressedKeys:			; CODE XREF: RAM:D120j	sub_E1F7+3p ...
;28999: 		di
;29000: 		ld	hl, ControlKey1
