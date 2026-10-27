; 10_circles_planets - оставшиеся подпрограммы
; Количество: 35

; === DataOfAround === DataOfAround:	db 13h,	0, 0		; field_0 ; DATA XREF: ReCalcCrdObj_coriolis+59o
DataOfAround:	db 13h,	0, 0		; field_0 ; DATA XREF: ReCalcCrdObj_coriolis+59o
					; sub_7C13+2Co
		db 1Ah,	0, 0		; field_3
		db 0A9h, 6, 0		; field_6
		db 0, 80h, 0, 81h, 0FFh, 7Fh; field_9
		db 0, 44h, 0FFh, 6Ch, 0, 1; field_F
		db 0FFh, 6Bh, 0, 0C3h, 0, 80h; field_15
		db 1Dh			; field_1B
					; speed
		db 0			; field_1C
					; seed up
		db 10h			; field_1D
					; oTurnLR
		db 0			; field_1E
					; oTurnDU
		db 1			; field_1F
					; oRadiusBlast
		db 0			; field_20
					; oFlg_1
		db 0			; field_21
					; oFlg_2
		db 0			; field_22
					; oEnerg
		dw 6228h		; field_23
					; oAdrData
		db 7Eh			; field_25
					; oFlg_3
		db 0			; field_26
					; oFlg_4


; === Altitude === Altitude:	db 0			; DATA XREF: PrintCompass+9Aw
Altitude:	db 0			; DATA XREF: PrintCompass+9Aw
					; sub_9946+21w	...
		db 0A0h	; а
byte_70F4:	db 41h			; DATA XREF: CalcAngSpeed4Prn+3Bw
		db 0A0h	; а


; === SunRadiusNova === SunRadiusNova:	db 0			; DATA XREF: sub_728D+18Bw
SunRadiusNova:	db 0			; DATA XREF: sub_728D+18Bw
					; CorrData4Nova+5r ...

; =============== S U B	R O U T	I N E =======================================




; === CircleFlg === CircleFlg:	db 0Fh			; DATA XREF: RAM:7641w	RAM:771Bw ...
CircleFlg:	db 0Fh			; DATA XREF: RAM:7641w	RAM:771Bw ...


; === CircleRadius === CircleRadius:	db 0D6h			; DATA XREF: PrnSunPlanet+1Dr
CircleRadius:	db 0D6h			; DATA XREF: PrnSunPlanet+1Dr
					; CorrData4Nova+Br ...
word_861E:	dw 0			; DATA XREF: PrnCircle_Clip+4w
					; PrnCircle_Clip+74w ...
word_8620:	dw 8040h		; DATA XREF: PrnCircle_Clip+15w
					; sub_87D4r ...
byte_8622:	db 0B1h			; DATA XREF: PrnCircle_Clip+11Bw
					; PrnCircle_Clip+136r ...
word_8623:	dw 4118h		; DATA XREF: sub_86A1:loc_8B0Fw
					; sub_86A1:loc_8B84r ...
word_8625:	dw 7AA4h		; DATA XREF: sub_8ACD+13w
					; sub_8ACD:loc_8AE3r
byte_8627:	db 40h			; DATA XREF: sub_89EF+2w sub_89EF+22r	...


; === DirPrnCircle === DirPrnCircle:	db 1			; DATA XREF: sub_5E36:loc_5E50r
DirPrnCircle:	db 1			; DATA XREF: sub_5E36:loc_5E50r
					; sub_862B+Bw ...


; === FillCircle === FillCircle:	db 1			; DATA XREF: sub_862B+Er sub_8643+Dr ...
FillCircle:	db 1			; DATA XREF: sub_862B+Er sub_8643+Dr ...
byte_862A:	db 20h			; DATA XREF: sub_8E77+4r

; =============== S U B	R O U T	I N E =======================================




; === DataFillCircle === DataFillCircle:	dw 0FFFFh		; DATA XREF: sub_5E36+Ar sub_86A1+52Co ...
DataFillCircle:	dw 0FFFFh		; DATA XREF: sub_5E36+Ar sub_86A1+52Co ...
; ---------------------------------------------------------------------------
; START	OF FUNCTION CHUNK FOR PrnCircle_Clip

