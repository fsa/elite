;28420: 		call	PrintSmallCursOnMap
;28421: 		call	SearchPlanetNearCursor
;28422: 		jp	PrintSmallCursOnMap
;28423: ; End of function SearchNearPlanet
;28424: 
;28425: 
;28426: ; =============== S U B	R O U T	I N E =======================================
;28427: 
;28428: 
;28429: SearchPlanetNearCursor:			; CODE XREF: sub_5FD5+3Ap InitGame+1Ap ...
;28430: 		call	CopyGalaxyToPlanet
;28431: 		ld	hl, 0FFFFh
;28432: 		ld	(MinDistance), hl
;28433: 		ld	b, 0
;28434: 
;28435: loc_DF7F:				; CODE XREF: SearchPlanetNearCursor+2Cj
;28436: 		push	bc
;28437: 		ld	a, (DataOfPlanet1+1)
;28438: 		ld	b, a
;28439: 		ld	a, (DataOfPlanet+1)
;28440: 		call	CalcDistanceToCursor
;28441: 		jr	c, loc_DF9C
;28442: 		ex	de, hl
;28443: 		ld	hl, (MinDistance)
;28444: 		and	a
;28445: 		sbc	hl, de
;28446: 		jr	c, loc_DF9C
;28447: 		ld	(MinDistance), de
;28448: 		call	SavePlanetData
;28449: 
;28450: loc_DF9C:				; CODE XREF: SearchPlanetNearCursor+16j
;28451: 					; SearchPlanetNearCursor+1Fj
;28452: 		call	NextPlanetData
;28453: 		pop	bc
;28454: 		djnz	loc_DF7F
;28455: 
;28456: loc_DFA2:				; CODE XREF: CalcDistanceToCursor+1Fj
;28457: 		ld	hl, (MinDistance)
;28458: 		and	a
;28459: 		call	CalcFuelForDistance
;28460: 		call	RestorePlanetData
;28461: 		call	GenerateInfoPlanet
;28462: 		call	GeneratePlanetName
;28463: 		call	CopyPlanetName
;28464: 
;28465: CalcFuelForJump:			; CODE XREF: sub_DF24+37j
;28466: 		ld	a, (Pilot_CrdXSystem)
;28467: 		ld	b, a
;28468: 		ld	a, (Pilot_CrdYSystem)
;28469: 		call	CalcDistanceToCursor2
;28470: 		call	CalcFuelForDistance
;28471: 		and	a
;28472: 		ret
;28473: ; End of function SearchPlanetNearCursor
;28474: 
;28475: 
;28476: ; =============== S U B	R O U T	I N E =======================================
;28477: 
;28478: 
;28479: NextPlanetData:				; CODE XREF: SearchPlanetNearCursor:loc_DF9Cp
;28480: 					; Cmd_I_GlobalMap+6Bp ...
;28481: 		ld	b, 4
;28482: 
;28483: loc_DFC6:				; CODE XREF: NextPlanetData+7j
;28484: 		push	bc
;28485: 		call	ModifyPlanetData
;28486: 		pop	bc
;28487: 		djnz	loc_DFC6
;28488: 		ret
;28489: ; End of function NextPlanetData
;28490: 
;28491: 
;28492: ; =============== S U B	R O U T	I N E =======================================
;28493: 
;28494: 
;28495: CalcDistanceToCursor2:			; CODE XREF: CalcDistanceToCursor+Dp
;28496: 					; SearchPlanetNearCursor+48p
;28497: 		push	af
;28498: 		ld	a, (Pilot_CrdXTarget)
;28499: 		call	ABS_A_sub_B
;28500: 		ld	d, a
;28501: 		call	Mult_HL_D_A
;28502: 		pop	bc
;28503: 		push	hl
;28504: 		ld	a, (Pilot_CrdYTarget)
;28505: 
;28506: loc_DFDE:				; CODE XREF: sub_E6C6+Dj
;28507: 		call	ABS_A_sub_B
;28508: 		srl	a
;28509: 		ld	d, a
;28510: 		call	Mult_HL_D_A
;28511: 		pop	de
;28512: 		add	hl, de
;28513: 		ret
;28514: ; End of function CalcDistanceToCursor2
;28515: 
;28516: 
;28517: ; =============== S U B	R O U T	I N E =======================================
;28518: 
;28519: 
;28520: ABS_A_sub_B:				; CODE XREF: CalcDistanceToCursor2+4p
;28521: 					; CalcDistanceToCursor2:loc_DFDEp ...
;28522: 		sub	b
;28523: 		ret	nc
;28524: 		cpl
;28525: 		ret
;28526: ; End of function ABS_A_sub_B
;28527: 
;28528: 
;28529: ; =============== S U B	R O U T	I N E =======================================
;28530: 
;28531: 
;28532: sub_DFEE:
;28533: 		ld	a, (hl)
;28534: 		xor	2
;28535: 		ret
;28536: ; End of function sub_DFEE
;28537: 
;28538: 
;28539: ; =============== S U B	R O U T	I N E =======================================
;28540: 
;28541: 
;28542: CalcFuelForDistance:			; CODE XREF: SearchPlanetNearCursor+32p
;28543: 					; SearchPlanetNearCursor+4Bp
;28544: 		jr	c, loc_DFF6
;28545: 		jr	loc_DFF9
;28546: ; ---------------------------------------------------------------------------
;28547: 
;28548: loc_DFF6:				; CODE XREF: CalcFuelForDistancej
;28549: 		ld	hl, 0FFFFh
;28550: 
;28551: loc_DFF9:				; CODE XREF: CalcFuelForDistance+2j
;28552: 		ld	a, h
;28553: 		or	l
;28554: 		jr	z, loc_E005
;28555: 		call	j_Sqrt_H_HL
;28556: ; ---------------------------------------------------------------------------
;28557: 		ld	l, h
;28558: 		ld	h, 0
;28559: 		add	hl, hl
;28560: 		add	hl, hl
;28561: 
;28562: loc_E005:				; CODE XREF: CalcFuelForDistance+9j
;28563: 		ld	(FuelToPlanet),	hl
;28564: 		ret
;28565: ; End of function CalcFuelForDistance
;28566: 
;28567: 
;28568: ; =============== S U B	R O U T	I N E =======================================
;28569: 
;28570: 
;28571: Mult_HL_D_A:				; CODE XREF: CalcDistanceToCursor2+8p
;28572: 					; CalcDistanceToCursor2+16p ...
;28573: 		ld	e, 0
;28574: 		ld	b, 8
;28575: 		ld	hl, 0
;28576: 
;28577: loc_E010:				; CODE XREF: Mult_HL_D_A:loc_E019j
;28578: 		srl	d
;28579: 		rr	e
;28580: 		rl	a
;28581: 		jr	nc, loc_E019
;28582: 		add	hl, de
;28583: 
;28584: loc_E019:				; CODE XREF: Mult_HL_D_A+Dj
;28585: 		djnz	loc_E010
;28586: 		ret
;28587: ; End of function Mult_HL_D_A
;28588: 
;28589: 
;28590: ; =============== S U B	R O U T	I N E =======================================
;28591: 
;28592: 
;28593: GeneratePlanetName:			; CODE XREF: sub_D857+Bp sub_DF24+Ap ...
;28594: 		ld	hl, TempName	; "LAVE"
;28595: 
;28596: loc_E01F:
;28597: 		ld	(word_DF01), hl
;28598: 		ld	b, 8
;28599: 
;28600: loc_E024:				; CODE XREF: GeneratePlanetName+Bj
