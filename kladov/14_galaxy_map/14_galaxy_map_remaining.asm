; 14_galaxy_map - оставшиеся подпрограммы
; Количество: 4

; === PilotNumGalaxy === PilotNumGalaxy:	db 0			; DATA XREF: ReCalcCrdObj_coriolis+19r
PilotNumGalaxy:	db 0			; DATA XREF: ReCalcCrdObj_coriolis+19r
					; TestPilotInZoneWar:loc_D67Dr	...


; === PilotECMSystem === PilotECMSystem:	db 0			; DATA XREF: LaunchDocking+3Eo
PilotECMSystem:	db 0			; DATA XREF: LaunchDocking+3Eo
					; sub_B7F4+13r	...


; === PilotGalaxyData0 === PilotGalaxyData0:db 4Ah			; DATA XREF: sub_79F9+22o sub_D857o ...
PilotGalaxyData0:db 4Ah			; DATA XREF: sub_79F9+22o sub_D857o ...
		db 5Ah
byte_D092:	db 48h			; DATA XREF: RAM:DF11o
		db 2
byte_D094:	db 53h			; DATA XREF: RAM:DF17o
		db 0B7h


; === ControlKeyInMap === ControlKeyInMap:db 0			; DATA XREF: ControlKeyIfMapOn+14r
ControlKeyInMap:db 0			; DATA XREF: ControlKeyIfMapOn+14r
					; ControlKeyIfMapOn+27w