loc_8FB8:				; CODE XREF: PrnCircle_Clip+207j
		ld	a, e
		and	0E0h ; 'р'
		or	1Eh
		ld	l, a
		ld	c, 1
		ld	a, (CircleFlg)
		bit	1, a
		call	nz, sub_862B
		call	z, sub_8643
		ret
; ---------------------------------------------------------------------------

loc_8FCC:				; CODE XREF: PrnCircle_Clip+240j
		ld	b, 80h ; 'А'
		ld	a, (CircleFlg)
		bit	1, a
		call	nz, sub_865A
		call	z, sub_8672
		ret
; END OF FUNCTION CHUNK	FOR PrnCircle_Clip
; ---------------------------------------------------------------------------
; START	OF FUNCTION CHUNK FOR sub_8985

loc_8FDA:				; CODE XREF: sub_8985+12j
		ld	a, e
		and	0E0h ; 'р'

loc_8FDD:				; DATA XREF: RAM:8EDDo
		or	1Eh
		ld	l, a
		ld	c, 1
		ret
; END OF FUNCTION CHUNK	FOR sub_8985
; ---------------------------------------------------------------------------
; START	OF FUNCTION CHUNK FOR sub_89D4

loc_8FE3:				; CODE XREF: sub_89D4+Ej
		ld	b, 1
		inc	a

loc_8FE6:				; CODE XREF: sub_89D4+615j
		rrc	b
		dec	a
		jr	nz, loc_8FE6
		ld	e, l
		ret
; END OF FUNCTION CHUNK	FOR sub_89D4
; ---------------------------------------------------------------------------
; START	OF FUNCTION CHUNK FOR sub_89EF

loc_8FED:				; CODE XREF: sub_89EF+18j
		ld	c, 80h ; 'А'

loc_8FEF:				; CODE XREF: sub_89EF+603j
		rlc	c
		dec	a
		jr	nz, loc_8FEF
		ld	a, (byte_8627)
		ld	b, a
		ret
; END OF FUNCTION CHUNK	FOR sub_89EF
; ---------------------------------------------------------------------------
; START	OF FUNCTION CHUNK FOR sub_8A25

loc_8FF9:				; CODE XREF: sub_8A25+4j
		ld	a, l
		and	0E0h ; 'р'

loc_8FFC:				; DATA XREF: RAM:8EEBo
		or	1
		ld	e, a
		ld	c, 1
		ld	b, 80h ; 'А'
		ret
; END OF FUNCTION CHUNK	FOR sub_8A25
; ---------------------------------------------------------------------------
; START	OF FUNCTION CHUNK FOR sub_86B8

loc_9004:				; CODE XREF: sub_86B8+6j
		push	af
		push	bc
		ld	b, a

loc_9007:				; DATA XREF: RAM:8EEDo
		ld	c, 0F7h	; 'ў'
		call	22B0h

loc_900C:				; DATA XREF: RAM:8EEFo	RAM:8EF1o
		set	7, h
		pop	bc
		push	bc
		dec	b
		inc	b
		jr	z, loc_901C

loc_9014:				; CODE XREF: sub_86B8:loc_901Aj
		ld	a, h
		dec	h
		and	7
		jr	z, loc_901F

loc_901A:				; CODE XREF: sub_86B8+971j
					; sub_86B8+976j
		djnz	loc_9014

loc_901C:				; CODE XREF: sub_86B8+95Aj
					; sub_86B8+978j
		pop	bc
		pop	af
		ret
; ---------------------------------------------------------------------------

loc_901F:				; CODE XREF: sub_86B8+960j
		ld	a, l
		sub	20h ; ' '
		ld	l, a
		jr	c, loc_902B
		ld	a, h
		add	a, 8
		ld	h, a
		jr	loc_901A
; ---------------------------------------------------------------------------

loc_902B:				; CODE XREF: sub_86B8+96Bj
		ld	a, h

loc_902C:				; DATA XREF: RAM:8EF3o
		cp	0C0h ; '└'
		jr	nc, loc_901A
		jr	loc_901C
; END OF FUNCTION CHUNK	FOR sub_86B8
; ---------------------------------------------------------------------------
; START	OF FUNCTION CHUNK FOR sub_86C4

loc_9032:				; CODE XREF: sub_86C4+6j
		push	af
		push	bc
		ld	b, a

