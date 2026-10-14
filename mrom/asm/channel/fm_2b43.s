; fm_2b43.s - M-ROM fixed_0000 [0x2b43..0x3740)
; Carved from mrom/asm/raw/fixed_0000.s by mrom/tools/split_region.py.
; Assembled through mrom/asm/fixed_0000.s; must stay byte-identical.

FM_Init_2B43:
	ld de,02100h		;2b43
	ld c,004h		;2b46
	rst 8			;2b48
	ld de,02801h		;2b49
	rst 8			;2b4c
	ld de,02802h		;2b4d
	rst 8			;2b50
	ld de,02805h		;2b51
	rst 8			;2b54
	ld de,02806h		;2b55
	rst 8			;2b58
l2b59h:
	ld de,0417fh		;2b59  YM reg 41h <- 7Fh
	rst 8			;2b5c
	ld de,0427fh		;2b5d  YM reg 42h <- 7Fh
	rst 8			;2b60
	ld de,0457fh		;2b61  YM reg 45h <- 7Fh
	rst 8			;2b64
	ld de,0467fh		;2b65  YM reg 46h <- 7Fh
	rst 8			;2b68
	ld de,0497fh		;2b69  YM reg 49h <- 7Fh
	rst 8			;2b6c
	ld de,04a7fh		;2b6d
	rst 8			;2b70
	ld de,04d7fh		;2b71  YM reg 4Dh <- 7Fh
	rst 8			;2b74
	ld de,04e7fh		;2b75  YM reg 4Eh <- 7Fh
	rst 8			;2b78
	ld a,c			;2b79
	cp 004h		;2b7a
	jr nz,l2b83h		;2b7c
	ld c,006h		;2b7e
	jp l2b59h		;2b80
l2b83h:
	ld ix,wFM_Ch1_F907		;2b83
	ld (ix+012h),001h		;2b87
	call FM_InitChannel_2BB7		;2b8b
	ld ix,wFM_Ch2_F96E		;2b8e
	ld (ix+012h),002h		;2b92
	call FM_InitChannel_2BB7		;2b96
	ld ix,wFM_Ch3_F9D5		;2b99
	ld (ix+012h),005h		;2b9d
	call FM_InitChannel_2BB7		;2ba1
	ld ix,wFM_Ch4_FA3C		;2ba4
	ld (ix+012h),006h		;2ba8
	call FM_InitChannel_2BB7		;2bac
	xor a			;2baf
	ld (wFM_Shared_FAA3),a		;2bb0
	ld (0faa4h),a		;2bb3
	ret			;2bb6
FM_InitChannel_2BB7:
	call Ch_InitStruct_1FE5		;2bb7
	ld (ix+002h),001h		;2bba
	xor a			;2bbe
	ld (ix+01bh),a		;2bbf
	ld (ix+024h),a		;2bc2
	ld (ix+02dh),a		;2bc5
	ld (ix+036h),a		;2bc8
	ld (ix+03ah),a		;2bcb
	ld (ix+03bh),a		;2bce
	ld (ix+03ch),a		;2bd1
	ld (ix+03dh),a		;2bd4
	dec a			;2bd7
	ld (ix+038h),a		;2bd8
	ld (ix+039h),a		;2bdb
	push ix		;2bde
	ld de,0004ch		;2be0  constant
	add ix,de		;2be3
	call Lfo_Clear_229D		;2be5
	pop ix		;2be8
	push ix		;2bea
	ld de,0003eh		;2bec  constant
	add ix,de		;2bef
	call Lfo_Clear_229D		;2bf1
	pop ix		;2bf4
	push ix		;2bf6
	ld de,0005ah		;2bf8  constant
	add ix,de		;2bfb
	call Pitch_Clear_23EE		;2bfd
	pop ix		;2c00
	ret			;2c02
FM_WriteOpRegs_2C03:
	ld b,a			;2c03
	ld e,(hl)			;2c04
	srl b		;2c05
	call c,YM_Write_RST8_0008		;2c07
	ld a,b			;2c0a
	and a			;2c0b
	ret z			;2c0c
	ld a,d			;2c0d
	add a,010h		;2c0e
	ld d,a			;2c10
	inc hl			;2c11
	ld e,(hl)			;2c12
	srl b		;2c13
	call c,YM_Write_RST8_0008		;2c15
	ld a,b			;2c18
	and a			;2c19
	ret z			;2c1a
	ld a,d			;2c1b
	add a,010h		;2c1c
	ld d,a			;2c1e
	inc hl			;2c1f
	ld e,(hl)			;2c20
	srl b		;2c21
	call c,YM_Write_RST8_0008		;2c23
	ld a,b			;2c26
	and a			;2c27
	ret z			;2c28
	ld a,d			;2c29
	add a,010h		;2c2a
	ld d,a			;2c2c
	inc hl			;2c2d
	ld e,(hl)			;2c2e
	srl b		;2c2f
	call c,YM_Write_RST8_0008		;2c31
	ld a,b			;2c34
	and a			;2c35
	ret z			;2c36
	ld a,d			;2c37
	add a,010h		;2c38
	ld d,a			;2c3a
	inc hl			;2c3b
	ld e,(hl)			;2c3c
	srl b		;2c3d
	call c,YM_Write_RST8_0008		;2c3f
	ld a,b			;2c42
	and a			;2c43
	ret z			;2c44
	ld a,d			;2c45
	add a,010h		;2c46
	ld d,a			;2c48
	inc hl			;2c49
	ld e,(hl)			;2c4a
	srl b		;2c4b
	call c,YM_Write_RST8_0008		;2c4d
	ld a,b			;2c50
	and a			;2c51
	ret z			;2c52
	ld a,d			;2c53
	add a,010h		;2c54
	ld d,a			;2c56
	inc hl			;2c57
	ld e,(hl)			;2c58
	srl b		;2c59
	jp c,YM_Write_RST8_0008		;2c5b
	ret			;2c5e
FM_Update_2C5F:
	ld hl,wFM_Shared_FAA3		;2c5f
	ld e,(hl)			;2c62
	inc hl			;2c63
	ld a,(hl)			;2c64
	ld (hl),e			;2c65
	cp e			;2c66
	jp z,l2c6fh		;2c67
	ld d,022h		;2c6a
	ld c,004h		;2c6c
	rst 8			;2c6e
l2c6fh:
	ld ix,wFM_Ch1_F907		;2c6f
	ld iy,(wFM_Ch1_F907)		;2c73
	bit 3,(iy+009h)		;2c77
	call nz,Ch_ResetStruct_201B		;2c7b
	ld a,(0f93fh)		;2c7e
	and a			;2c81
	jp p,l2c95h		;2c82
	ld hl,0f942h		;2c85
	xor a			;2c88
	ld (hl),a			;2c89
	ld e,a			;2c8a
	ld d,0b5h		;2c8b
	ld c,004h		;2c8d
	rst 8			;2c8f
	ld a,0ffh		;2c90
	ld (0f917h),a		;2c92
