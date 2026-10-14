; command_2050.s - M-ROM fixed_0000 [0x1fe5..0x2076)
; Carved from mrom/asm/raw/fixed_0000.s by mrom/tools/split_region.py.
; Assembled through mrom/asm/fixed_0000.s; must stay byte-identical.

Ch_InitStruct_1FE5:
	ld bc,00040h		;1fe5  constant
	ld (ix+000h),c		;1fe8
	ld (ix+001h),b		;1feb
	xor a			;1fee
	ld (ix+00bh),a		;1fef
	ld (ix+005h),a		;1ff2
	ld (ix+007h),a		;1ff5
	ld (ix+010h),a		;1ff8
	ld (ix+008h),a		;1ffb
	ld (ix+009h),a		;1ffe
	ld (ix+00ah),a		;2001
	dec a			;2004
	ld (ix+003h),a		;2005
	ld (ix+004h),a		;2008
	ld (ix+006h),a		;200b
	ld (ix+00eh),a		;200e
	ld (ix+00fh),a		;2011
	ld (ix+00ch),a		;2014
	ld (ix+00dh),a		;2017
	ret			;201a
Ch_ResetStruct_201B:
	xor a			;201b
	ld (ix+00bh),a		;201c
	ld (ix+005h),a		;201f
	ld (ix+007h),a		;2022
	dec a			;2025
	ld (ix+003h),a		;2026
	ld (ix+004h),a		;2029
	ld (ix+006h),a		;202c
	ld (ix+00eh),a		;202f
	ld (ix+00fh),a		;2032
	ld hl,00040h		;2035  constant
	ld (ix+000h),l		;2038
	ld (ix+001h),h		;203b
	ret			;203e
Cmd_InitState_203F:
	xor a			;203f
	ld (wSfxPending_F816),a		;2040
	ld (wSfxId_F81B),a		;2043
	ld (wCmdType_F81C),a		;2046
	ld (wSoundParam_F817),a		;2049
	ld (wMusicId_F818),a		;204c
	ret			;204f
Cmd_Dispatch_2050:
	call NMI_RingPop_1EBB		;2050
	ret nc			;2053
	ld hl,(wCmdCounter_F819)		;2054
	inc hl			;2057
	ld (wCmdCounter_F819),hl		;2058
	cp 020h		;205b
	jr nc,Cmd_DispatchHi_2096		;205d
	cp 00fh		;205f
	jr z,l2066h		;2061
	ld (wCmdType_F81C),a		;2063
l2066h:
	cp 010h		;2066
	ret nc			;2068
	add a,a			;2069
	ld e,a			;206a
	ld d,000h		;206b
	ld hl,CmdJumpTable_2076		;206d
	add hl,de			;2070
	ld a,(hl)			;2071
	inc hl			;2072
	ld h,(hl)			;2073
	ld l,a			;2074
	jp (hl)			;2075