loc_9035:				; DATA XREF: RAM:8EF5o
		ld	c, 0F7h	; 'ў'
		call	22B0h

loc_903A:				; DATA XREF: RAM:8EF7o	RAM:8EF9o
		set	7, h
		pop	bc
		push	bc
		dec	b
		inc	b
		jr	z, loc_904A

loc_9042:				; CODE XREF: sub_86C4:loc_9048j
		inc	h
		ld	a, h
		and	7
		jr	z, loc_904D

loc_9048:				; CODE XREF: sub_86C4+993j
					; sub_86C4+998j
		djnz	loc_9042

loc_904A:				; CODE XREF: sub_86C4+97Cj
					; sub_86C4+99Aj
		pop	bc
		pop	af
		ret
; ---------------------------------------------------------------------------

loc_904D:				; CODE XREF: sub_86C4+982j
		ld	a, l
		add	a, 20h ; ' '
		ld	l, a
		jr	c, loc_9059
		ld	a, h
		sub	8
		ld	h, a
		jr	loc_9048
; ---------------------------------------------------------------------------

loc_9059:				; CODE XREF: sub_86C4+98Dj
		ld	a, h

loc_905A:				; DATA XREF: RAM:8EFBo
		cp	0D0h ; '╨'
		jr	c, loc_9048
		jr	loc_904A
; END OF FUNCTION CHUNK	FOR sub_86C4
; ---------------------------------------------------------------------------
; START	OF FUNCTION CHUNK FOR sub_8643

loc_9060:				; CODE XREF: sub_862B+12j sub_8643+11j
		ld	a, e
		and	1Fh
		cp	20h ; ' '
		jr	nc, loc_9079
		ld	a, (DirPrnCircle)
		and	a
		jr	nz, loc_9074

loc_906D:				; DATA XREF: RAM:8EFFo
		ld	a, 0BFh	; '┐'
		cp	h
		jr	c, loc_907B
		jr	loc_9079
; ---------------------------------------------------------------------------

loc_9074:				; CODE XREF: sub_8643+A28j
					; DATA XREF: RAM:8EFDo
		ld	a, 0CFh	; '╧'
		cp	h
		jr	nc, loc_907B

loc_9079:				; CODE XREF: sub_8643+A22j
					; sub_8643+A2Fj
		pop	af
		ret
; ---------------------------------------------------------------------------

loc_907B:				; CODE XREF: sub_8643+A2Dj
					; sub_8643+A34j
		ld	(word_8623), sp

loc_907F:				; CODE XREF: sub_8643+A57j
					; sub_8643+A69j
		ld	d, l
		ld	l, e
		ld	a, (hl)
		or	b
		ld	(hl), a
		ld	l, d
		ld	a, (DirPrnCircle)
		and	a
		jr	nz, loc_9090
		call	sub_8DEE
		jr	loc_9093
; ---------------------------------------------------------------------------

loc_9090:				; CODE XREF: sub_8643+A46j
		call	loc_8E08

loc_9093:				; CODE XREF: sub_8643+A4Bj
		exx
		add	hl, de
		inc	e
		add	hl, de
		bit	7, h
		exx
		jr	nz, loc_907F
		rrc	b
		jr	c, loc_90B0

loc_90A0:				; CODE XREF: sub_8643+A72j
		exx
		add	hl, bc
		inc	c
		ex	af, af'
		cp	c
		jr	z, loc_90B7
		ex	af, af'
		add	hl, bc
		bit	7, h
		exx
		jr	nz, loc_907F
		jr	loc_90CC
; ---------------------------------------------------------------------------

loc_90B0:				; CODE XREF: sub_8643+A5Bj
		inc	e
		ld	a, l
		sub	e
		jr	c, loc_90BF
		jr	loc_90A0
; ---------------------------------------------------------------------------

loc_90B7:				; CODE XREF: sub_8643+A62j
		ex	af, af'
		jp	loc_914B
; ---------------------------------------------------------------------------

loc_90BB:				; CODE XREF: sub_8643+A9Aj
		ex	af, af'
		jp	loc_9191
; ---------------------------------------------------------------------------

loc_90BF:				; CODE XREF: sub_8643+A70j
					; sub_8643+A85j ...
		ld	sp, (word_8623)
		pop	af
		ret
