; main_1e00.s - M-ROM fixed_0000 [0x1e00..0x1fe5)
; Carved from mrom/asm/raw/fixed_0000.s by mrom/tools/split_region.py.
; Assembled through mrom/asm/fixed_0000.s; must stay byte-identical.

Main_Init_1E00:
	xor a			;1e00
	out (00ch),a		;1e01
	ld sp,wStackTop_FFF0		;1e03
	ld bc,00004h		;1e06
	ld (wYmPortLatch_F80B),bc		;1e09
	call Bank_SelectInit_1F7D		;1e0d
	call NMI_RingReset_1EB3		;1e10
	call Cmd_InitState_203F		;1e13
	call SSG_Init_2515		;1e16
	call FM_Init_2B43		;1e19
	call ADPCMA_Init_3740		;1e1c
	call ADPCMB_Init_3FCA		;1e1f
	call Seq_Init_4C84		;1e22
	call Seq_InitTracks_5025		;1e25
	xor a			;1e28
	ld (wTickPending_F801),a		;1e29
	ld (wYmStatus_F80A),a		;1e2c
	ld (wTickCount_F802),a		;1e2f
	ld a,001h		;1e32
	ld (wTickDiv_F803),a		;1e34
	call YM_InitTimers_1E6B		;1e37
	im 1		;1e3a
	out (008h),a		;1e3c
	ei			;1e3e
Main_Loop_1E3F:
	ld hl,wTick_F800		;1e3f
	inc (hl)			;1e42
	call Cmd_Dispatch_2050		;1e43
	call SSG_Update_2593		;1e46
	call FM_Update_2C5F		;1e49
	call ADPCMA_Update_379C		;1e4c
	call ADPCMB_Update_4024		;1e4f
	call Seq_UpdateTracks_514F		;1e52
	call Seq_Update_4CA6		;1e55
	ld hl,wTickPending_F801		;1e58
Main_WaitTick_1E5B:
	ld a,(hl)			;1e5b
	and a			;1e5c
	jp z,Main_WaitTick_1E5B		;1e5d
	dec a			;1e60
	ld (hl),a			;1e61
	inc hl			;1e62
	ld a,(hl)			;1e63
	ld (hl),000h		;1e64
	inc hl			;1e66
	ld (hl),a			;1e67
	jp Main_Loop_1E3F		;1e68
YM_InitTimers_1E6B:
	ld de,024ach		;1e6b  YM reg 24h <- ACh
	ld c,004h		;1e6e
	rst 8			;1e70
	ld de,02503h		;1e71  YM reg 25h <- 03h
	rst 8			;1e74
	ld de,02735h		;1e75  YM reg 27h <- 35h
	rst 8			;1e78
	ret			;1e79
IRQ_Timer_1E7A:
	push af			;1e7a
	push bc			;1e7b
	push de			;1e7c
	push hl			;1e7d
	push ix		;1e7e
	push iy		;1e80
	ld bc,(wYmPortLatch_F80B)		;1e82
	push bc			;1e86
	ld c,004h		;1e87
	ld de,02735h		;1e89  YM reg 27h <- 35h
	rst 8			;1e8c
	ld hl,wTickPending_F801		;1e8d
	ld a,(hl)			;1e90
	cp 002h		;1e91
	jr nc,l1e97h		;1e93
	inc a			;1e95
	ld (hl),a			;1e96
l1e97h:
	inc hl			;1e97
	inc (hl)			;1e98
	in a,(006h)		;1e99
	ld (wYmStatus_F80A),a		;1e9b
	ld c,004h		;1e9e
l1ea0h:
	in a,(c)		;1ea0
	jp m,l1ea0h		;1ea2
	pop bc			;1ea5
	out (c),b		;1ea6
	pop iy		;1ea8
	pop ix		;1eaa
	pop hl			;1eac
	pop de			;1ead
	pop bc			;1eae
	pop af			;1eaf
	ei			;1eb0
	reti		;1eb1
NMI_RingReset_1EB3:
	xor a			;1eb3
	ld (wNmiRingRd_F80F),a		;1eb4
	ld (wNmiRingWr_F810),a		;1eb7
	ret			;1eba
NMI_RingPop_1EBB:
	ld hl,wNmiRingWr_F810		;1ebb
	ld c,(hl)			;1ebe
	dec hl			;1ebf
	ld a,(hl)			;1ec0
	cp c			;1ec1
	jr z,l1ed4h		;1ec2
	and 00fh		;1ec4
	ld e,a			;1ec6
	ld d,000h		;1ec7
	inc a			;1ec9
	and 00fh		;1eca
	ld (hl),a			;1ecc
	ld hl,wStackTop_FFF0		;1ecd
	add hl,de			;1ed0
	ld a,(hl)			;1ed1
	scf			;1ed2
	ret			;1ed3
l1ed4h:
	and a			;1ed4
	ret			;1ed5
NMI_Command_1ED6:
	push af			;1ed6
	push bc			;1ed7
	push de			;1ed8
	push hl			;1ed9
	push ix		;1eda
	push iy		;1edc
	out (008h),a		;1ede
	in a,(000h)		;1ee0
	cp 001h		;1ee2
	jp z,NMI_SoftReset_1F0A		;1ee4
	cp 003h		;1ee7
	jp z,NMI_HardReset_1F4B		;1ee9
	ld c,a			;1eec
	ld a,(wNmiRingWr_F810)		;1eed
	and 00fh		;1ef0
	ld hl,wStackTop_FFF0		;1ef2
	ld e,a			;1ef5
	ld d,000h		;1ef6
	add hl,de			;1ef8
	ld (hl),c			;1ef9
	inc a			;1efa
	and 00fh		;1efb
	ld (wNmiRingWr_F810),a		;1efd
	pop iy		;1f00
	pop ix		;1f02
	pop hl			;1f04
	pop de			;1f05
	pop bc			;1f06
	pop af			;1f07
	retn		;1f08
