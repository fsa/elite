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
;17851: 		inc	a
;17852: 		ret	z
;17853: 		ld	a, e
;17854: 		and	h
;17855: 		inc	a
;17856: 		ret	z
;17857: 		ld	a, c
;17858: 		add	a, a
;17859: 		add	a, a
;17860: 		xor	c
;17861: 		cpl
;17862: 		ld	c, a
;17863: 		call	sub_B095
;17864: 		ld	a, d
;17865: 		sub	78h ; 'x'
;17866: 		jr	c, loc_B065
;17867: 		ld	b, 14h
;17868: 		call	loc_B21B
;17869: 		ld	e, a
;17870: 		ld	b, 0
;17871: 		call	sub_B09C
;17872: 
;17873: loc_B065:				; CODE XREF: PrnLineInBuf_clip+14j
;17874: 		ld	a, e
;17875: 		sub	3Fh ; '?'
;17876: 		jr	c, loc_B073
;17877: 		ld	b, 28h ; '('
;17878: 		call	loc_B1D7
;17879: 		ld	d, a
;17880: 		call	sub_B095
;17881: 
;17882: loc_B073:				; CODE XREF: PrnLineInBuf_clip+24j
;17883: 		ld	a, h
;17884: 		sub	78h ; 'x'
;17885: 		jr	c, loc_B085
;17886: 		ex	de, hl
;17887: 		ld	b, 5
;17888: 		call	loc_B21B
;17889: 		ld	e, a
;17890: 		ex	de, hl
;17891: 		ld	b, 0
;17892: 		call	loc_B0A6
;17893: 
;17894: loc_B085:				; CODE XREF: PrnLineInBuf_clip+32j
;17895: 		ld	a, l
;17896: 		sub	3Fh ; '?'
;17897: 		jr	c, loc_B0B6
;17898: 		ex	de, hl
;17899: 		ld	b, 0Ah
;17900: 		call	loc_B1D7
;17901: 		ld	d, a
;17902: 		ex	de, hl
;17903: 		jp	loc_B0B6
;17904: ; End of function PrnLineInBuf_clip
;17905: 
;17906: 
;17907: ; =============== S U B	R O U T	I N E =======================================
;17908: 
;17909: 
;17910: sub_B095:				; CODE XREF: PrnLineInBuf_clip+Ep
;17911: 					; PrnLineInBuf_clip+2Cp
;17912: 		ld	b, 0
;17913: 		ld	a, 77h ; 'w'
;17914: 		cp	d
;17915: 		rl	b
;17916: ; End of function sub_B095
;17917: 
;17918: 
;17919: ; =============== S U B	R O U T	I N E =======================================
;17920: 
;17921: 
;17922: sub_B09C:				; CODE XREF: PrnLineInBuf_clip+1Ep
;17923: 		ld	a, 3Eh ; '>'
;17924: 		cp	e
;17925: 		rl	b
;17926: 		ld	a, 77h ; 'w'
;17927: 		cp	h
;17928: 		rl	b
;17929: ; End of function sub_B09C
;17930: 
;17931: 
;17932: loc_B0A6:				; CODE XREF: PrnLineInBuf_clip+3Ep
;17933: 		ld	a, 3Eh ; '>'
;17934: 		cp	l
;17935: 		rl	b
;17936: 		jr	z, loc_B0B3
;17937: 		ld	a, b
;17938: 		rlca
;17939: 		rlca
;17940: 		and	b
;17941: 		and	c
;17942: 		ret	z
;17943: 
;17944: loc_B0B3:				; CODE XREF: RAM:B0ABj
;17945: 		inc	sp
;17946: 		inc	sp
;17947: 		ret	nz
;17948: ; START	OF FUNCTION CHUNK FOR PrnLineInBuf_clip
;17949: 
;17950: loc_B0B6:				; CODE XREF: PrnLineInBuf_clip+44j
;17951: 					; PrnLineInBuf_clip+4Ej
;17952: 		ld	a, 7Fh ; ''
;17953: 		bit	5, c
;17954: 		jr	nz, loc_B0BD+1
;17955: 		sub	d
;17956: 
;17957: loc_B0BD:				; CODE XREF: PrnLineInBuf_clip+76j
;17958: 		cp	82h ; 'В'
;17959: 		ld	d, a
;17960: 		ld	a, 3Fh ; '?'
;17961: 		bit	4, c
;17962: 		jr	z, loc_B0C7+1
;17963: 		sub	e
;17964: 
;17965: loc_B0C7:				; CODE XREF: PrnLineInBuf_clip+80j
;17966: 		cp	83h ; 'Г'
;17967: 		ld	e, a
;17968: 		ld	a, 7Fh ; ''
;17969: 		bit	1, c
;17970: 		jr	nz, loc_B0D1+1
;17971: 		sub	h
;17972: 
;17973: loc_B0D1:				; CODE XREF: PrnLineInBuf_clip+8Aj
;17974: 		cp	84h ; 'Д'
;17975: 		ld	h, a
;17976: 		ld	a, 3Fh ; '?'
;17977: 		bit	0, c
;17978: 		jr	z, loc_B0DB+1
;17979: 		sub	l
;17980: 
;17981: loc_B0DB:				; CODE XREF: PrnLineInBuf_clip+94j
;17982: 		cp	85h ; 'Е'
;17983: 		ld	l, a
;17984: 		cp	e
;17985: 		jr	nc, loc_B0E2
;17986: 		ex	de, hl
;17987: 
;17988: loc_B0E2:				; CODE XREF: PrnLineInBuf_clip+9Bj
;17989: 		push	hl
;17990: 		ld	a, d
;17991: 		and	7
;17992: 		ld	hl, 0B15Ah
;17993: 		add	a, l
;17994: 		ld	l, a
;17995: 		ld	c, (hl)
;17996: 		ld	a, e
;17997: 		and	a
;17998: 		rra
;17999: 		scf
;18000: 		rra
;18001: 		scf
;18002: 		rra
;18003: 		ld	l, a
;18004: 		xor	e
;18005: 		and	0F8h ; '°'
;18006: 		xor	e
;18007: 		ld	h, a
;18008: 		ld	a, d
;18009: 		xor	l
;18010: 		and	0F8h ; '°'
;18011: 		xor	l
;18012: 		rrca
;18013: 		rrca
;18014: 		rrca
;18015: 		ld	l, a
;18016: 		ex	(sp), hl
;18017: 		ld	a, l
;18018: 		sub	e
;18019: 		ld	e, a
;18020: 		ld	a, h
;18021: 		sub	d
;18022: 		push	af
;18023: 		jr	nc, loc_B10D
;18024: 		neg
;18025: 
;18026: loc_B10D:				; CODE XREF: PrnLineInBuf_clip+C5j
;18027: 		ld	d, a
;18028: 		cp	e
;18029: 		jp	nc, loc_B162
;18030: 		pop	af
;18031: 		pop	hl
;18032: 		ld	a, 1
;18033: 		jr	c, loc_B11A
;18034: 		ld	a, 9
;18035: 
;18036: loc_B11A:				; CODE XREF: PrnLineInBuf_clip+D2j
;18037: 		ld	(unk_B152), a
;18038: 		ld	a, 2Ch ; ','
;18039: 		adc	a, 0
;18040: 		ld	(unk_B156), a
;18041: 		ld	a, h
;18042: 		and	7
;18043: 		neg
;18044: 		add	a, 8
;18045: 		ld	b, a
;18046: 		xor	a
;18047: 		sub	e
;18048: 		ld	(unk_B150), a
;18049: 		inc	e
;18050: 		add	a, d
;18051: 		jr	nc, loc_B136
;18052: 		sub	d
;18053: 
;18054: loc_B136:				; CODE XREF: PrnLineInBuf_clip+EFj
;18055: 		ex	af, af'
;18056: 		ld	a, c
;18057: ; END OF FUNCTION CHUNK	FOR PrnLineInBuf_clip
;18058: ; ---------------------------------------------------------------------------
;18059: unk_B138:	db 0B6h	; ╢		; DATA XREF: MoveAllObjects:loc_9CD1w
;18060: 					; j_MoveAllObjects+1B16r
;18061: 					; тип печати OR/XOR
;18062: 		db  77h	; w
;18063: 		db  1Dh
;18064: 		db 0C8h	; ╚
;18065: 		db  24h	; $
;18066: 		db  10h
;18067: 		db  0Bh
;18068: 		db    6
;18069: 		db    8
;18070: 		db  7Dh	; }
;18071: 		db 0C6h	; ╞
;18072: 		db  20h
;18073: 		db  6Fh	; o
;18074: 		db  38h	; 8
;18075: 		db    3
;18076: 		db  7Ch	; |
;18077: 		db  90h	; Р
;18078: 		db  67h	; g
;18079: 		db    8
;18080: 		db  82h	; В
;18081: 		db 0D2h	; ╥
;18082: 		db  36h	; 6
;18083: 		db 0B1h	; ▒
;18084: 		db 0C6h	; ╞
;18085: unk_B150:	db 0BDh	; ╜		; DATA XREF: PrnLineInBuf_clip+EAw
;18086: 		db 0CBh	; ╦
;18087: unk_B152:	db    9			; DATA XREF: PrnLineInBuf_clip:loc_B11Aw
;18088: 		db 0D2h	; ╥
;18089: 		db  36h	; 6
;18090: 		db 0B1h	; ▒
;18091: unk_B156:	db  2Ch	; ,		; DATA XREF: PrnLineInBuf_clip+DDw
;18092: 		db 0C3h	; ├
;18093: 		db  36h	; 6
;18094: 		db 0B1h	; ▒
;18095: 		db  80h	; А
;18096: 		db  40h	; @
;18097: 		db  20h
;18098: 		db  10h
;18099: 		db    8
;18100: 		db    4
;18101: 		db    2
;18102: 		db    1
;18103: ; ---------------------------------------------------------------------------
;18104: ; START	OF FUNCTION CHUNK FOR PrnLineInBuf_clip
;18105: 
;18106: loc_B162:				; CODE XREF: PrnLineInBuf_clip+CBj
;18107: 		pop	af
;18108: 		pop	hl
;18109: 		ld	a, 1
;18110: 		jr	c, loc_B16A
;18111: 
;18112: loc_B168:				; CODE XREF: RAM:loc_B1E1j
;18113: 		ld	a, 9
;18114: 
;18115: loc_B16A:				; CODE XREF: PrnLineInBuf_clip+122j
;18116: 		ld	(loc_B19A+1), a
;18117: 
;18118: loc_B16D:
;18119: 		ld	a, 2Ch ; ','
;18120: 		adc	a, 0
;18121: 		ld	(loc_B191), a
;18122: 		ld	a, h
;18123: 		and	7
;18124: 
;18125: loc_B177:				; CODE XREF: RAM:loc_B1F2j
;18126: 		neg
;18127: 		add	a, 8
;18128: 		ld	b, a
;18129: 		xor	a
;18130: 		sub	d
;18131: 		ld	(loc_B1A3+1), a
;18132: 		inc	d
;18133: 		add	a, e
;18134: 		jr	nc, loc_B186
;18135: 		sub	e
;18136: 
;18137: loc_B186:				; CODE XREF: PrnLineInBuf_clip+13Fj
;18138: 		ex	af, af'
;18139: 		ld	a, c
;18140: 		call	loc_B198
;18141: ; END OF FUNCTION CHUNK	FOR PrnLineInBuf_clip
;18142: 
;18143: loc_B18B:				; DATA XREF: MoveAllObjects+19w
;18144: 		or	(hl)
;18145: 		ld	(hl), a
;18146: 		ret
;18147: ; ---------------------------------------------------------------------------
;18148: 
;18149: loc_B18E:				; CODE XREF: RAM:B19Cj
;18150: 					; DATA XREF: MoveAllObjects+1Cw
;18151: 		or	(hl)
;18152: 		ld	(hl), a
;18153: 		xor	a
;18154: 
;18155: loc_B191:				; DATA XREF: PrnLineInBuf_clip+12Dw
;18156: 		inc	l
;18157: 		ex	af, af'
;18158: 		add	a, e
;18159: 		jr	c, loc_B1A3
;18160: 
;18161: loc_B196:				; CODE XREF: RAM:B1A0j
;18162: 		ex	af, af'
;18163: 		or	c
;18164: 
;18165: loc_B198:				; CODE XREF: PrnLineInBuf_clip+144p
;18166: 					; RAM:loc_B1AAj ...
;18167: 		dec	d
;18168: 		ret	z
;18169: 
;18170: loc_B19A:				; DATA XREF: PrnLineInBuf_clip:loc_B16Aw
;18171: 		rrc	c
;18172: 		jr	c, loc_B18E
;18173: 		ex	af, af'
;18174: 		add	a, e
;18175: 		jp	nc, loc_B196
;18176: 
;18177: loc_B1A3:				; CODE XREF: RAM:B194j
;18178: 					; DATA XREF: PrnLineInBuf_clip+13Aw
;18179: 		add	a, 0EAh	; 'ъ'
;18180: 		ex	af, af'
;18181: 
;18182: loc_B1A6:				; DATA XREF: MoveAllObjects+1Fw
;18183: 		or	(hl)
;18184: 		ld	(hl), a
;18185: 		ld	a, c
;18186: 		inc	h
;18187: 
;18188: loc_B1AA:
;18189: 		djnz	loc_B198
;18190: 		ld	b, 8
;18191: 		ld	a, l
;18192: 		add	a, 20h ; ' '
;18193: 		ld	l, a
;18194: 		ld	a, c
;18195: 		jr	c, loc_B198
;18196: 		ld	a, h
;18197: 		sub	b
;18198: 		ld	h, a
;18199: 		ld	a, c
;18200: 		jp	loc_B198
;18201: 
;18202: ; =============== S U B	R O U T	I N E =======================================
;18203: 
;18204: 
;18205: sub_B1BC:				; CODE XREF: RAM:loc_B1F4p
;18206: 					; RAM:loc_B238p
;18207: 		exx
;18208: 		ld	hl, 0
;18209: 		ld	e, l
;18210: 		rra
;18211: 		rr	e
;18212: 		ld	d, a
;18213: 		ex	af, af'
;18214: 
;18215: loc_B1C6:				; CODE XREF: sub_B1BC+17j
;18216: 		srl	d
;18217: 		rr	e
;18218: 		sub	b
;18219: 		jr	c, loc_B1D0
;18220: 		add	hl, de
;18221: 		ret	z
;18222: ; ---------------------------------------------------------------------------
;18223: 		db 0FEh
;18224: ; ---------------------------------------------------------------------------
;18225: 
;18226: loc_B1D0:				; CODE XREF: sub_B1BC+Fj
;18227: 		add	a, b
;18228: 		srl	b
;18229: 		jp	nz, loc_B1C6
;18230: 		ret
;18231: ; End of function sub_B1BC
;18232: 
;18233: ; ---------------------------------------------------------------------------
;18234: 
;18235: loc_B1D7:				; CODE XREF: PrnLineInBuf_clip+28p
;18236: 					; PrnLineInBuf_clip+49p
;18237: 		inc	a
;18238: 		ex	af, af'
;18239: 		ld	a, e
;18240: 		ld	e, 3Eh ; '>'
;18241: 		bit	2, c
;18242: 		jr	z, loc_B1E1+1
;18243: 		sub	l
;18244: 
;18245: loc_B1E1:				; CODE XREF: RAM:B1DEj
;18246: 		jr	c, loc_B168
;18247: 		exx
;18248: 		rra
;18249: 		ld	b, a
;18250: 		exx
;18251: 		ld	a, d
;18252: 		bit	3, c
;18253: 		jr	z, loc_B1F2+1
;18254: 		sub	h
;18255: 		push	af
;18256: 		jr	nc, loc_B1F4
;18257: 		ld	a, h
;18258: 		sub	d
;18259: 
;18260: loc_B1F2:				; CODE XREF: RAM:B1EAj
;18261: 		jr	z, loc_B177+1
;18262: 
;18263: loc_B1F4:				; CODE XREF: RAM:B1EEj
;18264: 		call	sub_B1BC
;18265: 		add	hl, hl
;18266: 		ld	a, h
;18267: 		ex	af, af'
;18268: 		exx
;18269: 		bit	3, c
;18270: 		jr	z, loc_B205
;18271: 		pop	af
;18272: 		jr	nc, loc_B205
;18273: 		ex	af, af'
;18274: 		add	a, d
;18275: 		ret
;18276: ; ---------------------------------------------------------------------------
;18277: 
;18278: loc_B205:				; CODE XREF: RAM:B1FDj	RAM:B200j
;18279: 		ld	a, b
;18280: 		ld	b, d
;18281: 
;18282: loc_B207:				; CODE XREF: RAM:B24Bj
;18283: 		ld	(loc_B210+1), a
;18284: 		ex	af, af'
;18285: 		jr	nc, loc_B215
;18286: 		add	a, b
;18287: 
;18288: loc_B20E:				; CODE XREF: RAM:B216j
;18289: 		ex	af, af'
;18290: 		ld	a, c
;18291: 
;18292: loc_B210:				; DATA XREF: RAM:loc_B207w
;18293: 		xor	28h ; '('
;18294: 		ld	c, a
;18295: 		ex	af, af'
;18296: 		ret
;18297: ; ---------------------------------------------------------------------------
;18298: 
;18299: loc_B215:				; CODE XREF: RAM:B20Bj
;18300: 		sub	b
;18301: 		jr	nc, loc_B20E
;18302: 		neg
;18303: 		ret
;18304: ; ---------------------------------------------------------------------------
;18305: 
;18306: loc_B21B:				; CODE XREF: PrnLineInBuf_clip+18p
;18307: 					; PrnLineInBuf_clip+37p
;18308: 		inc	a
;18309: 		ex	af, af'
;18310: 		ld	a, d
;18311: 		ld	d, 77h ; 'w'
;18312: 		bit	3, c
;18313: 		jr	z, loc_B226
;18314: 		sub	h
;18315: ; ---------------------------------------------------------------------------
;18316: 		db 38h
;18317: ; ---------------------------------------------------------------------------
;18318: 
;18319: loc_B226:				; CODE XREF: RAM:B222j
;18320: 		add	a, h
;18321: 		exx
;18322: 		rra
;18323: 		ld	b, a
;18324: 		exx
;18325: 		ld	a, e
;18326: 		bit	2, c
;18327: 		jr	z, loc_B237
;18328: 		sub	l
;18329: 		push	af
;18330: 		jr	nc, loc_B238
;18331: 		ld	a, l
;18332: 		sub	e
;18333: ; ---------------------------------------------------------------------------
;18334: 		db 28h
;18335: ; ---------------------------------------------------------------------------
;18336: 
;18337: loc_B237:				; CODE XREF: RAM:B22Ej
;18338: 		add	a, l
;18339: 
;18340: loc_B238:				; CODE XREF: RAM:B232j
;18341: 		call	sub_B1BC
;18342: 		add	hl, hl
;18343: 		ld	a, h
;18344: 		ex	af, af'
;18345: 		exx
;18346: 		bit	2, c
;18347: 		jr	z, loc_B249
;18348: 		pop	af
;18349: 		jr	nc, loc_B249
;18350: 		ex	af, af'
;18351: 		add	a, e
;18352: 		ret
;18353: ; ---------------------------------------------------------------------------
;18354: 
;18355: loc_B249:				; CODE XREF: RAM:B241j	RAM:B244j
;18356: 		ld	a, b
;18357: 		ld	b, e
;18358: 		jp	loc_B207
;18359: 
;18360: ; =============== S U B	R O U T	I N E =======================================
;18361: 
;18362: ; вызывается при увеличении 3го	байта рейтинга
;18363: 
;18364: sub_B24E:				; CODE XREF: IncBattleRating2+Cp
;18365: 		ld	a, (hl)
;18366: 		cp	0C0h ; '└'      ; hl,PilotRating2
;18367: 		jr	nc, loc_B256
;18368: 		inc	(hl)
;18369: 		jr	loc_B258
;18370: ; ---------------------------------------------------------------------------
;18371: 
;18372: loc_B256:				; CODE XREF: sub_B24E+3j
;18373: 		res	0, (hl)
;18374: 
;18375: loc_B258:				; CODE XREF: sub_B24E+6j
;18376: 		call	sub_5E78	; вызывается при увеличении 3го	байта рейтинга
;18377: 		ret
;18378: ; End of function sub_B24E
;18379: 
;18380: 
;18381: ; =============== S U B	R O U T	I N E =======================================
;18382: 
;18383: 
;18384: sub_B25C:				; CODE XREF: sub_D14Dp
;18385: 		ld	a, (hl)
;18386: 		cp	0C0h ; '└'
;18387: 		jr	nc, loc_B263
;18388: 		inc	(hl)
;18389: 		ret
;18390: ; ---------------------------------------------------------------------------
;18391: 
;18392: loc_B263:				; CODE XREF: sub_B25C+3j
;18393: 		res	0, (hl)
;18394: 		ret
;18395: ; End of function sub_B25C
;18396: 
;18397: ; ---------------------------------------------------------------------------
;18398: 		db 0F3h	; є
;18399: 		db 0F3h	; є
;18400: 		db 0F3h	; є
;18401: 		db 0F3h	; є
;18402: 		db 0F3h	; є
;18403: 		db 0F3h	; є
;18404: 		db 0F3h	; є
;18405: 		db 0F3h	; є
;18406: 		db 0F3h	; є
;18407: 		db 0F3h	; є
;18408: 		db 0F3h	; є
;18409: 		db 0F3h	; є
;18410: 		db 0F3h	; є
;18411: 		db 0F3h	; є
;18412: 		db 0F3h	; є
;18413: 		db 0F3h	; є
;18414: 		db 0F3h	; є
;18415: 		db 0F3h	; є
;18416: 		db 0F3h	; є
;18417: 		db 0F3h	; є
;18418: 		db 0F3h	; є
;18419: 		db 0F3h	; є
;18420: 		db 0F3h	; є
;18421: 		db 0F3h	; є
;18422: 		db 0F3h	; є
;18423: 		db 0F3h	; є
;18424: 		db 0F3h	; є
;18425: 		db 0F3h	; є
;18426: 		db 0F3h	; є
;18427: 		db 0F3h	; є
;18428: 		db 0F3h	; є
;18429: 		db  76h	; v
;18430: ; ---------------------------------------------------------------------------
;18431: ; START	OF FUNCTION CHUNK FOR sub_9924
;18432: 
;18433: loc_B286:				; CODE XREF: sub_9924+4j
;18434: 		push	de
;18435: 		ld	de, 0
;18436: 		jr	loc_B28F
;18437: ; ---------------------------------------------------------------------------
;18438: 
;18439: loc_B28C:				; CODE XREF: sub_9924+1974j
;18440: 		and	a
;18441: 		jr	z, loc_B29C
;18442: 
;18443: loc_B28F:				; CODE XREF: sub_9924+1966j
;18444: 		rra
;18445: 		jr	nc, loc_B297
;18446: 		ex	de, hl
;18447: 		add	hl, de
;18448: 		jr	c, loc_B29A
;18449: 		ex	de, hl
;18450: 
;18451: loc_B297:				; CODE XREF: sub_9924+196Cj
;18452: 		add	hl, hl
;18453: 		jr	loc_B28C
;18454: ; ---------------------------------------------------------------------------
;18455: 
;18456: loc_B29A:				; CODE XREF: sub_9924+1970j
;18457: 		sbc	a, a
;18458: ; ---------------------------------------------------------------------------
;18459: 		db 0FEh
;18460: ; ---------------------------------------------------------------------------
;18461: 
;18462: loc_B29C:				; CODE XREF: sub_9924+1969j
;18463: 		ld	a, d
;18464: 		pop	de
;18465: 		ret
;18466: ; END OF FUNCTION CHUNK	FOR sub_9924
;18467: ; ---------------------------------------------------------------------------
;18468: 		db    0
;18469: 
;18470: ; =============== S U B	R O U T	I N E =======================================
;18471: 
;18472: 
;18473: sub_B2A0:				; CODE XREF: sub_7FCD+6p
;18474: 		ld	a, 4
;18475: 
;18476: loc_B2A2:				; CODE XREF: sub_B2A0+49j
;18477: 		ldi
;18478: 		ldi
;18479: 		ldi
;18480: 		ldi
;18481: 		ldi
;18482: 		ldi
;18483: 		ldi
;18484: 		ldi
;18485: 		ldi
;18486: 		ldi
;18487: 		ldi
;18488: 		ldi
;18489: 		ldi
;18490: 		ldi
;18491: 		ldi
;18492: 		ldi
;18493: 		ldi
;18494: 		ldi
;18495: 		ldi
;18496: 		ldi
;18497: 		ldi
;18498: 		ldi
;18499: 		ldi
;18500: 		ldi