; ---------------------------------------------------------------------------

loc_90C5:				; CODE XREF: sub_8643+A91j
		inc	e
		ld	a, l
		sub	e
		jr	c, loc_90BF
		jr	loc_90D6
; ---------------------------------------------------------------------------

loc_90CC:				; CODE XREF: sub_8643+A6Bj
					; sub_8643+AA1j ...
		ld	d, l
		ld	l, e
		ld	a, (hl)
		or	b
		ld	(hl), a
		ld	l, d
		rrc	b
		jr	c, loc_90C5

loc_90D6:				; CODE XREF: sub_8643+A87j
		exx
		add	hl, bc
		inc	c
		jr	z, loc_90BF
		ex	af, af'
		cp	c
		jr	z, loc_90BB
		ex	af, af'
		add	hl, bc
		bit	7, h
		exx
		jr	z, loc_90CC
		exx
		add	hl, de
		inc	e
		add	hl, de
		exx
		ld	a, (DirPrnCircle)
		and	a
		jr	nz, loc_90F6
		call	sub_8DEE
		jr	loc_90CC
; ---------------------------------------------------------------------------

loc_90F6:				; CODE XREF: sub_8643+AACj
		call	loc_8E08
		jr	loc_90CC
; END OF FUNCTION CHUNK	FOR sub_8643
; ---------------------------------------------------------------------------
; START	OF FUNCTION CHUNK FOR sub_86A1

loc_90FB:				; CODE XREF: sub_8689+12j sub_86A1+11j
		ld	a, e
		sub	l
		jr	nc, loc_9111
		ld	a, (DirPrnCircle)
		and	a
		jr	nz, loc_910C

loc_9105:				; DATA XREF: RAM:8F01o
		ld	a, 0BFh	; '┐'
		cp	h
		jr	c, loc_9113
		jr	loc_9111
; ---------------------------------------------------------------------------

loc_910C:				; CODE XREF: sub_86A1+A62j
					; DATA XREF: RAM:8F03o
		ld	a, 0CFh	; '╧'
		cp	h
		jr	nc, loc_9113

loc_9111:				; CODE XREF: sub_86A1+A5Cj
					; sub_86A1+A69j
		pop	af
		ret
; ---------------------------------------------------------------------------

loc_9113:				; CODE XREF: sub_86A1+A67j
					; sub_86A1+A6Ej
		ld	(word_8623), sp

loc_9117:				; CODE XREF: sub_86A1+A9Cj
					; sub_86A1+AAEj
		ld	a, (hl)
		or	c
		ld	(hl), a
		ld	d, l
		ld	l, e
		ld	a, (hl)
		or	b
		ld	(hl), a
		ld	l, d
		ld	a, (DirPrnCircle)
		and	a
		jr	nz, loc_912F
		ld	a, h
		dec	h
		and	7
		call	z, loc_8DF3
		jr	loc_9136
; ---------------------------------------------------------------------------

loc_912F:				; CODE XREF: sub_86A1+A83j
		inc	h
		ld	a, h
		and	7
		call	z, loc_8E0D

loc_9136:				; CODE XREF: sub_86A1+A8Cj
		exx
		add	hl, de
		inc	e
		add	hl, de
		bit	7, h
		exx
		jp	nz, loc_9117
		rrc	b
		jr	c, loc_9155

loc_9144:				; CODE XREF: sub_86A1+AB9j
		rlc	c
		jr	c, loc_915D

loc_9148:				; CODE XREF: sub_86A1+AC1j
		exx
		add	hl, bc
		inc	c

loc_914B:				; CODE XREF: sub_8643+A75j
					; sub_8672+BD8j
		add	hl, bc
		bit	7, h
		exx
		jp	nz, loc_9117
		jp	loc_917B
; ---------------------------------------------------------------------------

loc_9155:				; CODE XREF: sub_86A1+AA1j
		inc	e
		ld	a, l
		sub	e
		jr	z, loc_9165
		jp	loc_9144
; ---------------------------------------------------------------------------

loc_915D:				; CODE XREF: sub_86A1+AA5j
		dec	l
		ld	a, l
		sub	e
		jr	z, loc_9165
		jp	loc_9148
; ---------------------------------------------------------------------------

