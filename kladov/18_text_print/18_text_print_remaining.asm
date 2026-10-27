; 18_text_print - оставшиеся подпрограммы
; Количество: 6

; === TypeOfPrint === TypeOfPrint:	db 0			; DATA XREF: RAM:751Dw	PrnSunPlanet+1w ...
TypeOfPrint:	db 0			; DATA XREF: RAM:751Dw	PrnSunPlanet+1w ...

; =============== S U B	R O U T	I N E =======================================

; Attributes: thunk



; === FlgPrnNumber === FlgPrnNumber:	db 0
FlgPrnNumber:	db 0


; === DigitsInNumber === DigitsInNumber:	db 1			; DATA XREF: RAM:D2EDw	RAM:D30Er ...
DigitsInNumber:	db 1			; DATA XREF: RAM:D2EDw	RAM:D30Er ...
		db 0
byte_D0CE:	db 0			; DATA XREF: RAM:D325w
					; PrintNumber_HLDE+7w ...


; === msgTimeLife === msgTimeLife:	db 0			; DATA XREF: sub_DAB4+6o sub_DAB4+2Fw	...
msgTimeLife:	db 0			; DATA XREF: sub_DAB4+6o sub_DAB4+2Fw	...
byte_DA74:	db 42h			; DATA XREF: sub_DAB4+Do
					; цикл мигания сообщения в бою


; === msgNumber === msgNumber:	db 0			; DATA XREF: sub_D1E9r	sub_DAE6+5w ...
msgNumber:	db 0			; DATA XREF: sub_D1E9r	sub_DAE6+5w ...