l2c95h:
	ld a,(0f918h)		;2c95
	ld hl,(0f910h)		;2c98
	add a,h			;2c9b
	ld h,a			;2c9c
	ld e,(iy+010h)		;2c9d
	ld d,(iy+011h)		;2ca0
	add hl,de			;2ca3
	ld (wTmpWord_F812),hl		;2ca4
	ld hl,0f953h		;2ca7
	ld d,(iy+00ch)		;2caa
	call Env_Update_231E		;2cad
	ld l,a			;2cb0
	ld h,000h		;2cb1
	ld b,h			;2cb3
	add hl,hl			;2cb4
	and a			;2cb5
	sbc hl,bc		;2cb6
	ld de,(wTmpWord_F812)		;2cb8
	add hl,de			;2cbc
	ld (wTmpWord_F812),hl		;2cbd
	ld hl,0f961h		;2cc0
	call Vib_Update_248E		;2cc3
	ld de,(wTmpWord_F812)		;2cc6
	add hl,de			;2cca
	ld de,(0f915h)		;2ccb
	and a			;2ccf
	ex de,hl			;2cd0
	sbc hl,de		;2cd1
	jp z,l2d05h		;2cd3
	ld (0f915h),de		;2cd6
	ex de,hl			;2cda
	ld a,h			;2cdb
	and a			;2cdc
	jp p,l2ce2h		;2cdd
	xor a			;2ce0
	ld l,a			;2ce1
l2ce2h:
	ld h,012h		;2ce2
	ld d,(hl)			;2ce4
	inc h			;2ce5
	ld e,(hl)			;2ce6
	ex de,hl			;2ce7
	cp 008h		;2ce8
	jr c,l2cf7h		;2cea
	jr z,l2cf4h		;2cec
	ld hl,0ffffh		;2cee
	jp l2cf7h		;2cf1
l2cf4h:
	ld a,007h		;2cf4
	add hl,hl			;2cf6
l2cf7h:
	add a,a			;2cf7
	add a,a			;2cf8
	add a,a			;2cf9
	or h			;2cfa
	ld e,a			;2cfb
	ld c,004h		;2cfc
	ld d,0a5h		;2cfe
	rst 8			;2d00
	ld e,l			;2d01
	ld d,0a1h		;2d02
	rst 8			;2d04
l2d05h:
	ld hl,0f945h		;2d05
	ld d,(iy+00dh)		;2d08
	call Env_Update_231E		;2d0b
	sub c			;2d0e
	dec a			;2d0f
	ld b,a			;2d10
	ld a,(0f93fh)		;2d11
	and a			;2d14
	jp p,l2d1ah		;2d15
	ld b,000h		;2d18
l2d1ah:
	ld a,b			;2d1a
	ld l,a			;2d1b
	ld a,(0f912h)		;2d1c
	and a			;2d1f
	jr z,l2d37h		;2d20
	inc l			;2d22
	jr z,l2d37h		;2d23
	ex af,af'			;2d25
	dec l			;2d26
	ld a,l			;2d27
	jr z,l2d37h		;2d28
	ld h,014h		;2d2a
	ld a,(hl)			;2d2c
	ex af,af'			;2d2d
	ld l,a			;2d2e
	ex af,af'			;2d2f
	add a,(hl)			;2d30
	ld l,a			;2d31
	sbc a,a			;2d32
	jr nc,l2d37h		;2d33
	inc h			;2d35
	ld a,(hl)			;2d36
l2d37h:
	ld l,(iy+012h)		;2d37
	and a			;2d3a
	jr z,l2d52h		;2d3b
	inc l			;2d3d
	jr z,l2d52h		;2d3e
	ex af,af'			;2d40
	dec l			;2d41
	ld a,l			;2d42
	jr z,l2d52h		;2d43
	ld h,014h		;2d45
	ld a,(hl)			;2d47
	ex af,af'			;2d48
	ld l,a			;2d49
	ex af,af'			;2d4a
	add a,(hl)			;2d4b
	ld l,a			;2d4c
	sbc a,a			;2d4d
	jr nc,l2d52h		;2d4e
	inc h			;2d50
	ld a,(hl)			;2d51
l2d52h:
	ld l,a			;2d52
	ld a,0ffh		;2d53
	bit 1,(iy+009h)		;2d55
	jp z,l2d5fh		;2d59
	ld a,(wMasterVolume_F805)		;2d5c
l2d5fh:
	and a			;2d5f
	jr z,l2d77h		;2d60
	inc l			;2d62
	jr z,l2d77h		;2d63
	ex af,af'			;2d65
	dec l			;2d66
	ld a,l			;2d67
	jr z,l2d77h		;2d68
	ld h,014h		;2d6a
	ld a,(hl)			;2d6c
	ex af,af'			;2d6d
	ld l,a			;2d6e
	ex af,af'			;2d6f
	add a,(hl)			;2d70
	ld l,a			;2d71
	sbc a,a			;2d72
	jr nc,l2d77h		;2d73
	inc h			;2d75
	ld a,(hl)			;2d76
l2d77h:
	ld hl,0f917h		;2d77
	cp (hl)			;2d7a
	ld (hl),a			;2d7b
	jp z,l2de6h		;2d7c
	ld l,a			;2d7f
	set 7,l		;2d80
	ld h,018h		;2d82
	ld b,(hl)			;2d84
	ld a,(ix+038h)		;2d85
	and 007h		;2d88
	cp 004h		;2d8a
	jr c,l2dd2h		;2d8c
	cp 005h		;2d8e
	jr c,l2dbeh		;2d90
	cp 007h		;2d92
	jr c,l2daah		;2d94
	ld a,(0f923h)		;2d96
	add a,b			;2d99
	ld c,a			;2d9a
	rla			;2d9b
	sbc a,a			;2d9c
	or c			;2d9d
	ld hl,0f91ch		;2d9e
	cp (hl)			;2da1
	jr z,l2daah		;2da2
	ld (hl),a			;2da4
	ld hl,0f922h		;2da5
	set 1,(hl)		;2da8
l2daah:
	ld a,(0f935h)		;2daa
	add a,b			;2dad
	ld c,a			;2dae
	rla			;2daf
	sbc a,a			;2db0
	or c			;2db1
	ld hl,0f92eh		;2db2
	cp (hl)			;2db5
	jr z,l2dbeh		;2db6
	ld (hl),a			;2db8
	ld hl,0f934h		;2db9
	set 1,(hl)		;2dbc
l2dbeh:
	ld a,(0f92ch)		;2dbe
	add a,b			;2dc1
	ld c,a			;2dc2
	rla			;2dc3
	sbc a,a			;2dc4
	or c			;2dc5
	ld hl,0f925h		;2dc6
	cp (hl)			;2dc9
	jr z,l2dd2h		;2dca
	ld (hl),a			;2dcc
	ld hl,0f92bh		;2dcd
	set 1,(hl)		;2dd0
l2dd2h:
	ld a,(0f93eh)		;2dd2
	add a,b			;2dd5
	ld c,a			;2dd6
	rla			;2dd7
	sbc a,a			;2dd8
	or c			;2dd9
	ld hl,0f937h		;2dda
	cp (hl)			;2ddd
	jr z,l2de6h		;2dde
	ld (hl),a			;2de0
	ld hl,0f93dh		;2de1
	set 1,(hl)		;2de4
l2de6h:
	ld c,004h		;2de6
	ld a,(0f922h)		;2de8
	and a			;2deb
	jr z,l2dfah		;2dec
	ld hl,0f91bh		;2dee
	ld d,031h		;2df1
	call FM_WriteOpRegs_2C03		;2df3
	xor a			;2df6
	ld (0f922h),a		;2df7
l2dfah:
	ld a,(0f92bh)		;2dfa
	and a			;2dfd
	jr z,l2e0ch		;2dfe
	ld hl,0f924h		;2e00
	ld d,039h		;2e03
	call FM_WriteOpRegs_2C03		;2e05
	xor a			;2e08
	ld (0f92bh),a		;2e09