loc_9165:				; CODE XREF: sub_86A1+AB7j
					; sub_86A1+ABFj ...
		ld	sp, (word_8623)
		pop	af
		ret
; END OF FUNCTION CHUNK	FOR sub_86A1
; ---------------------------------------------------------------------------
; START	OF FUNCTION CHUNK FOR sub_8643

loc_916B:				; CODE XREF: sub_8643+B43j
		inc	e
		ld	a, l
		sub	e
		jr	z, loc_91AC
		jp	loc_9188
; ---------------------------------------------------------------------------

loc_9173:				; CODE XREF: sub_8643+B47j
		dec	l
		ld	a, l
		sub	e
		jr	z, loc_91C0
		jp	loc_918C
; ---------------------------------------------------------------------------

loc_917B:				; CODE XREF: sub_86A1+AB1j
					; sub_8643+B52j ...
		ld	a, (hl)
		or	c
		ld	(hl), a
		ld	d, l
		ld	l, e
		ld	a, (hl)
		or	b
		ld	(hl), a
		ld	l, d
		rrc	b
		jr	c, loc_916B

loc_9188:				; CODE XREF: sub_8643+B2Dj
		rlc	c
		jr	c, loc_9173

loc_918C:				; CODE XREF: sub_8643+B35j
		exx
		add	hl, bc
		inc	c
		jr	z, loc_91D1

loc_9191:				; CODE XREF: sub_8643+A79j
					; sub_8672+BDCj
		add	hl, bc
		bit	7, h
		exx
		jr	z, loc_917B
		exx
		add	hl, de
		inc	e
		add	hl, de
		exx
		ld	a, (DirPrnCircle)
		and	a
		jr	nz, loc_91A7
		call	sub_8DEE
		jr	loc_917B
; ---------------------------------------------------------------------------

loc_91A7:				; CODE XREF: sub_8643+B5Dj
		call	loc_8E08
		jr	loc_917B
; ---------------------------------------------------------------------------

loc_91AC:				; CODE XREF: sub_8643+B2Bj
		ld	l, e
		ld	a, (hl)
		ld	b, 0FFh
		and	a
		jr	z, loc_91CD
		ld	b, a
		ld	a, 0FFh

loc_91B6:				; CODE XREF: sub_8643+B78j
		srl	b
		jr	c, loc_91BD
		add	a, a
		jr	loc_91B6
; ---------------------------------------------------------------------------

loc_91BD:				; CODE XREF: sub_8643+B75j
		ld	(hl), a
		jr	loc_91CE
; ---------------------------------------------------------------------------

loc_91C0:				; CODE XREF: sub_8643+B33j
		ld	a, (hl)
		ld	b, 0FFh
		and	a
		jr	z, loc_91CD

loc_91C6:				; CODE XREF: sub_8643+B88j
		add	a, a
		jr	c, loc_91CD
		srl	b
		jr	loc_91C6
; ---------------------------------------------------------------------------

loc_91CD:				; CODE XREF: sub_8643+B6Ej
					; sub_8643+B81j ...
		ld	(hl), b

loc_91CE:				; CODE XREF: sub_8643+B7Bj
					; sub_8643+BAFj
		jp	loc_9165
; ---------------------------------------------------------------------------

loc_91D1:				; CODE XREF: sub_8643+B4Cj
		exx
		ld	a, (hl)
		and	a
		jr	z, loc_91E0
		ld	b, a
		ld	a, 0FFh

loc_91D9:				; CODE XREF: sub_8643+B9Bj
		srl	b
		jr	c, loc_91E2
		add	a, a
		jr	loc_91D9
; ---------------------------------------------------------------------------

loc_91E0:				; CODE XREF: sub_8643+B91j
		ld	a, 0FFh

loc_91E2:				; CODE XREF: sub_8643+B98j
		ld	(hl), a
		ld	l, e
		ld	a, (hl)
		ld	b, 0FFh
		and	a
		jr	z, loc_91F1

loc_91EA:				; CODE XREF: sub_8643+BACj
		add	a, a
		jr	c, loc_91F1
		srl	b
		jr	loc_91EA
; ---------------------------------------------------------------------------

loc_91F1:				; CODE XREF: sub_8643+BA5j
					; sub_8643+BA8j
		ld	(hl), b
		jr	loc_91CE