NMI_SoftReset_1F0A:
	di			;1f0a
	ld c,004h		;1f0b
	ld de,007ffh		;1f0d  YM reg 07h <- FFh
	rst 8			;1f10
	ld de,01100h		;1f11  YM reg 11h <- 00h
	rst 8			;1f14
	ld de,01001h		;1f15  YM reg 10h <- 01h
	rst 8			;1f18
	ld de,0b500h		;1f19
	rst 8			;1f1c
	inc d			;1f1d
	rst 8			;1f1e
	inc c			;1f1f
	inc c			;1f20
	rst 8			;1f21
	dec d			;1f22
	rst 8			;1f23
	ld de,000ffh		;1f24  YM reg 00h <- FFh
	rst 8			;1f27
	ld de,00800h		;1f28  YM reg 08h <- 00h
	rst 8			;1f2b
	inc d			;1f2c
	rst 8			;1f2d
	inc d			;1f2e
	rst 8			;1f2f
	inc d			;1f30
	rst 8			;1f31
	inc d			;1f32
	rst 8			;1f33
	inc d			;1f34
	rst 8			;1f35
	ld hl,wTick_F800		;1f36
	ld (hl),018h		;1f39
	inc hl			;1f3b
	ld (hl),0feh		;1f3c
	ld sp,wStackTop_FFF0		;1f3e
	ld hl,wTick_F800		;1f41
	push hl			;1f44
	ld a,001h		;1f45
	out (00ch),a		;1f47
	retn		;1f49
NMI_HardReset_1F4B:
	ld sp,wStackTop_FFF0		;1f4b
	ld hl,Reset_0000		;1f4e
	push hl			;1f51
	retn		;1f52
Util_TableLookup_1F54:
	ld hl,00000h		;1f54  constant
	ld a,c			;1f57
	and a			;1f58
	ret z			;1f59
	ld h,014h		;1f5a
	ld l,c			;1f5c
	ld a,(hl)			;1f5d
	ld l,e			;1f5e
	ld e,000h		;1f5f
	add a,(hl)			;1f61
	jr nc,l1f68h		;1f62
	ld l,a			;1f64
	inc h			;1f65
	ld e,(hl)			;1f66
	dec h			;1f67
l1f68h:
	ld l,c			;1f68
	ld a,(hl)			;1f69
	ld l,d			;1f6a
	ld d,000h		;1f6b
	add a,(hl)			;1f6d
	ld l,a			;1f6e
	inc h			;1f6f
	jr c,l1f77h		;1f70
	ld l,(hl)			;1f72
	ld h,000h		;1f73
	add hl,de			;1f75
	ret			;1f76
l1f77h:
	ld a,(hl)			;1f77
	inc h			;1f78
	ld l,(hl)			;1f79
	ld h,a			;1f7a
	add hl,de			;1f7b
	ret			;1f7c
Bank_SelectInit_1F7D:
	ld a,0ffh		;1f7d
	ld (wCurBank_F811),a		;1f7f
	ld b,002h		;1f82
	ld a,(wCurBank_F811)		;1f84
	cp b			;1f87
	jp z,l1fa5h		;1f88
	ld a,b			;1f8b
	ld (wCurBank_F811),a		;1f8c
	ld c,00bh		;1f8f
	in a,(c)		;1f91
	inc b			;1f93
	sla b		;1f94
	dec c			;1f96
	in a,(c)		;1f97
	inc b			;1f99
	sla b		;1f9a
	dec c			;1f9c
	in a,(c)		;1f9d
	inc b			;1f9f
	sla b		;1fa0
	dec c			;1fa2
	in a,(c)		;1fa3
l1fa5h:
	ret			;1fa5
Bank_ResolvePtr24_1FA6:
	ld e,(hl)			;1fa6
	inc hl			;1fa7
	ld d,(hl)			;1fa8
	inc hl			;1fa9
	ld a,(hl)			;1faa
	ld l,e			;1fab
	ld h,d			;1fac
	add hl,hl			;1fad
	adc a,a			;1fae
	add hl,hl			;1faf
	adc a,a			;1fb0
	ld b,a			;1fb1
	push bc			;1fb2
	ld a,(wCurBank_F811)		;1fb3
	cp b			;1fb6
	jp z,l1fd4h		;1fb7
	ld a,b			;1fba
	ld (wCurBank_F811),a		;1fbb
	ld c,00bh		;1fbe
	in a,(c)		;1fc0
	inc b			;1fc2
	sla b		;1fc3
	dec c			;1fc5
	in a,(c)		;1fc6
	inc b			;1fc8
	sla b		;1fc9
	dec c			;1fcb
	in a,(c)		;1fcc
	inc b			;1fce
	sla b		;1fcf
	dec c			;1fd1
	in a,(c)		;1fd2
l1fd4h:
	pop bc			;1fd4
	ld a,d			;1fd5
	and 03fh		;1fd6
	ld d,a			;1fd8
	ld hl,08000h		;1fd9
	add hl,de			;1fdc
	ret			;1fdd
Bank_ReadWordBank2_1FDE:
	ld e,(hl)			;1fde
	inc hl			;1fdf
	ld d,(hl)			;1fe0
	ex de,hl			;1fe1
	ld b,002h		;1fe2
	ret			;1fe4