l2e0ch:
	ld a,(0f934h)		;2e0c
	and a			;2e0f
	jr z,l2e1eh		;2e10
	ld hl,0f92dh		;2e12
	ld d,035h		;2e15
	call FM_WriteOpRegs_2C03		;2e17
	xor a			;2e1a
	ld (0f934h),a		;2e1b
l2e1eh:
	ld a,(0f93dh)		;2e1e
	and a			;2e21
	jr z,l2e30h		;2e22
	ld hl,0f936h		;2e24
	ld d,03dh		;2e27
	call FM_WriteOpRegs_2C03		;2e29
	xor a			;2e2c
	ld (0f93dh),a		;2e2d
l2e30h:
	ld hl,0f93fh		;2e30
	ld a,(hl)			;2e33
	and 07fh		;2e34
	ld (hl),a			;2e36
	inc hl			;2e37
	cp (hl)			;2e38
	jp z,l2e41h		;2e39
	ld (hl),a			;2e3c
	ld e,a			;2e3d
	ld d,0b1h		;2e3e
	rst 8			;2e40
l2e41h:
	inc hl			;2e41
	ld a,(hl)			;2e42
	ld b,a			;2e43
	and 0c0h		;2e44
	ld a,b			;2e46
	jr nz,l2e52h		;2e47
	ld b,a			;2e49
	ld a,(iy+00fh)		;2e4a
	and 003h		;2e4d
	rrca			;2e4f
	rrca			;2e50
	or b			;2e51
l2e52h:
	inc hl			;2e52
	cp (hl)			;2e53
	jr z,l2e5dh		;2e54
	ld (hl),a			;2e56
	ld e,a			;2e57
	ld d,0b5h		;2e58
	ld c,004h		;2e5a
	rst 8			;2e5c
l2e5dh:
	inc hl			;2e5d
	ld a,(hl)			;2e5e
	inc hl			;2e5f
	cp (hl)			;2e60
	jp z,l2e6bh		;2e61
	ld (hl),a			;2e64
	ld c,004h		;2e65
	ld d,028h		;2e67
	ld e,a			;2e69
	rst 8			;2e6a
l2e6bh:
	ld c,001h		;2e6b
	bit 1,(iy+009h)		;2e6d
	jr z,l2e77h		;2e71
	ld a,(wTempoStep_F809)		;2e73
	ld c,a			;2e76
l2e77h:
	ld hl,0f90dh		;2e77
	ld a,(hl)			;2e7a
	add a,c			;2e7b
	sbc a,000h		;2e7c
	ld (hl),a			;2e7e
	ld hl,0f90ch		;2e7f
	ld a,(hl)			;2e82
	and a			;2e83
	jp z,l2e98h		;2e84
	sub c			;2e87
	ld (hl),a			;2e88
	jr nz,l2eadh		;2e89
	bit 4,(iy+009h)		;2e8b
	jr z,l2ea7h		;2e8f
	set 4,(ix+003h)		;2e91
	jp l2eadh		;2e95
l2e98h:
	bit 4,(iy+009h)		;2e98
	jr nz,l2eadh		;2e9c
	ld hl,0f90ah		;2e9e
	bit 4,(hl)		;2ea1
	jr z,l2eadh		;2ea3
	res 4,(hl)		;2ea5
l2ea7h:
	ld a,(0f919h)		;2ea7
	ld (0f943h),a		;2eaa
l2eadh:
	ld ix,wFM_Ch2_F96E		;2ead
	ld iy,(wFM_Ch2_F96E)		;2eb1
	bit 3,(iy+009h)		;2eb5
	call nz,Ch_ResetStruct_201B		;2eb9
	ld a,(0f9a6h)		;2ebc
	and a			;2ebf
	jp p,l2ed3h		;2ec0
	ld hl,0f9a9h		;2ec3
	xor a			;2ec6
	ld (hl),a			;2ec7
	ld e,a			;2ec8
	ld d,0b6h		;2ec9
	ld c,004h		;2ecb
	rst 8			;2ecd
	ld a,0ffh		;2ece
	ld (0f97eh),a		;2ed0
l2ed3h:
	ld a,(0f97fh)		;2ed3
	ld hl,(0f977h)		;2ed6
	add a,h			;2ed9
	ld h,a			;2eda
	ld e,(iy+010h)		;2edb
	ld d,(iy+011h)		;2ede
	add hl,de			;2ee1
	ld (wTmpWord_F812),hl		;2ee2
	ld hl,0f9bah		;2ee5
	ld d,(iy+00ch)		;2ee8
	call Env_Update_231E		;2eeb
	ld l,a			;2eee
	ld h,000h		;2eef
	ld b,h			;2ef1
	add hl,hl			;2ef2
	and a			;2ef3
	sbc hl,bc		;2ef4
	ld de,(wTmpWord_F812)		;2ef6
	add hl,de			;2efa
	ld (wTmpWord_F812),hl		;2efb
	ld hl,0f9c8h		;2efe
	call Vib_Update_248E		;2f01
	ld de,(wTmpWord_F812)		;2f04
	add hl,de			;2f08
	ld de,(0f97ch)		;2f09
	and a			;2f0d
	ex de,hl			;2f0e
	sbc hl,de		;2f0f
	jp z,l2f43h		;2f11
	ld (0f97ch),de		;2f14
	ex de,hl			;2f18
	ld a,h			;2f19
	and a			;2f1a
	jp p,l2f20h		;2f1b
	xor a			;2f1e
	ld l,a			;2f1f
l2f20h:
	ld h,012h		;2f20
	ld d,(hl)			;2f22
	inc h			;2f23
	ld e,(hl)			;2f24
	ex de,hl			;2f25
	cp 008h		;2f26
	jr c,l2f35h		;2f28
	jr z,l2f32h		;2f2a
	ld hl,0ffffh		;2f2c
	jp l2f35h		;2f2f
l2f32h:
	ld a,007h		;2f32
	add hl,hl			;2f34
l2f35h:
	add a,a			;2f35
	add a,a			;2f36
	add a,a			;2f37
	or h			;2f38
	ld e,a			;2f39
	ld c,004h		;2f3a
	ld d,0a6h		;2f3c
	rst 8			;2f3e
	ld e,l			;2f3f
	ld d,0a2h		;2f40
	rst 8			;2f42
l2f43h:
	ld hl,0f9ach		;2f43
	ld d,(iy+00dh)		;2f46
	call Env_Update_231E		;2f49
	sub c			;2f4c
	dec a			;2f4d
	ld b,a			;2f4e
	ld a,(0f9a6h)		;2f4f
	and a			;2f52
	jp p,l2f58h		;2f53
	ld b,000h		;2f56
l2f58h:
	ld a,b			;2f58
	ld l,a			;2f59
	ld a,(0f979h)		;2f5a
	and a			;2f5d
	jr z,l2f75h		;2f5e
	inc l			;2f60
	jr z,l2f75h		;2f61
	ex af,af'			;2f63
	dec l			;2f64
	ld a,l			;2f65
	jr z,l2f75h		;2f66
	ld h,014h		;2f68
	ld a,(hl)			;2f6a
	ex af,af'			;2f6b
	ld l,a			;2f6c
	ex af,af'			;2f6d
	add a,(hl)			;2f6e
	ld l,a			;2f6f
	sbc a,a			;2f70
	jr nc,l2f75h		;2f71
	inc h			;2f73
	ld a,(hl)			;2f74