; END OF FUNCTION CHUNK	FOR sub_8643
; ---------------------------------------------------------------------------
; START	OF FUNCTION CHUNK FOR sub_8672

loc_91F4:				; CODE XREF: sub_865A+12j sub_8672+11j
		ld	a, l
		and	1Fh
		cp	20h ; ' '
		jr	nc, loc_920D
		ld	a, (DirPrnCircle)
		and	a
		jr	nz, loc_9208

loc_9201:				; DATA XREF: RAM:8F07o
		ld	a, 0BFh	; '┐'
		cp	h
		jr	c, loc_920F
		jr	loc_920D
; ---------------------------------------------------------------------------

loc_9208:				; CODE XREF: sub_8672+B8Dj
					; DATA XREF: RAM:8F05o
		ld	a, 0CFh	; '╧'
		cp	h
		jr	nc, loc_920F

loc_920D:				; CODE XREF: sub_8672+B87j
					; sub_8672+B94j
		pop	af
		ret
; ---------------------------------------------------------------------------

loc_920F:				; CODE XREF: sub_8672+B92j
					; sub_8672+B99j
		ld	(word_8623), sp

loc_9213:				; CODE XREF: sub_8672+BB9j
					; sub_8672+BCBj
		ld	a, (hl)
		or	c
		ld	(hl), a
		ld	a, (DirPrnCircle)
		and	a
		jr	nz, loc_9221
		call	sub_8DEE
		jr	loc_9224
; ---------------------------------------------------------------------------

loc_9221:				; CODE XREF: sub_8672+BA8j
		call	loc_8E08

loc_9224:				; CODE XREF: sub_8672+BADj
		exx
		add	hl, de
		inc	e
		add	hl, de
		bit	7, h
		exx
		jr	nz, loc_9213
		rlc	c
		jr	c, loc_9241

loc_9231:				; CODE XREF: sub_8672+BD5j
		exx
		add	hl, bc
		inc	c
		ex	af, af'
		cp	c
		jr	z, loc_9249
		ex	af, af'
		add	hl, bc
		bit	7, h
		exx
		jr	nz, loc_9213
		jr	loc_925F
; ---------------------------------------------------------------------------

loc_9241:				; CODE XREF: sub_8672+BBDj
		dec	l
		ld	a, l
		sub	e
		inc	a
		jr	z, loc_9251
		jr	loc_9231
; ---------------------------------------------------------------------------

loc_9249:				; CODE XREF: sub_8672+BC4j
		ex	af, af'
		jp	loc_914B
; ---------------------------------------------------------------------------

loc_924D:				; CODE XREF: sub_8672+BFBj
		ex	af, af'
		jp	loc_9191
; ---------------------------------------------------------------------------

loc_9251:				; CODE XREF: sub_8672+BD3j
					; sub_8672+BE9j ...
		ld	sp, (word_8623)
		pop	af
		ret
; ---------------------------------------------------------------------------

loc_9257:				; CODE XREF: sub_8672+BF2j
		dec	l
		ld	a, l
		sub	e
		inc	a
		jr	z, loc_9251
		jr	loc_9266
; ---------------------------------------------------------------------------

loc_925F:				; CODE XREF: sub_8672+BCDj
					; sub_8672+C02j ...
		ld	a, (hl)
		or	c
		ld	(hl), a
		rlc	c
		jr	c, loc_9257

loc_9266:				; CODE XREF: sub_8672+BEBj
		exx
		add	hl, bc
		inc	c
		jr	z, loc_9251
		ex	af, af'
		cp	c
		jr	z, loc_924D
		ex	af, af'
		add	hl, bc
		bit	7, h
		exx
		jr	z, loc_925F
		exx
		add	hl, de
		inc	e
		add	hl, de
		exx
		ld	a, (DirPrnCircle)
		and	a
		jr	nz, loc_9286
		call	sub_8DEE
		jr	loc_925F
; ---------------------------------------------------------------------------

loc_9286:				; CODE XREF: sub_8672+C0Dj
		call	loc_8E08
		jr	loc_925F
; END OF FUNCTION CHUNK	FOR sub_8672
; ---------------------------------------------------------------------------
		db    0

; =============== S U B	R O U T	I N E =======================================

