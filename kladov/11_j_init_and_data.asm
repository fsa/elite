;24780: 		jp	loc_DF03
;24781: 
;24782: ; =============== S U B	R O U T	I N E =======================================
;24783: 
;24784: ; Attributes: thunk
;24785: 
;24786: j_InitGame:				; CODE XREF: sub_728D+178p
;24787: 		jp	InitGame
;24788: ; End of function j_InitGame
;24789: 
;24790: ; [00000003 BYTES: COLLAPSED FUNCTION j_ClsAndPrintRamk. PRESS KEYPAD "+" TO EXPAND]
;24791: ; [00000003 BYTES: COLLAPSED FUNCTION j_PrintMessage2. PRESS KEYPAD "+" TO EXPAND]
;24792: 
;24793: ; =============== S U B	R O U T	I N E =======================================
;24794: 
;24795: ; Attributes: thunk
;24796: 
;24797: j_PrnTxtOnViewScr:			; CODE XREF: sub_74C6+Fp
;24798: 					; sub_728D:loc_74F4p ...
;24799: 		jp	PrnTxtOnViewScr
;24800: ; End of function j_PrnTxtOnViewScr
;24801: 
;24802: ; ---------------------------------------------------------------------------
;24803: JumpDocking:	db 0			; DATA XREF: GoDocking+28r
;24804: 					; ControlKeyIfMapOn+9r	...
;24805: 					; =1 межпланетный прыжок/стыковка
;24806: 					; =2 межгалактический прыжок
;24807: FlgJumpDocking:	db 0			; DATA XREF: sub_80D3r	sub_9296r ...
;24808: 					; =1 запрет на обработку нажатий клавиш
;24809: 					;   включено при: стыковке
;24810: 					;   работает п/п отрисовки концентрический кругов
;24811: 					;
;24812: 					; 0,Flg_Interrupts
;24813: CloakingDeviceOn:db 0			; DATA XREF: sub_70A9:SetAtrViewScrr
;24814: 					; Use_Y_Cloacking+19o ...
;24815: 
;24816: ; =============== S U B	R O U T	I N E =======================================
;24817: 
;24818: ; Attributes: thunk
;24819: 
;24820: j_GoJump:				; CODE XREF: sub_728D:loc_748Cp
;24821: 		jp	GoJump
;24822: ; End of function j_GoJump
;24823: 
;24824: ; ---------------------------------------------------------------------------
;24825: CrdXPrnSymbol:	dw 200h			; DATA XREF: sub_7430:loc_7438w
;24826: 					; sub_7447+3w ...
;24827: Missile1:	db 0			; DATA XREF: sub_9946+7w sub_99B5+31r	...
;24828: 
;24829: ; =============== S U B	R O U T	I N E =======================================
;24830: 
;24831: ; Attributes: thunk
;24832: 
;24833: j_PrintMessage1_A:			; CODE XREF: RAM:7139j	sub_7447+Fp ...
;24834: 		jp	PrintMessage1_A
;24835: ; End of function j_PrintMessage1_A
;24836: 
;24837: ; ---------------------------------------------------------------------------
;24838: word_D04B:	dw 0D548h		; DATA XREF: sub_99B5:loc_99F8r
;24839: 					; sub_99B5+80r	...
;24840: 		db    0
;24841: 
;24842: ; =============== S U B	R O U T	I N E =======================================
;24843: 
;24844: ; Attributes: noreturn thunk
;24845: 
;24846: MenuSaveLoad:				; CODE XREF: sub_728D+212p
;24847: 		jp	MenuSaveLoad2
;24848: ; End of function MenuSaveLoad
;24849: 
;24850: ; ---------------------------------------------------------------------------
;24851: PrnCrdXCursor:	db 6Dh			; DATA XREF: RAM:8E2Br
;24852: 					; PrintSmallCursOnMap:loc_E496w ...
;24853: PrnCrdYCursor:	db 7Eh			; DATA XREF: RAM:8E3Dr
;24854: 					; PrintSmallCursOnMap+4Dw ...
;24855: RadiusCircle:	db 2Ah			; DATA XREF: RAM:8E51r
;24856: 					; Cmd_I_GlobalMap+2Cw ...
;24857: PilotName:	.ascii 'JAMESON   ',0   ; DATA XREF: InitGame+6o sub_D822+Co ...
;24858: PilotLegalStatus:db 0			; DATA XREF: ReCalcCrdObj_coriolis+24r
;24859: 					; UseEscapePod+29o ...
;24860: PilotRating0:	db 0			; DATA XREF: DirectObject+6r
;24861: 					; IncBattleRating2o
;24862: PilotRating1:	db 0			; DATA XREF: sub_D60B+6r
;24863: PilotRating2:	db 0			; DATA XREF: sub_5E78:loc_5E9Cr
;24864: 					; sub_5E78+37w	...
;24865: 					; >=#84	в трюме	десант
;24866: 					; 7,=1 война была или идёт
;24867: 					; 6,=1 война закончилать
;24868: 					; 3-1 -	ранг (=текущая стадия войны)
;24869: JumpCounter:	dw 0
;24870: PilotNumGalaxy:	db 0			; DATA XREF: ReCalcCrdObj_coriolis+19r
;24871: 					; TestPilotInZoneWar:loc_D67Dr	...
;24872: PilotCash2low:	db 0			; DATA XREF: sub_D32Dr
;24873: 					; IncCASH:loc_D3E7r ...
;24874: PilotCash2hi:	db 0			; DATA XREF: sub_5FA1+6r
;24875: 					; sub_D1E9:loc_D21Fo
;24876: PilotCash1:	dw 3E8h			; DATA XREF: sub_D32D+3r IncCASHr ...
;24877: PilotRnd:	db 0			; DATA XREF: sub_9B83+9r
;24878: 					; GenerateProductPrice+3w ...
;24879: PilotCargo:	db 0			; DATA XREF: sub_E6D6+8o
;24880: 		db 0
;24881: 		db 0
;24882: cargoSlaves:	db 0			; DATA XREF: CallFunctionObj_1:SetRndShipr
;24883: 		db 0
;24884: 		db 0
;24885: cargoNarc:	db 0			; DATA XREF: CallFunctionObj_1+354r
;24886: 		db 0
;24887: 		db 0
;24888: 		db 0
;24889: cargoFireams:	db 0			; DATA XREF: CallFunctionObj_1+35Ar
;24890: 		db 0
;24891: cargoMinerals:	db 0			; DATA XREF: InitSpeedAngle+7o
;24892: 					; sub_7370+14r	...
;24893: 		db 0
;24894: 		db 0
;24895: cargoGemStones:	db 0			; DATA XREF: sub_E87E+46w
;24896: cargoAlienItems:db 0			; DATA XREF: sub_5EC5+2Er
;24897: PilotFreeCargo:	db 14h			; DATA XREF: RAM:80CDw
;24898: 					; CollisionWithObj+5Cr	...
;24899: PilotFrontLaser:db 1			; DATA XREF: RAM:72E9r	RAM:72F1w ...
;24900: 		db 0
