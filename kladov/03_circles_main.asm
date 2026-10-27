; 9080: 
; 9081: ; =============== S U B	R O U T	I N E =======================================
; 9082: 
; 9083: 
; 9084: PrnCircle_Fill:				; CODE XREF: PrnSunPlanet+1Ap
; 9085: 					; sub_F8D7+3p
; 9086: 		call	PrnSmallCircle_Fill ; ld a,#A0
; 9087: 		cp	9
; 9088: 		ret	c
; 9089: 		di
; 9090: 		xor	a
; 9091: 		ld	(FillCircle), a
; 9092: 		jp	PrnCircle_NoFill_NoRadius
; 9093: ; ---------------------------------------------------------------------------
; 9094: 
; 9095: PrnCircle_NoFill:			; CODE XREF: PrnSunPlanet+39j
; 9096: 					; sub_7E02+8Bp	...
; 9097: 		call	PrnSmallCircle_NoFill
; 9098: 		cp	9
; 9099: 		ret	c
; 9100: 
; 9101: loc_86E4:				; CODE XREF: sub_9296+79p
; 9102: 		ld	a, 1
; 9103: 		ld	(FillCircle), a
; 9104: 
; 9105: PrnCircle_NoFill_NoRadius:		; CODE XREF: PrnCircle_Fill+Bj
; 9106: 		call	PrnCircle_Clip
; 9107: 		ei
; 9108: 		ret
; 9109: ; End of function PrnCircle_Fill
; 9110: 
; 9111: 
; 9112: ; =============== S U B	R O U T	I N E =======================================
; 9113: 
; 9114: 
; 9115: PrnCircle_Clip:				; CODE XREF: PrnCircle_Fill:PrnCircle_NoFill_NoRadiusp
; 9116: 
; 9117: ; FUNCTION CHUNK AT 87C8 SIZE 0000000C BYTES
; 9118: ; FUNCTION CHUNK AT 87F4 SIZE 00000191 BYTES
; 9119: ; FUNCTION CHUNK AT 8FB8 SIZE 00000022 BYTES
; 9120: 
; 9121: 		ld	bc, (CircleCrdY)
; 9122: 		ld	(word_861E), bc
; 9123: 		ld	de, (CircleFlg)
; 9124: 
; 9125: loc_86FA:				; DATA XREF: RAM:off_8ECDo
; 9126: 		ld	a, 40h ; '@'
; 9127: 		bit	1, e
; 9128: 		jr	nz, loc_8702
; 9129: 		add	a, c
; 9130: 		add	a, c
; 9131: 
; 9132: loc_8702:				; CODE XREF: PrnCircle_Clip+10j
; 9133: 		sub	c
; 9134: 		ld	(word_8620), a
; 9135: 		ld	a, 80h ; 'А'
; 9136: 		bit	0, e
; 9137: 		jr	z, loc_870E
; 9138: 		add	a, b
; 9139: 		add	a, b
; 9140: 
; 9141: loc_870E:				; CODE XREF: PrnCircle_Clip+1Cj
; 9142: 		sub	b
; 9143: 		ld	(word_8620+1), a
; 9144: 		ld	a, c
; 9145: 
; 9146: loc_8713:				; DATA XREF: RAM:8ECFo
; 9147: 		cp	40h ; '@'
; 9148: 		bit	3, e
; 9149: 		ex	af, af'
; 9150: 		ld	a, b
; 9151: 
; 9152: loc_8719:				; DATA XREF: RAM:8ED1o
; 9153: 		cp	78h ; 'x'
; 9154: 		bit	2, e
; 9155: 		jr	z, loc_872B
; 9156: 		jr	nc, loc_872C
; 9157: 		ex	af, af'
; 9158: 		jr	z, loc_8728
; 9159: 		jr	nc, loc_875E
; 9160: 		jr	loc_8736
; 9161: ; ---------------------------------------------------------------------------
; 9162: 
; 9163: loc_8728:				; CODE XREF: PrnCircle_Clip+34j
; 9164: 		ret	nc
; 9165: 		jr	loc_875E
; 9166: ; ---------------------------------------------------------------------------
; 9167: 
; 9168: loc_872B:				; CODE XREF: PrnCircle_Clip+2Fj
; 9169: 		ret	nc
; 9170: 
; 9171: loc_872C:				; CODE XREF: PrnCircle_Clip+31j
; 9172: 		ex	af, af'
; 9173: 		jr	z, loc_8733
; 9174: 		jr	nc, loc_878E
; 9175: 		jr	loc_874B
; 9176: ; ---------------------------------------------------------------------------
; 9177: 
; 9178: loc_8733:				; CODE XREF: PrnCircle_Clip+3Fj
; 9179: 		ret	nc
; 9180: 		jr	loc_878E
; 9181: ; ---------------------------------------------------------------------------
; 9182: 
; 9183: loc_8736:				; CODE XREF: PrnCircle_Clip+38j
; 9184: 		ld	a, b
; 9185: 		add	a, d
; 9186: 		jr	c, loc_873F
; 9187: 
; 9188: loc_873A:				; DATA XREF: RAM:8ED3o
; 9189: 		cp	78h ; 'x'
; 9190: 		jp	c, loc_87C8
; 9191: 
; 9192: loc_873F:				; CODE XREF: PrnCircle_Clip+4Aj
; 9193: 		ld	a, d
; 9194: 		sub	b
; 9195: 		jr	c, loc_8745
; 9196: 
; 9197: loc_8743:				; DATA XREF: RAM:8ED5o
; 9198: 		cp	78h ; 'x'
; 9199: 
; 9200: loc_8745:				; CODE XREF: PrnCircle_Clip+53j
; 9201: 		jp	c, loc_8898
; 9202: 		jp	loc_87FE
; 9203: ; ---------------------------------------------------------------------------
; 9204: 
; 9205: loc_874B:				; CODE XREF: PrnCircle_Clip+43j
; 9206: 		ld	a, b
; 9207: 		sub	78h ; 'x'
; 9208: 		ld	b, a
; 9209: 		ld	(word_861E+1), a
; 9210: 		ld	a, d
; 9211: 		sub	b
; 9212: 		ret	c
; 9213: 		ret	z
; 9214: 		sub	0F0h ; 'Ё'
; 9215: 		jp	nc, loc_87F4
; 9216: 		jp	loc_888E
; 9217: ; ---------------------------------------------------------------------------
; 9218: 
; 9219: loc_875E:				; CODE XREF: PrnCircle_Clip+36j
; 9220: 					; PrnCircle_Clip+3Bj
; 9221: 		ld	a, c
; 9222: 		sub	40h ; '@'
; 9223: 		ld	c, a
; 9224: 		ld	(word_861E), a
; 9225: 		ld	a, d
; 9226: 		sub	c
; 9227: 		ret	c
; 9228: 		ld	a, d
; 9229: 		call	Sqr_DE_A
; 9230: 		ld	h, d
; 9231: 		ld	l, e
; 9232: 		ld	a, c
; 9233: 		call	Sqr_DE_A
; 9234: 		sbc	hl, de
; 9235: 		ld	a, 78h ; 'x'
; 9236: 		sub	b
; 9237: 		call	Sqr_DE_A
; 9238: 		sbc	hl, de
; 9239: 		jp	c, loc_893F
; 9240: 		add	hl, de
; 9241: 		ld	a, 78h ; 'x'
; 9242: 		add	a, b
; 9243: 		call	Sqr_DE_A
; 9244: 		sbc	hl, de
; 9245: 		jp	nc, loc_884A
; 9246: 		jp	loc_88C2
; 9247: ; ---------------------------------------------------------------------------
; 9248: 
; 9249: loc_878E:				; CODE XREF: PrnCircle_Clip+41j
; 9250: 					; PrnCircle_Clip+46j
; 9251: 		ld	a, b
; 9252: 		sub	78h ; 'x'
; 9253: 		ld	b, a
; 9254: 		ld	a, c
; 9255: 		sub	40h ; '@'
; 9256: 		ld	c, a
; 9257: 		ld	(word_861E), bc
; 9258: 		ld	a, d
; 9259: 		call	Sqr_DE_A
; 9260: 		ld	h, d
; 9261: 		ld	l, e
; 9262: 		ld	a, c
; 9263: 		call	Sqr_DE_A
; 9264: 		sbc	hl, de
; 9265: 		ret	c
; 9266: 		ld	a, b
; 9267: 		call	Sqr_DE_A
; 9268: 		sbc	hl, de
; 9269: 		ret	c
; 9270: 		add	hl, de
; 9271: 		ld	a, b
; 9272: 		add	a, 0F8h	; '°'
; 9273: 		jr	c, loc_87BC
; 9274: 		call	Sqr_DE_A
; 9275: 		sbc	hl, de
; 9276: 		jp	nc, loc_8840
; 9277: 
; 9278: loc_87BC:				; CODE XREF: PrnCircle_Clip+C4j
; 9279: 		jp	loc_88B8
; 9280: ; End of function PrnCircle_Clip
; 9281: 
; 9282: 
; 9283: ; =============== S U B	R O U T	I N E =======================================
; 9284: 
; 9285: 
; 9286: Sqr_DE_A:				; CODE XREF: CalcLenVectorCrd+29p
; 9287: 					; CalcLenVectorCrd+30p	...
; 9288: 		push	hl
; 9289: 		ld	h, 0BEh	; '╛'
; 9290: 		ld	l, a
; 9291: 		ld	e, (hl)
; 9292: 		inc	h
; 9293: 		ld	d, (hl)
; 9294: 		pop	hl
; 9295: 		ret
; 9296: ; End of function Sqr_DE_A
; 9297: 
; 9298: ; ---------------------------------------------------------------------------
; 9299: ; START	OF FUNCTION CHUNK FOR PrnCircle_Clip
; 9300: 
; 9301: loc_87C8:				; CODE XREF: PrnCircle_Clip+4Ej
; 9302: 		call	sub_87D4
; 9303: 		call	sub_86A1
; 9304: 		call	sub_87D4
; 9305: 		jp	sub_8689
; 9306: ; END OF FUNCTION CHUNK	FOR PrnCircle_Clip
; 9307: 
; 9308: ; =============== S U B	R O U T	I N E =======================================
; 9309: 
; 9310: 
; 9311: sub_87D4:				; CODE XREF: PrnCircle_Clip:loc_87C8p
; 9312: 					; PrnCircle_Clip+E0p
; 9313: 		ld	bc, (word_8620)
; 9314: 		ld	de, (CircleFlg)
; 9315: 		ld	a, d
; 9316: 		neg
; 9317: 		exx
; 9318: 		ld	c, a
; 9319: 		ld	e, 0
; 9320: 		exx
; 9321: 		ld	a, b
; 9322: 		sub	d
; 9323: 		call	sub_89D4
; 9324: 		ld	a, (word_8620)
; 9325: 		ld	c, a
; 9326: 		ld	a, (word_8620+1)
; 9327: 		add	a, d
; 9328: 		jp	sub_89EF
; 9329: ; End of function sub_87D4
; 9330: 
; 9331: ; ---------------------------------------------------------------------------
; 9332: ; START	OF FUNCTION CHUNK FOR PrnCircle_Clip
; 9333: 
; 9334: loc_87F4:				; CODE XREF: PrnCircle_Clip+6Aj
; 9335: 		ld	a, b
; 9336: 		add	a, 78h ; 'x'
; 9337: 		ld	b, a
; 9338: 		ld	(word_861E+1), a
; 9339: 		xor	a
; 9340: 		jr	loc_8802
; 9341: ; ---------------------------------------------------------------------------
; 9342: 
; 9343: loc_87FE:				; CODE XREF: PrnCircle_Clip+5Aj
; 9344: 					; DATA XREF: RAM:8ED7o
; 9345: 		ld	a, 88h ; 'И'
; 9346: 		add	a, b
; 9347: 		inc	a
; 9348: 
; 9349: loc_8802:				; CODE XREF: PrnCircle_Clip+10Ej
; 9350: 		ex	af, af'
; 9351: 
; 9352: loc_8803:				; DATA XREF: RAM:8ED9o
; 9353: 		ld	a, 78h ; 'x'
; 9354: 		add	a, b
; 9355: 		call	sub_8ACD
; 9356: 		ld	(byte_8622), a
; 9357: 		ld	b, a
; 9358: 		ld	a, (word_8620)
; 9359: 		call	sub_86B8
; 9360: 		call	sub_8A16
; 9361: 		call	sub_8A25
; 9362: 		ld	a, (CircleFlg)
; 9363: 		bit	0, a
; 9364: 		call	z, sub_8672
; 9365: 		call	nz, sub_8643
; 9366: 		ld	a, (byte_8622)
; 9367: 		ld	b, a
; 9368: 		ld	a, (word_8620)
; 9369: 		call	sub_86C4
; 9370: 		call	sub_8A16
; 9371: 		call	sub_8A25
; 9372: 		ld	a, (CircleFlg)
; 9373: 		bit	0, a
; 9374: 		call	z, sub_865A
; 9375: 		call	nz, sub_862B
; 9376: 		ret
; 9377: ; ---------------------------------------------------------------------------
; 9378: 
; 9379: loc_8840:				; CODE XREF: PrnCircle_Clip+CBj
; 9380: 		ld	a, b
; 9381: 		add	a, 78h ; 'x'
; 9382: 		ld	b, a
; 9383: 		ld	(word_861E+1), a
; 9384: 		xor	a
; 9385: 		jr	loc_884D
; 9386: ; ---------------------------------------------------------------------------
; 9387: 
; 9388: loc_884A:				; CODE XREF: PrnCircle_Clip+9Aj
; 9389: 		ld	a, b
; 9390: 		sub	77h ; 'w'
; 9391: 
; 9392: loc_884D:				; CODE XREF: PrnCircle_Clip+15Aj
; 9393: 		ex	af, af'
; 9394: 		ld	de, (CircleFlg)
; 9395: 		ld	a, 78h ; 'x'
; 9396: 		add	a, b
; 9397: 		call	sub_8ACD
; 9398: 		ld	(byte_8622), a
; 9399: 		sub	c
; 9400: 		ld	b, a
; 9401: 		bit	1, e
; 9402: 		jr	z, loc_8877
; 9403: 		xor	a
; 9404: 		call	sub_86C4
; 9405: 		call	sub_8A16
; 9406: 		call	sub_8A25
; 9407: 		ld	a, (CircleFlg)
; 9408: 		bit	0, a
; 9409: 		call	z, sub_865A
; 9410: 		call	nz, sub_862B
; 9411: 		ret
; 9412: ; ---------------------------------------------------------------------------
; 9413: 
; 9414: loc_8877:				; CODE XREF: PrnCircle_Clip+171j
; 9415: 		ld	a, 7Fh ; ''
; 9416: 		call	sub_86B8
; 9417: 		call	sub_8A16
; 9418: 		call	sub_8A25
; 9419: 		ld	a, (CircleFlg)
; 9420: 		bit	0, a
; 9421: 		call	z, sub_8672
; 9422: 		call	nz, sub_8643
; 9423: 		ret
; 9424: ; ---------------------------------------------------------------------------
; 9425: 
; 9426: loc_888E:				; CODE XREF: PrnCircle_Clip+6Dj
; 9427: 		ld	a, b
; 9428: 		add	a, 78h ; 'x'
; 9429: 		ld	(word_861E+1), a
; 9430: 		ld	b, a
; 9431: 		xor	a
; 9432: 		jr	loc_889B
; 9433: ; ---------------------------------------------------------------------------
; 9434: 
; 9435: loc_8898:				; CODE XREF: PrnCircle_Clip:loc_8745j
; 9436: 		ld	a, b
; 9437: 
; 9438: loc_8899:				; DATA XREF: RAM:8EDBo
; 9439: 		sub	77h ; 'w'
; 9440: 
; 9441: loc_889B:				; CODE XREF: PrnCircle_Clip+1A8j
; 9442: 		ex	af, af'
; 9443: 		bit	0, e
; 9444: 		jr	z, loc_88AC
; 9445: 		call	sub_8985
; 9446: 		call	sub_862B
; 9447: 		call	sub_8985
; 9448: 		jp	sub_8643
; 9449: ; ---------------------------------------------------------------------------
; 9450: 
; 9451: loc_88AC:				; CODE XREF: PrnCircle_Clip+1B0j
; 9452: 		call	sub_89A1
; 9453: 		call	sub_865A
; 9454: 		call	sub_89A1
; 9455: 		jp	sub_8672
; 9456: ; ---------------------------------------------------------------------------
; 9457: 
; 9458: loc_88B8:				; CODE XREF: PrnCircle_Clip:loc_87BCj
; 9459: 		ld	a, b
; 9460: 		add	a, 78h ; 'x'
; 9461: 		ld	(word_861E+1), a
; 9462: 		ld	b, a
; 9463: 		xor	a
; 9464: 		jr	loc_88C5
; 9465: ; ---------------------------------------------------------------------------
; 9466: 
; 9467: loc_88C2:				; CODE XREF: PrnCircle_Clip+9Dj
; 9468: 		ld	a, b
; 9469: 		sub	77h ; 'w'
; 9470: 
; 9471: loc_88C5:				; CODE XREF: PrnCircle_Clip+1D2j
; 9472: 		ex	af, af'
; 9473: 		ld	de, (CircleFlg)
; 9474: 		ld	a, c
; 9475: 		call	sub_8ACD
; 9476: 		ld	(byte_8622), a
; 9477: 		ld	a, c
; 9478: 		exx
; 9479: 		ld	e, a
; 9480: 		ld	a, (byte_8622)
; 9481: 		neg
; 9482: 		ld	c, a
; 9483: 		exx
; 9484: 		bit	0, e
; 9485: 		jr	z, loc_8910
; 9486: 		ld	a, (byte_8622)
; 9487: 		cpl
; 9488: 		add	a, 81h ; 'Б'
; 9489: 		add	a, b
; 9490: 		ld	c, 0
; 9491: 		bit	1, e
; 9492: 		jr	nz, loc_88EE
; 9493: 		ld	c, 7Fh ; ''
; 9494: 
; 9495: loc_88EE:				; CODE XREF: PrnCircle_Clip+1FCj
; 9496: 		call	sub_89D4
; 9497: 		ld	a, (FillCircle)
; 9498: 		and	a
; 9499: 		jp	nz, loc_8FB8
; 9500: 		ld	a, e
; 9501: 		and	1Fh
; 9502: 		cp	1Fh
; 9503: 		ret	z
; 9504: 		ld	a, e
; 9505: 		or	1Fh
; 9506: 		ld	l, a
; 9507: 		ld	c, 0
; 9508: 		ld	a, (CircleFlg)
; 9509: 		bit	1, a
; 9510: 		call	nz, sub_862B
; 9511: 		call	z, sub_8643
; 9512: 		ret
; 9513: ; ---------------------------------------------------------------------------
; 9514: 
; 9515: loc_8910:				; CODE XREF: PrnCircle_Clip+1EFj
; 9516: 		ld	a, (byte_8622)
; 9517: 		add	a, 80h ; 'А'
; 9518: 		sub	b
; 9519: 		cp	8
; 9520: 		ret	c
; 9521: 		ld	c, 0
; 9522: 		bit	1, e
; 9523: 		jr	nz, loc_8921
; 9524: 		ld	c, 7Fh ; ''
; 9525: 
; 9526: loc_8921:				; CODE XREF: PrnCircle_Clip+22Fj
; 9527: 		call	sub_89EF
; 9528: 		ld	a, l
; 9529: 		and	0E0h ; 'р'
; 9530: 		or	1
; 9531: 		ld	e, a
; 9532: 		ld	a, (FillCircle)
; 9533: 		and	a
; 9534: 		jp	nz, loc_8FCC
; 9535: 		ld	b, 0FFh
; 9536: 		ld	a, (CircleFlg)
; 9537: 		bit	1, a
; 9538: 		call	nz, sub_865A
; 9539: 		call	z, sub_8672
; 9540: 		ret
; 9541: ; ---------------------------------------------------------------------------
; 9542: 
; 9543: loc_893F:				; CODE XREF: PrnCircle_Clip+8Ej
; 9544: 		ld	de, (CircleFlg)
; 9545: 		ld	a, c
; 9546: 		call	sub_8ACD
; 9547: 		ld	(byte_8622), a
; 9548: 		ld	a, c
; 9549: 		exx
; 9550: 		ld	e, a
; 9551: 		ld	a, (byte_8622)
; 9552: 		neg
; 9553: 		ld	c, a
; 9554: 		exx
; 9555: 		xor	a
; 9556: 		bit	1, e
; 9557: 		jr	nz, loc_895B
; 9558: 		ld	a, 7Fh ; ''
; 9559: 
; 9560: loc_895B:				; CODE XREF: PrnCircle_Clip+269j
; 9561: 		ld	(word_8620), a
; 9562: 		ld	c, a
; 9563: 		ld	a, (byte_8622)
; 9564: 		ld	l, a
; 9565: 		ld	a, (word_8620+1)
; 9566: 		sub	l
; 9567: 		call	sub_89D4
; 9568: 		ld	a, (word_8620)
; 9569: 		ld	c, a
; 9570: 		ld	a, (byte_8622)
; 9571: 		ld	l, a
; 9572: 		ld	a, (word_8620+1)
; 9573: 		add	a, l
; 9574: 		call	sub_89EF
; 9575: 		ld	a, (CircleFlg)
; 9576: 		bit	1, a
; 9577: 		call	nz, sub_8689
; 9578: 		call	z, sub_86A1
; 9579: 		ret
; 9580: ; END OF FUNCTION CHUNK	FOR PrnCircle_Clip
; 9581: 
; 9582: ; =============== S U B	R O U T	I N E =======================================
; 9583: 
; 9584: 
; 9585: sub_8985:				; CODE XREF: PrnCircle_Clip+1B2p
; 9586: 					; PrnCircle_Clip+1B8p
; 9587: 
; 9588: ; FUNCTION CHUNK AT 8FDA SIZE 00000009 BYTES
; 9589: 
; 9590: 		call	sub_89C3
; 9591: 		ld	a, (word_8620)
; 9592: 		ld	c, a
; 9593: 		ld	a, 80h ; 'А'
; 9594: 		add	a, b
; 9595: 		sub	d
; 9596: 		call	sub_89D4
; 9597: 		ld	a, (FillCircle)
; 9598: 		and	a
; 9599: 		jp	nz, loc_8FDA
; 9600: 		ld	a, e
; 9601: 		or	1Fh
; 9602: 		ld	l, a
; 9603: 		ld	c, 0
; 9604: 		ret
; 9605: ; End of function sub_8985
; 9606: 
; 9607: 
; 9608: ; =============== S U B	R O U T	I N E =======================================
; 9609: 
; 9610: 
; 9611: sub_89A1:				; CODE XREF: PrnCircle_Clip:loc_88ACp
; 9612: 					; PrnCircle_Clip+1C4p
; 9613: 		call	sub_89C3
; 9614: 		ld	a, (word_8620)
; 9615: 		ld	c, a
; 9616: 		ld	a, 80h ; 'А'
; 9617: 		add	a, d
; 9618: 		sub	b
; 9619: 		call	sub_89EF
; 9620: 		ld	a, l
; 9621: 		and	0E0h ; 'р'
; 9622: 
; 9623: loc_89B2:				; DATA XREF: RAM:8EDFo
; 9624: 		or	1
; 9625: 		ld	e, a
; 9626: 		ld	b, a
; 9627: 		ld	a, (FillCircle)
; 9628: 		and	a
; 9629: 		ld	a, b
; 9630: 		jr	nz, loc_89C0
; 9631: 		ld	b, 0FFh
; 9632: 		ret
; 9633: ; ---------------------------------------------------------------------------
; 9634: 
; 9635: loc_89C0:				; CODE XREF: sub_89A1+1Aj
; 9636: 		ld	b, 80h ; 'А'
; 9637: 		ret
; 9638: ; End of function sub_89A1
; 9639: 
; 9640: 
; 9641: ; =============== S U B	R O U T	I N E =======================================
; 9642: 
; 9643: 
; 9644: sub_89C3:				; CODE XREF: sub_8985p	sub_89A1p
; 9645: 		ld	bc, (word_861E)
; 9646: 		ld	de, (CircleFlg)
; 9647: 		ld	a, d
; 9648: 		neg
; 9649: 		exx
; 9650: 		ld	c, a
; 9651: 		ld	e, 0
; 9652: 		exx
; 9653: 		ret
; 9654: ; End of function sub_89C3
; 9655: 
; 9656: 
; 9657: ; =============== S U B	R O U T	I N E =======================================
; 9658: 
; 9659: 
; 9660: sub_89D4:				; CODE XREF: sub_87D4+12p
; 9661: 					; PrnCircle_Clip:loc_88EEp ...
; 9662: 
; 9663: ; FUNCTION CHUNK AT 8FE3 SIZE 0000000A BYTES
; 9664: 
; 9665: 		ld	b, c
; 9666: 		ld	c, a
; 9667: 		ld	a, b
; 9668: 		call	22B0h
; 9669: 
; 9670: loc_89DA:				; DATA XREF: RAM:8EE1o	sub_8F88w ...
; 9671: 		set	7, h
; 9672: 		ld	b, a
; 9673: 		ld	a, (FillCircle)
; 9674: 		and	a
; 9675: 		ld	a, b
; 9676: 		jp	nz, loc_8FE3
; 9677: 		ld	b, 0FFh
; 9678: 		inc	a
; 9679: 
; 9680: loc_89E8:				; CODE XREF: sub_89D4+17j
; 9681: 		srl	b
; 9682: 		dec	a
; 9683: 		jr	nz, loc_89E8
; 9684: 		ld	e, l
; 9685: 		ret
; 9686: ; End of function sub_89D4
; 9687: 
; 9688: 
; 9689: ; =============== S U B	R O U T	I N E =======================================
; 9690: 
; 9691: 
; 9692: sub_89EF:				; CODE XREF: sub_87D4+1Dj
; 9693: 					; PrnCircle_Clip:loc_8921p ...
; 9694: 
; 9695: ; FUNCTION CHUNK AT 8FED SIZE 0000000C BYTES
; 9696: 
; 9697: 		push	af
; 9698: 		ld	a, b
; 9699: 		ld	(byte_8627), a
; 9700: 		pop	af
