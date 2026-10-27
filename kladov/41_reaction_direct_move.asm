;15140: LenVector:	dw 1585h		; DATA XREF: MoveObject+11w
;15141: 					; MoveObject+8Br
;15142: 
;15143: ; =============== S U B	R O U T	I N E =======================================
;15144: 
;15145: 
;15146: ReactionObject:				; CODE XREF: ReactionObject2+7p
;15147: 		ld	a, (iy+7)
;15148: 		ld	c, (ix+22h)
;15149: 		inc	c
;15150: 		cp	0F0h ; 'Ё'
;15151: 		jr	c, loc_A2CE
;15152: 		inc	c
;15153: 		inc	c
;15154: 		inc	c
;15155: 
;15156: loc_A2CE:				; CODE XREF: ReactionObject+9j
;15157: 		cp	c
;15158: 		jr	c, loc_A2D2
;15159: 		ld	a, c
;15160: 
;15161: loc_A2D2:				; CODE XREF: ReactionObject+Fj
;15162: 		ld	b, a
;15163: 		ld	a, (CloakingDeviceOn)
;15164: 		and	a
;15165: 		nop
;15166: 		ld	hl, Mission2
;15167: 		ld	a, b
;15168: 		cp	50h ; 'P'
;15169: 		ld	a, (hl)
;15170: 		jr	c, loc_A2E6
;15171: 		cp	2
;15172: 		jr	nz, loc_A2E6
;15173: 		dec	(hl)
;15174: 
;15175: loc_A2E6:				; CODE XREF: ReactionObject+1Fj
;15176: 					; ReactionObject+23j
;15177: 		cp	1
;15178: 		jr	nz, loc_A2F7
;15179: 		ld	a, b
;15180: 		cp	34h ; '4'
;15181: 		jr	nc, loc_A2F4	; энергия >=#34
;15182: 		ld	a, 2
;15183: 		ld	(hl), a
;15184: 		jr	loc_A2F7
;15185: ; ---------------------------------------------------------------------------
;15186: 
;15187: loc_A2F4:				; CODE XREF: ReactionObject+2Dj
;15188: 		sub	2
;15189: 		ld	b, a
;15190: 
;15191: loc_A2F7:				; CODE XREF: ReactionObject+28j
;15192: 					; ReactionObject+32j
;15193: 		ld	a, b
;15194: 		ld	(ix+22h), a
;15195: 		ld	b, (iy+7)
;15196: 		add	a, a
;15197: 		jr	c, loc_A36C	; энергии от 50% до 100%
;15198: 		cp	b
;15199: 		jr	nc, loc_A36C	; энергии от 50% до 100%
;15200: 		add	a, a
;15201: 		jr	c, loc_A30D	; энергии от 13% до 49%
;15202: 					; попытка запуска ракеты/корабля
;15203: 		add	a, a
;15204: 		jr	c, loc_A30D	; энергии от 13% до 49%
;15205: 					; попытка запуска ракеты/корабля
;15206: 		cp	b
;15207: 		jr	c, loc_A343	; критический уровень энергии. менее 13%
;15208: 					; попытка запуска объектом спасательной	капсулы/сброс груза
;15209: 
;15210: loc_A30D:				; CODE XREF: ReactionObject+45j
;15211: 					; ReactionObject+48j
;15212: 		ld	a, (ix+20h)	; энергии от 13% до 49%
;15213: 					; попытка запуска ракеты/корабля
;15214: 		and	7
;15215: 		jr	z, loc_A36C	; энергии от 50% до 100%
;15216: 		call	IntRnd_A
;15217: 		cp	80h ; 'А'
;15218: 		jr	nc, loc_A36C	; энергии от 50% до 100%
;15219: 		ld	a, (TimeWorkECM)
;15220: 		and	a
;15221: 		jr	nz, loc_A36C	; энергии от 50% до 100%
;15222: 		ld	a, (iy+0Ah)
;15223: 		and	0F0h ; 'Ё'
;15224: 		jr	nz, loc_A32A
;15225: 		or	6
;15226: 
;15227: loc_A32A:				; CODE XREF: ReactionObject+66j
;15228: 		xor	4
;15229: 		ld	b, a
;15230: 		push	bc
;15231: 		call	CallFunction_Obj
;15232: ; ---------------------------------------------------------------------------
;15233: 		pop	bc
;15234: 		jr	nz, loc_A36C	; энергии от 50% до 100%
;15235: 		dec	(ix+20h)
;15236: 		ld	a, b
;15237: 		cp	2
;15238: 		jr	nz, loc_A36C	; энергии от 50% до 100%
;15239: 		ld	hl, ShipLaunchMisile
;15240: 		set	7, (hl)
;15241: 		jr	loc_A36C	; энергии от 50% до 100%
;15242: ; ---------------------------------------------------------------------------
;15243: 
;15244: loc_A343:				; CODE XREF: ReactionObject+4Bj
;15245: 		call	IntRnd_A	; критический уровень энергии. менее 13%
;15246: 					; попытка запуска объектом спасательной	капсулы/сброс груза
;15247: 		cp	(ix+25h)
;15248: 		jr	c, loc_A3A2	; (шанс	запуска	EscapePod не сработал/это не тарглет)
;15249: 					; попытка включить ECM
;15250: 		bit	7, (ix+20h)
;15251: 		jr	z, loc_A361	; сброс	грузового отсека
;15252: 		ld	a, 7
;15253: 		call	CallFunction_Obj
;15254: ; ---------------------------------------------------------------------------
;15255: 		jr	nz, loc_A36C	; энергии от 50% до 100%
;15256: 		res	7, (ix+20h)
;15257: 		res	6, (ix+20h)
;15258: 		ret
;15259: ; ---------------------------------------------------------------------------
;15260: 
;15261: loc_A361:				; CODE XREF: ReactionObject+8Fj
;15262: 		ld	a, (iy+0)	; сброс	грузового отсека
;15263: 		and	a
;15264: 		jr	z, loc_A36C	; энергии от 50% до 100%
;15265: 		ld	a, 6
;15266: 		call	CallFunction_Obj
;15267: ; ---------------------------------------------------------------------------
;15268: 
;15269: loc_A36C:				; CODE XREF: ReactionObject+3Fj
;15270: 					; ReactionObject+42j ...
;15271: 		ld	a, (iy+12h)	; энергии от 50% до 100%
;15272: 		cp	4
;15273: 		jr	nz, loc_A3A2	; (шанс	запуска	EscapePod не сработал/это не тарглет)
;15274: 					; попытка включить ECM
;15275: 		push	ix		; обработка тарглета
;15276: 		ld	ix, objCoriolis
;15277: 		ld	de, 27h	; '''
;15278: 		ld	b, 6
;15279: 
;15280: loc_A37E:				; CODE XREF: ReactionObject:loc_A38Dj
;15281: 		add	ix, de
;15282: 		ld	a, (ix+22h)
;15283: 		and	a
;15284: 		jr	z, loc_A38D
;15285: 		ld	a, (ix+21h)
;15286: 		sub	0Ch
;15287: 		jr	z, loc_A390
;15288: 
;15289: loc_A38D:				; CODE XREF: ReactionObject+C4j
;15290: 		djnz	loc_A37E
;15291: 		inc	a
;15292: 
;15293: loc_A390:				; CODE XREF: ReactionObject+CBj
;15294: 		pop	ix
;15295: 		jp	z, DirectObject
;15296: 		res	6, (ix+20h)
;15297: 		set	1, (ix+26h)
;15298: 		srl	(ix+1Bh)
;15299: 		ret
;15300: ; ---------------------------------------------------------------------------
;15301: 
;15302: loc_A3A2:				; CODE XREF: ReactionObject+89j
;15303: 					; ReactionObject+B1j
;15304: 		bit	0, (ix+25h)	; (шанс	запуска	EscapePod не сработал/это не тарглет)
;15305: 					; попытка включить ECM
;15306: 		jr	z, loc_A3D3
;15307: 		ld	a, (PilotMission3) ; =#06 при стыковке будет захват таргонами (вроде нереальная	ситуация)
;15308: 					; =#04 в системе со танцией таргонов во	время войны
;15309: 		cp	4
;15310: 		jr	nz, loc_A3B6
;15311: 		ld	a, (PilotExtWeapon) ; 7,=1 - ECM Jammer
;15312: 					; 6,=1 - Cloaking device
;15313: 					; 5,=1 - нельзя	купить товары(в	трюме беженцы или десант)
;15314: 					; 4
;15315: 					; 3 =1 3я миссия была выполнена
;15316: 					; 2 =1 - третья	миссия выполнена
;15317: 					; 1 =1 - второя	миссия выполнена
;15318: 					; 0 =1 - первая	миссия выполнена
;15319: 		bit	7, a
;15320: 		jr	nz, loc_A3D3
;15321: 
;15322: loc_A3B6:				; CODE XREF: ReactionObject+EDj
;15323: 		ld	a, (TimeWorkECM)
;15324: 		and	a
;15325: 		jr	nz, loc_A3D3
;15326: 		push	ix
;15327: 		call	sub_F95A	; проверка есть	ли ракета нацеленная на	текущий	объект
;15328: 		ld	a, (ix+23h)
;15329: 		pop	ix
;15330: 		jr	nc, loc_A3D1
;15331: 		cp	0E3h ; 'у'
;15332: 		jr	nz, loc_A3D1
;15333: 		ld	a, 8
;15334: 		ld	(TimeWorkECM), a
;15335: 
;15336: loc_A3D1:				; CODE XREF: ReactionObject+106j
;15337: 					; ReactionObject+10Aj
;15338: 		nop
;15339: 		nop
;15340: 
;15341: loc_A3D3:				; CODE XREF: ReactionObject+E6j
;15342: 					; ReactionObject+F4j ...
;15343: 		bit	2, (ix+26h)
;15344: 		jr	z, DirectObject
;15345: 		ld	a, (nMissileObj)
;15346: 		cp	4
;15347: 		ret	nc
;15348: 		call	IntRnd_A
;15349: 		bit	2, (ix+21h)
;15350: 		jr	z, loc_A3FE
;15351: 		cp	24h ; '$'
;15352: 		ret	nc
;15353: 		ld	a, (PilotMission3) ; =#06 при стыковке будет захват таргонами (вроде нереальная	ситуация)
;15354: 					; =#04 в системе со танцией таргонов во	время войны
;15355: 		cp	4
;15356: 		ld	a, 14h
;15357: 		jr	nz, CallFunction_Obj
;15358: 		call	IntRnd_A
;15359: 		cp	88h ; 'И'
;15360: 		ret	nc
;15361: 		ld	a, 44h ; 'D'
;15362: 		jr	CallFunction_Obj
;15363: ; ---------------------------------------------------------------------------
;15364: 
;15365: loc_A3FE:				; CODE XREF: ReactionObject+126j
;15366: 		cp	2
;15367: 		ret	nc
;15368: 		ld	a, 24h ; '$'
;15369: ; End of function ReactionObject
;15370: 
;15371: 
;15372: ; =============== S U B	R O U T	I N E =======================================
;15373: 
;15374: ; Attributes: noreturn
;15375: 
;15376: CallFunction_Obj:			; CODE XREF: ReactionObject+6Ep
;15377: 					; ReactionObject+93p ...
;15378: 		push	iy
;15379: 		push	ix
;15380: 		call	SetObjbyShip	; установка объекта кораблем противника
;15381: ; ---------------------------------------------------------------------------
;15382: 		pop	ix
;15383: 		pop	iy
;15384: 		ret
;15385: ; End of function CallFunction_Obj
;15386: 
;15387: 
;15388: ; =============== S U B	R O U T	I N E =======================================
;15389: 
;15390: 
;15391: TestObjInRadius:			; CODE XREF: DirectObject+14p
;15392: 					; DirectObject+21p
;15393: 		ld	h, (ix+1)
;15394: 		ld	l, (ix+0)
;15395: 		and	a
;15396: 		sbc	hl, de
;15397: 		ret	nc
;15398: 		ld	h, (ix+4)
;15399: 		ld	l, (ix+3)
;15400: 		and	a
;15401: 		sbc	hl, de
;15402: 		ret	nc
;15403: 		ld	h, (ix+7)
;15404: 		ld	l, (ix+6)
;15405: 		inc	d
;15406: 		inc	d
;15407: 		and	a
;15408: 		sbc	hl, de
;15409: 		ret
;15410: ; End of function TestObjInRadius
;15411: 
;15412: 
;15413: ; =============== S U B	R O U T	I N E =======================================
;15414: 
;15415: ; Attributes: noreturn
;15416: 
;15417: DirectObject:				; CODE XREF: ReactionObject+D2j
;15418: 					; ReactionObject+117j
;15419: 		call	GetDirectObj
;15420: 		ret	nc
;15421: 		jr	z, loc_A440
;15422: 		ld	a, (PilotRating0)
;15423: 		cp	28h ; '('
;15424: 		jr	c, loc_A440
;15425: 		set	2, (ix+21h)
;15426: 
;15427: loc_A440:				; CODE XREF: DirectObject+4j
;15428: 					; DirectObject+Bj
;15429: 		ld	de, 200h
;15430: 		call	TestObjInRadius	; в радиусе реакции
;15431: 		jr	c, MoveObjFromPlayer
;15432: 		bit	5, (ix+26h)
;15433: 		jr	z, loc_A455
;15434: 		ld	d, 0Ah
;15435: 		call	TestObjInRadius
;15436: 		jr	c, MoveObjFromPlayer
;15437: 
;15438: loc_A455:				; CODE XREF: DirectObject+1Dj
;15439: 		bit	2, (ix+21h)
;15440: 		ret	z
;15441: 		ld	a, (PilotMission3) ; =#06 при стыковке будет захват таргонами (вроде нереальная	ситуация)
;15442: 					; =#04 в системе со танцией таргонов во	время войны
;15443: 		cp	4
;15444: 		jr	z, loc_A46F
;15445: 		ld	a, (byte_606A)	; objCoriolis+oEnerg
;15446: 		and	a
;15447: 		jr	z, loc_A46F
;15448: 		bit	3, (ix+21h)
;15449: 		jr	z, MoveObjToPlayer
;15450: 		jr	MoveObjFromPlayer
;15451: ; ---------------------------------------------------------------------------
;15452: 
;15453: loc_A46F:				; CODE XREF: DirectObject+30j
;15454: 					; DirectObject+36j
;15455: 		ld	a, (ix+25h)
;15456: 		and	7Fh ; ''
;15457: 		cp	8
;15458: 		jr	nc, MoveObjToPlayer
;15459: 		set	7, (ix+25h)
;15460: 
;15461: MoveObjFromPlayer:			; CODE XREF: DirectObject+17j
;15462: 					; DirectObject+24j ...
;15463: 		push	ix
;15464: 		call	HL_CrdNdiv4
;15465: 		ld	(deltaCrdX), hl
;15466: 		call	HL_CrdNdiv4
;15467: 		ld	(deltaCrdY), hl
;15468: 		call	HL_CrdNdiv4
;15469: 		ld	(deltaCrdZ), hl
;15470: 		pop	ix
;15471: 		set	5, (ix+26h)
;15472: 		jr	MoveObject1
;15473: ; ---------------------------------------------------------------------------
;15474: 
;15475: MoveObjToPlayer:			; CODE XREF: DirectObject+3Cj
;15476: 					; DirectObject+47j ...
;15477: 		push	ix
;15478: 		call	HL_CrdNdiv4_InvSign
;15479: 		ld	(deltaCrdX), hl
;15480: 		call	HL_CrdNdiv4_InvSign
;15481: 		ld	(deltaCrdY), hl
;15482: 		call	HL_CrdNdiv4_InvSign
;15483: 		ld	(deltaCrdZ), hl
;15484: 		pop	ix
;15485: 		res	5, (ix+26h)
;15486: 
;15487: MoveObject1:				; CODE XREF: DirectObject+67j
;15488: 					; MoveMissile+7Fj
;15489: 		jp	MoveObject
;15490: ; End of function DirectObject
;15491: 
;15492: 
;15493: ; =============== S U B	R O U T	I N E =======================================
;15494: 
;15495: 
;15496: MoveMissile:				; CODE XREF: sub_9D40+DFp
;15497: 		ld	a, (TimeWorkECM)
;15498: 		and	a
;15499: 		jr	z, loc_A4C4
;15500: 		call	sub_7D3E
;15501: 		ret	nz
;15502: 		set	4, (ix+20h)
;15503: 		ret
;15504: ; ---------------------------------------------------------------------------
;15505: 
;15506: loc_A4C4:				; CODE XREF: MoveMissile+4j
;15507: 		ld	a, c
;15508: 		and	0Fh
;15509: 		and	a
;15510: 		jr	z, MoveObjToPlayer
;15511: 		push	iy
;15512: 		ld	iy, loc_6021
;15513: 		ld	de, 27h	; '''
;15514: 		sub	0Ah
;15515: 		cpl
;15516: 		ld	b, a
;15517: 
;15518: loc_A4D7:				; CODE XREF: MoveMissile+24j
;15519: 		add	iy, de
;15520: 		djnz	loc_A4D7
;15521: 		push	ix
;15522: 		push	iy
;15523: 		call	sub_A53F
;15524: 		ld	(deltaCrdX), hl
;15525: 		call	sub_A53F
;15526: 		ld	(deltaCrdY), hl
;15527: 		call	sub_A53F
;15528: 		ld	(deltaCrdZ), hl
;15529: 		pop	iy
;15530: 		pop	ix
;15531: 		call	sub_A537
;15532: 		jr	nz, loc_A532
;15533: 		ld	hl, (deltaCrdY)
;15534: 		call	sub_A537
;15535: 		jr	nz, loc_A532
;15536: 		ld	hl, (deltaCrdX)
;15537: 		call	sub_F97E
;15538: 		jr	nz, loc_A532
;15539: 		bit	2, (iy+26h)
;15540: 		jr	z, loc_A527
;15541: 		ld	hl, PilotMission3 ; =#06 при стыковке будет захват таргонами (вроде нереальная ситуация)
;15542: 					; =#04 в системе со танцией таргонов во	время войны
;15543: 		call	sub_D6AA
;15544: 		jr	nz, loc_A52B
;15545: 		ld	hl, planetAlienItems
;15546: 		xor	a
;15547: 		ld	(hl), a
;15548: 		ld	de, 0FFFFh
;15549: 		call	j_IncBattleRating2
;15550: 		set	7, (iy+26h)
;15551: 
;15552: loc_A527:				; CODE XREF: MoveMissile+59j
;15553: 		set	4, (iy+20h)
;15554: 
;15555: loc_A52B:				; CODE XREF: MoveMissile+61j
;15556: 		pop	iy
;15557: 		ret
;15558: ; ---------------------------------------------------------------------------
;15559: 
;15560: loc_A52E:				; CODE XREF: sub_9D40+1j
;15561: 		ld	(hl), a
;15562: 		jp	loc_9E5C
;15563: ; ---------------------------------------------------------------------------
;15564: 
;15565: loc_A532:				; CODE XREF: MoveMissile+43j
;15566: 					; MoveMissile+4Bj ...
;15567: 		pop	iy
;15568: 		jp	MoveObject1
;15569: ; End of function MoveMissile
;15570: 
;15571: 
;15572: ; =============== S U B	R O U T	I N E =======================================
;15573: 
;15574: 
;15575: sub_A537:				; CODE XREF: MoveMissile+40p
;15576: 					; MoveMissile+48p ...
;15577: 		add	hl, hl
;15578: 		ld	a, h
;15579: 		and	a
;15580: 		ret	nz
;15581: 		ld	a, l
;15582: 		and	0C0h ; '└'
;15583: 		ret
;15584: ; End of function sub_A537
;15585: 
;15586: 
;15587: ; =============== S U B	R O U T	I N E =======================================
;15588: 
;15589: 
;15590: sub_A53F:				; CODE XREF: MoveMissile+2Ap
;15591: 					; MoveMissile+30p ...
;15592: 		ld	l, (iy+0)
;15593: 		ld	h, (iy+1)
;15594: 		ld	c, (iy+2)
;15595: 		ld	e, (ix+0)
;15596: 		ld	d, (ix+1)
;15597: 		ld	b, (ix+2)
;15598: 		sla	c
;15599: 		rr	h
;15600: 		rr	l
;15601: 		sla	b
;15602: 		rr	d
;15603: 		rr	e
;15604: 		call	sub_AFC9
;15605: 		ld	de, 3
;15606: 		add	ix, de
;15607: 		add	iy, de
;15608: 		ret
;15609: ; End of function sub_A53F
;15610: 
;15611: 
;15612: ; =============== S U B	R O U T	I N E =======================================
;15613: 
;15614: 
;15615: HL_CrdNdiv4_InvSign:			; CODE XREF: DirectObject+6Bp
;15616: 					; DirectObject+71p ...
;15617: 		ld	a, (ix+2)
;15618: 		xor	80h ; 'А'
;15619: 		jr	loc_A572
;15620: ; End of function HL_CrdNdiv4_InvSign
;15621: 
;15622: 
;15623: ; =============== S U B	R O U T	I N E =======================================
;15624: 
;15625: 
;15626: HL_CrdNdiv4:				; CODE XREF: DirectObject+4Fp
;15627: 					; DirectObject+55p ...
;15628: 		ld	a, (ix+2)
;15629: 
;15630: loc_A572:				; CODE XREF: HL_CrdNdiv4_InvSign+5j
;15631: 		ld	l, (ix+0)
;15632: 		ld	h, (ix+1)
;15633: 		srl	h
;15634: 		rr	l
;15635: 		rla
;15636: 		rr	h
;15637: 		rr	l
;15638: 		ld	de, 3
;15639: 		add	ix, de
;15640: 		ret
;15641: ; End of function HL_CrdNdiv4
;15642: 
;15643: 
;15644: ; =============== S U B	R O U T	I N E =======================================
;15645: 
;15646: ; Attributes: noreturn
;15647: 
;15648: MoveObject:				; CODE XREF: DirectObject:MoveObject1j
;15649: 		push	iy
;15650: 		ld	iy, deltaCrdX
