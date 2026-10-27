; 06_objects_render - оставшиеся подпрограммы
; Количество: 13

; === objCoriolis === objCoriolis:	db 0			; DATA XREF: NewJumpDockingCrdObj+4o
objCoriolis:	db 0			; DATA XREF: NewJumpDockingCrdObj+4o
					; CalcCrdCoriolis2+1A9Do ...
byte_6049:	db 0			; DATA XREF: sub_DD60+1Ao
byte_604A:	db 0			; DATA XREF: sub_5FD5+65o sub_DD60+62r
		db 0
		db 0
		db 0
		db 76h
byte_604F:	db 0C0h			; DATA XREF: sub_DD60+10o sub_DD60+4Cr
byte_6050:	db 80h			; DATA XREF: sub_DD60+79r
byte_6051:	db 0			; DATA XREF: sub_DD60+40o
byte_6052:	db 0			; DATA XREF: sub_DD60+5Ao
		db 0
		db 0
		db 0
byte_6056:	db 7Fh			; DATA XREF: sub_DD60+73o
		db 0
		db 0F6h
		db 0
byte_605A:	db 0B3h			; DATA XREF: sub_DD60+52w
		db 0
		db 0
		db 0
byte_605E:	db 0B8h			; DATA XREF: sub_DD60:loc_DDD0o
		db 0
		db 72h
		db 0
byte_6062:	db 0			; DATA XREF: sub_DD60+57o
		db 0
		db 0
byte_6065:	db 80h			; DATA XREF: sub_728Do
					; sub_728D:loc_739Eo ...
		db 70h
		db 0
		db 0C0h
byte_6069:	db 0			; DATA XREF: GoDocking+2Do
					; Use_Y_Cloacking+28o ...
byte_606A:	db 0			; DATA XREF: sub_7A9E+Br sub_7A9E+68r	...
		dw d_Coriolis		; d_Coriolis
		db 7Fh
byte_606E:	db 0Ch			; DATA XREF: sub_DCE9+114o


; === SlotObj_01 === SlotObj_01:	db 1Ch,	0, 80h		; field_0
SlotObj_01:	db 1Ch,	0, 80h		; field_0
		db 1, 0, 80h		; field_3
		db 21h,	7, 0		; field_6
		db 0, 1, 0, 4, 0FFh, 7Fh; field_9
		db 0, 0FBh, 0FFh, 20h, 0, 80h; field_F
		db 0FFh, 1Fh, 0, 7Ah, 0, 84h; field_15
		db 1Fh			; field_1B
					; speed
		db 0			; field_1C
					; seed up
		db 40h			; field_1D
					; oTurnLR
		db 0			; field_1E
					; oTurnDU
		db 1			; field_1F
					; oRadiusBlast
		db 0			; field_20
					; oFlg_1
		db 0			; field_21
					; oFlg_2
		db 0			; field_22
					; oEnerg
		dw 6211h		; field_23
					; oAdrData
		db 7Eh			; field_25
					; oFlg_3
		db 0			; field_26
					; oFlg_4


; === SlotObj_02 === SlotObj_02:	db 47h,	0, 80h		; field_0
SlotObj_02:	db 47h,	0, 80h		; field_0
		db 1, 0, 0		; field_3
		db 31h,	6, 0		; field_6
		db 0, 86h, 0, 83h, 0FFh, 7Fh; field_9
		db 0, 0, 0FFh, 7Fh, 0, 3; field_F
		db 0FFh, 7Eh, 0, 80h, 0, 5; field_15
		db 1Bh			; field_1B
					; speed
		db 0			; field_1C
					; seed up
		db 20h			; field_1D
					; oTurnLR
		db 0			; field_1E
					; oTurnDU
		db 1			; field_1F
					; oRadiusBlast
		db 0			; field_20
					; oFlg_1
		db 0			; field_21
					; oFlg_2
		db 0			; field_22
					; oEnerg
		dw 6211h		; field_23
					; oAdrData
		db 7Eh			; field_25
					; oFlg_3
		db 0			; field_26
					; oFlg_4


; === SlotObj_03 === SlotObj_03:	db 1, 1, 1		; field_0
SlotObj_03:	db 1, 1, 1		; field_0
		db 1, 1, 1		; field_3
		db 1, 1, 1		; field_6
		db 1, 1, 1, 1, 1, 1	; field_9
		db 1, 1, 1, 1, 1, 1	; field_F
		db 1, 1, 1, 1, 1, 1	; field_15
		db 1			; field_1B
					; speed
		db 1			; field_1C
					; seed up
		db 1			; field_1D
					; oTurnLR
		db 1			; field_1E
					; oTurnDU
		db 1			; field_1F
					; oRadiusBlast
		db 1			; field_20
					; oFlg_1
		db 1			; field_21
					; oFlg_2
		db 0			; field_22
					; oEnerg
		dw 101h			; field_23
					; oAdrData
		db 1			; field_25
					; oFlg_3
		db 1			; field_26
					; oFlg_4


