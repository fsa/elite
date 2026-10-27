; 07_3d_projection - оставшиеся подпрограммы
; Количество: 66

; === HighAngleLR === HighAngleLR:	db 0			; DATA XREF: CalcAngSpeed4Prn+4w
HighAngleLR:	db 0			; DATA XREF: CalcAngSpeed4Prn+4w
					; sub_78B1:loc_78D7o ...


; === HighAngleDU === HighAngleDU:	db 0			; DATA XREF: CalcAngSpeed4Prn+1Bw
HighAngleDU:	db 0			; DATA XREF: CalcAngSpeed4Prn+1Bw
					; sub_7F81:loc_7F9Dr ...


; === SunCrd === SunCrd:		db 0F0h			; DATA XREF: ReCalcCrdObj_coriolis+46o
SunCrd:		db 0F0h			; DATA XREF: ReCalcCrdObj_coriolis+46o
					; NewJumpDockingCrdObjo ...
		db 52h


; === SunCrdXhi === SunCrdXhi:	db 5			; DATA XREF: sub_5FD5+5Bo
SunCrdXhi:	db 5			; DATA XREF: sub_5FD5+5Bo
		db 33h
		db 98h
		db 7
		db 0B6h
		db 0FAh


; === SunCrdZhi === SunCrdZhi:	db 8Bh			; DATA XREF: LaunchDocking+A0r
SunCrdZhi:	db 8Bh			; DATA XREF: LaunchDocking+A0r
					; LaunchDocking+A5w


; === PlanetCrd === PlanetCrd:	db 0			; DATA XREF: PrnSunPlanet:loc_75ADo
PlanetCrd:	db 0			; DATA XREF: PrnSunPlanet:loc_75ADo
					; CalcCrdSunPlanet+7o ...
		db 0


; === PlanetCrdX_hi === PlanetCrdX_hi:	db 0			; DATA XREF: sub_DD60+17r sub_DD60+5Fo
PlanetCrdX_hi:	db 0			; DATA XREF: sub_DD60+17r sub_DD60+5Fo
		db 0
		db 0
		db 0
		db 0CAh


; === PlanetCrdZ_med === PlanetCrdZ_med:	db 0BFh			; DATA XREF: sub_DD60+49o
PlanetCrdZ_med:	db 0BFh			; DATA XREF: sub_DD60+49o


; === PlanetCrdZ_hi === PlanetCrdZ_hi:	db 0			; DATA XREF: sub_5FD5+6r RAM:DD5Dw ...
PlanetCrdZ_hi:	db 0			; DATA XREF: sub_5FD5+6r RAM:DD5Dw ...

; =============== S U B	R O U T	I N E =======================================

; Attributes: thunk



; === CircleCrdXlow === CircleCrdXlow:	db 0			; DATA XREF: PrnSunPlanet+7o
CircleCrdXlow:	db 0			; DATA XREF: PrnSunPlanet+7o
					; PrnSunPlanet+2Co ...


; === CircleCrdXmed === CircleCrdXmed:	db 0			; DATA XREF: RAM:75FEr	RAM:76A9r ...
CircleCrdXmed:	db 0			; DATA XREF: RAM:75FEr	RAM:76A9r ...


; === CircleCrdXhi === CircleCrdXhi:	db 0			; DATA XREF: RAM:76C7r	RAM:76D1w ...
CircleCrdXhi:	db 0			; DATA XREF: RAM:76C7r	RAM:76D1w ...


; === CircleCrdYlow === CircleCrdYlow:	db 0			; DATA XREF: RAM:7627r	RAM:7701r
CircleCrdYlow:	db 0			; DATA XREF: RAM:7627r	RAM:7701r


; === CircleCrdYmed === CircleCrdYmed:	db 0			; DATA XREF: RAM:7620r	RAM:76FAr
CircleCrdYmed:	db 0			; DATA XREF: RAM:7620r	RAM:76FAr


; === CircleCrdYhi === CircleCrdYhi:	db 0
CircleCrdYhi:	db 0


; === CircleCrdZlow === CircleCrdZlow:	db 0CAh			; DATA XREF: RAM:7601r	RAM:7623r ...
CircleCrdZlow:	db 0CAh			; DATA XREF: RAM:7601r	RAM:7623r ...


