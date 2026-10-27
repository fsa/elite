;  275: 
;  276: ; =============== S U B	R O U T	I N E =======================================
;  277: 
;  278: 
;  279: CalcCrdCoriolis2:			; CODE XREF: sub_728D+226p
;  280: 
;  281: ; FUNCTION CHUNK AT 7861 SIZE 00000037 BYTES
;  282: 
;  283: 		ld	a, (InFreeSpace)
;  284: 		and	a
;  285: 		ret	nz
;  286: 		jp	CalcCrdCoriolis
;  287: ; End of function CalcCrdCoriolis2
;  288: 
;  289: ; ---------------------------------------------------------------------------
;  290: 		db    0
;  291: 		db    0
;  292: 		db    0
;  293: 		db    0
;  294: 		db    0
;  295: 		db    8
;  296: 		db    8
;  297: 		db  10h
;  298: 		db    0
;  299: 		db    0
;  300: 		db    0
;  301: 		db    0
;  302: 		db  3Eh	; >
;  303: 		db    0
;  304: 		db    0
;  305: 		db    0
;  306: 		db    0
;  307: 		db    0
;  308: 		db    0
;  309: 		db    0
;  310: 		db    0
;  311: 		db  18h
;  312: 		db  18h
;  313: 		db    0
;  314: 		db    0
;  315: 		db    0
;  316: 		db    2
;  317: 		db    4
;  318: 		db    8
;  319: 		db  10h
;  320: 		db  20h
;  321: 		db    0
;  322: ; ---------------------------------------------------------------------------
;  323: ; START	OF FUNCTION CHUNK FOR sub_D499
;  324: 
;  325: loc_5DF0:				; CODE XREF: sub_D499+AAj
;  326: 		pop	de
;  327: 		ld	b, 8
;  328: 		nop
;  329: 		nop
;  330: 		call	sub_D546
;  331: 		nop
;  332: 		nop
;  333: 		ret
;  334: ; END OF FUNCTION CHUNK	FOR sub_D499
;  335: 
;  336: ; =============== S U B	R O U T	I N E =======================================
;  337: 
;  338: 
;  339: sub_5DFB:				; CODE XREF: sub_D499+59p
;  340: 		ld	hl, 3D00h
;  341: 
;  342: loc_5DFE:
;  343: 		cp	20h ; ' '
;  344: 		ret	z
;  345: 		cp	30h ; '0'
;  346: 		jr	c, loc_5E16
;  347: 		cp	3Bh ; ';'
;  348: 		ret	c
;  349: 		cp	41h ; 'A'
;  350: 		jr	c, loc_5E16
;  351: 		cp	5Bh ; '['
;  352: 		ret	c
;  353: 		cp	61h ; 'a'
;  354: 		jr	c, loc_5E16
;  355: 		cp	7Bh ; '{'
;  356: 		ret	c
;  357: 
;  358: loc_5E16:				; CODE XREF: sub_5DFB+8j sub_5DFB+Fj ...
;  359: 		ld	hl, 5D70h
;  360: 		ret
;  361: ; End of function sub_5DFB
;  362: 
;  363: 
;  364: ; =============== S U B	R O U T	I N E =======================================
;  365: 
;  366: 
;  367: sub_5E1A:				; CODE XREF: RAM:E278p
;  368: 		push	af
;  369: 		sub	0DCh ; '▄'
;  370: 		call	z, sub_D5B4	; гибель минеральной жизни при недостаточном количестве
;  371: 					; Вых: z=1 есть	минеральная жизнь в трюме
;  372: 					;	=0 минеральная жизнь померла
;  373: 					;	   нет минеральной жизни
;  374: 		jr	z, loc_5E24
;  375: 		ld	a, 0C9h	; '╔'
;  376: 
;  377: loc_5E24:				; CODE XREF: sub_5E1A+6j
;  378: 		ld	(loc_7013), a
;  379: 		ld	(loc_DE6A), a
;  380: 		jr	z, loc_5E30
;  381: 		pop	af
;  382: 		jp	PrintMessage1_A
;  383: ; ---------------------------------------------------------------------------
;  384: 
;  385: loc_5E30:				; CODE XREF: sub_5E1A+10j
;  386: 		pop	af
;  387: 		ld	a, 52h ; 'R'
;  388: 		jp	PrintMessage2
;  389: ; End of function sub_5E1A
;  390: 
;  391: 
;  392: ; =============== S U B	R O U T	I N E =======================================
;  393: 
;  394: 
;  395: sub_5E36:				; CODE XREF: sub_86A1:loc_8B25p
;  396: 					; sub_8672:loc_8C24p ...
;  397: 		ld	l, d
;  398: 		add	a, a
;  399: 		jr	z, loc_5E50
;  400: 		push	bc
;  401: 		push	de
;  402: 		push	hl
;  403: 		ld	c, a
;  404: 		ld	b, 0
;  405: 		ld	a, (DataFillCircle)
;  406: 		dec	hl
;  407: 		ld	(hl), a
;  408: 		dec	c
;  409: 		jr	z, loc_5E4D
;  410: 		ld	d, h
;  411: 		ld	e, l
;  412: 		dec	de
;  413: 		lddr
;  414: 
;  415: loc_5E4D:				; CODE XREF: sub_5E36+10j
;  416: 		pop	hl
;  417: 		pop	de
;  418: 		pop	bc
;  419: 
;  420: loc_5E50:				; CODE XREF: sub_5E36+2j
;  421: 		ld	a, (DirPrnCircle)
;  422: 		and	a
;  423: 		ret
;  424: ; End of function sub_5E36
;  425: 
;  426: ; ---------------------------------------------------------------------------
;  427: ; START	OF FUNCTION CHUNK FOR LaunchDocking
;  428: 
;  429: loc_5E55:				; CODE XREF: LaunchDocking+6Dj
;  430: 		ld	hl, DemoMode	; =#00 при стыковке со станцией	таргонов
