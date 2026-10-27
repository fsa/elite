; 02_input_keyboard - разобрано из kladov.asm
; Количество подпрограмм: 2

; === ScanKeyInPause === CODE XREF: sub_728D+1F0p
ScanKeyInPause:				; CODE XREF: sub_728D+1F0p

; FUNCTION CHUNK AT 733F SIZE 00000027 BYTES

		ld	a, (ControlKey2)
		and	80h ; 'А'
		ret	z
		ld	hl, FlagsSetUp

loc_72AC:				; CODE XREF: ScanKeyInPause+Ej
					; DATA XREF: ScanKeyInPause:loc_7340o
		halt
		ld	a, (PressKeyPrnSymbol)
		inc	a
		jr	nz, loc_72AC

loc_72B3:				; CODE XREF: ScanKeyInPause+25j
		halt
		ld	hl, PressKeyPrnSymbol
		ld	a, (hl)
		inc	hl
		push	hl
		ld	hl, unk_72FB
		call	sub_72D5
		jr	c, loc_72CA
		call	sub_72D4
		jr	c, loc_72D0
		pop	hl
		jr	loc_72B3
; ---------------------------------------------------------------------------

loc_72CA:				; CODE XREF: ScanKeyInPause+1Dj
		ld	a, (hl)
		pop	hl
		xor	(hl)
		ld	(hl), a
		jr	loc_733F
; ---------------------------------------------------------------------------

loc_72D0:				; CODE XREF: ScanKeyInPause+22j
		ld	l, (hl)
		ex	(sp), hl
		ld	a, (hl)
		ret
; End of function ScanKeyInPause


; =============== S U B	R O U T	I N E =======================================




; === ScanKeys === CODE XREF: sub_FF25+10p
ScanKeys:				; CODE XREF: sub_FF25+10p

; FUNCTION CHUNK AT FC10 SIZE 00000015 BYTES
; FUNCTION CHUNK AT FC3D SIZE 0000007D BYTES

		xor	a
		ld	(byte_FC0F), a
		dec	a
		ld	(PressKeyPrnSymbol), a
		ld	bc, 7FFEh
		ld	d, 8

loc_FD76:				; CODE XREF: ScanKeys-158j
		rlc	b
		in	a, (c)
		and	1Fh
		xor	1Fh
		ld	e, 5

loc_FD80:				; CODE XREF: ScanKeys+1Cj
		rra
		call	c, sub_FCBA
		dec	e
		jr	nz, loc_FD80

loc_FD87:				; DATA XREF: RAM:EB03o
		jp	loc_FC10
; End of function ScanKeys

; ---------------------------------------------------------------------------
		db    5
		db    6
		db  0Dh
		db  0Eh
unk_FD8E:	db 0CAh	; ╩		; DATA XREF: RAM:EF92o
		db  71h	; q
		db  89h	; Й
		db  21h	; !
		db  21h	; !
		db  61h	; a
		db  31h	; 1
		db  20h
unk_FD96:	db  79h	; y		; DATA XREF: sub_E87E+847o
		db  21h	; !
		db  20h
unk_FD99:	db  40h	; @		; DATA XREF: sub_D470+127o
		db  40h	; @
unk_FD9B:	db  41h	; A		; DATA XREF: RAM:D893o
		db  42h	; B
		db  4Fh	; O
		db  55h	; U
		db  53h	; S
		db  45h	; E
		db  49h	; I
		db  54h	; T
		db  49h	; I
		db  4Ch	; L
		db  45h	; E
		db  54h	; T
		db  53h	; S
		db  54h	; T
		db  4Fh	; O
		db  4Eh	; N
		db  4Ch	; L
		db  4Fh	; O
		db  4Eh	; N
		db  55h	; U
		db  54h	; T
		db  48h	; H
		db  4Eh	; N
		db  4Fh	; O
byte_FDB3:	db 41h			; DATA XREF: PrintMessage1_A+179o
					; sub_E03D+9o
		db 4Ch
		db 4Ch
		db 45h
		db 58h
		db 45h
		db 47h
		db  45h	; E
		db  5Ah	; Z
		db  41h	; A
		db  43h	; C
		db  45h	; E
		db  42h	; B
		db  49h	; I
		db  53h	; S
		db  4Fh	; O
		db  55h	; U
		db  53h	; S
		db  45h	; E
		db  53h	; S
		db  41h	; A
		db  52h	; R
		db  4Dh	; M
		db  41h	; A
		db  49h	; I
		db  4Eh	; N
		db  44h	; D
		db  49h	; I
		db  52h	; R
		db  45h	; E
		db  41h	; A
		db  3Fh	; ?
		db  45h	; E
		db  52h	; R
		db  41h	; A
		db  54h	; T
		db  45h	; E
		db  4Eh	; N
		db  42h	; B
		db  45h	; E
		db  52h	; R
		db  41h	; A
		db  4Ch	; L
		db  41h	; A
		db  56h	; V
		db  45h	; E
		db  54h	; T
		db  49h	; I
		db  45h	; E
		db  44h	; D
		db  4Fh	; O
		db  52h	; R
		db  51h	; Q
		db  55h	; U
		db  41h	; A
		db  4Eh	; N
		db  54h	; T
		db  45h	; E
		db  49h	; I
		db  53h	; S
		db  52h	; R
		db  49h	; I
		db  4Fh	; O
		db  4Eh	; N
unk_FDF3:	db 0B1h	; ▒		; DATA XREF: RAM:EDE5o
		db  42h	; B
		db  3Ah	; :
		db  52h	; R
		db  52h	; R
		db  54h	; T
		db  92h	; Т
		db  70h	; p
unk_FDFB:	db 0D9h	; ┘		; DATA XREF: sub_EF48+16o
		db 0F1h	; ё
		db 0F0h	; Ё
		db 0FEh	; ■
		db 0FEh	; ■
		db 0FEh	; ■

; =============== S U B	R O U T	I N E =======================================




