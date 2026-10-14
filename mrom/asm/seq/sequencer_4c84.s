; sequencer_4c84.s - M-ROM fixed_0000 [0x4c84..0x5362)
; Carved from mrom/asm/raw/fixed_0000.s by mrom/tools/split_region.py.
; Assembled through mrom/asm/fixed_0000.s; must stay byte-identical.

Seq_Init_4C84:
	ld hl,00000h		;4c84  constant
	ld (0fdf0h),hl		;4c87
	ld iy,wMusicTracks_FBFC		;4c8a
	ld b,00eh		;4c8e
	ld de,00023h		;4c90  constant
	xor a			;4c93
l4c94h:
	ld (iy+009h),a		;4c94
	ld (iy+000h),a		;4c97
	ld (iy+001h),a		;4c9a
	set 3,(iy+009h)		;4c9d
	add iy,de		;4ca1
	djnz l4c94h		;4ca3
	ret			;4ca5
Seq_Update_4CA6:
	ld iy,wMusicTracks_FBFC		;4ca6
	bit 6,(iy+009h)		;4caa
	call nz,Seq_TrackPortamento_52EA		;4cae
	ld iy,0fc1fh		;4cb1
	bit 6,(iy+009h)		;4cb5
	call nz,Seq_TrackPortamento_52EA		;4cb9
	ld iy,0fc42h		;4cbc
	bit 6,(iy+009h)		;4cc0
	call nz,Seq_TrackPortamento_52EA		;4cc4
	ld iy,0fc65h		;4cc7
	bit 6,(iy+009h)		;4ccb
	call nz,Seq_TrackPortamento_52EA		;4ccf
	ld iy,0fc88h		;4cd2
	bit 6,(iy+009h)		;4cd6
	call nz,Seq_TrackPortamento_52EA		;4cda
	ld iy,0fcabh		;4cdd
	bit 6,(iy+009h)		;4ce1
	call nz,Seq_TrackPortamento_52EA		;4ce5
	ld iy,0fcceh		;4ce8
	bit 6,(iy+009h)		;4cec
	call nz,Seq_TrackPortamento_52EA		;4cf0
	ld iy,0fcf1h		;4cf3
	bit 6,(iy+009h)		;4cf7
	call nz,Seq_TrackPortamento_52EA		;4cfb
	ld iy,0fd14h		;4cfe
	bit 6,(iy+009h)		;4d02
	call nz,Seq_TrackPortamento_52EA		;4d06
	ld iy,0fd37h		;4d09
	bit 6,(iy+009h)		;4d0d
	call nz,Seq_TrackPortamento_52EA		;4d11
	ld iy,0fd5ah		;4d14
	bit 6,(iy+009h)		;4d18
	call nz,Seq_TrackPortamento_52EA		;4d1c
	ld iy,0fd7dh		;4d1f
	bit 6,(iy+009h)		;4d23
	call nz,Seq_TrackPortamento_52EA		;4d27
	ld iy,0fda0h		;4d2a
	bit 6,(iy+009h)		;4d2e
	call nz,Seq_TrackPortamento_52EA		;4d32
	ld iy,0fdc3h		;4d35
	bit 6,(iy+009h)		;4d39
	call nz,Seq_TrackPortamento_52EA		;4d3d
	ld hl,(wSfxFlag_F804)		;4d40
	ld bc,(wVolumeLevel_F806)		;4d43
	ld a,(wFadeFlag_F808)		;4d47
	or a			;4d4a
	jr nz,l4d65h		;4d4b
	ld a,h			;4d4d
	cp 0ffh		;4d4e
	jp z,l4d70h		;4d50
	add hl,bc			;4d53
	jr c,l4d5eh		;4d54
	ld a,h			;4d56
	cp 0ffh		;4d57
	jr c,l4d5eh		;4d59
	jp l4d70h		;4d5b
l4d5eh:
	ld h,0ffh		;4d5e
	ld l,0ffh		;4d60
	jp l4d70h		;4d62
