; 11_combat_lasers - оставшиеся подпрограммы
; Количество: 16

; === objMissile === objMissile:	db 0, 0, 0		; field_0 ; DATA XREF: sub_A28Do
objMissile:	db 0, 0, 0		; field_0 ; DATA XREF: sub_A28Do
					; sub_A28D+24o	...
		db 0, 0, 80h		; field_3
		db 0, 2, 0		; field_6
		db 0, 7, 0, 78h, 0FFh, 2Ch; field_9
		db 0, 0F4h, 0FFh, 8Bh, 0, 33h; field_F
		db 0FFh, 0B3h, 0, 2Ah, 0, 0ECh;	field_15
		db 0			; field_1B
					; speed
		db 0			; field_1C
					; seed up
		db 0C0h			; field_1D
					; oTurnLR
		db 0C0h			; field_1E
					; oTurnDU
		db 1			; field_1F
					; oRadiusBlast
		db 80h			; field_20
					; oFlg_1
		db 20h			; field_21
					; oFlg_2
		db 0FAh			; field_22
					; oEnerg
		dw 62B2h		; field_23
					; oAdrData
		db 7Eh			; field_25
					; oFlg_3
		db 0			; field_26
					; oFlg_4


; === d_Missile === d_Missile:	ShipData <0, 28h, 0, 11h, 18h, 9, 2Ch, 2, 0, 0,	0Eh, 2A5h, 2E7h, 320h,\
d_Missile:	ShipData <0, 28h, 0, 11h, 18h, 9, 2Ch, 2, 0, 0,	0Eh, 2A5h, 2E7h, 320h,\
					; DATA XREF: RAM:6185o
			  0, 2,	0, 150h>


; === TempLaser === TempLaser:	db 0			; DATA XREF: sub_9946+Aw FireLaser+8r	...
TempLaser:	db 0			; DATA XREF: sub_9946+Aw FireLaser+8r	...
		db 0


; === DamageEnergyUnit === DamageEnergyUnit:db 0			; DATA XREF: Damage2Pilot+2Br
DamageEnergyUnit:db 0			; DATA XREF: Damage2Pilot+2Br
					; MoveAllObjects+43w ...


; === DamageFrontShield === DamageFrontShield:db 0			; DATA XREF: Damage2Pilot+18r
DamageFrontShield:db 0			; DATA XREF: Damage2Pilot+18r
					; MoveAllObjects+49w ...


; === DamageRearShield === DamageRearShield:db 0			; DATA XREF: Damage2Pilot+21r
DamageRearShield:db 0			; DATA XREF: Damage2Pilot+21r
					; MoveAllObjects+46w ...
; ---------------------------------------------------------------------------

loc_9CA5:				; CODE XREF: sub_71E4+Cj
		jp	sub_AFC9
; ---------------------------------------------------------------------------
		db 0C3h	; ├
		db  82h	; В
		db  9Fh	; Я
		db 0C3h	; ├
		db  8Dh	; Н
		db 0A2h	; в


; === nMissileObj === nMissileObj:	db 0			; DATA XREF: MoveAllObjects+36r
nMissileObj:	db 0			; DATA XREF: MoveAllObjects+36r
					; ReactionObject+119r ...


; === Missile1 === Missile1:	db 0			; DATA XREF: sub_9946+7w sub_99B5+31r	...
Missile1:	db 0			; DATA XREF: sub_9946+7w sub_99B5+31r	...

; =============== S U B	R O U T	I N E =======================================

; Attributes: thunk



; === cargoFireams === cargoFireams:	db 0			; DATA XREF: CallFunctionObj_1+35Ar
cargoFireams:	db 0			; DATA XREF: CallFunctionObj_1+35Ar
		db 0


; === PilotFrontLaser === PilotFrontLaser:db 1			; DATA XREF: RAM:72E9r	RAM:72F1w ...
PilotFrontLaser:db 1			; DATA XREF: RAM:72E9r	RAM:72F1w ...
		db 0


; === PilotLeftLaser === PilotLeftLaser:	db 0			; DATA XREF: RAM:72EEr	RAM:72F6w
PilotLeftLaser:	db 0			; DATA XREF: RAM:72EEr	RAM:72F6w
		db 0


; === PilotMissiles === PilotMissiles:	db 3			; DATA XREF: sub_5EC5+3o
PilotMissiles:	db 3			; DATA XREF: sub_5EC5+3o
					; PrintMissiles+Cr ...


; === MissileJammed === MissileJammed:	db 8			; DATA XREF: RAM:9C07o	sub_DAB4+17o ...
MissileJammed:	db 8			; DATA XREF: RAM:9C07o	sub_DAB4+17o ...
		db 0C9h


; === UntargetMissile === UntargetMissile:db 9			; DATA XREF: OffTargetMissiles+33o
UntargetMissile:db 9			; DATA XREF: OffTargetMissiles+33o
		db 0C8h


; === nObjMissile === nObjMissile:	db 0			; DATA XREF: UseMissile+Co
nObjMissile:	db 0			; DATA XREF: UseMissile+Co
					; UseMissile+45w ...
; ---------------------------------------------------------------------------
; START	OF FUNCTION CHUNK FOR CollisionWithObj



; === nMsgDamage === nMsgDamage:	db 0F5h			; DATA XREF: sub_D1E9:loc_D213w
nMsgDamage:	db 0F5h			; DATA XREF: sub_D1E9:loc_D213w
					; sub_DAB4:loc_DB13r

; =============== S U B	R O U T	I N E =======================================