; === Msg_B8 === Msg_B8:		db 0B0h	; ░
Msg_B8:		db 0B0h	; ░
		db  20h
		db 0B3h	; │
		db 0C2h	; ┬
		db  44h	; D
		db  45h	; E
		db  4Dh	; M
		db  4Fh	; O
		db 0EAh	; ъ
		db  4Ch	; L
		db  49h	; I
		db  5Ah	; Z
		db  49h	; I
		db 0FCh	; №
		db 0B1h	; ▒
		db  0Ch
		db 0B0h	; ░
		db 0D4h	; ╘
		db  20h
		db 0A7h	; з
		db  48h	; H
		db  41h	; A
		db  44h	; D
		db  20h
		db 0DEh	; ▐
		db  4Fh	; O
		db  50h	; P
		db  50h	; P
		db 0FCh	; №
		db  2Ch	; ,
		db  0Ch
		db  42h	; B
		db  55h	; U
		db  54h	; T
		db  20h
		db  49h	; I
		db  46h	; F
		db 0CAh	; ╩
		db 0DBh	; █
		db  92h	; Т
		db  45h	; E
		db 0FAh	; ·
		db  52h	; R
		db  20h
		db  3Fh	; ?
		db  2Eh	; .
		db  2Eh	; .
		db    0
		db    0
		db 0B0h	; ░
		db  20h
		db 0B3h	; │
		db 0C2h	; ┬
		db  50h	; P
		db  52h	; R
		db  4Fh	; O
		db  4Dh	; M
		db  4Fh	; O
		db  54h	; T
		db 0C4h	; ─
		db 0C9h	; ╔
		db  93h	; У
		db 0F8h	; °
		db  4Eh	; N
		db  4Bh	; K
		db  20h
		db  4Fh	; O
		db  46h	; F
		db  20h
		db  13h
		db    0
		db 0B1h	; ▒
		db  0Ch
		db    1
		db 0B3h	; │
		db  52h	; R
		db 0C5h	; ┼
		db  3Ah	; :
		db 0D3h	; ╙
		db 0C9h	; ╔
		db    0
		db  20h
		db  44h	; D
		db  45h	; E
		db  53h	; S
		db 0E9h	; щ
		db  4Eh	; N
		db  54h	; T
		db    0
		db  4Fh	; O
		db  43h	; C
		db 0C6h	; ╞
		db  50h	; P
		db  49h	; I
		db 0C4h	; ─
		db    0
		db    0
		db    0
		db 0D1h	; ╤
		db  41h	; A
		db 0E9h	; щ
		db  20h
		db 0DEh	; ▐
		db 0F5h	; ї
		db  49h	; I
		db 0DFh	; ▀
		db  20h
		db    0
		db 0F0h	; Ё
		db  48h	; H
		db  41h	; A
		db 0EAh	; ъ
		db  54h	; T
		db 0FFh
		db  54h	; T
		db    0
		db  20h
		db 0EEh	; ю
		db  45h	; E
		db  20h
		db    0
		db 0F0h	; Ё
		db  47h	; G
		db  20h
		db    0
		db 0FCh	; №
		db  20h
		db    0
		db  20h
		db  4Dh	; M
		db  98h	; Ш
		db  53h	; S
		db  49h	; I
		db 0DFh	; ▀
		db    0
		db  43h	; C
		db  55h	; U
		db    0
		db  42h	; B
		db 0F2h	; Є
		db  41h	; A
		db  4Bh	; K
		db    0
		db  4Eh	; N
		db  41h	; A
		db  4Dh	; M
		db  45h	; E
		db  3Fh	; ?
		db  20h
		db    0
		db  20h
		db  54h	; T
		db  4Fh	; O
		db  20h
		db    0
		db  20h
		db  98h	; Ш
		db  20h
		db    0
		db  20h
		db  53h	; S
		db  41h	; A
		db 0FAh	; ·
		db    0
		db  2Eh	; .
		db  0Ch
		db  20h
		db  13h
		db    0
		db  44h	; D
		db  4Fh	; O
		db 0D5h	; ╒
		db 0FCh	; №
		db    0
		db  20h
		db    1
		db  28h	; (
		db  59h	; Y
		db  2Fh	; /
		db  4Eh	; N
		db  29h	; )
		db  3Fh	; ?
		db    2
		db    0
		db    2
		db    6
		db  6Ch	; l
		db  0Ah
		db  20h
		db  4Ah	; J
		db  41h	; A
		db  4Dh	; M
		db  4Dh	; M
		db 0F4h	; Ї
		db    0
		db  20h
		db  41h	; A
		db  20h
		db    0
		db  53h	; S
		db  50h	; P
		db    0
		db  20h
		db  4Eh	; N
		db  45h	; E
		db  57h	; W
		db  20h
		db    0
		db  0Ch
		db  0Ch
		db    0
		db  20h
		db  13h
		db 0E2h	; т
		db 0EEh	; ю
		db  47h	; G
		db 0DFh	; ▀
		db    0
		db  43h	; C
		db  4Bh	; K
		db    0
		db 0B1h	; ▒
		db 0D3h	; ╙
		db  0Ah
		db  20h
		db    1
		db  47h	; G
		db  4Fh	; O
		db  4Fh	; O
		db  44h	; D
		db  20h
		db  4Ch	; L
		db  55h	; U
		db 0D5h	; ╒
		db    2
		db  2Ah	; *
		db    0
		db  50h	; P
		db  4Fh	; O
		db  50h	; P
		db  55h	; U
		db  4Ch	; L
		db 0F5h	; ї
		db  49h	; I
		db 0DFh	; ▀
		db    0
		db    0
		db  20h
		db  20h
		db    0
		db  40h	; @
		db  40h	; @
		db    0
		db    0
		db  40h	; @
		db  2Bh	; +
		db    0
		db    0
; ---------------------------------------------------------------------------
; START	OF FUNCTION CHUNK FOR sub_9D40

loc_F8B4:				; CODE XREF: sub_9D40+104j
		di
		ld	b, a
		pop	af
		push	af
		ld	(loc_A79C+1), a
		ld	a, 0C9h	; '╔'
		ld	(loc_FEFE), a
		ld	a, b
		ld	(loc_F8CB+1), sp
		ld	sp, 4020h
		call	PrnObjIfVisible

loc_F8CB:				; DATA XREF: sub_9D40+5B81w
		ld	sp, 4118h
		ld	a, 18h
		ld	(loc_FEFE), a
		ei
		jp	loc_9E47
; END OF FUNCTION CHUNK	FOR sub_9D40

; =============== S U B	R O U T	I N E =======================================




