# Структуры данных ядра

## SlotObj (sizeof=0x27)
Из kladov.asm:
```
SlotObj		struc ;	(sizeof=0x27)
field_0:	db 3 dup(?)
field_3:	db 3 dup(?)
field_6:	db 3 dup(?)
field_9:	db 6 dup(?)
field_F:	db 6 dup(?)
field_15:	db 6 dup(?)
field_1B:	db ?			; speed
field_1C:	db ?			; seed up
field_1D:	db ?			; oTurnLR
field_1E:	db ?			; oTurnDU
field_1F:	db ?			; oRadiusBlast
field_20:	db ?			; oFlg_1
field_21:	db ?			; oFlg_2
field_22:	db ?			; oEnerg
field_23:	dw ?			; oAdrData (адрес данных корабля)
field_25:	db ?			; oFlg_3
field_26:	db ?			; oFlg_4
SlotObj		ends
```

Координаты/векторы хранятся в первых 18 байтах (по 6 байт на ось X/Y/Z в формате fixed point).

## ShipData (sizeof=0x17)
```
ShipData	struc ;	(sizeof=0x17)
field_0:	db ?			; dCargo
field_1:	db ?			; dCollision
field_2:	db ?			; dChanceFire
field_3:	db ?			; dVertexN
field_4:	db ?			; dEdgeN
field_5:	db ?			; dNormalN
field_6:	db ?			; dMaxSpeed
field_7:	db ?			; dEnergy
field_8:	dw ?			; dCash
field_A:	db ?			; dFlg_1
field_B:	db ?			; dSize
field_C:	dw ?			; dVertex
field_E:	dw ?			; dEdge
field_10:	dw ?			; dNormal
field_12:	db ?			; dFlg_2
field_13:	db ?			; dFactor
field_14:	db ?			; dLaser
field_15:	dw ?			; dRating
ShipData	ends
```