; обратный отсчет до прыжка, круги
; Attributes: noreturn



; === RadiusCircle === RadiusCircle:	db 2Ah			; DATA XREF: RAM:8E51r
RadiusCircle:	db 2Ah			; DATA XREF: RAM:8E51r
					; Cmd_I_GlobalMap+2Cw ...


; === PilotPlanetData === PilotPlanetData:db 38h			; DATA XREF: sub_DCE9+7o
PilotPlanetData:db 38h			; DATA XREF: sub_DCE9+7o
					; sub_DCE9:loc_DE18o


; === PilotPlanetTechLvl === PilotPlanetTechLvl:db 5			; DATA XREF: RAM:6EF0r	sub_D8CE+6r ...
PilotPlanetTechLvl:db 5			; DATA XREF: RAM:6EF0r	sub_D8CE+6r ...


; === PilotPlanetGov === PilotPlanetGov:	db 3			; DATA XREF: CallFunctionObj_1+3CCr
PilotPlanetGov:	db 3			; DATA XREF: CallFunctionObj_1+3CCr
					; sub_DCE9+23w	...


; === PilotPlanetProduct === PilotPlanetProduct:db 11h		; DATA XREF: sub_9B83+1Do
PilotPlanetProduct:db 11h		; DATA XREF: sub_9B83+1Do


; === planetTextiles === planetTextiles:	db 12h			; DATA XREF: SetVars4War+Cw
planetTextiles:	db 12h			; DATA XREF: SetVars4War+Cw
		db 1Ah
		db 0Eh
		db 27h
		db 8
		db 0
		db 0
		db 0Ah
		db 19h


; === planetFireams === planetFireams:	db 0			; DATA XREF: SetVars4War+Fw
planetFireams:	db 0			; DATA XREF: SetVars4War+Fw
		db 0


; === planetMinerals === planetMinerals:	db 3Dh			; DATA XREF: SetVars4War+22w
planetMinerals:	db 3Dh			; DATA XREF: SetVars4War+22w
		db 0Eh
		db 11h
		db 0Ch


; === planetAlienItems === planetAlienItems:db 0			; DATA XREF: MoveMissile+63o
planetAlienItems:db 0			; DATA XREF: MoveMissile+63o
					; GenerateAnotherPrice+6Eo


; === PilotDataOfSun === PilotDataOfSun:	db 0F0h, 52h, 5, 33h, 98h, 7, 0B6h, 0FAh, 8Bh
PilotDataOfSun:	db 0F0h, 52h, 5, 33h, 98h, 7, 0B6h, 0FAh, 8Bh
					; DATA XREF: UseEscapePod:loc_9C71w
					; UseEscapePod+5Bw


; === PlanetData === PlanetData:	dw 0AD38h		; DATA XREF: sub_D8DD+4o
PlanetData:	dw 0AD38h		; DATA XREF: sub_D8DD+4o
					; GenerateInfoPlanet+3w ...


; === PlanetData1 === PlanetData1:	dw 149Ch		; DATA XREF: RAM:6ED8r
PlanetData1:	dw 149Ch		; DATA XREF: RAM:6ED8r
					; GenerateInfoPlanet+12w ...


; === PlanetData2 === PlanetData2:	dw 151Dh		; DATA XREF: GenerateInfoPlanet+77w
PlanetData2:	dw 151Dh		; DATA XREF: GenerateInfoPlanet+77w
					; RAM:EE92r ...


; === DataOfPlanet === DataOfPlanet:	dw 0AAh			; DATA XREF: sub_D857+3o
DataOfPlanet:	dw 0AAh			; DATA XREF: sub_D857+3o
					; GeneratePlanetName+Dr ...


; === DataOfPlanet1 === DataOfPlanet1:	dw 0ECB8h		; DATA XREF: ModifyPlanetData+3r
DataOfPlanet1:	dw 0ECB8h		; DATA XREF: ModifyPlanetData+3r
					; ModifyPlanetData+Fw ...


; === DataOfPlanet2 === DataOfPlanet2:	db 53h			; DATA XREF: sub_E03Dr
DataOfPlanet2:	db 53h			; DATA XREF: sub_E03Dr
					; ModifyPlanetData+Cr ...
