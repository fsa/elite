; 5250: 
; 5251: loc_753A:				; CODE XREF: RAM:755Ej
; 5252: 		push	bc
; 5253: 		call	ClearScrBuffer
; 5254: 		call	PrintStars
; 5255: 		call	j_j_MoveAllObjects
; 5256: ; ---------------------------------------------------------------------------
; 5257: 		ld	hl, (word_D0D5)
; 5258: 		inc	hl
; 5259: 		inc	hl
; 5260: 		push	hl
; 5261: 		set	7, (hl)
; 5262: 		ld	hl, 70Ah
; 5263: 		ld	(CrdXPrnSymbol), hl
; 5264: 		ld	a, 9
; 5265: 		call	sub_9381
; 5266: 		pop	hl
; 5267: 		res	7, (hl)
; 5268: 		call	CopyBuf2Screen
; 5269: 		pop	bc
; 5270: 		djnz	loc_753A
; 5271: 		call	ClearScrBuffer
; 5272: 		call	CopyBuf2Screen
; 5273: 		call	ClearAroundObj
; 5274: 		ld	b, 32h ; '2'
; 5275: 
; 5276: loc_756B:				; CODE XREF: RAM:756Cj
; 5277: 		halt
; 5278: 		djnz	loc_756B
; 5279: 		xor	a
; 5280: 		ld	(BlockControl),	a
; 5281: 		ld	(PlayerGameOver), a
; 5282: 
; 5283: loc_7575:				; DATA XREF: CalcPrnCrdSun+17o
; 5284: 		jp	loc_739E
; 5285: ; ---------------------------------------------------------------------------
; 5286: word_7578:	dw 0			; DATA XREF: RAM:7366w
; 5287: CircleCrdXlow:	db 0			; DATA XREF: PrnSunPlanet+7o
; 5288: 					; PrnSunPlanet+2Co ...
; 5289: CircleCrdXmed:	db 0			; DATA XREF: RAM:75FEr	RAM:76A9r ...
; 5290: CircleCrdXhi:	db 0			; DATA XREF: RAM:76C7r	RAM:76D1w ...
; 5291: CircleCrdYlow:	db 0			; DATA XREF: RAM:7627r	RAM:7701r
; 5292: CircleCrdYmed:	db 0			; DATA XREF: RAM:7620r	RAM:76FAr
; 5293: CircleCrdYhi:	db 0
; 5294: CircleCrdZlow:	db 0CAh			; DATA XREF: RAM:7601r	RAM:7623r ...
; 5295: CircleCrdZmed:	db 0BFh			; DATA XREF: RAM:75FAr	RAM:761Cr ...
; 5296: CircleCrdZhi:	db 0			; DATA XREF: RAM:75EDr	RAM:75F7w ...
; 5297: SunRadiusNova:	db 0			; DATA XREF: sub_728D+18Bw
; 5298: 					; CorrData4Nova+5r ...
; 5299: 
; 5300: ; =============== S U B	R O U T	I N E =======================================
; 5301: 
; 5302: 
; 5303: PrnSunPlanet:				; CODE XREF: sub_728D+261p
; 5304: 					; LaunchDocking+FAp
; 5305: 		xor	a
; 5306: 		ld	(TypeOfPrint), a
; 5307: 		ld	hl, SunCrd
; 5308: 		ld	de, CircleCrdXlow
; 5309: 		ld	bc, 9
; 5310: 		ldir
; 5311: 		call	CalcPrnCrdSun
; 5312: 		push	af
; 5313: 		call	CorrData4Nova
; 5314: 		pop	af
; 5315: 		and	a
; 5316: 		jr	z, loc_75AD
; 5317: 		call	PrnCircle_Fill
; 5318: 		ld	a, (CircleRadius)
; 5319: 		cp	28h ; '('
; 5320: 		jr	c, loc_75AD
; 5321: 		ld	a, 1
; 5322: 		ld	(TypeOfPrint), a
; 5323: 
; 5324: loc_75AD:				; CODE XREF: PrnSunPlanet+18j
; 5325: 					; PrnSunPlanet+22j
; 5326: 		ld	hl, PlanetCrd
; 5327: 		ld	de, CircleCrdXlow
; 5328: 		ld	bc, 9
; 5329: 		ldir
; 5330: 		call	CalcPrnCrdSun
; 5331: 		and	a
; 5332: 		ret	z
; 5333: 		jp	PrnCircle_NoFill
; 5334: ; End of function PrnSunPlanet
; 5335: 
; 5336: 
; 5337: ; =============== S U B	R O U T	I N E =======================================
; 5338: 
; 5339: 
; 5340: CorrData4Nova:				; CODE XREF: sub_74C6:loc_74D2p
; 5341: 					; PrnSunPlanet+13p
; 5342: 		ld	a, (PilotMission1_1)
; 5343: 		and	a
; 5344: 		ret	z
; 5345: 		ld	a, (SunRadiusNova)
; 5346: 		and	a
; 5347: 		jr	nz, loc_75CE
; 5348: 		ld	a, (CircleRadius)
; 5349: 
; 5350: loc_75CE:				; CODE XREF: CorrData4Nova+9j
; 5351: 		add	a, 2
; 5352: 		jr	z, loc_75D4
; 5353: 		jr	nc, loc_75DA
; 5354: 
; 5355: loc_75D4:				; CODE XREF: CorrData4Nova+10j
; 5356: 		ld	a, 1
; 5357: 		ld	(PlayerGameOver), a
; 5358: 		xor	a
; 5359: 
; 5360: loc_75DA:				; CODE XREF: CorrData4Nova+12j
; 5361: 		ld	(CircleRadius),	a
; 5362: 		ld	(SunRadiusNova), a
; 5363: 		ret
; 5364: ; End of function CorrData4Nova
; 5365: 
; 5366: ; ---------------------------------------------------------------------------
; 5367: 
; 5368: loc_75E1:				; CODE XREF: CalcPrnCrdSunp
; 5369: 		exx
; 5370: 		ld	bc, 3
; 5371: 		exx
; 5372: 		ld	a, (WhatOnScreen)
; 5373: 		and	a
; 5374: 		jp	nz, loc_7695
; 5375: 		ld	a, (CircleCrdZhi)
; 5376: 		bit	7, a
; 5377: 		jp	nz, loc_782A
; 5378: 		and	7Fh ; ''
; 5379: 		ld	(CircleCrdZhi),	a
; 5380: 		ld	hl, (CircleCrdZmed)
; 5381: 		ex	de, hl
; 5382: 		ld	hl, (CircleCrdXmed)
; 5383: 		ld	a, (CircleCrdZlow)
; 5384: 		ld	b, a
; 5385: 		ld	a, (CircleCrdXlow)
; 5386: 		ld	c, a
; 5387: 		ld	a, h
; 5388: 		bit	7, a
; 5389: 		jr	nz, loc_7612
; 5390: 		exx
; 5391: 		set	0, b
; 5392: 		exx
; 5393: 
; 5394: loc_7612:				; CODE XREF: RAM:760Cj	RAM:loc_76C0j
; 5395: 		and	7Fh ; ''
; 5396: 		ld	h, a
; 5397: 		call	sub_77A0
; 5398: 		ld	a, d
; 5399: 		ld	(CircleCrdY+1),	a
; 5400: 		ld	hl, (CircleCrdZmed)
; 5401: 		ex	de, hl
; 5402: 		ld	hl, (CircleCrdYmed)
; 5403: 		ld	a, (CircleCrdZlow)
; 5404: 		ld	b, a
; 5405: 		ld	a, (CircleCrdYlow)
; 5406: 		ld	c, a
; 5407: 		ld	a, h
; 5408: 		bit	7, a
; 5409: 		jr	nz, loc_7634
; 5410: 		exx
; 5411: 		set	1, b
; 5412: 		exx
; 5413: 
; 5414: loc_7634:				; CODE XREF: RAM:762Ej
; 5415: 		and	7Fh ; ''
; 5416: 		ld	h, a
; 5417: 		call	sub_77A0
; 5418: 		ld	a, d
; 5419: 		ld	(CircleCrdY), a
; 5420: 		exx
; 5421: 		ld	a, b
; 5422: 		exx
; 5423: 		ld	(CircleFlg), a
; 5424: 		ld	a, (CircleCrdZhi)
; 5425: 		and	a
; 5426: 		jr	nz, loc_7659
; 5427: 		ld	hl, 0A000h
; 5428: 		ld	de, (CircleCrdZlow)
; 5429: 		sbc	hl, de
; 5430: 		jr	c, loc_7659
; 5431: 		ld	a, 0FFh
; 5432: 		jr	loc_7692
; 5433: ; ---------------------------------------------------------------------------
; 5434: 
; 5435: loc_7659:				; CODE XREF: RAM:7648j	RAM:7653j
; 5436: 		ld	hl, 0A000h
; 5437: 		ld	de, (CircleCrdZmed)
; 5438: 		and	a
; 5439: 		sbc	hl, de
; 5440: 		jp	c, loc_782A
; 5441: 		ld	a, (CircleCrdZhi)
; 5442: 		and	a
; 5443: 		jr	z, loc_7688
; 5444: 		ld	b, a
; 5445: 		ld	a, (CircleCrdZmed)
; 5446: 		ld	c, a
; 5447: 		xor	a
; 5448: 		ld	e, 0A0h	; 'а'
; 5449: 
; 5450: loc_7674:				; CODE XREF: RAM:767Dj
