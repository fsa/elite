;15905: RadiusBlast:	db 28h			; DATA XREF: sub_7E02+50r
;15906: 					; PrnObjIfVisible+92w ...
;15907: 
;15908: ; =============== S U B	R O U T	I N E =======================================
;15909: 
;15910: 
;15911: PrnObjIfVisible:			; CODE XREF: sub_A28D+28j
;15912: 					; sub_9D40+5B88p
;15913: 
;15914: ; FUNCTION CHUNK AT A73B SIZE 0000009E BYTES
;15915: 
;15916: 		call	ResetSignInExtSlot
;15917: 		ld	b, a
;15918: 		cp	4
;15919: 		ret	nc
;15920: 		push	ix
;15921: 		pop	hl
;15922: 		bit	1, a
;15923: 		jr	nz, loc_A6F1
;15924: 		ld	de, 6
;15925: 		add	hl, de
;15926: 
;15927: loc_A6F1:				; CODE XREF: PrnObjIfVisible+Cj
;15928: 		ld	e, (hl)
;15929: 		inc	hl
;15930: 		ld	d, (hl)
;15931: 		inc	hl
;15932: 		ld	c, (hl)		; cde -	расстояние до объекта
;15933: 		srl	a
;15934: 		xor	b
;15935: 		rrca
;15936: 		xor	c
;15937: 		and	80h ; 'А'
;15938: 		ret	nz
;15939: 		ld	(DistanceToObj), de
;15940: 		ld	h, 0
;15941: 		ld	l, (iy+1)
;15942: 		push	hl
;15943: 		and	a
;15944: 		sbc	hl, de
;15945: 		jr	c, loc_A70F
;15946: 		pop	hl
;15947: 		ret
;15948: ; ---------------------------------------------------------------------------
;15949: 
;15950: loc_A70F:				; CODE XREF: PrnObjIfVisible+2Cj
;15951: 		push	de
;15952: 		push	de
;15953: 		bit	1, b
;15954: 		jr	nz, loc_A73B
;15955: 		ex	de, hl
;15956: 		ld	d, (ix+1)
;15957: 		ld	e, (ix+0)
;15958: 		xor	a
;15959: 		ld	c, a
;15960: 
;15961: loc_A71E:
;15962: 		call	Div_C_DEc_HLa
;15963: ; ---------------------------------------------------------------------------
;15964: 		ld	a, c
;15965: 		ld	(CrdXProjection), a
;15966: 		pop	hl
;15967: 		ld	d, (ix+4)
;15968: 		ld	e, (ix+3)
;15969: 		xor	a
;15970: 		ld	c, a
;15971: 		call	Div_C_DEc_HLa
;15972: ; ---------------------------------------------------------------------------
;15973: 		ld	a, c
;15974: 		ld	(CrdYProjection), a
;15975: 		ld	b, (ix+2)
;15976: 		jp	loc_A75E
;15977: ; End of function PrnObjIfVisible
;15978: 
;15979: ; ---------------------------------------------------------------------------
;15980: ; START	OF FUNCTION CHUNK FOR PrnObjIfVisible
;15981: 
;15982: loc_A73B:				; CODE XREF: PrnObjIfVisible+34j
;15983: 		ex	de, hl
;15984: 		ld	d, (ix+7)
;15985: 		ld	e, (ix+6)
;15986: 		xor	a
;15987: 		ld	c, a
;15988: 		call	Div_C_DEc_HLa
;15989: ; ---------------------------------------------------------------------------
;15990: 		ld	a, c
;15991: 		ld	(CrdXProjection), a
;15992: 		pop	hl
;15993: 		ld	d, (ix+4)
;15994: 		ld	e, (ix+3)
;15995: 		xor	a
;15996: 		ld	c, a
;15997: 		call	Div_C_DEc_HLa
;15998: ; ---------------------------------------------------------------------------
;15999: 		ld	a, c
;16000: 		ld	(CrdYProjection), a
;16001: 		ld	b, (ix+8)
;16002: 
;16003: loc_A75E:				; CODE XREF: PrnObjIfVisible+59j
;16004: 		ld	a, (ix+5)
;16005: 		sla	b
;16006: 		rla
;16007: 		rla
;16008: 		ld	(CrdXYsign), a
;16009: 		pop	hl
;16010: 		pop	de
;16011: 		xor	a
;16012: 		ld	c, a
;16013: 		call	Div_C_DEc_HLa
;16014: ; ---------------------------------------------------------------------------
;16015: 		ld	h, c
;16016: 		ld	a, c
;16017: 		ld	(RadiusBlast), a
;16018: 		ld	e, c
;16019: 		ld	a, (CrdYProjection)
;16020: 		ld	c, a
;16021: 		sub	h
;16022: 		jr	c, loc_A781
;16023: 		sub	40h ; '@'
;16024: 		jp	nc, objPrintLaser ; de - координаты откуда стреляет
;16025: 
;16026: loc_A781:				; CODE XREF: PrnObjIfVisible+9Bj
;16027: 		ld	a, (CrdXProjection)
;16028: 		ld	b, a
;16029: 		sub	h
;16030: 		jr	c, loc_A78D
;16031: 		sub	80h ; 'А'
;16032: 		jp	nc, objPrintLaser ; de - координаты откуда стреляет
;16033: 
;16034: loc_A78D:				; CODE XREF: PrnObjIfVisible+A7j
;16035: 		ld	a, b
;16036: 		add	a, c
;16037: 		ld	d, a
;16038: 		jr	c, loc_A7D2	; вне экрана
;16039: 		sub	e
;16040: 		jr	nc, loc_A7A3	; далеко от центра экрана
;16041: 		ld	a, (ix+7)
;16042: 		cp	2Ch ; ','
;16043: 		jr	nc, loc_A7A3
;16044: 
;16045: loc_A79C:				; DATA XREF: sub_9D40+5B78w
;16046: 		ld	a, 5
;16047: 		nop
;16048: 		nop
;16049: 		ld	(ObjInTarget), a
;16050: 
;16051: loc_A7A3:				; CODE XREF: PrnObjIfVisible+B4j
;16052: 					; PrnObjIfVisible+BBj
;16053: 		bit	6, (ix+20h)
;16054: 		jr	z, loc_A7D2
;16055: 		bit	2, (ix+21h)
;16056: 		jr	z, loc_A7D2
;16057: 		ld	a, (WhatOnScreen)
;16058: 		bit	1, a
;16059: 		ld	a, (ix+0Eh)
;16060: 		ld	e, (ix+7)
;16061: 		jr	z, loc_A7C2
;16062: 		ld	a, (ix+0Ah)
;16063: 		ld	e, (ix+1)
;16064: 
;16065: loc_A7C2:				; CODE XREF: PrnObjIfVisible+DBj
;16066: 		ld	b, a
;16067: 		and	7Fh ; ''
;16068: 		cp	40h ; '@'
;16069: 		jr	c, loc_A7D2
;16070: 		ld	a, d
;16071: 		cp	20h ; ' '
;16072: 		jr	nc, loc_A7D2
;16073: 		res	5, (ix+26h)
;16074: 
;16075: loc_A7D2:				; CODE XREF: PrnObjIfVisible+B1j
;16076: 					; PrnObjIfVisible+C8j ...
;16077: 		bit	4, (ix+20h)
;16078: 		jp	sub_7E02
;16079: ; END OF FUNCTION CHUNK	FOR PrnObjIfVisible
;16080: 
;16081: ; =============== S U B	R O U T	I N E =======================================
;16082: 
;16083: ; если включена	невидимость и это ASP из 2й миссии, то печать только луча лазера
;16084: 
;16085: sub_A7D9:				; CODE XREF: sub_7E02+17j
;16086: 					; sub_7E02:loc_7E9Dj
;16087: 
;16088: ; FUNCTION CHUNK AT A8DE SIZE 0000000E BYTES
;16089: 
;16090: 		ld	a, (Mission2)
;16091: 		dec	a
;16092: 		jr	nz, loc_A7E6
;16093: 		bit	6, (ix+26h)
;16094: 		jp	nz, objPrintLaser ; de - координаты откуда стреляет
;16095: 
;16096: loc_A7E6:				; CODE XREF: sub_A7D9+4j
;16097: 		ld	hl, (DistanceToObj)
;16098: 		ld	a, h
;16099: 		cp	(iy+0Bh)
;16100: 		jr	c, objPrnObj
;16101: 		ld	a, (CrdXProjection)
;16102: 		cp	7Fh ; ''
;16103: 		jp	nc, objPrintLaser ; de - координаты откуда стреляет
;16104: 		ld	c, a
;16105: 		ld	a, (CrdYProjection)
;16106: 		cp	3Fh ; '?'
;16107: 		jp	nc, objPrintLaser ; de - координаты откуда стреляет
;16108: 		ld	b, a
;16109: 		ld	a, (WhatOnScreen)
;16110: 		bit	0, a
;16111: 		ld	a, (CrdXYsign)
;16112: 		jr	z, loc_A80D
;16113: 		xor	2
;16114: 
;16115: loc_A80D:				; CODE XREF: sub_A7D9+30j
;16116: 		ld	d, a
;16117: 		bit	1, d
;16118: 		jr	nz, loc_A818
;16119: 		ld	a, c
;16120: 		add	a, 7Fh ; ''
;16121: 		ld	c, a
;16122: 		jr	loc_A81C
;16123: ; ---------------------------------------------------------------------------
;16124: 
;16125: loc_A818:				; CODE XREF: sub_A7D9+37j
;16126: 		ld	a, 7Fh ; ''
;16127: 		sub	c
;16128: 		ld	c, a
;16129: 
;16130: loc_A81C:				; CODE XREF: sub_A7D9+3Dj
;16131: 		bit	0, d
;16132: 		jr	z, loc_A826
;16133: 		ld	a, b
;16134: 		add	a, 3Fh ; '?'
;16135: 		ld	b, a
;16136: 		jr	loc_A82A
;16137: ; ---------------------------------------------------------------------------
;16138: 
;16139: loc_A826:				; CODE XREF: sub_A7D9+45j
;16140: 		ld	a, 3Fh ; '?'
;16141: 		sub	b
;16142: 		ld	b, a
;16143: 
;16144: loc_A82A:				; CODE XREF: sub_A7D9+4Bj
;16145: 		call	objPrnPixel
;16146: 		inc	b
;16147: 		call	objPrnPixel
;16148: 		inc	c
;16149: 		call	objPrnPixel
;16150: 		dec	b
;16151: 		call	objPrnPixel
;16152: 		jp	objPrintLaser	; de - координаты откуда стреляет
;16153: ; End of function sub_A7D9
;16154: 
;16155: 
;16156: ; =============== S U B	R O U T	I N E =======================================
;16157: 
;16158: 
;16159: objPrnPixel:				; CODE XREF: sub_78B1+13Fp
;16160: 					; sub_A7D9:loc_A82Ap ...
;16161: 		ld	a, b
;16162: 		call	22B1h
;16163: 		ld	d, a
;16164: 		inc	d
;16165: 		ld	a, 1
;16166: 
;16167: loc_A844:				; CODE XREF: objPrnPixel+Aj
;16168: 		rrca
;16169: 		dec	d
;16170: 		jr	nz, loc_A844
;16171: 
;16172: loc_A848:				; DATA XREF: RAM:B35Dw	RAM:B38Fw
;16173: 		set	7, h
;16174: 		xor	(hl)
;16175: 		ld	(hl), a
;16176: 		ret
;16177: ; End of function objPrnPixel
;16178: 
;16179: 
;16180: ; =============== S U B	R O U T	I N E =======================================
;16181: 
;16182: 
;16183: objPrnObj:				; CODE XREF: sub_A7D9+14j
;16184: 		push	iy
;16185: 		call	CalcMatrixRotate
;16186: 		pop	iy
;16187: 		ld	hl, (DistanceToObj)
;16188: 		add	hl, hl
;16189: 		ld	b, 1Fh
;16190: 		jr	c, loc_A861
;16191: 		ld	a, h
;16192: 		cp	b
;16193: 		jr	nc, loc_A861
;16194: 		ld	b, h
;16195: 
;16196: loc_A861:				; CODE XREF: objPrnObj+Dj
;16197: 					; objPrnObj+11j
;16198: 		ld	a, b
;16199: 		sub	2
;16200: 		jr	nc, loc_A867
;16201: 		xor	a
;16202: 
;16203: loc_A867:				; CODE XREF: objPrnObj+17j
;16204: 		ld	(DistanceToObj), a
;16205: 		ld	a, 0FDh	; '¤'
;16206: 		ld	(PointTablCrdPrn), a
;16207: 		call	CalcVisibleVerges
;16208: 		call	MakeTablPrnCrdVertex
;16209: 		call	PrintObject
;16210: 
;16211: loc_A878:				; CODE XREF: sub_7E02:loc_7EA6j
;16212: 		bit	3, (ix+20h)
;16213: 		ret	z		; нет выстрела из лазера
;16214: 		ld	a, (iy+14h)	; вершина с лазером
;16215: 		call	GetPrnCrdVertex	; Получение печатных координат вершины из таблицы по номеру
;16216: 					; Вх:  a - номер вершины
;16217: 					; Вых: de - печатные координаты	вершины
;16218: 					;      a - знаки координат
;16219: 					;      z =1 вершина печатается
;16220: 					;      hl- не определен
;16221: 		ld	c, a
;16222: 
;16223: objPrnLaser:				; CODE XREF: sub_A7D9+110j
;16224: 		call	SetBit2ExtSlot_rndA
;16225: 		ld	l, a
;16226: 		sub	0FFh
;16227: 		neg
;16228: 		ld	h, a
;16229: 		ld	a, c
;16230: 		cpl
;16231: 		and	3
;16232: 		sla	c
;16233: 		sla	c
;16234: 		or	c
;16235: 		ld	c, a
;16236: 		call	PrnLineInBuf_clip
;16237: 		jr	loc_A8A7
;16238: ; ---------------------------------------------------------------------------
;16239: unk_A89C:	db  68h	; h		; DATA XREF: CopyLocator2Scr+3Fo
;16240: 		db  94h	; Ф
;16241: 		db  95h	; Х
;16242: 		db 0B4h	; ┤
;16243: 		db 0B5h	; ╡
;16244: 		db  87h	; З
;16245: 		db  88h	; И
;16246: 		db 0A7h	; з
;16247: 		db 0A8h	; и
;16248: 		db 0A8h	; и
;16249: 		db 0FFh
;16250: ; ---------------------------------------------------------------------------
;16251: 
;16252: loc_A8A7:				; CODE XREF: objPrnObj+4Dj
;16253: 		res	3, (ix+20h)
;16254: 		ret
;16255: ; End of function objPrnObj
;16256: 
;16257: 
;16258: ; =============== S U B	R O U T	I N E =======================================
;16259: 
;16260: 
;16261: objDestroyed2:				; CODE XREF: sub_7E02+8j
;16262: 		ld	a, (CrdXProjection)
;16263: 		ld	(ix+1Ch), a
;16264: 		ld	a, (CrdYProjection)
;16265: 		ld	(ix+1Dh), a
;16266: 		ld	a, (RadiusBlast)
;16267: 		ld	(ix+1Fh), a
;16268: 		ld	a, (WhatOnScreen)
;16269: 		bit	0, a
;16270: 		ld	a, (CrdXYsign)
;16271: 		jr	z, loc_A8CA
;16272: 		xor	2
;16273: 
;16274: loc_A8CA:				; CODE XREF: objDestroyed2+1Aj
;16275: 		ld	c, a
;16276: 		xor	a
;16277: 		rr	c
;16278: 		rla
;16279: 		rr	c
;16280: 		rla
;16281: 		xor	3
;16282: 		add	a, 0Ch
;16283: 		ld	(ix+1Eh), a
;16284: 		set	5, (ix+20h)
;16285: 		ret
;16286: ; End of function objDestroyed2
;16287: 
;16288: ; ---------------------------------------------------------------------------
;16289: ; START	OF FUNCTION CHUNK FOR sub_A7D9
;16290: 
;16291: objPrintLaser:				; CODE XREF: PrnObjIfVisible+9Fj
;16292: 					; PrnObjIfVisible+ABj ...
;16293: 		call	GetCrdsProjection ; de - координаты откуда стреляет
;16294: 		bit	3, (ix+20h)
;16295: 		call	SetCrdCDEinExtSlot
;16296: 		ret	z
;16297: 		jp	objPrnLaser
;16298: ; END OF FUNCTION CHUNK	FOR sub_A7D9
;16299: 
;16300: ; =============== S U B	R O U T	I N E =======================================
;16301: 
;16302: 
;16303: NextExtSlot:				; CODE XREF: sub_9D40+11Dp
;16304: 		ld	hl, (word_B452)
;16305: 		ld	de, 0Ah
;16306: 		add	hl, de
;16307: 		ld	(word_B452), hl
;16308: 		ld	e, 27h ; '''
;16309: 		ret
;16310: ; End of function NextExtSlot
;16311: 
;16312: ; ---------------------------------------------------------------------------
;16313: 		db  7Bh	; {
;16314: TablCrdPrnVertex:db 17h			; DATA XREF: RAM:ADCDo
;16315: 					; GetPrnCrdVertex+2o ...
;16316: 		db  18h
;16317: 		db    0
;16318: 		db  13h
;16319: 		db  13h
;16320: 		db    2
;16321: 		db  17h
;16322: 		db  0Ah
;16323: 		db    2
;16324: 		db  10h
;16325: 		db  14h
;16326: 		db    1
;16327: 		db  16h
;16328: 		db  10h
;16329: 		db    3
;16330: 		db  16h
;16331: BufferCrdY:	db  0Dh			; DATA XREF: RAM:EC5Bo	sub_EDC5+3o
;16332: 		db    1
;16333: 		db    6
;16334: 		db  3Ch	; <
;16335: 		db    1
;16336: 		db  0Eh
;16337: 		db  34h	; 4
;16338: 		db    3
;16339: 		db    5
;16340: 		db  5Bh	; [
;16341: 		db    0
;16342: 		db  0Eh
;16343: 		db  39h	; 9
;16344: 		db    3
;16345: 		db    8
;16346: 		db  39h	; 9
;16347: 		db    1
;16348: 		db    5
;16349: 		db  1Ah
;16350: 		db    3
;16351: 		db  18h
;16352: 		db    6
;16353: 		db    2
;16354: 		db  0Eh
;16355: 		db    1
;16356: 		db    3
;16357: 		db    8
;16358: 		db    7
;16359: 		db    3
;16360: 		db    1
;16361: 		db  12h
;16362: 		db    1
;16363: 		db    2
;16364: 		db  18h
;16365: 		db    3
;16366: 		db  0Ch
;16367: 		db  0Fh
;16368: 		db    3
;16369: 		db  12h
;16370: 		db  0Ah
;16371: 		db    3
;16372: 		db  1Bh
;16373: 		db    0
;16374: 		db    3
;16375: 		db  15h
;16376: 		db  0Dh
;16377: 		db    0
;16378: 		db  19h
;16379: 		db  0Fh
;16380: 		db    0
;16381: 		db  29h	; )
;16382: 		db  10h
;16383: 		db    2
;16384: 		db  27h	; '
;16385: 		db  14h
;16386: 		db    2
;16387: 		db  2Bh	; +
;16388: 		db  14h
;16389: 		db    2
;16390: 		db  14h
;16391: 		db  28h	; (
;16392: 		db    1
;16393: 		db  11h
;16394: 		db  23h	; #
;16395: 		db    1
;16396: 		db  0Fh
;16397: 		db  27h	; '
;16398: 		db    1
;16399: 		db    3
;16400: 		db    0
;16401: 		db    2
;16402: 		db    0
;16403: 		db    0
;16404: 		db    0
;16405: 		db    0
;16406: 		db    0
;16407: 		db    0
;16408: 		db    0
;16409: 		db    0
;16410: 		db    0
;16411: 		db    0
;16412: 		db    0
;16413: 		db    0
;16414: 		db    0
;16415: 		db    0
;16416: 		db    0
;16417: 		db    0
;16418: 		db    0
;16419: 		db    0
;16420: 		db    0
;16421: 		db    0
;16422: 		db    0
;16423: 		db    0
;16424: 		db    0
;16425: 		db    0
;16426: 		db    0
;16427: 		db    0
;16428: 		db    0
;16429: 		db    0
;16430: 		db    0
;16431: 		db    0
;16432: 		db    0
;16433: 		db    0
;16434: 		db    0
;16435: 		db    0
;16436: 		db    0
;16437: 		db    0
;16438: 		db    0
;16439: 		db    0
;16440: 		db    0
;16441: 		db    0
;16442: 		db    0
;16443: 		db    0
;16444: 		db    0
;16445: 		db    0
;16446: 		db    0
;16447: 		db    0
;16448: 		db    0
;16449: 		db    0
;16450: 		db    0
;16451: 		db    0
;16452: 		db    0
;16453: VergesVisible:	db    0			; DATA XREF: CalcVisibleVerges+14o
;16454: 		db    1
;16455: 		db    0
;16456: 		db    1
;16457: 		db    0
;16458: 		db    0
;16459: 		db    0
;16460: 		db    1
;16461: 		db    0
;16462: 		db    1
;16463: byte_A98E:	db 1			; DATA XREF: TestVergesVisible:loc_AA27r
;16464: 		db    0
;16465: 		db    1
;16466: 		db    0
;16467: 		db    1
;16468: 		db    1
;16469: MatrixRotate:	db  7Eh	; ~		; DATA XREF: MakeTablPrnCrdVertex+11o
;16470: 					; CalcMatrixRotateo ...
;16471: 		db 0B3h	; │
;16472: 		db  81h	; Б
;16473: 		db 0F4h	; Ї
;16474: 		db 0F3h	; є
;16475: 		db    7
;16476: 		db  81h	; Б
;16477: 		db  2Ah	; *
;16478: 		db  7Eh	; ~
;16479: 		db  8Bh	; Л
;16480: 		db 0F3h	; є
;16481: 		db  78h	; x
;16482: 		db 0F2h	; Є
;16483: 		db 0ECh	; ь
;16484: 		db 0F2h	; Є
;16485: 		db  33h	; 3
;16486: 		db 0F3h	; є
;16487: 		db  2Ch	; ,
;16488: 		db    0
;16489: 		db  80h	; А
;16490: 		db  16h
;16491: 		db    0
;16492: 		db    6
;16493: 		db  80h	; А
;16494: 		db    0
;16495: 		db    0
;16496: 		db    2
;16497: 		db    0
;16498: 		db  69h	; i
;16499: 		db  80h	; А
;16500: 		db    0
;16501: 		db  80h	; А
;16502: 		db    9
;16503: 		db  80h	; А
;16504: 		db  27h	; '
;16505: 		db  80h	; А
;16506: DistanceToObj:	dw 202h			; DATA XREF: PrintObject_2+8r
;16507: 					; PrintObject_2+51r ...
;16508: PointTablCrdPrn:db 1Eh			; DATA XREF: objPrnObj+1Fw
;16509: 					; MakeTablPrnCrdVertex+16r ...
;16510: VertexCrdX:	dw 10h			; DATA XREF: MakeTablPrnCrdVertex+1B9w
;16511: 					; MakeTablPrnCrdVertex+1CAw ...
;16512: VertexCrdY:	dw 67h			; DATA XREF: MakeTablPrnCrdVertex+1BFw
;16513: 					; MakeTablPrnCrdVertex+1D0w ...
;16514: VertexCrdZ:	dw 1CFh			; DATA XREF: MakeTablPrnCrdVertex:loc_AB9Fw
;16515: 					; MakeTablPrnCrdVertex+313w ...
;16516: CrdXProjection:	db 8			; DATA XREF: sub_7E02+76r
;16517: 					; PrnObjIfVisible+43w ...
;16518: CrdYProjection:	db 39h			; DATA XREF: PrnObjIfVisible+53w
;16519: 					; PrnObjIfVisible+79w ...
;16520: CrdXYsign:	db 1			; DATA XREF: sub_7E02+61r
;16521: 					; PrnObjIfVisible+86w ...
;16522: VertexSignCrdX:	db 0			; DATA XREF: MakeTablPrnCrdVertex+338w
;16523: 					; RAM:AD74r ...
;16524: VertexSignCrdY:	db 80h			; DATA XREF: MakeTablPrnCrdVertex+357w
;16525: 					; RAM:loc_ADB5r
;16526: VertexSignCrdZ:	db 0			; DATA XREF: MakeTablPrnCrdVertex+376w
;16527: 					; RAM:AD7Ar ...
;16528: VertexIncFactor:db 1			; DATA XREF: RotateCrdN_Vertex:loc_AAE3r
;16529: 					; sub_AB1Er ...
;16530: VisiblePrevVertex:db 0			; DATA XREF: MakeTablPrnCrdVertex+3Ew
;16531: 					; RotateCrdN_Vertexr ...
;16532: 
;16533: ; =============== S U B	R O U T	I N E =======================================
;16534: 
;16535: 
;16536: MakeTablPrnCrdVertex:			; CODE XREF: objPrnObj+25p
;16537: 
;16538: ; FUNCTION CHUNK AT AB73 SIZE 0000003C BYTES
;16539: ; FUNCTION CHUNK AT ACA2 SIZE 000000B9 BYTES
;16540: ; FUNCTION CHUNK AT AD82 SIZE 0000000E BYTES
