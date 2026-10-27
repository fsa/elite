;16540: ; FUNCTION CHUNK AT AD82 SIZE 0000000E BYTES
;16541: ; FUNCTION CHUNK AT ADF2 SIZE 0000000F BYTES
;16542: 
;16543: 		push	iy
;16544: 		push	iy
;16545: 		ld	a, (iy+3)	; a - кол-во вершин
;16546: 		ex	(sp), ix
;16547: 		ld	d, (iy+0Dh)
;16548: 		call	sub_AE01	; de= кол-во вершин * 3
;16549: 		add	ix, de		; ix = адрес начала вершин + кол-во вершин * 3
;16550: 		ld	iy, MatrixRotate
;16551: 
;16552: loc_A9DE:				; CODE XREF: MakeTablPrnCrdVertex+430j
;16553: 		push	af
;16554: 		ld	a, (PointTablCrdPrn)
;16555: 		add	a, 3
;16556: 		ld	(PointTablCrdPrn), a
;16557: 		call	sub_AA2B
;16558: 		and	1Fh
;16559: 		cp	1Fh
;16560: 		jr	z, loc_A9F7
;16561: 		ld	b, a
;16562: 		ld	a, (DistanceToObj)
;16563: 		cp	b
;16564: 		jr	nc, loc_AA03
;16565: 
;16566: loc_A9F7:				; CODE XREF: MakeTablPrnCrdVertex+25j
;16567: 		inc	hl
;16568: 		ld	a, (hl)
;16569: 		call	TestVergesVisible
;16570: 		jr	nz, loc_AA03
;16571: 		inc	hl
;16572: 		ld	a, (hl)
;16573: 		call	TestVergesVisible
;16574: 
;16575: loc_AA03:				; CODE XREF: MakeTablPrnCrdVertex+2Cj
;16576: 					; MakeTablPrnCrdVertex+33j
;16577: 		jp	nz, CalcPrnCrdVertex
;16578: 		inc	a
;16579: 		ld	(VisiblePrevVertex), a
;16580: 		pop	af
;16581: 		jp	loc_ADF2
;16582: ; End of function MakeTablPrnCrdVertex
;16583: 
;16584: ; ---------------------------------------------------------------------------
;16585: 		db 3Fh
;16586: CrdNVertex:	db 15h			; DATA XREF: PrintObject_2:loc_6F80w
;16587: 					; PrintObject_2+5Fo ...
;16588: 					; координата N текущей вершины
;16589: 
;16590: ; =============== S U B	R O U T	I N E =======================================
;16591: 
;16592: 
;16593: TestVergesVisible:			; CODE XREF: PrintObject_2+5Ap
;16594: 					; MakeTablPrnCrdVertex+30p ...
;16595: 		ld	b, a
;16596: 		call	loc_AA20
;16597: 		push	bc
;16598: 		ld	b, a
;16599: 		pop	af
;16600: 		rrca
;16601: 		rrca
;16602: 		rrca
;16603: 		rrca
;16604: 		call	loc_AA20
;16605: 
;16606: loc_AA1E:				; DATA XREF: PrintObject_2+17w
;16607: 					; PrintObject_2+A6w
;16608: 		or	b
;16609: 		ret
;16610: ; ---------------------------------------------------------------------------
;16611: 
;16612: loc_AA20:				; CODE XREF: TestVergesVisible+1p
;16613: 					; TestVergesVisible+Bp
;16614: 		and	0Fh
;16615: 		add	a, 84h ; 'Д'
;16616: 		ld	(loc_AA27+1), a
;16617: 
;16618: loc_AA27:				; DATA XREF: TestVergesVisible+14w
;16619: 					; RAM:D79Ew ...
;16620: 		ld	a, (byte_A98E)
;16621: 		ret
;16622: ; End of function TestVergesVisible
;16623: 
;16624: 
;16625: ; =============== S U B	R O U T	I N E =======================================
;16626: 
;16627: 
;16628: sub_AA2B:				; CODE XREF: MakeTablPrnCrdVertex+1Ep
;16629: 		ld	hl, CrdNVertex	; координата N текущей вершины
;16630: 		ld	a, (hl)
;16631: 		dec	hl
;16632: 		ld	(hl), a
;16633: ; End of function sub_AA2B
;16634: 
;16635: 
;16636: ; =============== S U B	R O U T	I N E =======================================
;16637: 
;16638: 
;16639: sub_AA31:				; CODE XREF: CalcVisibleVerges+2Dp
;16640: 		ld	hl, (TempAdr)	; адрес	текущей	вершины
;16641: 		ld	a, (hl)
;16642: 		ld	(CrdNVertex), a	; координата N текущей вершины
;16643: 		or	1
;16644: 		ret
;16645: ; End of function sub_AA31
;16646: 
;16647: ; ---------------------------------------------------------------------------
;16648: 		db  9Ch	; Ь
;16649: 		db 0B8h	; ╕
;16650: 
;16651: ; =============== S U B	R O U T	I N E =======================================
;16652: 
;16653: 
;16654: RotateCrdN_Vertex:			; CODE XREF: MakeTablPrnCrdVertex+2FBp
;16655: 					; MakeTablPrnCrdVertex+304p ...
;16656: 		ld	a, (VisiblePrevVertex)
;16657: 		and	a
;16658: 		jr	nz, loc_AA8B
;16659: 		ld	a, (CrdNVertex)	; координата N текущей вершины
;16660: 		rrca
;16661: 		jr	c, loc_AA8B
;16662: 		ld	hl, CrdNVertex	; координата N текущей вершины
;16663: 		ld	a, (hl)
;16664: 		dec	hl
;16665: 		xor	(hl)
;16666: 		ld	b, a
;16667: 		ld	l, (iy+12h)
;16668: 		ld	h, (iy+13h)
;16669: 		ld	e, (iy+14h)
;16670: 		ld	d, (iy+15h)
;16671: 		xor	h
;16672: 		and	80h ; 'А'
;16673: 		ld	h, a
;16674: 		ld	(iy+13h), h
;16675: 		rl	b
;16676: 		ld	a, b
;16677: 		xor	d
;16678: 		and	80h ; 'А'
;16679: 		ld	d, a
;16680: 		ld	(iy+15h), d
;16681: 		call	AddSign_L_E
;16682: 		ld	l, a
;16683: 		ld	e, (iy+16h)
;16684: 		ld	d, (iy+17h)
;16685: 		ld	a, b
;16686: 		rla
;16687: 		xor	d
;16688: 		and	80h ; 'А'
;16689: 		ld	d, a
;16690: 		ld	(iy+17h), d
;16691: 		call	AddSign_L_E
;16692: 		ld	l, a
;16693: 		ld	de, 6
;16694: 		add	iy, de
;16695: 		jr	loc_AAE3
;16696: ; ---------------------------------------------------------------------------
;16697: 
;16698: loc_AA8B:				; CODE XREF: RotateCrdN_Vertex+4j
;16699: 					; RotateCrdN_Vertex+Aj
;16700: 		ld	b, (ix+0)
;16701: 		call	sub_AB1E
;16702: 		ld	a, (CrdNVertex)	; координата N текущей вершины
;16703: 		xor	(iy+1)
;16704: 		rla
;16705: 		rr	h
;16706: 		ld	(iy+12h), l
;16707: 		ld	(iy+13h), h
;16708: 		ex	de, hl
;16709: 		inc	iy
;16710: 		inc	iy
;16711: 		ld	b, (ix+1)
;16712: 		call	sub_AB1E
;16713: 		ld	a, (CrdNVertex)	; координата N текущей вершины
;16714: 		rla
;16715: 		xor	(iy+1)
;16716: 		rla
;16717: 		rr	h
;16718: 		ld	(iy+12h), l
;16719: 		ld	(iy+13h), h
;16720: 		call	AddSign_L_E
;16721: 		ld	l, a
;16722: 		ex	de, hl
;16723: 		inc	iy
;16724: 		inc	iy
;16725: 		ld	b, (ix+2)
;16726: 		call	sub_AB1E
;16727: 		ld	a, (CrdNVertex)	; координата N текущей вершины
;16728: 		rla
;16729: 		rla
;16730: 		xor	(iy+1)
;16731: 		rla
;16732: 		rr	h
;16733: 		ld	(iy+12h), l
;16734: 		ld	(iy+13h), h
;16735: 		call	AddSign_L_E
;16736: 		ld	l, a
;16737: 		inc	iy
;16738: 		inc	iy
;16739: 
;16740: loc_AAE3:				; CODE XREF: RotateCrdN_Vertex+4Cj
;16741: 		ld	a, (VertexIncFactor)
;16742: 		ld	b, a
;16743: 		xor	a
;16744: 		ld	c, h
;16745: 		res	7, h
;16746: 
;16747: loc_AAEB:				; CODE XREF: RotateCrdN_Vertex+B3j
;16748: 		srl	h
;16749: 		rr	l
;16750: 		rra
;16751: 		djnz	loc_AAEB
;16752: 		jr	nc, loc_AAF5
;16753: 		inc	hl
;16754: 
;16755: loc_AAF5:				; CODE XREF: RotateCrdN_Vertex+B5j
;16756: 		bit	7, c
;16757: 		ret	z
;16758: 		set	7, h
;16759: 		ret
;16760: ; End of function RotateCrdN_Vertex
;16761: 
;16762: 
;16763: ; =============== S U B	R O U T	I N E =======================================
;16764: 
;16765: 
;16766: AddSign_L_E:				; CODE XREF: sub_A6C9+1p sub_A6C9+Ap ...
;16767: 		ld	a, h
;16768: 		xor	d
;16769: 		rla
;16770: 		jr	c, loc_AB03
;16771: 		ld	a, l
;16772: 		add	a, e
;16773: 		ret
;16774: ; ---------------------------------------------------------------------------
;16775: 
;16776: loc_AB03:				; CODE XREF: AddSign_L_E+3j
;16777: 		ld	a, l
;16778: 		sub	e
;16779: 		ret	nc
;16780: 		neg
;16781: 		and	a
;16782: 		ld	h, d
;16783: 		ret
;16784: ; End of function AddSign_L_E
;16785: 
;16786: 
;16787: ; =============== S U B	R O U T	I N E =======================================
;16788: 
;16789: 
;16790: Mult_A_hiB_C:				; CODE XREF: sub_A0FD+7p
;16791: 					; CalcNewCrdNobj+32p ...
;16792: 		ld	a, b
;16793: 		sub	c
;16794: 		jr	nc, loc_AB11
;16795: 		neg
;16796: 
;16797: loc_AB11:				; CODE XREF: Mult_A_hiB_C+2j
;16798: 		srl	a
;16799: 		ld	h, 0BFh	; '┐'
;16800: 		ld	l, a
;16801: 		ld	d, (hl)
;16802: 		ld	a, b
;16803: 		add	a, c
;16804: 		rra
;16805: 		ld	l, a
;16806: 		ld	a, (hl)
;16807: 		sub	d
;16808: 		ret
;16809: ; End of function Mult_A_hiB_C
;16810: 
;16811: 
;16812: ; =============== S U B	R O U T	I N E =======================================
;16813: 
;16814: 
;16815: sub_AB1E:				; CODE XREF: RotateCrdN_Vertex+51p
;16816: 					; RotateCrdN_Vertex+6Bp ...
;16817: 		ld	a, (VertexIncFactor)
;16818: 
;16819: loc_AB21:				; CODE XREF: sub_AB1E+6j
;16820: 		sla	b
;16821: 		dec	a
;16822: 		jr	nz, loc_AB21
;16823: 		ld	c, (iy+1)
;16824: 		res	7, c
;16825: 		push	de
;16826: 		call	Mult_A_hiB_C
;16827: 		ld	l, a
;16828: 		ld	h, 0
;16829: 		pop	de
;16830: 		ret
;16831: ; End of function sub_AB1E
;16832: 
;16833: 
;16834: ; =============== S U B	R O U T	I N E =======================================
;16835: 
;16836: 
;16837: Mult_HL_B_C:				; CODE XREF: Sqr_HL_wordAdrIX+Ep
;16838: 					; sub_9F68+5p ...
;16839: 		ld	a, b
;16840: 		sub	c
;16841: 		jr	nc, loc_AB3A
;16842: 		neg
;16843: 
;16844: loc_AB3A:				; CODE XREF: Mult_HL_B_C+2j
;16845: 		srl	a
;16846: 		ld	h, 0BEh	; '╛'
;16847: 		ld	l, a
;16848: 		ld	e, (hl)
;16849: 		inc	h
;16850: 		ld	d, (hl)
;16851: 		dec	h
;16852: 		ld	a, b
;16853: 		add	a, c
;16854: 		rra
;16855: 		ld	l, a
;16856: 		ld	a, (hl)
;16857: 		inc	h
;16858: 		ld	h, (hl)
;16859: 		ld	l, a
;16860: 		and	a
;16861: 		sbc	hl, de
;16862: 		ret
;16863: ; End of function Mult_HL_B_C
;16864: 
;16865: 
;16866: ; =============== S U B	R O U T	I N E =======================================
;16867: 
;16868: 
;16869: AddCrdNobj_CrdNvertex:			; CODE XREF: MakeTablPrnCrdVertex+332p
;16870: 					; MakeTablPrnCrdVertex+351p ...
;16871: 		ld	c, d
;16872: 		rl	e
;16873: 		rl	d
;16874: 		ld	a, b
;16875: 		xor	c
;16876: 		jr	nz, loc_AB5B
;16877: 		add	hl, de
;16878: 		ld	a, b
;16879: 		ret
;16880: ; ---------------------------------------------------------------------------
;16881: 
;16882: loc_AB5B:				; CODE XREF: AddCrdNobj_CrdNvertex+7j
;16883: 		sbc	hl, de
;16884: 		jr	nc, loc_AB66
;16885: 		add	hl, de
;16886: 		ex	de, hl
;16887: 		and	a
;16888: 		sbc	hl, de
;16889: 		ld	a, c
;16890: 		ret
;16891: ; ---------------------------------------------------------------------------
;16892: 
;16893: loc_AB66:				; CODE XREF: AddCrdNobj_CrdNvertex+Ej
;16894: 		ld	a, h
;16895: 		or	l
;16896: 		ld	a, c
;16897: 		ret	z
;16898: 		pop	de
;16899: 		pop	af
;16900: 		neg
;16901: 		push	af
;16902: 		push	de
;16903: 		dec	hl
;16904: 		ld	a, b
;16905: 		ret
;16906: ; End of function AddCrdNobj_CrdNvertex
;16907: 
;16908: ; ---------------------------------------------------------------------------
;16909: ; START	OF FUNCTION CHUNK FOR MakeTablPrnCrdVertex
;16910: 
;16911: RotateCrdVertex:			; CODE XREF: MakeTablPrnCrdVertex+2E0j
;16912: 					; MakeTablPrnCrdVertex+2F8j
;16913: 		ld	a, (VisiblePrevVertex)
;16914: 		and	a
;16915: 		jr	nz, loc_AB90
;16916: 		ld	a, (CrdNVertex)	; координата N текущей вершины
;16917: 		rrca
;16918: 		jr	c, loc_AB90
;16919: 		call	sub_ABC0
;16920: 		ld	(VertexCrdX), hl
;16921: 		call	sub_ABC0
;16922: 		ld	(VertexCrdY), hl
;16923: 		call	sub_ABC0
;16924: 		jr	loc_AB9F
;16925: ; ---------------------------------------------------------------------------
;16926: 
;16927: loc_AB90:				; CODE XREF: MakeTablPrnCrdVertex+1AEj
;16928: 					; MakeTablPrnCrdVertex+1B4j
;16929: 		call	RotateCrdN
;16930: 		ld	(VertexCrdX), hl
;16931: 		call	RotateCrdN
;16932: 		ld	(VertexCrdY), hl
;16933: 		call	RotateCrdN
;16934: 
;16935: loc_AB9F:				; CODE XREF: MakeTablPrnCrdVertex+1C5j
;16936: 		ld	(VertexCrdZ), hl
;16937: 		xor	a
;16938: 		ld	(VertexCrdXlow), a
;16939: 		ld	(VertexCrdYlow), a
;16940: 		ld	(VertexCrdZlow), a
;16941: 		jp	loc_ACDF
;16942: ; END OF FUNCTION CHUNK	FOR MakeTablPrnCrdVertex
;16943: 
;16944: ; =============== S U B	R O U T	I N E =======================================
;16945: 
;16946: 
;16947: sub_ABAF:				; CODE XREF: GenerateProductPricep
;16948: 		ld	hl, RndWord
;16949: 		ld	a, (Pilot_CrdYSystem)
;16950: 		add	a, (hl)
;16951: 		ret
;16952: ; End of function sub_ABAF
;16953: 
;16954: ; ---------------------------------------------------------------------------
;16955: 		db 0C9h	; ╔
;16956: 		db  21h	; !
;16957: 		db    5
;16958: 		db  71h	; q
;16959: 		db  3Ah	; :
;16960: 		db  97h	; Ч
;16961: 		db 0D0h	; ╨
;16962: 		db  86h	; Ж
;16963: 		db 0C9h	; ╔
;16964: 
;16965: ; =============== S U B	R O U T	I N E =======================================
;16966: 
;16967: 
;16968: sub_ABC0:				; CODE XREF: MakeTablPrnCrdVertex+1B6p
;16969: 					; MakeTablPrnCrdVertex+1BCp ...
;16970: 		ld	hl, 0AA0Fh
;16971: 		ld	a, (hl)
;16972: 		dec	hl
;16973: 		xor	(hl)
;16974: 		ld	b, a
;16975: 		ld	l, (iy+12h)
;16976: 		ld	h, (iy+13h)
;16977: 		ld	e, (iy+14h)
;16978: 		ld	d, (iy+15h)
;16979: 		xor	h
;16980: 		and	80h ; 'А'
;16981: 		ld	h, a
;16982: 		ld	(iy+13h), h
;16983: 		rl	b
;16984: 		ld	a, b
;16985: 		xor	d
;16986: 		and	80h ; 'А'
;16987: 		ld	d, a
;16988: 		ld	(iy+15h), d
;16989: 		call	AddSign_L_E
;16990: 		ld	l, a
;16991: 		ld	e, (iy+16h)
;16992: 		ld	d, (iy+17h)
;16993: 		ld	a, b
;16994: 		rla
;16995: 		xor	d
;16996: 		and	80h ; 'А'
;16997: 		ld	d, a
;16998: 		ld	(iy+17h), d
;16999: 		call	AddSign_L_E
;17000: 		ld	l, a
;17001: 		ld	de, 6
;17002: 		add	iy, de
;17003: 		ret
;17004: ; End of function sub_ABC0
;17005: 
;17006: 
;17007: ; =============== S U B	R O U T	I N E =======================================
;17008: 
;17009: 
;17010: RotateCrdN:				; CODE XREF: MakeTablPrnCrdVertex:loc_AB90p
;17011: 					; MakeTablPrnCrdVertex+1CDp ...
;17012: 		ld	b, (ix+0)
;17013: 		ld	c, (iy+1)
;17014: 		res	7, c
;17015: 		call	Mult_A_hiB_C
;17016: 		ld	l, a
;17017: 		ld	a, (CrdNVertex)	; координата N текущей вершины
;17018: 		xor	(iy+1)
;17019: 		and	80h ; 'А'
;17020: 		ld	h, a
;17021: 		ld	(iy+12h), l
;17022: 		ld	(iy+13h), h
;17023: 		ex	de, hl
;17024: 
;17025: loc_AC1D:
;17026: 		ld	b, (ix+1)
;17027: 		ld	c, (iy+3)
;17028: 		res	7, c
;17029: 		push	de
;17030: 		call	Mult_A_hiB_C
;17031: 		pop	de
;17032: 		ld	l, a
;17033: 		ld	a, (CrdNVertex)	; координата N текущей вершины
;17034: 		rla
;17035: 		xor	(iy+3)
;17036: 		and	80h ; 'А'
;17037: 		ld	h, a
;17038: 		ld	(iy+14h), l
;17039: 		ld	(iy+15h), h
;17040: 		call	AddSign_L_E
;17041: 		ld	l, a
;17042: 		ex	de, hl
;17043: 		ld	b, (ix+2)
;17044: 		ld	c, (iy+5)
;17045: 		res	7, c
;17046: 		push	de
;17047: 		call	Mult_A_hiB_C
;17048: 		pop	de
;17049: 		ld	l, a
;17050: 		ld	a, (CrdNVertex)	; координата N текущей вершины
;17051: 		rla
;17052: 		rla
;17053: 		xor	(iy+5)
;17054: 		and	80h ; 'А'
;17055: 		ld	h, a
;17056: 		ld	(iy+16h), l
;17057: 		ld	(iy+17h), h
;17058: 		call	AddSign_L_E
;17059: 		ld	l, a
;17060: 		ld	de, 6
;17061: 		add	iy, de
;17062: 		ret
;17063: ; End of function RotateCrdN
;17064: 
;17065: 
;17066: ; =============== S U B	R O U T	I N E =======================================
;17067: 
;17068: ; Attributes: noreturn
;17069: 
;17070: Div_C_DEc_HLa:				; CODE XREF: PrnObjIfVisible:loc_A71Ep
;17071: 					; PrnObjIfVisible+4Fp ...
;17072: 		ld	b, 9
;17073: 
;17074: loc_AC6B:				; CODE XREF: Div_C_DEc_HLa+10j
;17075: 		dec	b
;17076: 		jr	z, loc_AC84
;17077: 		rla
;17078: 		adc	hl, hl
;17079: 		jr	c, loc_AC80
;17080: 		rl	c
;17081: 		rl	e
;17082: 		rl	d
;17083: 		jr	nc, loc_AC6B
;17084: 		rr	d
;17085: 		rr	e
;17086: 		scf
;17087: 
;17088: loc_AC80:				; CODE XREF: Div_C_DEc_HLa+8j
;17089: 		rr	h
;17090: 		rr	l
;17091: 
;17092: loc_AC84:				; CODE XREF: Div_C_DEc_HLa+3j
;17093: 		ld	a, d
;17094: 		ld	c, 0FFh
;17095: 		cp	h
;17096: 		ret	nc
;17097: 		ld	b, h
;17098: 		dec	c
;17099: 
;17100: loc_AC8B:				; CODE XREF: Div_C_DEc_HLa+2Dj
;17101: 					; Div_C_DEc_HLa+34j
;17102: 		sla	e
;17103: 		rla
;17104: 		jr	c, loc_AC98
;17105: 		cp	b
;17106: 		jr	nc, loc_AC98
;17107: 		sla	c
;17108: 		ret	nc
;17109: 		jr	loc_AC8B
;17110: ; ---------------------------------------------------------------------------
;17111: 
;17112: loc_AC98:				; CODE XREF: Div_C_DEc_HLa+25j
;17113: 					; Div_C_DEc_HLa+28j
;17114: 		sub	b
;17115: 		scf
;17116: 		rl	c
;17117: 		ret	nc
;17118: 		jr	loc_AC8B
;17119: ; End of function Div_C_DEc_HLa
;17120: 
;17121: ; ---------------------------------------------------------------------------
;17122: VertexCrdXlow:	db 0			; DATA XREF: MakeTablPrnCrdVertex+1DAw
;17123: 					; MakeTablPrnCrdVertex+2FEw ...
;17124: VertexCrdYlow:	db 0			; DATA XREF: MakeTablPrnCrdVertex+1DDw
;17125: 					; MakeTablPrnCrdVertex+307w ...
;17126: VertexCrdZlow:	db 0FFh			; DATA XREF: MakeTablPrnCrdVertex+1E0w
;17127: 					; MakeTablPrnCrdVertex+310w ...
;17128: ; ---------------------------------------------------------------------------
;17129: ; START	OF FUNCTION CHUNK FOR MakeTablPrnCrdVertex
;17130: 
;17131: CalcPrnCrdVertex:			; CODE XREF: MakeTablPrnCrdVertex:loc_AA03j
;17132: 		push	iy		; расчёт печатных координат вершины
;17133: 					; вх:  ix - адрес вершины
;17134: 					;      (sp-2) -	адрес слота объекта
;17135: 		ld	a, (DistanceToObj)
;17136: 		cp	4
;17137: 		jp	nc, RotateCrdVertex
;17138: 		ld	a, (ix+0)
;17139: 		or	(ix+1)
;17140: 		or	(ix+2)
