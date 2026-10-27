;17140: 		or	(ix+2)
;17141: 		ld	b, 0FFh
;17142: 		scf
;17143: 
;17144: loc_ACB8:				; CODE XREF: MakeTablPrnCrdVertex+2F1j
;17145: 		rla
;17146: 		inc	b
;17147: 		jr	nc, loc_ACB8
;17148: 		ld	a, b
;17149: 		ld	(VertexIncFactor), a
;17150: 		and	a
;17151: 		jp	z, RotateCrdVertex
;17152: 		call	RotateCrdN_Vertex
;17153: 		ld	(VertexCrdXlow), a
;17154: 		ld	(VertexCrdX), hl
;17155: 		call	RotateCrdN_Vertex
;17156: 		ld	(VertexCrdYlow), a
;17157: 		ld	(VertexCrdY), hl
;17158: 		call	RotateCrdN_Vertex
;17159: 		ld	(VertexCrdZlow), a
;17160: 		ld	(VertexCrdZ), hl
;17161: 
;17162: loc_ACDF:				; CODE XREF: MakeTablPrnCrdVertex+1E3j
;17163: 		xor	a
;17164: 		ld	(VisiblePrevVertex), a
;17165: 		pop	iy
;17166: 		pop	hl
;17167: 		ex	(sp), ix
;17168: 		push	hl
;17169: 		ld	l, (ix+0)
;17170: 		ld	h, (ix+1)
;17171: 		ld	b, (ix+2)
;17172: 		ld	de, (VertexCrdX)
;17173: 		ld	a, (VertexCrdXlow)
;17174: 		rla
;17175: 		push	af
;17176: 		call	AddCrdNobj_CrdNvertex
;17177: 		ld	(VertexCrdX), hl
;17178: 		ld	(VertexSignCrdX), a
;17179: 		pop	af
;17180: 		ld	(VertexCrdXlow), a
;17181: 		ld	l, (ix+3)
;17182: 		ld	h, (ix+4)
;17183: 		ld	b, (ix+5)
;17184: 		ld	de, (VertexCrdY)
;17185: 		ld	a, (VertexCrdYlow)
;17186: 		rla
;17187: 		push	af
;17188: 		call	AddCrdNobj_CrdNvertex
;17189: 		ld	(VertexCrdY), hl
;17190: 		ld	(VertexSignCrdY), a
;17191: 		pop	af
;17192: 		ld	(VertexCrdYlow), a
;17193: 		ld	l, (ix+6)
;17194: 		ld	h, (ix+7)
;17195: 		ld	b, (ix+8)
;17196: 		ld	de, (VertexCrdZ)
;17197: 		ld	a, (VertexCrdZlow)
;17198: 		rla
;17199: 		push	af
;17200: 		call	AddCrdNobj_CrdNvertex
;17201: 		ld	(VertexCrdZ), hl
;17202: 		ld	(VertexSignCrdZ), a
;17203: 		pop	af
;17204: 		ld	(VertexCrdZlow), a
;17205: 		ld	a, (WhatOnScreen)
;17206: 		bit	1, a
;17207: 		jr	nz, loc_AD82
;17208: 		ld	de, (VertexCrdX)
;17209: 		ld	a, (VertexCrdXlow)
;17210: 		ld	c, a
;17211: 		ld	a, (VertexCrdZlow)
;17212: 		call	Div_C_DEc_HLa
;17213: ; END OF FUNCTION CHUNK	FOR MakeTablPrnCrdVertex
;17214: ; ---------------------------------------------------------------------------
;17215: 		ld	a, c
;17216: 		ld	(CrdXProjection), a
;17217: 		ld	de, (VertexCrdY)
;17218: 		ld	hl, (VertexCrdZ)
;17219: 		ld	a, (VertexCrdYlow)
;17220: 		ld	c, a
;17221: 		ld	a, (VertexCrdZlow)
;17222: 		call	Div_C_DEc_HLa
;17223: ; ---------------------------------------------------------------------------
;17224: 		ld	a, c
;17225: 		ld	(CrdYProjection), a
;17226: 		ld	a, (VertexSignCrdX)
;17227: 		rla
;17228: 		rl	c
;17229: 		ld	a, (VertexSignCrdZ)
;17230: 		rla
;17231: 		rl	b
;17232: 		jr	loc_ADB5
;17233: ; ---------------------------------------------------------------------------
;17234: ; START	OF FUNCTION CHUNK FOR MakeTablPrnCrdVertex
;17235: 
;17236: loc_AD82:				; CODE XREF: MakeTablPrnCrdVertex+382j
;17237: 		ex	de, hl
;17238: 		ld	hl, (VertexCrdX)
;17239: 		ld	a, (VertexCrdZlow)
;17240: 		ld	c, a
;17241: 		ld	a, (VertexCrdXlow)
;17242: 		call	Div_C_DEc_HLa
;17243: ; END OF FUNCTION CHUNK	FOR MakeTablPrnCrdVertex
;17244: ; ---------------------------------------------------------------------------
;17245: 		ld	a, c
;17246: 		ld	(CrdXProjection), a
;17247: 		ld	de, (VertexCrdY)
;17248: 		ld	hl, (VertexCrdX)
;17249: 		ld	a, (VertexCrdYlow)
;17250: 		ld	c, a
;17251: 		ld	a, (VertexCrdXlow)
;17252: 		call	Div_C_DEc_HLa
;17253: ; ---------------------------------------------------------------------------
;17254: 		ld	a, c
;17255: 		ld	(CrdYProjection), a
;17256: 		ld	a, (VertexSignCrdX)
;17257: 		rla
;17258: 		rl	b
;17259: 		ld	a, (VertexSignCrdZ)
;17260: 		rla
;17261: 		rl	c
;17262: 
;17263: loc_ADB5:				; CODE XREF: RAM:AD80j
;17264: 		ld	a, (VertexSignCrdY)
;17265: 		rla
;17266: 		rl	c
;17267: 		ld	a, (WhatOnScreen)
;17268: 		bit	0, a
;17269: 		ld	a, c
;17270: 		jr	z, loc_ADC5
;17271: 		xor	2
;17272: 
;17273: loc_ADC5:				; CODE XREF: RAM:ADC1j
;17274: 		and	3
;17275: 		ld	(CrdXYsign), a
;17276: 		ld	a, (PointTablCrdPrn)
;17277: 		ld	hl, TablCrdPrnVertex
;17278: 		add	a, l
;17279: 		jr	nc, loc_ADD4
;17280: 		inc	h
;17281: 
;17282: loc_ADD4:				; CODE XREF: RAM:ADD1j
;17283: 		ld	l, a
;17284: 		ld	a, (CrdXProjection)
;17285: 		ld	(hl), a
;17286: 		inc	hl
;17287: 		ld	a, (CrdYProjection)
;17288: 		ld	(hl), a
;17289: 		inc	hl
;17290: 		ld	a, (WhatOnScreen)
;17291: 		ld	c, a
;17292: 		rra
;17293: 		xor	c
;17294: 		xor	b
;17295: 		rra
;17296: 		ld	a, (CrdXYsign)
;17297: 		jr	nc, loc_ADEE
;17298: 		ld	a, 0FFh
;17299: 
;17300: loc_ADEE:				; CODE XREF: RAM:ADEAj
;17301: 		ld	(hl), a
;17302: 		pop	af
;17303: 		ex	(sp), ix
;17304: ; START	OF FUNCTION CHUNK FOR MakeTablPrnCrdVertex
;17305: 
;17306: loc_ADF2:				; CODE XREF: MakeTablPrnCrdVertex+42j
;17307: 		call	sub_AE1D
;17308: 		add	ix, bc
;17309: 		inc	de
;17310: 		dec	a
;17311: 		jp	nz, loc_A9DE
;17312: 		pop	ix
;17313: 		pop	iy
;17314: 		ret
;17315: ; END OF FUNCTION CHUNK	FOR MakeTablPrnCrdVertex
;17316: 
;17317: ; =============== S U B	R O U T	I N E =======================================
;17318: 
;17319: 
;17320: sub_AE01:				; CODE XREF: MakeTablPrnCrdVertex+Cp
;17321: 		ld	e, (iy+0Ch)
;17322: 		add	ix, de
;17323: 		ld	e, a
;17324: 		push	af
;17325: 		add	a, a
;17326: 		add	a, e
;17327: 		ld	e, a
;17328: 		pop	af
;17329: 		jr	loc_AE16
;17330: ; End of function sub_AE01
;17331: 
;17332: 
;17333: ; =============== S U B	R O U T	I N E =======================================
;17334: 
;17335: 
;17336: sub_AE0E:				; CODE XREF: CalcVisibleVerges+Fp
;17337: 		ld	e, (ix+10h)
;17338: 		add	ix, de
;17339: 		ld	e, (iy+5)
;17340: 
;17341: loc_AE16:				; CODE XREF: sub_AE01+Bj
;17342: 		ld	(TempAdr), ix	; адрес	текущей	вершины
;17343: 		ld	d, 0
;17344: 		ret
;17345: ; End of function sub_AE0E
;17346: 
;17347: 
;17348: ; =============== S U B	R O U T	I N E =======================================
;17349: 
;17350: 
;17351: sub_AE1D:				; CODE XREF: MakeTablPrnCrdVertex:loc_ADF2p
;17352: 		ld	bc, 3
;17353: 		ld	hl, (TempAdr)	; адрес	текущей	вершины
;17354: 		add	hl, bc
;17355: 		jr	loc_AE2D
;17356: ; End of function sub_AE1D
;17357: 
;17358: 
;17359: ; =============== S U B	R O U T	I N E =======================================
;17360: 
;17361: 
;17362: sub_AE26:				; CODE XREF: CalcVisibleVerges+7Dp
;17363: 		ld	bc, 3
;17364: 		ld	hl, (TempAdr)	; адрес	текущей	вершины
;17365: 		inc	hl
;17366: 
;17367: loc_AE2D:				; CODE XREF: sub_AE1D+7j
;17368: 		ld	(TempAdr), hl	; адрес	текущей	вершины
;17369: 		bit	0, (hl)
;17370: 		ret	nz
;17371: 		ld	c, b
;17372: 		ret
;17373: ; End of function sub_AE26
;17374: 
;17375: 
;17376: ; =============== S U B	R O U T	I N E =======================================
;17377: 
;17378: 
;17379: SetObjInSlot_Intro:			; CODE XREF: SetShip4Intro+7p
;17380: 		ld	hl, AdrDataShips
;17381: 		ld	b, a
;17382: 		jp	loc_B89C
;17383: ; End of function SetObjInSlot_Intro
;17384: 
;17385: ; ---------------------------------------------------------------------------
;17386: byte_AE3C:	db 6, 7, 8, 0Bh, 0Ch, 0Dh, 11h,	0Eh, 14h, 0Fh, 12h, 13h
;17387: 					; DATA XREF: sub_728D+12Ao
;17388: 					; номера кораблей для интро
;17389: 		db  7Eh	; ~
;17390: 		db  22h	; "
;17391: 		db 0D8h	; ╪
;17392: 		db 0F3h	; є
;17393: 		db  76h	; v
;17394: 		db 0C9h	; ╔
;17395: 		db 0C9h	; ╔
;17396: 		db 0C9h	; ╔
;17397: 
;17398: ; =============== S U B	R O U T	I N E =======================================
;17399: 
;17400: ; Получение печатных координат вершины из таблицы по номеру
;17401: ; Вх:  a - номер вершины
;17402: ; Вых: de - печатные координаты	вершины
;17403: ;      a - знаки координат
;17404: ;      z =1 вершина печатается
;17405: ;      hl- не определен
;17406: 
;17407: GetPrnCrdVertex:			; CODE XREF: PrintObject_2:loc_6FB5p
;17408: 					; PrintObject_2+83p ...
;17409: 		ld	b, a
;17410: 		add	a, a
;17411: 		ld	hl, TablCrdPrnVertex
;17412: 		jr	nc, loc_AE58
;17413: 		inc	h
;17414: 
;17415: loc_AE58:				; CODE XREF: GetPrnCrdVertex+5j
;17416: 		add	a, b
;17417: 		jr	nc, loc_AE5C
;17418: 		inc	h
;17419: 
;17420: loc_AE5C:				; CODE XREF: GetPrnCrdVertex+9j
;17421: 		add	a, l
;17422: 		ld	l, a
;17423: 		jr	nc, loc_AE61
;17424: 		inc	h
;17425: 
;17426: loc_AE61:				; CODE XREF: GetPrnCrdVertex+Ej
;17427: 		ld	d, (hl)
;17428: 		inc	hl
;17429: 		ld	e, (hl)
;17430: 		inc	hl
;17431: 		ld	a, (hl)
;17432: 		bit	7, a
;17433: 		ret
;17434: ; End of function GetPrnCrdVertex
;17435: 
;17436: 
;17437: ; =============== S U B	R O U T	I N E =======================================
;17438: 
;17439: 
;17440: CorrAxirY_SKO:				; CODE XREF: sub_9D40+ECp sub_A28D+17p
;17441: 		push	ix
;17442: 		push	ix
;17443: 		ex	(sp), iy
;17444: 		ld	de, 9
;17445: 		add	ix, de
;17446: 		ld	e, 0Fh
;17447: 		add	iy, de
;17448: 		call	ScalarMultVectors
;17449: 		ld	e, a
;17450: 		call	sub_AE8A
;17451: 		call	sub_AE8A
;17452: 		call	sub_AE8A
;17453: 		pop	iy
;17454: 		pop	ix
;17455: 		ret
;17456: ; End of function CorrAxirY_SKO
;17457: 
;17458: 
;17459: ; =============== S U B	R O U T	I N E =======================================
;17460: 
;17461: 
;17462: sub_AE8A:				; CODE XREF: CorrAxirY_SKO+13p
;17463: 					; CorrAxirY_SKO+16p ...
;17464: 		ld	c, (ix+1)
;17465: 		ld	b, e
;17466: 		sla	b
;17467: 		sla	c
;17468: 		call	Mult_A_hiB_C
;17469: 		ld	c, a
;17470: 		ld	a, e
;17471: 		xor	(ix+1)
;17472: 		rla
;17473: 		rr	c
;17474: 		ld	b, (iy+1)
;17475: 		ld	a, b
;17476: 		xor	c
;17477: 		sla	c
;17478: 		rl	h
;17479: 		sla	b
;17480: 		rl	h
;17481: 		rla
;17482: 		ld	a, b
;17483: 		jr	c, loc_AEB6
;17484: 		add	a, c
;17485: 		jr	nc, loc_AEBD
;17486: 		ld	a, 0FFh
;17487: 		jp	loc_AEBD
;17488: ; ---------------------------------------------------------------------------
;17489: 
;17490: loc_AEB6:				; CODE XREF: sub_AE8A+22j
;17491: 		sub	c
;17492: 		jr	nc, loc_AEBD
;17493: 		neg
;17494: 		rr	h
;17495: 
;17496: loc_AEBD:				; CODE XREF: sub_AE8A+25j sub_AE8A+29j ...
;17497: 		rr	h
;17498: 		rra
;17499: 		ld	(iy+1),	a
;17500: 		inc	ix
;17501: 		inc	ix
;17502: 		inc	iy
;17503: 		inc	iy
;17504: 		ret
;17505: ; End of function sub_AE8A
;17506: 
;17507: 
;17508: ; =============== S U B	R O U T	I N E =======================================
;17509: 
;17510: 
;17511: CalcMatrixRotate:			; CODE XREF: objPrnObj+2p
;17512: 		ld	iy, MatrixRotate
;17513: 		ld	b, (ix+0Ch)
;17514: 		ld	c, (ix+14h)
;17515: 		ld	d, (ix+0Eh)
;17516: 		ld	e, (ix+12h)
;17517: 		call	MultCrdsVectors
;17518: 		ld	(ix+16h), h
;17519: 		ld	(iy+1),	h
;17520: 		ld	b, (ix+0Eh)
;17521: 		ld	(iy+11h), b
;17522: 		ld	c, (ix+10h)
;17523: 		ld	(iy+3),	c
;17524: 		ld	d, (ix+0Ah)
;17525: 		ld	e, (ix+14h)
;17526: 		ld	(iy+0Fh), e
;17527: 
;17528: loc_AEFA:
;17529: 		call	MultCrdsVectors
;17530: 		ld	(ix+18h), h
;17531: 		ld	(iy+7),	h
;17532: 		ld	b, (ix+0Ah)
;17533: 		ld	(iy+5),	b
;17534: 		ld	c, (ix+12h)
;17535: 		ld	(iy+9),	c
;17536: 		ld	d, (ix+0Ch)
;17537: 		ld	(iy+0Bh), d
;17538: 		ld	e, (ix+10h)
;17539: 		call	MultCrdsVectors
;17540: 		ld	(ix+1Ah), h
;17541: 		ld	(iy+0Dh), h
;17542: 		ret
;17543: ; End of function CalcMatrixRotate
;17544: 
;17545: 
;17546: ; =============== S U B	R O U T	I N E =======================================
;17547: 
;17548: 
;17549: MultCrdsVectors:			; CODE XREF: CalcMatrixRotate+10p
;17550: 					; CalcMatrixRotate:loc_AEFAp ...
;17551: 		sla	b
;17552: 		rra
;17553: 		xor	c
;17554: 		sla	c
;17555: 		and	80h ; 'А'
;17556: 		ld	h, a
;17557: 		sla	d
;17558: 		rra
;17559: 		xor	e
;17560: 		sla	e
;17561: 		and	80h ; 'А'
;17562: 		xor	h
;17563: 		sla	h
;17564: 		rra
;17565: 		xor	80h ; 'А'
;17566: 		push	af
;17567: 		push	de
;17568: 		call	Mult_HL_B_C
;17569: 		pop	bc
;17570: 		push	hl
;17571: 		call	Mult_HL_B_C
;17572: 		pop	de
;17573: 		pop	af
;17574: 		bit	6, a
;17575: 		jr	z, loc_AF51
;17576: 		add	hl, de
;17577: 		jr	nc, loc_AF5E
;17578: 		ld	hl, 0FFFFh
;17579: 		jr	loc_AF5E
;17580: ; ---------------------------------------------------------------------------
;17581: 
;17582: loc_AF51:				; CODE XREF: MultCrdsVectors+25j
;17583: 		ex	de, hl
;17584: 		and	a
;17585: 		sbc	hl, de
;17586: 		jr	nc, loc_AF5E
;17587: 		add	hl, de
;17588: 		ex	de, hl
;17589: 		and	a
;17590: 		sbc	hl, de
;17591: 		xor	80h ; 'А'
;17592: 
;17593: loc_AF5E:				; CODE XREF: MultCrdsVectors+28j
;17594: 					; MultCrdsVectors+2Dj ...
;17595: 		rla
;17596: 		rr	h
;17597: 		rr	l
;17598: 		ret
;17599: ; End of function MultCrdsVectors
;17600: 
;17601: ; ---------------------------------------------------------------------------
;17602: 		db 0DDh	; ▌
;17603: 		db 0E5h	; х
;17604: 		db 0DDh	; ▌
;17605: 		db  21h	; !
;17606: 		db    0
;17607: 		db    0
;17608: 		db  47h	; G
;17609: 		db 0C5h	; ┼
;17610: 		db 0D5h	; ╒
;17611: 		db 0C5h	; ┼
;17612: 		db 0D5h	; ╒
;17613: 		db 0C5h	; ┼
;17614: 		db 0D5h	; ╒
;17615: 		db  43h	; C
;17616: 		db 0CDh	; ═
;17617: 		db  34h	; 4
;17618: 		db 0ABh	; л
;17619: 		db 0EBh	; ы
;17620: 		db 0DDh	; ▌
;17621: 		db  19h
;17622: 		db 0D1h	; ╤
;17623: 		db 0C1h	; ┴
;17624: 		db  42h	; B
;17625: 		db 0CDh	; ═
;17626: 		db  34h	; 4
;17627: 		db 0ABh	; л
;17628: 		db  1Eh
;17629: 		db    0
;17630: 		db  55h	; U
;17631: 		db 0DDh	; ▌
;17632: 		db  19h
;17633: 		db  6Ch	; l
;17634: 		db  63h	; c
;17635: 		db  30h	; 0
;17636: 		db    1
;17637: 		db  23h	; #
;17638: 		db 0C1h	; ┴
;17639: 		db 0D1h	; ╤
;17640: 		db 0E5h	; х
;17641: 		db  42h	; B
;17642: 		db 0CDh	; ═
;17643: 		db  34h	; 4
;17644: 		db 0ABh	; л
;17645: 		db  1Eh
;17646: 		db    0
;17647: 		db  55h	; U
;17648: 		db 0DDh	; ▌
;17649: 		db  19h
;17650: 		db 0D1h	; ╤
;17651: 		db  6Ch	; l
;17652: 		db  26h	; &
;17653: 		db    0
;17654: 		db  30h	; 0
;17655: 		db    1
;17656: 		db  23h	; #
;17657: 		db  19h
;17658: 		db 0C1h	; ┴
;17659: 		db 0D1h	; ╤
;17660: 		db  4Ah	; J
;17661: 		db 0E5h	; х
;17662: 		db 0CDh	; ═
;17663: 		db  34h	; 4
;17664: 		db 0ABh	; л
;17665: 		db 0D1h	; ╤
;17666: 		db  19h
;17667: 		db 0DDh	; ▌
;17668: 		db 0E1h	; с
;17669: 		db 0C9h	; ╔
;17670: 
;17671: ; =============== S U B	R O U T	I N E =======================================
;17672: 
;17673: ; Attributes: noreturn
;17674: 
;17675: Div_BC_HLAdivDE_1:			; CODE XREF: Div_BC_HLAdivDEj
;17676: 					; CaclNormalVector+9p ...
;17677: 		ld	bc, 0FFFEh
;17678: 
;17679: loc_AFAB:				; CODE XREF: Div_BC_HLAdivDE_1+14j
;17680: 					; Div_BC_HLAdivDE_1+1Fj
;17681: 		add	a, a
;17682: 		rl	l
;17683: 		rl	h
;17684: 		jr	c, loc_AFBE
;17685: 		sbc	hl, de
;17686: 		jr	nc, loc_AFC1
;17687: 		add	hl, de
;17688: 		sla	c
;17689: 		rl	b
;17690: 		ret	nc
;17691: 		jr	loc_AFAB
;17692: ; ---------------------------------------------------------------------------
;17693: 
;17694: loc_AFBE:				; CODE XREF: Div_BC_HLAdivDE_1+8j
;17695: 		and	a
;17696: 		sbc	hl, de
;17697: 
;17698: loc_AFC1:				; CODE XREF: Div_BC_HLAdivDE_1+Cj
;17699: 		scf
;17700: 		rl	c
;17701: 		rl	b
;17702: 		ret	nc
;17703: 		jr	loc_AFAB
;17704: ; End of function Div_BC_HLAdivDE_1
;17705: 
;17706: 
;17707: ; =============== S U B	R O U T	I N E =======================================
;17708: 
;17709: 
;17710: sub_AFC9:				; CODE XREF: RAM:loc_9CA5j
;17711: 					; sub_A53F+1Ep
;17712: 		ld	a, d
;17713: 		xor	80h ; 'А'
;17714: 		ld	d, a
;17715: ; End of function sub_AFC9
;17716: 
;17717: 
;17718: ; =============== S U B	R O U T	I N E =======================================
;17719: 
;17720: 
;17721: AddSign_HL_DE:				; CODE XREF: ScalarMultVectors16_8+27p
;17722: 					; ScalarMultVectors16_8+33j
;17723: 		ld	a, h
;17724: 		xor	d
;17725: 		ld	b, h
;17726: 		ld	c, d
;17727: 		res	7, h
;17728: 		res	7, d
;17729: 		and	80h ; 'А'
;17730: 		jr	nz, loc_AFDC
;17731: 		add	hl, de
;17732: 		jr	loc_AFE6
;17733: ; ---------------------------------------------------------------------------
;17734: 
;17735: loc_AFDC:				; CODE XREF: AddSign_HL_DE+Aj
;17736: 		sbc	hl, de
;17737: 		jr	nc, loc_AFE6
;17738: 		add	hl, de
;17739: 		ex	de, hl
;17740: 		and	a
;17741: 		sbc	hl, de
;17742: 		ld	b, c
;17743: 
;17744: loc_AFE6:				; CODE XREF: AddSign_HL_DE+Dj
;17745: 					; AddSign_HL_DE+11j
;17746: 		srl	h
;17747: 		rr	l
;17748: 		bit	7, b
;17749: 		ret	z
;17750: 		set	7, h
;17751: 		ret
;17752: ; End of function AddSign_HL_DE
;17753: 
;17754: ; ---------------------------------------------------------------------------
;17755: 		db    0
;17756: 		db    4
;17757: 		db    2
;17758: 		db    3
;17759: 		db    0
;17760: 		db    4
;17761: 		db    6
;17762: 		db    5
;17763: 		db    0
;17764: 		db    8
;17765: 		db    2
;17766: 		db    1
;17767: 		db    0
;17768: 		db    8
;17769: 		db    6
;17770: 		db    7
;17771: 		db    0
;17772: 		db    0
;17773: 		db    0
;17774: 		db    2
;17775: 		db    1
;17776: 		db    2
;17777: 		db    0
;17778: 		db    0
;17779: 		db    0
;17780: 		db    0
;17781: 		db    0
;17782: 		db    2
;17783: 		db    2
;17784: 		db    1
;17785: 		db    2
;17786: 		db    2
;17787: 		db    0
;17788: 		db    0
;17789: 		db    0
;17790: 		db    0
;17791: 		db    0
;17792: 		db    2
;17793: 		db    1
;17794: 		db    2
;17795: 		db    2
;17796: 		db    2
;17797: 		db    0
;17798: 		db    0
;17799: 		db    0
;17800: 		db    2
;17801: 		db    2
;17802: 		db    1
;17803: 		db    1
;17804: 		db    2
;17805: 		db    0
;17806: 		db    0
;17807: 		db    0
;17808: 		db    0
;17809: 		db    0
;17810: 		db    2
;17811: 		db    2
;17812: 		db    1
;17813: 		db    2
;17814: 		db    2
;17815: 		db    0
;17816: 		db    0
;17817: 		db    0
;17818: 		db    2
;17819: 		db    0
;17820: 		db    2
;17821: 		db    1
;17822: 		db    2
;17823: 		db    0
;17824: 		db    0
;17825: 		db    0
;17826: 		db    0
;17827: 		db    0
;17828: 		db    2
;17829: 		db    2
;17830: 		db    1
;17831: 		db    2
;17832: 		db    2
;17833: 		db    0
;17834: 		db    0
;17835: 		db    0
;17836: 		db    0
;17837: 		db    0
;17838: 		db    0
;17839: 
;17840: ; =============== S U B	R O U T	I N E =======================================
;17841: 
;17842: 
;17843: PrnLineInBuf_clip:			; CODE XREF: PrintObject_2+91p
;17844: 					; sub_7C60+C5p	...
;17845: 
;17846: ; FUNCTION CHUNK AT B0B6 SIZE 00000082 BYTES
;17847: ; FUNCTION CHUNK AT B162 SIZE 00000029 BYTES
;17848: 
;17849: 		ld	a, d
;17850: 		and	l