l2f75h:
	ld l,(iy+012h)		;2f75
	and a			;2f78
	jr z,l2f90h		;2f79
	inc l			;2f7b
	jr z,l2f90h		;2f7c
	ex af,af'			;2f7e
	dec l			;2f7f
	ld a,l			;2f80
	jr z,l2f90h		;2f81
	ld h,014h		;2f83
	ld a,(hl)			;2f85
	ex af,af'			;2f86
	ld l,a			;2f87
	ex af,af'			;2f88
	add a,(hl)			;2f89
	ld l,a			;2f8a
	sbc a,a			;2f8b
	jr nc,l2f90h		;2f8c
	inc h			;2f8e
	ld a,(hl)			;2f8f
l2f90h:
	ld l,a			;2f90
	ld a,0ffh		;2f91
	bit 1,(iy+009h)		;2f93
	jp z,l2f9dh		;2f97
	ld a,(wMasterVolume_F805)		;2f9a
l2f9dh:
	and a			;2f9d
	jr z,l2fb5h		;2f9e
	inc l			;2fa0
	jr z,l2fb5h		;2fa1
	ex af,af'			;2fa3
	dec l			;2fa4
	ld a,l			;2fa5
	jr z,l2fb5h		;2fa6
	ld h,014h		;2fa8
	ld a,(hl)			;2faa
	ex af,af'			;2fab
	ld l,a			;2fac
	ex af,af'			;2fad
	add a,(hl)			;2fae
	ld l,a			;2faf
	sbc a,a			;2fb0
	jr nc,l2fb5h		;2fb1
	inc h			;2fb3
	ld a,(hl)			;2fb4
l2fb5h:
	ld hl,0f97eh		;2fb5
	cp (hl)			;2fb8
	ld (hl),a			;2fb9
	jp z,l3024h		;2fba
	ld l,a			;2fbd
	set 7,l		;2fbe
	ld h,018h		;2fc0
	ld b,(hl)			;2fc2
	ld a,(ix+038h)		;2fc3
	and 007h		;2fc6
	cp 004h		;2fc8
	jr c,l3010h		;2fca
	cp 005h		;2fcc
	jr c,l2ffch		;2fce
	cp 007h		;2fd0
	jr c,l2fe8h		;2fd2
	ld a,(0f98ah)		;2fd4
	add a,b			;2fd7
	ld c,a			;2fd8
	rla			;2fd9
	sbc a,a			;2fda
	or c			;2fdb
	ld hl,0f983h		;2fdc
	cp (hl)			;2fdf
	jr z,l2fe8h		;2fe0
	ld (hl),a			;2fe2
	ld hl,0f989h		;2fe3
	set 1,(hl)		;2fe6
l2fe8h:
	ld a,(0f99ch)		;2fe8
	add a,b			;2feb
	ld c,a			;2fec
	rla			;2fed
	sbc a,a			;2fee
	or c			;2fef
	ld hl,0f995h		;2ff0
	cp (hl)			;2ff3
	jr z,l2ffch		;2ff4
	ld (hl),a			;2ff6
	ld hl,0f99bh		;2ff7
	set 1,(hl)		;2ffa
l2ffch:
	ld a,(0f993h)		;2ffc
	add a,b			;2fff
	ld c,a			;3000
	rla			;3001
	sbc a,a			;3002
	or c			;3003
	ld hl,0f98ch		;3004
	cp (hl)			;3007
	jr z,l3010h		;3008
	ld (hl),a			;300a
	ld hl,0f992h		;300b
	set 1,(hl)		;300e
l3010h:
	ld a,(0f9a5h)		;3010
	add a,b			;3013
	ld c,a			;3014
	rla			;3015
	sbc a,a			;3016
	or c			;3017
	ld hl,0f99eh		;3018
	cp (hl)			;301b
	jr z,l3024h		;301c
	ld (hl),a			;301e
	ld hl,0f9a4h		;301f
	set 1,(hl)		;3022
l3024h:
	ld c,004h		;3024
	ld a,(0f989h)		;3026
	and a			;3029
	jr z,l3038h		;302a
	ld hl,0f982h		;302c
	ld d,032h		;302f
	call FM_WriteOpRegs_2C03		;3031
	xor a			;3034
	ld (0f989h),a		;3035
l3038h:
	ld a,(0f992h)		;3038
	and a			;303b
	jr z,l304ah		;303c
	ld hl,0f98bh		;303e
	ld d,03ah		;3041
	call FM_WriteOpRegs_2C03		;3043
	xor a			;3046
	ld (0f992h),a		;3047
l304ah:
	ld a,(0f99bh)		;304a
	and a			;304d
	jr z,l305ch		;304e
	ld hl,0f994h		;3050
	ld d,036h		;3053
	call FM_WriteOpRegs_2C03		;3055
	xor a			;3058
	ld (0f99bh),a		;3059
l305ch:
	ld a,(0f9a4h)		;305c
	and a			;305f
	jr z,l306eh		;3060
	ld hl,0f99dh		;3062
	ld d,03eh		;3065
	call FM_WriteOpRegs_2C03		;3067
	xor a			;306a
	ld (0f9a4h),a		;306b
l306eh:
	ld hl,0f9a6h		;306e
	ld a,(hl)			;3071
	and 07fh		;3072
	ld (hl),a			;3074
	inc hl			;3075
	cp (hl)			;3076
	jp z,l307fh		;3077
	ld (hl),a			;307a
	ld e,a			;307b
	ld d,0b2h		;307c
	rst 8			;307e
l307fh:
	inc hl			;307f
	ld a,(hl)			;3080
	ld b,a			;3081
	and 0c0h		;3082
	ld a,b			;3084
	jr nz,l3090h		;3085
	ld b,a			;3087
	ld a,(iy+00fh)		;3088
	and 003h		;308b
	rrca			;308d
	rrca			;308e
	or b			;308f
l3090h:
	inc hl			;3090
	cp (hl)			;3091
	jr z,l309bh		;3092
	ld (hl),a			;3094
	ld e,a			;3095
	ld d,0b6h		;3096
	ld c,004h		;3098
	rst 8			;309a
l309bh:
	inc hl			;309b
	ld a,(hl)			;309c
	inc hl			;309d
	cp (hl)			;309e
	jp z,l30a9h		;309f
	ld (hl),a			;30a2
	ld c,004h		;30a3
	ld d,028h		;30a5
	ld e,a			;30a7
	rst 8			;30a8
l30a9h:
	ld c,001h		;30a9
	bit 1,(iy+009h)		;30ab
	jr z,l30b5h		;30af
	ld a,(wTempoStep_F809)		;30b1
	ld c,a			;30b4
l30b5h:
	ld hl,0f974h		;30b5
	ld a,(hl)			;30b8
	add a,c			;30b9
	sbc a,000h		;30ba
	ld (hl),a			;30bc
	ld hl,0f973h		;30bd
	ld a,(hl)			;30c0
	and a			;30c1
	jp z,l30d6h		;30c2
	sub c			;30c5
	ld (hl),a			;30c6
	jr nz,l30ebh		;30c7
	bit 4,(iy+009h)		;30c9
	jr z,l30e5h		;30cd
	set 4,(ix+003h)		;30cf
	jp l30ebh		;30d3
l30d6h:
	bit 4,(iy+009h)		;30d6
	jr nz,l30ebh		;30da
	ld hl,0f971h		;30dc
	bit 4,(hl)		;30df
	jr z,l30ebh		;30e1
	res 4,(hl)		;30e3
l30e5h:
	ld a,(0f980h)		;30e5
	ld (0f9aah),a		;30e8
