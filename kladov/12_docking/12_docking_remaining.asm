; 12_docking - оставшиеся подпрограммы
; Количество: 5

; === AutoDocking === AutoDocking:	db 0			; DATA XREF: GoDocking+35w
AutoDocking:	db 0			; DATA XREF: GoDocking+35w
					; sub_9946+10w	...
; ---------------------------------------------------------------------------

loc_712A:				; CODE XREF: sub_E776+45p
		jp	loc_7130
; ---------------------------------------------------------------------------

loc_712D:				; CODE XREF: sub_D1E9+2Dp
		jp	loc_9974
; ---------------------------------------------------------------------------

loc_7130:				; CODE XREF: RAM:loc_712Aj
		ld	a, (hl)
		add	a, a
		ld	c, a
		add	a, a
		add	a, a
		add	a, c
		add	a, b
		ld	(hl), a
		ld	a, d
		jp	j_PrintMessage1_A

; =============== S U B	R O U T	I N E =======================================




; === BeginDocking === BeginDocking:
BeginDocking:
		ld	a, 38h ; '8'
		ld	(loc_932A+1), a
		ld	a, 0Ch
		ld	(HighDeltaSpeed), a
		xor	a
		ld	(HighAngleDU), a
		ld	(HighAngleLR), a
		ld	(TimeWorkECM), a
		inc	a
		ld	(BlockControl),	a
		ld	hl, PilotECMSystem
		res	7, (hl)
		call	ClearScrBuffer
		call	PrnDockingTonnel ; обратный отсчет до прыжка, круги
; ---------------------------------------------------------------------------
		ld	b, 2Fh ; '/'

loc_9438:				; CODE XREF: LaunchDocking+56j
		push	bc
		call	ClearScrBuffer
		call	PrnDockingTonnel ; обратный отсчет до прыжка, круги
; ---------------------------------------------------------------------------
		call	CopyBuf2Screen
		pop	bc
		djnz	loc_9438
		call	FX_Launch
		xor	a
		ld	(IndDanger), a
		call	SaveCocpitInBuffer
		call	InitSpeedAngle2
		call	sub_5FA1	; предложение покупки кобры МК-4
; ---------------------------------------------------------------------------
		call	sub_728D	; захват пиратами/таргонами при	стыковке
; ---------------------------------------------------------------------------
		ld	a, 7		; Cmd_L_PilotInfo
		jp	loc_5E55
; ---------------------------------------------------------------------------



; === ManualDocking === ManualDocking:	db 0			; DATA XREF: GoDockingr GoDocking+7w ...
ManualDocking:	db 0			; DATA XREF: GoDockingr GoDocking+7w ...


; === JumpDocking === JumpDocking:	db 0			; DATA XREF: GoDocking+28r
JumpDocking:	db 0			; DATA XREF: GoDocking+28r
					; ControlKeyIfMapOn+9r	...
					; =1 межпланетный прыжок/стыковка
					; =2 межгалактический прыжок


; === FlgJumpDocking === FlgJumpDocking:	db 0			; DATA XREF: sub_80D3r	sub_9296r ...
FlgJumpDocking:	db 0			; DATA XREF: sub_80D3r	sub_9296r ...
					; =1 запрет на обработку нажатий клавиш
					;   включено при: стыковке
					;   работает п/п отрисовки концентрический кругов
					;
					; 0,Flg_Interrupts


