; 19_sound - разобрано из kladov.asm
; Количество подпрограмм: 1

; === j_FX_Beep === CODE XREF: MenuSaveLoad2+Cp
j_FX_Beep:				; CODE XREF: MenuSaveLoad2+Cp
					; sub_E7CF+8j ...
		call	sub_E1E8
; End of function j_FX_Beep

; ---------------------------------------------------------------------------
		call	Pause_24Int
		ld	a, 1
		scf
		ret

; =============== S U B	R O U T	I N E =======================================




