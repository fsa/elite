;19355: ; End of function sub_B743
;19356: 
;19357: ; ---------------------------------------------------------------------------
;19358: ; START	OF FUNCTION CHUNK FOR FireLaser
;19359: 
;19360: PrnLaser:				; CODE XREF: FireLaser+79j
;19361: 					; FireLaser+9Bj ...
;19362: 		call	IntRnd_A
;19363: 		ld	b, a
;19364: 		and	0Ch
;19365: 		ld	c, a
;19366: 		ld	(byte_B4D6), a
;19367: 		ld	a, b
;19368: 		and	3
;19369: 		or	1
;19370: 		sub	1
;19371: 		ld	d, a
;19372: 		rlc	b
;19373: 		rlc	b
;19374: 		ld	a, b
;19375: 		and	3
;19376: 		or	1
;19377: 		sub	1
;19378: 		ld	e, a
;19379: 		push	de
;19380: 		ld	a, c
;19381: 		or	3
;19382: 		ld	c, a
;19383: 		ld	hl, 543Fh
;19384: 		push	hl
;19385: 		call	PrnLineInBuf_clip
;19386: 		pop	hl
;19387: 		pop	de
;19388: 		ld	a, (byte_B4D6)
;19389: 		or	1
;19390: 		ld	c, a
;19391: 		push	de
;19392: 		call	PrnLineInBuf_clip
;19393: 		pop	de
;19394: 		ld	hl, 403Fh
;19395: 		ld	a, (byte_B4D6)
;19396: 		or	3
;19397: 		ld	c, a
;19398: 		push	hl
;19399: 		push	de
;19400: 		call	PrnLineInBuf_clip
;19401: 		pop	de
;19402: 		pop	hl
;19403: 		ld	a, (byte_B4D6)
;19404: 		or	1
;19405: 		ld	c, a
;19406: 		jp	PrnLineInBuf_clip
;19407: ; END OF FUNCTION CHUNK	FOR FireLaser
;19408: 
;19409: ; =============== S U B	R O U T	I N E =======================================
;19410: 
;19411: 
;19412: IntRnd_A:				; CODE XREF: sub_78B1+10Ap
;19413: 					; sub_78B1+10Fp ...
;19414: 		push	hl
;19415: 		push	bc
;19416: 		ld	hl, (word_B7AA)
;19417: 		ld	a, h
;19418: 		adc	a, a
;19419: 		ld	b, a
;19420: 		adc	a, l
;19421: 		ld	h, a
;19422: 		ld	l, b
;19423: 		ld	(word_B7AA), hl
;19424: 		pop	bc
;19425: 		pop	hl
;19426: 		ret
;19427: ; End of function IntRnd_A
;19428: 
;19429: ; ---------------------------------------------------------------------------
;19430: word_B7AA:	dw 0A26Ch		; DATA XREF: IntRnd_A+2r IntRnd_A+Bw
;19431: 
;19432: ; =============== S U B	R O U T	I N E =======================================
;19433: 
;19434: 
;19435: Process_LaserECM2:			; CODE XREF: j_MoveAllObjects+1B03j
;19436: 		xor	a
;19437: 		ld	(TableBasePrice+28h), a	; +#28 Fireams
;19438: 		ld	(TableBasePrice+29h), a	; +#28 Fireams
;19439: 		ret
;19440: ; End of function Process_LaserECM2
;19441: 
;19442: ; ---------------------------------------------------------------------------
;19443: ; START	OF FUNCTION CHUNK FOR j_MoveAllObjects
;19444: 
;19445: Processing_LaserECM:			; CODE XREF: RAM:9CB8j
;19446: 		ld	a, (PlayerGameOver)
;19447: 		and	a
;19448: 		jr	nz, Process_LaserECM2
;19449: 		ld	a, (WhatOnScreen)
;19450: 		cp	4
;19451: 		ret	nc
;19452: 		ld	hl, PilotFrontLaser
;19453: 		ld	d, 0
;19454: 		ld	e, a
;19455: 		add	hl, de
;19456: 		ld	a, (hl)
;19457: 		and	a
;19458: 		jr	z, sub_B7F4	; ;проверка 3й миссии
;19459: 		ld	a, (unk_B138)	; тип печати OR/XOR
;19460: 		ld	(loc_B7DB), a
;19461: 		ld	(loc_B7EA), a
;19462: 		ld	hl, unk_C7EE
;19463: 		ld	b, 2
;19464: 
;19465: loc_B7D9:				; CODE XREF: RAM:B7E0j
;19466: 		ld	a, 0FEh	; '■'
;19467: ; END OF FUNCTION CHUNK	FOR j_MoveAllObjects
;19468: 
;19469: loc_B7DB:				; DATA XREF: j_MoveAllObjects+1B19w
;19470: 		or	(hl)
;19471: 		ld	(hl), a
;19472: 		ld	l, 0F1h	; 'ё'
;19473: 		ld	c, b
;19474: 		djnz	loc_B7D9
;19475: 		ld	de, unk_CF2F
;19476: 		ld	l, d
;19477: 		ld	b, 0Eh
;19478: 
;19479: loc_B7E8:				; CODE XREF: RAM:B7EDj
;19480: 		ld	a, c
;19481: 		dec	h
;19482: 
;19483: loc_B7EA:				; DATA XREF: j_MoveAllObjects+1B1Cw
;19484: 		or	(hl)
;19485: 		ld	(hl), a
;19486: 		ex	de, hl
;19487: 		djnz	loc_B7E8
;19488: 		ld	b, 0
;19489: 		call	FireLaser
;19490: 
;19491: ; =============== S U B	R O U T	I N E =======================================
;19492: 
;19493: ; ;проверка 3й миссии
;19494: 
;19495: sub_B7F4:				; CODE XREF: j_MoveAllObjects+1B14j
;19496: 		ld	a, (PilotExtWeapon) ; 7,=1 - ECM Jammer
;19497: 					; 6,=1 - Cloaking device
;19498: 					; 5,=1 - нельзя	купить товары(в	трюме беженцы или десант)
;19499: 					; 4
;19500: 					; 3 =1 3я миссия была выполнена
;19501: 					; 2 =1 - третья	миссия выполнена
;19502: 					; 1 =1 - второя	миссия выполнена
;19503: 					; 0 =1 - первая	миссия выполнена
;19504: 		bit	7, a
;19505: 		jr	z, loc_B801	; ;счётчик ECM
;19506: 		ld	a, (PilotMission3) ; =#06 при стыковке будет захват таргонами (вроде нереальная	ситуация)
;19507: 					; =#04 в системе со танцией таргонов во	время войны
;19508: 		cp	4
;19509: 		ret	z
;19510: 
;19511: loc_B801:				; CODE XREF: sub_B7F4+5j
;19512: 		ld	hl, ControlKey3	; ;счётчик ECM
;19513: 		ld	b, (hl)
;19514: 		res	4, (hl)
;19515: 		ld	a, (PilotECMSystem)
;19516: 		ld	c, a
;19517: 		ld	a, (TimeWorkECM)
;19518: 		and	a
;19519: 		jr	z, ECM_On
;19520: 		dec	a
;19521: 		ld	(TimeWorkECM), a
;19522: 		jr	nz, loc_B819
;19523: 		res	7, c
;19524: 
;19525: loc_B819:				; CODE XREF: sub_B7F4+21j
;19526: 		ld	hl, nBeepFX
;19527: 		set	5, (hl)
;19528: 		ld	a, c
;19529: 		and	a
;19530: 		ret	z
;19531: 		ld	(PilotECMSystem), a
;19532: 		ld	a, (DamageEnergyUnit)
;19533: 		inc	a
;19534: 		ld	(DamageEnergyUnit), a
;19535: 		ret
;19536: ; ---------------------------------------------------------------------------
;19537: 
;19538: ECM_On:					; CODE XREF: sub_B7F4+1Bj
;19539: 		ld	a, c
;19540: 		and	a
;19541: 		ret	z
;19542: 		bit	4, b
;19543: 		ret	z
;19544: 		or	80h ; 'А'
;19545: 		ld	(PilotECMSystem), a
;19546: 		ld	a, 0Ah
;19547: 		ld	(TimeWorkECM), a
;19548: 		ret
;19549: ; End of function sub_B7F4
;19550: 
;19551: ; ---------------------------------------------------------------------------
;19552: 		db    0
;19553: PatternObject:	db 0, 32h, 0		; DATA XREF: SetObjInSlot+20o
;19554: 		db 0, 32h, 0
;19555: 		db 0, 32h, 0
;19556: 		db 0, 0, 0, 0, 0FFh, 0FFh
;19557: 		db 0, 0, 0FFh, 7Fh, 0, 0
;19558: 		db 0FFh, 0FFh, 0, 0, 0,	0
;19559: 		db 0
;19560: 		db 0
;19561: 		db 0
;19562: 		db 0
;19563: SKO4Intro:	db  58h	; X		; DATA XREF: SetShip4Intro+3Bo
;19564: 		db  44h	; D
;19565: 		db  41h	; A
;19566: 		db  98h	; Ш
;19567: 		db  64h	; d
;19568: 		db 0CBh	; ╦
;19569: 		db  5Ah	; Z
;19570: 		db 0A7h	; з
;19571: 		db 0D1h	; ╤
;19572: byte_B866:	db 0			; DATA XREF: RotateSKOobjr
;19573: 					; RotateSKOobj+5w
;19574: 
;19575: ; =============== S U B	R O U T	I N E =======================================
;19576: 
;19577: ; Attributes: noreturn thunk
;19578: 
;19579: sub_B867:				; CODE XREF: SetObjbyShip+2j
;19580: 					; SetObjbyShip+5p ...
;19581: 		jp	sub_DA51
;19582: ; End of function sub_B867
;19583: 
;19584: 
;19585: ; =============== S U B	R O U T	I N E =======================================
;19586: 
;19587: ; Attributes: noreturn
;19588: 
;19589: CallFunctionObj_1:			; CODE XREF: sub_DA51+Bp
;19590: 
;19591: ; FUNCTION CHUNK AT B96B SIZE 000000BC BYTES
;19592: ; FUNCTION CHUNK AT BA94 SIZE 00000008 BYTES
;19593: ; FUNCTION CHUNK AT BAE8 SIZE 0000006D BYTES
;19594: ; FUNCTION CHUNK AT BBBA SIZE 00000038 BYTES
;19595: ; FUNCTION CHUNK AT BBFD SIZE 000000CB BYTES
;19596: 
;19597: 		srl	c
;19598: 		srl	c
;19599: 		srl	c
;19600: 		and	0Fh
;19601: 		jp	z, SetRndShip
;19602: 		dec	a
;19603: 		jp	z, Set3_4Targons
;19604: 		dec	a
;19605: 		jp	z, SetObjMissile
;19606: 		dec	a
;19607: 		ret	z
;19608: 		dec	a
;19609: 		jp	z, ShipSetShip
;19610: 		dec	a
;19611: 		jp	z, Set2Alloys
;19612: 		dec	a
;19613: 		jp	z, SetCargo
;19614: 		dec	a
;19615: 		jp	z, SetCobra_EscPod
;19616: 		dec	a
;19617: 		jp	z, Set2Minerals
;19618: 		dec	a
;19619: 		jp	z, SetShip4Intro
;19620: 		ret
;19621: ; End of function CallFunctionObj_1
;19622: 
;19623: 
;19624: ; =============== S U B	R O U T	I N E =======================================
;19625: 
;19626: ; a - номер объекта для	установки
;19627: 
;19628: SetObjInSlot:				; CODE XREF: sub_5EC5p
;19629: 					; ReCalcCrdObj_coriolis+86p ...
;19630: 		call	sub_EF01	; a - номер объекта
;19631: 		ld	a, b
;19632: 
;19633: loc_B89C:				; CODE XREF: SetObjInSlot_Intro+4j
;19634: 		and	3Fh ; '?'
;19635: 		add	a, a
;19636: 		add	a, a
;19637: 		add	a, l
;19638: 		ld	l, a
;19639: 		jr	nc, loc_B8A5
;19640: 		inc	h
;19641: 
;19642: loc_B8A5:				; CODE XREF: SetObjInSlot+Aj
;19643: 		inc	hl
;19644: 		ld	e, (hl)
;19645: 		inc	hl
;19646: 		ld	d, (hl)
;19647: 		push	de
;19648: 		pop	iy
;19649: 		ld	(ix+24h), d
;19650: 		ld	(ix+23h), e
;19651: 		push	ix
;19652: 		pop	de
;19653: 		ld	bc, 1Fh
;19654: 		ld	hl, PatternObject
;19655: 		ldir
;19656: 		ld	a, (iy+7)
;19657: 		ld	(ix+22h), a
;19658: 		ld	a, (iy+12h)
;19659: 		ld	c, a
;19660: 		and	6Fh ; 'o'
;19661: 		ld	(ix+21h), a
;19662: 		call	sub_D60B
;19663: 		and	7
;19664: 		rla
;19665: 		rl	c
;19666: 		rra
;19667: 		ld	(ix+20h), a
;19668: 		ld	a, (iy+6)
;19669: 		srl	a
;19670: 		ld	(ix+1Bh), a
;19671: 		ld	a, 7Eh ; '~'
;19672: 		ld	(ix+25h), a
;19673: 		xor	a
;19674: 		ld	(ix+26h), a
;19675: 		set	6, (ix+20h)
;19676: 		bit	2, (ix+21h)
;19677: 		jr	nz, loc_B903
;19678: 		call	IntRnd_A
;19679: 		and	7
;19680: 		ld	(ix+1),	a
;19681: 		call	IntRnd_A
;19682: 		and	7
;19683: 		ld	(ix+4),	a
;19684: 
;19685: loc_B903:				; CODE XREF: SetObjInSlot+59j
;19686: 		call	IntRnd_A
;19687: 		ld	(ix+0),	a
;19688: 		call	IntRnd_A
;19689: 		ld	(ix+3),	a
;19690: 		call	IntRnd_A
;19691: 		rra
;19692: 		rr	(ix+2)
;19693: 		rra
;19694: 		rr	(ix+5)
;19695: 		xor	a
;19696: 		ret
;19697: ; End of function SetObjInSlot
;19698: 
;19699: 
;19700: ; =============== S U B	R O U T	I N E =======================================
;19701: 
;19702: ; Attributes: noreturn
;19703: 
;19704: ShipSetObject:				; CODE XREF: CallFunctionObj_1+167p
;19705: 					; CallFunctionObj_1+1A5p ...
;19706: 		ld	c, a
;19707: 		push	ix
;19708: 		call	sub_FACA
;19709: ; ---------------------------------------------------------------------------
;19710: 		pop	hl
