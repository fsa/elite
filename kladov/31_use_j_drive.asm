; 7205: 
; 7206: ; =============== S U B	R O U T	I N E =======================================
; 7207: 
; 7208: 
; 7209: Use_J_Drive:				; CODE XREF: sub_728D:loc_74B6p
; 7210: 		ld	a, (ControlKey2)
; 7211: 		bit	6, a
; 7212: 		push	af
; 7213: 		and	10111111b
; 7214: 		ld	(ControlKey2), a
; 7215: 		pop	af
; 7216: 		ret	z
; 7217: 		ld	a, (InFreeSpace)
; 7218: 		and	a
; 7219: 		jr	nz, loc_7EFB
; 7220: 		ld	hl, (J_Drive_Ready2)
; 7221: 		ld	a, (J_Drive_Coriolis)
; 7222: 		inc	a
; 7223: 		or	h
; 7224: 		or	l
; 7225: 		ret	nz
; 7226: 
; 7227: loc_7EFB:				; CODE XREF: Use_J_Drive+11j
; 7228: 		ld	a, (J_Drive_Ready4)
; 7229: 		and	a
; 7230: 		ret	nz
; 7231: 		ld	(Mission2), a
; 7232: 		ld	ix, objCoriolis
; 7233: 		call	ClearAroundObj
; 7234: 
; 7235: loc_7F0A:				; DATA XREF: Use_J_Drive+30w
; 7236: 		ld	a, 55h ; 'U'
; 7237: 		nop
; 7238: 		rrca
; 7239: 		ld	(loc_7F0A+1), a
; 7240: 		ld	a, 1
; 7241: 		jr	c, loc_7F18
; 7242: 		ld	(FrameCounter),	a
; 7243: 
; 7244: loc_7F18:				; CODE XREF: Use_J_Drive+35j
; 7245: 		ld	(StarsFastScroll), a
; 7246: 		ld	ix, SunCrd
; 7247: 		call	sub_7F2D
; 7248: 		ld	ix, PlanetCrd
; 7249: 		call	sub_7F2D
; 7250: 		ld	ix, objCoriolis
; 7251: ; End of function Use_J_Drive
; 7252: 
; 7253: 
; 7254: ; =============== S U B	R O U T	I N E =======================================
; 7255: 
; 7256: 
; 7257: sub_7F2D:				; CODE XREF: Use_J_Drive+41p
; 7258: 					; Use_J_Drive+48p
; 7259: 		ld	bc, 20h	; ' '
; 7260: 		ld	h, (ix+8)
; 7261: 		ld	l, (ix+7)
; 7262: 		ld	d, l
; 7263: 		bit	7, h
; 7264: 		jr	nz, loc_7F42
; 7265: 		and	a
; 7266: 		sbc	hl, bc
; 7267: 		jr	c, loc_7F4C
; 7268: 		jr	loc_7F45
; 7269: ; ---------------------------------------------------------------------------
; 7270: 
; 7271: loc_7F42:				; CODE XREF: sub_7F2D+Cj
; 7272: 		add	hl, bc
; 7273: 		jr	c, loc_7F61
; 7274: 
; 7275: loc_7F45:				; CODE XREF: sub_7F2D+13j
; 7276: 		ld	(ix+8),	h
; 7277: 		ld	(ix+7),	l
; 7278: 		ret
; 7279: ; ---------------------------------------------------------------------------
; 7280: 
; 7281: loc_7F4C:				; CODE XREF: sub_7F2D+11j
; 7282: 		ld	hl, 2000h
; 7283: 		ld	e, (ix+6)
; 7284: 		and	a
; 7285: 		sbc	hl, de
; 7286: 		ld	a, 80h ; 'А'
; 7287: 		ld	(ix+8),	a
; 7288: 		ld	(ix+7),	h
; 7289: 		ld	(ix+6),	l
; 7290: 		ret
; 7291: ; ---------------------------------------------------------------------------
; 7292: 
; 7293: loc_7F61:				; CODE XREF: sub_7F2D+16j
; 7294: 		jp	sub_5FD5
; 7295: ; End of function sub_7F2D
; 7296: 
; 7297: 
; 7298: ; =============== S U B	R O U T	I N E =======================================
; 7299: 
; 7300: ; Установка станции таргонов в межпланетном пространстве для последней подмиссии в войне с таргонами
; 7301: 
; 7302: SetTargonStation:			; CODE XREF: sub_5FD5:loc_6028p
; 7303: 					; CallFunctionObj_1:Set3_4Targonsp
; 7304: 		call	TestTargonStation ; определение	возможности установки станции таргонов в межпланетном пространстве для последней подмиссии в войне с таргонами
; 7305: 					; Вх:  ---
; 7306: 					; Вых: cy =1 станция установлена
; 7307: 					;      hl,bc,a - не определены
; 7308: 		jp	nz, IntRnd_A
; 7309: 		ld	hl, objCoriolis
; 7310: 		ld	b, 8
; 7311: 
; 7312: loc_7F6F:				; CODE XREF: SetTargonStation+Ej
; 7313: 		ld	(hl), 0
; 7314: 		inc	hl
; 7315: 		djnz	loc_7F6F
; 7316: 		ld	(hl), 1
; 7317: 		ld	l, 6Ah ; 'j'
; 7318: 		ld	(hl), 0F0h ; 'Ё'
; 7319: 		ld	a, 4
; 7320: 		ld	(PilotMission3), a ; =#06 при стыковке будет захват таргонами (вроде нереальная	ситуация)
; 7321: 					; =#04 в системе со танцией таргонов во	время войны
; 7322: 		pop	hl
; 7323: 		ret
; 7324: ; End of function SetTargonStation
; 7325: 
; 7326: 
; 7327: ; =============== S U B	R O U T	I N E =======================================
; 7328: 
; 7329: 
; 7330: sub_7F81:				; CODE XREF: CalcNewCrdObj_Plp
; 7331: 		ld	a, (byte_D08E)	; <>0 есть Cobra Mk4
; 7332: 		and	a
; 7333: 		jr	z, loc_7F9D
; 7334: 		ld	a, 0BFh	; '┐'
; 7335: 		in	a, (0FEh)
; 7336: 		bit	3, a
; 7337: 		jr	nz, loc_7F9D
; 7338: 		ld	hl, HighAngleLR
; 7339: 
; 7340: loc_7F92:				; CODE XREF: sub_7F81+1Aj
; 7341: 		ld	a, (hl)
; 7342: 		add	a, a
; 7343: 		or	(hl)
; 7344: 		and	0BFh ; '┐'
; 7345: 		ld	(hl), a
; 7346: 		bit	1, l
; 7347: 		inc	hl
; 7348: 		jr	z, loc_7F92
; 7349: 
; 7350: loc_7F9D:				; CODE XREF: sub_7F81+4j sub_7F81+Cj
; 7351: 		ld	a, (HighAngleDU)
; 7352: 		ret
; 7353: ; End of function sub_7F81
; 7354: 
; 7355: 
; 7356: ; =============== S U B	R O U T	I N E =======================================
; 7357: 
; 7358: 
; 7359: sub_7FA1:				; CODE XREF: sub_E87E+39p
; 7360: 		bit	5, (hl)
; 7361: 		ret	z
; 7362: 		ld	a, (PilotTargonWar) ; =#81 при сбросе миссий
; 7363: 					; 7,=1 начало войны, предолжение участвовать
; 7364: 					; 6-0, чем больше число	тем дальше от левого края карты	захват планет таргонами
; 7365: 					;
; 7366: 					; =#01 война закончена
; 7367: 		sub	2
; 7368: 		ret	c
; 7369: 		ld	a, (PilotRating2) ; >=#84 в трюме десант
; 7370: 					; 7,=1 война была или идёт
; 7371: 					; 6,=1 война закончилать
; 7372: 					; 3-1 -	ранг (=текущая стадия войны)
; 7373: 		cp	84h ; 'Д'
; 7374: 		ccf
; 7375: 		push	hl
; 7376: 		call	nc, TestPilotInZoneCombat ; Определяем находится ли пилот в зоне #3C (активных боёв)
; 7377: 					; Вх:  ---
; 7378: 					; Вых: cy =1 пилот в зоне #3C
; 7379: 					;	  =0 пилот в другой галактике галактике
; 7380: 					;	     война окончена/не начиналась
; 7381: 					;	     пилот в другой зоне
; 7382: 					;      hl,de,b,a - не определены
; 7383: 		pop	hl
; 7384: 		jr	nc, loc_7FB9
; 7385: 		xor	a
; 7386: 		ret
; 7387: ; ---------------------------------------------------------------------------
; 7388: 
; 7389: loc_7FB9:				; CODE XREF: sub_7FA1+14j
; 7390: 		set	0, (hl)
; 7391: 		inc	h
; 7392: 		dec	h
; 7393: 		ret
; 7394: ; End of function sub_7FA1
; 7395: 