l30ebh:
	ld ix,wFM_Ch3_F9D5		;30eb
	ld iy,(wFM_Ch3_F9D5)		;30ef
	bit 3,(iy+009h)		;30f3
	call nz,Ch_ResetStruct_201B		;30f7
	ld a,(0fa0dh)		;30fa
	and a			;30fd
	jp p,l3111h		;30fe
	ld hl,0fa10h		;3101
	xor a			;3104
	ld (hl),a			;3105
	ld e,a			;3106
	ld d,0b5h		;3107
	ld c,006h		;3109
	rst 8			;310b
	ld a,0ffh		;310c
	ld (0f9e5h),a		;310e
l3111h:
	ld a,(0f9e6h)		;3111
	ld hl,(0f9deh)		;3114
	add a,h			;3117
	ld h,a			;3118
	ld e,(iy+010h)		;3119
	ld d,(iy+011h)		;311c
	add hl,de			;311f
	ld (wTmpWord_F812),hl		;3120
	ld hl,0fa21h		;3123
	ld d,(iy+00ch)		;3126
	call Env_Update_231E		;3129
	ld l,a			;312c
	ld h,000h		;312d
	ld b,h			;312f
	add hl,hl			;3130
	and a			;3131
	sbc hl,bc		;3132
	ld de,(wTmpWord_F812)		;3134
	add hl,de			;3138
	ld (wTmpWord_F812),hl		;3139
	ld hl,0fa2fh		;313c
	call Vib_Update_248E		;313f
	ld de,(wTmpWord_F812)		;3142
	add hl,de			;3146
	ld de,(0f9e3h)		;3147
	and a			;314b
	ex de,hl			;314c
	sbc hl,de		;314d
	jp z,l3181h		;314f
	ld (0f9e3h),de		;3152
	ex de,hl			;3156
	ld a,h			;3157
	and a			;3158
	jp p,l315eh		;3159
	xor a			;315c
	ld l,a			;315d
l315eh:
	ld h,012h		;315e
	ld d,(hl)			;3160
	inc h			;3161
	ld e,(hl)			;3162
	ex de,hl			;3163
	cp 008h		;3164
	jr c,l3173h		;3166
	jr z,l3170h		;3168
	ld hl,0ffffh		;316a
	jp l3173h		;316d
l3170h:
	ld a,007h		;3170
	add hl,hl			;3172
l3173h:
	add a,a			;3173
	add a,a			;3174
	add a,a			;3175
	or h			;3176
	ld e,a			;3177
	ld c,006h		;3178
	ld d,0a5h		;317a
	rst 8			;317c
	ld e,l			;317d
	ld d,0a1h		;317e
	rst 8			;3180
l3181h:
	ld hl,0fa13h		;3181
	ld d,(iy+00dh)		;3184
	call Env_Update_231E		;3187
	sub c			;318a
	dec a			;318b
	ld b,a			;318c
	ld a,(0fa0dh)		;318d
	and a			;3190
	jp p,l3196h		;3191
	ld b,000h		;3194
l3196h:
	ld a,b			;3196
	ld l,a			;3197
	ld a,(0f9e0h)		;3198
	and a			;319b
	jr z,l31b3h		;319c
	inc l			;319e
	jr z,l31b3h		;319f
	ex af,af'			;31a1
	dec l			;31a2
	ld a,l			;31a3
	jr z,l31b3h		;31a4
	ld h,014h		;31a6
	ld a,(hl)			;31a8
	ex af,af'			;31a9
	ld l,a			;31aa
	ex af,af'			;31ab
	add a,(hl)			;31ac
	ld l,a			;31ad
	sbc a,a			;31ae
	jr nc,l31b3h		;31af
	inc h			;31b1
	ld a,(hl)			;31b2
l31b3h:
	ld l,(iy+012h)		;31b3
	and a			;31b6
	jr z,l31ceh		;31b7
	inc l			;31b9
	jr z,l31ceh		;31ba
	ex af,af'			;31bc
	dec l			;31bd
	ld a,l			;31be
	jr z,l31ceh		;31bf
	ld h,014h		;31c1
	ld a,(hl)			;31c3
	ex af,af'			;31c4
	ld l,a			;31c5
	ex af,af'			;31c6
	add a,(hl)			;31c7
	ld l,a			;31c8
	sbc a,a			;31c9
	jr nc,l31ceh		;31ca
	inc h			;31cc
	ld a,(hl)			;31cd
l31ceh:
	ld l,a			;31ce
	ld a,0ffh		;31cf
	bit 1,(iy+009h)		;31d1
	jp z,l31dbh		;31d5
	ld a,(wMasterVolume_F805)		;31d8
l31dbh:
	and a			;31db
	jr z,l31f3h		;31dc
	inc l			;31de
	jr z,l31f3h		;31df
	ex af,af'			;31e1
	dec l			;31e2
	ld a,l			;31e3
	jr z,l31f3h		;31e4
	ld h,014h		;31e6
	ld a,(hl)			;31e8
	ex af,af'			;31e9
	ld l,a			;31ea
	ex af,af'			;31eb
	add a,(hl)			;31ec
	ld l,a			;31ed
	sbc a,a			;31ee
	jr nc,l31f3h		;31ef
	inc h			;31f1
	ld a,(hl)			;31f2
l31f3h:
	ld hl,0f9e5h		;31f3
	cp (hl)			;31f6
	ld (hl),a			;31f7
	jp z,l3262h		;31f8
	ld l,a			;31fb
	set 7,l		;31fc
	ld h,018h		;31fe
	ld b,(hl)			;3200
	ld a,(ix+038h)		;3201
	and 007h		;3204
	cp 004h		;3206
	jr c,l324eh		;3208
	cp 005h		;320a
	jr c,l323ah		;320c
	cp 007h		;320e
	jr c,l3226h		;3210
	ld a,(0f9f1h)		;3212
	add a,b			;3215
	ld c,a			;3216
	rla			;3217
	sbc a,a			;3218
	or c			;3219
	ld hl,0f9eah		;321a
	cp (hl)			;321d
	jr z,l3226h		;321e
	ld (hl),a			;3220
	ld hl,0f9f0h		;3221
	set 1,(hl)		;3224
l3226h:
	ld a,(0fa03h)		;3226
	add a,b			;3229
	ld c,a			;322a
	rla			;322b
	sbc a,a			;322c
	or c			;322d
	ld hl,0f9fch		;322e
	cp (hl)			;3231
	jr z,l323ah		;3232
	ld (hl),a			;3234
	ld hl,0fa02h		;3235
	set 1,(hl)		;3238
l323ah:
	ld a,(0f9fah)		;323a
	add a,b			;323d
	ld c,a			;323e
	rla			;323f
	sbc a,a			;3240
	or c			;3241
	ld hl,0f9f3h		;3242
	cp (hl)			;3245
	jr z,l324eh		;3246
	ld (hl),a			;3248
	ld hl,0f9f9h		;3249
	set 1,(hl)		;324c
l324eh:
	ld a,(0fa0ch)		;324e
	add a,b			;3251
	ld c,a			;3252
	rla			;3253
	sbc a,a			;3254
	or c			;3255
	ld hl,0fa05h		;3256
	cp (hl)			;3259
	jr z,l3262h		;325a
	ld (hl),a			;325c
	ld hl,0fa0bh		;325d
	set 1,(hl)		;3260
l3262h:
	ld c,006h		;3262
	ld a,(0f9f0h)		;3264
	and a			;3267
	jr z,l3276h		;3268
	ld hl,0f9e9h		;326a
	ld d,031h		;326d
	call FM_WriteOpRegs_2C03		;326f
	xor a			;3272
	ld (0f9f0h),a		;3273
