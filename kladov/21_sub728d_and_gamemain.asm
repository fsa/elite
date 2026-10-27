; 4675: ; =============== S U B	R O U T	I N E =======================================
; 4676: 
; 4677: ; Attributes: noreturn
; 4678: 
; 4679: sub_728D:				; CODE XREF: LaunchDocking+68p
; 4680: 
; 4681: ; FUNCTION CHUNK AT 739E SIZE 0000003A BYTES
; 4682: ; FUNCTION CHUNK AT 73E2 SIZE 00000044 BYTES
; 4683: ; FUNCTION CHUNK AT 7471 SIZE 00000031 BYTES
; 4684: ; FUNCTION CHUNK AT 74AA SIZE 0000001C BYTES
; 4685: ; FUNCTION CHUNK AT 74E4 SIZE 00000016 BYTES
; 4686: 
; 4687: 		ld	hl, byte_6065
; 4688: 		call	sub_841A
; 4689: 		nop
; 4690: 		nop
; 4691: 
; 4692: loc_7295:				; CODE XREF: GoMission3+12j
; 4693: 		call	sub_FC33
; 4694: 		call	PrnCocpit_Ifn
; 4695: 		ld	sp, 4120h
; 4696: 		nop
; 4697: 		jp	loc_739E
; 4698: ; End of function sub_728D
; 4699: 
; 4700: ; ---------------------------------------------------------------------------
; 4701: byte_72A2:	db 3			; DATA XREF: sub_728D:loc_73F7w
; 4702: 
; 4703: ; =============== S U B	R O U T	I N E =======================================
; 4704: 
; 4705: 
; 4706: ScanKeyInPause:				; CODE XREF: sub_728D+1F0p
; 4707: 
; 4708: ; FUNCTION CHUNK AT 733F SIZE 00000027 BYTES
; 4709: 
; 4710: 		ld	a, (ControlKey2)
; 4711: 		and	80h ; 'А'
; 4712: 		ret	z
; 4713: 		ld	hl, FlagsSetUp
; 4714: 
; 4715: loc_72AC:				; CODE XREF: ScanKeyInPause+Ej
; 4716: 					; DATA XREF: ScanKeyInPause:loc_7340o
; 4717: 		halt
; 4718: 		ld	a, (PressKeyPrnSymbol)
; 4719: 		inc	a
; 4720: 		jr	nz, loc_72AC
; 4721: 
; 4722: loc_72B3:				; CODE XREF: ScanKeyInPause+25j
; 4723: 		halt
; 4724: 		ld	hl, PressKeyPrnSymbol
; 4725: 		ld	a, (hl)
; 4726: 		inc	hl
; 4727: 		push	hl
; 4728: 		ld	hl, unk_72FB
; 4729: 		call	sub_72D5
; 4730: 		jr	c, loc_72CA
; 4731: 		call	sub_72D4
; 4732: 		jr	c, loc_72D0
; 4733: 		pop	hl
; 4734: 		jr	loc_72B3
; 4735: ; ---------------------------------------------------------------------------
; 4736: 
; 4737: loc_72CA:				; CODE XREF: ScanKeyInPause+1Dj
; 4738: 		ld	a, (hl)
; 4739: 		pop	hl
; 4740: 		xor	(hl)
; 4741: 		ld	(hl), a
; 4742: 		jr	loc_733F
; 4743: ; ---------------------------------------------------------------------------
; 4744: 
; 4745: loc_72D0:				; CODE XREF: ScanKeyInPause+22j
; 4746: 		ld	l, (hl)
; 4747: 		ex	(sp), hl
; 4748: 		ld	a, (hl)
; 4749: 		ret
; 4750: ; End of function ScanKeyInPause
; 4751: 
; 4752: 
; 4753: ; =============== S U B	R O U T	I N E =======================================
; 4754: 
; 4755: 
; 4756: sub_72D4:				; CODE XREF: ScanKeyInPause+1Fp
; 4757: 		inc	hl
; 4758: ; End of function sub_72D4
; 4759: 
; 4760: 
; 4761: ; =============== S U B	R O U T	I N E =======================================
; 4762: 
; 4763: 
; 4764: sub_72D5:				; CODE XREF: ScanKeyInPause+1Ap
; 4765: 		push	bc
; 4766: 		ld	c, a
; 4767: 		call	16DCh
; 4768: 		ld	a, c
; 4769: 		pop	bc
; 4770: 		ret
; 4771: ; End of function sub_72D5
; 4772: 
; 4773: ; ---------------------------------------------------------------------------
; 4774: 
; 4775: loc_72DD:				; CODE XREF: RAM:7315j
; 4776: 		call	sub_7A84
; 4777: 		nop
; 4778: 		jr	loc_7340
; 4779: ; ---------------------------------------------------------------------------
; 4780: 
; 4781: loc_72E3:				; CODE XREF: RAM:7317j
; 4782: 		ld	a, (byte_D08E)	; <>0 есть Cobra Mk4
; 4783: 		and	a
; 4784: 		jr	z, loc_733E
; 4785: 		ld	hl, (PilotFrontLaser)
; 4786: 		ld	a, h
; 4787: 		ld	d, l
; 4788: 		ld	hl, (PilotLeftLaser)
; 4789: 		ld	(PilotFrontLaser), hl
; 4790: 		ld	h, d
; 4791: 		ld	l, a
; 4792: 		ld	(PilotLeftLaser), hl
; 4793: 		jr	loc_7340
; 4794: ; ---------------------------------------------------------------------------
; 4795: unk_72FB:	db  42h	; B		; DATA XREF: ScanKeyInPause+17o
; 4796: 		db  10h
; 4797: 		db  44h	; D
; 4798: 		db    8
; 4799: 		db  46h	; F
; 4800: 		db  40h	; @
; 4801: 		db  52h	; R
; 4802: 		db    4
; 4803: 		db  59h	; Y
; 4804: 		db  20h
; 4805: 		db  53h	; S
; 4806: 		db  80h	; А
; 4807: 		db    0
; 4808: 		db  20h
; 4809: 		db  19h
; 4810: 		db  4Ch	; L
; 4811: 		db  15h
; 4812: 		db  41h	; A
; 4813: 		db  17h
; 4814: 		db  4Bh	; K
; 4815: 		db  28h	; (
; 4816: 		db  43h	; C
; 4817: 		db  39h	; 9
; 4818: 		db  51h	; Q
; 4819: 		db  1Ah
; 4820: 		db    0
; 4821: ; ---------------------------------------------------------------------------
; 4822: 		jr	loc_72DD
; 4823: ; ---------------------------------------------------------------------------
; 4824: 		jr	loc_72E3
; 4825: ; ---------------------------------------------------------------------------
; 4826: 		db 0AFh	; п
; 4827: 		db  3Eh	; >
; 4828: 		db    0
; 4829: 		db  2Eh	; .
; 4830: 		db    8
; 4831: 		db  77h	; w
; 4832: 		db  23h	; #
; 4833: 		db  77h	; w
; 4834: 		db  23h	; #
; 4835: 		db  77h	; w
; 4836: 		db  23h	; #
; 4837: 		db  77h	; w
; 4838: 		db  23h	; #
; 4839: 		db  77h	; w
; 4840: 		db 0C9h	; ╔
; 4841: 		db  57h	; W
; 4842: 		db 0E6h	; ц
; 4843: 		db    3
; 4844: 		db  3Ah	; :
; 4845: 		db 0A2h	; в
; 4846: 		db  72h	; r
; 4847: 		db  20h
; 4848: 		db    3
; 4849: 		db 0A7h	; з
; 4850: 		db  28h	; (
; 4851: 		db  0Dh
; 4852: 		db  3Ah	; :
; 4853: 		db 0A2h	; в
; 4854: 		db  72h	; r
; 4855: 		db 0AAh	; к
; 4856: 		db  18h
; 4857: 		db    6
; 4858: 		db  21h	; !
; 4859: 		db  0Ah
; 4860: 		db 0FDh	; ¤
; 4861: 		db 0CDh	; ═
; 4862: 		db 0EEh	; ю
; 4863: ; ---------------------------------------------------------------------------
; 4864: 
; 4865: loc_733E:				; CODE XREF: RAM:72E7j
; 4866: 		rst	18h
; 4867: ; START	OF FUNCTION CHUNK FOR ScanKeyInPause
; 4868: 
; 4869: loc_733F:				; CODE XREF: ScanKeyInPause+2Bj
; 4870: 		ld	(hl), a
; 4871: 
; 4872: loc_7340:				; CODE XREF: RAM:72E1j	RAM:72F9j
; 4873: 		ld	hl, loc_72AC
; 4874: 		push	hl
; 4875: 
; 4876: loc_7344:				; CODE XREF: InitSpeedAngle+15j
; 4877: 		ld	b, 6
; 4878: 		ld	e, 0
; 4879: 
; 4880: loc_7348:				; CODE XREF: ScanKeyInPause+C0j
; 4881: 		push	bc
; 4882: 		halt
; 4883: 		ld	c, 4
; 4884: 
; 4885: loc_734C:				; CODE XREF: ScanKeyInPause+BDj
; 4886: 		ld	a, e
; 4887: 		add	a, a
; 4888: 		add	a, 5
; 4889: 		ld	e, a
; 4890: 		ld	b, a
; 4891: 		ld	a, 110000b
; 4892: 		out	(0FEh),	a
; 4893: 
; 4894: loc_7356:				; CODE XREF: ScanKeyInPause:loc_7356j
; 4895: 		djnz	$
; 4896: 		ld	a, e
; 4897: 		ld	b, a
; 4898: 		xor	a
; 4899: 		out	(0FEh),	a
; 4900: 
; 4901: loc_735D:				; CODE XREF: ScanKeyInPause:loc_735Dj
; 4902: 		djnz	$
; 4903: 		dec	c
; 4904: 		jr	nz, loc_734C
; 4905: 		pop	bc
; 4906: 		djnz	loc_7348
; 4907: 		ret
; 4908: ; END OF FUNCTION CHUNK	FOR ScanKeyInPause
; 4909: ; ---------------------------------------------------------------------------
; 4910: 		ld	(word_7578), sp
; 4911: 		call	InitSpeedAngle2
; 4912: 		call	PrnCocpit_Ifn
; 4913: 
; 4914: ; =============== S U B	R O U T	I N E =======================================
; 4915: 
; 4916: ; определение возможности сбора	минеральной жизни
; 4917: 
; 4918: sub_7370:				; CODE XREF: CollisionWithObj+1CCAp
; 4919: 		ld	a, (PilotMineralLife) ;	<>0 в трюме есть минеральная жизнь
; 4920: 					; =#FF минеральная жизнь померла
; 4921: 		and	a
; 4922: 		ret	nz
; 4923: 		push	hl
; 4924: 		call	sub_83FE	; 83fe	 PlanetCrd
; 4925: 					; 83fe+2 SunCrd
; 4926: 		pop	hl
; 4927: 		ret	c
; 4928: 		cp	38h ; '8'
; 4929: 		ret	c
; 4930: 		push	hl
; 4931: 		call	TestMainDiagonal ; Определение главной диагонали
; 4932: 					; Вых: z=1 на главной диагонале
; 4933: 		pop	hl
; 4934: 		ret	nz
; 4935: 		ld	a, (cargoMinerals)
; 4936: 		cp	2
; 4937: 		ret	c
; 4938: 		ld	(PilotMineralLife), a ;	<>0 в трюме есть минеральная жизнь
; 4939: 					; =#FF минеральная жизнь померла
; 4940: 		ret
; 4941: ; End of function sub_7370
; 4942: 
; 4943: ; ---------------------------------------------------------------------------
; 4944: unk_738E:	db    3			; DATA XREF: SetCrdSunPlInFreeSpace+4o
; 4945: 					; начальные координаты "солнца" и "планеты" в межпланетном пространстве
; 4946: 		db    0
; 4947: 		db    3
; 4948: 		db  83h	; Г
; 4949: 		db    0
; 4950: 		db    3
; 4951: ; ---------------------------------------------------------------------------
; 4952: 
; 4953: loc_7394:				; CODE XREF: RAM:74A2j
; 4954: 		xor	a
; 4955: 		ld	(WhatOnScreen),	a
; 4956: 		call	j_ClsAndPrintRamk
; 4957: 		call	PrnCocpit_Ifn
; 4958: ; START	OF FUNCTION CHUNK FOR sub_728D
; 4959: 
; 4960: loc_739E:				; CODE XREF: sub_728D+12j
; 4961: 					; sub_728D+11Dj ...
; 4962: 		ld	hl, byte_6065
; 4963: 		set	7, (hl)
; 4964: 		call	sub_7461
; 4965: 		ld	a, (PressKeyPrnSymbol)
; 4966: 		inc	a
; 4967: 		jr	nz, loc_739E
; 4968: 		ld	hl, unk_7470
; 4969: 
; 4970: loc_73AF:				; CODE XREF: sub_728D+128j
; 4971: 		inc	(hl)
; 4972: 		ld	a, (hl)
; 4973: 		and	0Fh
; 4974: 		cp	0Ch
; 4975: 		jr	nc, loc_73AF
; 4976: 		ld	hl, byte_AE3C	; номера кораблей для интро
; 4977: 		ld	e, a
; 4978: 		ld	d, 0
; 4979: 		add	hl, de
; 4980: 		ld	c, (hl)
; 4981: 		call	SetShip4Intro
; 4982: 		nop
; 4983: 
; 4984: loc_73C3:				; CODE XREF: sub_728D+165j
; 4985: 		call	ClearScrBuffer
; 4986: 		call	sub_7430
; 4987: 		call	sub_9CAF
; 4988: 		call	CopyBuf2Screen
; 4989: 		ld	bc, 0FF1Fh
; 4990: 		ld	hl, byte_FC0F
; 4991: 		halt
; 4992: 		jr	loc_73E2
; 4993: ; END OF FUNCTION CHUNK	FOR sub_728D
; 4994: ; ---------------------------------------------------------------------------
; 4995: 		db    0
; 4996: 		db    0
; 4997: 		db    0
; 4998: 		db    0
; 4999: 		db    0
; 5000: 		db    0
; 5001: 		db    0
; 5002: 		db    0
; 5003: 		db    0
; 5004: 		db    0
; 5005: ; ---------------------------------------------------------------------------
; 5006: ; START	OF FUNCTION CHUNK FOR sub_728D
; 5007: 
; 5008: loc_73E2:				; CODE XREF: sub_728D+149j
; 5009: 		ld	a, 2
; 5010: 		bit	1, (hl)
; 5011: 		jr	nz, loc_73F7
; 5012: 		inc	a
; 5013: 		bit	3, (hl)
; 5014: 		jr	nz, loc_73F7
; 5015: 		ld	a, (PressKeyPrnSymbol)
; 5016: 		sub	20h ; ' '
; 5017: 		jr	nz, loc_73C3
; 5018: 		ld	b, a
; 5019: 		jr	loc_73FD
; 5020: ; ---------------------------------------------------------------------------
; 5021: 
; 5022: loc_73F7:				; CODE XREF: sub_728D+159j
; 5023: 					; sub_728D+15Ej
; 5024: 		ld	(byte_72A2), a
; 5025: 		and	3
; 5026: 		ld	b, a
; 5027: 
; 5028: loc_73FD:				; CODE XREF: sub_728D+168j
; 5029: 		ld	hl, FlagsSetUp
; 5030: 		ld	a, (hl)
; 5031: 		and	0FCh ; '№'
; 5032: 		or	b
; 5033: 		ld	(hl), a
; 5034: 		call	j_InitGame
; 5035: 		xor	a
; 5036: 		ld	(InFreeSpace), a
; 5037: 		ld	(IndDanger), a
; 5038: 		ld	(PilotMission1_1), a
; 5039: 		ld	(FuelScoopsOn),	a
; 5040: 		ld	(CloakingDeviceOn), a
; 5041: 		ld	(SunRadiusNova), a
; 5042: 		call	SaveCocpitInBuffer
; 5043: 		call	j_Cmd_L_PilotInfo
; 5044: 		call	InitSpeedAngle2
; 5045: 		jr	GameMain
; 5046: ; END OF FUNCTION CHUNK	FOR sub_728D
; 5047: ; ---------------------------------------------------------------------------
; 5048: 		call	sub_7447
; 5049: 		ld	hl, 0E01h
; 5050: 		ld	a, 6
; 5051: 		jr	loc_7438
; 5052: 
; 5053: ; =============== S U B	R O U T	I N E =======================================
; 5054: 
; 5055: 
; 5056: sub_7430:				; CODE XREF: sub_728D+139p
; 5057: 		call	sub_7447
; 5058: 		ld	hl, 0D04h
; 5059: 		ld	a, 7
; 5060: 
; 5061: loc_7438:				; CODE XREF: RAM:742Ej
; 5062: 		ld	(CrdXPrnSymbol), hl
; 5063: 		ld	hl, (word_D0D5)
; 5064: 		inc	hl
; 5065: 		inc	hl
; 5066: 		set	7, (hl)
; 5067: 		call	j_PrintMessage2
; 5068: 		jr	loc_7459
; 5069: ; End of function sub_7430
; 5070: 
; 5071: 
; 5072: ; =============== S U B	R O U T	I N E =======================================
; 5073: 
; 5074: 
; 5075: sub_7447:				; CODE XREF: RAM:7426p	sub_7430p
; 5076: 		ld	hl, 105h
; 5077: 		ld	(CrdXPrnSymbol), hl
; 5078: 		ld	a, 1Dh
; 5079: 		ld	hl, (word_D0D5)
; 5080: 		inc	hl
; 5081: 		inc	hl
; 5082: 		set	7, (hl)
; 5083: 		call	j_PrintMessage1_A
; 5084: 
; 5085: loc_7459:				; CODE XREF: sub_7430+15j
; 5086: 		ld	hl, (word_D0D5)
; 5087: 		inc	hl
; 5088: 		inc	hl
; 5089: 		res	7, (hl)
; 5090: 		ret
; 5091: ; End of function sub_7447
; 5092: 
; 5093: 
; 5094: ; =============== S U B	R O U T	I N E =======================================
; 5095: 
; 5096: 
; 5097: sub_7461:				; CODE XREF: sub_728D+116p
; 5098: 		ld	hl, 5820h
; 5099: 		call	loc_7467
; 5100: 
; 5101: loc_7467:
; 5102: 		ld	b, 0F0h	; 'Ё'
; 5103: 		ld	a, 7
; 5104: 
; 5105: loc_746B:				; CODE XREF: sub_7461+Cj
; 5106: 		ld	(hl), a
; 5107: 		inc	hl
; 5108: 		djnz	loc_746B
; 5109: 		ret
; 5110: ; End of function sub_7461
; 5111: 
; 5112: ; ---------------------------------------------------------------------------
; 5113: unk_7470:	db  22h	; "             ; DATA XREF: sub_728D+11Fo
; 5114: ; ---------------------------------------------------------------------------
; 5115: ; START	OF FUNCTION CHUNK FOR sub_728D
; 5116: 
; 5117: GameMain:				; CODE XREF: sub_728D+197j
; 5118: 					; sub_728D+210j ...
; 5119: 		ld	a, (BlockControl)
; 5120: 		and	a
; 5121: 		jr	nz, loc_747A
; 5122: 		call	CalcAngSpeed4Prn
; 5123: 
; 5124: loc_747A:				; CODE XREF: sub_728D+1E8j
; 5125: 		call	sub_80D3	; проверка нажатия (и соответствующий переход) клавиш 1, 2, 3, 4, I, O,	P, K, L, enter
; 5126: 		call	ScanKeyInPause
; 5127: 		jr	nc, loc_748C
; 5128: 		xor	a
; 5129: 		ld	(WhatOnScreen),	a
; 5130: 		call	PrnCocpit_Ifn
; 5131: 		jp	loc_739E
; 5132: ; ---------------------------------------------------------------------------
; 5133: 
; 5134: loc_748C:				; CODE XREF: sub_728D+1F3j
; 5135: 		call	j_GoJump
; 5136: 		ld	a, (IndDanger)
; 5137: 		and	a
; 5138: 		jr	nz, loc_74AA
; 5139: 		call	ControlKeyIfMapOn
; 5140: 		ld	a, (ControlKeyMove)
; 5141: 		bit	5, a
; 5142: 		jr	z, GameMain
; 5143: 		call	MenuSaveLoad
; 5144: ; END OF FUNCTION CHUNK	FOR sub_728D
; 5145: ; ---------------------------------------------------------------------------
; 5146: 		jp	nz, loc_7394
; 5147: 		call	j_Cmd_L_PilotInfo
; 5148: 		jr	GameMain
; 5149: ; ---------------------------------------------------------------------------
; 5150: ; START	OF FUNCTION CHUNK FOR sub_728D
; 5151: 
; 5152: loc_74AA:				; CODE XREF: sub_728D+206j
; 5153: 		ld	a, (InFreeSpace)
; 5154: 		and	a
; 5155: 		jr	nz, loc_74B6
; 5156: 		call	GoDocking
; 5157: ; ---------------------------------------------------------------------------
; 5158: 		call	CalcCrdCoriolis2
; 5159: 
; 5160: loc_74B6:				; CODE XREF: sub_728D+221j
; 5161: 		call	Use_J_Drive
; 5162: 		call	CalcCrdSunPlanet
; 5163: 		ld	a, (WhatOnScreen)
; 5164: 		cp	4
; 5165: 		jr	c, loc_74E4
; 5166: 		call	j_j_MoveAllObjects
; 5167: ; END OF FUNCTION CHUNK	FOR sub_728D
; 5168: 
; 5169: ; =============== S U B	R O U T	I N E =======================================
; 5170: 
; 5171: 
; 5172: sub_74C6:
; 5173: 
; 5174: ; FUNCTION CHUNK AT 750F SIZE 00000003 BYTES
; 5175: 
; 5176: 		ld	a, (IndDanger)
; 5177: 		and	a
; 5178: 		jr	z, loc_74D2
; 5179: 		call	SetIndStation
; 5180: 		call	Damage2Pilot
; 5181: 
; 5182: loc_74D2:				; CODE XREF: sub_74C6+4j
; 5183: 		call	CorrData4Nova
; 5184: 		call	j_PrnTxtOnViewScr
; 5185: 		ld	b, 8
; 5186: 
; 5187: loc_74DA:				; CODE XREF: sub_74C6+1Aj
; 5188: 		push	bc
; 5189: 		call	ControlKeyIfMapOn
; 5190: 		halt
; 5191: 		pop	bc
; 5192: 		djnz	loc_74DA
; 5193: 		jr	loc_750F
; 5194: ; End of function sub_74C6
; 5195: 
; 5196: ; ---------------------------------------------------------------------------
; 5197: ; START	OF FUNCTION CHUNK FOR sub_728D
; 5198: 
; 5199: loc_74E4:				; CODE XREF: sub_728D+234j
; 5200: 		call	PrnCocpit_Ifn
; 5201: 		call	ClearScrBuffer
; 5202: 		ld	a, (InFreeSpace)
; 5203: 		and	a
; 5204: 		call	z, PrnSunPlanet
; 5205: 		call	PrintStars
; 5206: 
; 5207: loc_74F4:				; CODE XREF: sub_7A9E+24j
; 5208: 		call	j_PrnTxtOnViewScr
; 5209: 		call	j_j_MoveAllObjects
; 5210: ; END OF FUNCTION CHUNK	FOR sub_728D
; 5211: ; ---------------------------------------------------------------------------
; 5212: 		call	sub_9296
; 5213: ; ---------------------------------------------------------------------------
; 5214: 		jr	loc_7502
; 5215: ; ---------------------------------------------------------------------------
; 5216: 		db    0
; 5217: 		db  18h
; 5218: 		db 0DDh	; ▌
; 5219: ; ---------------------------------------------------------------------------
; 5220: 
; 5221: loc_7502:				; CODE XREF: RAM:74FDj
; 5222: 		call	sub_ED60
; 5223: 		call	UpdateAllIndicators
; 5224: 		ei
; 5225: 		call	Use_Y_Cloacking
; 5226: 		call	Process_MissBombEPod ; Печать индикаторов, запуск ракеты, бомбы
; 5227: ; START	OF FUNCTION CHUNK FOR sub_74C6
; 5228: 
; 5229: loc_750F:				; CODE XREF: sub_74C6+1Cj
; 5230: 		call	PrintCompass
; 5231: ; END OF FUNCTION CHUNK	FOR sub_74C6
; 5232: ; ---------------------------------------------------------------------------
; 5233: 		ld	a, (PlayerGameOver)
; 5234: 		and	a
; 5235: 		jp	z, GameMain
; 5236: 		xor	a
; 5237: 		ld	(HighDeltaSpeed), a
; 5238: 		ld	(TypeOfPrint), a
; 5239: 		inc	a
; 5240: 		ld	(BlockControl),	a
; 5241: 		ld	l, 7
; 5242: 		call	j_SetAtrViewScr
; 5243: 		call	CopyScaner2Scr
; 5244: 		call	SetShipwreck
; 5245: ; ---------------------------------------------------------------------------
; 5246: 		call	sub_9548
; 5247: 		call	FX_Launch
; 5248: 		call	PrnCocpit_Ifn
; 5249: 		ld	b, 1Eh
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