l4d65h:
	ld a,l			;4d65
	or h			;4d66
	jr z,l4d70h		;4d67
	sbc hl,bc		;4d69
	jr nc,l4d70h		;4d6b
	ld hl,00000h		;4d6d  constant
l4d70h:
	ld (wSfxFlag_F804),hl		;4d70
	xor a			;4d73
	ld (wTempoStep_F809),a		;4d74
	ld a,(0fde7h)		;4d77
	push af			;4d7a
	ld hl,0fde7h		;4d7b
	ld a,(hl)			;4d7e
	inc hl			;4d7f
	sub (hl)			;4d80
	jp nz,l4d87h		;4d81
	jp l4dc9h		;4d84
l4d87h:
	inc hl			;4d87
	ld a,(hl)			;4d88
	and a			;4d89
	jp nz,l4d94h		;4d8a
	dec hl			;4d8d
	ld a,(hl)			;4d8e
	dec hl			;4d8f
	ld (hl),a			;4d90
	jp l4dc9h		;4d91
l4d94h:
	dec hl			;4d94
	ld a,(hl)			;4d95
	sra a		;4d96
	dec hl			;4d98
	ld b,(hl)			;4d99
	sra b		;4d9a
	sub b			;4d9c
	jr nz,l4da5h		;4d9d
	inc hl			;4d9f
	inc hl			;4da0
	ld (hl),a			;4da1
	jp l4dc9h		;4da2
l4da5h:
	inc hl			;4da5
	inc hl			;4da6
	ld c,a			;4da7
	add a,a			;4da8
	sbc a,a			;4da9
	ld b,a			;4daa
	xor c			;4dab
	ld d,014h		;4dac
	ld e,a			;4dae
	ld a,(de)			;4daf
	ld e,(hl)			;4db0
	ex de,hl			;4db1
	add a,(hl)			;4db2
	jp c,l4db7h		;4db3
	xor a			;4db6
l4db7h:
	inc h			;4db7
	ld l,a			;4db8
	ld a,(hl)			;4db9
	xor b			;4dba
	ld h,b			;4dbb
	ld l,a			;4dbc
	add hl,hl			;4dbd
	add hl,hl			;4dbe
	add hl,hl			;4dbf
	add hl,hl			;4dc0
	ex de,hl			;4dc1
	ld hl,(wMusicHdr_FDE6)		;4dc2
	add hl,de			;4dc5
	ld (wMusicHdr_FDE6),hl		;4dc6
l4dc9h:
	pop bc			;4dc9
	ld a,(0fde7h)		;4dca
	cp b			;4dcd
	call nz,sub_4f40h		;4dce
	ld hl,0fdebh		;4dd1
	ld a,(hl)			;4dd4
	inc hl			;4dd5
	sub (hl)			;4dd6
	jp nz,l4dddh		;4dd7
	jp l4e1fh		;4dda
l4dddh:
	inc hl			;4ddd
	ld a,(hl)			;4dde
	and a			;4ddf
	jp nz,l4deah		;4de0
	dec hl			;4de3
	ld a,(hl)			;4de4
	dec hl			;4de5
	ld (hl),a			;4de6
	jp l4e1fh		;4de7
l4deah:
	dec hl			;4dea
	ld a,(hl)			;4deb
	sra a		;4dec
	dec hl			;4dee
	ld b,(hl)			;4def
	sra b		;4df0
	sub b			;4df2
	jr nz,l4dfbh		;4df3
	inc hl			;4df5
	inc hl			;4df6
	ld (hl),a			;4df7
	jp l4e1fh		;4df8
l4dfbh:
	inc hl			;4dfb
	inc hl			;4dfc
	ld c,a			;4dfd
	add a,a			;4dfe
	sbc a,a			;4dff
	ld b,a			;4e00
	xor c			;4e01
	ld d,014h		;4e02
	ld e,a			;4e04
	ld a,(de)			;4e05
	ld e,(hl)			;4e06
	ex de,hl			;4e07
	add a,(hl)			;4e08
	jp c,l4e0dh		;4e09
	xor a			;4e0c
