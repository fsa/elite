; 17_save_load - разобрано из kladov.asm
; Количество подпрограмм: 3

; === MenuSaveLoad === CODE XREF: sub_728D+212p
MenuSaveLoad:				; CODE XREF: sub_728D+212p
		jp	MenuSaveLoad2
; End of function MenuSaveLoad

; ---------------------------------------------------------------------------


; === MenuSaveLoad2 === CODE XREF: MenuSaveLoadj
MenuSaveLoad2:				; CODE XREF: MenuSaveLoadj
		di
		ld	hl, DataCocpit
		call	sub_D91A
		call	byte_C800
		ei
		push	af
		call	j_FX_Beep
; End of function MenuSaveLoad2

; ---------------------------------------------------------------------------
		db 0F1h	; ё
		db 0C3h	; ├
		db    5
		db 0E2h	; т

; =============== S U B	R O U T	I N E =======================================




; === SaveCocpitInBuffer === CODE XREF: sub_728D+18Ep
SaveCocpitInBuffer:			; CODE XREF: sub_728D+18Ep
					; LaunchDocking+5Fp
		push	af
		call	CopyScaner2Scr
		pop	af
		ld	de, DataCocpit
		ld	hl, 5000h
		ld	bc, 800h
		ldir
		ld	de, 5B00h
		ld	hl, 5A00h
		ld	bc, 100h
		ldir
		ret
; End of function SaveCocpitInBuffer

; ---------------------------------------------------------------------------
; START	OF FUNCTION CHUNK FOR sub_815D

loc_819A:				; CODE XREF: sub_815D+14j
		cp	4
		jr	nc, loc_8173
		xor	a
		ld	(WhatOnScreen),	a
		ret
; END OF FUNCTION CHUNK	FOR sub_815D
; ---------------------------------------------------------------------------
		db    0