; === SlotObj_04 === SlotObj_04:	db 1, 1, 1		; field_0
SlotObj_04:	db 1, 1, 1		; field_0
		db 1, 1, 1		; field_3
		db 1, 1, 1		; field_6
		db 1, 1, 1, 1, 1, 1	; field_9
		db 1, 1, 1, 1, 1, 1	; field_F
		db 1, 1, 1, 1, 1, 1	; field_15
		db 1			; field_1B
					; speed
		db 1			; field_1C
					; seed up
		db 1			; field_1D
					; oTurnLR
		db 1			; field_1E
					; oTurnDU
		db 1			; field_1F
					; oRadiusBlast
		db 1			; field_20
					; oFlg_1
		db 1			; field_21
					; oFlg_2
		db 0			; field_22
					; oEnerg
		dw 101h			; field_23
					; oAdrData
		db 1			; field_25
					; oFlg_3
		db 1			; field_26
					; oFlg_4


; === SlotObj_05 === SlotObj_05:	db 1, 1, 1		; field_0 ; DATA XREF: MoveAllObjects+59o
SlotObj_05:	db 1, 1, 1		; field_0 ; DATA XREF: MoveAllObjects+59o
					; CallFunctionObj_1+298o
		db 1, 1, 1		; field_3
		db 1, 1, 1		; field_6
		db 1, 1, 1, 1, 1, 1	; field_9
		db 1, 1, 1, 1, 1, 1	; field_F
		db 1, 1, 1, 1, 1, 1	; field_15
		db 1			; field_1B
					; speed
		db 1			; field_1C
					; seed up
		db 1			; field_1D
					; oTurnLR
		db 1			; field_1E
					; oTurnDU
		db 1			; field_1F
					; oRadiusBlast
		db 1			; field_20
					; oFlg_1
		db 1			; field_21
					; oFlg_2
		db 0			; field_22
					; oEnerg
		dw 101h			; field_23
					; oAdrData
		db 1			; field_25
					; oFlg_3
		db 1			; field_26
					; oFlg_4


; === FlgObjNear === FlgObjNear:	db 10h			; DATA XREF: PrnExplodeObjr sub_96DCr	...
FlgObjNear:	db 10h			; DATA XREF: PrnExplodeObjr sub_96DCr	...


; === ObjInTarget === ObjInTarget:	db 5			; DATA XREF: sub_7D64+3r
ObjInTarget:	db 5			; DATA XREF: sub_7D64+3r
					; UseMissile+13r ...
		db 0


; === DistanceToObj === DistanceToObj:	dw 202h			; DATA XREF: PrintObject_2+8r
DistanceToObj:	dw 202h			; DATA XREF: PrintObject_2+8r
					; PrintObject_2+51r ...


; === PatternObject === PatternObject:	db 0, 32h, 0		; DATA XREF: SetObjInSlot+20o
PatternObject:	db 0, 32h, 0		; DATA XREF: SetObjInSlot+20o
		db 0, 32h, 0
		db 0, 32h, 0
		db 0, 0, 0, 0, 0FFh, 0FFh
		db 0, 0, 0FFh, 7Fh, 0, 0
		db 0FFh, 0FFh, 0, 0, 0,	0
		db 0
		db 0
		db 0
		db 0


; === AdrSlotObj === AdrSlotObj:	dw 6159h		; DATA XREF: CalcVisibleVerges+9w
AdrSlotObj:	dw 6159h		; DATA XREF: CalcVisibleVerges+9w
					; CalcVisibleVerges+43r


; === FlgPrnSym1 === FlgPrnSym1:	db 0			; DATA XREF: sub_D499+9r sub_D499+2Ar	...
FlgPrnSym1:	db 0			; DATA XREF: sub_D499+9r sub_D499+2Ar	...
byte_D0D2:	db 1			; DATA XREF: sub_D499+1Aw
					; sub_D499:loc_D4B8r ...
byte_D0D3:	db 0			; DATA XREF: sub_D499+10r sub_D499+17w ...
byte_D0D4:	db 0			; DATA XREF: sub_D499:loc_D4D8r
word_D0D5:	dw 0D508h		; DATA XREF: sub_7430+Br sub_7447+8r ...
; ---------------------------------------------------------------------------
; START	OF FUNCTION CHUNK FOR sub_FC33

loc_D0D7:				; CODE XREF: sub_FC33+5j
		jp	loc_D0F6
; END OF FUNCTION CHUNK	FOR sub_FC33

; =============== S U B	R O U T	I N E =======================================

; Attributes: thunk



; === PressKeyPrnSymbol === PressKeyPrnSymbol:db 0FFh		; DATA XREF: ScanKeyInPause+Ar
PressKeyPrnSymbol:db 0FFh		; DATA XREF: ScanKeyInPause+Ar
					; ScanKeyInPause+11o ...