l4e0dh:
	inc h			;4e0d
	ld l,a			;4e0e
	ld a,(hl)			;4e0f
	xor b			;4e10
	ld h,b			;4e11
	ld l,a			;4e12
	add hl,hl			;4e13
	add hl,hl			;4e14
	add hl,hl			;4e15
	add hl,hl			;4e16
	ex de,hl			;4e17
	ld hl,(0fdeah)		;4e18
	add hl,de			;4e1b
	ld (0fdeah),hl		;4e1c
l4e1fh:
	ld a,(0fdebh)		;4e1f
	ld b,a			;4e22
	ld a,(0fde7h)		;4e23
	add a,b			;4e26
	jp po,l4e2dh		;4e27
	sbc a,a			;4e2a
	xor 07fh		;4e2b
l4e2dh:
	add a,080h		;4e2d
	ld l,a			;4e2f
	ld h,000h		;4e30
	inc hl			;4e32
	ld a,(wTickDiv_F803)		;4e33
	and a			;4e36
	ret z			;4e37
	ex de,hl			;4e38
	ld hl,(0fdf0h)		;4e39
	dec a			;4e3c
	jp z,l4e46h		;4e3d
	dec a			;4e40
	jp z,l4e45h		;4e41
	add hl,de			;4e44
l4e45h:
	add hl,de			;4e45
l4e46h:
	add hl,de			;4e46
	ld (0fdf0h),hl		;4e47
	ld a,h			;4e4a
	and a			;4e4b
	ret z			;4e4c
	dec a			;4e4d
	and 003h		;4e4e
	ld (0fdf1h),a		;4e50
	ld a,001h		;4e53
	ld (wTempoStep_F809),a		;4e55
	ld iy,wMusicTracks_FBFC		;4e58
	ld hl,0fc05h		;4e5c
	ld a,(hl)			;4e5f
	and 0f7h		;4e60
	ld (hl),a			;4e62
	and 004h		;4e63
	call nz,Seq_TrackStep_532B		;4e65
	ld iy,0fc1fh		;4e68
	ld hl,0fc28h		;4e6c
	ld a,(hl)			;4e6f
	and 0f7h		;4e70
	ld (hl),a			;4e72
	and 004h		;4e73
	call nz,Seq_TrackStep_532B		;4e75
	ld iy,0fc42h		;4e78
	ld hl,0fc4bh		;4e7c
	ld a,(hl)			;4e7f
	and 0f7h		;4e80
	ld (hl),a			;4e82
	and 004h		;4e83
	call nz,Seq_TrackStep_532B		;4e85
	ld iy,0fc65h		;4e88
	ld hl,0fc6eh		;4e8c
	ld a,(hl)			;4e8f
	and 0f7h		;4e90
	ld (hl),a			;4e92
	and 004h		;4e93
	call nz,Seq_TrackStep_532B		;4e95
	ld iy,0fc88h		;4e98
	ld hl,0fc91h		;4e9c
	ld a,(hl)			;4e9f
	and 0f7h		;4ea0
	ld (hl),a			;4ea2
	and 004h		;4ea3
	call nz,Seq_TrackStep_532B		;4ea5
	ld iy,0fcabh		;4ea8
	ld hl,0fcb4h		;4eac
	ld a,(hl)			;4eaf
	and 0f7h		;4eb0
	ld (hl),a			;4eb2
	and 004h		;4eb3
	call nz,Seq_TrackStep_532B		;4eb5
	ld iy,0fcceh		;4eb8
	ld hl,0fcd7h		;4ebc
	ld a,(hl)			;4ebf
	and 0f7h		;4ec0
	ld (hl),a			;4ec2
	and 004h		;4ec3
	call nz,Seq_TrackStep_532B		;4ec5
	ld iy,0fcf1h		;4ec8
	ld hl,0fcfah		;4ecc
	ld a,(hl)			;4ecf
	and 0f7h		;4ed0
	ld (hl),a			;4ed2
	and 004h		;4ed3
	call nz,Seq_TrackStep_532B		;4ed5
	ld iy,0fd14h		;4ed8
	ld hl,0fd1dh		;4edc
	ld a,(hl)			;4edf
	and 0f7h		;4ee0
	ld (hl),a			;4ee2
	and 004h		;4ee3
	call nz,Seq_TrackStep_532B		;4ee5
	ld iy,0fd37h		;4ee8
	ld hl,0fd40h		;4eec
	ld a,(hl)			;4eef
	and 0f7h		;4ef0
	ld (hl),a			;4ef2
	and 004h		;4ef3
	call nz,Seq_TrackStep_532B		;4ef5
	ld iy,0fd5ah		;4ef8
	ld hl,0fd63h		;4efc
	ld a,(hl)			;4eff
	and 0f7h		;4f00
	ld (hl),a			;4f02
	and 004h		;4f03
	call nz,Seq_TrackStep_532B		;4f05
	ld iy,0fd7dh		;4f08
	ld hl,0fd86h		;4f0c
	ld a,(hl)			;4f0f
	and 0f7h		;4f10
	ld (hl),a			;4f12
	and 004h		;4f13
	call nz,Seq_TrackStep_532B		;4f15
	ld iy,0fda0h		;4f18
	ld hl,0fda9h		;4f1c
	ld a,(hl)			;4f1f
	and 0f7h		;4f20
	ld (hl),a			;4f22
	and 004h		;4f23
	call nz,Seq_TrackStep_532B		;4f25
	ld iy,0fdc3h		;4f28
	ld hl,0fdcch		;4f2c
	ld a,(hl)			;4f2f
	and 0f7h		;4f30
	ld (hl),a			;4f32
	and 004h		;4f33
	call nz,Seq_TrackStep_532B		;4f35
	ret			;4f38
