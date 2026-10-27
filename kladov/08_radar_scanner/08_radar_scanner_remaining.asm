; 08_radar_scanner - оставшиеся подпрограммы
; Количество: 10

; === CompassIndFill === CompassIndFill:	db 18h,	3Ch, 3Ch, 18h	; DATA XREF: sub_6F29+3o
CompassIndFill:	db 18h,	3Ch, 3Ch, 18h	; DATA XREF: sub_6F29+3o


; === CompassIndEmpty === CompassIndEmpty:db 3Ch,	24h, 24h, 3Ch
CompassIndEmpty:db 3Ch,	24h, 24h, 3Ch


; === CompassInd_N === CompassInd_N:	db 9, 1Ah, 2Ch,	48h
CompassInd_N:	db 9, 1Ah, 2Ch,	48h


; === CompassInd_S === CompassInd_S:	db 0Eh,	10h, 8,	70h
CompassInd_S:	db 0Eh,	10h, 8,	70h


; === CompassInd_W === CompassInd_W:	db 41h,	82h, 0B4h, 58h
CompassInd_W:	db 41h,	82h, 0B4h, 58h


; === CompassInd_E === CompassInd_E:	db 0Eh,	1Ch, 20h, 1Ch
CompassInd_E:	db 0Eh,	1Ch, 20h, 1Ch

; =============== S U B	R O U T	I N E =======================================




; === CompasPointerType === CompasPointerType:db 1			; DATA XREF: PrintCompass:loc_82AAw
CompasPointerType:db 1			; DATA XREF: PrintCompass:loc_82AAw
					; PrintCompass+1F5r

; =============== S U B	R O U T	I N E =======================================

; Attributes: noreturn



; === DataCompass === DataCompass:	db 0, 0, 1		; DATA XREF: PrnCompasso
DataCompass:	db 0, 0, 1		; DATA XREF: PrnCompasso
		db 0, 7Eh, 1		; круг компаса
		db 1, 81h, 81h
		db 2, 0, 41h
		db 4, 0, 21h
		db 8, 0, 11h
		db 10h,	0, 9
		db 10h,	0, 9
		db 20h,	0, 5
		db 20h,	0, 5
		db 20h,	0, 5
		db 20h,	0, 5

; =============== S U B	R O U T	I N E =======================================




; === ScanerNumObj === ScanerNumObj:	db 1			; DATA XREF: sub_96DC+Aw
ScanerNumObj:	db 1			; DATA XREF: sub_96DC+Aw
					; sub_96DC:loc_982Er ...


; === FlgSaveScaner === FlgSaveScaner:	db 0			; DATA XREF: sub_7A84+16w sub_96DC+46w ...
FlgSaveScaner:	db 0			; DATA XREF: sub_7A84+16w sub_96DC+46w ...

; =============== S U B	R O U T	I N E =======================================




