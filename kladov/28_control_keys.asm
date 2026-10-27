; 4520: 
; 4521: ; =============== S U B	R O U T	I N E =======================================
; 4522: 
; 4523: 
; 4524: ControlKeyIfMapOn:			; CODE XREF: sub_728D+208p
; 4525: 					; sub_74C6+15p
; 4526: 		ld	a, (WhatOnScreen)
; 4527: 		cp	7
; 4528: 		ret	c
; 4529: 		cp	9
; 4530: 		ret	nc
; 4531: 		ld	a, (JumpDocking) ; =1 межпланетный прыжок/стыковка
; 4532: 					; =2 межгалактический прыжок
; 4533: 		and	a
; 4534: 		ret	nz
; 4535: 		ld	a, (ControlKeyMove)
; 4536: 		and	0Fh
; 4537: 		ld	c, a
; 4538: 		ld	a, (ControlKeyInMap)
; 4539: 		and	70h ; 'p'
; 4540: 		or	c
; 4541: 		ret	z
; 4542: 		ld	b, 8
; 4543: 
; 4544: loc_7210:				; CODE XREF: ControlKeyIfMapOn+20j
; 4545: 		inc	b
; 4546: 		srl	a
; 4547: 		jr	nc, loc_7210
; 4548: 		ld	a, b
; 4549: 		call	sub_813E
; 4550: 		xor	a
; 4551: 		ld	(ControlKeyInMap), a
; 4552: 		ld	(ControlKeyMove), a
; 4553: 		ret
; 4554: ; End of function ControlKeyIfMapOn
; 4555: 
; 4556: ; ---------------------------------------------------------------------------
; 4557: byte_7221:	db 2			; DATA XREF: Use_Y_Cloackingo
; 4558: 					; Use_Y_Cloacking+Cw
; 4559: 
; 4560: ; =============== S U B	R O U T	I N E =======================================
; 4561: 
; 4562: 
; 4563: Use_Y_Cloacking:			; CODE XREF: RAM:7509p
; 4564: 		ld	hl, byte_7221
; 4565: 		dec	(hl)
; 4566: 		ld	hl, ControlKey1
; 4567: 		res	7, (hl)
; 4568: 		ret	nz
; 4569: 		ld	a, 2
; 4570: 		ld	(byte_7221), a
; 4571: 		halt
; 4572: 		bit	7, (hl)
; 4573: 		ret	z
; 4574: 		ld	hl, PilotExtWeapon ; 7,=1 - ECM	Jammer
; 4575: 					; 6,=1 - Cloaking device
; 4576: 					; 5,=1 - нельзя	купить товары(в	трюме беженцы или десант)
; 4577: 					; 4
; 4578: 					; 3 =1 3я миссия была выполнена
; 4579: 					; 2 =1 - третья	миссия выполнена
; 4580: 					; 1 =1 - второя	миссия выполнена
; 4581: 					; 0 =1 - первая	миссия выполнена
; 4582: 		bit	6, (hl)
; 4583: 		ret	z
; 4584: 		ld	hl, CloakingDeviceOn
; 4585: 		dec	(hl)
; 4586: 		jr	z, loc_7257
; 4587: 		ld	a, (byte_70FA)
; 4588: 		cp	28h ; '('
; 4589: 		ret	c
; 4590: 		ld	a, 1
; 4591: 		ld	(hl), a
; 4592: 		ld	hl, byte_6069
; 4593: 		res	2, (hl)
; 4594: 		xor	a
; 4595: 		ld	(PlayerPirate),	a
; 4596: 
; 4597: loc_7253:				; CODE XREF: sub_7265+4j
; 4598: 		ld	l, 1111b
; 4599: 		jr	loc_7259
; 4600: ; ---------------------------------------------------------------------------
; 4601: 
; 4602: loc_7257:				; CODE XREF: Use_Y_Cloacking+1Dj
; 4603: 		ld	l, 7
; 4604: 
; 4605: loc_7259:				; CODE XREF: Use_Y_Cloacking+33j
; 4606: 					; sub_725C+6j
; 4607: 		jp	j_SetAtrViewScr
; 4608: ; End of function Use_Y_Cloacking
; 4609: 
; 4610: 
; 4611: ; =============== S U B	R O U T	I N E =======================================
; 4612: 
; 4613: 
; 4614: sub_725C:				; CODE XREF: PrnCocpit_Ifn+35p
; 4615: 		ld	a, (PilotMission1)
; 4616: 		and	a
; 4617: 
; 4618: loc_7260:				; CODE XREF: sub_726C+Aj
; 4619: 		ld	l, 2
; 4620: 		jr	nz, loc_7259
; 4621: 		ret
; 4622: ; End of function sub_725C
; 4623: 
; 4624: 
; 4625: ; =============== S U B	R O U T	I N E =======================================
; 4626: 
; 4627: 
; 4628: sub_7265:				; CODE XREF: PrnCocpit_Ifn+38p
; 4629: 		ld	a, (CloakingDeviceOn)
; 4630: 		and	a
; 4631: 		jr	nz, loc_7253
; 4632: 		ret
; 4633: ; End of function sub_7265
; 4634: 
; 4635: 
; 4636: ; =============== S U B	R O U T	I N E =======================================
; 4637: 
; 4638: 
; 4639: sub_726C:				; CODE XREF: LaunchDocking+90p
; 4640: 		ld	a, (PilotMission1)
; 4641: 		and	a
; 4642: 		ret	z
; 4643: 		ld	a, 1
; 4644: 		ld	(PilotMission1_1), a
; 4645: 		jr	loc_7260
; 4646: ; End of function sub_726C
; 4647: 
; 4648: 
; 4649: ; =============== S U B	R O U T	I N E =======================================
; 4650: 
; 4651: 
; 4652: GoMission3:				; CODE XREF: sub_5FA1+3p
; 4653: 
; 4654: ; FUNCTION CHUNK AT D0DD SIZE 00000003 BYTES
; 4655: ; FUNCTION CHUNK AT D0ED SIZE 00000003 BYTES
; 4656: ; FUNCTION CHUNK AT D123 SIZE 00000025 BYTES
; 4657: 
; 4658: 		ld	a, (PilotMission3) ; =#06 при стыковке будет захват таргонами (вроде нереальная	ситуация)
; 4659: 					; =#04 в системе со танцией таргонов во	время войны
; 4660: 		dec	a
; 4661: 		jr	z, loc_727F
; 4662: 		dec	a
; 4663: 
; 4664: loc_727F:				; CODE XREF: GoMission3+4j
; 4665: 		jp	z, GiveMission3
; 4666: 		sub	3
; 4667: 		jp	z, EndMission3
; 4668: 		dec	a
; 4669: 		ld	a, 0D4h	; '╘'
; 4670: 		jr	z, loc_7295
; 4671: 		ret
; 4672: ; End of function GoMission3
; 4673: 
; 4674: 
; 4675: ; =============== S U B	R O U T	I N E =======================================