Seq_SetTempo_4F39:
	ld (0fdedh),a		;4f39
	ld a,b			;4f3c
	ld (0fdeeh),a		;4f3d
sub_4f40h:
	ld a,(0fdeeh)		;4f40
	cp 040h		;4f43
	jr nz,l4f4ch		;4f45
	xor a			;4f47
	ld (0fdech),a		;4f48
	ret			;4f4b
l4f4ch:
	ld l,a			;4f4c
	ld a,(0fde7h)		;4f4d
	call sub_4f69h		;4f50
	ld hl,0fde7h		;4f53
	sub (hl)			;4f56
	ld (0fdech),a		;4f57
	ret			;4f5a
l4f5bh:
	ld (0fde9h),a		;4f5b
	ld l,b			;4f5e
	ld a,(0fdefh)		;4f5f
	call sub_4f69h		;4f62
	ld (0fde8h),a		;4f65
	ret			;4f68
sub_4f69h:
	add a,080h		;4f69
	ld h,014h		;4f6b
	ld e,(hl)			;4f6d
	ld l,a			;4f6e
	ld a,(hl)			;4f6f
	add a,e			;4f70
	ld b,0c0h		;4f71
	jp c,l4f7dh		;4f73
	sub b			;4f76
	jr nc,l4f82h		;4f77
	xor a			;4f79
	jp l4f82h		;4f7a
l4f7dh:
	sub b			;4f7d
	jr c,l4f82h		;4f7e
	ld a,0ffh		;4f80
l4f82h:
	ld l,a			;4f82
	inc h			;4f83
	ld a,(hl)			;4f84
	sub 080h		;4f85
	ret			;4f87
Seq_SetVolumeLevel_4F88:
	and a			;4f88
	jr nz,l4f92h		;4f89
	ld hl,0ffffh		;4f8b
	ld (wVolumeLevel_F806),hl		;4f8e
	ret			;4f91
l4f92h:
	ld l,a			;4f92
	ld h,014h		;4f93
	ld a,(hl)			;4f95
	cpl			;4f96
	inc h			;4f97
	ld l,(hl)			;4f98
	ld h,000h		;4f99
	add hl,hl			;4f9b
	add hl,hl			;4f9c
	add hl,hl			;4f9d
	add hl,hl			;4f9e
	ld (wVolumeLevel_F806),hl		;4f9f
	ret			;4fa2
