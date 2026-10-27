;15650: 		ld	iy, deltaCrdX
;15651: 		push	ix
;15652: 		ld	ix, deltaCrdX
;15653: 		call	CalcLenVector
;15654: ; ---------------------------------------------------------------------------
;15655: 		pop	ix
;15656: 		ld	(LenVector), de
;15657: 		push	ix
;15658: 		ld	de, 9
;15659: 		add	ix, de
;15660: 		call	ScalarMultVectors
;15661: 		ld	de, 6
;15662: 		add	ix, de
;15663: 		ld	h, a
;15664: 		push	hl
;15665: 		call	ScalarMultVectors
;15666: 		pop	hl
;15667: 		ld	l, a
;15668: 		push	hl
;15669: 		ld	de, 6
;15670: 		add	ix, de
;15671: 		call	ScalarMultVectors
;15672: 		pop	hl
;15673: 		pop	ix
;15674: 		pop	iy
;15675: 		ld	d, a
;15676: 		and	7Fh ; ''
;15677: 		jr	z, loc_A5E7
;15678: 		ld	c, a
;15679: 		srl	c
;15680: 		srl	c
;15681: 		srl	c
;15682: 		adc	a, c
;15683: 		srl	a
;15684: 		srl	a
;15685: 		cp	0Fh
;15686: 		jr	c, loc_A5D7
;15687: 		ld	a, 0Fh
;15688: 
;15689: loc_A5D7:				; CODE XREF: MoveObject+4Cj
;15690: 		ld	b, a
;15691: 		ld	a, d
;15692: 		xor	l
;15693: 		and	80h ; 'А'
;15694: 		rra
;15695: 		or	b
;15696: 		ld	(ix+1Dh), a
;15697: 		and	3Fh ; '?'
;15698: 		cp	4
;15699: 		jr	nc, loc_A605
;15700: 
;15701: loc_A5E7:				; CODE XREF: MoveObject+3Cj
;15702: 		ld	a, h
;15703: 		bit	7, a
;15704: 		jr	z, loc_A5FB
;15705: 		and	7Fh ; ''
;15706: 		add	a, 50h ; 'P'
;15707: 		srl	a
;15708: 		cpl
;15709: 		add	a, 68h ; 'h'
;15710: 		srl	a
;15711: 		cp	40h ; '@'
;15712: 		jr	c, loc_A5FD
;15713: 
;15714: loc_A5FB:				; CODE XREF: MoveObject+63j
;15715: 		ld	a, 3Fh ; '?'
;15716: 
;15717: loc_A5FD:				; CODE XREF: MoveObject+72j
;15718: 		sla	l
;15719: 		rra
;15720: 		srl	a
;15721: 		ld	(ix+1Eh), a
;15722: 
;15723: loc_A605:				; CODE XREF: MoveObject+5Ej
;15724: 		xor	a
;15725: 		bit	7, (ix+25h)
;15726: 		jr	z, loc_A60E
;15727: 		ld	a, 2
;15728: 
;15729: loc_A60E:				; CODE XREF: MoveObject+83j
;15730: 		ld	(ix+1Ch), a
;15731: 		ret	nz
;15732: 		ld	de, (LenVector)
;15733: 		ld	a, d
;15734: 		sla	h
;15735: 		jr	nc, loc_A686
;15736: 		ld	b, 0Ah
;15737: 		bit	0, (ix+26h)
;15738: 		jr	z, loc_A625
;15739: 		ld	b, 2
;15740: 
;15741: loc_A625:				; CODE XREF: MoveObject+9Aj
;15742: 		cp	b
;15743: 		jr	c, loc_A632
;15744: 		ld	b, 1
;15745: 		cp	10h
;15746: 		jr	nc, loc_A62F
;15747: 		inc	b
;15748: 
;15749: loc_A62F:				; CODE XREF: MoveObject+A5j
;15750: 		ld	(ix+1Ch), b
;15751: 
;15752: loc_A632:				; CODE XREF: MoveObject+9Fj
;15753: 		bit	2, (ix+21h)
;15754: 		ret	z
;15755: 		ld	a, (ix+8)
;15756: 		xor	(ix+0Eh)
;15757: 		and	80h ; 'А'
;15758: 		ret	z
;15759: 		ld	b, (iy+2)
;15760: 		ld	a, (PilotRating2) ; >=#84 в трюме десант
;15761: 					; 7,=1 война была или идёт
;15762: 					; 6,=1 война закончилать
;15763: 					; 3-1 -	ранг (=текущая стадия войны)
;15764: 		and	a
;15765: 		jr	nz, loc_A64B
;15766: 		srl	b
;15767: 
;15768: loc_A64B:				; CODE XREF: MoveObject+C0j
;15769: 		call	IntRnd_A
;15770: 		cp	b
;15771: 		ret	nc
;15772: 		ld	a, (ix+26h)
;15773: 		and	0Fh
;15774: 		ret	nz
;15775: 		ld	a, d
;15776: 		cp	0Ch
;15777: 		ret	nc
;15778: 		sla	h
;15779: 		ret	nc
;15780: 		call	sub_FA84
;15781: 		ret	nc
;15782: 		ld	a, d
;15783: 		cp	4
;15784: 		jr	c, loc_A669
;15785: 		sla	h
;15786: 		ret	nc
;15787: 
;15788: loc_A669:				; CODE XREF: MoveObject+DDj
;15789: 		ld	hl, nBeepFX
;15790: 		set	6, (hl)
;15791: 		bit	7, (ix+8)
;15792: 		jr	z, loc_A67D
;15793: 		ld	a, (DamageRearShield)
;15794: 		add	a, 7
;15795: 		ld	(DamageRearShield), a
;15796: 		ret
;15797: ; ---------------------------------------------------------------------------
;15798: 
;15799: loc_A67D:				; CODE XREF: MoveObject+EBj
;15800: 		ld	a, (DamageFrontShield)
;15801: 		add	a, 7
;15802: 		ld	(DamageFrontShield), a
;15803: 		ret
;15804: ; ---------------------------------------------------------------------------
;15805: 
;15806: loc_A686:				; CODE XREF: MoveObject+92j
;15807: 		ld	b, 0Bh
;15808: 		bit	0, (ix+26h)
;15809: 		jr	z, loc_A690
;15810: 		ld	b, 3
;15811: 
;15812: loc_A690:				; CODE XREF: MoveObject+105j
;15813: 		cp	b
;15814: 		ret	c
;15815: 		ld	b, 0FFh
;15816: 		cp	12h
;15817: 		jr	c, loc_A699
;15818: 		dec	b
;15819: 
;15820: loc_A699:				; CODE XREF: MoveObject+10Fj
;15821: 		ld	(ix+1Ch), b
;15822: 		ret
;15823: ; End of function MoveObject
;15824: 
;15825: 
;15826: ; =============== S U B	R O U T	I N E =======================================
;15827: 
;15828: ; Умножение hl = -1 * high((a*2) * (b*2))
;15829: ; Вх:  b - первый множитель, 7,b - знак
;15830: ;      a - второй множитель, 7,a - знак
;15831: ; Вых: l,a = high((a*2)	* (b*2))
;15832: ;	l,a = high((a*2) * (b*2))
;15833: ;	7,h - знак
;15834: 
;15835: Mult_HL_A2_B2:				; CODE XREF: ScalarMultVectors+6p
;15836: 					; ScalarMultVectors+10p ...
;15837: 		ld	c, a
;15838: 		xor	b
;15839: 		xor	80h ; 'А'
;15840: 		sla	b
;15841: 		sla	c
;15842: 		push	af
;15843: 		call	Mult_A_hiB_C
;15844: 		pop	hl
;15845: 		ld	l, a
;15846: 		ret
;15847: ; End of function Mult_HL_A2_B2
;15848: 
;15849: 
;15850: ; =============== S U B	R O U T	I N E =======================================
;15851: 
;15852: 
;15853: ScalarMultVectors:			; CODE XREF: MoveObject+1Cp
;15854: 					; MoveObject+26p ...
;15855: 		ld	b, (ix+1)
;15856: 		ld	a, (iy+1)
;15857: 		call	Mult_HL_A2_B2	; Умножение hl = -1 * high((a*2) * (b*2))
;15858: 					; Вх:  b - первый множитель, 7,b - знак
;15859: 					;      a - второй множитель, 7,a - знак
;15860: 					; Вых: l,a = high((a*2)	* (b*2))
;15861: 					;	l,a = high((a*2) * (b*2))
;15862: 					;	7,h - знак
;15863: 		push	hl
;15864: 		ld	b, (ix+3)
;15865: 		ld	a, (iy+3)
;15866: 		call	Mult_HL_A2_B2	; Умножение hl = -1 * high((a*2) * (b*2))
;15867: 					; Вх:  b - первый множитель, 7,b - знак
;15868: 					;      a - второй множитель, 7,a - знак
;15869: 					; Вых: l,a = high((a*2)	* (b*2))
;15870: 					;	l,a = high((a*2) * (b*2))
;15871: 					;	7,h - знак
;15872: 		push	hl
;15873: 		ld	b, (ix+5)
;15874: 		ld	a, (iy+5)
;15875: 		call	Mult_HL_A2_B2	; Умножение hl = -1 * high((a*2) * (b*2))
;15876: ; End of function ScalarMultVectors	; Вх:  b - первый множитель, 7,b - знак
;15877: 					;      a - второй множитель, 7,a - знак
;15878: 					; Вых: l,a = high((a*2)	* (b*2))
;15879: 					;	l,a = high((a*2) * (b*2))
;15880: 					;	7,h - знак
;15881: 
;15882: ; =============== S U B	R O U T	I N E =======================================
;15883: 
;15884: 
;15885: sub_A6C9:
;15886: 		pop	de
;15887: 		call	AddSign_L_E
;15888: 		jr	nc, loc_A6D1
;15889: 		ld	a, 0FFh
;15890: 
;15891: loc_A6D1:				; CODE XREF: sub_A6C9+4j
;15892: 		pop	de
;15893: 		ld	l, a
;15894: 		call	AddSign_L_E
;15895: 		jr	nc, loc_A6DA
;15896: 		ld	a, 0FFh
;15897: 
;15898: loc_A6DA:				; CODE XREF: sub_A6C9+Dj
;15899: 		rl	h
;15900: 		rra
;15901: 		ret
;15902: ; End of function sub_A6C9
;15903: 
;15904: ; ---------------------------------------------------------------------------
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