; === CircleCrdZmed === CircleCrdZmed:	db 0BFh			; DATA XREF: RAM:75FAr	RAM:761Cr ...
CircleCrdZmed:	db 0BFh			; DATA XREF: RAM:75FAr	RAM:761Cr ...


; === CircleCrdZhi === CircleCrdZhi:	db 0			; DATA XREF: RAM:75EDr	RAM:75F7w ...
CircleCrdZhi:	db 0			; DATA XREF: RAM:75EDr	RAM:75F7w ...


; === CompasPointerCrdX === CompasPointerCrdX:db 0			; DATA XREF: PrintCompass+B9w
CompasPointerCrdX:db 0			; DATA XREF: PrintCompass+B9w
					; PrintCompass+E6w ...


; === CompasPointerCrdY === CompasPointerCrdY:db 0			; DATA XREF: PrintCompass:loc_829Cw
CompasPointerCrdY:db 0			; DATA XREF: PrintCompass:loc_829Cw
					; PrintCompass+200r


; === CircleCrdY === CircleCrdY:	dw 0			; DATA XREF: RAM:763Bw	RAM:7715w ...
CircleCrdY:	dw 0			; DATA XREF: RAM:763Bw	RAM:7715w ...


; === fsSunCrdXl === fsSunCrdXl:	db 0D1h			; DATA XREF: CalcCrdSunPlanet_2+3o
fsSunCrdXl:	db 0D1h			; DATA XREF: CalcCrdSunPlanet_2+3o
					; SetCrdSunPlInFreeSpace+7o ...
					; координаты "солнца" и "планеты" в межпланетном пространстве
		db 0CFh
		db 80h
		db 86h
		db 0F6h
		db 3
		db 0FDh
		db 12h


; === fsSunCrdZh === fsSunCrdZh:	db 0			; DATA XREF: sub_5FD5+1Er
fsSunCrdZh:	db 0			; DATA XREF: sub_5FD5+1Er


; === fsPlCrdXl === fsPlCrdXl:	db 0BAh			; DATA XREF: CalcCrdSunPlanet_2+10o
fsPlCrdXl:	db 0BAh			; DATA XREF: CalcCrdSunPlanet_2+10o
		db 5Eh
		db 4
		db 6Bh
		db 6Eh
		db 0
		db 0FCh
		db 2Bh


; === fsPlCrdZh === fsPlCrdZh:	db 80h			; DATA XREF: sub_5FD5+13r
fsPlCrdZh:	db 80h			; DATA XREF: sub_5FD5+13r
		db 3

; =============== S U B	R O U T	I N E =======================================




; === ScanerCrdXobj === ScanerCrdXobj:	db 0			; DATA XREF: sub_9736+Fw sub_9736+55r	...
ScanerCrdXobj:	db 0			; DATA XREF: sub_9736+Fw sub_9736+55r	...


; === ScanerCrdYobj === ScanerCrdYobj:	db 0			; DATA XREF: sub_9736:loc_9765w
ScanerCrdYobj:	db 0			; DATA XREF: sub_9736:loc_9765w
					; sub_9736:loc_97F9r


; === ScanerCrdZobj === ScanerCrdZobj:	db 93h			; DATA XREF: sub_9736:loc_9785w
ScanerCrdZobj:	db 93h			; DATA XREF: sub_9736:loc_9785w
					; sub_9736+A1r


; === AngleLR4Calc === AngleLR4Calc:	dw 8C94h		; DATA XREF: ReCalcCrdObj+29w
AngleLR4Calc:	dw 8C94h		; DATA XREF: ReCalcCrdObj+29w
					; ReCalcCrdObj:loc_9FDDw ...

; =============== S U B	R O U T	I N E =======================================




; === deltaCrdX === deltaCrdX:	dw 0CB6Bh		; DATA XREF: DirectObject+52w
deltaCrdX:	dw 0CB6Bh		; DATA XREF: DirectObject+52w
					; DirectObject+6Ew ...


; === deltaCrdY === deltaCrdY:	dw 0CAE2h		; DATA XREF: DirectObject+58w
deltaCrdY:	dw 0CAE2h		; DATA XREF: DirectObject+58w
					; DirectObject+74w ...