Seq_StartMusic_4FA3:
	xor a			;4fa3
	out (00ch),a		;4fa4
	ld l,(ix+000h)		;4fa6
	ld h,000h		;4fa9
	ld de,NoteMapTable_4B84		;4fab
	add hl,de			;4fae
	ld a,(hl)			;4faf
	ld (0fdefh),a		;4fb0
	ld (0fde7h),a		;4fb3
	ld (0fde8h),a		;4fb6
	xor a			;4fb9
	ld (0fde9h),a		;4fba
	ld (wMusicHdr_FDE6),a		;4fbd
	ld a,040h		;4fc0
	ld (0fdeeh),a		;4fc2
	xor a			;4fc5
	ld (0fdf0h),a		;4fc6
	ld (0fdebh),a		;4fc9
	ld (0fdech),a		;4fcc
	xor a			;4fcf
	ld (0fdedh),a		;4fd0
	ld (0fdeah),a		;4fd3
	inc ix		;4fd6
	xor a			;4fd8
	ld (wMasterVolume_F805),a		;4fd9
	ld (wFadeFlag_F808),a		;4fdc
	ld iy,wMusicTracks_FBFC		;4fdf
	ld b,00eh		;4fe3
l4fe5h:
	ld l,(ix+000h)		;4fe5
	ld h,(ix+001h)		;4fe8
	ld a,(ix+002h)		;4feb
	or h			;4fee
	or l			;4fef
	jr z,l500dh		;4ff0
	push bc			;4ff2
	call Seq_TrackInit_5251		;4ff3
	set 1,(iy+009h)		;4ff6
	ld a,(ix+004h)		;4ffa
	ld (iy+002h),a		;4ffd
	ld de,00023h		;5000  constant
	add iy,de		;5003
	ld de,00005h		;5005
	add ix,de		;5008
	pop bc			;500a
	djnz l4fe5h		;500b
l500dh:
	ld a,b			;500d
	and a			;500e
	jp z,l5024h		;500f
	xor a			;5012
l5013h:
	ld (iy+009h),008h		;5013
	ld (iy+000h),a		;5017
	ld (iy+001h),a		;501a
	ld de,00023h		;501d  constant
	add iy,de		;5020
	djnz l5013h		;5022
l5024h:
	ret			;5024
Seq_InitTracks_5025:
	ld iy,wSfxTracks_FDF2		;5025
	ld b,008h		;5029
	ld de,00023h		;502b  constant
	xor a			;502e
l502fh:
	ld (iy+009h),a		;502f
	set 3,(iy+009h)		;5032
	ld (iy+002h),0ffh		;5036
	ld (iy+003h),a		;503a
	ld (iy+000h),a		;503d
	ld (iy+001h),a		;5040
	ld (iy+00ah),a		;5043
	ld (iy+00bh),a		;5046
	add iy,de		;5049
	djnz l502fh		;504b
	ret			;504d
Seq_StopAllTracks_504E:
	ld iy,wSfxTracks_FDF2		;504e
	ld b,008h		;5052
	ld de,00023h		;5054  constant
	ld c,008h		;5057
l5059h:
	ld a,(iy+00bh)		;5059
	inc a			;505c
	jr z,l5076h		;505d
	ld (iy+009h),c		;505f
	xor a			;5062
	ld (iy+002h),0ffh		;5063
	ld (iy+003h),a		;5067
	ld (iy+000h),a		;506a
	ld (iy+001h),a		;506d
	ld (iy+00ah),a		;5070
	ld (iy+00bh),a		;5073
l5076h:
	add iy,de		;5076
	djnz l5059h		;5078
	ret			;507a
Seq_StartSfx_507B:
	ld c,(ix+000h)		;507b
	ld b,(ix+001h)		;507e
	call Seq_StopTrackById_5210		;5081
	ld c,(ix+002h)		;5084
	ld b,(ix+003h)		;5087
	call Seq_StopTrackById_5210		;508a
	ld c,(ix+004h)		;508d
	ld b,(ix+005h)		;5090
	call Seq_StopTrackById_5210		;5093
	ld c,(ix+006h)		;5096
	ld b,(ix+007h)		;5099
	call Seq_StopTrackById_5210		;509c
	ld d,(ix+008h)		;509f
	ld bc,00009h		;50a2  constant
	add ix,bc		;50a5
