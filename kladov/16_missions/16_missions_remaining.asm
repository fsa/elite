; 16_missions - оставшиеся подпрограммы
; Количество: 27

; === d_Targon === d_Targon:	ShipData <0, 63h, 0FFh,	14h, 1Ah, 0Ah, 27h, 0F0h, 1F4h,	37h, 37h, \
d_Targon:	ShipData <0, 63h, 0FFh,	14h, 1Ah, 0Ah, 27h, 0F0h, 1F4h,	37h, 37h, \
					; DATA XREF: RAM:61BDo
			  6FAh,	74Bh, 787h, 0Ch, 2, 0Fh, 890h>


; === vTargon6 === vTargon6:	db 1Fh,	24h, 66h
vTargon6:	db 1Fh,	24h, 66h
		db 3Eh,	35h, 66h
		db 1Fh,	2, 66h
		db 3Eh,	3, 0FFh
		db 5Eh,	14h, 66h
		db 7Eh,	15h, 66h
		db 9Fh,	24h, 0FFh
		db 0BEh, 35h, 77h
		db 9Fh,	2, 77h
		db 0BEh, 3, 77h
		db 0DEh, 14h, 77h
		db 0FEh, 15h, 77h
		db 9Dh,	77h, 77h
		db 0BCh, 77h, 77h
		db 20h,	0, 50h
		db 20h,	45h, 28h
		db 18h,	0, 0A0h
		db 18h,	8Bh, 50h
		db 18h,	0, 40h


; === nTargon6 === nTargon6:	db  1Fh
nTargon6:	db  1Fh
		db  5Eh	; ^
		db  1Fh
		db  3Eh	; >
		db  5Eh	; ^
		db  7Eh	; ~
		db  1Fh
		db  9Eh	; Ю
		db  4Bh	; K
		db  2Bh	; +
		db    0
		db  4Bh	; K
		db  16h
		db  25h	; %
		db  20h
		db    0
		db    0
		db  18h
		db    0
		db    0


; === eTargon6 === eTargon6:	dw unk_6C72
eTargon6:	dw unk_6C72
		dw unk_6C75
		db  26h	; &
		db  46h	; F
		db  24h	; $
		db  36h	; 6
		db  56h	; V
		db  35h	; 5
		db    6
		db    2
		db    3
		db  16h
		db  14h
		db  15h
		db  27h	; '
		db  47h	; G
		db  37h	; 7
		db  57h	; W
		db    7
		db  17h
		db  77h	; w
unk_6C72:	db  12h			; DATA XREF: RAM:eTargon6o
		db  3Fh	; ?
		db  3Dh	; =
