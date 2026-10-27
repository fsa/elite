;12600: 		ld	c, 2
;12601: 		jr	loc_967F
;12602: 
;12603: ; =============== S U B	R O U T	I N E =======================================
;12604: 
;12605: 
;12606: sub_969B:				; CODE XREF: FX_Launch+6p
;12607: 
;12608: ; FUNCTION CHUNK AT 9684 SIZE 0000000E BYTES
;12609: 
;12610: 		ld	a, 3
;12611: 		ld	(byte_9522), a
;12612: 		di
;12613: 		ld	bc, 5002h
;12614: 		ld	de, 4020h
;12615: 		jr	loc_9684
;12616: ; End of function sub_969B
;12617: 
;12618: 
;12619: ; =============== S U B	R O U T	I N E =======================================
;12620: 
;12621: 
;12622: sub_96A9:				; CODE XREF: sub_9296+2A2p
;12623: 		ld	bc, 0C0F0h
;12624: 		ld	de, 0C00Ch
;12625: 
;12626: loc_96AF:				; CODE XREF: sub_96B7+6j
;12627: 		di
;12628: 		ld	a, 3
;12629: 		ld	(byte_9522), a
;12630: 		jr	loc_9684
;12631: ; End of function sub_96A9
;12632: 
;12633: 
;12634: ; =============== S U B	R O U T	I N E =======================================
;12635: 
;12636: 
;12637: sub_96B7:				; CODE XREF: sub_953D+6p
;12638: 		ld	bc, 0C010h
;12639: 		ld	de, 100Ah
;12640: 		jr	loc_96AF
;12641: ; End of function sub_96B7
;12642: 
;12643: ; ---------------------------------------------------------------------------
;12644: byte_96BF:	db 2			; DATA XREF: sub_9579+18w sub_9579+9Br
;12645: ScanerCrdXobj:	db 0			; DATA XREF: sub_9736+Fw sub_9736+55r	...
;12646: ScanerCrdYobj:	db 0			; DATA XREF: sub_9736:loc_9765w
;12647: 					; sub_9736:loc_97F9r
;12648: ScanerCrdZobj:	db 93h			; DATA XREF: sub_9736:loc_9785w
;12649: 					; sub_9736+A1r
;12650: ScanerNumObj:	db 1			; DATA XREF: sub_96DC+Aw
;12651: 					; sub_96DC:loc_982Er ...
;12652: FlgSaveScaner:	db 0			; DATA XREF: sub_7A84+16w sub_96DC+46w ...
;12653: 
;12654: ; =============== S U B	R O U T	I N E =======================================
;12655: 
;12656: 
;12657: sub_96C5:				; CODE XREF: sub_9736+1Cp sub_9736+3Cp
;12658: 		srl	b
;12659: 		rr	c
;12660: ; End of function sub_96C5
;12661: 
;12662: 
;12663: ; =============== S U B	R O U T	I N E =======================================
;12664: 
;12665: 
;12666: sub_96C9:				; CODE XREF: sub_9736p
;12667: 		srl	b
;12668: 		rr	c
;12669: ; End of function sub_96C9
;12670: 
;12671: 
;12672: ; =============== S U B	R O U T	I N E =======================================
;12673: 
;12674: 
;12675: sub_96CD:				; CODE XREF: sub_9736+21p sub_9736+41p
;12676: 		srl	b
;12677: 		rr	c
;12678: ; End of function sub_96CD
;12679: 
;12680: 
;12681: ; =============== S U B	R O U T	I N E =======================================
;12682: 
;12683: 
;12684: sub_96D1:				; CODE XREF: sub_9736+5p
;12685: 		srl	b
;12686: 		rr	c
;12687: 		ret
;12688: ; End of function sub_96D1
;12689: 
;12690: ; ---------------------------------------------------------------------------
;12691: ; START	OF FUNCTION CHUNK FOR sub_96DC
;12692: 
;12693: loc_96D6:				; CODE XREF: sub_96DC+15j sub_96DC+1Cj ...
;12694: 		jp	loc_982E
;12695: ; END OF FUNCTION CHUNK	FOR sub_96DC
;12696: ; ---------------------------------------------------------------------------
;12697: ; START	OF FUNCTION CHUNK FOR sub_ED60
;12698: 
;12699: PrintObjScaner:				; CODE XREF: sub_ED60+6j
;12700: 		call	CopyLocator2Scr
;12701: ; END OF FUNCTION CHUNK	FOR sub_ED60
;12702: 
;12703: ; =============== S U B	R O U T	I N E =======================================
;12704: 
;12705: 
;12706: sub_96DC:
;12707: 
;12708: ; FUNCTION CHUNK AT 96D6 SIZE 00000003 BYTES
;12709: ; FUNCTION CHUNK AT 982E SIZE 00000024 BYTES
;12710: ; FUNCTION CHUNK AT B2EF SIZE 00000005 BYTES
;12711: 
;12712: 		ld	a, (FlgObjNear)
;12713: 		and	a
;12714: 		ret	z
;12715: 		call	SetIndStation
;12716: 		ld	a, 8
;12717: 		ld	(ScanerNumObj),	a
;12718: 		ld	ix, objCoriolis
;12719: 
;12720: loc_96ED:				; CODE XREF: sub_96DC+160j
;12721: 		ld	a, (ix+22h)
;12722: 		and	a
;12723: 		jr	z, loc_96D6
;12724: 		ld	a, (ix+1)
;12725: 		cp	40h ; '@'
;12726: 		jr	nc, loc_96D6
;12727: 		ld	a, (ix+4)
;12728: 		cp	40h ; '@'
;12729: 		jr	nc, loc_96D6
;12730: 		ld	a, (ix+7)
;12731: 		cp	40h ; '@'
;12732: 		jr	nc, loc_96D6
;12733: 		ld	a, (ix+1)
;12734: 		call	Sqr_DE_A
;12735: 		ex	de, hl
;12736: 		ld	a, (ix+7)
;12737: 		call	Sqr_DE_A
;12738: 		add	hl, de
;12739: 		jr	c, loc_96D6
;12740: 		ld	de, 1600h
;12741: 		and	a
;12742: 		sbc	hl, de
;12743: 		jr	nc, loc_96D6
;12744: 		ld	a, 1
;12745: 		ld	(FlgSaveScaner), a
;12746: 		ld	de, 4000h
;12747: 		xor	a
;12748: 		ld	l, (ix+0)
;12749: 		ld	h, (ix+1)
;12750: 
;12751: loc_972F:				; DATA XREF: sub_7A84o
;12752: 		and	a
;12753: 		jp	c, loc_B2EF
;12754: 		call	Div_BC_HLAdivDE
;12755: ; End of function sub_96DC
;12756: 
;12757: 
;12758: ; =============== S U B	R O U T	I N E =======================================
;12759: 
;12760: ; Attributes: noreturn
;12761: 
;12762: sub_9736:
;12763: 
;12764: ; FUNCTION CHUNK AT 985E SIZE 0000000C BYTES
;12765: 
;12766: 		call	sub_96C9
;12767: 		ld	h, b
;12768: 		ld	l, c
;12769: 		call	sub_96D1
;12770: 		add	hl, bc
;12771: 		ld	a, (ix+2)
;12772: 		and	80h ; 'А'
;12773: 		or	h
;12774: 		ld	(ScanerCrdXobj), a
;12775: 		xor	a
;12776: 		ld	l, (ix+3)
;12777: 		ld	h, (ix+4)
;12778: 		call	Div_BC_HLAdivDE
;12779: ; ---------------------------------------------------------------------------
;12780: 		call	sub_96C5
;12781: 		ld	h, b
;12782: 		ld	l, c
;12783: 		call	sub_96CD
;12784: 		add	hl, bc
;12785: 		ld	a, (ix+5)
;12786: 		and	80h ; 'А'
;12787: 		ld	a, h
;12788: 		jr	z, loc_9765
;12789: 		or	80h ; 'А'
;12790: 
;12791: loc_9765:				; CODE XREF: sub_9736+2Bj
;12792: 		ld	(ScanerCrdYobj), a
;12793: 		xor	a
;12794: 		ld	l, (ix+6)
;12795: 		ld	h, (ix+7)
;12796: 		call	Div_BC_HLAdivDE
;12797: ; ---------------------------------------------------------------------------
;12798: 		call	sub_96C5
;12799: 		ld	h, b
;12800: 		ld	l, c
;12801: 		call	sub_96CD
;12802: 		add	hl, bc
;12803: 		ld	a, (ix+8)
;12804: 		and	80h ; 'А'
;12805: 		ld	a, h
;12806: 		jr	z, loc_9785
;12807: 		or	80h ; 'А'
;12808: 
;12809: loc_9785:				; CODE XREF: sub_9736+4Bj
;12810: 		ld	(ScanerCrdZobj), a
;12811: 		ld	hl, 558Fh
;12812: 		ld	a, (ScanerCrdXobj)
;12813: 		and	7Fh ; ''
;12814: 		sub	4
;12815: 		ld	c, a
;12816: 		jp	c, loc_97BB
;12817: 		rra
;12818: 		rra
;12819: 		rra
;12820: 		and	1Fh
;12821: 		ld	b, a
;12822: 		ld	a, c
;12823: 		and	7
;12824: 		ld	c, a
;12825: 		jr	z, loc_97A3
;12826: 		inc	b
;12827: 
;12828: loc_97A3:				; CODE XREF: sub_9736+6Aj
;12829: 		ld	a, (ScanerCrdXobj)
;12830: 		and	80h ; 'А'
;12831: 		jr	nz, loc_97C7
;12832: 
;12833: loc_97AA:				; CODE XREF: sub_9736+75j
;12834: 		inc	hl
;12835: 		djnz	loc_97AA
;12836: 
;12837: loc_97AD:				; CODE XREF: sub_9736+8Fj
;12838: 		ld	a, c
;12839: 		and	a
;12840: 		ld	b, a
;12841: 		ld	a, 1
;12842: 		jr	z, loc_97D6
;12843: 		xor	a
;12844: 		scf
;12845: 
;12846: loc_97B6:				; CODE XREF: sub_9736+81j
;12847: 		rra
;12848: 		djnz	loc_97B6
;12849: 		jr	loc_97D6
;12850: ; ---------------------------------------------------------------------------
;12851: 
;12852: loc_97BB:				; CODE XREF: sub_9736+5Dj
;12853: 		add	a, 8
;12854: 		ld	c, a
;12855: 		ld	a, (ScanerCrdXobj)
;12856: 		and	80h ; 'А'
;12857: 		jr	nz, loc_97CA
;12858: 		jr	loc_97AD
;12859: ; ---------------------------------------------------------------------------
;12860: 
;12861: loc_97C7:				; CODE XREF: sub_9736+72j sub_9736+92j
;12862: 		dec	hl
;12863: 		djnz	loc_97C7
;12864: 
;12865: loc_97CA:				; CODE XREF: sub_9736+8Dj
;12866: 		ld	a, c
;12867: 		and	a
;12868: 		ld	b, a
;12869: 		ld	a, 80h ; 'А'
;12870: 		jr	z, loc_97D6
;12871: 		xor	a
;12872: 		scf
;12873: 
;12874: loc_97D3:				; CODE XREF: sub_9736+9Ej
;12875: 		rla
;12876: 		djnz	loc_97D3
;12877: 
;12878: loc_97D6:				; CODE XREF: sub_9736+7Cj sub_9736+83j ...
;12879: 		ld	c, a
;12880: 		ld	a, (ScanerCrdZobj)
;12881: 		ld	d, a
;12882: 		and	7Fh ; ''
;12883: 		ld	b, a
;12884: 		jr	z, loc_97F9
;12885: 		ld	a, d
;12886: 		and	80h ; 'А'
;12887: 		jr	nz, loc_97F0
;12888: 
;12889: loc_97E5:				; CODE XREF: sub_9736+B6j
;12890: 		ld	a, h
;12891: 		dec	h
;12892: 		and	7
;12893: 		call	z, compUpScrAdr_HL
;12894: 		djnz	loc_97E5
;12895: 		jr	loc_97F9
;12896: ; ---------------------------------------------------------------------------
;12897: 
;12898: loc_97F0:				; CODE XREF: sub_9736+ADj sub_9736+C1j
;12899: 		inc	h
;12900: 		ld	a, h
;12901: 		and	7
;12902: 		call	z, compDownScrAdr_HL
;12903: 		djnz	loc_97F0
;12904: 
;12905: loc_97F9:				; CODE XREF: sub_9736+A8j sub_9736+B8j
;12906: 		ld	a, (ScanerCrdYobj)
;12907: 		ld	d, a
;12908: 		and	7Fh ; ''
;12909: 		ld	b, a
;12910: 		jr	z, loc_9821
;12911: 		ld	a, d
;12912: 		and	80h ; 'А'
;12913: 		jr	nz, loc_9815
;12914: 
;12915: loc_9807:				; CODE XREF: sub_9736+DBj
;12916: 		ld	a, (hl)
;12917: 		or	c
;12918: 		ld	(hl), a
;12919: 		ld	a, h
;12920: 		dec	h
;12921: 		and	7
;12922: 		call	z, sub_987C
;12923: 		djnz	loc_9807
;12924: 		jr	loc_9821
;12925: ; ---------------------------------------------------------------------------
;12926: 
;12927: loc_9815:				; CODE XREF: sub_9736+CFj sub_9736+E9j
;12928: 		ld	a, (hl)
;12929: 		or	c
;12930: 		ld	(hl), a
;12931: 		inc	h
;12932: 		ld	a, h
;12933: 		and	7
;12934: 		call	z, sub_986A
;12935: 		djnz	loc_9815
;12936: 
;12937: loc_9821:				; CODE XREF: sub_9736+CAj sub_9736+DDj
;12938: 		ld	a, c
;12939: 		add	a, a
;12940: 		jr	c, sub_9852
;12941: 		or	c
;12942: 		add	a, a
;12943: 		jr	c, loc_985E
;12944: 		or	c
;12945: 		ld	e, a
;12946: 		ld	a, (hl)
;12947: 		or	e
;12948: 		ld	(hl), a
;12949: ; End of function sub_9736
;12950: 
;12951: ; START	OF FUNCTION CHUNK FOR sub_96DC
;12952: 
;12953: loc_982E:				; CODE XREF: sub_96DC:loc_96D6j
;12954: 					; sub_9852+Aj ...
;12955: 		ld	a, (ScanerNumObj)
;12956: 		dec	a
;12957: 		jr	z, loc_983F
;12958: 		ld	(ScanerNumObj),	a
;12959: 		ld	de, 27h	; '''
;12960: 		add	ix, de
;12961: 		jp	loc_96ED
;12962: ; ---------------------------------------------------------------------------
;12963: 
;12964: loc_983F:				; CODE XREF: sub_96DC+156j
;12965: 		xor	a
;12966: 		ld	c, 7
;12967: 		ld	hl, 5009h
;12968: 
;12969: loc_9845:				; CODE XREF: sub_96DC+173j
;12970: 		ld	b, 0Dh
;12971: 		push	hl
;12972: 
;12973: loc_9848:				; CODE XREF: sub_96DC+16Ej
;12974: 		ld	(hl), a
;12975: 		inc	l
;12976: 		djnz	loc_9848
;12977: 		pop	hl
;12978: 		inc	h
;12979: 		dec	c
;12980: 		jr	nz, loc_9845
;12981: 		ret
;12982: ; END OF FUNCTION CHUNK	FOR sub_96DC
;12983: 
;12984: ; =============== S U B	R O U T	I N E =======================================
;12985: 
;12986: 
;12987: sub_9852:				; CODE XREF: sub_9736+EDj
;12988: 		dec	l
;12989: 		ld	a, (hl)
;12990: 		or	3
;12991: 		ld	(hl), a
;12992: 		inc	l
;12993: 		ld	a, (hl)
;12994: 		or	80h ; 'А'
;12995: 		ld	(hl), a
;12996: 		jr	loc_982E
;12997: ; End of function sub_9852
;12998: 
;12999: ; ---------------------------------------------------------------------------
;13000: ; START	OF FUNCTION CHUNK FOR sub_9736
;13001: 
;13002: loc_985E:				; CODE XREF: sub_9736+F1j
;13003: 		dec	l
;13004: 		ld	a, (hl)
;13005: 		or	1
;13006: 		ld	(hl), a
;13007: 		inc	l
;13008: 		ld	a, (hl)
;13009: 		or	0C0h ; '└'
;13010: 		ld	(hl), a
;13011: 		jr	loc_982E
;13012: ; END OF FUNCTION CHUNK	FOR sub_9736
;13013: 
;13014: ; =============== S U B	R O U T	I N E =======================================
;13015: 
;13016: 
;13017: sub_986A:				; CODE XREF: sub_9736+E6p
;13018: 		ld	a, l
;13019: 		add	a, 20h ; ' '
;13020: 		ld	l, a
;13021: 		jr	nc, loc_9877
;13022: 		ld	a, 58h ; 'X'
;13023: 		cp	h
;13024: 		ret	nz
;13025: 		pop	af
;13026: 		jr	loc_982E
;13027: ; ---------------------------------------------------------------------------
;13028: 
;13029: loc_9877:				; CODE XREF: sub_986A+4j
;13030: 		ld	a, h
;13031: 		sub	8
;13032: 		ld	h, a
;13033: 		ret
;13034: ; End of function sub_986A
;13035: 
;13036: 
;13037: ; =============== S U B	R O U T	I N E =======================================
;13038: 
;13039: 
;13040: sub_987C:				; CODE XREF: sub_9736+D8p
;13041: 		ld	a, l
;13042: 		cp	0Fh
;13043: 		jr	nz, loc_9885
;13044: 		pop	af
;13045: 		jp	loc_982E
;13046: ; ---------------------------------------------------------------------------
;13047: 
;13048: loc_9885:				; CODE XREF: sub_987C+3j
;13049: 		sub	20h ; ' '
;13050: 		ld	l, a
;13051: 		ret	c
;13052: 		ld	a, h
;13053: 		add	a, 8
;13054: 		ld	h, a
;13055: 		ret
;13056: ; End of function sub_987C
;13057: 
;13058: 
;13059: ; =============== S U B	R O U T	I N E =======================================
;13060: 
;13061: 
;13062: SetIndStation:				; CODE XREF: sub_74C6+6p sub_96DC+5p
;13063: 		ld	a, (FlgCoriolisNear)
;13064: 		and	a
;13065: 		ret	z
;13066: 		ld	a, (IndDanger)
;13067: 		cp	63h ; 'c'
;13068: 		ret	z
;13069: 		cp	2
;13070: 		ret	z
;13071: 		ld	a, 6
;13072: 		ld	(IndDanger), a
;13073: 		ret
;13074: ; End of function SetIndStation
;13075: 
;13076: ; ---------------------------------------------------------------------------
;13077: 
;13078: loc_98A2:				; CODE XREF: PrnCocpit_Ifn+41Aj
;13079: 		call	CopyScaner2Scr
;13080: 		ld	bc, 5
;13081: 		ld	a, 0EBh	; 'ы'
;13082: 		ld	hl, loc_9912
;13083: 		ld	(hl), a
;13084: 		ld	a, 0E5h	; 'х'
;13085: 		ld	hl, loc_9915
;13086: 		ld	(hl), a
;13087: 		add	hl, bc
;13088: 		ld	(hl), a
;13089: 		ld	a, 0E1h	; 'с'
;13090: 		ld	hl, loc_9935
;13091: 		ld	(hl), a
;13092: 		inc	hl
;13093: 		ld	a, 24h ; '$'
;13094: 		ld	(hl), a
;13095: 		xor	a
;13096: 		ld	hl, loc_9940
;13097: 		ld	(hl), a
;13098: 		ld	a, 0EBh	; 'ы'
;13099: 		ld	hl, loc_9939
;13100: 		ld	(hl), a
;13101: 		jr	loc_990C
;13102: 
;13103: ; =============== S U B	R O U T	I N E =======================================
;13104: 
;13105: 
;13106: CopyScaner2Scr:				; CODE XREF: RAM:7529p
;13107: 					; SaveCocpitInBuffer+1p ...
;13108: 
;13109: ; FUNCTION CHUNK AT 9935 SIZE 00000010 BYTES
;13110: 
;13111: 		ld	a, (FlgSaveScaner)
;13112: 		dec	a
;13113: 		ret	nz
;13114: 		ld	(FlgSaveScaner), a
;13115: 		xor	a
;13116: 		ld	c, 8
;13117: 		ld	hl, 502Ah
;13118: 
;13119: loc_98DB:				; CODE XREF: CopyScaner2Scr+18j
;13120: 		ld	b, 0Bh
;13121: 		push	hl
;13122: 
;13123: loc_98DE:				; CODE XREF: CopyScaner2Scr+13j
;13124: 		ld	(hl), a
;13125: 		inc	l
;13126: 		djnz	loc_98DE
;13127: 		pop	hl
;13128: 		inc	h
;13129: 		dec	c
;13130: 		jr	nz, loc_98DB
;13131: 		ld	bc, 5
;13132: 		xor	a
;13133: 		ld	hl, loc_9912
;13134: 		ld	(hl), a
;13135: 		ld	a, 0D5h	; '╒'
;13136: 		ld	hl, loc_9915
;13137: 		ld	(hl), a
;13138: 		add	hl, bc
;13139: 		ld	(hl), a
;13140: 		ld	a, 0D1h	; '╤'
;13141: 		ld	hl, loc_9935
;13142: 		ld	(hl), a
;13143: 		inc	hl
;13144: 		ld	a, 14h
;13145: 		ld	(hl), a
;13146: 		ld	a, 0EBh	; 'ы'
;13147: 		ld	hl, loc_9940
;13148: 		ld	(hl), a
;13149: 		xor	a
;13150: 		ld	hl, loc_9939
;13151: 		ld	(hl), a
;13152: 
;13153: loc_990C:				; CODE XREF: RAM:98CBj
;13154: 		ld	de, 5049h
;13155: 		ld	hl, 5B00h
;13156: 
;13157: loc_9912:				; DATA XREF: RAM:98AAo
;13158: 					; CopyScaner2Scr+1Eo
;13159: 		nop
;13160: 		ld	a, 6
;13161: 
;13162: loc_9915:				; CODE XREF: CopyScaner2Scr+75j
;13163: 					; DATA XREF: RAM:98B0o	...
;13164: 		push	de
;13165: 		ld	b, 8
;13166: 		ld	c, 0FFh
;13167: 
;13168: loc_991A:				; CODE XREF: CopyScaner2Scr+6Aj
;13169: 		push	de
;13170: 		push	bc
;13171: 		ld	bc, 0Dh
;13172: 		ldir
;13173: 		pop	bc
;13174: 		jr	loc_9935
;13175: ; End of function CopyScaner2Scr
;13176: 
;13177: 
;13178: ; =============== S U B	R O U T	I N E =======================================
;13179: 
;13180: 
;13181: sub_9924:				; CODE XREF: sub_78B1+7Bp sub_78B1+83p ...
;13182: 
;13183: ; FUNCTION CHUNK AT B286 SIZE 00000019 BYTES
;13184: 
;13185: 		ld	h, 0
;13186: 		add	a, a
;13187: 		and	a
;13188: 		jp	loc_B286
;13189: ; End of function sub_9924
;13190: 
;13191: ; ---------------------------------------------------------------------------
;13192: byte_992B:	db 21h			; DATA XREF: CopyLocator2Scr+Do
;13193: 		db    6
;13194: 		db 0ACh	; м
;13195: 		db    6
;13196: 		db  21h	; !
;13197: 		db    0
;13198: 		db 0EDh	; э
;13199: 		db 0A0h	; а
;13200: 		db 0EDh	; э
;13201: 		db 0A0h	; а
;13202: ; ---------------------------------------------------------------------------
;13203: ; START	OF FUNCTION CHUNK FOR CopyScaner2Scr
;13204: 
;13205: loc_9935:				; CODE XREF: CopyScaner2Scr+55j
;13206: 					; DATA XREF: RAM:98B8o	...
;13207: 		pop	de
;13208: 		inc	d
;13209: 		djnz	loc_991A
;13210: 
;13211: loc_9939:				; DATA XREF: RAM:98C7o
;13212: 					; CopyScaner2Scr+3Bo
;13213: 		nop
;13214: 		pop	de
;13215: 		ex	de, hl
;13216: 		ld	bc, 20h	; ' '
;13217: 		add	hl, bc
;13218: 
;13219: loc_9940:				; DATA XREF: RAM:98C1o
;13220: 					; CopyScaner2Scr+36o
;13221: 		ex	de, hl
;13222: 		dec	a
;13223: 		jr	nz, loc_9915
;13224: 		ret
;13225: ; END OF FUNCTION CHUNK	FOR CopyScaner2Scr
;13226: ; ---------------------------------------------------------------------------
;13227: 		db    0
;13228: 
;13229: ; =============== S U B	R O U T	I N E =======================================
;13230: 
;13231: 
;13232: sub_9946:				; CODE XREF: LaunchDocking+8Dp
;13233: 		call	j_PrintMissiles
;13234: 		call	j_ResetBattleMessages
;13235: 		xor	a
;13236: 		ld	(Missile1), a
;13237: 		ld	(TempLaser), a
;13238: 		ld	(Temperature), a
;13239: 		ld	(AutoDocking), a
;13240: 		ld	(TimeWorkECM), a
;13241: 		ld	(byte_9C1B), a
;13242: 		ld	a, 0A0h	; 'а'
;13243: 		ld	(ShieldFront), a
;13244: 		ld	(ShieldRear), a
;13245: 		ld	(Altitude), a
;13246: 		ld	hl, ShieldUnit1
;13247: 		ld	b, 4
;13248: 
;13249: loc_996F:				; CODE XREF: sub_9946+2Cj
;13250: 		ld	(hl), a
;13251: 		inc	hl
;13252: 		inc	hl
;13253: 		djnz	loc_996F
;13254: 
;13255: loc_9974:				; CODE XREF: RAM:loc_712Dj
;13256: 		ld	hl, 5A23h
;13257: 		ld	a, (WhatOnScreen)
;13258: 		cp	4
;13259: 		jr	c, loc_997F
;13260: 		inc	h
;13261: 
;13262: loc_997F:				; CODE XREF: sub_9946+36j
;13263: 		ld	c, 17h
;13264: 		ld	a, (PilotEscapePod)
;13265: 		and	a
;13266: 		jr	z, loc_9989
;13267: 		ld	c, 1Fh
;13268: 
;13269: loc_9989:				; CODE XREF: sub_9946+3Fj
;13270: 		ld	de, 1Bh
;13271: 		push	hl
;13272: 		call	sub_9995
;13273: 		pop	hl
;13274: 		ld	a, l
;13275: 		add	a, 15h
;13276: 		ld	l, a
;13277: ; End of function sub_9946
;13278: 
;13279: 
;13280: ; =============== S U B	R O U T	I N E =======================================
;13281: 
;13282: 
;13283: sub_9995:				; CODE XREF: sub_9946+47p
;13284: 		ld	b, 7
;13285: 
;13286: loc_9997:				; CODE XREF: RAM:99A0j
;13287: 		push	bc
;13288: 
;13289: loc_9998:
;13290: 		ld	b, 5
;13291: ; End of function sub_9995
;13292: 
;13293: 
;13294: loc_999A:				; CODE XREF: RAM:999Cj
;13295: 		ld	(hl), c
;13296: 		inc	hl
;13297: 		djnz	loc_999A
;13298: 		pop	bc
;13299: 		add	hl, de
;13300: 		djnz	loc_9997
