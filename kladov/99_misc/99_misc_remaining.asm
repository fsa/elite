; 99_misc - оставшиеся подпрограммы
; Количество: 105

; === AdrDataShips === AdrDataShips:	db 80h			; DATA XREF: sub_7A9E+19o
AdrDataShips:	db 80h			; DATA XREF: sub_7A9E+19o
					; SetObjInSlot_Introo ...
		dw d_Coriolis		; d_Coriolis
		db 0
		db 84h
		dw d_Missile
		db 0
		db 0B0h
		dw d_EscapePod
		db 1
		db 93h
		dw d_Alloys
		db 1
		db 93h
		dw d_Cargo
		db 1
		db 93h
		dw d_Minerals
		db 1
		db 87h
		dw d_Asteroid2
		db 2
		db 0D0h
		dw d_Asteroid1
		db 2
		db 0C0h
		dw d_Viper
		db 2
		db 0F0h
		dw d_Cobra1
		db 2
		db 0F0h
		dw d_Python1
		db 2
		db 0F0h
		dw d_Python2
		db 3
		db 0F0h
		dw d_Cobra2
		db 3
		db 0FCh
		dw d_ASP
		db 4
		db 0F0h
		dw d_FerDeLance
		db 3
		db 0FFh
		dw d_Targon
		db 3
		db 0C4h
		dw dSidewinderH
		db 4
		db 0D4h
		dw d_Adder
		db 4
		db 0C7h
		dw dTargletH
		db 4
		db 0FFh
		dw stru_6381		; =13 targon6
		db 3
		db 0D0h
		dw stru_6398
		db 2
		db 0D4h
		dw stru_63AF
		db 4
		db 0D4h
		dw d_Krait
		db 4
		db 0

; =============== S U B	R O U T	I N E =======================================




; === d_EscapePod === d_EscapePod:	ShipData <20h, 10h, 0, 4, 6, 4,	8, 11h,	0, 0, 8, 321h, 336h, 348h, 1, \
d_EscapePod:	ShipData <20h, 10h, 0, 4, 6, 4,	8, 11h,	0, 0, 8, 321h, 336h, 348h, 1, \
					; DATA XREF: RAM:6189o
			  4, 0,	32h>


; === d_Alloys === d_Alloys:	ShipData <80h, 0Ah, 0, 4, 4, 0,	10h, 10h, 0, 0,	0Ch, 33Eh, 356h, 0, 0,\
d_Alloys:	ShipData <80h, 0Ah, 0, 4, 4, 0,	10h, 10h, 0, 0,	0Ch, 33Eh, 356h, 0, 0,\
					; DATA XREF: RAM:618Do
			  3, 0,	20h>


; === d_Minerals === d_Minerals:	ShipData <0B0h,	10h, 0,	4, 6, 4, 0Ah, 14h, 0, 0, 0Ah, 394h, 2F1h, \
d_Minerals:	ShipData <0B0h,	10h, 0,	4, 6, 4, 0Ah, 14h, 0, 0, 0Ah, 394h, 2F1h, \
					; DATA XREF: RAM:6195o
			  3ACh,	0, 5, 0, 1Eh>


; === d_Asteroid2 === d_Asteroid2:	ShipData <0, 50h, 0, 9,	15h, 0Eh, 1Eh, 3Ch, 5, 0, 32h, 3A5h, 3D8h, \
d_Asteroid2:	ShipData <0, 50h, 0, 9,	15h, 0Eh, 1Eh, 3Ch, 5, 0, 32h, 3A5h, 3D8h, \
					; DATA XREF: RAM:6199o
			  408h,	0, 1, 0, 2Fh>


; === d_Asteroid1 === d_Asteroid1:	ShipData <7, 50h, 50h, 9, 15h, 0Eh, 1Eh, 0B4h, 0, 53h, 32h, 38Eh, \
d_Asteroid1:	ShipData <7, 50h, 50h, 9, 15h, 0Eh, 1Eh, 0B4h, 0, 53h, 32h, 38Eh, \
					; DATA XREF: RAM:619Do
			  3C1h,	3F1h, 0A1h, 1, 0, 12Fh>


; === d_Viper === d_Viper:	ShipData <0, 4Bh, 78h, 0Fh, 14h, 7, 20h, 8Ch, 0, 2, 17h, 40Fh, 44Eh, \
d_Viper:	ShipData <0, 4Bh, 78h, 0Fh, 14h, 7, 20h, 8Ch, 0, 2, 17h, 40Fh, 44Eh, \
					; DATA XREF: RAM:61A1o
			  47Eh,	0C2h, 1, 0, 50h>


; === d_Cobra1 === d_Cobra1:	ShipData <3, 5Fh, 8Ch, 1Ch, 26h, 0Dh, 1Ch, 96h,	0, 4, 32h, 477h, 501h,\
d_Cobra1:	ShipData <3, 5Fh, 8Ch, 1Ch, 26h, 0Dh, 1Ch, 96h,	0, 4, 32h, 477h, 501h,\
					; DATA XREF: RAM:61A5o
			  554h,	0A0h, 1, 15h, 280h>


; === d_Python1 === d_Python1:	ShipData <5, 50h, 82h, 0Bh, 1Ah, 0Dh, 14h, 0FAh, 0, 13h, 28h, 565h, \
d_Python1:	ShipData <5, 50h, 82h, 0Bh, 1Ah, 0Dh, 14h, 0FAh, 0, 13h, 28h, 565h, \
					; DATA XREF: RAM:61A9o
			  598h,	5D4h, 0A0h, 0, 8, 280h>


; === d_Python2 === d_Python2:	ShipData <2, 50h, 96h, 0Bh, 1Ah, 0Dh, 14h, 0FAh, 0C8h, 53h, 28h, 54Eh,\
d_Python2:	ShipData <2, 50h, 96h, 0Bh, 1Ah, 0Dh, 14h, 0FAh, 0C8h, 53h, 28h, 54Eh,\
					; DATA XREF: RAM:61ADo
			  581h,	5BDh, 8Ch, 0, 8, 380h>


; === d_Cobra2 === d_Cobra2:	ShipData <1, 5Fh, 0A0h,	1Ch, 26h, 0Dh, 1Ch, 96h, 0AFh, 2, 32h, 432h, \
d_Cobra2:	ShipData <1, 5Fh, 0A0h,	1Ch, 26h, 0Dh, 1Ch, 96h, 0AFh, 2, 32h, 432h, \
					; DATA XREF: RAM:61B1o
			  4BCh,	50Fh, 8Ch, 1, 15h, 380h>


; === d_ASP === d_ASP:		ShipData <0, 3Ch, 0FAh,	13h, 1Ch, 0Ch, 28h, 96h, 0C8h, 1, 18h, 5A8h, \
d_ASP:		ShipData <0, 3Ch, 0FAh,	13h, 1Ch, 0Ch, 28h, 96h, 0C8h, 1, 18h, 5A8h, \
					; DATA XREF: RAM:61B5o
			  602h,	647h, 8Ch, 1, 5, 3C0h>


; === d_FerDeLance === d_FerDeLance:	ShipData <0, 28h, 0FFh,	13h, 1Bh, 0Ah, 1Eh, 0A0h, 0, 2,	1Eh, 654h, \
d_FerDeLance:	ShipData <0, 28h, 0FFh,	13h, 1Bh, 0Ah, 1Eh, 0A0h, 0, 2,	1Eh, 654h, \
					; DATA XREF: RAM:61B9o
			  6ABh,	6F2h, 82h, 1, 0, 380h>


; === dSidewinderH === dSidewinderH:	ShipData <0, 41h, 82h, 0Ah, 0Fh, 7, 25h, 46h, 32h, 1, 16h, 786h, 7B0h,\
dSidewinderH:	ShipData <0, 41h, 82h, 0Ah, 0Fh, 7, 25h, 46h, 32h, 1, 16h, 786h, 7B0h,\
					; DATA XREF: sub_5EC5+22o RAM:61C1o
			  7D5h,	0Ch, 2,	0, 100h>


; === d_Adder === d_Adder:	db 0			; field_0 ; DATA XREF: RAM:61C5o
d_Adder:	db 0			; field_0 ; DATA XREF: RAM:61C5o
					; dCargo
		db 32h			; field_1
					; dCollision
		db 64h			; field_2
					; dChanceFire
		db 12h			; field_3
					; dVertexN
		db 1Dh			; field_4
					; dEdgeN
		db 0Dh			; field_5
					; dNormalN
		db 18h			; field_6
					; dMaxSpeed
		db 55h			; field_7
					; dEnergy
		dw 28h			; field_8
					; dCash
		db 1			; field_A
					; dFlg_1
		db 14h			; field_B
					; dSize
		dw 7CEh			; field_C
					; dVertex
		dw 819h			; field_E
					; dEdge
		dw 85Ch			; field_10
					; dNormal
		db 8Ch			; field_12
					; dFlg_2
		db 2			; field_13
					; dFactor
		db 0			; field_14
					; dLaser
		dw 140h			; field_15
					; dRating


; === dTargletH === dTargletH:	ShipData <0F0h,	28h, 78h, 0Ah, 0Fh, 7, 1Eh, 14h, 32h, 1, 14h, 861h, \
dTargletH:	ShipData <0F0h,	28h, 78h, 0Ah, 0Fh, 7, 1Eh, 14h, 32h, 1, 14h, 861h, \
					; DATA XREF: sub_5EC5+41o RAM:61C9o
			  232h,	891h, 4, 2, 5, 64h>


; === stru_6381 === stru_6381:	ShipData <0, 63h, 0FFh,	0Eh, 13h, 8, 2Fh, 0F0h,	258h, 36h, 37h,	88Dh, \
stru_6381:	ShipData <0, 63h, 0FFh,	0Eh, 13h, 8, 2Fh, 0F0h,	258h, 36h, 37h,	88Dh, \
					; DATA XREF: RAM:61CDo
			  8DAh,	8C6h, 0Ch, 2, 6, 990h>


; === stru_6398 === stru_6398:	ShipData <0, 28h, 0FEh,	1Dh, 1Fh, 9, 27h, 90h, 0FAh, 2,	17h, 8F0h, \
stru_6398:	ShipData <0, 28h, 0FEh,	1Dh, 1Fh, 9, 27h, 90h, 0FAh, 2,	17h, 8F0h, \
					; DATA XREF: RAM:61D1o
			  983h,	96Bh, 8Ch, 1, 0, 480h>


; === stru_63AF === stru_63AF:	ShipData <2, 1Eh, 98h, 18h, 1Eh, 0Ch, 22h, 8Ch,	64h, 2,	17h, 9B4h, \
stru_63AF:	ShipData <2, 1Eh, 98h, 18h, 1Eh, 0Ch, 22h, 8Ch,	64h, 2,	17h, 9B4h, \
					; DATA XREF: RAM:61D5o
			  0A2Fh, 0A17h,	8Ch, 1,	16h, 14Fh>


; === d_Coriolis === d_Coriolis:	ShipData <0, 0A0h, 0, 10h, 1Ch,	0Eh, 0,	0F0h, 0, 6, 78h, 2Eh, 6Ah, \
d_Coriolis:	ShipData <0, 0A0h, 0, 10h, 1Ch,	0Eh, 0,	0F0h, 0, 6, 78h, 2Eh, 6Ah, \
					; DATA XREF: RAM:606Bo	RAM:6181o
			  0A8h,	0, 0, 0, 0> ; d_Coriolis