l50a7h:
	ld l,(ix+000h)		;50a7
	ld h,(ix+001h)		;50aa
	ld a,(ix+002h)		;50ad
	or l			;50b0
	or h			;50b1
	jr z,l50dah		;50b2
	ld e,(ix+004h)		;50b4
	push de			;50b7
	push hl			;50b8
	call Seq_FindFreeTrack_50DB		;50b9
	ld a,h			;50bc
	pop hl			;50bd
	pop de			;50be
	and a			;50bf
	jr z,l50d1h		;50c0
	push de			;50c2
	call Seq_TrackInit_5251		;50c3
	pop de			;50c6
	ld (iy+002h),e		;50c7
	ld (iy+003h),d		;50ca
	set 0,(iy+009h)		;50cd
l50d1h:
	ld bc,00005h		;50d1
	add ix,bc		;50d4
	jp l50a7h		;50d6
	ret			;50d9
l50dah:
	ret			;50da
Seq_FindFreeTrack_50DB:
	ld iy,wSfxTracks_FDF2		;50db
	ld b,008h		;50df
	ld c,e			;50e1
	ld h,000h		;50e2
l50e4h:
	bit 2,(iy+009h)		;50e4
	jp z,l5133h		;50e8
	ld a,d			;50eb
	and a			;50ec
	jr z,l511bh		;50ed
	cp (iy+003h)		;50ef
	jp nz,l511bh		;50f2
	exx			;50f5
	ld e,(iy+007h)		;50f6
	ld d,(iy+008h)		;50f9
	ld hl,(wCmdCounter_F819)		;50fc
	and a			;50ff
	sbc hl,de		;5100
	exx			;5102
	jr z,l511bh		;5103
	ld a,(iy+002h)		;5105
	cp e			;5108
	jr nc,l510fh		;5109
	ld h,000h		;510b
	jr l514ah		;510d
l510fh:
	push bc			;510f
	push de			;5110
	push hl			;5111
	call Seq_TrackStop_578D		;5112
	pop hl			;5115
	pop de			;5116
	pop bc			;5117
	jp l5133h		;5118
l511bh:
	ld a,h			;511b
	cp 002h		;511c
	jp z,l5141h		;511e
	ld a,(iy+002h)		;5121
	cp c			;5124
	jr c,l5141h		;5125
	ld c,(iy+002h)		;5127
	ld h,001h		;512a
	ld (wCurTrack_FF0A),iy		;512c
	jp l5141h		;5130
l5133h:
	ld h,002h		;5133
	ld (wCurTrack_FF0A),iy		;5135
	ld a,d			;5139
	and a			;513a
	jp z,l514ah		;513b
	jp l5141h		;513e
l5141h:
	exx			;5141
	ld de,00023h		;5142  constant
	add iy,de		;5145
	exx			;5147
	djnz l50e4h		;5148
l514ah:
	ld iy,(wCurTrack_FF0A)		;514a
	ret			;514e
Seq_UpdateTracks_514F:
	ld hl,0fdfbh		;514f
	ld a,(hl)			;5152
	and 0f7h		;5153
	ld (hl),a			;5155
	bit 2,a		;5156
	jp z,l5167h		;5158
	ld iy,wSfxTracks_FDF2		;515b
	bit 6,a		;515f
	call nz,Seq_TrackPortamento_52EA		;5161
	call Seq_TrackStep_532B		;5164
l5167h:
	ld hl,0fe1eh		;5167
	ld a,(hl)			;516a
	and 0f7h		;516b
	ld (hl),a			;516d
	bit 2,a		;516e
	jp z,l517fh		;5170
	ld iy,0fe15h		;5173
	bit 6,a		;5177
	call nz,Seq_TrackPortamento_52EA		;5179
	call Seq_TrackStep_532B		;517c
