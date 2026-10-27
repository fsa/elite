;12090: 		db  38h	; 8
;12091: 		db    0
;12092: 		db    0
;12093: 
;12094: ; =============== S U B	R O U T	I N E =======================================
;12095: 
;12096: ; Attributes: noreturn
;12097: 
;12098: LaunchDocking:				; CODE XREF: GoDocking+42j
;12099: 					; sub_80D3+3Cp
;12100: 
;12101: ; FUNCTION CHUNK AT 5E55 SIZE 00000013 BYTES
;12102: 
;12103: 		ld	a, (InFreeSpace)
;12104: 		and	a
;12105: 		ret	nz
;12106: 		ld	hl, unk_93E3
;12107: 		ld	(loc_92F9+1), hl
;12108: 		ld	a, 8
;12109: 		ld	(byte_93D0), a
;12110: 		ld	a, 30h ; '0'
;12111: 		ld	(loc_9325+1), a
;12112: 		ld	a, 40h ; '@'
;12113: 		ld	(byte_93D2), a
;12114: 		ld	hl, 101h
;12115: 		ld	(JumpDocking), hl ; =1 межпланетный прыжок/стыковка
;12116: 					; =2 межгалактический прыжок
;12117: 		ld	a, (IndDanger)
;12118: 		and	a
;12119: 		jr	z, BeginLaunch
;12120: 
;12121: BeginDocking:
;12122: 		ld	a, 38h ; '8'
;12123: 		ld	(loc_932A+1), a
;12124: 		ld	a, 0Ch
;12125: 		ld	(HighDeltaSpeed), a
;12126: 		xor	a
;12127: 		ld	(HighAngleDU), a
;12128: 		ld	(HighAngleLR), a
;12129: 		ld	(TimeWorkECM), a
;12130: 		inc	a
;12131: 		ld	(BlockControl),	a
;12132: 		ld	hl, PilotECMSystem
;12133: 		res	7, (hl)
;12134: 		call	ClearScrBuffer
;12135: 		call	PrnDockingTonnel ; обратный отсчет до прыжка, круги
;12136: ; ---------------------------------------------------------------------------
;12137: 		ld	b, 2Fh ; '/'
;12138: 
;12139: loc_9438:				; CODE XREF: LaunchDocking+56j
;12140: 		push	bc
;12141: 		call	ClearScrBuffer
;12142: 		call	PrnDockingTonnel ; обратный отсчет до прыжка, круги
;12143: ; ---------------------------------------------------------------------------
;12144: 		call	CopyBuf2Screen
;12145: 		pop	bc
;12146: 		djnz	loc_9438
;12147: 		call	FX_Launch
;12148: 		xor	a
;12149: 		ld	(IndDanger), a
;12150: 		call	SaveCocpitInBuffer
;12151: 		call	InitSpeedAngle2
;12152: 		call	sub_5FA1	; предложение покупки кобры МК-4
;12153: ; ---------------------------------------------------------------------------
;12154: 		call	sub_728D	; захват пиратами/таргонами при	стыковке
;12155: ; ---------------------------------------------------------------------------
;12156: 		ld	a, 7		; Cmd_L_PilotInfo
;12157: 		jp	loc_5E55
;12158: ; ---------------------------------------------------------------------------
;12159: 
;12160: BeginLaunch:				; CODE XREF: LaunchDocking+24j
;12161: 		ld	a, 4
;12162: 		ld	(IndDanger), a
;12163: 		call	FX_Launch
;12164: 		xor	a
;12165: 		ld	(ManualDocking), a
;12166: 		ld	(WhatOnScreen),	a
;12167: 		ld	(J_Drive_Coriolis), a
;12168: 		call	sub_E6EE
;12169: 		ld	a, 32h ; '2'
;12170: 		ld	(loc_932A+1), a
;12171: 		call	PrnCocpit_Ifn
;12172: 		call	sub_9946
;12173: 		call	sub_726C
;12174: 		ld	hl, PlanetCrd
;12175: 		ld	b, 7
;12176: 		call	ClearMemory
;12177: 		ld	(hl), 80h ; 'А'
;12178: 		inc	hl
;12179: 		ld	(hl), 1
;12180: 		ld	a, (SunCrdZhi)
;12181: 		xor	80h ; 'А'
;12182: 		ld	(SunCrdZhi), a
;12183: 		ld	hl, objCoriolis
;12184: 		ld	b, 1Bh
;12185: 		call	ClearMemory
;12186: 		ld	ix, objCoriolis
;12187: 		ld	a, 0F0h	; 'Ё'
;12188: 		ld	(ix+6),	a
;12189: 		ld	a, 80h ; 'А'
;12190: 		ld	(ix+8),	a
;12191: 		dec	a
;12192: 		ld	(ix+0Eh), a
;12193: 		ld	(ix+12h), a
;12194: 		ld	(ix+16h), a
;12195: 		ld	a, 0F0h	; 'Ё'
;12196: 		ld	(ix+22h), a
;12197: 		call	ClearAroundObj
;12198: 		call	UpdateAllIndicators
;12199: 		call	ClearScrBuffer
;12200: 		call	PrnDockingTonnel ; обратный отсчет до прыжка, круги
;12201: ; ---------------------------------------------------------------------------
;12202: 		ld	b, 24h ; '$'
;12203: 
;12204: loc_94C8:				; CODE XREF: LaunchDocking+E9j
;12205: 		push	bc
;12206: 		call	ClearScrBuffer
;12207: 		call	PrintStars
;12208: 		call	PrnDockingTonnel ; обратный отсчет до прыжка, круги
;12209: ; ---------------------------------------------------------------------------
;12210: 		call	CopyBuf2Screen
;12211: 		pop	bc
;12212: 		djnz	loc_94C8
;12213: 		ld	b, 0Bh
;12214: 
;12215: loc_94DA:				; CODE XREF: LaunchDocking+101j
;12216: 		push	bc
;12217: 		call	ClearScrBuffer
;12218: 		call	CalcCrdSunPlanet
;12219: 		call	PrintStars
;12220: 		call	PrnDockingTonnel ; обратный отсчет до прыжка, круги
;12221: ; ---------------------------------------------------------------------------
;12222: 		call	PrnSunPlanet
;12223: 		call	CopyBuf2Screen
;12224: 		pop	bc
;12225: 		djnz	loc_94DA
;12226: 		xor	a
;12227: 		ld	(BlockControl),	a
;12228: ; End of function LaunchDocking
;12229: 
;12230: 
;12231: ; =============== S U B	R O U T	I N E =======================================
;12232: 
;12233: 
;12234: InitSpeedAngle2:			; CODE XREF: InitSpeedAnglep RAM:736Ap ...
;12235: 		ld	b, 5
;12236: 		call	sub_F9AB
;12237: 		call	ClearMemory
;12238: 		ld	(hl), 0Ch
;12239: 
;12240: loc_94FE:				; CODE XREF: PrnCocpit_Ifn+6p
;12241: 					; sub_813E+1Cj
;12242: 		ld	b, 5
;12243: 		ld	hl, ControlKey1
;12244: ; End of function InitSpeedAngle2
;12245: 
;12246: 
;12247: ; =============== S U B	R O U T	I N E =======================================
;12248: 
;12249: 
;12250: ClearMemory:				; CODE XREF: LaunchDocking+98p
;12251: 					; LaunchDocking+ADp ...
;12252: 		xor	a
;12253: 
;12254: loc_9504:				; CODE XREF: ClearMemory+3j
;12255: 		ld	(hl), a
;12256: 		inc	hl
;12257: 		djnz	loc_9504
;12258: 		ret
;12259: ; End of function ClearMemory
;12260: 
;12261: 
;12262: ; =============== S U B	R O U T	I N E =======================================
;12263: 
;12264: 
;12265: ClearAroundObj:				; CODE XREF: RAM:7566p	Use_J_Drive+29p ...
;12266: 		ld	ix, objCoriolis
;12267: 		res	2, (ix+21h)
;12268: 		xor	a
;12269: 		ld	(PlayerPirate),	a
;12270: 		ld	de, 27h	; '''
;12271: 		ld	b, 7
;12272: 
;12273: loc_951A:				; CODE XREF: ClearAroundObj+16j
;12274: 		add	ix, de
;12275: 		ld	(ix+22h), a
;12276: 		djnz	loc_951A
;12277: 		ret
;12278: ; End of function ClearAroundObj
;12279: 
;12280: ; ---------------------------------------------------------------------------
;12281: byte_9522:	db 0			; DATA XREF: sub_9548:loc_9551r
;12282: 					; sub_9579+6r ...
;12283: word_9523:	dw 8000h		; DATA XREF: sub_9579+30r sub_9579+47w ...
;12284: word_9525:	dw 0B27h		; DATA XREF: sub_9579:loc_95A6r
;12285: 					; sub_9579+4Bw	...
;12286: 
;12287: ; =============== S U B	R O U T	I N E =======================================
;12288: 
;12289: 
;12290: FX_Launch:				; CODE XREF: RAM:7532p
;12291: 					; LaunchDocking+58p ...
;12292: 		ld	a, (FlagsSetUp)
;12293: 		bit	7, a
;12294: 		ret	z
;12295: 		call	sub_969B
;12296: 		jr	loc_9551
;12297: ; End of function FX_Launch
;12298: 
;12299: ; ---------------------------------------------------------------------------
;12300: ; START	OF FUNCTION CHUNK FOR sub_9296
