; chtype_4aa0.s - M-ROM fixed_0000 [0x4aa0..0x4b84)
; Carved from mrom/asm/raw/fixed_0000.s by mrom/tools/split_region.py.
; Assembled through mrom/asm/fixed_0000.s; must stay byte-identical.

ChTypeJumpTable0_4AA0:
	defw SSG_LoadPatch_2AA8		;4aa0
	defw FM_LoadPatch_3568		;4aa2
	defw ADPCMA_LoadPatch_3F41		;4aa4
	defw ADPCMB_LoadPatch_427F		;4aa6
Ch_DispatchKeyOn_4AA8:
	ld hl,Ch_DispatchPitch_4AC4		;4aa8
	push hl			;4aab
	ld a,(ix+002h)		;4aac
	add a,a			;4aaf
	ld l,a			;4ab0
	ld h,000h		;4ab1
	ld de,ChTypeJumpTable1_4ABC		;4ab3
	add hl,de			;4ab6
	ld e,(hl)			;4ab7
	inc hl			;4ab8
	ld d,(hl)			;4ab9
	ex de,hl			;4aba
	jp (hl)			;4abb
ChTypeJumpTable1_4ABC:
	defw SSG_KeyOn_2B12		;4abc
	defw FM_KeyOn_3615		;4abe
	defw ADPCMA_KeyOn_3FBA		;4ac0
	defw ADPCMB_KeyOn_42EF		;4ac2
Ch_DispatchPitch_4AC4:
	ld hl,Ch_DispatchDone_4AE0		;4ac4
	push hl			;4ac7
	ld a,(ix+002h)		;4ac8
	add a,a			;4acb
	ld l,a			;4acc
	ld h,000h		;4acd
	ld de,ChTypeJumpTable2_4AD8		;4acf
	add hl,de			;4ad2
	ld e,(hl)			;4ad3
	inc hl			;4ad4
	ld d,(hl)			;4ad5
	ex de,hl			;4ad6
	jp (hl)			;4ad7
ChTypeJumpTable2_4AD8:
	defw SSG_SetPitch_2B02		;4ad8
	defw FM_SetPitch_3605		;4ada
	defw ADPCMA_SelectSample_3F60		;4adc
	defw ADPCMB_SetPitch_42CA		;4ade
Ch_DispatchDone_4AE0:
	scf			;4ae0
	ret			;4ae1
Ch_KeyOn_4AE2:
	ld e,(iy+000h)		;4ae2
	ld d,(iy+001h)		;4ae5
	ld a,e			;4ae8
	or d			;4ae9
	ret z			;4aea
	defb 0ddh,06bh	;ld ixl,e		;4aeb
	defb 0ddh,062h	;ld ixh,d		;4aed
	ld l,(ix+000h)		;4aef
	ld h,(ix+001h)		;4af2
	defb 0fdh,05dh	;ld e,iyl		;4af5
	defb 0fdh,054h	;ld d,iyh		;4af7
	and a			;4af9
	sbc hl,de		;4afa
	jp z,Ch_KeyOn_L4B01		;4afc
	and a			;4aff
	ret			;4b00
Ch_KeyOn_L4B01:
	scf			;4b01
	ret			;4b02
Ch_IsFM_4B03:
	ld a,(ix+002h)		;4b03
	xor 001h		;4b06
	ret nz			;4b08
	scf			;4b09
	ret			;4b0a
Ch_SetNote_4B0B:
	call Ch_KeyOn_4AE2		;4b0b
	ret nc			;4b0e
	call Ch_IsFM_4B03		;4b0f
	jr nc,Ch_SetNote_L4B19		;4b12
	call FM_OpVolumeUp_3672		;4b14
	scf			;4b17
	ret			;4b18
Ch_SetNote_L4B19:
	ld a,(iy+012h)		;4b19
	add a,0feh		;4b1c
	ld b,a			;4b1e
	sbc a,a			;4b1f
	and b			;4b20
	ld (iy+012h),a		;4b21
	scf			;4b24
	ret			;4b25
Ch_SetNote2_4B26:
	call Ch_KeyOn_4AE2		;4b26
	ret nc			;4b29
	call Ch_IsFM_4B03		;4b2a
	jr nc,Ch_SetNote2_L4B34		;4b2d
	call FM_OpVolumeDown_36AA		;4b2f
	scf			;4b32
	ret			;4b33
Ch_SetNote2_L4B34:
	ld a,(iy+012h)		;4b34
	add a,002h		;4b37
	ld b,a			;4b39
	sbc a,a			;4b3a
	or b			;4b3b
	ld (iy+012h),a		;4b3c
	scf			;4b3f
	ret			;4b40
Ch_SetNote3_4B41:
	call Ch_KeyOn_4AE2		;4b41
	ret nc			;4b44
	call Ch_IsFM_4B03		;4b45
	ccf			;4b48
	ret c			;4b49
	call FM_OpParam3_36E1		;4b4a
	scf			;4b4d
	ret			;4b4e
Ch_SetNote4_4B4F:
	call Ch_KeyOn_4AE2		;4b4f
	ret nc			;4b52
	call Ch_IsFM_4B03		;4b53
	ccf			;4b56
	ret c			;4b57
	call FM_OpParam4_3711		;4b58
	scf			;4b5b
	ret			;4b5c
Instr_ResolvePtr_4B5D:
	ld l,(iy+00ah)		;4b5d
	ld h,000h		;4b60
	add hl,hl			;4b62
	add hl,hl			;4b63
	ex de,hl			;4b64
	ld h,(iy+00bh)		;4b65
	ld a,h			;4b68
	inc a			;4b69
	jr nz,Instr_ResolvePtr_L4B72		;4b6a
	ld hl,InstrDefault_2509		;4b6c
	jp Instr_ResolvePtr_L4B7F		;4b6f
Instr_ResolvePtr_L4B72:
	ld l,000h		;4b72
	add hl,hl			;4b74
	add hl,de			;4b75
	ex de,hl			;4b76
	push de			;4b77
	ld hl,BankPtrTable_0080		;4b78
	call Bank_ResolvePtr24_1FA6		;4b7b
	pop de			;4b7e
Instr_ResolvePtr_L4B7F:
	add hl,de			;4b7f
	call Bank_ResolvePtr24_1FA6		;4b80
	ret			;4b83