l517fh:
	ld hl,0fe41h		;517f
	ld a,(hl)			;5182
	and 0f7h		;5183
	ld (hl),a			;5185
	bit 2,a		;5186
	jp z,l5197h		;5188
	ld iy,0fe38h		;518b
	bit 6,a		;518f
	call nz,Seq_TrackPortamento_52EA		;5191
	call Seq_TrackStep_532B		;5194
l5197h:
	ld hl,0fe64h		;5197
	ld a,(hl)			;519a
	and 0f7h		;519b
	ld (hl),a			;519d
	bit 2,a		;519e
	jp z,l51afh		;51a0
	ld iy,0fe5bh		;51a3
	bit 6,a		;51a7
	call nz,Seq_TrackPortamento_52EA		;51a9
	call Seq_TrackStep_532B		;51ac
l51afh:
	ld hl,0fe87h		;51af
	ld a,(hl)			;51b2
	and 0f7h		;51b3
	ld (hl),a			;51b5
	bit 2,a		;51b6
	jp z,l51c7h		;51b8
	ld iy,0fe7eh		;51bb
	bit 6,a		;51bf
	call nz,Seq_TrackPortamento_52EA		;51c1
	call Seq_TrackStep_532B		;51c4
l51c7h:
	ld hl,0feaah		;51c7
	ld a,(hl)			;51ca
	and 0f7h		;51cb
	ld (hl),a			;51cd
	bit 2,a		;51ce
	jp z,l51dfh		;51d0
	ld iy,0fea1h		;51d3
	bit 6,a		;51d7
	call nz,Seq_TrackPortamento_52EA		;51d9
	call Seq_TrackStep_532B		;51dc
l51dfh:
	ld hl,0fecdh		;51df
	ld a,(hl)			;51e2
	and 0f7h		;51e3
	ld (hl),a			;51e5
	bit 2,a		;51e6
	jp z,l51f7h		;51e8
	ld iy,0fec4h		;51eb
	bit 6,a		;51ef
	call nz,Seq_TrackPortamento_52EA		;51f1
	call Seq_TrackStep_532B		;51f4
l51f7h:
	ld hl,0fef0h		;51f7
	ld a,(hl)			;51fa
	and 0f7h		;51fb
	ld (hl),a			;51fd
	bit 2,a		;51fe
	jp z,l520fh		;5200
	ld iy,0fee7h		;5203
	bit 6,a		;5207
	call nz,Seq_TrackPortamento_52EA		;5209
	call Seq_TrackStep_532B		;520c
l520fh:
	ret			;520f
Seq_StopTrackById_5210:
	ld a,c			;5210
	or b			;5211
	ret z			;5212
	push ix		;5213
	ld ix,wSfxTracks_FDF2		;5215
	ld a,008h		;5219
l521bh:
	ld l,(ix+005h)		;521b
	ld h,(ix+006h)		;521e
	and a			;5221
	sbc hl,bc		;5222
	jp nz,l5246h		;5224
	ld hl,(wCmdCounter_F819)		;5227
	ld e,(ix+007h)		;522a
	ld d,(ix+008h)		;522d
	and a			;5230
	sbc hl,de		;5231
	jp z,l5246h		;5233
	ld (ix+009h),008h		;5236
	ld (ix+000h),000h		;523a
	ld (ix+001h),000h		;523e
	ld (ix+002h),0ffh		;5242
l5246h:
	ld de,00023h		;5246  constant
	add ix,de		;5249
	dec a			;524b
	jr nz,l521bh		;524c
	pop ix		;524e
	ret			;5250
