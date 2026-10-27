; 8580: 
; 8581: ; =============== S U B	R O U T	I N E =======================================
; 8582: 
; 8583: 
; 8584: UpdateAllIndicators:			; CODE XREF: RAM:7505p
; 8585: 					; LaunchDocking+D0p
; 8586: 		ld	de, word_8524
; 8587: 		exx
; 8588: 		ld	c, 0Bh
; 8589: 		exx
; 8590: 		ld	hl, ShieldFront
; 8591: 
; 8592: loc_84E3:				; CODE XREF: UpdateAllIndicators+23j
; 8593: 		ld	a, (hl)
; 8594: 		dec	hl
; 8595: 		cp	(hl)
; 8596: 		jr	z, loc_84F4
; 8597: 		ld	(hl), a
; 8598: 		srl	a
; 8599: 		srl	a
; 8600: 		push	hl
; 8601: 		push	de
; 8602: 		call	sub_854F
; 8603: 		pop	de
; 8604: 		pop	hl
; 8605: 
; 8606: loc_84F4:				; CODE XREF: UpdateAllIndicators+Dj
; 8607: 		inc	de
; 8608: 		inc	de
; 8609: 		inc	hl
; 8610: 		inc	hl
; 8611: 		inc	hl
; 8612: 		exx
; 8613: 		dec	c
; 8614: 		exx
; 8615: 		jr	nz, loc_84E3
; 8616: 		ld	a, 1
; 8617: 		ld	(byte_8523), a
; 8618: 		exx
; 8619: 		ld	c, 2
; 8620: 		exx
; 8621: 
; 8622: loc_8507:				; CODE XREF: UpdateAllIndicators+43j
; 8623: 		ld	a, (hl)
; 8624: 		dec	hl
; 8625: 		cp	(hl)
; 8626: 		jr	z, loc_8514
; 8627: 		ld	(hl), a
; 8628: 		push	hl
; 8629: 		push	de
; 8630: 		call	sub_8540
; 8631: 		pop	de
; 8632: 		pop	hl
; 8633: 
; 8634: loc_8514:				; CODE XREF: UpdateAllIndicators+31j
; 8635: 		inc	de
; 8636: 		inc	de
; 8637: 		inc	hl
; 8638: 		inc	hl
; 8639: 		inc	hl
; 8640: 		exx
; 8641: 		dec	c
; 8642: 		exx
; 8643: 		jr	nz, loc_8507
; 8644: 		xor	a
; 8645: 		ld	(byte_8523), a
; 8646: 		ret
; 8647: ; End of function UpdateAllIndicators
; 8648: 
; 8649: ; ---------------------------------------------------------------------------
; 8650: byte_8523:	db 0			; DATA XREF: UpdateAllIndicators+27w
; 8651: 					; UpdateAllIndicators+46w ...
; 8652: word_8524:	dw 5123h		; DATA XREF: UpdateAllIndicatorso
; 8653: 		dw 5143h
; 8654: 		dw 5163h
; 8655: 		dw 5183h
; 8656: 		dw 51A3h
; 8657: 		dw 51C3h
; 8658: 		dw 5138h
; 8659: 		dw 5198h
; 8660: 		dw 51B8h
; 8661: 		dw 51D8h
; 8662: 		dw 51F8h
; 8663: 		dw 5158h
; 8664: 		dw 5178h
; 8665: 		dw 51E3h
; 8666: 
; 8667: ; =============== S U B	R O U T	I N E =======================================
; 8668: 
; 8669: 
; 8670: sub_8540:				; CODE XREF: UpdateAllIndicators+36p
; 8671: 		ld	b, a
; 8672: 		xor	a
; 8673: 		ld	(loc_8596+1), a
; 8674: 		ld	(loc_85B8+1), a
; 8675: 		ld	a, 3
; 8676: 		ld	(loc_856C+1), a
; 8677: 		jr	loc_855B
; 8678: ; End of function sub_8540
; 8679: 
; 8680: 
; 8681: ; =============== S U B	R O U T	I N E =======================================
; 8682: 
; 8683: 
; 8684: sub_854F:				; CODE XREF: UpdateAllIndicators+16p
; 8685: 		ld	b, a
; 8686: 		ld	a, 0FFh
; 8687: 		ld	(loc_856C+1), a
; 8688: 		ld	(loc_8596+1), a
; 8689: 		ld	(loc_85B8+1), a
; 8690: 
; 8691: loc_855B:				; CODE XREF: sub_8540+Dj
; 8692: 		ld	a, (de)
; 8693: 		ld	l, a
; 8694: 		inc	de
; 8695: 		ld	a, (de)
; 8696: 		ld	h, a
; 8697: 		exx
; 8698: 		ld	b, 5
; 8699: 		exx
; 8700: 		ld	a, b
; 8701: 		and	a
; 8702: 		jp	z, loc_85E1
; 8703: 		ld	d, a
; 8704: 		and	7
; 8705: 
; 8706: loc_856C:				; DATA XREF: sub_8540+Aw sub_854F+3w
; 8707: 		ld	c, 0FFh
; 8708: 		ld	b, a
; 8709: 		ld	a, d
; 8710: 		jr	z, loc_8582
; 8711: 		ld	a, (byte_8523)
; 8712: 		dec	a
; 8713: 		jp	z, loc_85F5
; 8714: 		xor	a
; 8715: 
; 8716: loc_857A:				; CODE XREF: sub_854F+2Dj
; 8717: 		scf
; 8718: 		rra
; 8719: 		djnz	loc_857A
; 8720: 
; 8721: loc_857E:				; CODE XREF: sub_854F:loc_8603j
; 8722: 		ld	c, a
; 8723: 		ld	a, d
; 8724: 		add	a, 8
; 8725: 
; 8726: loc_8582:				; CODE XREF: sub_854F+21j
; 8727: 		and	0F8h ; '°'
; 8728: 		srl	a
; 8729: 		srl	a
; 8730: 		srl	a