l3276h:
	ld a,(0f9f9h)		;3276
	and a			;3279
	jr z,l3288h		;327a
	ld hl,0f9f2h		;327c
	ld d,039h		;327f
	call FM_WriteOpRegs_2C03		;3281
	xor a			;3284
	ld (0f9f9h),a		;3285
l3288h:
	ld a,(0fa02h)		;3288
	and a			;328b
	jr z,l329ah		;328c
	ld hl,0f9fbh		;328e
	ld d,035h		;3291
	call FM_WriteOpRegs_2C03		;3293
	xor a			;3296
	ld (0fa02h),a		;3297
l329ah:
	ld a,(0fa0bh)		;329a
	and a			;329d
	jr z,l32ach		;329e
	ld hl,0fa04h		;32a0
	ld d,03dh		;32a3
	call FM_WriteOpRegs_2C03		;32a5
	xor a			;32a8
	ld (0fa0bh),a		;32a9
l32ach:
	ld hl,0fa0dh		;32ac
	ld a,(hl)			;32af
	and 07fh		;32b0
	ld (hl),a			;32b2
	inc hl			;32b3
	cp (hl)			;32b4
	jp z,l32bdh		;32b5
	ld (hl),a			;32b8
	ld e,a			;32b9
	ld d,0b1h		;32ba
	rst 8			;32bc
l32bdh:
	inc hl			;32bd
	ld a,(hl)			;32be
	ld b,a			;32bf
	and 0c0h		;32c0
	ld a,b			;32c2
	jr nz,l32ceh		;32c3
	ld b,a			;32c5
	ld a,(iy+00fh)		;32c6
	and 003h		;32c9
	rrca			;32cb
	rrca			;32cc
	or b			;32cd
l32ceh:
	inc hl			;32ce
	cp (hl)			;32cf
	jr z,l32d9h		;32d0
	ld (hl),a			;32d2
	ld e,a			;32d3
	ld d,0b5h		;32d4
	ld c,006h		;32d6
	rst 8			;32d8
l32d9h:
	inc hl			;32d9
	ld a,(hl)			;32da
	inc hl			;32db
	cp (hl)			;32dc
	jp z,l32e7h		;32dd
	ld (hl),a			;32e0
	ld c,004h		;32e1
	ld d,028h		;32e3
	ld e,a			;32e5
	rst 8			;32e6
l32e7h:
	ld c,001h		;32e7
	bit 1,(iy+009h)		;32e9
	jr z,l32f3h		;32ed
	ld a,(wTempoStep_F809)		;32ef
	ld c,a			;32f2
l32f3h:
	ld hl,0f9dbh		;32f3
	ld a,(hl)			;32f6
	add a,c			;32f7
	sbc a,000h		;32f8
	ld (hl),a			;32fa
	ld hl,0f9dah		;32fb
	ld a,(hl)			;32fe
	and a			;32ff
	jp z,l3314h		;3300
	sub c			;3303
	ld (hl),a			;3304
	jr nz,l3329h		;3305
	bit 4,(iy+009h)		;3307
	jr z,l3323h		;330b
	set 4,(ix+003h)		;330d
	jp l3329h		;3311
l3314h:
	bit 4,(iy+009h)		;3314
	jr nz,l3329h		;3318
	ld hl,0f9d8h		;331a
	bit 4,(hl)		;331d
	jr z,l3329h		;331f
	res 4,(hl)		;3321
l3323h:
	ld a,(0f9e7h)		;3323
	ld (0fa11h),a		;3326
l3329h:
	ld ix,wFM_Ch4_FA3C		;3329
	ld iy,(wFM_Ch4_FA3C)		;332d
	bit 3,(iy+009h)		;3331
	call nz,Ch_ResetStruct_201B		;3335
	ld a,(0fa74h)		;3338
	and a			;333b
	jp p,l334fh		;333c
	ld hl,0fa77h		;333f
	xor a			;3342
	ld (hl),a			;3343
	ld e,a			;3344
	ld d,0b6h		;3345
	ld c,006h		;3347
	rst 8			;3349
	ld a,0ffh		;334a
	ld (0fa4ch),a		;334c
l334fh:
	ld a,(0fa4dh)		;334f
	ld hl,(0fa45h)		;3352
	add a,h			;3355
	ld h,a			;3356
	ld e,(iy+010h)		;3357
	ld d,(iy+011h)		;335a
	add hl,de			;335d
	ld (wTmpWord_F812),hl		;335e
	ld hl,0fa88h		;3361
	ld d,(iy+00ch)		;3364
	call Env_Update_231E		;3367
	ld l,a			;336a
	ld h,000h		;336b
	ld b,h			;336d
	add hl,hl			;336e
	and a			;336f
	sbc hl,bc		;3370
	ld de,(wTmpWord_F812)		;3372
	add hl,de			;3376
	ld (wTmpWord_F812),hl		;3377
	ld hl,0fa96h		;337a
	call Vib_Update_248E		;337d
	ld de,(wTmpWord_F812)		;3380
	add hl,de			;3384
	ld de,(0fa4ah)		;3385
	and a			;3389
	ex de,hl			;338a
	sbc hl,de		;338b
	jp z,l33bfh		;338d
	ld (0fa4ah),de		;3390
	ex de,hl			;3394
	ld a,h			;3395
	and a			;3396
	jp p,l339ch		;3397
	xor a			;339a
	ld l,a			;339b
l339ch:
	ld h,012h		;339c
	ld d,(hl)			;339e
	inc h			;339f
	ld e,(hl)			;33a0
	ex de,hl			;33a1
	cp 008h		;33a2
	jr c,l33b1h		;33a4
	jr z,l33aeh		;33a6
	ld hl,0ffffh		;33a8
	jp l33b1h		;33ab
l33aeh:
	ld a,007h		;33ae
	add hl,hl			;33b0
l33b1h:
	add a,a			;33b1
	add a,a			;33b2
	add a,a			;33b3
	or h			;33b4
	ld e,a			;33b5
	ld c,006h		;33b6
	ld d,0a6h		;33b8
	rst 8			;33ba
	ld e,l			;33bb
	ld d,0a2h		;33bc
	rst 8			;33be
l33bfh:
	ld hl,0fa7ah		;33bf
	ld d,(iy+00dh)		;33c2
	call Env_Update_231E		;33c5
	sub c			;33c8
	dec a			;33c9
	ld b,a			;33ca
	ld a,(0fa74h)		;33cb
	and a			;33ce
	jp p,l33d4h		;33cf
	ld b,000h		;33d2
l33d4h:
	ld a,b			;33d4
	ld l,a			;33d5
	ld a,(0fa47h)		;33d6
	and a			;33d9
	jr z,l33f1h		;33da
	inc l			;33dc
	jr z,l33f1h		;33dd
	ex af,af'			;33df
	dec l			;33e0
	ld a,l			;33e1
	jr z,l33f1h		;33e2
	ld h,014h		;33e4
	ld a,(hl)			;33e6
	ex af,af'			;33e7
	ld l,a			;33e8
	ex af,af'			;33e9
	add a,(hl)			;33ea
	ld l,a			;33eb
	sbc a,a			;33ec
	jr nc,l33f1h		;33ed
	inc h			;33ef
	ld a,(hl)			;33f0