unk_6C75:	db    2			; DATA XREF: RAM:6C5Do
		db    4
		db    6
		db  13h
		db  15h
		db  17h
		db  23h	; #
		db  28h	; (
		db  39h	; 9
		db  45h	; E
		db  4Ah	; J
		db  5Bh	; [
		db  68h	; h
		db  6Ah	; j
		db  79h	; y
		db  7Bh	; {
		db  89h	; Й
		db 0ABh	; л
		db 0CDh	; ═


; === Mission2 === Mission2:	db 0			; DATA XREF: DestroyObj+6r
Mission2:	db 0			; DATA XREF: DestroyObj+6r
					; DestroyObj+12w ...


; === PilotName === PilotName:	.ascii 'JAMESON   ',0   ; DATA XREF: InitGame+6o sub_D822+Co ...
PilotName:	.ascii 'JAMESON   ',0   ; DATA XREF: InitGame+6o sub_D822+Co ...


; === PilotLegalStatus === PilotLegalStatus:db 0			; DATA XREF: ReCalcCrdObj_coriolis+24r
PilotLegalStatus:db 0			; DATA XREF: ReCalcCrdObj_coriolis+24r
					; UseEscapePod+29o ...


; === PilotRating0 === PilotRating0:	db 0			; DATA XREF: DirectObject+6r
PilotRating0:	db 0			; DATA XREF: DirectObject+6r
					; IncBattleRating2o


; === PilotRating1 === PilotRating1:	db 0			; DATA XREF: sub_D60B+6r
PilotRating1:	db 0			; DATA XREF: sub_D60B+6r


; === PilotRating2 === PilotRating2:	db 0			; DATA XREF: sub_5E78:loc_5E9Cr
PilotRating2:	db 0			; DATA XREF: sub_5E78:loc_5E9Cr
					; sub_5E78+37w	...
					; >=#84	в трюме	десант
					; 7,=1 война была или идёт
					; 6,=1 война закончилать
					; 3-1 -	ранг (=текущая стадия войны)


; === PilotCash2low === PilotCash2low:	db 0			; DATA XREF: sub_D32Dr
PilotCash2low:	db 0			; DATA XREF: sub_D32Dr
					; IncCASH:loc_D3E7r ...


; === PilotCash2hi === PilotCash2hi:	db 0			; DATA XREF: sub_5FA1+6r
PilotCash2hi:	db 0			; DATA XREF: sub_5FA1+6r
					; sub_D1E9:loc_D21Fo


; === PilotCash1 === PilotCash1:	dw 3E8h			; DATA XREF: sub_D32D+3r IncCASHr ...
PilotCash1:	dw 3E8h			; DATA XREF: sub_D32D+3r IncCASHr ...


; === PilotRnd === PilotRnd:	db 0			; DATA XREF: sub_9B83+9r
PilotRnd:	db 0			; DATA XREF: sub_9B83+9r
					; GenerateProductPrice+3w ...


; === PilotMission1 === PilotMission1:	db 0			; DATA XREF: sub_5E78+9r
PilotMission1:	db 0			; DATA XREF: sub_5E78+9r
					; FastSetAtrInViewScrr	...


; === Pilot_Fuel === Pilot_Fuel:	db 46h			; DATA XREF: sub_9B83o	sub_D822+1Er ...
Pilot_Fuel:	db 46h			; DATA XREF: sub_9B83o	sub_D822+1Er ...


; === PilotAlienComputer === PilotAlienComputer:db 0			; DATA XREF: sub_5EC5+28r sub_8AA3+7o	...
PilotAlienComputer:db 0			; DATA XREF: sub_5EC5+28r sub_8AA3+7o	...
					; 0,=1 есть Alien Computer


; === PilotMineralLife === PilotMineralLife:db 0			; DATA XREF: sub_6FFB+5w sub_7370r ...
PilotMineralLife:db 0			; DATA XREF: sub_6FFB+5w sub_7370r ...
					; <>0 в	трюме есть минеральная жизнь
					; =#FF минеральная жизнь померла


; === PilotFuelScoops === PilotFuelScoops:db 0			; DATA XREF: CollisionWithObj+4Fr
PilotFuelScoops:db 0			; DATA XREF: CollisionWithObj+4Fr
					; sub_DAB4:loc_DB6Cr


; === PilotEscapePod === PilotEscapePod:	db 0			; DATA XREF: sub_9946+3Br
PilotEscapePod:	db 0			; DATA XREF: sub_9946+3Br
					; UseEscapePod+13r


; === PilotEnergyBomb === PilotEnergyBomb:db 0			; DATA XREF: UseEnergyBomb+6o
PilotEnergyBomb:db 0			; DATA XREF: UseEnergyBomb+6o
byte_D08B:	db 0			; DATA XREF: EnergyRecoveryr


; === PilotDocComputer === PilotDocComputer:db 0			; DATA XREF: GoDocking+1Er
PilotDocComputer:db 0			; DATA XREF: GoDocking+1Er


; === PilotGyperdrive === PilotGyperdrive:db 0			; DATA XREF: sub_5D70o	GoJump+1Er
PilotGyperdrive:db 0			; DATA XREF: sub_5D70o	GoJump+1Er
byte_D08E:	db 0			; DATA XREF: sub_5FA1+11r
					; RAM:loc_72E3r ...
					; <>0 есть Cobra Mk4


; === PilotTargonWar === PilotTargonWar:	db 0			; DATA XREF: sub_5E78r	sub_5E78+18r ...
PilotTargonWar:	db 0			; DATA XREF: sub_5E78r	sub_5E78+18r ...
					; =#81 при сбросе миссий
					; 7,=1 начало войны, предолжение участвовать
					; 6-0, чем больше число	тем дальше от левого края карты	захват планет таргонами
					;
					; =#01 война закончена


; === PilotMission3 === PilotMission3:	db 0			; DATA XREF: ReCalcCrdObj_coriolis+1Er
PilotMission3:	db 0			; DATA XREF: ReCalcCrdObj_coriolis+1Er
					; GoMission3r ...
					; =#06 при стыковке будет захват таргонами (вроде нереальная ситуация)
					; =#04 в системе со танцией таргонов во	время войны


; === PilotExtWeapon === PilotExtWeapon:	db 0			; DATA XREF: sub_5EC5+Dr
PilotExtWeapon:	db 0			; DATA XREF: sub_5EC5+Dr
					; Use_Y_Cloacking+13o ...
					; 7,=1 - ECM Jammer
					; 6,=1 - Cloaking device
					; 5,=1 - нельзя	купить товары(в	трюме беженцы или десант)
					; 4
					; 3 =1 3я миссия была выполнена
					; 2 =1 - третья	миссия выполнена
					; 1 =1 - второя	миссия выполнена
					; 0 =1 - первая	миссия выполнена


; === PilotMission1_1 === PilotMission1_1:db 0			; DATA XREF: sub_726C+7w sub_728D+182w ...
PilotMission1_1:db 0			; DATA XREF: sub_726C+7w sub_728D+182w ...