; === d_Krait === d_Krait:	ShipData <1, 3Ch, 6Eh, 11h, 15h, 6, 1Eh, 50h, 14h, 0, 14h, 0A49h, \
d_Krait:	ShipData <1, 3Ch, 6Eh, 11h, 15h, 6, 1Eh, 50h, 14h, 0, 14h, 0A49h, \
					; DATA XREF: RAM:61D9o
			  0AA3h, 0A97h,	8Ch, 1,	0, 140h>
		db  1Fh			; вершины кориолис
		db    8
		db 0C1h	; ┴
		db  3Eh	; >
		db  5Ch	; \
		db  96h	; Ц
		db  9Eh	; Ю
		db  28h	; (
		db 0D3h	; ╙
		db 0BEh	; ╛
		db  4Dh	; M
		db  97h	; Ч
		db  5Fh	; _
		db 0A0h	; а
		db  5Ch	; \
		db  1Eh
		db 0C1h	; ┴
		db  6Bh	; k
		db  9Eh	; Ю
		db 0D2h	; ╥
		db  7Bh	; {
		db 0DEh	; ▐
		db 0A3h	; г
		db  4Dh	; M
		db  1Fh
		db  18h
		db 0B2h	; ▓
		db  3Eh	; >
		db  6Bh	; k
		db  97h	; Ч
		db  5Eh	; ^
		db    8
		db 0A3h	; г
		db  7Eh	; ~
		db  4Ah	; J
		db  95h	; Х
		db  5Fh	; _
		db  88h	; И
		db  88h	; И
		db  1Eh
		db  88h	; И
		db  88h	; И
		db  9Eh	; Ю
		db  88h	; И
		db  88h	; И
		db 0DEh	; ▐
		db  88h	; И
		db  88h	; И
		db 0A0h	; а
		db    0
		db 0A0h	; а
		db 0A0h	; а
		db 0A0h	; а
		db    0
		db    0
		db 0A0h	; а
		db 0A0h	; а
		db  0Ah
		db  1Eh
		db 0A0h	; а
		dw unk_644E		; ребра	кориолис
		dw unk_6452
		db    8
		db  18h
		db  28h	; (
		db  38h	; 8
		db 0A0h	; а
		db 0C0h	; └
		db 0C1h	; ┴
		db 0B1h	; ▒
		db 0B2h	; ▓
		db 0D2h	; ╥
		db 0D3h	; ╙
		db 0A3h	; г
		db  95h	; Х
		db  96h	; Ц
		db  97h	; Ч
		db  94h	; Ф
		db  5Ah	; Z
		db  5Ch	; \
		db  6Ch	; l
		db  6Bh	; k
		db  7Bh	; {
		db  7Dh	; }
		db  4Dh	; M
		db  4Ah	; J
		db  88h	; И
		db 0FFh
unk_644E:	db  18h			; DATA XREF: RAM:6430o
		db  3Fh	; ?
		db    4
		db  3Eh	; >
unk_6452:	db  0Ah			; DATA XREF: RAM:6432o
		db    8
		db  82h	; В
		db  2Ah	; *
		db  4Ah	; J
		db    4
		db    5
		db  58h	; X
		db  86h	; Ж
		db  26h	; &
		db  27h	; '
		db  7Ah	; z
		db  1Bh
		db  91h	; С
		db  39h	; 9
		db  3Bh	; ;
		db  4Bh	; K
		db  41h	; A
		db  51h	; Q
		db  59h	; Y
		db  69h	; i
		db  63h	; c
		db  73h	; s
		db  7Bh	; {
		db 0CDh	; ═
		db 0DEh	; ▐
		db 0EFh	; я
		db 0FCh	; №
		db  5Fh	; _		; нормали кориолис
		db  1Eh
		db  9Eh	; Ю
		db 0DEh	; ▐
		db 0FEh	; ■
		db  7Eh	; ~
		db  3Eh	; >
		db 0BEh	; ╛
		db  1Fh
		db  3Eh	; >
		db  5Fh	; _
		db  1Eh
		db  1Fh
		db  9Eh	; Ю
		db  6Bh	; k
		db  6Bh	; k
		db  6Bh	; k
		db    0
		db    0
		db 0A0h	; а
		db    0
		db 0A0h	; а
		db    0
		db 0A0h	; а
		db    0
		db    0
		db  1Fh
		db  20h
		db  31h	; 1
		db  5Fh	; _
		db  12h
		db  64h	; d
		db  1Eh
		db  31h	; 1
		db  74h	; t
		db  9Eh	; Ю
		db  30h	; 0
		db  76h	; v
		db 0DEh	; ▐
		db  20h
		db  56h	; V
		db  3Fh	; ?
		db  74h	; t
		db  88h	; И
		db  7Eh	; ~
		db  64h	; d
		db  88h	; И
		db 0FEh	; ■
		db  56h	; V
		db  88h	; И
		db 0BEh	; ╛
		db  75h	; u
		db  88h	; И
		db  29h	; )
		db  74h	; t
		db  88h	; И
		db  68h	; h
		db  64h	; d
		db  88h	; И
		db 0E8h	; ш
		db  56h	; V
		db  88h	; И
		db 0A8h	; и
		db  75h	; u
		db  88h	; И
		db 0A9h	; й
		db  75h	; u
		db  77h	; w
		db 0E8h	; ш
		db  56h	; V
		db  55h	; U
		db  28h	; (
		db  74h	; t
		db  77h	; w
		db  68h	; h
		db  64h	; d
		db  66h	; f
		db    0
		db    0
		db  44h	; D
		db    8
		db    8
		db  24h	; $
		db    8
		db    8
		db  2Ch	; ,
		db  0Ch
		db  0Ch
		db  2Ch	; ,
		db    8
		db    8
		db  0Ch
		db 0E6h	; ц
		db  64h	; d
		db 0EBh	; ы
		db  64h	; d
		db  12h
		db  13h
		db    3
		db    2
		db  14h
		db  26h	; &
		db  46h	; F
		db  37h	; 7
		db  47h	; G
		db    5
		db  56h	; V
		db  48h	; H
		db  78h	; x
		db  57h	; W
		db  68h	; h
		db  58h	; X
		db  78h	; x
		db  68h	; h
		db  68h	; h
		db  78h	; x
		db  47h	; G
		db  56h	; V
		db  75h	; u
		db  46h	; F
		db  10h
		db  3Fh	; ?
		db    7
		db  29h	; )
		db 0A9h	; й
		db    1
		db    2
		db    3
		db    4
		db  12h
		db  14h
		db  16h
		db  23h	; #
		db  25h	; %
		db  34h	; 4
		db  47h	; G
		db  56h	; V
		db  58h	; X
		db  38h	; 8
		db  67h	; g
		db  78h	; x
		db  59h	; Y
		db  6Ah	; j
		db  7Bh	; {
		db  8Ch	; М
		db  9Fh	; Я
		db 0BEh	; ╛
		db 0CDh	; ═
		db  0Ah
		db  9Fh	; Я
		db  1Eh
		db  5Fh	; _
		db  1Eh
		db  1Fh
		db  9Eh	; Ю
		db  5Fh	; _
		db  1Eh
		db  3Fh	; ?
		db  40h	; @
		db    0
		db  10h
		db    0
		db  40h	; @
		db  10h
		db  20h
		db    0
		db    0
		db    0
		db  20h
		db    0
		db    0
		db    0
		db 0B0h	; ░
		db  9Fh	; Я
		db  12h
		db  33h	; 3
		db 0FFh
		db    2
		db  33h	; 3
		db 0BEh	; ╛
		db    1
		db  33h	; 3
		db  1Fh
		db    1
		db  22h	; "
		db    7
		db    0
		db  24h	; $
		db    7
		db  0Eh
		db  0Ch
		db  15h
		db    0
		db    0
		db  3Ah	; :
		db  65h	; e
		db  3Ch	; <
		db  65h	; e
		db  23h	; #
		db    3
		db    1
		db  12h
		db  13h
		db    2
		db    6
		db  3Fh	; ?
		db    1
		db  12h
		db  23h	; #
		db    3
		db    2
		db  13h
		db  3Fh	; ?
		db  1Fh
		db  5Eh	; ^
		db  9Fh	; Я
		db  34h	; 4
		db    0
		db  7Ah	; z
		db  27h	; '
		db  67h	; g
		db  1Eh
		db  70h	; p
		db    0
		db    0
		db 0FFh
		db 0FFh
		db 0FFh
		db 0BFh	; ┐
		db 0FFh
		db 0FFh
		db  15h
		db 0FFh
		db 0FFh
		db  55h	; U
		db 0FFh
		db 0FFh
		db  0Fh
		db  16h
		db    9
		db  0Fh
		db  26h	; &
		db    9
		db  13h
		db  20h
		db  0Bh
		db  0Ah
		db  2Eh	; .
		db    6
		db  6Dh	; m
		db  65h	; e
		db  71h	; q
		db  65h	; e
		db 0FFh
		db 0FFh
		db  3Fh	; ?
		db  31h	; 1
		db  35h	; 5
		db  31h	; 1
		db    1
		db  12h
		db  23h	; #
		db    3
		db 0DFh	; ▀
		db  23h	; #
		db  11h
		db 0FEh	; ■
		db  42h	; B
		db  11h
		db  5Eh	; ^
		db  30h	; 0
		db  22h	; "
		db  7Eh	; ~
		db  20h
		db  44h	; D
		db  1Fh
		db  60h	; `
		db  55h	; U
		db  9Eh	; Ю
		db  56h	; V
		db  11h
		db  9Fh	; Я
		db  36h	; 6
		db  11h
		db 0BEh	; ╛
		db  54h	; T
		db  11h
		db  1Eh
		db  60h	; `
		db  33h	; 3
		db  3Eh	; >
		db  40h	; @
		db  55h	; U
		db  18h
		db  0Dh
		db    9
		db  18h
		db  10h
		db    0
		db  18h
		db    5
		db  0Fh
		db 0AFh	; п
		db  65h	; e
		db 0B1h	; ▒
		db  65h	; e
		db    6
		db    3
		db    2
		db    4
		db    5
		db  56h	; V
		db  36h	; 6
		db  23h	; #
		db  24h	; $
		db  45h	; E
		db  16h
		db  13h
		db  12h
		db  14h
		db  15h
		db  0Fh
		db  3Fh	; ?
		db  48h	; H
		db  28h	; (
		db  23h	; #
		db  39h	; 9
		db  49h	; I
		db  45h	; E
		db  68h	; h
		db    2
		db  13h
		db  79h	; y
		db  56h	; V
		db    6
		db    1
		db  17h
		db  57h	; W
		db  1Fh
		db  9Eh	; Ю
		db  5Fh	; _
		db  5Fh	; _
		db  7Eh	; ~
		db  3Fh	; ?
		db  1Eh
		db  60h	; `
		db    0
		db    0
		db    0
		db  33h	; 3
		db    0
		db    0
		db  12h
		db  30h	; 0
		db    0
		db  29h	; )
		db  1Eh
		db 0DFh	; ▀
		db  12h
		db  33h	; 3
		db  3Fh	; ?
		db    2
		db  33h	; 3
		db  5Fh	; _
		db    1
		db  33h	; 3
		db  1Fh
		db    1
		db  22h	; "
		db  18h
		db  19h
		db  10h
		db    0
		db  0Ch
		db  0Ah
		db  0Bh
		db    6
		db    2
		db  0Ch
		db  2Ah	; *
		db    7
		db  3Fh	; ?
		db  5Fh	; _
		db 0BFh	; ┐
		db 0FFh
		db 0B1h	; ▒
		db  0Ch
		db  94h	; Ф
		db  64h	; d
		db  19h
		db 0D5h	; ╒
		db 0CAh	; ╩
		db  68h	; h
		db  22h	; "
		db    2
		db  33h	; 3
		db  4Ah	; J
		db  1Fh
		db 0FFh
		db 0FFh
		db  5Eh	; ^
		db 0FFh
		db 0FFh
		db 0DFh	; ▀
		db 0FFh
		db 0FFh
		db  5Fh	; _
		db 0FFh
		db 0FFh
		db  1Fh
		db  65h	; e
		db 0DCh	; ▄
		db  1Fh
		db 0FFh
		db 0FFh
		db  9Fh	; Я
		db  10h
		db  32h	; 2
		db  3Fh	; ?
		db 0FFh
		db 0FFh
		db  7Fh	; 
		db  98h	; Ш
		db 0BAh	; ║
		db    0
		db  50h	; P
		db    0
		db  50h	; P
		db  0Ah
		db    0
		db  46h	; F
		db  28h	; (
		db    0
		db  3Ch	; <
		db  32h	; 2
		db    0
		db  32h	; 2
		db    0
		db  3Ch	; <
		db  28h	; (
		db    0
		db  46h	; F
		db    0
		db  1Eh
		db  4Bh	; K
		db    0
		db  32h	; 2
		db  3Ch	; <
		db  47h	; G
		db  66h	; f
		db  49h	; I
		db  66h	; f
		db  27h	; '
		db  6Dh	; m
		db  5Ch	; \
		db  4Bh	; K
		db  3Ah	; :
		db  23h	; #
		db  13h
		db  14h
		db    1
		db    6
		db  45h	; E
		db    2
		db  56h	; V
		db  8Ah	; К
		db  78h	; x
		db  7Dh	; }
		db 0CDh	; ═
		db  9Ch	; Ь
		db  9Bh	; Ы
		db 0ABh	; л
		db  89h	; Й
		db  15h
		db  3Fh	; ?
		db    2
		db    4
		db  34h	; 4
		db  13h
		db  12h
		db  26h	; &
		db  16h
		db  15h
		db  56h	; V
		db    5
		db  35h	; 5
		db    6
		db  45h	; E
		db  28h	; (
		db  27h	; '
		db    7
		db  47h	; G
		db  37h	; 7
		db  38h	; 8
		db  18h
		db  78h	; x
		db  1Fh
		db  5Eh	; ^
		db  9Fh	; Я
		db 0DFh	; ▀
		db  5Fh	; _
		db  1Fh
		db  1Fh
		db 0BFh	; ┐
		db 0FFh
		db  7Fh	; 
		db 0FFh
		db  7Fh	; 
		db  3Fh	; ?
		db  3Fh	; ?
		db    9
		db  42h	; B
		db  51h	; Q
		db  48h	; H
		db  40h	; @
		db  1Fh
		db  40h	; @
		db  49h	; I
		db  2Fh	; /
		db  2Dh	; -
		db  4Fh	; O
		db  41h	; A
		db  87h	; З
		db  0Fh
		db  23h	; #
		db  26h	; &
		db  4Ch	; L
		db  46h	; F
		db  42h	; B
		db  3Bh	; ;
		db  27h	; '
		db  43h	; C
		db  0Fh
		db  50h	; P
		db  42h	; B
		db  0Eh
		db  4Bh	; K
		db  46h	; F
		db  50h	; P
		db  28h	; (
		db  3Ah	; :
		db  66h	; f
		db  33h	; 3
		db  51h	; Q
		db    9
		db  43h	; C
		db  2Fh	; /
		db  5Eh	; ^
		db  3Fh	; ?
		db  1Fh
		db  21h	; !
		db 0FFh
		db  1Fh
		db  16h
		db  22h	; "
		db  5Eh	; ^
		db  43h	; C
		db  55h	; U
		db  3Fh	; ?
		db  42h	; B
		db 0FFh
		db 0BEh	; ╛
		db  31h	; 1
		db    0
		db  7Fh	; 
		db  54h	; T
		db    0
		db 0FEh	; ■
		db  35h	; 5
		db    0
		db  3Eh	; >
		db  26h	; &
		db    0
		db 0BEh	; ╛
		db  16h
		db    0
		db 0B3h	; │
		db    0
		db    0
		db  32h	; 2
		db    0
		db    0
		db  33h	; 3
		db    0
		db    0
		db 0B2h	; ▓
		db    0
		db    0
		db 0F2h	; Є
		db    0
		db    0
		db  72h	; r
		db    0
		db    0
		db    0
		db    0
		db  48h	; H
		db    0
		db  10h
		db  18h
		db  30h	; 0
		db    0
		db  18h
		db  18h
		db  10h
		db  18h
		db  20h
		db    0
		db  18h
		db    8
		db    8
		db  18h
		db 0E6h	; ц
		db  66h	; f
		db 0EEh	; ю
		db  66h	; f
		db  24h	; $
		db  13h
		db    6
		db    1
		db    2
		db  12h
		db  34h	; 4
		db  26h	; &
		db  16h
		db  45h	; E
		db  35h	; 5
		db    5
		db    3
		db    4
		db    0
		db 0FFh
		db    5
		db  3Fh	; ?
		db    9
		db  3Eh	; >
		db    4
		db  33h	; 3
		db    2
		db  30h	; 0
		db    3
		db    4
		db  78h	; x
		db  48h	; H
		db  37h	; 7
		db    1
		db    2
		db  17h
		db  18h
		db  25h	; %
		db  26h	; &
		db  56h	; V
		db  46h	; F
		db  35h	; 5
		db  9Ch	; Ь
		db  9Dh	; Э
		db 0ABh	; л
		db 0AEh	; о
		db 0BEh	; ╛
		db 0CDh	; ═
		db  3Fh	; ?
		db  9Fh	; Я
		db  1Eh
		db 0DEh	; ▐
		db  5Eh	; ^
		db  5Fh	; _
		db  1Eh
		db    0
		db    0
		db  30h	; 0
		db  16h
		db  21h	; !
		db  0Bh
		db    0
		db  20h
		db    0
		db  1Fh
		db 0FFh
		db 0FFh
		db  9Eh	; Ю
		db 0FFh
		db 0FFh
		db  1Fh
		db 0FFh
		db 0FFh
		db 0FFh
		db  73h	; s
		db 0FFh
		db  7Eh	; ~
		db  84h	; Д
		db 0CCh	; ╠
		db 0BFh	; ┐
		db 0FFh
		db 0FFh
		db  3Eh	; >
		db 0FFh
		db 0FFh
		db  7Fh	; 
		db  98h	; Ш
		db 0CCh	; ╠
		db 0FEh	; ■
		db  97h	; Ч
		db 0AAh	; к
		db  3Fh	; ?
		db  65h	; e
		db  99h	; Щ
		db 0FFh
		db 0A9h	; й
		db 0BBh	; ╗
		db  7Eh	; ~
		db 0B9h	; ╣
		db 0CCh	; ╠
		db 0B5h	; ╡
		db  99h	; Щ
		db  99h	; Щ
		db 0B5h	; ╡
		db  99h	; Щ
		db  99h	; Щ
		db  34h	; 4
		db  99h	; Щ
		db  99h	; Щ
		db  35h	; 5
		db  99h	; Щ
		db  99h	; Щ
		db  75h	; u
		db  99h	; Щ
		db  99h	; Щ
		db  75h	; u
		db  99h	; Щ
		db  99h	; Щ
		db 0F4h	; Ї
		db  99h	; Щ
		db  99h	; Щ
		db 0F5h	; ї
		db  99h	; Щ
		db  99h	; Щ
		db    7
		db    0
		db  0Bh
		db  1Fh
		db    0
		db 0FFh
		db 0E9h	; щ
		db  99h	; Щ
		db  99h	; Щ
		db 0A8h	; и
		db  99h	; Щ
		db  99h	; Щ
		db 0A7h	; з
		db  99h	; Щ
		db  99h	; Щ
		db  26h	; &
		db  99h	; Щ
		db  99h	; Щ
		db  29h	; )
		db  99h	; Щ
		db  99h	; Щ
		db  68h	; h
		db  99h	; Щ
		db  99h	; Щ
		db  20h
		db    0
		db  4Ch	; L
		db    0
		db  1Ah
		db  18h
		db  78h	; x
		db    3
		db    8
		db  58h	; X
		db  10h
		db  28h	; (
		db  80h	; А
		db    8
		db  28h	; (
		db    0
		db  1Ah
		db  28h	; (
		db  20h
		db  18h
		db  28h	; (
		db  24h	; $
		db    8
		db  28h	; (
		db    8
		db  0Ch
		db  28h	; (
		db  24h	; $
		db    8
		db  28h	; (
		db  24h	; $
		db  0Ch
		db  28h	; (
		db    8
		db  10h
		db  28h	; (
		db  24h	; $
		db  0Ch
		db  28h	; (
		db    0
		db    0
		db  4Ch	; L
		db    0
		db    0
		db  5Ah	; Z
		db  50h	; P
		db    6
		db  28h	; (
		db  58h	; X
		db    0
		db  28h	; (
		db  50h	; P
		db    6
		db  28h	; (
		db 0BAh	; ║
		db  67h	; g
		db 0C9h	; ╔
		db  67h	; g
		db  0Bh
		db  4Ch	; L
		db  3Ah	; :
		db  7Ah	; z
		db  8Ch	; М
		db  89h	; Й
		db  69h	; i
		db  59h	; Y
		db  79h	; y
		db  15h
		db  26h	; &
		db  37h	; 7
		db  48h	; H
		db    1
		db    2
		db  9Ah	; Ъ
		db  9Bh	; Ы
		db  9Ch	; Ь
		db 0ABh	; л
		db 0BCh	; ╝
		db  13h
		db  24h	; $
		db  56h	; V
		db  0Bh
		db  99h	; Щ
		db 0FFh
		db  14h
		db  3Fh	; ?
		db    3
		db  3Dh	; =
		db    5
		db 0E7h	; ч
		db    2
		db  35h	; 5
		db    2
		db 0F5h	; ї
		db 0B1h	; ▒
		db    3
		db 0B3h	; │
		db    2
		db 0E9h	; щ
		db    1
		db    4
		db  13h
		db  38h	; 8
		db  47h	; G
		db  67h	; g
		db  69h	; i
		db  59h	; Y
		db  58h	; X
		db  25h	; %
		db  26h	; &
		db  35h	; 5
		db  46h	; F
		db  12h
		db    2
		db  8Ah	; К
		db 0ABh	; л
		db  7Bh	; {
		db  1Ah
		db  0Bh
		db  15h
		db    6
		db  29h	; )
		db  45h	; E
		db  68h	; h
		db  78h	; x
		db  9Ah	; Ъ
		db  9Bh	; Ы
		db 0CDh	; ═
		db 0EFh	; я
		db    1
		db  23h	; #
		db  1Eh
		db  0Fh
		db  2Dh	; -
		db  3Ch	; <
		db  67h	; g
		db 0ABh	; л
		db  1Fh
		db  9Fh	; Я
		db  1Eh
		db  9Fh	; Я
		db  1Eh
		db  9Fh	; Я
		db  1Eh
		db  9Fh	; Я
		db  1Eh
		db  3Fh	; ?
		db 0DFh	; ▀
		db  5Fh	; _
		db  5Fh	; _
		db    0
		db  3Eh	; >
		db  1Fh
		db  12h
		db  37h	; 7
		db  10h
		db  10h
		db  34h	; 4
		db  0Eh
		db  0Eh
		db  2Fh	; /
		db    0
		db  3Dh	; =
		db  66h	; f
		db    0
		db    0
		db    0
		db  50h	; P
		db    7
		db  2Ah	; *
		db    9
		db    0
		db  1Eh
		db    6
		db    7
		db  2Ah	; *
		db    9
		db  5Fh	; _
		db  32h	; 2
		db  76h	; v
		db  1Eh
		db  10h
		db  54h	; T
		db  3Fh	; ?
		db 0FFh
		db 0FFh
		db 0BEh	; ╛
		db 0FFh
		db 0FFh
		db  3Fh	; ?
		db  54h	; T
		db  98h	; Ш
		db  7Eh	; ~
		db  76h	; v
		db 0BAh	; ║
		db 0BFh	; ┐
		db 0B8h	; ╕
		db 0CCh	; ╠
		db  3Eh	; >
		db 0A9h	; й
		db 0CCh	; ╠
		db  1Fh
		db  10h
		db 0FFh
		db  3Fh	; ?
		db  89h	; Й
		db 0CCh	; ╠
		db  7Eh	; ~
		db 0BAh	; ║
		db 0CCh	; ╠
		db    0
		db  30h	; 0
		db  30h	; 0
		db  60h	; `
		db    0
		db  10h
		db    0
		db  30h	; 0
		db  20h
		db  30h	; 0
		db    0
		db  70h	; p
		db    0
		db    0
		db 0E0h	; р
		db    0
		db  18h
		db  70h	; p
		db  68h	; h
		db  68h	; h
		db  6Ch	; l
		db  68h	; h
		db  23h	; #
		db    2
		db  13h
		db    1
		db  59h	; Y
		db  15h
		db  37h	; 7
		db    4
		db  26h	; &
		db  7Ah	; z
		db  48h	; H
		db  6Bh	; k
		db  9Ah	; Ъ
		db  8Bh	; Л
		db  8Ch	; М
		db  9Ch	; Ь
		db 0ACh	; м
		db 0BCh	; ╝
		db  89h	; Й
		db 0ABh	; л
		db  45h	; E
		db  67h	; g
		db  88h	; И
		db 0BBh	; ╗
		db  99h	; Щ
		db 0AAh	; к
		db  16h
		db  3Fh	; ?
		db    4
		db  29h	; )
		db    8
		db  38h	; 8
		db  28h	; (
		db  18h
		db  24h	; $
		db  12h
		db    2
		db  13h
		db    3
		db  25h	; %
		db  34h	; 4
		db  35h	; 5
		db  27h	; '
		db  36h	; 6
		db  69h	; i
		db  79h	; y
		db  7Ah	; z
		db  6Ah	; j
		db  49h	; I
		db  5Ah	; Z
		db  14h
		db    5
		db  39h	; 9
		db  3Ah	; :
		db  29h	; )
		db  2Ah	; *
		db  9Fh	; Я
		db  1Eh
		db 0DEh	; ▐
		db  5Eh	; ^
		db  9Fh	; Я
		db  1Eh
		db 0DEh	; ▐
		db  5Eh	; ^
		db 0BFh	; ┐
		db  3Eh	; >
		db  7Eh	; ~
		db 0FEh	; ■
		db  3Fh	; ?
		db  1Bh
		db  28h	; (
		db  0Bh
		db  13h
		db  26h	; &
		db    0
		db  19h
		db  25h	; %
		db  0Bh
		db    0
		db    0
		db  70h	; p
		db  3Fh	; ?
		db  35h	; 5
		db  6Bh	; k
		db  7Eh	; ~
		db  12h
		db 0BBh	; ╗
		db  3Fh	; ?
		db  16h
		db 0BBh	; ╗
		db 0BEh	; ╛
		db  25h	; %
		db 0FFh
		db 0DFh	; ▀
		db    4
		db  8Ah	; К
		db  5Eh	; ^
		db    4
		db 0FFh
		db 0DFh	; ▀
		db  25h	; %
		db  8Ah	; К
		db  5Eh	; ^
		db  16h
		db  79h	; y
		db 0DFh	; ▀
		db    2
		db  88h	; И
		db  5Eh	; ^
		db    1
		db  77h	; w
		db  1Fh
		db  34h	; 4
		db  69h	; i
		db  9Eh	; Ю
		db  34h	; 4
		db  5Ah	; Z
		db  57h	; W
		db    1
		db  22h	; "
		db 0ABh	; л
		db 0BBh	; ╗
		db 0BBh	; ╗
		db  28h	; (
		db 0BBh	; ╗
		db 0BBh	; ╗
		db  6Bh	; k
		db 0BBh	; ╗
		db 0BBh	; ╗
		db  28h	; (
		db 0BBh	; ╗
		db 0BBh	; ╗
		db  4Bh	; K
		db    4
		db    4
		db  4Bh	; K
		db    4
		db    4
		db    0
		db    9
		db  2Dh	; -
		db  2Bh	; +
		db    0
		db  2Dh	; -
		db  1Ah
		db    7
		db  49h	; I
		db  45h	; E
		db    3
		db    0
		db  2Bh	; +
		db  0Eh
		db  1Ch
		db  2Bh	; +
		db  0Eh
		db  1Ch
		db    0
		db  12h
		db    0
		db  11h
		db    0
		db  2Dh	; -
		db    0
		db    4
		db  2Dh	; -
		db    0
		db    7
		db  49h	; I
		db    0
		db    7
		db  53h	; S
		db  17h
		db  69h	; i
		db  22h	; "
		db  69h	; i
		db  36h	; 6
		db  35h	; 5
		db  12h
		db    2
		db    1
		db  34h	; 4
		db  6Bh	; k
		db  5Bh	; [
		db  1Bh
		db  2Bh	; +
		db  16h
		db  25h	; %
		db    4
		db    8
		db  4Ah	; J
		db    7
		db  49h	; I
		db  28h	; (
		db  5Ah	; Z
		db  17h
		db  69h	; i
		db  8Ah	; К
		db  79h	; y
		db    4
		db 0BBh	; ╗
		db 0FFh
		db    6
		db  37h	; 7
		db  0Fh
		db  3Fh	; ?
		db    2
		db  31h	; 1
		db 0EBh	; ы
		db  2Bh	; +
		db  29h	; )
		db 0A8h	; и
		db 0A9h	; й
		db  0Ah
		db  0Bh
		db  1Ch
		db  8Ch	; М
		db  9Ch	; Ь
		db 0ABh	; л
		db    2
		db    3
		db  12h
		db  13h
		db  27h	; '
		db  36h	; 6
		db  45h	; E
		db  48h	; H
		db  4Bh	; K
		db  59h	; Y
		db  5Ah	; Z
		db  68h	; h
		db  6Bh	; k
		db  79h	; y
		db  7Ah	; z
		db  46h	; F
		db  57h	; W
		db  12h
		db 0DFh	; ▀
		db 0EFh	; я
		db  0Eh
		db  0Dh
		db  5Fh	; _
		db  7Fh	; 
		db 0FEh	; ■
		db  37h	; 7
		db  1Fh
		db 0BFh	; ┐
		db  3Eh	; >
		db  5Fh	; _
		db 0DEh	; ▐
		db  1Fh
		db  9Eh	; Ю
		db  3Fh	; ?
		db    0
		db  23h	; #
		db    5
		db    8
		db  26h	; &
		db    7
		db    0
		db  18h
		db    1
		db    0
		db  2Bh	; +
		db  13h
		db    6
		db  1Ch
		db    2
		db  3Bh	; ;
		db  40h	; @
		db  1Fh
		db  50h	; P
		db  2Eh	; .
		db  32h	; 2
		db    0
		db    0
		db  5Ah	; Z
		db  5Fh	; _
		db    1
		db 0FFh
		db  2Fh	; /
		db    7
		db  60h	; `
		db 0FFh
		db  15h
		db  99h	; Щ
		db  3Ah	; :
		db    4
		db 0FFh
		db  7Eh	; ~
		db  42h	; B
		db  99h	; Щ
		db 0BAh	; ║
		db    1
		db  57h	; W
		db 0BBh	; ╗
		db  53h	; S
		db  76h	; v
		db  3Ah	; :
		db  34h	; 4
		db  78h	; x
		db  7Fh	; 
		db  34h	; 4
		db  99h	; Щ
		db 0FEh	; ■
		db  53h	; S
		db  99h	; Щ
		db 0CBh	; ╦
		db    0
		db    0
		db  4Ah	; J
		db    0
		db    0
		db  6Bh	; k
		db  99h	; Щ
		db  99h	; Щ
		db  89h	; Й
		db    0
		db    0
		db    8
		db    0
		db    0
		db  2Bh	; +
		db    0
		db    0
		db 0AAh	; к
		db    0
		db    0
		db 0CBh	; ╦
		db  99h	; Щ
		db  99h	; Щ
		db  4Ah	; J
		db  99h	; Щ
		db  99h	; Щ
		db    0
		db  0Eh
		db  6Ch	; l
		db    0
		db  12h
		db  14h
		db  28h	; (
		db  0Eh
		db    4
		db  0Ch
		db    2
		db  34h	; 4
		db  0Ch
		db  0Eh
		db  34h	; 4
		db    3
		db  0Bh
		db  61h	; a
		db    0
		db  0Eh
		db  14h
		db  1Ah
		db    8
		db  12h
		db  10h
		db  0Eh
		db    4
		db  0Eh
		db  0Eh
		db  2Ch	; ,
		db 0D4h	; ╘
		db  69h	; i
		db 0E5h	; х
		db  69h	; i
		db  19h
		db  29h	; )
		db  59h	; Y
		db  49h	; I
		db  39h	; 9
		db    7
		db    8
		db    2
		db    1
		db  48h	; H
		db  57h	; W
		db  36h	; 6
		db  15h
		db  24h	; $
		db  35h	; 5
		db  34h	; 4
		db  99h	; Щ
		db  99h	; Щ
		db  99h	; Щ
		db  67h	; g
		db  68h	; h
		db    0
		db 0FFh
		db    5
		db  3Fh	; ?
		db    2
		db  2Fh	; /
		db    5
		db  3Bh	; ;
		db    4
		db  2Dh	; -
		db 0EBh	; ы
		db 0ABh	; л
		db 0ABh	; л
		db    3
		db  2Bh	; +
		db 0ABh	; л
		db 0A9h	; й
		db    3
		db  29h	; )
		db    2
		db    4
		db  29h	; )
		db  48h	; H
		db  89h	; Й
		db  15h
		db  13h
		db    3
		db    5
		db  37h	; 7
		db  56h	; V
		db  67h	; g
		db  25h	; %
		db  34h	; 4
		db  69h	; i
		db  78h	; x
		db  12h
		db  1Ch
		db  2Ch	; ,
		db  16h
		db  17h
		db 0BFh	; ┐
		db  0Ah
		db  0Dh
		db 0ADh	; н
		db 0BEh	; ╛
		db 0EFh	; я
		db  1Bh
		db  9Fh	; Я
		db  1Eh
		db  3Fh	; ?
		db  3Fh	; ?
		db 0BEh	; ╛
		db  3Bh	; ;
		db 0BBh	; ╗
		db  3Eh	; >
		db  5Fh	; _
		db    0
		db  18h
		db    6
		db  44h	; D
		db    0
		db  18h
		db    0
		db    0
		db  68h	; h
		db  3Fh	; ?
		db    0
		db  25h	; %
		db    0
		db  2Dh	; -
		db  16h
		db  0Ch
		db  2Eh	; .
		db  13h
		db    0
		db  1Ch
		db    0
		db  5Fh	; _
		db  40h	; @
		db  88h	; И
		db  3Eh	; >
		db  45h	; E
		db  33h	; 3
		db  7Eh	; ~
		db  61h	; a
		db  44h	; D
		db  1Eh
		db  24h	; $
		db 0FFh
		db  5Fh	; _
		db  10h
		db  44h	; D
		db  1Eh
		db  34h	; 4
		db  22h	; "
		db  3Fh	; ?
		db  56h	; V
		db  44h	; D
		db  1Eh
		db  74h	; t
		db  88h	; И
		db 0DFh	; ▀
		db  80h	; А
		db  99h	; Щ
		db 0BEh	; ╛
		db  35h	; 5
		db  99h	; Щ
		db 0FEh	; ■
		db  61h	; a
		db  99h	; Щ
		db  9Eh	; Ю
		db  72h	; r
		db  99h	; Щ
		db 0DFh	; ▀
		db  10h
		db  99h	; Щ
		db  9Eh	; Ю
		db  23h	; #
		db  99h	; Щ
		db 0BFh	; ┐
		db  56h	; V
		db  99h	; Щ
		db  9Eh	; Ю
		db  87h	; З
		db 0FFh
		db  9Dh	; Э
		db  99h	; Щ
		db  99h	; Щ
		db 0BCh	; ╝
		db  99h	; Щ
		db  99h	; Щ
		db 0FCh	; №
		db  99h	; Щ
		db  99h	; Щ
		db 0DCh	; ▄
		db  99h	; Щ
		db  99h	; Щ
		db  20h
		db  30h	; 0
		db  30h	; 0
		db  20h
		db  44h	; D
		db    0
		db  20h
		db    0
		db  44h	; D
		db  18h
		db  74h	; t
		db  74h	; t
		db  18h
		db 0A4h	; д
		db    0
		db  18h
		db    0
		db 0A4h	; д
		db  18h
		db  40h	; @
		db  50h	; P
		db  8Eh	; О
		db  6Ah	; j
		db  92h	; Т
		db  6Ah	; j
		db    4
		db  48h	; H
		db    8
		db  34h	; 4
		db  45h	; E
		db  35h	; 5
		db  14h
		db  46h	; F
		db  16h
		db  24h	; $
		db  47h	; G
		db  27h	; '
		db    1
		db  23h	; #
		db  56h	; V
		db  78h	; x
		db    9
		db  89h	; Й
		db  39h	; 9
		db  59h	; Y
		db  19h
		db  69h	; i
		db  29h	; )
		db  79h	; y
		db  99h	; Щ
		db  99h	; Щ
		db  18h
		db  3Fh	; ?
		db    2
		db 0FDh	; ¤
		db    4
		db    7
		db    8
		db  15h
		db  16h
		db  19h
		db  24h	; $
		db  26h	; &
		db  2Ah	; *
		db  35h	; 5
		db  37h	; 7
		db  3Bh	; ;
		db  4Ch	; L
		db  5Dh	; ]
		db  6Eh	; n
		db  7Fh	; 
		db  8Ch	; М
		db  8Fh	; П
		db  9Dh	; Э
		db  9Eh	; Ю
		db 0ACh	; м
		db 0AEh	; о
		db 0BDh	; ╜
		db 0BFh	; ┐
		db    1
		db  23h	; #
		db  5Fh	; _
		db  7Eh	; ~
		db  1Eh
		db  3Eh	; >
		db  1Fh
		db  3Fh	; ?
		db  7Eh	; ~
		db  1Eh
		db  5Eh	; ^
		db  9Fh	; Я
		db  67h	; g
		db  3Ch	; <
		db  19h
		db  40h	; @
		db    0
		db    0
		db  67h	; g
		db  19h
		db  3Ch	; <
		db  30h	; 0
		db    0
		db    0
		db  9Fh	; Я
		db  16h
		db 0FFh
		db  1Eh
		db  26h	; &
		db  35h	; 5
		db  3Fh	; ?
		db    2
		db  33h	; 3
		db 0BEh	; ╛
		db    1
		db 0FFh
		db  3Fh	; ?
		db  16h
		db    2
		db  7Eh	; ~
		db  40h	; @
		db  35h	; 5
		db 0AFh	; п
		db    0
		db    0
		db  2Eh	; .
		db    0
		db    0
		db  6Ch	; l
		db    0
		db    0
		db 0ECh	; ь
		db    0
		db    0
		db  20h
		db    0
		db  24h	; $
		db  40h	; @
		db    0
		db  1Ch
		db    0
		db  10h
		db  1Ch
		db  0Ch
		db    6
		db  1Ch
		db 0FDh	; ¤
		db  6Ah	; j
		db    2
		db  6Bh	; k
		db  56h	; V
		db  14h
		db  16h
		db  45h	; E
		db  23h	; #
		db  26h	; &
		db  35h	; 5
		db    2
		db    3
		db    1
		db    4
		db    0
		db 0FFh
		db  0Bh
		db  3Fh	; ?
		db  2Fh	; /
		db    3
		db  2Bh	; +
		db    1
		db    3
		db    4
		db    5
		db  12h
		db  14h
		db  15h
		db  24h	; $
		db  25h	; %
		db  34h	; 4
		db  35h	; 5
		db  67h	; g
		db  69h	; i
		db  78h	; x
		db  89h	; Й
		db  3Fh	; ?
		db  9Fh	; Я
		db  1Eh
		db  5Eh	; ^
		db 0DEh	; ▐
		db  5Fh	; _
		db  1Eh
		db    0
		db    0
		db  70h	; p
		db  0Ch
		db  2Fh	; /
		db    6
		db    0
		db  20h
		db    8
		db  9Fh	; Я
		db    1
		db 0FFh
		db  1Eh
		db    1
		db  23h	; #
		db  3Fh	; ?
		db  23h	; #
		db  67h	; g
		db 0BEh	; ╛
		db  89h	; Й
		db 0FFh
		db  3Fh	; ?
		db  67h	; g
		db 0CCh	; ╠
		db 0BEh	; ╛
		db 0C8h	; ╚
		db  99h	; Щ
		db  7Fh	; 
		db  7Ch	; |
		db 0CBh	; ╦
		db 0FEh	; ■
		db 0CCh	; ╠
		db  9Bh	; Ы
		db 0BEh	; ╛
		db 0CCh	; ╠
		db  8Ah	; К
		db  3Eh	; >
		db  6Ch	; l
		db 0CAh	; ╩
		db  9Fh	; Я
		db    8
		db  4Ah	; J
		db  1Eh
		db    2
		db  6Ah	; j
		db 0DEh	; ▐
		db  19h
		db  5Bh	; [
		db  5Eh	; ^
		db  13h
		db  7Bh	; {
		db  87h	; З
		db    0
		db    0
		db    6
		db    0
		db    0
		db    5
		db    0
		db    0
		db  84h	; Д
		db    0
		db    0
		db  12h
		db    0
		db  28h	; (
		db  1Eh
		db    0
		db  18h
		db  1Eh
		db    0
		db  28h	; (
		db  12h
		db    7
		db  28h	; (
		db  12h
		db    7
		db  0Dh
		db  0Bh
		db    3
		db  1Dh
		db  0Bh
		db    4
		db  18h
		db  8Bh	; Л
		db  6Bh	; k
		db  92h	; Т
		db  6Bh	; k
		db    1
		db    4
		db  15h
		db    2
		db  13h
		db  67h	; g
		db  26h	; &
		db  37h	; 7
		db  89h	; Й
		db  48h	; H
		db  59h	; Y
		db  7Ch	; |
		db  6Ch	; l
		db  9Ch	; Ь
		db  8Ch	; М
		db 0BCh	; ╝
		db  7Bh	; {
		db  9Bh	; Ы
		db 0ACh	; м
		db  8Ah	; К
		db  6Ah	; j
		db  0Ah
		db  1Bh
		db  45h	; E
		db  23h	; #
		db    0
		db 0FFh
		db  17h
		db  3Fh	; ?
		db    3
		db  27h	; '
		db    2
		db 0A3h	; г
		db 0E5h	; х
		db    1
		db  0Ah
		db  0Ch
		db  1Bh
		db  1Dh
		db  24h	; $
		db  2Bh	; +
		db  2Dh	; -
		db  35h	; 5
		db  3Ah	; :
		db  3Ch	; <
		db  46h	; F
		db  49h	; I
		db  57h	; W
		db  58h	; X
		db  67h	; g
		db  6Dh	; m
		db  7Ch	; |
		db  89h	; Й
		db  8Ah	; К
		db  9Bh	; Ы
		db 0ABh	; л
		db 0CDh	; ═
		db    3
		db  12h
		db 0EFh	; я
		db  0Fh
		db  1Eh
		db    1
		db  1Fh
		db  5Eh	; ^
		db  1Fh
		db  5Eh	; ^
		db  9Eh	; Ю
		db 0DEh	; ▐
		db  1Fh
		db  5Eh	; ^
		db  9Eh	; Ю
		db 0DEh	; ▐
		db  1Fh
		db  5Eh	; ^
		db  3Fh	; ?
		db    0
		db  27h	; '
		db  0Ah
		db  45h	; E
		db  32h	; 2
		db  0Dh
		db  1Eh
		db  34h	; 4
		db    0
		db    0
		db  1Ch
		db    0
		db    0
		db    0
		db 0A0h	; а
		db 0FFh
		db  13h
		db  22h	; "
		db 0BEh	; ╛
		db  12h
		db  44h	; D
		db  7Fh	; 
		db  32h	; 2
		db    0
		db  3Eh	; >
		db  24h	; $
		db 0FFh
		db  3Fh	; ?
		db  65h	; e
		db    0
		db  9Fh	; Я
		db  16h
		db 0FFh
		db 0DFh	; ▀
		db  16h
		db  33h	; 3
		db  9Eh	; Ю
		db  14h
		db  55h	; U
		db  7Fh	; 
		db  63h	; c
		db    0
		db  3Eh	; >
		db  45h	; E
		db    0
		db    9
		db  18h
		db  20h
		db    9
		db    6
		db  1Ah
		db    9
		db    0
		db    8
		db    9
		db    0
		db  28h	; (
		db    9
		db  26h	; &
		db  0Ch
		db    9
		db  0Ah
		db  0Fh
		db  1Fh
		db  9Eh	; Ю
		db  3Fh	; ?
		db  7Fh	; 
		db  3Eh	; >
		db  1Fh
		db  5Eh	; ^
		db  24h	; $
		db    0
		db    0
		db  24h	; $
		db    0
		db  68h	; h
		db  2Eh	; .
		db  2Ah	; *
		db  0Eh
		db  14h
		db    5
		db    7


; === vShipNew1 === vShipNew1:	db 1Fh,	1, 0FFh
vShipNew1:	db 1Fh,	1, 0FFh
		db 1Fh,	2, 77h
		db 9Eh,	13h, 77h
		db 5Eh,	2, 0FFh
		db 0DEh, 13h, 88h
		db 3Fh,	24h, 77h
		db 0BEh, 35h, 77h
		db 7Eh,	24h, 88h
		db 0FEh, 35h, 88h
		db 3Fh,	46h, 77h
		db 0BEh, 56h, 77h
		db 7Eh,	46h, 88h
		db 0FEh, 56h, 88h
		db 3Fh,	66h, 77h
		db 0BEh, 66h, 77h
		db 7Eh,	66h, 88h
		db 0FEh, 66h, 88h
		db 3Fh,	66h, 77h
		db 7Eh,	66h, 88h
		db 0BDh, 66h, 66h
		db 3Ch,	66h, 66h
		db 0BDh, 66h, 66h
		db 3Ch,	66h, 66h
		db 0FCh, 66h, 66h
		db 7Ch,	66h, 66h
		db 5Dh,	28h, 38h
		db 5Dh,	28h, 38h
		db 0DDh, 28h, 38h
		db 7Dh,	28h, 38h
		db 0, 0, 50h
		db 14h,	7, 14h
		db 28h,	0Ch, 14h
		db 3Ch,	0Fh, 28h
		db 1Eh,	0Ah, 23h
		db 0, 8, 20h
		db 1Eh,	0, 23h
		db 32h,	8, 25h
		db 0, 8, 14h
		db 5, 0Ah, 0
		db 5, 9, 0Ah
		db 0, 0Bh, 0Fh


; === nShipNew1 === nShipNew1:	db  1Fh
nShipNew1:	db  1Fh
		db  9Eh	; Ю
		db  1Fh
		db  9Eh	; Ю
		db  1Fh
		db  9Eh	; Ю
		db  3Fh	; ?
		db  1Fh
		db  5Eh	; ^
		db  18h
		db    0
		db  12h
		db  18h
		db    0
		db  0Ch
		db  0Ah
		db    0
		db  0Ah
		db    0
		db    0
		db  28h	; (
		db    0
		db  0Ah
		db    1


; === eShipNew1 === eShipNew1:	dw unk_6D3A
eShipNew1:	dw unk_6D3A
		dw unk_6D44
		db    7
		db  17h
		db    8
		db  18h
		db  27h	; '
		db  37h	; 7
		db  28h	; (
		db  38h	; 8
		db  47h	; G
		db  57h	; W
		db  48h	; H
		db  58h	; X
		db  46h	; F
		db  67h	; g
		db  56h	; V
		db  67h	; g
		db  68h	; h
		db  67h	; g
		db  67h	; g
		db  68h	; h
		db  68h	; h
		db  68h	; h
		db  88h	; И
		db  88h	; И
		db  88h	; И
		db  66h	; f
		db 0FFh
unk_6D3A:	db  11h			; DATA XREF: RAM:eShipNew1o
		db  3Fh	; ?
		db    4
		db 0BFh	; ┐
		db 0FFh
		db 0EDh	; э
		db    6
		db 0EEh	; ю
		db    2
		db 0EDh	; э
unk_6D44:	db    1			; DATA XREF: RAM:6D1Do
		db    2
		db    3
		db    4
		db  15h
		db  26h	; &
		db  37h	; 7
		db  48h	; H
		db  59h	; Y
		db  6Ah	; j
		db  7Bh	; {
		db  8Ch	; М
		db  9Bh	; Ы
		db  9Dh	; Э
		db 0ACh	; м
		db 0AEh	; о
		db 0BFh	; ┐
		db  1Dh
		db  1Eh
		db  0Ch
		db  2Fh	; /
		db    2
		db 0ABh	; л
		db  9Ah	; Ъ
		db 0BCh	; ╝
		db  35h	; 5
		db  37h	; 7
		db  46h	; F
		db  48h	; H
		db  68h	; h
		db  57h	; W


; === vShipNew2 === vShipNew2:	db 1Fh,	1, 9Ah
vShipNew2:	db 1Fh,	1, 9Ah
		db 9Eh,	2, 9Ah
		db 5Eh,	1, 9Ah
		db 0DEh, 2, 0FFh
		db 1Fh,	13h, 9Ah
		db 9Eh,	24h, 9Ah
		db 5Eh,	13h, 9Ah
		db 0DEh, 24h, 9Ah
		db 1Fh,	35h, 9Ah
		db 9Eh,	46h, 9Ah
		db 5Eh,	35h, 9Ah
		db 0DEh, 46h, 9Ah
		db 3Fh,	37h, 9Ah
		db 0BEh, 68h, 9Ah
		db 7Eh,	37h, 9Ah
		db 0FEh, 68h, 9Ah
		db 3Fh,	77h, 9Ah
		db 0BEh, 88h, 9Ah
		db 3Fh,	9Ah, 0ABh
		db 0BEh, 9Ah, 0ABh
		db 3Fh,	9Ah, 0ABh
		db 7Eh,	9Ah, 0ABh
		db 1Fh,	0, 0FFh
		db 1Dh,	99h, 99h
		db 13h,	0Ah, 2Dh
		db 15h,	8, 1Eh
		db 19h,	5, 5
		db 23h,	2, 14h
		db 2Dh,	0, 23h
		db 19h,	0, 1Eh
		db 0, 6, 1Ch
		db 0, 0, 2Dh
		db 0, 8, 1Eh


; === nShipNew2 === nShipNew2:	db  1Fh
nShipNew2:	db  1Fh
		db  1Fh
		db  9Eh	; Ю
		db  1Eh
		db  9Eh	; Ю
		db  1Eh
		db  9Eh	; Ю
		db  1Eh
		db  9Eh	; Ю
		db  3Fh	; ?
		db  7Eh	; ~
		db  3Fh	; ?
		db    0
		db    0
		db  5Ah	; Z
		db  2Eh	; .
		db    0
		db  12h
		db    0
		db  50h	; P
		db    5
		db    0
		db    0
		db  3Ch	; <


; === eShipNew2 === eShipNew2:	dw unk_6E00
eShipNew2:	dw unk_6E00
		dw unk_6E08
		db    9
		db    2
		db  0Ah
		db    1
		db  19h
		db  1Ah
		db  29h	; )
		db  2Ah	; *
		db  39h	; 9
		db  49h	; I
		db  3Ah	; :
		db  4Ah	; J
		db  59h	; Y
		db  69h	; i
		db  5Ah	; Z
		db  6Ah	; j
		db  79h	; y
		db  7Ah	; z
		db  89h	; Й
		db  8Ah	; К
		db  9Ah	; Ъ
		db  9Ah	; Ъ
		db  9Bh	; Ы
		db  9Bh	; Ы
		db 0ABh	; л
		db 0ABh	; л
		db    0
		db    0
		db  99h	; Щ
		db  99h	; Щ
unk_6E00:	db  10h			; DATA XREF: RAM:eShipNew2o
		db  3Fh	; ?
		db    4
		db 0BFh	; ┐
		db    6
		db 0FFh
		db    4
		db 0BDh	; ╜
unk_6E08:	db    1			; DATA XREF: RAM:6DE0o
		db  13h
		db  32h	; 2
		db  20h
		db    4
		db  26h	; &
		db  15h
		db  37h	; 7
		db  48h	; H
		db  59h	; Y
		db  6Ah	; j
		db  7Bh	; {
		db  8Ch	; М
		db  9Dh	; Э
		db 0AEh	; о
		db 0BFh	; ┐
		db  0Ch
		db  0Eh
		db  1Dh
		db  1Fh
		db    2
		db  13h
		db  24h	; $
		db  34h	; 4
		db  25h	; %
		db  35h	; 5
		db  61h	; a
		db  60h	; `
		db  71h	; q
		db  70h	; p


; === vShipNew3 === vShipNew3:	db 1Fh,	1, 0FFh
vShipNew3:	db 1Fh,	1, 0FFh
		db 3Fh,	3, 45h
		db 7Eh,	12h, 45h
		db 3Fh,	1, 0FFh
		db 0BEh, 23h, 55h
		db 1Fh,	1, 11h
		db 9Eh,	23h, 33h
		db 9, 0, 33h
		db 7, 0, 33h
		db 89h,	33h, 33h
		db 8, 0, 0
		db 29h,	44h, 44h
		db 68h,	44h, 44h
		db 0E8h, 55h, 55h
		db 0A8h, 55h, 55h
		db 29h,	44h, 44h
		db 0A8h, 55h, 55h
		db 0, 0, 60h
		db 0, 12h, 30h
		db 5Ah,	0, 3
		db 5Ah,	0, 57h
		db 0, 5, 35h
		db 0, 7, 26h
		db 12h,	7, 13h
		db 12h,	0Bh, 27h
		db 24h,	0, 1Eh


; === nShipNew3 === nShipNew3:	db  1Fh
nShipNew3:	db  1Fh
		db  5Eh	; ^
		db 0DEh	; ▐
		db  9Eh	; Ю
		db  3Fh	; ?
		db 0BEh	; ╛
		db    3
		db  18h
		db    3
		db  26h	; &
		db    0
		db  4Dh	; M


; === eX4 === eX4:		db 98h
eX4:		db 98h
		db  6Eh	; n
		db 0A7h	; з
		db  6Eh	; n
		db    3
		db  12h
		db    1
		db  23h	; #
		db  35h	; 5
		db  25h	; %
		db  14h
		db    4
		db    1
		db  23h	; #
		db  45h	; E
		db    0
		db    0
		db  33h	; 3
		db  33h	; 3
		db  44h	; D
		db  44h	; D
		db  44h	; D
		db  55h	; U
		db 0FFh
		db    8
		db  3Fh	; ?
		db    2
		db  3Eh	; >
		db  28h	; (
		db  29h	; )
		db  26h	; &
		db  29h	; )
		db  26h	; &
		db    2
		db  28h	; (
		db    2
		db  27h	; '
		db    2
		db 0A8h	; и
		db    1
		db    2
		db    3
		db    4
		db  14h
		db  24h	; $
		db  23h	; #
		db  13h
		db  35h	; 5
		db  46h	; F
		db  12h
		db  7Ah	; z
		db  8Ah	; К
		db  79h	; y
		db  89h	; Й
		db 0BFh	; ┐
		db 0CFh	; ╧
		db 0BCh	; ╝
		db 0EDh	; э
		db  0Dh
		db  0Eh

; =============== S U B	R O U T	I N E =======================================




; === TempAdr === TempAdr:	dw 6838h		; DATA XREF: sub_AA31r
TempAdr:	dw 6838h		; DATA XREF: sub_AA31r
					; sub_AE0E:loc_AE16w ...
					; адрес	текущей	вершины
		db    0
		db    0
		db    0
		db    0
		db    0

; =============== S U B	R O U T	I N E =======================================

; Attributes: thunk



; === WhatOnScreen === WhatOnScreen:	db 0Bh			; DATA XREF: GoDocking+39w
WhatOnScreen:	db 0Bh			; DATA XREF: GoDocking+39w
					; ControlKeyIfMapOnr ...


; === HighDeltaSpeed === HighDeltaSpeed:	db 0Ch			; DATA XREF: CalcAngSpeed4Prn+32w
HighDeltaSpeed:	db 0Ch			; DATA XREF: CalcAngSpeed4Prn+32w
					; RAM:751Aw ...
		db 0A0h	; а


; === ShieldFront === ShieldFront:	db 0A0h			; DATA XREF: UpdateAllIndicators+7o
ShieldFront:	db 0A0h			; DATA XREF: UpdateAllIndicators+7o
					; sub_9946+1Bw	...
		db 0A0h	; а


; === ShieldRear === ShieldRear:	db 0A0h			; DATA XREF: sub_9946+1Ew RAM:99B0r ...
ShieldRear:	db 0A0h			; DATA XREF: sub_9946+1Ew RAM:99B0r ...
		db 0


; === Fuel === Fuel:		db 0			; DATA XREF: SetPilotFuel+13w
Fuel:		db 0			; DATA XREF: SetPilotFuel+13w
		db 0


; === Temperature === Temperature:	db 0			; DATA XREF: sub_8B3F:loc_8B47w
Temperature:	db 0			; DATA XREF: sub_8B3F:loc_8B47w
					; sub_9946+Dw ...
		db 0


; === ShieldUnit1 === ShieldUnit1:	db 0A0h	; а		; DATA XREF: sub_9946+24o sub_9A8Do
ShieldUnit1:	db 0A0h	; а		; DATA XREF: sub_9946+24o sub_9A8Do
		db 0A0h	; а
		db 0A0h	; а
		db 0A0h	; а
byte_70FA:	db 0A0h			; DATA XREF: Use_Y_Cloacking+1Fr
		db 0A0h	; а


; === ShieldUnit4 === ShieldUnit4:	db 0A0h			; DATA XREF: Damage2Pilot+6r sub_9AE2o ...
ShieldUnit4:	db 0A0h			; DATA XREF: Damage2Pilot+6r sub_9AE2o ...
		db  15h
byte_70FE:	db 15h			; DATA XREF: CalcAngSpeed4Prn:loc_7151w
		db  15h
byte_7100:	db 15h			; DATA XREF: CalcAngSpeed4Prn:loc_7168w
		db    0
		db    0


; === InFreeSpace === InFreeSpace:	db 0			; DATA XREF: CalcCrdCoriolis2r
InFreeSpace:	db 0			; DATA XREF: CalcCrdCoriolis2r
					; sub_5FD5+42w	...


; === IndDanger === IndDanger:	db 0			; DATA XREF: GoDocking+23r
IndDanger:	db 0			; DATA XREF: GoDocking+23r
					; sub_728D+17Fw ...


; === RndWord === RndWord:	db 0			; DATA XREF: RAM:8C43r	sub_ABAFo ...
RndWord:	db 0			; DATA XREF: RAM:8C43r	sub_ABAFo ...
byte_7106:	db 4Ah			; DATA XREF: sub_DCE9+3Bo


; === CoriolisNear === CoriolisNear:	db 1			; DATA XREF: GoDocking+19r
CoriolisNear:	db 1			; DATA XREF: GoDocking+19r
					; sub_7FF0+21r	...


; === J_Drive_Coriolis === J_Drive_Coriolis:db 0			; DATA XREF: CalcCrdCoriolis2:CalcCrdCoriolisr
J_Drive_Coriolis:db 0			; DATA XREF: CalcCrdCoriolis2:CalcCrdCoriolisr
					; CalcCrdCoriolis2+1AA9w ...


; === FuelScoopsOn === FuelScoopsOn:	db 0			; DATA XREF: sub_728D+185w
FuelScoopsOn:	db 0			; DATA XREF: sub_728D+185w
					; PrintCompass:loc_81ECw ...


; === sub_74C6 === sub_74C6:
sub_74C6:

; FUNCTION CHUNK AT 750F SIZE 00000003 BYTES

		ld	a, (IndDanger)
		and	a
		jr	z, loc_74D2
		call	SetIndStation
		call	Damage2Pilot

loc_74D2:				; CODE XREF: sub_74C6+4j
		call	CorrData4Nova
		call	j_PrnTxtOnViewScr
		ld	b, 8

loc_74DA:				; CODE XREF: sub_74C6+1Aj
		push	bc
		call	ControlKeyIfMapOn
		halt
		pop	bc
		djnz	loc_74DA
		jr	loc_750F
; End of function sub_74C6

; ---------------------------------------------------------------------------
; START	OF FUNCTION CHUNK FOR sub_728D

loc_74E4:				; CODE XREF: sub_728D+234j
		call	PrnCocpit_Ifn
		call	ClearScrBuffer
		ld	a, (InFreeSpace)
		and	a
		call	z, PrnSunPlanet
		call	PrintStars

loc_74F4:				; CODE XREF: sub_7A9E+24j
		call	j_PrnTxtOnViewScr
		call	j_j_MoveAllObjects
; END OF FUNCTION CHUNK	FOR sub_728D
; ---------------------------------------------------------------------------
		call	sub_9296
; ---------------------------------------------------------------------------
		jr	loc_7502
; ---------------------------------------------------------------------------
		db    0
		db  18h
		db 0DDh	; ▌
; ---------------------------------------------------------------------------

loc_7502:				; CODE XREF: RAM:74FDj
		call	sub_ED60
		call	UpdateAllIndicators
		ei
		call	Use_Y_Cloacking
		call	Process_MissBombEPod ; Печать индикаторов, запуск ракеты, бомбы
; START	OF FUNCTION CHUNK FOR sub_74C6

loc_750F:				; CODE XREF: sub_74C6+1Cj
		call	PrintCompass
; END OF FUNCTION CHUNK	FOR sub_74C6
; ---------------------------------------------------------------------------
		ld	a, (PlayerGameOver)
		and	a
		jp	z, GameMain
		xor	a
		ld	(HighDeltaSpeed), a
		ld	(TypeOfPrint), a
		inc	a
		ld	(BlockControl),	a
		ld	l, 7
		call	j_SetAtrViewScr
		call	CopyScaner2Scr
		call	SetShipwreck
; ---------------------------------------------------------------------------
		call	sub_9548
		call	FX_Launch
		call	PrnCocpit_Ifn
		ld	b, 1Eh

loc_753A:				; CODE XREF: RAM:755Ej
		push	bc
		call	ClearScrBuffer
		call	PrintStars
		call	j_j_MoveAllObjects
; ---------------------------------------------------------------------------
		ld	hl, (word_D0D5)
		inc	hl
		inc	hl
		push	hl
		set	7, (hl)
		ld	hl, 70Ah
		ld	(CrdXPrnSymbol), hl
		ld	a, 9
		call	sub_9381
		pop	hl
		res	7, (hl)
		call	CopyBuf2Screen
		pop	bc
		djnz	loc_753A
		call	ClearScrBuffer
		call	CopyBuf2Screen
		call	ClearAroundObj
		ld	b, 32h ; '2'

loc_756B:				; CODE XREF: RAM:756Cj
		halt
		djnz	loc_756B
		xor	a
		ld	(BlockControl),	a
		ld	(PlayerGameOver), a

loc_7575:				; DATA XREF: CalcPrnCrdSun+17o
		jp	loc_739E
; ---------------------------------------------------------------------------
word_7578:	dw 0			; DATA XREF: RAM:7366w


; === sub_74C6 === sub_74C6:
sub_74C6:

; FUNCTION CHUNK AT 750F SIZE 00000003 BYTES

		ld	a, (IndDanger)
		and	a
		jr	z, loc_74D2
		call	SetIndStation
		call	Damage2Pilot

loc_74D2:				; CODE XREF: sub_74C6+4j
		call	CorrData4Nova
		call	j_PrnTxtOnViewScr
		ld	b, 8

loc_74DA:				; CODE XREF: sub_74C6+1Aj
		push	bc
		call	ControlKeyIfMapOn
		halt
		pop	bc
		djnz	loc_74DA
		jr	loc_750F
; End of function sub_74C6

; ---------------------------------------------------------------------------
; START	OF FUNCTION CHUNK FOR sub_728D

loc_74E4:				; CODE XREF: sub_728D+234j
		call	PrnCocpit_Ifn
		call	ClearScrBuffer
		ld	a, (InFreeSpace)
		and	a
		call	z, PrnSunPlanet
		call	PrintStars

loc_74F4:				; CODE XREF: sub_7A9E+24j
		call	j_PrnTxtOnViewScr
		call	j_j_MoveAllObjects
; END OF FUNCTION CHUNK	FOR sub_728D
; ---------------------------------------------------------------------------
		call	sub_9296
; ---------------------------------------------------------------------------
		jr	loc_7502
; ---------------------------------------------------------------------------
		db    0
		db  18h
		db 0DDh	; ▌
; ---------------------------------------------------------------------------

loc_7502:				; CODE XREF: RAM:74FDj
		call	sub_ED60
		call	UpdateAllIndicators
		ei
		call	Use_Y_Cloacking
		call	Process_MissBombEPod ; Печать индикаторов, запуск ракеты, бомбы
; START	OF FUNCTION CHUNK FOR sub_74C6

loc_750F:				; CODE XREF: sub_74C6+1Cj
		call	PrintCompass
; END OF FUNCTION CHUNK	FOR sub_74C6
; ---------------------------------------------------------------------------
		ld	a, (PlayerGameOver)
		and	a
		jp	z, GameMain
		xor	a
		ld	(HighDeltaSpeed), a
		ld	(TypeOfPrint), a
		inc	a
		ld	(BlockControl),	a
		ld	l, 7
		call	j_SetAtrViewScr
		call	CopyScaner2Scr
		call	SetShipwreck
; ---------------------------------------------------------------------------
		call	sub_9548
		call	FX_Launch
		call	PrnCocpit_Ifn
		ld	b, 1Eh

loc_753A:				; CODE XREF: RAM:755Ej
		push	bc
		call	ClearScrBuffer
		call	PrintStars
		call	j_j_MoveAllObjects
; ---------------------------------------------------------------------------
		ld	hl, (word_D0D5)
		inc	hl
		inc	hl
		push	hl
		set	7, (hl)
		ld	hl, 70Ah
		ld	(CrdXPrnSymbol), hl
		ld	a, 9
		call	sub_9381
		pop	hl
		res	7, (hl)
		call	CopyBuf2Screen
		pop	bc
		djnz	loc_753A
		call	ClearScrBuffer
		call	CopyBuf2Screen
		call	ClearAroundObj
		ld	b, 32h ; '2'

loc_756B:				; CODE XREF: RAM:756Cj
		halt
		djnz	loc_756B
		xor	a
		ld	(BlockControl),	a
		ld	(PlayerGameOver), a

loc_7575:				; DATA XREF: CalcPrnCrdSun+17o
		jp	loc_739E
; ---------------------------------------------------------------------------
word_7578:	dw 0			; DATA XREF: RAM:7366w


; === sub_7DA0 === sub_7DA0:
sub_7DA0:
		dec	hl
		set	2, (hl)
		ret
; End of function sub_7DA0


; =============== S U B	R O U T	I N E =======================================




; === sub_7DA0 === sub_7DA0:
sub_7DA0:
		dec	hl
		set	2, (hl)
		ret
; End of function sub_7DA0


; =============== S U B	R O U T	I N E =======================================




; === StarsFastScroll === StarsFastScroll:db 0			; DATA XREF: PrintStarsr
StarsFastScroll:db 0			; DATA XREF: PrintStarsr
					; Use_J_Drive:loc_7F18w ...

; =============== S U B	R O U T	I N E =======================================




; === J_Drive_Ready2 === J_Drive_Ready2:	db 0			; DATA XREF: Use_J_Drive+13r
J_Drive_Ready2:	db 0			; DATA XREF: Use_J_Drive+13r
					; PrintCompass+1w ...


; === J_Drive_Ready3 === J_Drive_Ready3:	db 1			; DATA XREF: PrintCompass+4Fw
J_Drive_Ready3:	db 1			; DATA XREF: PrintCompass+4Fw
					; PrintCompass+77w


; === sub_96DC === sub_96DC:
sub_96DC:

; FUNCTION CHUNK AT 96D6 SIZE 00000003 BYTES
; FUNCTION CHUNK AT 982E SIZE 00000024 BYTES
; FUNCTION CHUNK AT B2EF SIZE 00000005 BYTES

		ld	a, (FlgObjNear)
		and	a
		ret	z
		call	SetIndStation
		ld	a, 8
		ld	(ScanerNumObj),	a
		ld	ix, objCoriolis

loc_96ED:				; CODE XREF: sub_96DC+160j
		ld	a, (ix+22h)
		and	a
		jr	z, loc_96D6
		ld	a, (ix+1)
		cp	40h ; '@'
		jr	nc, loc_96D6
		ld	a, (ix+4)
		cp	40h ; '@'
		jr	nc, loc_96D6
		ld	a, (ix+7)
		cp	40h ; '@'
		jr	nc, loc_96D6
		ld	a, (ix+1)
		call	Sqr_DE_A
		ex	de, hl
		ld	a, (ix+7)
		call	Sqr_DE_A
		add	hl, de
		jr	c, loc_96D6
		ld	de, 1600h
		and	a
		sbc	hl, de
		jr	nc, loc_96D6
		ld	a, 1
		ld	(FlgSaveScaner), a
		ld	de, 4000h
		xor	a
		ld	l, (ix+0)
		ld	h, (ix+1)

loc_972F:				; DATA XREF: sub_7A84o
		and	a
		jp	c, loc_B2EF
		call	Div_BC_HLAdivDE
; End of function sub_96DC


; =============== S U B	R O U T	I N E =======================================

; Attributes: noreturn



; === sub_96DC === sub_96DC:
sub_96DC:

; FUNCTION CHUNK AT 96D6 SIZE 00000003 BYTES
; FUNCTION CHUNK AT 982E SIZE 00000024 BYTES
; FUNCTION CHUNK AT B2EF SIZE 00000005 BYTES

		ld	a, (FlgObjNear)
		and	a
		ret	z
		call	SetIndStation
		ld	a, 8
		ld	(ScanerNumObj),	a
		ld	ix, objCoriolis

loc_96ED:				; CODE XREF: sub_96DC+160j
		ld	a, (ix+22h)
		and	a
		jr	z, loc_96D6
		ld	a, (ix+1)
		cp	40h ; '@'
		jr	nc, loc_96D6
		ld	a, (ix+4)
		cp	40h ; '@'
		jr	nc, loc_96D6
		ld	a, (ix+7)
		cp	40h ; '@'
		jr	nc, loc_96D6
		ld	a, (ix+1)
		call	Sqr_DE_A
		ex	de, hl
		ld	a, (ix+7)
		call	Sqr_DE_A
		add	hl, de
		jr	c, loc_96D6
		ld	de, 1600h
		and	a
		sbc	hl, de
		jr	nc, loc_96D6
		ld	a, 1
		ld	(FlgSaveScaner), a
		ld	de, 4000h
		xor	a
		ld	l, (ix+0)
		ld	h, (ix+1)

loc_972F:				; DATA XREF: sub_7A84o
		and	a
		jp	c, loc_B2EF
		call	Div_BC_HLAdivDE
; End of function sub_96DC


; =============== S U B	R O U T	I N E =======================================

; Attributes: noreturn



; === sub_9736 === sub_9736:
sub_9736:

; FUNCTION CHUNK AT 985E SIZE 0000000C BYTES

		call	sub_96C9
		ld	h, b
		ld	l, c
		call	sub_96D1
		add	hl, bc
		ld	a, (ix+2)
		and	80h ; 'А'
		or	h
		ld	(ScanerCrdXobj), a
		xor	a
		ld	l, (ix+3)
		ld	h, (ix+4)
		call	Div_BC_HLAdivDE
; ---------------------------------------------------------------------------
		call	sub_96C5
		ld	h, b
		ld	l, c
		call	sub_96CD
		add	hl, bc
		ld	a, (ix+5)
		and	80h ; 'А'
		ld	a, h
		jr	z, loc_9765
		or	80h ; 'А'

loc_9765:				; CODE XREF: sub_9736+2Bj
		ld	(ScanerCrdYobj), a
		xor	a
		ld	l, (ix+6)
		ld	h, (ix+7)
		call	Div_BC_HLAdivDE
; ---------------------------------------------------------------------------
		call	sub_96C5
		ld	h, b
		ld	l, c
		call	sub_96CD
		add	hl, bc
		ld	a, (ix+8)
		and	80h ; 'А'
		ld	a, h
		jr	z, loc_9785
		or	80h ; 'А'

loc_9785:				; CODE XREF: sub_9736+4Bj
		ld	(ScanerCrdZobj), a
		ld	hl, 558Fh
		ld	a, (ScanerCrdXobj)
		and	7Fh ; ''
		sub	4
		ld	c, a
		jp	c, loc_97BB
		rra
		rra
		rra
		and	1Fh
		ld	b, a
		ld	a, c
		and	7
		ld	c, a
		jr	z, loc_97A3
		inc	b

loc_97A3:				; CODE XREF: sub_9736+6Aj
		ld	a, (ScanerCrdXobj)
		and	80h ; 'А'
		jr	nz, loc_97C7

loc_97AA:				; CODE XREF: sub_9736+75j
		inc	hl
		djnz	loc_97AA

loc_97AD:				; CODE XREF: sub_9736+8Fj
		ld	a, c
		and	a
		ld	b, a
		ld	a, 1
		jr	z, loc_97D6
		xor	a
		scf

loc_97B6:				; CODE XREF: sub_9736+81j
		rra
		djnz	loc_97B6
		jr	loc_97D6
; ---------------------------------------------------------------------------

loc_97BB:				; CODE XREF: sub_9736+5Dj
		add	a, 8
		ld	c, a
		ld	a, (ScanerCrdXobj)
		and	80h ; 'А'
		jr	nz, loc_97CA
		jr	loc_97AD
; ---------------------------------------------------------------------------

loc_97C7:				; CODE XREF: sub_9736+72j sub_9736+92j
		dec	hl
		djnz	loc_97C7

loc_97CA:				; CODE XREF: sub_9736+8Dj
		ld	a, c
		and	a
		ld	b, a
		ld	a, 80h ; 'А'
		jr	z, loc_97D6
		xor	a
		scf

loc_97D3:				; CODE XREF: sub_9736+9Ej
		rla
		djnz	loc_97D3

loc_97D6:				; CODE XREF: sub_9736+7Cj sub_9736+83j ...
		ld	c, a
		ld	a, (ScanerCrdZobj)
		ld	d, a
		and	7Fh ; ''
		ld	b, a
		jr	z, loc_97F9
		ld	a, d
		and	80h ; 'А'
		jr	nz, loc_97F0

loc_97E5:				; CODE XREF: sub_9736+B6j
		ld	a, h
		dec	h
		and	7
		call	z, compUpScrAdr_HL
		djnz	loc_97E5
		jr	loc_97F9
; ---------------------------------------------------------------------------

loc_97F0:				; CODE XREF: sub_9736+ADj sub_9736+C1j
		inc	h
		ld	a, h
		and	7
		call	z, compDownScrAdr_HL
		djnz	loc_97F0

loc_97F9:				; CODE XREF: sub_9736+A8j sub_9736+B8j
		ld	a, (ScanerCrdYobj)
		ld	d, a
		and	7Fh ; ''
		ld	b, a
		jr	z, loc_9821
		ld	a, d
		and	80h ; 'А'
		jr	nz, loc_9815

loc_9807:				; CODE XREF: sub_9736+DBj
		ld	a, (hl)
		or	c
		ld	(hl), a
		ld	a, h
		dec	h
		and	7
		call	z, sub_987C
		djnz	loc_9807
		jr	loc_9821
; ---------------------------------------------------------------------------

loc_9815:				; CODE XREF: sub_9736+CFj sub_9736+E9j
		ld	a, (hl)
		or	c
		ld	(hl), a
		inc	h
		ld	a, h
		and	7
		call	z, sub_986A
		djnz	loc_9815

loc_9821:				; CODE XREF: sub_9736+CAj sub_9736+DDj
		ld	a, c
		add	a, a
		jr	c, sub_9852
		or	c
		add	a, a
		jr	c, loc_985E
		or	c
		ld	e, a
		ld	a, (hl)
		or	e
		ld	(hl), a
; End of function sub_9736

; START	OF FUNCTION CHUNK FOR sub_96DC

loc_982E:				; CODE XREF: sub_96DC:loc_96D6j
					; sub_9852+Aj ...
		ld	a, (ScanerNumObj)
		dec	a
		jr	z, loc_983F
		ld	(ScanerNumObj),	a
		ld	de, 27h	; '''
		add	ix, de
		jp	loc_96ED
; ---------------------------------------------------------------------------

loc_983F:				; CODE XREF: sub_96DC+156j
		xor	a
		ld	c, 7
		ld	hl, 5009h

loc_9845:				; CODE XREF: sub_96DC+173j
		ld	b, 0Dh
		push	hl

loc_9848:				; CODE XREF: sub_96DC+16Ej
		ld	(hl), a
		inc	l
		djnz	loc_9848
		pop	hl
		inc	h
		dec	c
		jr	nz, loc_9845
		ret
; END OF FUNCTION CHUNK	FOR sub_96DC

; =============== S U B	R O U T	I N E =======================================




; === sub_9736 === sub_9736:
sub_9736:

; FUNCTION CHUNK AT 985E SIZE 0000000C BYTES

		call	sub_96C9
		ld	h, b
		ld	l, c
		call	sub_96D1
		add	hl, bc
		ld	a, (ix+2)
		and	80h ; 'А'
		or	h
		ld	(ScanerCrdXobj), a
		xor	a
		ld	l, (ix+3)
		ld	h, (ix+4)
		call	Div_BC_HLAdivDE
; ---------------------------------------------------------------------------
		call	sub_96C5
		ld	h, b
		ld	l, c
		call	sub_96CD
		add	hl, bc
		ld	a, (ix+5)
		and	80h ; 'А'
		ld	a, h
		jr	z, loc_9765
		or	80h ; 'А'

loc_9765:				; CODE XREF: sub_9736+2Bj
		ld	(ScanerCrdYobj), a
		xor	a
		ld	l, (ix+6)
		ld	h, (ix+7)
		call	Div_BC_HLAdivDE
; ---------------------------------------------------------------------------
		call	sub_96C5
		ld	h, b
		ld	l, c
		call	sub_96CD
		add	hl, bc
		ld	a, (ix+8)
		and	80h ; 'А'
		ld	a, h
		jr	z, loc_9785
		or	80h ; 'А'

loc_9785:				; CODE XREF: sub_9736+4Bj
		ld	(ScanerCrdZobj), a
		ld	hl, 558Fh
		ld	a, (ScanerCrdXobj)
		and	7Fh ; ''
		sub	4
		ld	c, a
		jp	c, loc_97BB
		rra
		rra
		rra
		and	1Fh
		ld	b, a
		ld	a, c
		and	7
		ld	c, a
		jr	z, loc_97A3
		inc	b

loc_97A3:				; CODE XREF: sub_9736+6Aj
		ld	a, (ScanerCrdXobj)
		and	80h ; 'А'
		jr	nz, loc_97C7

loc_97AA:				; CODE XREF: sub_9736+75j
		inc	hl
		djnz	loc_97AA

loc_97AD:				; CODE XREF: sub_9736+8Fj
		ld	a, c
		and	a
		ld	b, a
		ld	a, 1
		jr	z, loc_97D6
		xor	a
		scf

loc_97B6:				; CODE XREF: sub_9736+81j
		rra
		djnz	loc_97B6
		jr	loc_97D6
; ---------------------------------------------------------------------------

loc_97BB:				; CODE XREF: sub_9736+5Dj
		add	a, 8
		ld	c, a
		ld	a, (ScanerCrdXobj)
		and	80h ; 'А'
		jr	nz, loc_97CA
		jr	loc_97AD
; ---------------------------------------------------------------------------

loc_97C7:				; CODE XREF: sub_9736+72j sub_9736+92j
		dec	hl
		djnz	loc_97C7

loc_97CA:				; CODE XREF: sub_9736+8Dj
		ld	a, c
		and	a
		ld	b, a
		ld	a, 80h ; 'А'
		jr	z, loc_97D6
		xor	a
		scf

loc_97D3:				; CODE XREF: sub_9736+9Ej
		rla
		djnz	loc_97D3

loc_97D6:				; CODE XREF: sub_9736+7Cj sub_9736+83j ...
		ld	c, a
		ld	a, (ScanerCrdZobj)
		ld	d, a
		and	7Fh ; ''
		ld	b, a
		jr	z, loc_97F9
		ld	a, d
		and	80h ; 'А'
		jr	nz, loc_97F0

loc_97E5:				; CODE XREF: sub_9736+B6j
		ld	a, h
		dec	h
		and	7
		call	z, compUpScrAdr_HL
		djnz	loc_97E5
		jr	loc_97F9
; ---------------------------------------------------------------------------

loc_97F0:				; CODE XREF: sub_9736+ADj sub_9736+C1j
		inc	h
		ld	a, h
		and	7
		call	z, compDownScrAdr_HL
		djnz	loc_97F0

loc_97F9:				; CODE XREF: sub_9736+A8j sub_9736+B8j
		ld	a, (ScanerCrdYobj)
		ld	d, a
		and	7Fh ; ''
		ld	b, a
		jr	z, loc_9821
		ld	a, d
		and	80h ; 'А'
		jr	nz, loc_9815

loc_9807:				; CODE XREF: sub_9736+DBj
		ld	a, (hl)
		or	c
		ld	(hl), a
		ld	a, h
		dec	h
		and	7
		call	z, sub_987C
		djnz	loc_9807
		jr	loc_9821
; ---------------------------------------------------------------------------

loc_9815:				; CODE XREF: sub_9736+CFj sub_9736+E9j
		ld	a, (hl)
		or	c
		ld	(hl), a
		inc	h
		ld	a, h
		and	7
		call	z, sub_986A
		djnz	loc_9815

loc_9821:				; CODE XREF: sub_9736+CAj sub_9736+DDj
		ld	a, c
		add	a, a
		jr	c, sub_9852
		or	c
		add	a, a
		jr	c, loc_985E
		or	c
		ld	e, a
		ld	a, (hl)
		or	e
		ld	(hl), a
; End of function sub_9736

; START	OF FUNCTION CHUNK FOR sub_96DC

loc_982E:				; CODE XREF: sub_96DC:loc_96D6j
					; sub_9852+Aj ...
		ld	a, (ScanerNumObj)
		dec	a
		jr	z, loc_983F
		ld	(ScanerNumObj),	a
		ld	de, 27h	; '''
		add	ix, de
		jp	loc_96ED
; ---------------------------------------------------------------------------

loc_983F:				; CODE XREF: sub_96DC+156j
		xor	a
		ld	c, 7
		ld	hl, 5009h

loc_9845:				; CODE XREF: sub_96DC+173j
		ld	b, 0Dh
		push	hl

loc_9848:				; CODE XREF: sub_96DC+16Ej
		ld	(hl), a
		inc	l
		djnz	loc_9848
		pop	hl
		inc	h
		dec	c
		jr	nz, loc_9845
		ret
; END OF FUNCTION CHUNK	FOR sub_96DC

; =============== S U B	R O U T	I N E =======================================




; === sub_99B5 === sub_99B5:
sub_99B5:
		rlca

loc_99B6:				; CODE XREF: RAM:99AEj
		ld	a, (hl)
		cp	63h ; 'c'
		jr	z, loc_99BD
		ld	(hl), 2

loc_99BD:				; CODE XREF: sub_99B5+4j
		ld	a, (hl)
		cp	63h ; 'c'
		jr	nz, loc_99CF
		ld	hl, unk_99A3
		inc	(hl)
		bit	1, (hl)
		jr	nz, loc_99CD
		xor	a
		jr	loc_99CF
; ---------------------------------------------------------------------------

loc_99CD:				; CODE XREF: sub_99B5+13j
		ld	a, 2

loc_99CF:				; CODE XREF: sub_99B5+Bj sub_99B5+16j
		ld	d, a
		ld	a, (WhatOnScreen)
		cp	4
		jr	nc, loc_99F8
		ld	hl, 5A28h
		ld	(hl), d
		inc	hl
		ld	(hl), d
		ld	hl, 5A48h
		ld	(hl), d
		inc	hl
		ld	(hl), d
		call	sub_9A3A
		ld	a, (Missile1)
		cp	1
		jr	nz, loc_99EF
		ld	a, 4

loc_99EF:				; CODE XREF: sub_99B5+36j
		add	a, a
		add	a, a
		add	a, a
		add	a, 7
		ld	(5AE7h), a
		ret
; ---------------------------------------------------------------------------

loc_99F8:				; CODE XREF: sub_99B5+20j
		ld	hl, (word_D04B)
		ld	(hl), 0
		ld	hl, 11Bh
		ld	(CrdXPrnSymbol), hl
		cp	9
		jr	nc, loc_9A23
		ld	a, d
		and	a
		jr	z, loc_9A1D
		cp	2
		jr	nz, loc_9A1F
		ld	a, 21h ; '!'
		call	j_PrintMessage1_A
		ld	a, 22h ; '"'
		call	j_PrintMessage1_A
		ld	d, 17h
		jr	loc_9A2F
; ---------------------------------------------------------------------------

loc_9A1D:				; CODE XREF: sub_99B5+54j
		ld	d, 2

loc_9A1F:				; CODE XREF: sub_99B5+58j
		ld	a, 10h
		add	a, d
		ld	d, a

loc_9A23:				; CODE XREF: sub_99B5+50j
		push	de
		ld	a, 23h ; '#'
		call	j_PrintMessage1_A
		ld	a, 24h ; '$'
		call	j_PrintMessage1_A
		pop	de

loc_9A2F:				; CODE XREF: sub_99B5+66j
		ld	hl, 583Ch
		ld	(hl), d
		inc	hl
		ld	(hl), d
		ld	hl, (word_D04B)
		ld	(hl), 0A9h ; 'й'
; End of function sub_99B5


; =============== S U B	R O U T	I N E =======================================




; === sub_99B5 === sub_99B5:
sub_99B5:
		rlca

loc_99B6:				; CODE XREF: RAM:99AEj
		ld	a, (hl)
		cp	63h ; 'c'
		jr	z, loc_99BD
		ld	(hl), 2

loc_99BD:				; CODE XREF: sub_99B5+4j
		ld	a, (hl)
		cp	63h ; 'c'
		jr	nz, loc_99CF
		ld	hl, unk_99A3
		inc	(hl)
		bit	1, (hl)
		jr	nz, loc_99CD
		xor	a
		jr	loc_99CF
; ---------------------------------------------------------------------------

loc_99CD:				; CODE XREF: sub_99B5+13j
		ld	a, 2

loc_99CF:				; CODE XREF: sub_99B5+Bj sub_99B5+16j
		ld	d, a
		ld	a, (WhatOnScreen)
		cp	4
		jr	nc, loc_99F8
		ld	hl, 5A28h
		ld	(hl), d
		inc	hl
		ld	(hl), d
		ld	hl, 5A48h
		ld	(hl), d
		inc	hl
		ld	(hl), d
		call	sub_9A3A
		ld	a, (Missile1)
		cp	1
		jr	nz, loc_99EF
		ld	a, 4

loc_99EF:				; CODE XREF: sub_99B5+36j
		add	a, a
		add	a, a
		add	a, a
		add	a, 7
		ld	(5AE7h), a
		ret
; ---------------------------------------------------------------------------

loc_99F8:				; CODE XREF: sub_99B5+20j
		ld	hl, (word_D04B)
		ld	(hl), 0
		ld	hl, 11Bh
		ld	(CrdXPrnSymbol), hl
		cp	9
		jr	nc, loc_9A23
		ld	a, d
		and	a
		jr	z, loc_9A1D
		cp	2
		jr	nz, loc_9A1F
		ld	a, 21h ; '!'
		call	j_PrintMessage1_A
		ld	a, 22h ; '"'
		call	j_PrintMessage1_A
		ld	d, 17h
		jr	loc_9A2F
; ---------------------------------------------------------------------------

loc_9A1D:				; CODE XREF: sub_99B5+54j
		ld	d, 2

loc_9A1F:				; CODE XREF: sub_99B5+58j
		ld	a, 10h
		add	a, d
		ld	d, a

loc_9A23:				; CODE XREF: sub_99B5+50j
		push	de
		ld	a, 23h ; '#'
		call	j_PrintMessage1_A
		ld	a, 24h ; '$'
		call	j_PrintMessage1_A
		pop	de

loc_9A2F:				; CODE XREF: sub_99B5+66j
		ld	hl, 583Ch
		ld	(hl), d
		inc	hl
		ld	(hl), d
		ld	hl, (word_D04B)
		ld	(hl), 0A9h ; 'й'
; End of function sub_99B5


; =============== S U B	R O U T	I N E =======================================




; === FrameCounter === FrameCounter:	db 96h			; DATA XREF: CounterShipHyperJumpr
FrameCounter:	db 96h			; DATA XREF: CounterShipHyperJumpr
					; Use_J_Drive+37w ...


; === J_Drive_Ready4 === J_Drive_Ready4:	db 1			; DATA XREF: Use_J_Drive:loc_7EFBr
J_Drive_Ready4:	db 1			; DATA XREF: Use_J_Drive:loc_7EFBr
					; MoveAllObjects+55w ...


; === EnergyBombUsed === EnergyBombUsed:	db 0			; DATA XREF: UseEnergyBomb+Fw
EnergyBombUsed:	db 0			; DATA XREF: UseEnergyBomb+Fw
					; sub_7C60w ...


; === PlayerPirate === PlayerPirate:	db 0			; DATA XREF: Use_Y_Cloacking+2Ew
PlayerPirate:	db 0			; DATA XREF: Use_Y_Cloacking+2Ew
					; ClearAroundObj+9w ...


; === TimeWorkECM === TimeWorkECM:	db 0			; DATA XREF: LaunchDocking+37w
TimeWorkECM:	db 0			; DATA XREF: LaunchDocking+37w
					; sub_9946+13w	...

; =============== S U B	R O U T	I N E =======================================

; Attributes: noreturn thunk



; === FlgCoriolisNear === FlgCoriolisNear:db 10h			; DATA XREF: SetIndStationr
FlgCoriolisNear:db 10h			; DATA XREF: SetIndStationr
					; MoveAllObjects+4Fw ...

; =============== S U B	R O U T	I N E =======================================

; Attributes: thunk



; === InverseNumSlotIX === InverseNumSlotIX:db 1			; DATA XREF: sub_B4B1w
InverseNumSlotIX:db 1			; DATA XREF: sub_B4B1w
					; OffTargetMissilesr ...

; =============== S U B	R O U T	I N E =======================================

; Attributes: noreturn thunk



; === sub_9D40 === sub_9D40:
sub_9D40:

; FUNCTION CHUNK AT F8B4 SIZE 00000023 BYTES

		and	a
		jp	z, loc_A52E	; следующий объект
		call	sub_F916
		bit	2, (ix+26h)
		jr	nz, loc_9D50
		ld	(FlgCoriolisNear), a

loc_9D50:				; CODE XREF: sub_9D40+Bj
		call	CounterShipHyperJump
		ld	a, c
		bit	3, (ix+26h)
		jr	nz, loc_9D5F
		ld	a, 1
		ld	(J_Drive_Ready4), a

loc_9D5F:				; CODE XREF: sub_9D40+18j
		ld	a, (EnergyBombUsed)
		and	a
		jr	z, loc_9D76
		bit	2, (ix+26h)
		jr	nz, loc_9D76
		ld	a, (iy+6)
		cp	27h ; '''
		jr	z, loc_9D76
		set	4, (ix+20h)

loc_9D76:				; CODE XREF: sub_9D40+23j sub_9D40+29j ...
		bit	2, (ix+21h)
		jr	z, loc_9D88
		ld	a, (ix+21h)
		and	60h ; '`'
		jr	z, loc_9D88
		ld	a, 1
		ld	(PlayerPirate),	a

loc_9D88:				; CODE XREF: sub_9D40+3Aj sub_9D40+41j
		ld	a, (objMissile.field_22)
		and	a
		jr	nz, loc_9D94
		ld	a, (PlayerPirate)
		and	a
		jr	z, loc_9D98

loc_9D94:				; CODE XREF: sub_9D40+4Cj
		call	SetAgrFireLaser	; установка агрессии при попадании из лазера
		nop

loc_9D98:				; CODE XREF: sub_9D40+52j
		ld	h, (ix+24h)
		ld	l, (ix+23h)
		push	hl
		pop	iy
		ld	a, (ix+20h)
		and	30h ; '0'
		jr	z, loc_9E0F
		sub	20h ; ' '
		jp	nz, loc_9E41
		res	5, (ix+20h)
		ld	(ix+1Eh), a
		ld	a, 0C0h	; '└'
		ld	(ix+1Dh), a
		bit	0, (ix+26h)
		jr	nz, loc_9DE1
		ld	hl, nBeepFX
		set	2, (hl)
		ld	a, (Mission2)
		and	a
		jr	z, loc_9DE1
		cp	3
		jr	z, loc_9DD6
		bit	6, (ix+26h)
		jr	z, loc_9DE1
		jr	loc_9DDD
; ---------------------------------------------------------------------------

loc_9DD6:				; CODE XREF: sub_9D40+8Cj
		ld	a, (iy+1)
		cp	1Eh
		jr	nz, loc_9DE1

loc_9DDD:				; CODE XREF: sub_9D40+94j
		xor	a
		ld	(Mission2), a

loc_9DE1:				; CODE XREF: sub_9D40+7Dj sub_9D40+88j ...
		xor	a
		ld	(ix+22h), a
		call	OffTargetMissiles
		ld	d, (iy+9)
		ld	e, (iy+8)
		call	AddCashObj
		ld	d, (iy+16h)
		ld	e, (iy+15h)
		srl	d
		rr	e
		call	IncBattleRating
		ld	a, (ix+21h)
		and	60h ; '`'
		jr	z, loc_9E5C
		push	hl
		ld	hl, PilotLegalStatus
		call	sub_FC25
		pop	hl
		jr	loc_9E5C
; ---------------------------------------------------------------------------

loc_9E0F:				; CODE XREF: sub_9D40+66j
		ld	a, b
		xor	c
		and	0Fh
		bit	6, (ix+20h)
		jr	z, loc_9E29
		call	sub_FA0D
		ld	a, c
		jr	z, loc_9E24
		call	MoveMissile
		jr	loc_9E29
; ---------------------------------------------------------------------------

loc_9E24:				; CODE XREF: sub_9D40+DDj
		and	3
		call	z, ReactionObject2

loc_9E29:				; CODE XREF: sub_9D40+D7j sub_9D40+E2j
		call	ReCalcCrdObj
		call	CorrAxirY_SKO
		push	ix
		ld	de, 9
		add	ix, de
		call	CorrAxisVector
		call	CorrAxisVector
		call	CorrAxisVector
		pop	ix

loc_9E41:				; CODE XREF: sub_9D40+6Aj
		call	CollisionWithObj
		jp	loc_F8B4
; ---------------------------------------------------------------------------

loc_9E47:				; CODE XREF: sub_9D40+5B94j
		res	3, (ix+20h)
		ld	a, (ix+20h)
		and	30h ; '0'
		cp	10h
		jr	nz, loc_9E5C
		set	5, (ix+20h)
		res	4, (ix+20h)

loc_9E5C:				; CODE XREF: CounterShipHyperJump+1Cj
					; sub_9D40+C3j	...
		pop	af
		call	NextExtSlot
		add	ix, de
		dec	a
		jp	sub_F8E4
; End of function sub_9D40


; =============== S U B	R O U T	I N E =======================================




; === sub_9D40 === sub_9D40:
sub_9D40:

; FUNCTION CHUNK AT F8B4 SIZE 00000023 BYTES

		and	a
		jp	z, loc_A52E	; следующий объект
		call	sub_F916
		bit	2, (ix+26h)
		jr	nz, loc_9D50
		ld	(FlgCoriolisNear), a

loc_9D50:				; CODE XREF: sub_9D40+Bj
		call	CounterShipHyperJump
		ld	a, c
		bit	3, (ix+26h)
		jr	nz, loc_9D5F
		ld	a, 1
		ld	(J_Drive_Ready4), a

loc_9D5F:				; CODE XREF: sub_9D40+18j
		ld	a, (EnergyBombUsed)
		and	a
		jr	z, loc_9D76
		bit	2, (ix+26h)
		jr	nz, loc_9D76
		ld	a, (iy+6)
		cp	27h ; '''
		jr	z, loc_9D76
		set	4, (ix+20h)

loc_9D76:				; CODE XREF: sub_9D40+23j sub_9D40+29j ...
		bit	2, (ix+21h)
		jr	z, loc_9D88
		ld	a, (ix+21h)
		and	60h ; '`'
		jr	z, loc_9D88
		ld	a, 1
		ld	(PlayerPirate),	a

loc_9D88:				; CODE XREF: sub_9D40+3Aj sub_9D40+41j
		ld	a, (objMissile.field_22)
		and	a
		jr	nz, loc_9D94
		ld	a, (PlayerPirate)
		and	a
		jr	z, loc_9D98

loc_9D94:				; CODE XREF: sub_9D40+4Cj
		call	SetAgrFireLaser	; установка агрессии при попадании из лазера
		nop

loc_9D98:				; CODE XREF: sub_9D40+52j
		ld	h, (ix+24h)
		ld	l, (ix+23h)
		push	hl
		pop	iy
		ld	a, (ix+20h)
		and	30h ; '0'
		jr	z, loc_9E0F
		sub	20h ; ' '
		jp	nz, loc_9E41
		res	5, (ix+20h)
		ld	(ix+1Eh), a
		ld	a, 0C0h	; '└'
		ld	(ix+1Dh), a
		bit	0, (ix+26h)
		jr	nz, loc_9DE1
		ld	hl, nBeepFX
		set	2, (hl)
		ld	a, (Mission2)
		and	a
		jr	z, loc_9DE1
		cp	3
		jr	z, loc_9DD6
		bit	6, (ix+26h)
		jr	z, loc_9DE1
		jr	loc_9DDD
; ---------------------------------------------------------------------------

loc_9DD6:				; CODE XREF: sub_9D40+8Cj
		ld	a, (iy+1)
		cp	1Eh
		jr	nz, loc_9DE1

loc_9DDD:				; CODE XREF: sub_9D40+94j
		xor	a
		ld	(Mission2), a

loc_9DE1:				; CODE XREF: sub_9D40+7Dj sub_9D40+88j ...
		xor	a
		ld	(ix+22h), a
		call	OffTargetMissiles
		ld	d, (iy+9)
		ld	e, (iy+8)
		call	AddCashObj
		ld	d, (iy+16h)
		ld	e, (iy+15h)
		srl	d
		rr	e
		call	IncBattleRating
		ld	a, (ix+21h)
		and	60h ; '`'
		jr	z, loc_9E5C
		push	hl
		ld	hl, PilotLegalStatus
		call	sub_FC25
		pop	hl
		jr	loc_9E5C
; ---------------------------------------------------------------------------

loc_9E0F:				; CODE XREF: sub_9D40+66j
		ld	a, b
		xor	c
		and	0Fh
		bit	6, (ix+20h)
		jr	z, loc_9E29
		call	sub_FA0D
		ld	a, c
		jr	z, loc_9E24
		call	MoveMissile
		jr	loc_9E29
; ---------------------------------------------------------------------------

loc_9E24:				; CODE XREF: sub_9D40+DDj
		and	3
		call	z, ReactionObject2

loc_9E29:				; CODE XREF: sub_9D40+D7j sub_9D40+E2j
		call	ReCalcCrdObj
		call	CorrAxirY_SKO
		push	ix
		ld	de, 9
		add	ix, de
		call	CorrAxisVector
		call	CorrAxisVector
		call	CorrAxisVector
		pop	ix

loc_9E41:				; CODE XREF: sub_9D40+6Aj
		call	CollisionWithObj
		jp	loc_F8B4
; ---------------------------------------------------------------------------

loc_9E47:				; CODE XREF: sub_9D40+5B94j
		res	3, (ix+20h)
		ld	a, (ix+20h)
		and	30h ; '0'
		cp	10h
		jr	nz, loc_9E5C
		set	5, (ix+20h)
		res	4, (ix+20h)

loc_9E5C:				; CODE XREF: CounterShipHyperJump+1Cj
					; sub_9D40+C3j	...
		pop	af
		call	NextExtSlot
		add	ix, de
		dec	a
		jp	sub_F8E4
; End of function sub_9D40


; =============== S U B	R O U T	I N E =======================================




; === sub_A6C9 === sub_A6C9:
sub_A6C9:
		pop	de
		call	AddSign_L_E
		jr	nc, loc_A6D1
		ld	a, 0FFh

loc_A6D1:				; CODE XREF: sub_A6C9+4j
		pop	de
		ld	l, a
		call	AddSign_L_E
		jr	nc, loc_A6DA
		ld	a, 0FFh

loc_A6DA:				; CODE XREF: sub_A6C9+Dj
		rl	h
		rra
		ret
; End of function sub_A6C9

; ---------------------------------------------------------------------------


; === sub_A6C9 === sub_A6C9:
sub_A6C9:
		pop	de
		call	AddSign_L_E
		jr	nc, loc_A6D1
		ld	a, 0FFh

loc_A6D1:				; CODE XREF: sub_A6C9+4j
		pop	de
		ld	l, a
		call	AddSign_L_E
		jr	nc, loc_A6DA
		ld	a, 0FFh

loc_A6DA:				; CODE XREF: sub_A6C9+Dj
		rl	h
		rra
		ret
; End of function sub_A6C9

; ---------------------------------------------------------------------------


; === RadiusBlast === RadiusBlast:	db 28h			; DATA XREF: sub_7E02+50r
RadiusBlast:	db 28h			; DATA XREF: sub_7E02+50r
					; PrnObjIfVisible+92w ...

; =============== S U B	R O U T	I N E =======================================




; === VergesVisible === VergesVisible:	db    0			; DATA XREF: CalcVisibleVerges+14o
VergesVisible:	db    0			; DATA XREF: CalcVisibleVerges+14o
		db    1
		db    0
		db    1
		db    0
		db    0
		db    0
		db    1
		db    0
		db    1
byte_A98E:	db 1			; DATA XREF: TestVergesVisible:loc_AA27r
		db    0
		db    1
		db    0
		db    1
		db    1


; === VertexIncFactor === VertexIncFactor:db 1			; DATA XREF: RotateCrdN_Vertex:loc_AAE3r
VertexIncFactor:db 1			; DATA XREF: RotateCrdN_Vertex:loc_AAE3r
					; sub_AB1Er ...


; === VisiblePrevVertex === VisiblePrevVertex:db 0			; DATA XREF: MakeTablPrnCrdVertex+3Ew
VisiblePrevVertex:db 0			; DATA XREF: MakeTablPrnCrdVertex+3Ew
					; RotateCrdN_Vertexr ...

; =============== S U B	R O U T	I N E =======================================




; === PlayerAggressor === PlayerAggressor:db 0			; DATA XREF: SetBit2ExtSlot_rndA+5o
PlayerAggressor:db 0			; DATA XREF: SetBit2ExtSlot_rndA+5o
					; sub_7D64:loc_7D7Fr ...
byte_B457:	db 0, 0, 0, 0, 0, 0, 0,	0, 0, 0	; DATA XREF: sub_7C60+11o
					; sub_7D64o ...
					; 5,+0 =0 - объектом был запущен корабль
					; +1,+2,+3 - печатные координаты вершины с лазером
					; +1,+2,+3 - печатные координаты вершины №3 xor	3
					; +8 биты 0-3,=0 можно увеличить рейтинг
byte_B461:	db 0, 0, 0, 0, 0, 0, 0,	0, 0, 0	; DATA XREF: sub_7C13+2Fo
					; sub_FA0D+42o
		db 0, 0, 0, 0, 0, 0, 0,	0, 0, 0
		db 0, 0, 0, 0, 0, 0, 0,	0, 0, 0
		db 0, 0, 0, 0, 0, 0, 0,	0, 0, 0
		db 0, 0, 0, 0, 0, 0, 0,	0, 0, 0
		db 0, 0, 0, 0, 0, 0, 0,	0, 0, 0
		db 0, 0, 0, 0, 0, 0, 0,	0, 0, 0
byte_B4A7:	db 0			; DATA XREF: sub_9391+3o
		db 0
		db 5
		db 5Bh
		db 1
		db 13h
		db 17h
		db 0
		db 0
		db 0

; =============== S U B	R O U T	I N E =======================================




; === SKO4Intro === SKO4Intro:	db  58h	; X		; DATA XREF: SetShip4Intro+3Bo
SKO4Intro:	db  58h	; X		; DATA XREF: SetShip4Intro+3Bo
		db  44h	; D
		db  41h	; A
		db  98h	; Ш
		db  64h	; d
		db 0CBh	; ╦
		db  5Ah	; Z
		db 0A7h	; з
		db 0D1h	; ╤
byte_B866:	db 0			; DATA XREF: RotateSKOobjr
					; RotateSKOobj+5w

; =============== S U B	R O U T	I N E =======================================

; Attributes: noreturn thunk



; === IY_dFactor === IY_dFactor:	db 0			; DATA XREF: CalcVisibleVerges+1Aw
IY_dFactor:	db 0			; DATA XREF: CalcVisibleVerges+1Aw
					; RotateCrdN_Normal+19r


; === TablSqr === TablSqr:	db 0, 1, 4, 9, 10h, 19h, 24h, 31h
TablSqr:	db 0, 1, 4, 9, 10h, 19h, 24h, 31h
		db 40h,	51h, 64h, 79h, 90h, 0A9h, 0C4h,	0E1h
		db 0, 21h, 44h,	69h, 90h, 0B9h,	0E4h, 11h
		db 40h,	71h, 0A4h, 0D9h, 10h, 49h, 84h,	0C1h
		db 0, 41h, 84h,	0C9h, 10h, 59h,	0A4h, 0F1h
		db 40h,	91h, 0E4h, 39h,	90h, 0E9h, 44h,	0A1h
		db 0, 61h, 0C4h, 29h, 90h, 0F9h, 64h, 0D1h
		db 40h,	0B1h, 24h, 99h,	10h, 89h, 4, 81h
		db 0, 81h, 4, 89h, 10h,	99h, 24h, 0B1h
		db 40h,	0D1h, 64h, 0F9h, 90h, 29h, 0C4h, 61h
		db 0, 0A1h, 44h, 0E9h, 90h, 39h, 0E4h, 91h
		db 40h,	0F1h, 0A4h, 59h, 10h, 0C9h, 84h, 41h
		db 0, 0C1h, 84h, 49h, 10h, 0D9h, 0A4h, 71h
		db 40h,	11h, 0E4h, 0B9h, 90h, 69h, 44h,	21h
		db 0, 0E1h, 0C4h, 0A9h,	90h, 79h, 64h, 51h
		db 40h,	31h, 24h, 19h, 10h, 9, 4, 1
		db 0, 1, 4, 9, 10h, 19h, 24h, 31h
		db 40h,	51h, 64h, 79h, 90h, 0A9h, 0C4h,	0E1h
		db 0, 21h, 44h,	69h, 90h, 0B9h,	0E4h, 11h
		db 40h,	71h, 0A4h, 0D9h, 10h, 49h, 84h,	0C1h
		db 0, 41h, 84h,	0C9h, 10h, 59h,	0A4h, 0F1h
		db 40h,	91h, 0E4h, 39h,	90h, 0E9h, 44h,	0A1h
		db 0, 61h, 0C4h, 29h, 90h, 0F9h, 64h, 0D1h
		db 40h,	0B1h, 24h, 99h,	10h, 89h, 4, 81h
		db 0, 81h, 4, 89h, 10h,	99h, 24h, 0B1h
		db 40h,	0D1h, 64h, 0F9h, 90h, 29h, 0C4h, 61h
		db 0, 0A1h, 44h, 0E9h, 90h, 39h, 0E4h, 91h
		db 40h,	0F1h, 0A4h, 59h, 10h, 0C9h, 84h, 41h
		db 0, 0C1h, 84h, 49h, 10h, 0D9h, 0A4h, 71h
		db 40h,	11h, 0E4h, 0B9h, 90h, 69h, 44h,	21h
		db 0, 0E1h, 0C4h, 0A9h,	90h, 79h, 64h, 51h
		db 40h,	31h, 24h, 19h, 10h, 9, 4, 1
		db 0, 0, 0, 0, 0, 0, 0,	0
		db 0, 0, 0, 0, 0, 0, 0,	0
		db 1, 1, 1, 1, 1, 1, 1,	2
		db 2, 2, 2, 2, 3, 3, 3,	3
		db 4, 4, 4, 4, 5, 5, 5,	5
		db 6, 6, 6, 7, 7, 7, 8,	8
		db 9, 9, 9, 0Ah, 0Ah, 0Ah, 0Bh,	0Bh
		db 0Ch,	0Ch, 0Dh, 0Dh, 0Eh, 0Eh, 0Fh, 0Fh
		db 10h,	10h, 11h, 11h, 12h, 12h, 13h, 13h
		db 14h,	14h, 15h, 15h, 16h, 17h, 17h, 18h
		db 19h,	19h, 1Ah, 1Ah, 1Bh, 1Ch, 1Ch, 1Dh
		db 1Eh,	1Eh, 1Fh, 20h, 21h, 21h, 22h, 23h
		db 24h,	24h, 25h, 26h, 27h, 27h, 28h, 29h
		db 2Ah,	2Bh, 2Bh, 2Ch, 2Dh, 2Eh, 2Fh, 30h
		db 31h,	31h, 32h, 33h, 34h, 35h, 36h, 37h
		db 38h,	39h, 3Ah, 3Bh, 3Ch, 3Dh, 3Eh, 3Fh
		db 40h,	41h, 42h, 43h, 44h, 45h, 46h, 47h
		db 48h,	49h, 4Ah, 4Bh, 4Ch, 4Dh, 4Eh, 4Fh
		db 51h,	52h, 53h, 54h, 55h, 56h, 57h, 59h
		db 5Ah,	5Bh, 5Ch, 5Dh, 5Fh, 60h, 61h, 62h
		db 64h,	65h, 66h, 67h, 69h, 6Ah, 6Bh, 6Ch
		db 6Eh,	6Fh, 70h, 72h, 73h, 74h, 76h, 77h
		db 79h,	7Ah, 7Bh, 7Dh, 7Eh, 7Fh, 81h, 82h
		db 84h,	85h, 87h, 88h, 8Ah, 8Bh, 8Dh, 8Eh
		db 90h,	91h, 93h, 94h, 96h, 97h, 99h, 9Ah
		db 9Ch,	9Dh, 9Fh, 0A0h,	0A2h, 0A4h, 0A5h, 0A7h
		db 0A9h, 0AAh, 0ACh, 0ADh, 0AFh, 0B1h, 0B2h, 0B4h
		db 0B6h, 0B7h, 0B9h, 0BBh, 0BDh, 0BEh, 0C0h, 0C2h
		db 0C4h, 0C5h, 0C7h, 0C9h, 0CBh, 0CCh, 0CEh, 0D0h
		db 0D2h, 0D4h, 0D5h, 0D7h, 0D9h, 0DBh, 0DDh, 0DFh
		db 0E1h, 0E2h, 0E4h, 0E6h, 0E8h, 0EAh, 0ECh, 0EEh
		db 0F0h, 0F2h, 0F4h, 0F6h, 0F8h, 0FAh, 0FCh, 0FEh


; === DataCocpit === DataCocpit:	db    0			; DATA XREF: PrnCocpit_Ifn+22o
DataCocpit:	db    0			; DATA XREF: PrnCocpit_Ifn+22o
					; SaveCocpitInBuffer+5o ...
		db    0
		db    0
		db    0
		db    0
		db 0
		db    0
		db    0
		db    0
		db    0
		db    0
		db    0
		db    0
		db    0
		db    0
		db    0
		db    0
		db    0
		db    0
		db    0
		db    0
		db    0
		db    0
		db    0
		db    0
		db    0
		db    0
		db    0
		db    0
		db    0
		db    0
		db    0
		db    0
		db  80h	; А
		db    1
		db    0
		db    0
		db    0
		db    0
		db    1
		db    0
		db    0
		db    0
		db    0
		db    0
		db    0
		db    0
		db    0
		db    0
		db    0
		db    0
		db    0
		db    0
		db    0
		db    0
		db    1
		db    0
		db    0
		db    0
		db    0
		db    1
		db    0
		db    1
		db    0
		db    0
		db  80h	; А
		db    1
		db    0
		db    0
		db    0
		db    0
		db    1
		db  7Fh	; 
		db 0FEh	; ■
		db    0
		db    0
		db    0
		db    0
		db    0
		db    0
		db    0
		db    0
		db    0
		db    0
		db    0
		db  20h
		db    0
		db    5
		db    0
		db    0
		db    0
		db    0
		db    1
		db    0
		db    1
		db    0
		db    0
		db  80h	; А
		db    1
		db    0
		db    0
		db    0
		db    0
		db    1
		db    0
		db    0
		db    0
		db    7
		db    0
		db  44h	; D
		db    0
		db    0
		db    0
		db  11h
		db    0
		db  60h	; `
		db    0
		db  10h
		db    0
		db    9
		db    0
		db    0
		db    0
		db    0
		db    1
		db    0
		db    1
		db    0
		db    0
		db  80h	; А
		db    1
		db    0
		db    0
		db    0
		db    0
		db    1
		db    0
		db    0
		db    0
		db    0
		db  40h	; @
		db    0
		db    4
		db    0
		db  10h
		db    0
		db    1
		db    0
		db    0
		db  40h	; @
		db    0
		db    1
		db    0
		db    0
		db    0
		db    0
		db    1
		db    0
		db    1
		db    0
		db    0
		db  80h	; А
		db    1
		db    0
		db    0
		db    0
		db    0
		db    1
		db    0
		db    2
		db    0
		db  40h	; @
		db    0
		db    0
		db    0
		db    4
		db    0
		db    0
		db    0
		db    1
		db    0
		db  20h
		db    0
		db    1
		db    0
		db    0
		db    0
		db    0
		db    1
		db    0
		db    1
		db    0
		db    0
		db  80h	; А
		db    1
		db    0
		db    0
		db    0
		db    0
		db    1
		db    0
		db    0
		db    0
		db 0C0h	; └
		db    0
		db    0
		db    0
		db    4
		db    0
		db    0
		db    0
		db    1
		db 0C0h	; └
		db  7Fh	; 
		db  3Eh	; >
		db    1
		db    0
		db    0
		db    0
		db    0
		db    1
		db    0
		db    1
		db    0
		db    0
		db  80h	; А
		db    1
		db    0
		db    0
		db    0
		db    0
		db  81h	; Б
		db    0
		db    0
		db    0
		db    0
		db    0
		db    0
		db    0
		db    0
		db    0
		db    0
		db    0
		db    0
		db    0
		db    0
		db    0
		db    1
		db    0
		db    0
		db    0
		db    0
		db    1
		db    0
		db    1
		db    0
		db    0
		db    0
		db    0
		db    0
		db    0
		db    0
byte_C106:	db 0			; DATA XREF: GoJump:loc_DC04o
		db    0
		db    0
		db    0
		db    0
		db    0
		db    0
		db    0
		db    0
		db    0
		db    0
		db    0
		db    0
		db    0
		db    0
		db    0
		db    0
		db    0
		db    0
		db    0
		db    0
		db    0
		db    0
		db    0
		db    0
		db    0
		db    0
		db 0BFh	; ┐
		db  3Dh	; =
		db 0FFh
		db 0FFh
		db 0FFh
		db 0FFh
		db 0FFh
		db    3
		db 0C0h	; └
		db    0
		db    0
		db    0
		db    0
		db    0
		db    0
		db    0
		db    0
		db    0
		db    0
		db    0
		db    0
		db  7Eh	; ~
		db    1
		db 0FFh
		db 0FFh
		db 0FFh
		db 0FFh
		db 0FFh
		db  3Eh	; >
		db 0FDh	; ¤
		db    0
		db    0
		db 0BCh	; ╝
		db  3Dh	; =
		db 0FFh
		db 0FFh
		db 0FFh
		db 0FFh
		db 0FFh
		db  7Fh	; 
		db 0FEh	; ■
		db    0
		db    0
		db    0
		db    0
		db    0
		db    0
		db    0
		db    0
		db    0
		db    0
		db    0
		db  20h
		db  3Ch	; <
		db    5
		db    0
		db    0
		db  18h
		db    0
		db    1
		db  7Eh	; ~
		db  81h	; Б
		db    0
		db    0
		db 0BFh	; ┐
		db  45h	; E
		db    0
		db    0
		db    0
		db    0
		db    1
		db    0
		db    0
		db    0
		db  30h	; 0
		db    0
		db    2
		db    0
		db    0
		db    0
		db  20h
		db    0
		db  0Eh
		db    0
		db    8
		db    0
		db  11h
		db    0
		db    0
		db  18h
		db    0
		db    1
		db  78h	; x
		db 0FDh	; ¤
		db    0
		db    0
		db 0BFh	; ┐
		db  7Dh	; }
		db 0E0h	; р
		db    0
		db    0
		db    0
		db    1
		db    0
		db    2
		db    0
		db    0
		db    0
		db    0
		db    2
		db    0
		db  20h
		db    0
		db    0
		db    0
		db    0
		db    0
		db    0
		db    1
		db 0FFh
		db 0FFh
		db 0FFh
		db 0FFh
		db 0FFh
		db    3
		db  81h	; Б
		db    0
		db    0
		db 0A0h	; а
		db  7Dh	; }
		db    0
		db    0
		db    0
		db    0
		db    1
		db    0
		db    1
		db    0
		db    0
		db    0
		db    0
		db    0
		db    8
		db    0
		db    0
		db    0
		db    0
		db    0
		db  40h	; @
		db    0
		db    1
		db 0FFh
		db 0FFh
		db 0FFh
		db 0FFh
		db 0FFh
		db    7
		db 0E1h	; с
		db    0
		db    0
		db 0BCh	; ╝
		db  41h	; A
		db    0
		db    0
		db    0
		db    0
		db    1
		db    0
		db    0
		db    0
		db  30h	; 0
		db    0
		db    0
		db    0
		db    8
		db    0
		db    0
		db    0
		db    6
		db    0
		db  63h	; c
		db  63h	; c
		db    1
		db 0FFh
		db 0FFh
		db 0FFh
		db 0FFh
		db 0FFh
		db    7
		db 0E1h	; с
		db    0
		db    0
		db 0B0h	; ░
		db    1
		db  7Eh	; ~
		db  7Eh	; ~
		db  7Eh	; ~
		db    0
		db  81h	; Б
		db    0
		db    0
		db    0
		db    0
		db    0
		db    0
		db    0
		db    0
		db    0
		db    0
		db    0
		db    0
		db    0
		db    0
		db    0
		db    1
		db 0FFh
		db 0FFh
		db 0FFh
		db 0FFh
		db 0FFh
		db    6
		db    1
		db    0
		db    0
		db    0
		db    0
		db    0
		db    0
		db    0
		db    0
		db    0
		db    0
		db    0
		db    0
		db    0
		db    0
		db    0
		db    0
		db    0
		db    0
		db    0
		db    0
		db    0
		db    0
		db    0
		db    0
		db    0
		db    0
		db    0
		db    0
		db    0
		db    0
		db    0
		db    0
		db    0
		db    0
		db 0A0h	; а
		db  41h	; A
		db 0FFh
		db 0FFh
		db 0FFh
		db 0FFh
		db 0FFh
		db  0Fh
		db 0F0h	; Ё
		db    0
		db    0
		db    0
		db    0
		db    0
		db    0
		db    0
		db    0
		db    0
		db    0
		db    0
		db    1
		db  81h	; Б
		db  81h	; Б
		db 0FFh
		db 0FFh
		db 0FFh
		db 0FFh
		db 0FFh
		db  40h	; @
		db  85h	; Е
		db    0
		db    0
		db 0A2h	; в
		db  41h	; A
		db 0FFh
		db 0FFh
		db 0FFh
		db 0FFh
		db 0FFh
		db  7Fh	; 
		db 0FEh	; ■
		db    0
		db    0
		db    0
		db    0
		db    0
		db    0
		db    0
		db    0
		db    0
		db    0
		db    0
		db  20h
		db  24h	; $
		db    5
		db    0
		db    0
		db  18h
		db    0
		db    1
		db  42h	; B
		db  81h	; Б
		db    0
		db    0
		db 0A0h	; а
		db  45h	; E
		db    0
		db    0
		db    0
		db    0
		db    1
		db    0
		db    0
		db    0
		db  80h	; А
		db    1
		db    1
		db    0
		db    0
		db    0
		db  40h	; @
		db  40h	; @
		db    0
		db  80h	; А
		db    4
		db    0
		db  21h	; !
		db    0
		db    0
		db  18h
		db    0
		db    1
		db  44h	; D
		db  81h	; Б
		db    0
		db    0
		db 0A0h	; а
		db  11h
		db 0E0h	; р
		db    0
		db    0
		db    0
		db    1
		db    0
		db    4
		db    0
		db    1
		db    0
		db    0
		db    1
		db    0
		db  40h	; @
		db    0
		db    0
		db  40h	; @
		db    0
		db  20h
		db    0
		db    1
		db 0FFh
		db 0FFh
		db 0FFh
		db 0FFh
		db 0FFh
		db    1
		db  81h	; Б
		db    0
		db    0
		db 0A0h	; а
		db  11h
		db    0
		db    0
		db    0
		db    0
		db    1
		db    0
		db    0
		db  81h	; Б
		db    0
		db    0
		db    0
		db    0
		db    4
		db    0
		db    0
		db    0
		db    0
		db  40h	; @
		db  80h	; А
		db    0
		db    1
		db 0FFh
		db 0FFh
		db 0FFh
		db 0FFh
		db 0FFh
		db    0
		db  61h	; a
		db    0
		db    0
		db 0A2h	; в
		db  41h	; A
		db    0
		db    0
		db    0
		db    0
		db    1
		db    0
		db    0
		db    0
		db  0Eh
		db    0
		db    0
		db    0
		db    4
		db    0
		db    0
		db    0
		db  38h	; 8
		db    0
		db  60h	; `
		db  60h	; `
		db    1
		db 0FFh
		db 0FFh
		db 0FFh
		db 0FFh
		db 0FFh
		db    0
		db  61h	; a
		db    0
		db    0
		db 0AFh	; п
		db 0F9h	; ∙
		db  42h	; B
		db  42h	; B
		db  42h	; B
		db    0
		db  81h	; Б
		db    0
		db    0
		db    0
		db    0
		db    0
		db    0
		db    0
		db    0
		db    0
		db    0
		db    0
		db    0
		db    0
		db    0
		db    0
		db    1
		db 0FFh
		db 0FFh
		db 0FFh
		db 0FFh
		db 0FFh
		db    6
		db  61h	; a
		db    0
		db    0
		db    0
		db    0
		db    0
		db    0
		db    0
		db    0
		db    0
		db 0
		db    0
		db    0
		db    0
		db    0
		db    0
		db    0
		db    0
		db    0
		db    0
		db    0
		db    0
		db    0
		db    0
		db    0
		db    0
		db    0
		db    0
		db    0
		db    0
		db    0
		db    0
		db    0
		db    0
		db    0
		db 0BFh	; ┐
		db  39h	; 9
		db 0FFh
		db 0FFh
		db 0FFh
		db 0FFh
		db 0FFh
		db  1Fh
		db 0F8h	; °
		db    0
		db    0
		db    0
		db    0
		db    0
		db    0
		db    0
		db    0
		db    0
		db    0
		db    0
		db    2
		db    0
		db  41h	; A
		db 0FFh
		db 0FFh
		db 0FFh
		db 0FFh
		db 0FFh
		db  3Ch	; <
		db 0F9h	; ∙
		db    0
		db    0
		db 0BEh	; ╛
		db  39h	; 9
		db 0FFh
		db 0FFh
		db 0FFh
		db 0FFh
		db 0FFh
		db  3Fh	; ?
		db 0FCh	; №
		db    0
		db    0
		db    0
		db    0
		db  3Fh	; ?
		db 0FFh
		db 0FEh	; ■
		db    0
		db    0
		db    0
		db    0
		db  20h
		db  24h	; $
		db    5
		db    0
		db    0
		db  18h
		db    0
		db    1
		db  7Ch	; |
		db  81h	; Б
		db    0
		db    0
		db 0BFh	; ┐
		db  45h	; E
		db    0
		db    0
		db    0
		db    0
		db    1
		db    0
		db    0
		db    4
		db    0
		db    0
		db    0
		db  80h	; А
		db    0
		db    0
		db  80h	; А
		db    0
		db    0
		db  20h
		db    2
		db    0
		db  41h	; A
		db    0
		db    0
		db  18h
		db    0
		db    1
		db  42h	; B
		db  81h	; Б
		db    0
		db    0
		db 0A0h	; а
		db  11h
		db 0E0h	; р
		db    0
		db    0
		db    0
		db    1
		db    0
		db    4
		db    0
		db    0
		db    0
		db    0
		db    0
		db  80h	; А
		db  80h	; А
		db    0
		db    0
		db    0
		db    0
		db  10h
		db    0
		db    1
		db 0FFh
		db 0FFh
		db 0FFh
		db 0FFh
		db 0FFh
		db    1
		db  81h	; Б
		db    0
		db    0
		db 0A0h	; а
		db  11h
		db    0
		db    0
		db    0
		db    0
		db    1
		db    0
		db    0
		db  40h	; @
		db    0
		db    0
		db    0
		db    0
		db    8
		db    0
		db    0
		db    0
		db    0
		db    1
		db    0
		db    0
		db    1
		db 0FFh
		db 0FFh
		db 0FFh
		db 0FFh
		db 0FFh
		db    7
		db 0E1h	; с
		db    0
		db    0
		db 0BEh	; ╛
		db  41h	; A
		db    0
		db    0
		db    0
		db    0
		db    1
		db    0
		db    0
		db    0
		db    1
		db  80h	; А
		db    0
		db    0
		db    8
		db    0
		db    0
		db    0
		db 0C0h	; └
		db    0
		db  64h	; d
		db  38h	; 8
		db    1
		db 0FFh
		db 0FFh
		db 0FFh
		db 0FFh
		db 0FFh
		db    7
		db 0E1h	; с
		db    0
		db    0
		db 0BFh	; ┐
		db 0FDh	; ¤
		db  42h	; B
		db  42h	; B
		db  42h	; B
		db    0
		db  81h	; Б
		db    0
		db    0
		db    0
		db    0
		db    0
		db    0
		db    0
		db    0
		db    0
		db    0
		db    0
		db    0
		db    0
		db    0
		db    0
		db    1
		db 0FFh
		db 0FFh
		db 0FFh
		db 0FFh
		db 0FFh
		db    6
		db  61h	; a
		db    0
		db    0
		db    0
		db    0
		db    0
		db    0
		db    0
		db    0
		db    0
		db    0
		db    0
		db    0
		db    0
		db    0
		db    0
		db    0
		db    0
		db    0
		db    0
		db    0
		db    0
		db    0
		db    0
		db    0
		db    0
		db    0
		db    0
		db    0
		db    0
		db    0
		db    0
		db    0
		db    0
		db    0
		db 0A0h	; а
		db    5
		db 0FFh
		db 0FFh
		db 0FFh
		db 0FFh
		db 0FFh
		db  3Fh	; ?
		db 0FCh	; №
		db    0
		db    0
		db    0
		db    0
		db    0
		db    0
		db    0
		db    0
		db    0
		db    0
		db    0
		db    4
		db    0
		db  21h	; !
		db 0FFh
		db 0FFh
		db 0FFh
		db 0FFh
		db 0FFh
		db    2
		db  81h	; Б
		db    0
		db    0
		db 0A2h	; в
		db    5
		db 0FFh
		db 0FFh
		db 0FFh
		db 0FFh
		db 0FFh
		db  3Fh	; ?
		db 0FCh	; №
		db    0
		db    0
		db    0
		db    7
		db  80h	; А
		db    0
		db    0
		db 0F0h	; Ё
		db    0
		db    0
		db    0
		db  20h
		db  3Ch	; <
		db    5
		db    0
		db    0
		db  18h
		db    0
		db    1
		db  44h	; D
		db  81h	; Б
		db    0
		db    0
		db 0A0h	; а
		db  45h	; E
		db    0
		db    0
		db    0
		db    0
		db    1
		db    0
		db    0
		db  12h
		db  49h	; I
		db  24h	; $
		db  92h	; Т
		db  40h	; @
		db    0
		db    1
		db  48h	; H
		db  92h	; Т
		db  44h	; D
		db  88h	; И
		db    1
		db  81h	; Б
		db  81h	; Б
		db    0
		db    0
		db  18h
		db    0
		db    1
		db  42h	; B
		db  81h	; Б
		db    0
		db    0
		db 0A0h	; а
		db  11h
		db 0E0h	; р
		db    0
		db    0
		db    0
		db    1
		db    0
		db    4
		db    0
		db    4
		db    0
		db    0
		db    0
		db  41h	; A
		db    0
		db    0
		db    0
		db  10h
		db    0
		db  10h
		db    0
		db    1
		db 0FFh
		db 0FFh
		db 0FFh
		db 0FFh
		db 0FFh
		db    1
		db  81h	; Б
		db    0
		db    0
		db 0A0h	; а
		db  11h
		db    0
		db    0
		db    0
		db    0
		db    1
		db    0
		db    0
		db  24h	; $
		db    0
		db    0
		db    0
		db    0
		db    4
		db    0
		db    0
		db    0
		db    0
		db  12h
		db    0
		db    0
		db    1
		db 0FFh
		db 0FFh
		db 0FFh
		db 0FFh
		db 0FFh
		db    6
		db    1
		db    0
		db    0
		db 0A2h	; в
		db  41h	; A
		db    0
		db    0
		db    0
		db    0
		db    1
		db    0
		db    0
		db    0
		db    0
		db  7Eh	; ~
		db    0
		db    0
		db    4
		db    0
		db    0
		db  3Fh	; ?
		db    0
		db    0
		db  7Ch	; |
		db  0Eh
		db    1
		db 0FFh
		db 0FFh
		db 0FFh
		db 0FFh
		db 0FFh
		db    0
		db  61h	; a
		db    0
		db    0
		db 0AFh	; п
		db 0F9h	; ∙
		db  42h	; B
		db  42h	; B
		db  42h	; B
		db    0
		db  81h	; Б
		db    0
		db    0
		db    0
		db    0
		db    0
		db    0
		db    0
		db    0
		db    0
		db    0
		db    0
		db    0
		db    0
		db    0
		db    0
		db    1
		db 0FFh
		db 0FFh
		db 0FFh
		db 0FFh
		db 0FFh
		db    7
		db 0E1h	; с
		db    0
		db    0
		db    0
		db    0
		db    0
		db    0
		db    0
		db    0
		db    0
		db    0
		db    0
		db    0
		db    0
		db    0
		db    0
		db    0
		db    0
		db    0
		db    0
		db    0
		db    0
		db    0
		db    0
		db    0
		db    0
		db    0
		db    0
		db    0
		db    0
		db    0
		db    0
		db    0
		db    0
		db    0
		db 0A0h	; а
		db  79h	; y
		db 0FFh
		db 0FFh
		db 0FFh
		db 0FFh
		db 0FFh
		db  3Fh	; ?
		db 0FCh	; №
		db    0
		db    0
		db    0
		db    0
		db    0
		db    0
		db    0
		db    0
		db    0
		db    0
		db    0
		db    8
		db    0
		db  11h
		db 0FFh
		db 0FFh
		db 0FFh
		db 0FFh
		db 0FFh
		db  7Ch	; |
		db  81h	; Б
		db    0
		db    0
		db 0A2h	; в
		db  79h	; y
		db 0FFh
		db 0FFh
		db 0FFh
		db 0FFh
		db 0FFh
		db  1Fh
		db 0F8h	; °
		db    0
		db    0
		db    0
		db 0F8h	; °
		db    0
		db    0
		db    0
		db  0Fh
		db  80h	; А
		db    0
		db    0
		db  20h
		db    0
		db    5
		db    0
		db    0
		db  18h
		db    0
		db    1
		db  42h	; B
		db 0FDh	; ¤
		db    0
		db    0
		db 0A0h	; а
		db  7Dh	; }
		db    0
		db    0
		db    0
		db    0
		db    1
		db    0
		db    0
		db  40h	; @
		db    0
		db    0
		db    0
		db  20h
		db    0
		db    2
		db    0
		db    0
		db    0
		db    2
		db    0
		db  7Eh	; ~
		db    1
		db    0
		db    0
		db  18h
		db    0
		db    1
		db  7Eh	; ~
		db 0FDh	; ¤
		db    0
		db    0
		db 0BFh	; ┐
		db  11h
		db 0E0h	; р
		db    0
		db    0
		db    0
		db    1
		db    0
		db    4
		db  10h
		db  20h
		db  10h
		db  20h
		db  40h	; @
		db 0A2h	; в
		db  81h	; Б
		db    2
		db    4
		db    2
		db    4
		db  10h
		db    0
		db    1
		db 0FFh
		db 0FFh
		db 0FFh
		db 0FFh
		db 0FFh
		db    7
		db 0E1h	; с
		db    0
		db    0
		db 0BEh	; ╛
		db  11h
		db    0
		db    0
		db    0
		db    0
		db    1
		db    0
		db    0
		db  18h
		db    0
		db    0
		db    0
		db    0
		db    8
		db    0
		db    0
		db    0
		db    0
		db  0Ch
		db    0
		db    0
		db    1
		db 0FFh
		db 0FFh
		db 0FFh
		db 0FFh
		db 0FFh
		db    7
		db 0E1h	; с
		db    0
		db    0
		db 0A2h	; в
		db  7Dh	; }
		db    0
		db    0
		db    0
		db    0
		db    1
		db    0
		db    0
		db    0
		db    0
		db    1
		db 0F0h	; Ё
		db    0
		db    8
		db    0
		db    7
		db 0C0h	; └
		db    0
		db    0
		db  60h	; `
		db    3
		db    1
		db 0FFh
		db 0FFh
		db 0FFh
		db 0FFh
		db 0FFh
		db    7
		db 0E1h	; с
		db    0
		db    0
		db 0B0h	; ░
		db    1
		db  7Eh	; ~
		db  7Eh	; ~
		db  7Eh	; ~
		db    0
		db  81h	; Б
		db    0
		db    0
		db    0
		db    0
		db    0
		db    0
		db    0
		db    0
		db    0
		db    0
		db    0
		db    0
		db    0
		db    0
		db    0
		db    1
		db 0FFh
		db 0FFh
		db 0FFh
		db 0FFh
		db 0FFh
		db    0
		db  61h	; a
		db    0
		db    0
		db    0
		db    0
		db    0
		db    0
		db    0
		db    0
		db    0
		db    0
		db    0
		db    0
		db    0
		db    0
		db    0
		db    0
		db    0
		db    0
		db    0
		db    0
		db    0
		db    0
		db    0
		db    0
		db    0
		db    0
		db    0
		db    0
		db    0
		db    0
		db    0
		db    0
		db    0
		db    0
		db  80h	; А
		db    1
		db    0
		db    0
		db    0
		db    0
		db    1
		db  7Fh	; 
		db 0FEh	; ■
		db    0
		db    0
		db    0
		db    0
		db    0
		db    0
		db    0
		db    0
		db    0
		db    0
		db    0
		db  10h
		db    0
		db    9
		db    0
		db    0
		db    0
		db    0
		db    1
		db    0
		db    1
		db    0
		db    0
		db  80h	; А
		db    1
		db    0
		db    0
		db    0
		db    0
		db    1
		db  0Fh
		db 0F0h	; Ё
		db    0
		db    0
		db  0Eh
		db  10h
		db    0
		db    0
		db    0
		db    4
		db  38h	; 8
		db    0
		db    0
		db  20h
		db    0
		db    5
		db    0
		db    0
		db    0
		db    0
		db    1
		db    0
		db    1
		db    0
		db    0
		db  80h	; А
		db    1
		db    0
		db    0
		db    0
		db    0
		db    1
		db    0
		db    0
		db    0
		db    0
		db  10h
		db    0
		db  10h
		db    0
		db    4
		db    0
		db    4
		db    0
		db    0
		db    0
		db    0
		db    1
		db    0
		db    0
		db    0
		db    0
		db    1
		db    0
		db    1
		db    0
		db    0
		db  80h	; А
		db    1
		db    0
		db    0
		db    0
		db    0
		db    1
		db    0
		db    4
		db    0
		db  10h
		db    0
		db    0
		db    0
		db  14h
		db    0
		db    0
		db    0
		db    4
		db    0
		db  10h
		db    0
		db    1
		db    0
		db    0
		db    0
		db    0
		db    1
		db    0
		db    1
		db    0
		db    0
		db  80h	; А
		db    1
		db    0
		db    0
		db    0
		db    0
		db    1
		db    0
		db    0
		db  0Eh
		db  44h	; D
		db  44h	; D
		db  44h	; D
		db  44h	; D
		db  45h	; E
		db  11h
		db  11h
		db  11h
		db  11h
		db  18h
		db    0
		db    0
		db    1
		db    0
		db    0
		db    0
		db    0
		db    1
		db    0
		db    1
		db    0
		db    0
		db  80h	; А
		db    1
		db    0
		db    0
		db    0
		db    0
		db    1
		db    0
		db    0
		db    0
		db    0
		db    0
		db  0Fh
		db 0E0h	; р
		db    8
		db    3
		db 0F8h	; °
		db    0
		db    0
		db    0
		db  63h	; c
		db  63h	; c
		db    1
		db    0
		db    0
		db    0
		db    0
		db    1
		db    0
		db    1
		db    0
		db    0
		db  80h	; А
		db    1
		db    0
		db    0
		db    0
		db    0
		db  81h	; Б
		db    0
		db    0
		db    0
		db    0
		db    0
		db    0
		db    0
		db    0
		db    0
		db    0
		db    0
		db    0
		db    0
		db    0
		db    0
		db    1
		db    0
		db    0
		db    0
		db    0
		db    1
		db    0
		db    1
		db    0
		db    0
		db 0FFh
		db 0FFh
		db 0FFh
		db 0FFh
		db 0FFh
		db 0FFh
		db 0FFh
		db 0FFh
		db 0FFh
		db 0FFh
		db 0FFh
		db 0FFh
		db 0FFh
		db 0FFh
		db 0FFh
		db 0FFh
		db 0FFh
		db 0FFh
		db 0FFh
		db 0FFh
		db 0FFh
		db 0FFh
		db 0FFh
		db 0FFh
		db 0FFh
		db 0FFh
		db 0FFh
		db 0FFh
		db 0FFh
		db 0FFh
		db    0
		db    0
		db 0FFh
		db 0FFh
		db 0FFh
		db 0FFh
		db 0FFh
		db 0FFh
		db 0FFh
		db  7Fh	; 
		db 0FEh	; ■
		db    0
		db    0
		db    0
		db    0
		db    0
		db    0
		db    0
		db    0
		db    0
		db    0
		db    0
		db  10h
		db    0
		db    9
		db 0FFh
		db 0FFh
		db 0FFh
		db 0FFh
		db 0FFh
		db 0FFh
		db 0FFh
		db    0
		db    0
		db 0FFh
		db 0FFh
		db 0FFh
		db 0FFh
		db 0FFh
		db 0FFh
		db 0FFh
		db    3
		db 0C0h	; └
		db    0
		db    0
		db 0E0h	; р
		db    8
		db    0
		db    0
		db    0
		db    8
		db    3
		db  80h	; А
		db    0
		db  10h
		db    0
		db    9
		db 0FFh
		db 0FFh
		db 0FFh
		db 0FFh
		db 0FFh
		db 0FFh
		db 0FFh
		db    0
		db    0
		db 0FFh
		db 0FFh
		db 0FFh
		db 0FFh
		db 0FFh
		db 0FFh
		db 0FFh
		db    0
		db    1
		db    0
		db    0
		db    0
		db    0
		db    8
		db    0
		db    8
		db    0
		db    0
		db    0
		db    1
		db    0
		db    0
		db    1
		db 0FFh
		db 0FFh
		db 0FFh
		db 0FFh
		db 0FFh
		db 0FFh
		db 0FFh
		db    0
		db    0
		db 0FFh
		db 0FFh
		db 0FFh
		db 0FFh
		db 0FFh
		db 0FFh
		db 0FFh
		db    0
		db    2
		db    0
		db    0
		db    0
		db    0
		db    0
		db    8
		db    0
		db    0
		db    0
		db    0
		db    0
		db  20h
		db    0
		db    1
		db 0FFh
		db 0FFh
		db 0FFh
		db 0FFh
		db 0FFh
		db 0FFh
		db 0FFh
		db    0
		db    0
		db 0FFh
		db 0FFh
		db 0FFh
		db 0FFh
		db 0FFh
		db 0FFh
		db 0FFh
		db    0
		db    0
		db    3
		db    0
		db    0
		db    0
		db    0
		db    8
		db    0
		db    0
		db    0
		db    0
		db  30h	; 0
		db    0
		db    0
		db    1
		db 0FFh
		db 0FFh
		db 0FFh
		db 0FFh
		db 0FFh
		db 0FFh
		db 0FFh
		db    0
		db    0
		db 0FFh
		db 0FFh
		db 0FFh
		db 0FFh
		db 0FFh
		db 0FFh
		db 0FFh
		db    0
		db    0
		db    0
		db    0
		db    0
		db    0
		db  1Fh
		db 0FFh
		db 0FCh	; №
		db    0
		db    0
		db    0
		db    0
		db  7Fh	; 
		db  3Eh	; >
		db    1
		db 0FFh
		db 0FFh
		db 0FFh
		db 0FFh
		db 0FFh
		db 0FFh
		db 0FFh
		db    0
		db    0
		db 0FFh
		db 0FFh
		db 0FFh
		db 0FFh
		db 0FFh
		db 0FFh
		db 0FFh
		db 0FFh
		db 0FFh
		db 0FFh
		db 0FFh
		db 0FFh
		db 0FFh
unk_C7EE:	db 0FFh			; DATA XREF: j_MoveAllObjects+1B1Fo
		db 0FFh
		db 0FFh
		db 0FFh
		db 0FFh
		db 0FFh
		db 0FFh
		db 0FFh
		db 0FFh
		db 0FFh
		db 0FFh
		db 0FFh
		db 0FFh
		db 0FFh
		db 0FFh
		db 0FFh
		db 0FFh
		db    0
byte_C800:	db 0			; CODE XREF: MenuSaveLoad2+7p
					; DATA XREF: sub_D91A+16o ...
		db  80h	; А
		db    0
		db    0
		db    0
		db    0
		db    0
		db    0
		db    0
		db    0
		db    0
		db    0
		db    0
		db 0C4h	; ─
		db    0
		db    6
		db    0
		db    0
		db    2
		db    0
		db    0
		db    0
		db    0
		db    0
		db    0
		db    0
		db    0
		db    0
		db    0
		db    0
		db    1
		db    0
		db    0
		db  80h	; А
		db    0
		db    0
		db    0
		db    0
		db    0
		db    0
		db    0
		db    0
		db    0
		db    0
		db    0
		db 0A0h	; а
		db  40h	; @
		db    0
		db    0
		db    6
		db    2
		db    0
		db    0
		db    0
		db    0
		db    0
		db    0
		db    0
		db    0
		db    0
		db    0
		db    0
		db    1
		db    0
		db    0
		db  80h	; А
		db    0
		db    0
		db    0
		db    0
		db    0
		db    0
		db    0
		db    0
		db    0
		db    0
		db    0
		db  58h	; X
		db    8
		db    0
		db    0
		db    0
		db    8
		db    0
		db    0
		db    0
		db    0
		db    0
		db    0
		db    0
		db    0
		db    0
		db    0
		db    0
		db    1
		db    0
		db    0
		db  80h	; А
		db    0
		db    0
		db    0
		db    0
		db    0
		db    0
		db    0
		db    0
		db    0
		db    0
		db    0
		db  2Ch	; ,
		db    0
		db  80h	; А
		db    0
		db    0
		db  40h	; @
		db    0
		db    0
		db    0
		db    0
		db    0
		db    0
		db    0
		db    0
		db    0
		db    0
		db    0
		db    1
		db    0
		db    0
		db  80h	; А
		db    0
		db    0
		db    0
		db    0
		db    0
		db    0
		db    0
		db    0
		db    0
		db    0
		db    0
		db  0Eh
		db    0
		db    8
		db    0
		db    1
		db    0
		db    0
		db    0
		db    0
		db    0
		db    0
		db    0
		db    0
unk_C89A:	db    0			; DATA XREF: DemoModeOno
		db    0
		db    0
		db    0
		db    1
		db    0
		db    0
		db  80h	; А
		db    0
		db    0
		db    0
		db    0
		db    0
		db    0
		db    0
		db    0
		db    0
		db    0
		db    0
		db    3
		db    0
		db    0
		db  80h	; А
		db    8
		db    0
		db    0
		db    0
		db    0
		db    0
		db    0
		db    0
		db    0
		db    0
		db    0
		db    0
		db    0
		db    1
		db    0
		db    0
		db  80h	; А
		db    0
		db    0
		db    0
		db    0
		db    0
		db    0
		db    0
		db    0
		db    0
		db    0
		db    0
		db    1
		db  80h	; А
		db    0
		db  10h
		db  20h
		db    0
		db    0
		db    0
		db    0
		db    0
		db    0
		db    0
		db    0
		db    0
		db    0
		db    0
		db    0
		db    1
		db    0
		db    0
		db  80h	; А
		db    0
		db    0
		db    0
		db    0
		db    0
		db    0
		db    0
		db    0
		db    0
		db    0
		db    0
		db    0
		db  7Eh	; ~
		db    0
		db    3
		db    0
		db    0
		db    0
		db    0
		db    0
		db    0
		db    0
		db    0
		db    0
		db    0
		db    0
		db    0
		db    0
		db    1
		db    0
unk_C900:	db    0			; DATA XREF: DemoModeOn+15o
		db  80h	; А
		db    0
		db    0
		db    0
		db    0
		db    0
		db    0
		db    0
		db    0
		db    0
		db    0
		db    0
		db 0C2h	; ┬
		db    0
		db    1
		db  80h	; А
		db    0
		db    2
		db    0
		db    0
		db    0
		db    0
		db    0
		db    0
		db    0
		db    0
		db    0
		db    0
		db    0
		db    1
		db    0
		db    0
		db  80h	; А
		db    0
		db    0
		db    0
		db    0
		db    0
		db    0
		db    0
		db    0
		db    0
		db    0
		db    0
		db 0B0h	; ░
		db  40h	; @
		db    0
		db    0
		db    1
		db  82h	; В
		db    0
		db    0
		db    0
		db    0
		db    0
		db    0
		db    0
		db    0
		db    0
		db    0
		db    0
		db    1
		db    0
		db    0
		db  80h	; А
		db    0
		db    0
		db    0
		db    0
		db    0
		db    0
		db    0
		db    0
		db    0
		db    0
		db    0
		db  58h	; X
		db    4
		db    0
		db    0
		db    0
		db    8
		db    0
		db    0
		db    0
		db    0
		db    0
		db    0
		db    0
		db    0
		db    0
		db    0
		db    0
		db    1
		db    0
		db    0
		db  80h	; А
		db    0
		db    0
		db    0
		db    0
		db    0
		db    0
		db    0
		db    0
		db    0
		db    0
		db    0
		db  1Ch
		db    0
		db  40h	; @
		db    0
		db    0
		db  40h	; @
		db    0
		db    0
		db    0
		db    0
		db    0
		db    0
		db    0
		db    0
		db    0
		db    0
		db    0
		db    1
		db    0
		db    0
		db  80h	; А
		db    0
		db    0
		db    0
		db    0
		db    0
		db    0
		db    0
		db    0
		db    0
		db    0
		db    0
		db  0Ah
		db    0
		db    8
		db    0
		db    2
		db    0
		db    0
		db    0
		db    0
		db    0
		db    0
		db    0
		db    0
		db    0
		db    0
		db    0
		db    0
		db    1
		db    0
		db    0
		db  80h	; А
		db    0
		db    0
		db    0
		db  7Ch	; |
		db    0
		db    0
		db    0
		db    0
		db    0
		db  3Ch	; <
		db    0
		db    3
		db    0
		db    0
		db  80h	; А
		db    8
		db  3Ch	; <
		db    0
		db    0
		db    0
		db    0
		db    0
		db    4
		db    0
		db    0
		db    0
		db    0
		db    0
		db    1
		db    0
		db    0
		db  80h	; А
		db    0
		db    0
		db    0
		db    0
		db    0
		db    0
		db    0
		db    0
		db    0
		db    0
		db    0
		db    1
		db  80h	; А
		db    0
		db    8
		db  40h	; @
		db    0
		db    0
		db    0
		db    0
		db    0
		db    0
		db    0
		db    0
		db    0
		db    0
		db    0
		db    0
		db    1
		db    0
		db    0
		db  80h	; А
		db    0
		db    0
		db    0
		db    0
		db    0
		db    0
		db    0
		db    0
		db    0
		db    0
		db    0
		db    0
		db    1
		db 0FCh	; №
		db    2
		db    0
		db    0
		db    0
		db    0
		db    0
		db    0
		db    0
		db    0
		db    0
		db    0
		db    0
		db    0
		db    0
		db    1
		db    0
		db    0
		db  80h	; А
		db    0
		db    0
		db    0
		db    0
		db    0
		db    0
		db    0
		db    0
		db    0
		db    0
		db    0
		db 0E2h	; т
		db    0
		db    0
		db  60h	; `
		db    0
		db    2
		db    0
		db    0
		db    0
		db    0
		db    0
		db    0
		db    0
		db    0
		db    0
		db    0
		db    0
		db    1
		db    0
		db    0
		db  80h	; А
		db    0
		db    0
		db    0
		db    0
		db    0
		db    0
		db    0
		db    0
		db    0
		db    0
		db    0
		db 0B0h	; ░
		db  20h
		db    0
		db    0
		db    0
		db  62h	; b
		db    0
		db    0
		db    0
		db    0
		db    0
		db    0
		db    0
		db    0
		db    0
		db    0
		db    0
		db    1
		db    0
		db    0
		db  80h	; А
		db    0
		db    0
		db    0
		db    0
		db    0
		db    0
		db    0
		db    0
		db    0
		db    0
		db    0
		db  48h	; H
		db    4
		db    0
		db    0
		db    0
		db  10h
		db    0
		db    0
		db    0
		db    0
		db    0
		db    0
		db    0
		db    0
		db    0
		db    0
		db    0
		db    1
		db    0
		db    0
		db  80h	; А
		db    0
		db    0
		db    0
		db    0
		db    0
		db    0
		db    0
		db    0
		db    0
		db    0
		db    0
		db  14h
		db    0
		db  40h	; @
		db    0
		db    0
		db  40h	; @
		db    0
		db    0
		db    0
		db    0
		db    0
		db    0
		db    0
		db    0
		db    0
		db    0
		db    0
		db    1
		db    0
		db    0
		db  80h	; А
		db    0
		db    0
		db    0
		db    0
		db    0
		db    0
		db    0
		db    0
		db    0
		db    0
		db    0
		db  0Ah
		db    0
		db    4
		db    0
		db    2
		db    0
		db    0
		db    0
		db    0
		db    0
		db    0
		db    0
		db    0
		db    0
		db    0
		db    0
		db    0
		db    1
		db    0
		db    0
		db  80h	; А
		db    0
		db    0
		db    0
		db  42h	; B
		db  1Ch
		db  38h	; 8
		db  38h	; 8
		db  38h	; 8
		db    0
		db  40h	; @
		db  78h	; x
		db  3Bh	; ;
		db  1Ch
		db  38h	; 8
		db  80h	; А
		db    8
		db  42h	; B
		db  38h	; 8
		db  68h	; h
		db  68h	; h
		db  38h	; 8
		db  78h	; x
		db    4
		db  38h	; 8
		db  1Ch
		db    0
		db    0
		db    0
		db    1
		db    0
		db    0
		db  80h	; А
		db    0
		db    0
		db    0
		db    0
		db    0
		db    0
		db    0
		db    0
		db    0
		db    0
		db    0
		db    0
		db 0C0h	; └
		db    0
		db    8
		db  40h	; @
		db    0
		db    0
		db    0
		db    0
		db    0
		db    0
		db    0
		db    0
		db    0
		db    0
		db    0
		db    0
		db    1
		db    0
		db    0
		db  80h	; А
		db    0
		db    0
		db    0
		db    0
		db    0
		db    0
		db    0
		db    0
		db    0
		db    0
		db    0
		db    0
		db    0
		db    3
		db 0F4h	; Ї
		db    0
		db    0
		db    0
		db    0
		db    0
		db    0
		db    0
		db    0
		db    0
		db    0
		db    0
		db    0
		db    0
		db    1
		db    0
		db    0
		db  80h	; А
		db    0
		db    0
		db    0
		db    0
		db    0
		db    0
		db    0
		db    0
		db    0
		db    0
		db    0
		db 0A2h	; в
		db    0
		db    0
		db  18h
		db    0
		db    2
		db    0
		db    0
		db    0
		db    0
		db    0
		db    0
		db    0
		db    0
		db    0
		db    0
		db    0
		db    1
		db    0
		db    0
		db  80h	; А
		db    0
		db    0
		db    0
		db    0
		db    0
		db    0
		db    0
		db    0
		db    0
		db    0
		db    0
		db  90h	; Р
		db  20h
		db    0
		db    0
		db    0
		db  14h
		db    0
		db    0
		db    0
		db    0
		db    0
		db    0
		db    0
		db    0
		db    0
		db    0
		db    0
		db    1
		db    0
		db    0
		db  80h	; А
		db    0
		db    0
		db    0
		db    0
		db    0
		db    0
		db    0
		db    0
		db    0
		db    0
		db    0
		db  48h	; H
		db    2
		db    0
		db    0
		db    0
		db  10h
		db    0
		db    0
		db    0
		db    0
		db    0
		db    0
		db    0
		db    0
		db    0
		db    0
		db    0
		db    1
		db    0
		db    0
		db  80h	; А
		db    0
		db    0
		db    0
		db    0
		db    0
		db    0
		db    0
		db    0
		db    0
		db    0
		db    0
		db  14h
		db    0
		db  20h
		db    0
		db    0
		db  80h	; А
		db    0
		db    0
		db    0
		db    0
		db    0
		db    0
		db    0
		db    0
		db    0
		db    0
		db    0
		db    1
		db    0
		db    0
		db  80h	; А
		db    0
		db    0
		db    0
		db    0
		db    0
		db    0
		db    0
		db    0
		db    0
		db    0
		db    0
		db    6
		db    0
		db    4
		db    0
		db    2
		db    0
		db    0
		db    0
		db    0
		db    0
		db    0
		db    0
		db    0
		db    0
		db    0
		db    0
		db    0
		db    1
		db    0
		db    0
		db  80h	; А
		db    0
		db    0
		db    0
		db  42h	; B
		db  20h
		db  44h	; D
		db  40h	; @
		db  40h	; @
		db    0
		db  3Ch	; <
		db  44h	; D
		db    7
		db  20h
		db  44h	; D
		db  40h	; @
		db  10h
		db  40h	; @
		db  44h	; D
		db  54h	; T
		db  54h	; T
		db    4
		db  44h	; D
		db  3Ch	; <
		db  44h	; D
		db  20h
		db    0
		db    0
		db    0
		db    1
		db    0
		db    0
		db  80h	; А
		db    0
		db    0
		db    0
		db    0
		db    0
		db    0
		db    0
		db    0
		db    0
		db    0
		db    0
		db    0
		db 0F8h	; °
		db    0
		db    4
		db  40h	; @
		db    0
		db    0
		db    0
		db    0
		db    0
		db    0
		db    0
		db    0
		db    0
		db    0
		db    0
		db    0
		db    1
		db    0
		db    0
		db  80h	; А
		db    0
		db    0
		db    0
		db    0
		db    0
		db    0
		db    0
		db    0
		db    0
		db    0
		db    0
		db    0
		db    0
		db    0
		db  0Ch
		db    0
		db    0
		db    0
		db    0
		db    0
		db    0
		db    0
		db    0
		db    0
		db    0
		db    0
		db    0
		db    0
		db    1
		db    0
		db    0
		db  80h	; А
		db    0
		db    0
		db    0
		db    0
		db    0
		db    0
		db    0
		db    0
		db    0
		db    0
		db    0
		db 0A1h	; б
		db    0
		db    0
		db    6
		db    0
		db    2
		db    0
		db    0
		db    0
		db    0
		db    0
		db    0
		db    0
		db    0
		db    0
		db    0
		db    0
		db    1
		db    0
		db    0
		db  80h	; А
		db    0
		db    0
		db    0
		db    0
		db    0
		db    0
		db    0
		db    0
		db    0
		db    0
		db    0
		db  90h	; Р
		db  10h
		db    0
		db    0
		db    0
		db  0Ch
		db    0
		db    0
		db    0
		db    0
		db    0
		db    0
		db    0
		db    0
		db    0
		db    0
		db    0
		db    1
		db    0
		db    0
		db  80h	; А
		db    0
		db    0
		db    0
		db    0
		db    0
		db    0
		db    0
		db    0
		db    0
		db    0
		db    0
		db  28h	; (
		db    2
		db    0
		db    0
		db    0
		db  10h
		db    0
		db    0
		db    0
		db    0
		db    0
		db    0
		db    0
		db    0
		db    0
		db    0
		db    0
		db    1
		db    0
		db    0
		db  80h	; А
		db    0
		db    0
		db    0
		db    0
		db    0
		db    0
		db    0
		db    0
		db    0
		db    0
		db    0
		db  14h
		db    0
		db  20h
		db    0
		db    0
		db  80h	; А
		db    0
		db    0
		db    0
		db    0
		db    0
		db    0
		db    0
		db    0
		db    0
		db    0
		db    0
		db    1
		db    0
		db    0
		db  80h	; А
		db    0
		db    0
		db    0
		db    0
		db    0
		db    0
		db    0
		db    0
		db    0
		db    0
		db    0
		db    6
		db    0
		db    2
		db    0
		db    2
		db    0
		db    0
		db    0
		db    0
		db    0
		db    0
		db    0
		db    0
		db    0
		db    0
		db    0
		db    0
		db    1
		db    0
		db    0
		db  80h	; А
		db    0
		db    0
		db    0
		db  7Ch	; |
		db  20h
		db  78h	; x
		db  38h	; 8
		db  38h	; 8
		db    0
		db    2
		db  44h	; D
		db  3Fh	; ?
		db 0A0h	; а
		db  78h	; x
		db  40h	; @
		db  10h
		db  40h	; @
		db  44h	; D
		db  54h	; T
		db  54h	; T
		db  3Ch	; <
		db  44h	; D
		db  44h	; D
		db  78h	; x
		db  20h
		db    0
		db    0
		db    0
		db    1
		db    0
		db    0
		db  80h	; А
		db    0
		db    0
		db    0
		db    0
		db    0
		db    0
		db    0
		db    0
		db    0
		db    0
		db    0
		db    0
		db 0C7h	; ╟
		db  80h	; А
		db    4
		db  80h	; А
		db    0
		db    0
		db    0
		db    0
		db    0
		db    0
		db    0
		db    0
		db    0
		db    0
		db    0
		db    0
		db    1
		db    0
		db    0
		db  80h	; А
		db    0
		db    0
		db    0
		db    0
		db    0
		db    0
		db    0
		db    0
		db    0
		db    0
		db    0
		db    0
		db    0
		db    0
		db    0
		db    0
		db    0
		db    0
		db    0
		db    0
		db    0
		db    0
		db    0
		db    0
		db    0
		db    0
		db    0
		db    0
		db    1
		db    0
		db    0
		db  80h	; А
		db    0
		db    0
		db    0
		db    0
		db    0
		db    0
		db    0
		db    0
		db    0
		db    0
		db    0
		db 0A1h	; б
		db    0
		db    0
		db    1
		db  80h	; А
		db    2
		db    0
		db    0
		db    0
		db    0
		db    0
		db    0
		db    0
		db    0
		db    0
		db    0
		db    0
		db    1
		db    0
		db    0
		db  80h	; А
		db    0
		db    0
		db    0
		db    0
		db    0
		db    0
		db    0
		db    0
		db    0
		db    0
		db    0
		db  90h	; Р
		db  10h
		db    0
		db    0
		db    0
		db    4
		db    0
		db    0
		db    0
		db    0
		db    0
		db    0
		db    0
		db    0
		db    0
		db    0
		db    0
		db    1
		db    0
		db    0
		db  80h	; А
		db    0
		db    0
		db    0
		db    0
		db    0
		db    0
		db    0
		db    0
		db    0
		db    0
		db    0
		db  28h	; (
		db    1
		db    0
		db    0
		db    0
		db  20h
		db    0
		db    0
		db    0
		db    0
		db    0
		db    0
		db    0
		db    0
		db    0
		db    0
		db    0
		db    1
		db    0
		db    0
		db  80h	; А
		db    0
		db    0
		db    0
		db    0
		db    0
		db    0
		db    0
		db    0
		db    0
		db    0
		db    0
		db  14h
		db    0
		db  20h
		db    0
		db    0
		db  80h	; А
		db    0
		db    0
		db    0
		db    0
		db    0
		db    0
		db    0
		db    0
		db    0
		db    0
		db    0
		db    1
		db    0
		db    0
		db  80h	; А
		db    0
		db    0
		db    0
		db    0
		db    0
		db    0
		db    0
		db    0
		db    0
		db    0
		db    0
		db    7
		db    0
		db    2
		db    0
		db    4
		db    0
		db    0
		db    0
		db    0
		db    0
		db    0
		db    0
		db    0
		db    0
		db    0
		db    0
		db    0
		db    1
		db    0
		db    0
		db  80h	; А
		db    0
		db    0
		db    0
		db  40h	; @
		db  20h
		db  40h	; @
		db    4
		db    4
		db    0
		db  42h	; B
		db  78h	; x
		db  45h	; E
		db 0A0h	; а
		db  40h	; @
		db  28h	; (
		db  10h
		db  42h	; B
		db  44h	; D
		db  54h	; T
		db  54h	; T
		db  44h	; D
		db  44h	; D
		db  44h	; D
		db  40h	; @
		db  20h
		db    0
		db    0
		db    0
		db    1
		db    0
		db    0
		db  80h	; А
		db    0
		db    0
		db    0
		db    0
		db    0
		db    0
		db    0
		db    0
		db    0
		db    0
		db    0
		db    0
		db 0C0h	; └
		db  7Ch	; |
		db    2
		db  80h	; А
		db    0
		db    0
		db    0
		db    0
		db    0
		db    0
		db    0
		db    0
		db    0
		db    0
		db    0
		db    0
		db    1
		db    0
		db    0
		db  80h	; А
		db    0
		db    0
		db    0
		db    0
		db    0
		db    0
		db    0
		db    0
		db    0
		db    0
		db    0
		db    0
		db    0
		db    0
		db    0
		db    0
		db    0
		db    0
		db    0
		db    0
		db    0
		db    0
		db    0
		db    0
		db    0
		db    0
		db    0
		db    0
		db    1
		db    0
		db    0
		db  80h	; А
		db    0
		db    0
		db    0
		db    0
		db    0
		db    0
		db    0
		db    0
		db    0
		db    0
		db    0
		db 0A0h	; а
		db  80h	; А
		db    0
		db    0
		db  60h	; `
		db    2
		db    0
		db    0
		db    0
		db    0
		db    0
		db    0
		db    0
		db    0
		db    0
		db    0
		db    0
		db    1
		db    0
		db    0
		db  80h	; А
		db    0
		db    0
		db    0
		db    0
		db    0
		db    0
		db    0
		db    0
		db    0
		db    0
		db    0
		db  50h	; P
		db    8
		db    0
		db    0
		db    0
		db    4
		db    0
		db    0
		db    0
		db    0
		db    0
		db    0
		db    0
		db    0
		db    0
		db    0
		db    0
		db    1
		db    0
		db    0
		db  80h	; А
		db    0
		db    0
		db    0
		db    0
		db    0
		db    0
		db    0
		db    0
		db    0
		db    0
		db    0
		db  28h	; (
		db    1
		db    0
		db    0
		db    0
		db  20h
		db    0
		db    0
		db    0
		db    0
		db    0
		db    0
		db    0
		db    0
		db    0
		db    0
		db    0
		db    1
		db    0
		db    0
		db  80h	; А
		db    0
		db    0
		db    0
		db    0
		db    0
		db    0
		db    0
		db    0
		db    0
		db    0
		db    0
		db  0Eh
		db    0
		db  10h
		db    0
		db    1
		db    0
		db    0
		db    0
		db    0
		db    0
		db    0
		db    0
		db    0
		db    0
		db    0
		db    0
		db    0
		db    1
		db    0
		db    0
		db  80h	; А
		db    0
		db    0
		db    0
		db    0
		db    0
		db    0
		db    0
		db    0
		db    0
		db    0
		db    0
		db    7
		db    0
		db    1
		db    0
		db    4
		db    0
		db    0
		db    0
		db    0
		db    0
		db    0
		db    0
		db    0
		db    0
		db    0
		db    0
		db    0
		db    1
		db    0
		db    0
		db  80h	; А
		db    0
		db    0
		db    0
		db  40h	; @
		db  20h
		db  3Ch	; <
		db  78h	; x
		db  78h	; x
		db    0
		db  3Ch	; <
		db  40h	; @
		db  3Dh	; =
		db  9Ch	; Ь
		db  3Ch	; <
		db  28h	; (
		db  20h
		db  3Ch	; <
		db  38h	; 8
		db  54h	; T
		db  54h	; T
		db  3Ch	; <
		db  44h	; D
		db  3Ch	; <
		db  3Ch	; <
		db  20h
		db    0
		db    0
		db    0
		db    1
		db    0
		db    0
		db  80h	; А
		db    0
		db    0
		db    0
		db    0
		db    0
		db    0
		db    0
		db    0
		db    0
		db    0
		db    0
		db    0
		db 0C0h	; └
		db    3
		db 0C2h	; ┬
		db  80h	; А
		db    0
		db    0
		db    0
		db    0
		db    0
		db    0
		db    0
		db    0
		db    0
		db    0
		db    0
		db    0
		db    1
		db    0
		db    0
		db  80h	; А
		db    0
		db    0
		db    0
		db    0
		db    0
		db    0
		db    0
		db    0
		db    0
		db    0
		db    0
		db    0
		db    0
		db    0
		db    0
		db    0
		db    0
		db    0
		db    0
		db    0
		db    0
		db    0
		db    0
		db    0
		db    0
		db    0
		db    0
		db    0
		db    1
		db    0
		db    0
		db  80h	; А
		db    0
		db    0
		db    0
		db    0
		db    0
		db    0
		db    0
		db    0
		db    0
		db    0
		db    0
		db 0A0h	; а
		db  80h	; А
		db    0
		db    0
		db  18h
		db    2
		db    0
		db    0
		db    0
		db    0
		db    0
		db    0
		db    0
		db    0
		db    0
		db    0
		db    0
		db    1
		db    0
		db    0
		db  80h	; А
		db    0
		db    0
		db    0
		db    0
		db    0
		db    0
		db    0
		db    0
		db    0
		db    0
		db    0
		db  50h	; P
		db    8
unk_CF2F:	db    0			; DATA XREF: RAM:B7E2o
		db    0
		db    0
		db    8
		db    0
		db    0
		db    0
		db    0
		db    0
		db    0
		db    0
		db    0
		db    0
		db    0
		db    0
		db    1
		db    0
		db    0
		db  80h	; А
		db    0
		db    0
		db    0
		db    0
		db    0
		db    0
		db    0
		db    0
		db    0
		db    0
		db    0
		db  2Ch	; ,
		db    0
		db  80h	; А
		db    0
		db    0
		db  20h
		db    0
		db    0
		db    0
		db    0
		db    0
		db    0
		db    0
		db    0
		db    0
		db    0
		db    0
		db    1
		db    0
		db    0
		db  80h	; А
		db    0
		db    0
		db    0
		db    0
		db    0
		db    0
		db    0
		db    0
		db    0
		db    0
		db    0
		db  0Eh
		db    0
		db  10h
		db    0
		db    1
		db    0
		db    0
		db    0
		db    0
		db    0
		db    0
		db    0
		db    0
		db    0
		db    0
		db    0
		db    0
		db    1
		db    0
		db    0
		db  80h	; А
		db    0
		db    0
		db    0
		db    0
		db    0
		db    0
		db    0
		db    0
		db    0
		db    0
		db    0
		db    7
		db    0
		db    1
		db    0
		db    4
		db    0
		db    0
		db    0
		db    0
		db    0
		db    0
		db    0
		db    0
		db    0
		db    0
		db    0
		db    0
		db    1
		db    0
		db    0
		db  80h	; А
		db    0
		db    0
		db    0
		db    0
		db    0
		db    0
		db    0
		db    0
		db    0
		db    0
		db  40h	; @
		db    1
		db  80h	; А
		db    0
		db  10h
		db  20h
		db    0
		db    0
		db    0
		db    0
		db    0
		db    0
		db    0
		db    0
		db    0
		db    0
		db    0
		db    0
		db    1
		db    0
		db    0
		db 0FFh
		db 0FFh
		db 0FFh
		db 0FFh
		db 0FFh
		db 0FFh
		db 0FFh
		db 0FFh
		db 0FFh
		db 0FFh
		db 0FFh
		db 0FFh
		db 0FFh
		db 0FFh
		db 0FFh
		db 0FFh
		db 0FFh
		db 0FFh
		db 0FFh
		db 0FFh
		db 0FFh
		db 0FFh
		db 0FFh
		db 0FFh
		db 0FFh
		db 0FFh
		db 0FFh
		db 0FFh
		db 0FFh
		db 0FFh
		db    0
		db 0EEh	; ю
		db  79h	; y
		db  6Fh	; o
		db 0E7h	; ч
		db 0FFh
		db  88h	; И
		db  2Ah	; *
		db 0DDh	; ▌
		db 0F6h	; Ў
		db  54h	; T
		db  58h	; X
		db  86h	; Ж
		db  0Bh
		db 0D9h	; ┘
		db  85h	; Е
		db  7Ch	; |
		db 0B6h	; ╢
		db  7Dh	; }
		db  78h	; x
		db 0AEh	; о
		db 0ECh	; ь
		db 0AEh	; о
		db  6Ch	; l
		db  78h	; x
		db  38h	; 8
		db  38h	; 8
		db  5Fh	; _
		db 0BDh	; ╜
		db  6Fh	; o
		db 0FAh	; ·
		db 0FDh	; ¤
		db  4Ch	; L
; ---------------------------------------------------------------------------



; === CloakingDeviceOn === CloakingDeviceOn:db 0			; DATA XREF: sub_70A9:SetAtrViewScrr
CloakingDeviceOn:db 0			; DATA XREF: sub_70A9:SetAtrViewScrr
					; Use_Y_Cloacking+19o ...

; =============== S U B	R O U T	I N E =======================================

; Attributes: thunk



; === ShipLaunchMisile === ShipLaunchMisile:db 7			; DATA XREF: ReactionObject+7Co
ShipLaunchMisile:db 7			; DATA XREF: ReactionObject+7Co
		db 78h


; === CashBonus === CashBonus:	db 0Ch			; DATA XREF: IncBattleRating2+1Co
CashBonus:	db 0Ch			; DATA XREF: IncBattleRating2+1Co
		db 0E2h


; === RatingRightOn === RatingRightOn:	db 6
RatingRightOn:	db 6
		db 65h


; === NoFuel === NoFuel:		db 0Ah			; DATA XREF: SetFuelZero+5o
NoFuel:		db 0Ah			; DATA XREF: SetFuelZero+5o
		db 1Fh


; === sub_D822 === sub_D822:
sub_D822:

; FUNCTION CHUNK AT D878 SIZE 00000003 BYTES
; FUNCTION CHUNK AT D8B0 SIZE 00000004 BYTES

		ld	a, 0C5h	; '┼'
		call	loc_E297
		call	sub_D32D
		ld	a, 0E2h	; 'т'
		jr	loc_D848
; ---------------------------------------------------------------------------
		ld	hl, PilotName	; "JAMESON   "
		jr	loc_D878
; ---------------------------------------------------------------------------
		ld	a, 0
		jr	loc_D8B0
; ---------------------------------------------------------------------------
		ld	a, 69h ; 'i'
		call	loc_E297
		call	PrintSymbol_SP
; ---------------------------------------------------------------------------
		db 20h
; ---------------------------------------------------------------------------
		ld	a, (Pilot_Fuel)
		call	sub_D2DE
		ld	a, 0C3h	; '├'

loc_D848:				; CODE XREF: sub_D822+Aj
		jp	PrnMsg_A_CRLF
; End of function sub_D822


; =============== S U B	R O U T	I N E =======================================




; === sub_D822 === sub_D822:
sub_D822:

; FUNCTION CHUNK AT D878 SIZE 00000003 BYTES
; FUNCTION CHUNK AT D8B0 SIZE 00000004 BYTES

		ld	a, 0C5h	; '┼'
		call	loc_E297
		call	sub_D32D
		ld	a, 0E2h	; 'т'
		jr	loc_D848
; ---------------------------------------------------------------------------
		ld	hl, PilotName	; "JAMESON   "
		jr	loc_D878
; ---------------------------------------------------------------------------
		ld	a, 0
		jr	loc_D8B0
; ---------------------------------------------------------------------------
		ld	a, 69h ; 'i'
		call	loc_E297
		call	PrintSymbol_SP
; ---------------------------------------------------------------------------
		db 20h
; ---------------------------------------------------------------------------
		ld	a, (Pilot_Fuel)
		call	sub_D2DE
		ld	a, 0C3h	; '├'

loc_D848:				; CODE XREF: sub_D822+Aj
		jp	PrnMsg_A_CRLF
; End of function sub_D822


; =============== S U B	R O U T	I N E =======================================




; === sub_D84B === sub_D84B:
sub_D84B:
		ld	a, 2
		ld	(byte_D0CE), a
		ld	a, (PilotNumGalaxy)
		inc	a
		jp	PrintNumber_A
; End of function sub_D84B


; =============== S U B	R O U T	I N E =======================================




; === sub_D84B === sub_D84B:
sub_D84B:
		ld	a, 2
		ld	(byte_D0CE), a
		ld	a, (PilotNumGalaxy)
		inc	a
		jp	PrintNumber_A
; End of function sub_D84B


; =============== S U B	R O U T	I N E =======================================




; === sub_D857 === sub_D857:
sub_D857:
		ld	hl, PilotGalaxyData0
		ld	de, DataOfPlanet
		ld	bc, 6
		ldir
		call	GeneratePlanetName
		ld	hl, TempName	; "LAVE"
		jr	loc_D878
; End of function sub_D857


; =============== S U B	R O U T	I N E =======================================




; === sub_D857 === sub_D857:
sub_D857:
		ld	hl, PilotGalaxyData0
		ld	de, DataOfPlanet
		ld	bc, 6
		ldir
		call	GeneratePlanetName
		ld	hl, TempName	; "LAVE"
		jr	loc_D878
; End of function sub_D857


; =============== S U B	R O U T	I N E =======================================




; === sub_D86A === sub_D86A:
sub_D86A:
		call	SetFlgPrn_xx1
		ld	hl, PlanetPresent
		jr	loc_D878
; End of function sub_D86A


; =============== S U B	R O U T	I N E =======================================




; === sub_D86A === sub_D86A:
sub_D86A:
		call	SetFlgPrn_xx1
		ld	hl, PlanetPresent
		jr	loc_D878
; End of function sub_D86A


; =============== S U B	R O U T	I N E =======================================




; === sub_D872 === sub_D872:
sub_D872:
		call	SetFlgPrn_xx1
		ld	hl, PlanetName
; End of function sub_D872

; START	OF FUNCTION CHUNK FOR sub_D822

loc_D878:				; CODE XREF: sub_D822+Fj sub_D857+11j	...
		jp	sub_E311
; END OF FUNCTION CHUNK	FOR sub_D822
; ---------------------------------------------------------------------------
		call	SetCrdX_SP
; ---------------------------------------------------------------------------
		db 4

; =============== S U B	R O U T	I N E =======================================




; === sub_D872 === sub_D872:
sub_D872:
		call	SetFlgPrn_xx1
		ld	hl, PlanetName
; End of function sub_D872

; START	OF FUNCTION CHUNK FOR sub_D822

loc_D878:				; CODE XREF: sub_D822+Fj sub_D857+11j	...
		jp	sub_E311
; END OF FUNCTION CHUNK	FOR sub_D822
; ---------------------------------------------------------------------------
		call	SetCrdX_SP
; ---------------------------------------------------------------------------
		db 4

; =============== S U B	R O U T	I N E =======================================




; === DemoModeOn === DemoModeOn:
DemoModeOn:
		ld	hl, unk_C89A
		di
		ld	a, 0C9h	; '╔'
		call	loc_D91C
		ld	hl, byte_C800
		push	hl
		ld	de, PilotName	; "JAMESON   "
		ld	bc, 66h	; 'f'
		ldir
		ld	hl, unk_C900
		call	sub_D91A
		ret
; End of function DemoModeOn

; ---------------------------------------------------------------------------
		db    0
		db    0
		db    0
		db    0
		db    0
		db    0
		db    0
		db    0
		db    0
		db    0
		db    0
		db    0
		db    0
		db    0
		db    0
		db    0
; ---------------------------------------------------------------------------

loc_D970:				; CODE XREF: RAM:D97Fj
		nop
		nop
		nop
		nop
		nop
		ret
; ---------------------------------------------------------------------------

loc_D976:				; CODE XREF: RAM:D988j
		ld	a, 3Ch ; '<'
		jr	nc, loc_D97C
		ld	a, 2Ch ; ','

loc_D97C:				; CODE XREF: RAM:D978j
		ld	hl, 2F4Dh
		jr	loc_D970
; ---------------------------------------------------------------------------
		add	hl, bc
		inc	l
		ld	a, d
		push	hl
		or	a
		rra
		ld	c, a
		jr	loc_D976
; ---------------------------------------------------------------------------
		ld	hl, 2A53h
		push	hl
; ---------------------------------------------------------------------------
		db 18h
; ---------------------------------------------------------------------------
		xor	b
		ld	de, 0C800h
		push	de
		ld	hl, 0BE00h

loc_D997:				; CODE XREF: RAM:D9A5j
		ld	a, (de)
		add	a, (hl)
		dec	l
		xor	(hl)
		dec	l
		sub	(hl)
		cpl
		xor	e
		xor	d
		ld	(de), a
		inc	de
		ld	a, d
		cp	0D0h ; '╨'
		jr	nz, loc_D997
		ret
; ---------------------------------------------------------------------------
		db    0
		db    0
		db    0
		db    0
		db    0
		db    0
		db    0
		db    0
		db    0
		db    0
		db    0
		db    0
		db    0
		db    0
		db    0
		db    0
		db    0
		db    0
		db    0
		db    0
		db    0
		db    0
		db    0
		db    0
		db    0
		db    0
		db    0
		db    0
		db    0
		db    0
		db    0
		db    0
		db    0
		db    0
		db    0
		db    0
		db    0
		db    0
		db    0
		db    0
		db    0
		db    0
		db    0
		db    0
		db    0
		db    0
		db    0
		db    0
		db    0
		db    0
		db    0
		db    0
		db    0
		db    0
		db    0
		db    0
		db    0
		db    0
		db    0
		db    0
		db    0
		db    0
		db    0
		db    0
		db    0
		db    0
		db    0
		db    0
		db    0
		db    0
		db    0
		db    0
		db    0
		db    0
		db    0
		db    0
		db    0
		db    0
		db    0
		db    0
		db    0
		db    0
		db    0
		db    0
		db    0
		db    0
		db    0
		db    0
		db    0
		db    0
		db    0
		db    0
		db    0
		db    0
		db    0
		db    0
		db    0
		db    0
		db    0
		db    0
		db    0
		db    0
		db    0
		db    0
		db    0
		db    0
		db    0
		db    0
		db    0
		db    0
		db    0
		db    0
		db    0
		db    0
		db    0
		db    0
		db    0
		db    0
		db    0
		db    0
		db    0

; =============== S U B	R O U T	I N E =======================================




; === LastBonusCash === LastBonusCash:	dw 0AFh			; DATA XREF: IncBattleRating2+15w
LastBonusCash:	dw 0AFh			; DATA XREF: IncBattleRating2+15w
					; sub_DAB4:loc_DB25r


; === TempName === TempName:	.ascii 'LAVE',0         ; DATA XREF: sub_D857+Eo sub_DF24+10o ...
TempName:	.ascii 'LAVE',0         ; DATA XREF: sub_D857+Eo sub_DF24+10o ...
		db 0
		db 0
		db 0
		db 0


; === MinDistance === MinDistance:	dw 0			; DATA XREF: sub_5FD5+3Dr
MinDistance:	dw 0			; DATA XREF: sub_5FD5+3Dr
					; CalcDistanceToCursor+16w ...

; =============== S U B	R O U T	I N E =======================================




; === sub_DFEE === sub_DFEE:
sub_DFEE:
		ld	a, (hl)
		xor	2
		ret
; End of function sub_DFEE


; =============== S U B	R O U T	I N E =======================================




; === sub_DFEE === sub_DFEE:
sub_DFEE:
		ld	a, (hl)
		xor	2
		ret
; End of function sub_DFEE


; =============== S U B	R O U T	I N E =======================================




; === DemoMode === DemoMode:	db 0E0h			; DATA XREF: LaunchDocking:loc_5E55o
DemoMode:	db 0E0h			; DATA XREF: LaunchDocking:loc_5E55o
					; sub_848A+3w ...
					; =#00 при стыковке со станцией	таргонов
					;      после использования escape pod
					; =#01 включен демо режим
					; =#E0 после прыжка
word_E07B:	dw 2			; DATA XREF: LaunchDocking-358Fr
					; LaunchDocking-358Bw
		db 0
		db 0
		db 0

; =============== S U B	R O U T	I N E =======================================




; === sub_E3AE === sub_E3AE:
sub_E3AE:
		ld	hl, byte_DED0
		call	sub_E311
; End of function sub_E3AE

; ---------------------------------------------------------------------------
		call	PrintSymbol_SP
		ex	af, af'
		call	PrintSymbol_SP
		pop	hl
		call	sub_E1E8
; ---------------------------------------------------------------------------
		db 0C3h	; ├
		db    5
		db 0E2h	; т
; ---------------------------------------------------------------------------

loc_E3C2:				; CODE XREF: RAM:E3ACj
		call	PrintSymbol_SP
		ex	af, af'
		jp	loc_E45E
; ---------------------------------------------------------------------------
byte_E3C9:	db 0			; DATA XREF: sub_E3CCw
word_E3CA:	dw 0			; DATA XREF: sub_E3CC+7w sub_E3CC+Ar

; =============== S U B	R O U T	I N E =======================================




; === sub_E3AE === sub_E3AE:
sub_E3AE:
		ld	hl, byte_DED0
		call	sub_E311
; End of function sub_E3AE

; ---------------------------------------------------------------------------
		call	PrintSymbol_SP
		ex	af, af'
		call	PrintSymbol_SP
		pop	hl
		call	sub_E1E8
; ---------------------------------------------------------------------------
		db 0C3h	; ├
		db    5
		db 0E2h	; т
; ---------------------------------------------------------------------------

loc_E3C2:				; CODE XREF: RAM:E3ACj
		call	PrintSymbol_SP
		ex	af, af'
		jp	loc_E45E
; ---------------------------------------------------------------------------
byte_E3C9:	db 0			; DATA XREF: sub_E3CCw
word_E3CA:	dw 0			; DATA XREF: sub_E3CC+7w sub_E3CC+Ar

; =============== S U B	R O U T	I N E =======================================




; === CursorWidth === CursorWidth:	db 10h			; DATA XREF: PrintSmallCursOnMapr
CursorWidth:	db 10h			; DATA XREF: PrintSmallCursOnMapr
					; PrintSmallCursOnMap:PrintCursorOnScrr ...
byte_E52C:	db 18h			; DATA XREF: PrintSmallCursOnMap+1DEr
					; Cmd_I_GlobalMap+5w ...


; === FlagsSetUp === FlagsSetUp:	db 8Ch			; DATA XREF: ScanKeyInPause+6o
FlagsSetUp:	db 8Ch			; DATA XREF: ScanKeyInPause+6o
					; sub_728D:loc_73FDo ...
byte_FC0F:	db 0			; DATA XREF: sub_728D+145o
					; ScanKeys-12Br ...
; ---------------------------------------------------------------------------
; START	OF FUNCTION CHUNK FOR ScanKeys

loc_FC10:				; CODE XREF: ScanKeys:loc_FD87j
		dec	d
		jp	nz, loc_FD76
		ld	a, (ControlKeyMove)
		and	0Fh
		ld	d, a
		ld	a, (FlagsSetUp)
		ld	e, a
		and	3
		jr	z, loc_FC63
		dec	a
		jr	loc_FC3D
; END OF FUNCTION CHUNK	FOR ScanKeys

; =============== S U B	R O U T	I N E =======================================




; === DeltaSpeed === DeltaSpeed:	db 0			; DATA XREF: UseEscapePod+25w
DeltaSpeed:	db 0			; DATA XREF: UseEscapePod+25w
					; sub_FE01+EAr	...
		db 0Ch
; ---------------------------------------------------------------------------

loc_FF0E:				; CODE XREF: sub_FE01+FBj
		jr	nc, loc_FF1D
		ld	hl, 0
		jp	loc_FF1D
; ---------------------------------------------------------------------------

loc_FF16:				; CODE XREF: sub_FE01+104j
		bit	5, h
		jr	z, loc_FF1D
		and	a
		sbc	hl, de

loc_FF1D:				; CODE XREF: sub_FE01+F6j
					; sub_FE01+101j ...
		ld	(DeltaSpeed), hl
		xor	a
		ld	(ControlKeyMove), a
		ret
; End of function sub_FE01


; =============== S U B	R O U T	I N E =======================================