l33f1h:
	ld l,(iy+012h)		;33f1
	and a			;33f4
	jr z,l340ch		;33f5
	inc l			;33f7
	jr z,l340ch		;33f8
	ex af,af'			;33fa
	dec l			;33fb
	ld a,l			;33fc
	jr z,l340ch		;33fd
	ld h,014h		;33ff
	ld a,(hl)			;3401
	ex af,af'			;3402
	ld l,a			;3403
	ex af,af'			;3404
	add a,(hl)			;3405
	ld l,a			;3406
	sbc a,a			;3407
	jr nc,l340ch		;3408
	inc h			;340a
	ld a,(hl)			;340b
l340ch:
	ld l,a			;340c
	ld a,0ffh		;340d
	bit 1,(iy+009h)		;340f
	jp z,l3419h		;3413
	ld a,(wMasterVolume_F805)		;3416
l3419h:
	and a			;3419
	jr z,l3431h		;341a
	inc l			;341c
	jr z,l3431h		;341d
	ex af,af'			;341f
	dec l			;3420
	ld a,l			;3421
	jr z,l3431h		;3422
	ld h,014h		;3424
	ld a,(hl)			;3426
	ex af,af'			;3427
	ld l,a			;3428
	ex af,af'			;3429
	add a,(hl)			;342a
	ld l,a			;342b
	sbc a,a			;342c
	jr nc,l3431h		;342d
	inc h			;342f
	ld a,(hl)			;3430
l3431h:
	ld hl,0fa4ch		;3431
	cp (hl)			;3434
	ld (hl),a			;3435
	jp z,l34a0h		;3436
	ld l,a			;3439
	set 7,l		;343a
	ld h,018h		;343c
	ld b,(hl)			;343e
	ld a,(ix+038h)		;343f
	and 007h		;3442
	cp 004h		;3444
	jr c,l348ch		;3446
	cp 005h		;3448
	jr c,l3478h		;344a
	cp 007h		;344c
	jr c,l3464h		;344e
	ld a,(0fa58h)		;3450
	add a,b			;3453
	ld c,a			;3454
	rla			;3455
	sbc a,a			;3456
	or c			;3457
	ld hl,0fa51h		;3458
	cp (hl)			;345b
	jr z,l3464h		;345c
	ld (hl),a			;345e
	ld hl,0fa57h		;345f
	set 1,(hl)		;3462
l3464h:
	ld a,(0fa6ah)		;3464
	add a,b			;3467
	ld c,a			;3468
	rla			;3469
	sbc a,a			;346a
	or c			;346b
	ld hl,0fa63h		;346c
	cp (hl)			;346f
	jr z,l3478h		;3470
	ld (hl),a			;3472
	ld hl,0fa69h		;3473
	set 1,(hl)		;3476
l3478h:
	ld a,(0fa61h)		;3478
	add a,b			;347b
	ld c,a			;347c
	rla			;347d
	sbc a,a			;347e
	or c			;347f
	ld hl,0fa5ah		;3480
	cp (hl)			;3483
	jr z,l348ch		;3484
	ld (hl),a			;3486
	ld hl,0fa60h		;3487
	set 1,(hl)		;348a
l348ch:
	ld a,(0fa73h)		;348c
	add a,b			;348f
	ld c,a			;3490
	rla			;3491
	sbc a,a			;3492
	or c			;3493
	ld hl,0fa6ch		;3494
	cp (hl)			;3497
	jr z,l34a0h		;3498
	ld (hl),a			;349a
	ld hl,0fa72h		;349b
	set 1,(hl)		;349e
l34a0h:
	ld c,006h		;34a0
	ld a,(0fa57h)		;34a2
	and a			;34a5
	jr z,l34b4h		;34a6
	ld hl,0fa50h		;34a8
	ld d,032h		;34ab
	call FM_WriteOpRegs_2C03		;34ad
	xor a			;34b0
	ld (0fa57h),a		;34b1
l34b4h:
	ld a,(0fa60h)		;34b4
	and a			;34b7
	jr z,l34c6h		;34b8
	ld hl,0fa59h		;34ba
	ld d,03ah		;34bd
	call FM_WriteOpRegs_2C03		;34bf
	xor a			;34c2
	ld (0fa60h),a		;34c3
l34c6h:
	ld a,(0fa69h)		;34c6
	and a			;34c9
	jr z,l34d8h		;34ca
	ld hl,0fa62h		;34cc
	ld d,036h		;34cf
	call FM_WriteOpRegs_2C03		;34d1
	xor a			;34d4
	ld (0fa69h),a		;34d5
l34d8h:
	ld a,(0fa72h)		;34d8
	and a			;34db
	jr z,l34eah		;34dc
	ld hl,0fa6bh		;34de
	ld d,03eh		;34e1
	call FM_WriteOpRegs_2C03		;34e3
	xor a			;34e6
	ld (0fa72h),a		;34e7
l34eah:
	ld hl,0fa74h		;34ea
	ld a,(hl)			;34ed
	and 07fh		;34ee
	ld (hl),a			;34f0
	inc hl			;34f1
	cp (hl)			;34f2
	jp z,l34fbh		;34f3
	ld (hl),a			;34f6
	ld e,a			;34f7
	ld d,0b2h		;34f8
	rst 8			;34fa
l34fbh:
	inc hl			;34fb
	ld a,(hl)			;34fc
	ld b,a			;34fd
	and 0c0h		;34fe
	ld a,b			;3500
	jr nz,l350ch		;3501
	ld b,a			;3503
	ld a,(iy+00fh)		;3504
	and 003h		;3507
	rrca			;3509
	rrca			;350a
	or b			;350b
l350ch:
	inc hl			;350c
	cp (hl)			;350d
	jr z,l3517h		;350e
	ld (hl),a			;3510
	ld e,a			;3511
	ld d,0b6h		;3512
	ld c,006h		;3514
	rst 8			;3516
l3517h:
	inc hl			;3517
	ld a,(hl)			;3518
	inc hl			;3519
	cp (hl)			;351a
	jp z,l3525h		;351b
	ld (hl),a			;351e
	ld c,004h		;351f
	ld d,028h		;3521
	ld e,a			;3523
	rst 8			;3524
l3525h:
	ld c,001h		;3525
	bit 1,(iy+009h)		;3527
	jr z,l3531h		;352b
	ld a,(wTempoStep_F809)		;352d
	ld c,a			;3530
l3531h:
	ld hl,0fa42h		;3531
	ld a,(hl)			;3534
	add a,c			;3535
	sbc a,000h		;3536
	ld (hl),a			;3538
	ld hl,0fa41h		;3539
	ld a,(hl)			;353c
	and a			;353d
	jp z,l3552h		;353e
	sub c			;3541
	ld (hl),a			;3542
	jr nz,l3567h		;3543
	bit 4,(iy+009h)		;3545
	jr z,l3561h		;3549
	set 4,(ix+003h)		;354b
	jp l3567h		;354f
l3552h:
	bit 4,(iy+009h)		;3552
	jr nz,l3567h		;3556
	ld hl,0fa3fh		;3558
	bit 4,(hl)		;355b
	jr z,l3567h		;355d
	res 4,(hl)		;355f
l3561h:
	ld a,(0fa4eh)		;3561
	ld (0fa78h),a		;3564
l3567h:
	ret			;3567
FM_LoadPatch_3568:
	exx			;3568
	push hl			;3569
	ex (sp),iy		;356a
	ld de,00006h		;356c  constant
	add hl,de			;356f
	defb 0ddh,05dh	;ld e,ixl		;3570
	defb 0ddh,054h	;ld d,ixh		;3572
	ex de,hl			;3574
	ld bc,00014h		;3575  constant
	add hl,bc			;3578
	ld b,004h		;3579
