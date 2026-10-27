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
; 7701: 
; 7702: loc_8104:				; CODE XREF: sub_80D3+21j
; 7703: 		call	sub_815D
; 7704: 		jr	loc_8136
; 7705: ; ---------------------------------------------------------------------------
; 7706: 
; 7707: loc_8109:				; CODE XREF: sub_80D3+10j
; 7708: 		ld	a, (IndDanger)
; 7709: 		and	a
; 7710: 		jr	nz, loc_8114
; 7711: 		call	LaunchDocking
; 7712: ; ---------------------------------------------------------------------------
; 7713: 		jr	loc_8136
; 7714: ; ---------------------------------------------------------------------------
; 7715: 
; 7716: loc_8114:				; CODE XREF: sub_80D3+3Aj
; 7717: 		call	sub_815D
; 7718: 		jr	loc_8136
; 7719: ; ---------------------------------------------------------------------------
; 7720: 
; 7721: loc_8119:				; CODE XREF: sub_80D3+Aj
; 7722: 		ld	a, (ControlKey1)
; 7723: 		and	3Fh ; '?'
; 7724: 		add	a, a
; 7725: 		ret	z
; 7726: 		add	a, a
; 7727: 		ld	bc, 607h
; 7728: 
; 7729: loc_8124:				; CODE XREF: sub_80D3+55j
; 7730: 		add	a, a
; 7731: 		jr	c, loc_812A
; 7732: 		inc	c
; 7733: 		djnz	loc_8124
; 7734: 
; 7735: loc_812A:				; CODE XREF: sub_80D3+52j
; 7736: 		call	sub_815D
; 7737: 		cp	7
; 7738: 		jr	c, loc_8136
; 7739: 		sub	4
; 7740: 		call	sub_813E
; 7741: 
; 7742: loc_8136:				; CODE XREF: sub_80D3+2Fj sub_80D3+34j ...
; 7743: 		xor	a
; 7744: 		ld	(ControlKey2), a
; 7745: 		ld	(ControlKey1), a
; 7746: 		ret
; 7747: ; End of function sub_80D3
; 7748: 
; 7749: 
; 7750: ; =============== S U B	R O U T	I N E =======================================
; 7751: 
; 7752: 
; 7753: sub_813E:				; CODE XREF: LaunchDocking:loc_5E65j
; 7754: 					; ControlKeyIfMapOn+23p ...
; 7755: 		ld	(word_80D1), sp
; 7756: 		nop
; 7757: 		nop
; 7758: 		nop
; 7759: 		ld	b, a
; 7760: 		add	a, a
; 7761: 		add	a, b
; 7762: 		ld	hl, FunctionKernel
; 7763: 		add	a, l
; 7764: 		ld	l, a
; 7765: 		jr	nc, loc_8150
; 7766: 		inc	h
; 7767: 
; 7768: loc_8150:				; CODE XREF: sub_813E+Fj
; 7769: 		ld	(loc_8153+1), hl
; 7770: 
; 7771: loc_8153:				; DATA XREF: sub_813E:loc_8150w
; 7772: 		call	sub_D021
; 7773: 		ld	sp, (word_80D1)
; 7774: 		jp	loc_94FE
; 7775: ; End of function sub_813E
; 7776: 
; 7777: 
; 7778: ; =============== S U B	R O U T	I N E =======================================
; 7779: 
; 7780: 
; 7781: sub_815D:				; CODE XREF: sub_80D3+27p
; 7782: 					; sub_80D3:loc_8104p ...
; 7783: 
; 7784: ; FUNCTION CHUNK AT 819A SIZE 00000009 BYTES
; 7785: 
; 7786: 		ld	a, (WhatOnScreen)
; 7787: 		ld	b, a
; 7788: 		cp	c
; 7789: 		jr	nz, loc_816C
; 7790: 		ld	a, c
; 7791: 		cp	7
; 7792: 		jr	c, loc_816C
; 7793: 		pop	hl
; 7794: 		jr	loc_8136
; 7795: ; ---------------------------------------------------------------------------
; 7796: 
; 7797: loc_816C:				; CODE XREF: sub_815D+5j sub_815D+Aj
; 7798: 		ld	a, (JumpDocking) ; =1 межпланетный прыжок/стыковка
; 7799: 					; =2 межгалактический прыжок
; 7800: 		and	a
; 7801: 		ld	a, c
; 7802: 		jr	nz, loc_819A
; 7803: 
; 7804: loc_8173:				; CODE XREF: sub_815D+3Fj
; 7805: 		ld	(WhatOnScreen),	a
; 7806: 		ld	a, b
; 7807: 		cp	4
; 7808: 		ld	a, c
; 7809: 		ret	nc
; 7810: 		cp	4
; 7811: 		ret	c
; 7812: ; End of function sub_815D
; 7813: 
; 7814: 
; 7815: ; =============== S U B	R O U T	I N E =======================================
; 7816: 
; 7817: 
; 7818: SaveCocpitInBuffer:			; CODE XREF: sub_728D+18Ep
; 7819: 					; LaunchDocking+5Fp
; 7820: 		push	af
