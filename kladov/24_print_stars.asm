; 5870: 		ret
; 5871: ; END OF FUNCTION CHUNK	FOR CalcCrdCoriolis2
; 5872: 
; 5873: ; =============== S U B	R O U T	I N E =======================================
; 5874: 
; 5875: 
; 5876: PrintStars:				; CODE XREF: sub_728D+264p RAM:753Ep ...
; 5877: 		ld	a, (StarsFastScroll)
; 5878: 		dec	a
; 5879: 		jr	nz, sub_78B1
; 5880: 		call	sub_F9A8
; 5881: 		ld	a, 1Eh
; 5882: 		ld	(HighDeltaSpeed), a
; 5883: 		call	sub_78B1
; 5884: 		ld	a, 10h
; 5885: 		ld	(HighDeltaSpeed), a
; 5886: 		call	sub_78B1
; 5887: ; End of function PrintStars
; 5888: 
; 5889: 
; 5890: ; =============== S U B	R O U T	I N E =======================================
; 5891: 
; 5892: 
; 5893: sub_78B1:				; CODE XREF: PrintStars+4j
; 5894: 					; PrintStars+Ep
; 5895: 		ld	a, (WhatOnScreen)
; 5896: 
; 5897: loc_78B4:				; DATA XREF: sub_78B1+5w
; 5898: 		cp	0
; 5899: 		ld	(loc_78B4+1), a
; 5900: 		jr	z, loc_78C5
; 5901: 		ld	b, 20h ; ' '
; 5902: 		ld	hl, byte_7A39
; 5903: 
; 5904: loc_78C0:				; CODE XREF: sub_78B1+12j
; 5905: 		ld	(hl), 0FFh
; 5906: 		inc	hl
; 5907: 		djnz	loc_78C0
; 5908: 
; 5909: loc_78C5:				; CODE XREF: sub_78B1+8j
; 5910: 		ld	b, 0
; 5911: 		cp	2
; 5912: 		jr	nc, loc_78D7
; 5913: 		push	af
; 5914: 		rrca
; 5915: 		ld	b, a
; 5916: 		ld	a, (HighDeltaSpeed)
; 5917: 		srl	a
; 5918: 		inc	a
; 5919: 		or	b
; 5920: 		ld	b, a
; 5921: 		pop	af
; 5922: 
; 5923: loc_78D7:				; CODE XREF: sub_78B1+18j
; 5924: 		ld	hl, HighAngleLR
; 5925: 		jr	c, loc_78DD
; 5926: 		inc	hl
; 5927: 
; 5928: loc_78DD:				; CODE XREF: sub_78B1+29j
; 5929: 		push	af
; 5930: 		ld	a, (hl)
; 5931: 		jr	c, loc_78E9
; 5932: 		and	80h ; 'А'
; 5933: 		ld	c, a
; 5934: 		ld	a, (hl)
; 5935: 		rrca
; 5936: 		and	3Fh ; '?'
; 5937: 		or	c
; 5938: 
; 5939: loc_78E9:				; CODE XREF: sub_78B1+2Ej
; 5940: 		ld	c, a
; 5941: 		pop	af
; 5942: 		push	af
; 5943: 		rrca
; 5944: 		and	80h ; 'А'
; 5945: 		push	af
; 5946: 		xor	c
; 5947: 		ld	c, a
; 5948: 		ld	a, 1
; 5949: 		xor	l
; 5950: 		ld	l, a
; 5951: 		pop	de
; 5952: 		push	de
; 5953: 		rrca
; 5954: 		cpl
; 5955: 		and	80h ; 'А'
; 5956: 		xor	d
; 5957: 		xor	(hl)
; 5958: 		ld	d, a
; 5959: 		pop	af
; 5960: 		ld	l, 0E6h	; 'ц'
; 5961: 		xor	(hl)
; 5962: 		ld	e, a
; 5963: 		inc	e
; 5964: 		pop	af
; 5965: 		cp	2
; 5966: 		jr	nc, loc_790C
; 5967: 		ld	e, 0
; 5968: 
; 5969: loc_790C:				; CODE XREF: sub_78B1+57j
; 5970: 		ld	a, (InFreeSpace)
; 5971: 		and	a
; 5972: 		ld	a, 10h
; 5973: 		jr	z, loc_7916
; 5974: 		ld	a, 4
; 5975: 
; 5976: loc_7916:				; CODE XREF: sub_78B1+61j
; 5977: 		push	af
; 5978: 		ld	hl, byte_7A39
; 5979: 
; 5980: loc_791A:				; CODE XREF: sub_78B1+F6j
; 5981: 		push	af
; 5982: 		ld	a, (hl)
; 5983: 		inc	hl
; 5984: 		and	(hl)
; 5985: 		inc	a
; 5986: 		jp	z, loc_79A4
; 5987: 		push	de
; 5988: 		dec	hl
; 5989: 		ld	d, (hl)
; 5990: 		inc	hl
; 5991: 		ld	e, (hl)
; 5992: 		push	hl
; 5993: 		ld	a, d
; 5994: 		add	a, a
; 5995: 		ld	l, a
; 5996: 		ld	a, c
; 5997: 		call	sub_9924
; 5998: 		ld	d, a
; 5999: 		ld	a, e
; 6000: 		add	a, a
; 6001: 		ld	l, a
; 6002: 		ld	a, c
; 6003: 		call	sub_9924
; 6004: 		ld	e, a
; 6005: 		pop	hl
; 6006: 		ld	a, (hl)
; 6007: 		cpl
; 6008: 		xor	c
; 6009: 		and	80h ; 'А'
; 6010: 		or	e
; 6011: 		ex	af, af'
; 6012: 		dec	hl
; 6013: 		ld	a, (hl)
; 6014: 		ex	af, af'
; 6015: 		call	sub_7A59
; 6016: 		inc	hl
; 6017: 		ex	af, af'
; 6018: 		xor	c
; 6019: 		and	80h ; 'А'
; 6020: 		or	d
; 6021: 		call	sub_7A59
; 6022: 		ld	a, b
; 6023: 		and	a
; 6024: 		jr	z, loc_797B
; 6025: 		dec	hl
; 6026: 		ld	a, (hl)
; 6027: 		add	a, a
; 6028: 		push	hl
; 6029: 		ld	l, a
; 6030: 		ld	a, b
; 6031: 		call	sub_9924
; 6032: 		inc	a
; 6033: 		ld	d, a
; 6034: 		pop	hl
; 6035: 		ld	a, (hl)
; 6036: 		xor	b
; 6037: 		and	80h ; 'А'
; 6038: 		or	d
; 6039: 		call	sub_7A59
; 6040: 		inc	hl
; 6041: 		ld	a, (hl)
; 6042: 		push	hl
; 6043: 		add	a, a
; 6044: 		ld	l, a
; 6045: 		ld	a, b
; 6046: 		call	sub_9924
; 6047: 		inc	a
; 6048: 		pop	hl
; 6049: 		ld	e, a
; 6050: 		ld	a, (hl)
; 6051: 		xor	b
; 6052: 		and	80h ; 'А'
; 6053: 		or	e
; 6054: 		call	sub_7A59
; 6055: 
; 6056: loc_797B:				; CODE XREF: sub_78B1+A0j
; 6057: 		pop	de
; 6058: 		ld	a, e
; 6059: 		call	sub_7A59
; 6060: 		dec	hl
; 6061: 		ld	a, d
; 6062: 		call	sub_7A59
; 6063: 		ld	a, b
; 6064: 		rlca
; 6065: 		ld	a, (hl)
; 6066: 		inc	hl
; 6067: 		jr	nc, loc_7992
; 6068: 		or	(hl)
; 6069: 		and	70h ; 'p'
; 6070: 		jr	nz, loc_7992
; 6071: 		ld	(hl), 0FFh
; 6072: 
; 6073: loc_7992:				; CODE XREF: sub_78B1+D8j sub_78B1+DDj
; 6074: 		ld	a, r
; 6075: 		rrca
; 6076: 		and	81h ; 'Б'
; 6077: 		call	sub_7A59
; 6078: 		dec	hl
; 6079: 		ld	a, r
; 6080: 		rrca
; 6081: 		and	81h ; 'Б'
; 6082: 		call	sub_7A59
; 6083: 		inc	hl
; 6084: 
; 6085: loc_79A4:				; CODE XREF: sub_78B1+6Ej
; 6086: 		inc	hl
; 6087: 		pop	af
; 6088: 		dec	a
; 6089: 		jp	nz, loc_791A
; 6090: 		pop	af
; 6091: 		push	af
; 6092: 
; 6093: loc_79AC:				; CODE XREF: sub_78B1+119j
; 6094: 		push	af
; 6095: 		dec	hl
; 6096: 		ld	a, (hl)
; 6097: 		dec	hl
; 6098: 		or	80h ; 'А'
; 6099: 		inc	a
; 6100: 		jr	z, loc_79BA
