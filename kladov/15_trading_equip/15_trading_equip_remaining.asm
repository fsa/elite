; 15_trading_equip - оставшиеся подпрограммы
; Количество: 13

; === d_Cargo === d_Cargo:	ShipData <0, 14h, 0, 0Ah, 0Fh, 7, 0Fh, 11h, 0, 0, 0Ch, 34Dh, 374h, \
d_Cargo:	ShipData <0, 14h, 0, 0Ah, 0Fh, 7, 0Fh, 11h, 0, 0, 0Ch, 34Dh, 374h, \
					; DATA XREF: RAM:6191o
			  398h,	0, 2, 0, 1Fh>


; === TableBasePrice === TableBasePrice:	db 13h,	82h, 6,	1	; DATA XREF: GenerateAnotherPrice+7o
TableBasePrice:	db 13h,	82h, 6,	1	; DATA XREF: GenerateAnotherPrice+7o
		db 14h,	81h, 0Ah, 3	; +#28 Fireams
		db 41h,	83h, 2,	7
		db 28h,	85h, 0E2h, 1Fh
		db 53h,	85h, 0FBh, 0Fh
		db 0C4h, 8, 36h, 3
		db 0EBh, 1Dh, 8, 78h
		db 9Ah,	0Eh, 38h, 3
		db 75h,	6, 28h,	7
		db 4Eh,	1, 11h,	1Fh
		db 0, 0, 1Dh, 7
		db 0B0h, 89h, 0DCh, 3Fh
		db 20h,	81h, 35h, 3
		db 61h,	0A1h, 42h, 7
		db 0ABh, 0A2h, 37h, 1Fh
		db 2Dh,	0C1h, 0FAh, 0Fh
		db 35h,	0Fh, 0C0h, 7
unk_705F:	db    0			; DATA XREF: RAM:E98Ao	sub_EB2F+1o
byte_7060:	db 0			; DATA XREF: sub_9B83:loc_9B9Dw
					; PrnCountdownJump+48w	...
		db    0
		db    0
		db  2Ch	; ,
		db    1
		db    0
		db 0A0h	; а
		db  0Fh
		db    0
		db  70h	; p
		db  17h
		db    0
		db 0A0h	; а
		db  0Fh
		db    1
		db    0
		db    0
		db    0
		db  82h	; В
		db  14h
		db    1
		db    0
		db    0
		db    0
		db  28h	; (
		db  23h	; #
		db    1
		db  88h	; И
		db  13h
		db    1
		db  88h	; И
		db  13h
		db    5
		db    0
		db    0
		db    6
		db    0
		db    0
		db    0
		db  40h	; @
		db  1Fh

; =============== S U B	R O U T	I N E =======================================




; === PilotCargo === PilotCargo:	db 0			; DATA XREF: sub_E6D6+8o
PilotCargo:	db 0			; DATA XREF: sub_E6D6+8o
		db 0
		db 0


; === cargoSlaves === cargoSlaves:	db 0			; DATA XREF: CallFunctionObj_1:SetRndShipr
cargoSlaves:	db 0			; DATA XREF: CallFunctionObj_1:SetRndShipr
		db 0
		db 0


; === cargoNarc === cargoNarc:	db 0			; DATA XREF: CallFunctionObj_1+354r
cargoNarc:	db 0			; DATA XREF: CallFunctionObj_1+354r
		db 0
		db 0
		db 0


; === cargoMinerals === cargoMinerals:	db 0			; DATA XREF: InitSpeedAngle+7o
cargoMinerals:	db 0			; DATA XREF: InitSpeedAngle+7o
					; sub_7370+14r	...
		db 0
		db 0


; === cargoGemStones === cargoGemStones:	db 0			; DATA XREF: sub_E87E+46w
cargoGemStones:	db 0			; DATA XREF: sub_E87E+46w


; === cargoAlienItems === cargoAlienItems:db 0			; DATA XREF: sub_5EC5+2Er
cargoAlienItems:db 0			; DATA XREF: sub_5EC5+2Er


; === PilotFreeCargo === PilotFreeCargo:	db 14h			; DATA XREF: RAM:80CDw
PilotFreeCargo:	db 14h			; DATA XREF: RAM:80CDw
					; CollisionWithObj+5Cr	...


; === PilotLargeCargo === PilotLargeCargo:db 0			; DATA XREF: RAM:SetMaxTonnager
PilotLargeCargo:db 0			; DATA XREF: RAM:SetMaxTonnager
					; RAM:EA50o ...


; === DestDeviceCargo === DestDeviceCargo:db 2			; DATA XREF: sub_D1E9+30o
DestDeviceCargo:db 2			; DATA XREF: sub_D1E9+30o
		db 0FDh


; === CargoFound === CargoFound:	db 0Bh			; DATA XREF: ReCalcCrdObj_coriolis+73w
CargoFound:	db 0Bh			; DATA XREF: ReCalcCrdObj_coriolis+73w
		db 0


; === ProductPrice === ProductPrice:	dw  24h			; DATA XREF: GenerateAnotherPrice+3o
ProductPrice:	dw  24h			; DATA XREF: GenerateAnotherPrice+3o
		dw  3Ch
		dw 0C8h
		dw  3Ch
		dw 0E8h
		dw 3B0h
		dw 5F0h
		dw 380h
		dw 24Ch
		dw 14Ch
		dw    0
		dw 20Ch
		dw  6Ch
		dw 170h
		dw 284h
		dw 0A0h
		dw 200h
byte_E1DF:	db 0			; DATA XREF: sub_E212r	RAM:E273r ...
word_E1E0:	dw 0			; DATA XREF: sub_700Fr
					; UseEscapePod+49r ...
word_E1E2:	dw 0			; DATA XREF: sub_E212+13r sub_E212+18w ...
word_E1E4:	dw 0			; DATA XREF: sub_E212+2Cr sub_E212+31w ...
word_E1E6:	dw 0			; DATA XREF: sub_E364+3w sub_E38Dr

; =============== S U B	R O U T	I N E =======================================

; Attributes: noreturn



