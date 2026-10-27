;14075: 
;14076: ; ---------------------------------------------------------------------------
;14077: 		jp	Processing_LaserECM
;14078: 
;14079: ; =============== S U B	R O U T	I N E =======================================
;14080: 
;14081: ; Attributes: noreturn
;14082: 
;14083: MoveAllObjects:				; CODE XREF: j_MoveAllObjectsp
;14084: 		ld	a, (PilotMission3) ; =#06 при стыковке будет захват таргонами (вроде нереальная	ситуация)
;14085: 					; =#04 в системе со танцией таргонов во	время войны
;14086: 		cp	4
;14087: 		jr	nz, loc_9CC7
;14088: 		ld	hl, byte_6069
;14089: 		set	2, (hl)
;14090: 
;14091: loc_9CC7:				; CODE XREF: MoveAllObjects+5j
;14092: 		ld	a, (TypeOfPrint)
;14093: 		and	a
;14094: 		ld	a, 0B6h	; '╢'
;14095: 		jr	z, loc_9CD1
;14096: 		ld	a, 0AEh	; 'о'
;14097: 
;14098: loc_9CD1:				; CODE XREF: MoveAllObjects+12j
;14099: 		ld	(unk_B138), a	; тип печати OR/XOR
;14100: 		ld	(loc_B18B), a
;14101: 		ld	(loc_B18E), a
;14102: 		ld	(loc_B1A6), a
;14103: 		ld	a, (FrameCounter)
;14104: 		dec	a
;14105: 		ld	(FrameCounter),	a
;14106: 		ld	b, a
;14107: 		ld	a, (CoriolisNear)
;14108: 		and	a
;14109: 		jr	nz, loc_9CFD
;14110: 		ld	a, (InFreeSpace)
;14111: 		and	a
;14112: 		jr	nz, loc_9CFD
;14113: 		ld	a, (nMissileObj)
;14114: 		cp	4
;14115: 		jr	nc, loc_9CFD
;14116: 		ld	a, b
;14117: 		and	a
;14118: 		call	z, sub_B867	; CallFunctionObj  set rnd ship
;14119: ; ---------------------------------------------------------------------------
;14120: 
;14121: loc_9CFD:				; CODE XREF: MoveAllObjects+2Ej
;14122: 					; MoveAllObjects+34j ...
;14123: 		xor	a
;14124: 		ld	(DamageEnergyUnit), a
;14125: 		ld	(DamageRearShield), a
;14126: 		ld	(DamageFrontShield), a
;14127: 		ld	(ObjInTarget), a
;14128: 		ld	(FlgCoriolisNear), a
;14129: 		ld	(FlgObjNear), a
;14130: 		ld	(J_Drive_Ready4), a
;14131: 		ld	c, a
;14132: 		ld	ix, SlotObj_05
;14133: 		ld	de, 0FFD9h
;14134: 		ld	b, 6
;14135: 
;14136: loc_9D1D:				; CODE XREF: MoveAllObjects+71j
;14137: 		ld	a, (ix+22h)
;14138: 		and	a
;14139: 		jr	z, loc_9D2A
;14140: 		bit	6, (ix+20h)
;14141: 		jr	z, loc_9D2A
;14142: 		inc	c
;14143: 
;14144: loc_9D2A:				; CODE XREF: MoveAllObjects+66j
;14145: 					; MoveAllObjects+6Cj
;14146: 		add	ix, de
;14147: 		djnz	loc_9D1D
;14148: 		ld	a, c		; a,c кол-во ракет в пространстве
;14149: 		call	SetFirstExtSlot	; установка адреса первого допслота
;14150: 		ld	a, d
;14151: 		ld	(PrintObject_2+1), a ; a=0
;14152: 		ld	a, 8
;14153: 
;14154: loc_9D38:				; CODE XREF: sub_F8E4j
;14155: 		push	af
;14156: 		ld	b, a
;14157: 		call	sub_B4B1	; допслот 7,+0 =0
;14158: 		call	sub_ED69	; a,(ix+22h)
;14159: ; End of function MoveAllObjects
;14160: 
;14161: 
;14162: ; =============== S U B	R O U T	I N E =======================================
;14163: 
;14164: 
;14165: sub_9D40:
;14166: 
;14167: ; FUNCTION CHUNK AT F8B4 SIZE 00000023 BYTES
;14168: 
;14169: 		and	a
;14170: 		jp	z, loc_A52E	; следующий объект
;14171: 		call	sub_F916
;14172: 		bit	2, (ix+26h)
;14173: 		jr	nz, loc_9D50
;14174: 		ld	(FlgCoriolisNear), a
;14175: 
;14176: loc_9D50:				; CODE XREF: sub_9D40+Bj
;14177: 		call	CounterShipHyperJump
;14178: 		ld	a, c
;14179: 		bit	3, (ix+26h)
;14180: 		jr	nz, loc_9D5F
;14181: 		ld	a, 1
;14182: 		ld	(J_Drive_Ready4), a
;14183: 
;14184: loc_9D5F:				; CODE XREF: sub_9D40+18j
;14185: 		ld	a, (EnergyBombUsed)
;14186: 		and	a
;14187: 		jr	z, loc_9D76
;14188: 		bit	2, (ix+26h)
;14189: 		jr	nz, loc_9D76
;14190: 		ld	a, (iy+6)
;14191: 		cp	27h ; '''
;14192: 		jr	z, loc_9D76
;14193: 		set	4, (ix+20h)
;14194: 
;14195: loc_9D76:				; CODE XREF: sub_9D40+23j sub_9D40+29j ...
;14196: 		bit	2, (ix+21h)
;14197: 		jr	z, loc_9D88
;14198: 		ld	a, (ix+21h)
;14199: 		and	60h ; '`'
;14200: 		jr	z, loc_9D88
;14201: 		ld	a, 1
;14202: 		ld	(PlayerPirate),	a
;14203: 
;14204: loc_9D88:				; CODE XREF: sub_9D40+3Aj sub_9D40+41j
;14205: 		ld	a, (objMissile.field_22)
;14206: 		and	a
;14207: 		jr	nz, loc_9D94
;14208: 		ld	a, (PlayerPirate)
;14209: 		and	a
;14210: 		jr	z, loc_9D98
;14211: 
;14212: loc_9D94:				; CODE XREF: sub_9D40+4Cj
;14213: 		call	SetAgrFireLaser	; установка агрессии при попадании из лазера
;14214: 		nop
;14215: 
;14216: loc_9D98:				; CODE XREF: sub_9D40+52j
;14217: 		ld	h, (ix+24h)
;14218: 		ld	l, (ix+23h)
;14219: 		push	hl
;14220: 		pop	iy
;14221: 		ld	a, (ix+20h)
;14222: 		and	30h ; '0'
;14223: 		jr	z, loc_9E0F
;14224: 		sub	20h ; ' '
;14225: 		jp	nz, loc_9E41
;14226: 		res	5, (ix+20h)
;14227: 		ld	(ix+1Eh), a
;14228: 		ld	a, 0C0h	; '└'
;14229: 		ld	(ix+1Dh), a
;14230: 		bit	0, (ix+26h)
;14231: 		jr	nz, loc_9DE1
;14232: 		ld	hl, nBeepFX
;14233: 		set	2, (hl)
;14234: 		ld	a, (Mission2)
;14235: 		and	a
;14236: 		jr	z, loc_9DE1
;14237: 		cp	3
;14238: 		jr	z, loc_9DD6
;14239: 		bit	6, (ix+26h)
;14240: 		jr	z, loc_9DE1
;14241: 		jr	loc_9DDD
;14242: ; ---------------------------------------------------------------------------
;14243: 
;14244: loc_9DD6:				; CODE XREF: sub_9D40+8Cj
;14245: 		ld	a, (iy+1)
;14246: 		cp	1Eh
;14247: 		jr	nz, loc_9DE1
;14248: 
;14249: loc_9DDD:				; CODE XREF: sub_9D40+94j
;14250: 		xor	a
;14251: 		ld	(Mission2), a
;14252: 
;14253: loc_9DE1:				; CODE XREF: sub_9D40+7Dj sub_9D40+88j ...
;14254: 		xor	a
;14255: 		ld	(ix+22h), a
;14256: 		call	OffTargetMissiles
;14257: 		ld	d, (iy+9)
;14258: 		ld	e, (iy+8)
;14259: 		call	AddCashObj
;14260: 		ld	d, (iy+16h)
;14261: 		ld	e, (iy+15h)
;14262: 		srl	d
;14263: 		rr	e
;14264: 		call	IncBattleRating
;14265: 		ld	a, (ix+21h)
;14266: 		and	60h ; '`'
;14267: 		jr	z, loc_9E5C
;14268: 		push	hl
;14269: 		ld	hl, PilotLegalStatus
;14270: 		call	sub_FC25
;14271: 		pop	hl
;14272: 		jr	loc_9E5C
;14273: ; ---------------------------------------------------------------------------
;14274: 
;14275: loc_9E0F:				; CODE XREF: sub_9D40+66j
;14276: 		ld	a, b
;14277: 		xor	c
;14278: 		and	0Fh
;14279: 		bit	6, (ix+20h)
;14280: 		jr	z, loc_9E29
;14281: 		call	sub_FA0D
;14282: 		ld	a, c
;14283: 		jr	z, loc_9E24
;14284: 		call	MoveMissile
;14285: 		jr	loc_9E29
;14286: ; ---------------------------------------------------------------------------
;14287: 
;14288: loc_9E24:				; CODE XREF: sub_9D40+DDj
;14289: 		and	3
;14290: 		call	z, ReactionObject2
;14291: 
;14292: loc_9E29:				; CODE XREF: sub_9D40+D7j sub_9D40+E2j
;14293: 		call	ReCalcCrdObj
;14294: 		call	CorrAxirY_SKO
;14295: 		push	ix
;14296: 		ld	de, 9
;14297: 		add	ix, de
;14298: 		call	CorrAxisVector
;14299: 		call	CorrAxisVector
;14300: 		call	CorrAxisVector
;14301: 		pop	ix
;14302: 
;14303: loc_9E41:				; CODE XREF: sub_9D40+6Aj
;14304: 		call	CollisionWithObj
;14305: 		jp	loc_F8B4
;14306: ; ---------------------------------------------------------------------------
;14307: 
;14308: loc_9E47:				; CODE XREF: sub_9D40+5B94j
;14309: 		res	3, (ix+20h)
;14310: 		ld	a, (ix+20h)
;14311: 		and	30h ; '0'
;14312: 		cp	10h
;14313: 		jr	nz, loc_9E5C
;14314: 		set	5, (ix+20h)
;14315: 		res	4, (ix+20h)
;14316: 
;14317: loc_9E5C:				; CODE XREF: CounterShipHyperJump+1Cj
;14318: 					; sub_9D40+C3j	...
;14319: 		pop	af
;14320: 		call	NextExtSlot
;14321: 		add	ix, de
;14322: 		dec	a
;14323: 		jp	sub_F8E4
;14324: ; End of function sub_9D40
;14325: 
;14326: 
;14327: ; =============== S U B	R O U T	I N E =======================================
;14328: 
;14329: 
;14330: MoveAllObjAI:				; CODE XREF: sub_F8E4+16p
;14331: 		xor	a
;14332: 		call	sub_7C60
;14333: 		ld	a, (J_Drive_Ready4)
;14334: 		and	a
;14335: 		ret	nz
;14336: 		ld	(PlayerPirate),	a
;14337: 		ret
;14338: ; End of function MoveAllObjAI
;14339: 
;14340: 
;14341: ; =============== S U B	R O U T	I N E =======================================
;14342: 
;14343: ; Attributes: noreturn
;14344: 
;14345: CalcLenVector:				; CODE XREF: MoveObject+Cp
;14346: 		push	ix
;14347: 		call	Sqr_HL_wordAdrIX
;14348: 		push	hl
;14349: 		call	Sqr_HL_wordAdrIX
;14350: 		push	hl
;14351: 		call	Sqr_HL_wordAdrIX
;14352: 		pop	de
;14353: 		add	hl, de
;14354: 		pop	de
;14355: 		add	hl, de
;14356: 		pop	ix
;14357: 		call	Sqrt_H_HL
;14358: ; ---------------------------------------------------------------------------
;14359: 		ex	de, hl
;14360: 		push	ix
;14361: 		call	CaclNormalVector
;14362: ; ---------------------------------------------------------------------------
;14363: 		call	CaclNormalVector
;14364: ; ---------------------------------------------------------------------------
;14365: 		call	CaclNormalVector
;14366: ; ---------------------------------------------------------------------------
;14367: 		pop	ix
;14368: 		ret
;14369: ; End of function CalcLenVector
;14370: 
;14371: 
;14372: ; =============== S U B	R O U T	I N E =======================================
;14373: 
;14374: 
;14375: Sqr_HL_wordAdrIX:			; CODE XREF: CalcLenVector+2p
;14376: 					; CalcLenVector+6p ...
;14377: 		ld	b, (ix+1)
;14378: 		ld	c, (ix+0)
;14379: 		res	7, b
;14380: 		push	ix
;14381: 		ld	ix, 0
;14382: 		call	Mult_HL_B_C
;14383: 		xor	a
;14384: 		ld	d, a
;14385: 		add	hl, hl
;14386: 		rl	d
;14387: 		ld	e, h
;14388: 		add	ix, de
;14389: 		ld	h, l
;14390: 		ld	l, a
;14391: 		push	hl
;14392: 		ld	h, 0BEh	; '╛'
;14393: 		ld	l, b
;14394: 		ld	e, (hl)
;14395: 		inc	h
;14396: 		ld	d, (hl)
;14397: 		dec	h
;14398: 		add	ix, de
;14399: 		ld	l, c
;14400: 		ld	e, (hl)
;14401: 		inc	h
;14402: 		ld	d, (hl)
;14403: 		pop	hl
;14404: 		add	hl, de
;14405: 		jr	nc, loc_9EC7
;14406: 		inc	ix
;14407: 
;14408: loc_9EC7:				; CODE XREF: Sqr_HL_wordAdrIX+2Bj
;14409: 		ex	(sp), ix
;14410: 		pop	hl
;14411: 		inc	ix
;14412: 		inc	ix
;14413: 		ret
;14414: ; End of function Sqr_HL_wordAdrIX
;14415: 
;14416: 
;14417: ; =============== S U B	R O U T	I N E =======================================
;14418: 
;14419: ; Attributes: noreturn
;14420: 
;14421: CaclNormalVector:			; CODE XREF: CalcLenVector+19p
;14422: 					; CalcLenVector+1Cp ...
;14423: 		ld	h, (ix+1)
;14424: 		ld	l, (ix+0)
;14425: 		push	hl
;14426: 		res	7, h
;14427: 		call	Div_BC_HLAdivDE_1
;14428: ; ---------------------------------------------------------------------------
;14429: 		srl	b
;14430: 		rr	c
;14431: 		pop	hl
;14432: 		bit	7, h
;14433: 		jr	z, loc_9EE6
;14434: 		set	7, b
;14435: 
;14436: loc_9EE6:				; CODE XREF: CaclNormalVector+13j
;14437: 		ld	(ix+0),	c
;14438: 		ld	(ix+1),	b
;14439: 		inc	ix
;14440: 		inc	ix
;14441: 		ret
;14442: ; End of function CaclNormalVector
;14443: 
;14444: 
;14445: ; =============== S U B	R O U T	I N E =======================================
;14446: 
;14447: ; Attributes: noreturn
;14448: 
;14449: Sqrt_H_HL:				; CODE XREF: j_Sqrt_H_HLj
;14450: 					; CalcLenVector+13p
;14451: 		ld	a, h
;14452: 		cp	10h
;14453: 		jr	nc, sub_9F18
;14454: 		and	a
;14455: 		jr	z, loc_9F0A
;14456: 		ld	b, 4
;14457: 
;14458: loc_9EFB:				; CODE XREF: Sqrt_H_HL+Bj
;14459: 		add	hl, hl
;14460: 		djnz	loc_9EFB
;14461: 		call	sub_9F18
;14462: ; ---------------------------------------------------------------------------
;14463: 		srl	h
;14464: 		rr	l
;14465: 		srl	h
;14466: 		rr	l
;14467: 		ret
;14468: ; ---------------------------------------------------------------------------
;14469: 
;14470: loc_9F0A:				; CODE XREF: Sqrt_H_HL+6j
;14471: 		ld	h, l
;14472: 		ld	l, a
;14473: 		call	sub_9F18
;14474: ; ---------------------------------------------------------------------------
;14475: 		ld	b, 4
;14476: 
;14477: loc_9F11:				; CODE XREF: Sqrt_H_HL+24j
;14478: 		srl	h
;14479: 		rr	l
;14480: 		djnz	loc_9F11
;14481: 		ret
;14482: ; End of function Sqrt_H_HL
;14483: 
;14484: 
;14485: ; =============== S U B	R O U T	I N E =======================================
;14486: 
;14487: ; Attributes: noreturn
;14488: 
;14489: sub_9F18:				; CODE XREF: Sqrt_H_HL+3j Sqrt_H_HL+Dp ...
;14490: 		xor	a
;14491: 		push	hl
;14492: 		push	hl
;14493: 		srl	h
;14494: 		rr	l
;14495: 		rra
;14496: 		ld	de, 8000h
;14497: 		call	loc_9F35
;14498: ; ---------------------------------------------------------------------------
;14499: 		pop	de
;14500: 		call	loc_9F2B