; === deltaCrdZ === deltaCrdZ:	dw 0C601h		; DATA XREF: DirectObject+5Ew
deltaCrdZ:	dw 0C601h		; DATA XREF: DirectObject+5Ew
					; DirectObject+7Aw ...


; === LenVector === LenVector:	dw 1585h		; DATA XREF: MoveObject+11w
LenVector:	dw 1585h		; DATA XREF: MoveObject+11w
					; MoveObject+8Br

; =============== S U B	R O U T	I N E =======================================




; === TablCrdPrnVertex === TablCrdPrnVertex:db 17h			; DATA XREF: RAM:ADCDo
TablCrdPrnVertex:db 17h			; DATA XREF: RAM:ADCDo
					; GetPrnCrdVertex+2o ...
		db  18h
		db    0
		db  13h
		db  13h
		db    2
		db  17h
		db  0Ah
		db    2
		db  10h
		db  14h
		db    1
		db  16h
		db  10h
		db    3
		db  16h


; === BufferCrdY === BufferCrdY:	db  0Dh			; DATA XREF: RAM:EC5Bo	sub_EDC5+3o
BufferCrdY:	db  0Dh			; DATA XREF: RAM:EC5Bo	sub_EDC5+3o
		db    1
		db    6
		db  3Ch	; <
		db    1
		db  0Eh
		db  34h	; 4
		db    3
		db    5
		db  5Bh	; [
		db    0
		db  0Eh
		db  39h	; 9
		db    3
		db    8
		db  39h	; 9
		db    1
		db    5
		db  1Ah
		db    3
		db  18h
		db    6
		db    2
		db  0Eh
		db    1
		db    3
		db    8
		db    7
		db    3
		db    1
		db  12h
		db    1
		db    2
		db  18h
		db    3
		db  0Ch
		db  0Fh
		db    3
		db  12h
		db  0Ah
		db    3
		db  1Bh
		db    0
		db    3
		db  15h
		db  0Dh
		db    0
		db  19h
		db  0Fh
		db    0
		db  29h	; )
		db  10h
		db    2
		db  27h	; '
		db  14h
		db    2
		db  2Bh	; +
		db  14h
		db    2
		db  14h
		db  28h	; (
		db    1
		db  11h
		db  23h	; #
		db    1
		db  0Fh
		db  27h	; '
		db    1
		db    3
		db    0
		db    2
		db    0
		db    0
		db    0
		db    0
		db    0
		db    0
		db    0
		db    0
		db    0
		db    0
		db    0
		db    0
		db    0
		db    0
		db    0
		db    0
		db    0
		db    0
		db    0
		db    0
		db    0
		db    0
		db    0
		db    0
		db    0
		db    0
		db    0
		db    0
		db    0
		db    0
		db    0
		db    0
		db    0
		db    0
		db    0
		db    0
		db    0
		db    0
		db    0
		db    0
		db    0
		db    0
		db    0
		db    0
		db    0
		db    0
		db    0
		db    0
		db    0
		db    0
		db    0


; === MatrixRotate === MatrixRotate:	db  7Eh	; ~		; DATA XREF: MakeTablPrnCrdVertex+11o
MatrixRotate:	db  7Eh	; ~		; DATA XREF: MakeTablPrnCrdVertex+11o
					; CalcMatrixRotateo ...
		db 0B3h	; │
		db  81h	; Б
		db 0F4h	; Ї
		db 0F3h	; є
		db    7
		db  81h	; Б
		db  2Ah	; *
		db  7Eh	; ~
		db  8Bh	; Л
		db 0F3h	; є
		db  78h	; x
		db 0F2h	; Є
		db 0ECh	; ь
		db 0F2h	; Є
		db  33h	; 3
		db 0F3h	; є
		db  2Ch	; ,
		db    0
		db  80h	; А
		db  16h
		db    0
		db    6
		db  80h	; А
		db    0
		db    0
		db    2
		db    0
		db  69h	; i
		db  80h	; А
		db    0
		db  80h	; А
		db    9
		db  80h	; А
		db  27h	; '
		db  80h	; А


; === PointTablCrdPrn === PointTablCrdPrn:db 1Eh			; DATA XREF: objPrnObj+1Fw
PointTablCrdPrn:db 1Eh			; DATA XREF: objPrnObj+1Fw
					; MakeTablPrnCrdVertex+16r ...


; === VertexCrdX === VertexCrdX:	dw 10h			; DATA XREF: MakeTablPrnCrdVertex+1B9w
VertexCrdX:	dw 10h			; DATA XREF: MakeTablPrnCrdVertex+1B9w
					; MakeTablPrnCrdVertex+1CAw ...


; === VertexCrdY === VertexCrdY:	dw 67h			; DATA XREF: MakeTablPrnCrdVertex+1BFw
VertexCrdY:	dw 67h			; DATA XREF: MakeTablPrnCrdVertex+1BFw
					; MakeTablPrnCrdVertex+1D0w ...


; === VertexCrdZ === VertexCrdZ:	dw 1CFh			; DATA XREF: MakeTablPrnCrdVertex:loc_AB9Fw
VertexCrdZ:	dw 1CFh			; DATA XREF: MakeTablPrnCrdVertex:loc_AB9Fw
					; MakeTablPrnCrdVertex+313w ...


; === CrdXProjection === CrdXProjection:	db 8			; DATA XREF: sub_7E02+76r
CrdXProjection:	db 8			; DATA XREF: sub_7E02+76r
					; PrnObjIfVisible+43w ...


; === CrdYProjection === CrdYProjection:	db 39h			; DATA XREF: PrnObjIfVisible+53w
CrdYProjection:	db 39h			; DATA XREF: PrnObjIfVisible+53w
					; PrnObjIfVisible+79w ...


; === CrdXYsign === CrdXYsign:	db 1			; DATA XREF: sub_7E02+61r
CrdXYsign:	db 1			; DATA XREF: sub_7E02+61r
					; PrnObjIfVisible+86w ...


; === VertexSignCrdX === VertexSignCrdX:	db 0			; DATA XREF: MakeTablPrnCrdVertex+338w
VertexSignCrdX:	db 0			; DATA XREF: MakeTablPrnCrdVertex+338w
					; RAM:AD74r ...


; === VertexSignCrdY === VertexSignCrdY:	db 80h			; DATA XREF: MakeTablPrnCrdVertex+357w
VertexSignCrdY:	db 80h			; DATA XREF: MakeTablPrnCrdVertex+357w
					; RAM:loc_ADB5r


; === VertexSignCrdZ === VertexSignCrdZ:	db 0			; DATA XREF: MakeTablPrnCrdVertex+376w
VertexSignCrdZ:	db 0			; DATA XREF: MakeTablPrnCrdVertex+376w
					; RAM:AD7Ar ...


; === CrdNVertex === CrdNVertex:	db 15h			; DATA XREF: PrintObject_2:loc_6F80w
CrdNVertex:	db 15h			; DATA XREF: PrintObject_2:loc_6F80w
					; PrintObject_2+5Fo ...
					; координата N текущей вершины

; =============== S U B	R O U T	I N E =======================================




; === VertexCrdXlow === VertexCrdXlow:	db 0			; DATA XREF: MakeTablPrnCrdVertex+1DAw
VertexCrdXlow:	db 0			; DATA XREF: MakeTablPrnCrdVertex+1DAw
					; MakeTablPrnCrdVertex+2FEw ...


; === VertexCrdYlow === VertexCrdYlow:	db 0			; DATA XREF: MakeTablPrnCrdVertex+1DDw
VertexCrdYlow:	db 0			; DATA XREF: MakeTablPrnCrdVertex+1DDw
					; MakeTablPrnCrdVertex+307w ...


; === VertexCrdZlow === VertexCrdZlow:	db 0FFh			; DATA XREF: MakeTablPrnCrdVertex+1E0w
VertexCrdZlow:	db 0FFh			; DATA XREF: MakeTablPrnCrdVertex+1E0w
					; MakeTablPrnCrdVertex+310w ...
; ---------------------------------------------------------------------------
; START	OF FUNCTION CHUNK FOR MakeTablPrnCrdVertex



; === AddNormalCrdX === AddNormalCrdX:	dw 8003h		; DATA XREF: CalcVisibleVerges+4Dw
AddNormalCrdX:	dw 8003h		; DATA XREF: CalcVisibleVerges+4Dw
					; CalcVisibleVerges+65o


; === AddNormalCrdY === AddNormalCrdY:	dw 8034h		; DATA XREF: CalcVisibleVerges+56w
AddNormalCrdY:	dw 8034h		; DATA XREF: CalcVisibleVerges+56w


; === AddNormalCrdZ === AddNormalCrdZ:	dw 0EDh			; DATA XREF: CalcVisibleVerges+5Fw
AddNormalCrdZ:	dw 0EDh			; DATA XREF: CalcVisibleVerges+5Fw


; === NormalCrdX === NormalCrdX:	db 83h			; DATA XREF: CalcVisibleVerges+4Aw
NormalCrdX:	db 83h			; DATA XREF: CalcVisibleVerges+4Aw
					; CalcVisibleVerges+69o


; === NormalCrdY === NormalCrdY:	db 0B4h			; DATA XREF: CalcVisibleVerges+53w
NormalCrdY:	db 0B4h			; DATA XREF: CalcVisibleVerges+53w


; === NormalCrdZ === NormalCrdZ:	db 93h			; DATA XREF: CalcVisibleVerges+5Cw
NormalCrdZ:	db 93h			; DATA XREF: CalcVisibleVerges+5Cw

; =============== S U B	R O U T	I N E =======================================




; === CrdXPrnSymbol === CrdXPrnSymbol:	dw 200h			; DATA XREF: sub_7430:loc_7438w
CrdXPrnSymbol:	dw 200h			; DATA XREF: sub_7430:loc_7438w
					; sub_7447+3w ...


; === PrnCrdXCursor === PrnCrdXCursor:	db 6Dh			; DATA XREF: RAM:8E2Br
PrnCrdXCursor:	db 6Dh			; DATA XREF: RAM:8E2Br
					; PrintSmallCursOnMap:loc_E496w ...


; === PrnCrdYCursor === PrnCrdYCursor:	db 7Eh			; DATA XREF: RAM:8E3Dr
PrnCrdYCursor:	db 7Eh			; DATA XREF: RAM:8E3Dr
					; PrintSmallCursOnMap+4Dw ...


; === Pilot_CrdYSystem === Pilot_CrdYSystem:db 0ADh		; DATA XREF: sub_5FD5+10o sub_5FD5+33r ...
Pilot_CrdYSystem:db 0ADh		; DATA XREF: sub_5FD5+10o sub_5FD5+33r ...
		db  9Ch	; Ь


; === Pilot_CrdXSystem === Pilot_CrdXSystem:db 14h			; DATA XREF: sub_5FD5:loc_6005r
Pilot_CrdXSystem:db 14h			; DATA XREF: sub_5FD5:loc_6005r
					; InitGame+Er ...
		db 1Dh
		db 15h


; === Pilot_CrdXTarget === Pilot_CrdXTarget:db 14h			; DATA XREF: sub_5FD5+37w InitGame+11w ...
Pilot_CrdXTarget:db 14h			; DATA XREF: sub_5FD5+37w InitGame+11w ...


; === Pilot_CrdYTarget === Pilot_CrdYTarget:db 0ADh		; DATA XREF: CalcDistanceToCursor+1o
Pilot_CrdYTarget:db 0ADh		; DATA XREF: CalcDistanceToCursor+1o
					; InitGame+17w	...


; === msgCrdY === msgCrdY:	db 0Ah			; DATA XREF: sub_DAE6+1w sub_DAB4+45r
msgCrdY:	db 0Ah			; DATA XREF: sub_DAE6+1w sub_DAB4+45r


; === CrdYDownGalaxyMap === CrdYDownGalaxyMap:db 0A7h		; DATA XREF: PrintSmallCursOnMap+1F2r
CrdYDownGalaxyMap:db 0A7h		; DATA XREF: PrintSmallCursOnMap+1F2r
					; Cmd_I_GlobalMap+Aw ...
; ---------------------------------------------------------------------------



; === AngleLeftRight === AngleLeftRight:	dw 0			; DATA XREF: sub_F9AB+1Ao sub_FE01+Er	...
AngleLeftRight:	dw 0			; DATA XREF: sub_F9AB+1Ao sub_FE01+Er	...
byte_FF0A:	db 0			; DATA XREF: sub_FE01+7Cr
					; sub_FE01:loc_FEE8w
		db 0


