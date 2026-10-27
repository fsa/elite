; 7575: ; End of function PrnCocpit_Ifn
; 7576: 
; 7577: 
; 7578: ; =============== S U B	R O U T	I N E =======================================
; 7579: 
; 7580: 
; 7581: ClearScrBuffer:				; CODE XREF: sub_728D:loc_73C3p
; 7582: 					; sub_728D+25Ap ...
; 7583: 
; 7584: ; FUNCTION CHUNK AT DA5F SIZE 00000005 BYTES
; 7585: 
; 7586: 		di
; 7587: 		ld	hl, 0CFE0h
; 7588: 		ld	de, 0C020h
; 7589: 		ld	bc, 5720h
; 7590: 
; 7591: loc_808A:				; CODE XREF: ClearScrBuffer+10j
; 7592: 		dec	l
; 7593: 		dec	e
; 7594: 		dec	c
; 7595: 		ld	a, (bc)
; 7596: 		ld	(hl), a
; 7597: 		ld	(de), a
; 7598: 		jr	nz, loc_808A
; 7599: 		ld	(loc_DA5F+1), sp
; 7600: 		ld	sp, hl
; 7601: 		ex	de, hl
; 7602: 		add	hl, hl
; 7603: 		ld	e, l
; 7604: 		ld	d, l
; 7605: 		ld	b, 7Dh ; '}'
; 7606: 
; 7607: loc_809D:				; CODE XREF: ClearScrBuffer+2Fj
; 7608: 		inc	e
; 7609: 		push	de
; 7610: 		dec	e
; 7611: 		push	de
; 7612: 		push	de
; 7613: 		push	de
; 7614: 		push	de
; 7615: 		push	de
; 7616: 		push	de
; 7617: 		push	de
; 7618: 		push	de
; 7619: 		push	de
; 7620: 		push	de
; 7621: 		push	de
; 7622: 		push	de
; 7623: 		push	de
; 7624: 		push	de
; 7625: 		push	hl
; 7626: 		djnz	loc_809D
; 7627: 		jp	loc_DA5F
; 7628: ; End of function ClearScrBuffer
; 7629: 
; 7630: ; ---------------------------------------------------------------------------
; 7631: 
; 7632: loc_80B4:				; CODE XREF: sub_EB2F+4p
; 7633: 		cp	0Eh
; 7634: 		jr	nc, loc_80BC
; 7635: 		ld	b, a
; 7636: 		add	a, a
; 7637: 		add	a, b
; 7638: 		ret
; 7639: ; ---------------------------------------------------------------------------
; 7640: 
; 7641: loc_80BC:				; CODE XREF: RAM:80B6j
; 7642: 		ld	hl, 667h
; 7643: 		xor	a
; 7644: 		ret
; 7645: ; ---------------------------------------------------------------------------
; 7646: 
; 7647: SetMaxTonnage:				; CODE XREF: sub_5EC5+1Bp
; 7648: 					; UseEscapePod+3Fp ...
; 7649: 		ld	a, (PilotLargeCargo) ; установка максимальной грузоподъемности
; 7650: 		ld	b, a
; 7651: 		dec	b
; 7652: 		ld	a, 5
; 7653: 
; 7654: loc_80C8:				; CODE XREF: RAM:80CBj
; 7655: 		add	a, 0Fh
; 7656: 		inc	b
; 7657: 		jr	nz, loc_80C8
; 7658: 		ld	(PilotFreeCargo), a
; 7659: 		ret
; 7660: ; ---------------------------------------------------------------------------
; 7661: word_80D1:	dw 411Ah		; DATA XREF: sub_813Ew	sub_813E+18r
; 7662: 
; 7663: ; =============== S U B	R O U T	I N E =======================================
; 7664: 
; 7665: ; проверка нажатия (и соответствующий переход) клавиш 1, 2, 3, 4, I, O,	P, K, L, enter
; 7666: 
; 7667: sub_80D3:				; CODE XREF: sub_728D:loc_747Ap
; 7668: 		ld	a, (FlgJumpDocking) ; =1 запрет	на обработку нажатий клавиш
; 7669: 					;   включено при: стыковке
; 7670: 					;   работает п/п отрисовки концентрический кругов
; 7671: 					;
; 7672: 					; 0,Flg_Interrupts
; 7673: 		and	a
; 7674: 		ret	nz
; 7675: 		ld	a, (ControlKey2)
; 7676: 		and	0Fh
; 7677: 		jr	z, loc_8119
; 7678: 		ld	c, 0
; 7679: 		srl	a
; 7680: 		jr	c, loc_8109
; 7681: 		inc	c
; 7682: 		srl	a
; 7683: 		jr	c, loc_80F0
; 7684: 		inc	c
; 7685: 		srl	a
; 7686: 		jr	c, loc_80F0
; 7687: 		inc	c
; 7688: 
; 7689: loc_80F0:				; CODE XREF: sub_80D3+15j sub_80D3+1Aj
; 7690: 		ld	a, (IndDanger)
; 7691: 		and	a
; 7692: 		jr	nz, loc_8104
; 7693: 		ld	a, 3
; 7694: 		add	a, c
; 7695: 		ld	c, a
; 7696: 		call	sub_815D
; 7697: 		sub	4
; 7698: 		call	sub_813E
; 7699: 		jr	loc_8136
; 7700: ; ---------------------------------------------------------------------------