l357bh:
	push bc			;357b
	ex de,hl			;357c
	ld bc,00007h		;357d  constant
	ldir		;3580
	ex de,hl			;3582
	ld (hl),07fh		;3583
	inc hl			;3585
	inc hl			;3586
	pop bc			;3587
	djnz l357bh		;3588
	ld a,(ix+015h)		;358a
	ld (ix+01ch),a		;358d
	ld a,(ix+01eh)		;3590
	ld (ix+025h),a		;3593
	ld a,(ix+027h)		;3596
	ld (ix+02eh),a		;3599
	ld a,(ix+030h)		;359c
	ld (ix+037h),a		;359f
	ld a,(iy+001h)		;35a2
	and 0f0h		;35a5
	ld (ix+013h),a		;35a7
	ld a,(iy+002h)		;35aa
	or 080h		;35ad
	ld (ix+038h),a		;35af
	ld a,(iy+003h)		;35b2
	bit 3,a		;35b5
	jr z,l35c5h		;35b7
	ld (wFM_Shared_FAA3),a		;35b9
	ld a,(iy+004h)		;35bc
	ld (ix+03ah),a		;35bf
	jp l35cdh		;35c2
l35c5h:
	ld a,(iy+004h)		;35c5
	and 0c0h		;35c8
	ld (ix+03ah),a		;35ca
l35cdh:
	xor a			;35cd
	ld (ix+03bh),a		;35ce
	ld a,(iy+005h)		;35d1
	ld (ix+011h),a		;35d4
	push ix		;35d7
	ld de,00022h		;35d9  constant
	add iy,de		;35dc
	ld de,0003eh		;35de  constant
	add ix,de		;35e1
	call Lfo_Load_22A8		;35e3
	ld de,00006h		;35e6  constant
	add iy,de		;35e9
	ld de,0000eh		;35eb  constant
	add ix,de		;35ee
	call Lfo_Load_22A8		;35f0
	ld de,00006h		;35f3  constant
	add iy,de		;35f6
	ld de,0000eh		;35f8  constant
	add ix,de		;35fb
	call Pitch_Load_242E		;35fd
	pop ix		;3600
	pop iy		;3602
	ret			;3604
FM_SetPitch_3605:
	ld l,(ix+008h)		;3605
	ld h,011h		;3608
	ld c,(hl)			;360a
	set 7,l		;360b
	ld b,(hl)			;360d
	ld (ix+009h),c		;360e
	ld (ix+00ah),b		;3611
	ret			;3614
FM_KeyOn_3615:
	ld e,(ix+012h)		;3615
	ld a,e			;3618
	cp (ix+03dh)		;3619
	jr z,l3626h		;361c
	ld (ix+03dh),e		;361e
	ld d,028h		;3621
	ld c,004h		;3623
	rst 8			;3625
l3626h:
	ld a,(ix+013h)		;3626
	and 0f0h		;3629
	or e			;362b
	ld (ix+03ch),a		;362c
	bit 6,(ix+038h)		;362f
	jr z,l364ch		;3633
	ld a,(wFM_Shared_FAA3)		;3635
	or 008h		;3638
	and 00fh		;363a
	ld (wFM_Shared_FAA3),a		;363c
	and 007h		;363f
	ld e,a			;3641
	ld d,022h		;3642
	ld c,004h		;3644
	rst 8			;3646
	ld a,0ffh		;3647
	ld (0faa4h),a		;3649
l364ch:
	defb 0ddh,05dh	;ld e,ixl		;364c
	defb 0ddh,054h	;ld d,ixh		;364e
	ld hl,0003eh		;3650  constant
	add hl,de			;3653
	call Env_Init_22F4		;3654
	defb 0ddh,05dh	;ld e,ixl		;3657
	defb 0ddh,054h	;ld d,ixh		;3659
	ld hl,0004ch		;365b  constant
	add hl,de			;365e
	call Env_Init_22F4		;365f
	push ix		;3662
	ld de,0005ah		;3664  constant
	add ix,de		;3667
	ld c,(iy+00eh)		;3669
	call Pitch_Update_2461		;366c
	pop ix		;366f
	ret			;3671
FM_OpVolumeUp_3672:
	ld (ix+00ch),0ffh		;3672
	ld (ix+00dh),0ffh		;3676
	push ix		;367a
	ld de,00014h		;367c  constant
	add ix,de		;367f
	ld de,00009h		;3681  constant
	ld b,004h		;3684
l3686h:
	srl c		;3686
	jr nc,l369eh		;3688
	push bc			;368a
	ld a,(ix+008h)		;368b
	cp 07fh		;368e
	jr nc,l369dh		;3690
	inc a			;3692
	ld (ix+008h),a		;3693
	ld (ix+001h),a		;3696
	set 1,(ix+007h)		;3699
l369dh:
	pop bc			;369d
l369eh:
	add ix,de		;369e
	djnz l3686h		;36a0
	pop ix		;36a2
	ld a,0ffh		;36a4
	ld (0fa4ch),a		;36a6
	ret			;36a9
FM_OpVolumeDown_36AA:
	ld (ix+00ch),0ffh		;36aa
	ld (ix+00dh),0ffh		;36ae
	push ix		;36b2
	ld de,00014h		;36b4  constant
	add ix,de		;36b7
	ld de,00009h		;36b9  constant
	ld b,004h		;36bc
l36beh:
	srl c		;36be
	jr nc,l36d5h		;36c0
	push bc			;36c2
	ld a,(ix+008h)		;36c3
	and a			;36c6
	jr z,l36d4h		;36c7
	dec a			;36c9
	ld (ix+008h),a		;36ca
	ld (ix+001h),a		;36cd
	set 1,(ix+007h)		;36d0
l36d4h:
	pop bc			;36d4
l36d5h:
	add ix,de		;36d5
	djnz l36beh		;36d7
	pop ix		;36d9
	ld a,0ffh		;36db
	ld (0fa4ch),a		;36dd
	ret			;36e0
FM_OpParam3_36E1:
	ld (ix+00ch),0ffh		;36e1
	ld (ix+00dh),0ffh		;36e5
	push ix		;36e9
	ld de,00014h		;36eb  constant
	add ix,de		;36ee
	ld de,00009h		;36f0  constant
	ld b,004h		;36f3
l36f5h:
	srl c		;36f5
	jr nc,l370ah		;36f7
	push bc			;36f9
	ld a,(ix+000h)		;36fa
	cp 00fh		;36fd
	jr nc,l3709h		;36ff
	inc a			;3701
	ld (ix+000h),a		;3702
	set 0,(ix+007h)		;3705
l3709h:
	pop bc			;3709
l370ah:
	add ix,de		;370a
	djnz l36f5h		;370c
	pop ix		;370e
	ret			;3710
FM_OpParam4_3711:
	ld (ix+00ch),0ffh		;3711
	ld (ix+00dh),0ffh		;3715
	push ix		;3719
	ld de,00014h		;371b  constant
	add ix,de		;371e
	ld de,00009h		;3720  constant
	ld b,004h		;3723
l3725h:
	srl c		;3725
	jr nc,l3739h		;3727
	push bc			;3729
	ld a,(ix+000h)		;372a
	and a			;372d
	jr z,l3738h		;372e
	dec a			;3730
	ld (ix+000h),a		;3731
	set 0,(ix+007h)		;3734
l3738h:
	pop bc			;3738
l3739h:
	add ix,de		;3739
	djnz l3725h		;373b
	pop ix		;373d
	ret			;373f