Seq_TrackInit_5251:
	defb 0ddh,05dh	;ld e,ixl		;5251
	defb 0ddh,054h	;ld d,ixh		;5253
	ex de,hl			;5255
	ld a,(wCurBank_F811)		;5256
	ld b,a			;5259
	push bc			;525a
	call Bank_ResolvePtr24_1FA6		;525b
	ld (iy+014h),l		;525e
	ld (iy+015h),h		;5261
	ld (iy+004h),b		;5264
	pop bc			;5267
	ld a,(wCurBank_F811)		;5268
	cp b			;526b
	jp z,l5289h		;526c
	ld a,b			;526f
	ld (wCurBank_F811),a		;5270
	ld c,00bh		;5273
	in a,(c)		;5275
	inc b			;5277
	sla b		;5278
	dec c			;527a
	in a,(c)		;527b
	inc b			;527d
	sla b		;527e
	dec c			;5280
	in a,(c)		;5281
	inc b			;5283
	sla b		;5284
	dec c			;5286
	in a,(c)		;5287
l5289h:
	xor a			;5289
	ld (iy+003h),a		;528a
	ld (iy+009h),a		;528d
	set 2,(iy+009h)		;5290
	set 3,(iy+009h)		;5294
	ld (iy+000h),a		;5298
	ld (iy+001h),a		;529b
	ld (iy+00ah),0ffh		;529e
	ld (iy+00bh),0ffh		;52a2
	ld (iy+016h),a		;52a6
	ld (iy+00ch),a		;52a9
	ld (iy+00dh),a		;52ac
	ld (iy+00eh),a		;52af
	ld (iy+010h),a		;52b2
	ld (iy+011h),a		;52b5
	ld (iy+012h),0ffh		;52b8
	ld (iy+00fh),003h		;52bc
	ld (iy+013h),a		;52c0
	ld (iy+022h),006h		;52c3
	ld hl,(wCmdCounter_F819)		;52c7
	ld (iy+007h),l		;52ca
	ld (iy+008h),h		;52cd
	ld hl,(wSfxId_F81B)		;52d0
	ld (iy+005h),l		;52d3
	ld (iy+006h),h		;52d6
	ld (iy+017h),040h		;52d9
	ld (iy+018h),0c0h		;52dd
	ld (iy+019h),0c0h		;52e1
	ld (iy+01ah),07fh		;52e5
	ret			;52e9
Seq_TrackPortamento_52EA:
	bit 6,(iy+009h)		;52ea
	ret z			;52ee
	ld a,(iy+010h)		;52ef
	or (iy+011h)		;52f2
	ret z			;52f5
	ld a,(iy+013h)		;52f6
	and 07fh		;52f9
	add a,a			;52fb
	ld h,01ah		;52fc
	ld l,a			;52fe
	ld e,(hl)			;52ff
	inc hl			;5300
	ld d,(hl)			;5301
	ld l,(iy+010h)		;5302
	ld h,(iy+011h)		;5305
	ld a,h			;5308
	and a			;5309
	jp p,l5318h		;530a
	adc hl,de		;530d
	jr c,l5323h		;530f
	ld (iy+010h),l		;5311
	ld (iy+011h),h		;5314
	ret			;5317
l5318h:
	sbc hl,de		;5318
	jr c,l5323h		;531a
	ld (iy+010h),l		;531c
	ld (iy+011h),h		;531f
	ret			;5322
l5323h:
	xor a			;5323
	ld (iy+010h),a		;5324
	ld (iy+011h),a		;5327
	ret			;532a
Seq_TrackStep_532B:
	ld a,(iy+016h)		;532b
	and a			;532e
	jp nz,l534dh		;532f
	ld l,(iy+014h)		;5332
	ld h,(iy+015h)		;5335
	ld (wSeqPtr_FF0D),hl		;5338
	jp Seq_TrackFetch_5362		;533b
Seq_TrackNextOp_533E:
	ld a,(iy+016h)		;533e
	and a			;5341
	jr z,Seq_TrackFetch_5362		;5342
	ld hl,(wSeqPtr_FF0D)		;5344
	ld (iy+014h),l		;5347
	ld (iy+015h),h		;534a
l534dh:
	dec a			;534d
	ld (iy+016h),a		;534e
	bit 1,(iy+009h)		;5351
	ret nz			;5355
	bit 5,(iy+009h)		;5356
	ret z			;535a
	call Ch_KeyOn_4AE2		;535b
	ret c			;535e
	jp Seq_TrackEndCheck_5786		;535f
