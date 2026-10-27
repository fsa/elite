; 4375: ; =============== S U B	R O U T	I N E =======================================
; 4376: 
; 4377: 
; 4378: CalcAngSpeed4Prn:			; CODE XREF: sub_728D+1EAp
; 4379: 		ld	hl,  AngleLeftRight+1
; 4380: 		ld	a, (hl)
; 4381: 		ld	(HighAngleLR), a
; 4382: 		bit	7, a
; 4383: 		jr	nz, loc_714B
; 4384: 		add	a, 15h
; 4385: 		jr	loc_7151
; 4386: ; ---------------------------------------------------------------------------
; 4387: 
; 4388: loc_714B:				; CODE XREF: CalcAngSpeed4Prn+9j
; 4389: 		and	7Fh ; ''
; 4390: 		ld	c, a
; 4391: 		ld	a, 15h
; 4392: 		sub	c
; 4393: 
; 4394: loc_7151:				; CODE XREF: CalcAngSpeed4Prn+Dj
; 4395: 		ld	(byte_70FE), a
; 4396: 		inc	l
; 4397: 		inc	l
; 4398: 		ld	a, (hl)
; 4399: 		ld	(HighAngleDU), a
; 4400: 		bit	7, a
; 4401: 		jr	nz, loc_7162
; 4402: 		add	a, 15h
; 4403: 		jr	loc_7168
; 4404: ; ---------------------------------------------------------------------------
; 4405: 
; 4406: loc_7162:				; CODE XREF: CalcAngSpeed4Prn+20j
; 4407: 		and	7Fh ; ''
; 4408: 		ld	c, a
; 4409: 		ld	a, 15h
; 4410: 		sub	c
; 4411: 
; 4412: loc_7168:				; CODE XREF: CalcAngSpeed4Prn+24j
; 4413: 		ld	(byte_7100), a
; 4414: 		inc	l
; 4415: 		inc	l
; 4416: 		ld	a, (hl)
; 4417: 		ld	(HighDeltaSpeed), a
; 4418: 		ld	b, a
; 4419: 		add	a, a
; 4420: 		add	a, a
; 4421: 		add	a, b
; 4422: 		add	a, 5
; 4423: 		ld	(byte_70F4), a
; 4424: 		ret
; 4425: ; End of function CalcAngSpeed4Prn
; 4426: 
; 4427: 
; 4428: ; =============== S U B	R O U T	I N E =======================================
; 4429: 
; 4430: ; Attributes: noreturn
; 4431: 
; 4432: GoDocking:				; CODE XREF: sub_728D+223p
; 4433: 		ld	a, (ManualDocking)
; 4434: 		and	a
; 4435: 		jr	z, loc_7187
; 4436: 		xor	a
; 4437: 		ld	(ManualDocking), a
; 4438: 		jr	loc_71B3
; 4439: ; ---------------------------------------------------------------------------
; 4440: 
; 4441: loc_7187:				; CODE XREF: GoDocking+4j
; 4442: 		ld	a, (ControlKey3)
; 4443: 		ld	b, a
; 4444: 		and	80h ; 'А'
; 4445: 		res	7, b
; 4446: 		ld	a, b
; 4447: 		ld	(ControlKey3), a
; 4448: 		ret	z
; 4449: 		ld	a, (CoriolisNear)
; 4450: 		and	a
; 4451: 		ret	z
; 4452: 		ld	a, (PilotDocComputer)
; 4453: 		and	a
; 4454: 		ret	z
; 4455: 		ld	a, (IndDanger)
; 4456: 		and	a
; 4457: 		ret	z
; 4458: 		ld	a, (JumpDocking) ; =1 межпланетный прыжок/стыковка
; 4459: 					; =2 межгалактический прыжок
; 4460: 		and	a
; 4461: 		ret	nz
; 4462: 		ld	hl, byte_6069	; objCoriolis+oFlg_2
; 4463: 		bit	2, (hl)
; 4464: 		ret	nz
; 4465: 		ld	a, 1
; 4466: 		ld	(AutoDocking), a
; 4467: 
; 4468: loc_71B3:				; CODE XREF: GoDocking+Aj
; 4469: 		xor	a
; 4470: 		ld	(WhatOnScreen),	a
; 4471: 		call	PrnCocpit_Ifn
; 4472: 
; 4473: loc_71BA:				; CODE XREF: UseEscapePod+9j
; 4474: 		call	NewJumpDockingCrdObj
; 4475: 		jp	LaunchDocking
; 4476: ; End of function GoDocking
; 4477: 
; 4478: 
; 4479: ; =============== S U B	R O U T	I N E =======================================
; 4480: 
; 4481: 
; 4482: NewJumpDockingCrdObj:			; CODE XREF: GoDocking:loc_71BAp
; 4483: 		ld	iy, SunCrd
; 4484: 		ld	ix, objCoriolis
; 4485: 		call	sub_71CE
; 4486: 		call	sub_71CE
; 4487: ; End of function NewJumpDockingCrdObj
; 4488: 
; 4489: 
; 4490: ; =============== S U B	R O U T	I N E =======================================
; 4491: 
; 4492: 
; 4493: sub_71CE:				; CODE XREF: NewJumpDockingCrdObj+8p
; 4494: 		call	sub_71E4
; 4495: 		add	hl, hl
; 4496: 		jr	nc, loc_71D6
; 4497: 		set	7, h
; 4498: 
; 4499: loc_71D6:				; CODE XREF: sub_71CE+4j
; 4500: 		ld	(iy+2),	h
; 4501: 		ld	(iy+1),	l
; 4502: 		ld	de, 3
; 4503: 		add	iy, de
; 4504: 		add	ix, de
; 4505: 		ret
; 4506: ; End of function sub_71CE
; 4507: 
; 4508: 
; 4509: ; =============== S U B	R O U T	I N E =======================================
; 4510: 
; 4511: 
; 4512: sub_71E4:				; CODE XREF: sub_71CEp
; 4513: 		ld	h, (iy+2)
; 4514: 		ld	l, (iy+1)
; 4515: 		ld	d, (ix+2)
; 4516: 		ld	e, (ix+1)
; 4517: 		jp	loc_9CA5
; 4518: ; End of function sub_71E4
; 4519: 
; 4520: 