byte_DEC5:	db 0C4h			; DATA XREF: PrnPlanetOnSmallMap+Br


; === DataOfPlanetSv === DataOfPlanetSv:	db 38h,	0ADh, 9Ch, 14h,	1Dh, 15h ; DATA	XREF: SavePlanetData+3o
DataOfPlanetSv:	db 38h,	0ADh, 9Ch, 14h,	1Dh, 15h ; DATA	XREF: SavePlanetData+3o
					; RestorePlanetDatao ...


; === DescrPlanetData === DescrPlanetData:db 0, 0, 0, 0		; DATA XREF: sub_D5E1r	sub_D5E1+Cw ...
DescrPlanetData:db 0, 0, 0, 0		; DATA XREF: sub_D5E1r	sub_D5E1+Cw ...
byte_DED0:	db 4Ah,	41h, 4Dh, 45h, 53h, 4Fh, 4Eh, 20h, 20h,	20h, 0
					; DATA XREF: sub_DF24+Do RAM:E39Co ...


; === PlanetPresent === PlanetPresent:	db 4Ch			; DATA XREF: InitGame+2Co sub_D86A+3o	...
PlanetPresent:	db 4Ch			; DATA XREF: InitGame+2Co sub_D86A+3o	...
		db 41h
		db 56h
		db 45h
		db 0
		db 0
		db 0
		db 0
		db 0


; === PlanetName === PlanetName:	db 4Ch			; DATA XREF: InitGame+29o sub_D872+3o	...
PlanetName:	db 4Ch			; DATA XREF: InitGame+29o sub_D872+3o	...
		db 41h
		db 56h
		db 45h
		db 0
		db 0
		db 0
		db 0
		db 0


; === FuelToPlanet === FuelToPlanet:	dw 0			; DATA XREF: GoJump+30r
FuelToPlanet:	dw 0			; DATA XREF: GoJump+30r
					; PrnCountdownJump+6Br	...


; === PlanetEconomy === PlanetEconomy:	db 5			; DATA XREF: PrnCountdownJump+7Br
PlanetEconomy:	db 5			; DATA XREF: PrnCountdownJump+7Br
					; GenerateInfoPlanet+Cw ...


; === PlanetGovernment === PlanetGovernment:db 3			; DATA XREF: sub_DCE9+20r
PlanetGovernment:db 3			; DATA XREF: sub_DCE9+20r
					; GenerateInfoPlanet+22w ...


; === PlanetTechLevel === PlanetTechLevel:db 4			; DATA XREF: sub_DCE9+1Ar
PlanetTechLevel:db 4			; DATA XREF: sub_DCE9+1Ar
					; GenerateInfoPlanet+43w ...


; === PlanetPopulation === PlanetPopulation:db 19h			; DATA XREF: GenerateInfoPlanet+54w
PlanetPopulation:db 19h			; DATA XREF: GenerateInfoPlanet+54w
					; GenerateInfoPlanet+67r ...


; === PlanetGrossProduct === PlanetGrossProduct:dw 1B58h		; DATA XREF: GenerateInfoPlanet+71w
PlanetGrossProduct:dw 1B58h		; DATA XREF: GenerateInfoPlanet+71w
					; RAM:EE46r
		db 0
		db 0
byte_DF00:	db 0			; DATA XREF: sub_DF24+4w
					; sub_DF24:loc_DF43o
word_DF01:	dw 0DEDFh		; DATA XREF: GeneratePlanetName:loc_E01Fw
					; sub_E03D+14r	...
; ---------------------------------------------------------------------------

loc_DF03:				; CODE XREF: RAM:D030j
		ld	hl, PilotNumGalaxy
		ld	a, (hl)
		inc	a
		and	7
		ld	(hl), a
		ld	hl, PilotGalaxyData0
		call	sub_DF1A
		ld	hl, byte_D092
		call	sub_DF1A
		ld	hl, byte_D094

; =============== S U B	R O U T	I N E =======================================




; === PrnSmallCircle_Fill === PrnSmallCircle_Fill:db 3Eh		; CODE XREF: PrnCircle_Fillp
PrnSmallCircle_Fill:db 3Eh		; CODE XREF: PrnCircle_Fillp
					; ld a,#A0

; =============== S U B	R O U T	I N E =======================================




