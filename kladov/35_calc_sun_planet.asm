; 5822: 
; 5823: 
; 5824: ; =============== S U B	R O U T	I N E =======================================
; 5825: 
; 5826: 
; 5827: CalcCrdSunPlanet:			; CODE XREF: sub_728D+22Cp
; 5828: 					; LaunchDocking+F1p
; 5829: 		ld	ix, SunCrd
; 5830: 		call	j_CalcNewCrdObj_Pl
; 5831: 		ld	ix, PlanetCrd
; 5832: 		jp	CalcCrdSunPlanet_2
; 5833: ; End of function CalcCrdSunPlanet
; 5834: 
; 5835: ; ---------------------------------------------------------------------------
; 5836: ; START	OF FUNCTION CHUNK FOR CalcCrdCoriolis2
; 5837: 
; 5838: CalcCrdCoriolis:			; CODE XREF: CalcCrdCoriolis2+5j
; 5839: 		ld	a, (J_Drive_Coriolis)
; 5840: 		and	a
; 5841: 		ld	ix, objCoriolis
; 5842: 		jr	nz, loc_7874
; 5843: 		ld	a, (ix+22h)
; 5844: 		and	a
; 5845: 		ret	nz
; 5846: 		dec	a
; 5847: 		ld	(J_Drive_Coriolis), a
; 5848: 
; 5849: loc_7874:				; CODE XREF: CalcCrdCoriolis2+1AA1j
; 5850: 		call	ReCalcCrdObj_coriolis
; 5851: 		ld	a, (ix+8)
; 5852: 		and	7Fh ; ''
; 5853: 		ret	nz
; 5854: 		ld	a, (ix+2)
; 5855: 		and	7Fh ; ''
; 5856: 		ret	nz
; 5857: 		ld	a, (ix+5)
; 5858: 		and	7Fh ; ''
; 5859: 		ret	nz
; 5860: 
; 5861: loc_7889:				; CODE XREF: ReCalcCrdObj_coriolis+6Dj
; 5862: 		bit	7, (ix+26h)
; 5863: 		ret	nz
; 5864: 		xor	a
; 5865: 		ld	(J_Drive_Coriolis), a
; 5866: 
; 5867: loc_7892:				; CODE XREF: ReCalcCrdObj_coriolis+62p
; 5868: 		ld	a, 0F0h	; 'Ё'
; 5869: 		ld	(ix+22h), a
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
