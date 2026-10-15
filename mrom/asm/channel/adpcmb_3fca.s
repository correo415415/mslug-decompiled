; adpcmb_3fca.s - M-ROM fixed_0000 [0x3fca..0x4aa0)
; Carved from mrom/asm/raw/fixed_0000.s by mrom/tools/split_region.py.
; Assembled through mrom/asm/fixed_0000.s; must stay byte-identical.

ADPCMB_Init_3FCA:
	ld ix,wPCMB_Ch_FB9B		;3fca
	call ADPCMB_InitChannel_3FE0		;3fce
	ld c,004h		;3fd1
	ld de,01b00h		;3fd3  YM reg 1Bh <- 00h
	rst 8			;3fd6
	ld de,01001h		;3fd7  YM reg 10h <- 01h
	rst 8			;3fda
	ld de,01100h		;3fdb  YM reg 11h <- 00h
	rst 8			;3fde
	ret			;3fdf
ADPCMB_InitChannel_3FE0:
	ld ix,wPCMB_Ch_FB9B		;3fe0
	call Ch_InitStruct_1FE5		;3fe4
	ld (ix+002h),003h		;3fe7
	xor a			;3feb
	ld (0fbbch),a		;3fec
	ld (0fbbdh),a		;3fef
	ld (wPCMB_Shared_FBBA),a		;3ff2
	push ix		;3ff5
	ld de,00023h		;3ff7  constant
	add ix,de		;3ffa
	rst 30h			;3ffc
	pop ix		;3ffd
	push ix		;3fff
	ld de,0003fh		;4001  constant
	add ix,de		;4004
	call Lfo_Clear_229D		;4006
	pop ix		;4009
	push ix		;400b
	ld de,00031h		;400d  constant
	add ix,de		;4010
	call Lfo_Clear_229D		;4012
	pop ix		;4015
	push ix		;4017
	ld de,0004dh		;4019  constant
	add ix,de		;401c
	call Pitch_Clear_23EE		;401e
	pop ix		;4021
	ret			;4023
ADPCMB_Update_4024:
	ld ix,wPCMB_Ch_FB9B		;4024
	ld iy,(wPCMB_Ch_FB9B)		;4028
	bit 3,(iy+009h)		;402c
	call nz,Ch_ResetStruct_201B		;4030
	ld a,(wPCMB_Shared_FBBA)		;4033
	and a			;4036
	jp z,ADPCMB_Update_L423C		;4037
	cp 004h		;403a
	jp z,ADPCMB_Update_L4054		;403c
	ex af,af'			;403f
	ld a,(wYmStatus_F80A)		;4040
	and 080h		;4043
	jp z,ADPCMB_Update_L410A		;4045
	ex af,af'			;4048
	dec a			;4049
	jp z,ADPCMB_Update_L40FA		;404a
	dec a			;404d
	jp z,ADPCMB_Update_L4083		;404e
	jp ADPCMB_Update_L4069		;4051
ADPCMB_Update_L4054:
	ld hl,(0fbb2h)		;4054
	ld bc,(0fbb4h)		;4057
	ld a,b			;405b
	or c			;405c
	jp z,ADPCMB_Update_L4071		;405d
	dec bc			;4060
	ld a,003h		;4061
	ld (wPCMB_Shared_FBBA),a		;4063
	jp ADPCMB_Update_L40B9		;4066
ADPCMB_Update_L4069:
	ld hl,(0fbb2h)		;4069
	ld bc,(0fbb4h)		;406c
	add hl,bc			;4070
ADPCMB_Update_L4071:
	ld bc,(0fbb6h)		;4071
	ld a,b			;4075
	or c			;4076
	jp z,ADPCMB_Update_L40AB		;4077
	dec bc			;407a
	ld a,002h		;407b
	ld (wPCMB_Shared_FBBA),a		;407d
	jp ADPCMB_Update_L40B9		;4080
ADPCMB_Update_L4083:
	ld a,(0fbbbh)		;4083
	cp 0ffh		;4086
	jp z,ADPCMB_Update_L410A		;4088
	and a			;408b
	jr z,ADPCMB_Update_L409E		;408c
	dec a			;408e
	ld (0fbbbh),a		;408f
	ld c,004h		;4092
	ld de,01c80h		;4094  YM reg 1Ch <- 80h
	rst 8			;4097
	ld e,000h		;4098
	rst 8			;409a
	jp ADPCMB_Update_L410A		;409b
ADPCMB_Update_L409E:
	ld hl,(0fbb2h)		;409e
	ld bc,(0fbb4h)		;40a1
	add hl,bc			;40a5
	ld bc,(0fbb6h)		;40a6
	add hl,bc			;40aa
ADPCMB_Update_L40AB:
	ld bc,(0fbb8h)		;40ab
	ld a,b			;40af
	or c			;40b0
	jp z,ADPCMB_Update_L40FA		;40b1
	ld a,001h		;40b4
	ld (wPCMB_Shared_FBBA),a		;40b6
ADPCMB_Update_L40B9:
	push bc			;40b9
	ld de,01b00h		;40ba  YM reg 1Bh <- 00h
	ld c,004h		;40bd
	rst 8			;40bf
	xor a			;40c0
	ld (0fbabh),a		;40c1
	ld de,01100h		;40c4  YM reg 11h <- 00h
	rst 8			;40c7
	xor a			;40c8
	ld (0fbbdh),a		;40c9
	ld e,l			;40cc
	ld d,012h		;40cd
	rst 8			;40cf
	ld e,h			;40d0
	inc d			;40d1
	rst 8			;40d2
	pop bc			;40d3
	add hl,bc			;40d4
	ld e,l			;40d5
	inc d			;40d6
	ld c,004h		;40d7
	rst 8			;40d9
	ld e,h			;40da
	inc d			;40db
	rst 8			;40dc
	ld de,01c80h		;40dd  YM reg 1Ch <- 80h
	rst 8			;40e0
	ld de,01c00h		;40e1  YM reg 1Ch <- 00h
	rst 8			;40e4
	ld de,01001h		;40e5  YM reg 10h <- 01h
	rst 8			;40e8
	ld e,080h		;40e9
	ld a,(wPCMB_Shared_FBBA)		;40eb
	cp 002h		;40ee
	jr nz,ADPCMB_Update_L40F4		;40f0
	ld e,090h		;40f2
ADPCMB_Update_L40F4:
	ld d,010h		;40f4
	rst 8			;40f6
	jp ADPCMB_Update_L410A		;40f7
ADPCMB_Update_L40FA:
	ld de,01b00h		;40fa  YM reg 1Bh <- 00h
	ld c,004h		;40fd
	rst 8			;40ff
	xor a			;4100
	ld (wPCMB_Shared_FBBA),a		;4101
	ld (0fbcbh),a		;4104
	jp ADPCMB_Update_L423C		;4107
ADPCMB_Update_L410A:
	ld a,(0fbbch)		;410a
	and a			;410d
	jr nz,ADPCMB_Update_L4113		;410e
	ld a,(iy+00fh)		;4110
ADPCMB_Update_L4113:
	rrca			;4113
	rrca			;4114
	ld hl,0fbbdh		;4115
	cp (hl)			;4118
	jr z,ADPCMB_Update_L4122		;4119
	ld (hl),a			;411b
	ld e,a			;411c
	ld d,011h		;411d
	ld c,004h		;411f
	rst 8			;4121
ADPCMB_Update_L4122:
	ld l,(iy+010h)		;4122
	ld h,(iy+011h)		;4125
	ld de,(0fba4h)		;4128
	add hl,de			;412c
	ld (wTmpWord_F812),hl		;412d
	ld hl,0fbdah		;4130
	ld d,(iy+00ch)		;4133
	call Env_Update_231E		;4136
	ld l,a			;4139
	ld h,000h		;413a
	ld b,h			;413c
	add hl,hl			;413d
	and a			;413e
	sbc hl,bc		;413f
	ld de,(wTmpWord_F812)		;4141
	add hl,de			;4145
	ld (wTmpWord_F812),hl		;4146
	ld hl,0fbe8h		;4149
	call Vib_Update_248E		;414c
	ld de,(wTmpWord_F812)		;414f
	add hl,de			;4153
	ld c,l			;4154
	ld b,h			;4155
	ld hl,(0fba9h)		;4156
	and a			;4159
	sbc hl,bc		;415a
	jr z,ADPCMB_Update_L419C		;415c
	ld (0fba9h),bc		;415e
	ld de,(0fbafh)		;4162
	ld h,017h		;4166
	ld l,c			;4168
	ld c,(hl)			;4169
	push de			;416a
	call Util_TableLookup_1F54		;416b
	pop de			;416e
	add hl,de			;416f
	jr nc,ADPCMB_Update_L4177		;4170
	rr h		;4172
	rr l		;4174
	inc b			;4176
ADPCMB_Update_L4177:
	ld a,b			;4177
	and a			;4178
	jp z,ADPCMB_Update_L4193		;4179
	jp p,ADPCMB_Update_L418A		;417c
ADPCMB_Update_L417F:
	srl h		;417f
	rr l		;4181
	inc a			;4183
	jp nz,ADPCMB_Update_L417F		;4184
	jp ADPCMB_Update_L4193		;4187
ADPCMB_Update_L418A:
	add hl,hl			;418a
	jr nc,ADPCMB_Update_L418F		;418b
	sbc hl,hl		;418d
ADPCMB_Update_L418F:
	dec a			;418f
	jp nz,ADPCMB_Update_L418A		;4190
ADPCMB_Update_L4193:
	ld e,l			;4193
	ld d,019h		;4194
	ld c,004h		;4196
	rst 8			;4198
	inc d			;4199
	ld e,h			;419a
	rst 8			;419b
ADPCMB_Update_L419C:
	ld hl,0fbbeh		;419c
	call Env_Step_222D		;419f
	ld (wTmpByte_F814),a		;41a2
	ld d,(iy+00dh)		;41a5
	ld hl,0fbcch		;41a8
	call Env_Update_231E		;41ab
	sub c			;41ae
	dec a			;41af
	ld l,a			;41b0
	ld a,(wTmpByte_F814)		;41b1
	and a			;41b4
	jr z,ADPCMB_Update_L41CC		;41b5
	inc l			;41b7
	jr z,ADPCMB_Update_L41CC		;41b8
	ex af,af'			;41ba
	dec l			;41bb
	ld a,l			;41bc
	jr z,ADPCMB_Update_L41CC		;41bd
	ld h,014h		;41bf
	ld a,(hl)			;41c1
	ex af,af'			;41c2
	ld l,a			;41c3
	ex af,af'			;41c4
	add a,(hl)			;41c5
	ld l,a			;41c6
	sbc a,a			;41c7
	jr nc,ADPCMB_Update_L41CC		;41c8
	inc h			;41ca
	ld a,(hl)			;41cb
ADPCMB_Update_L41CC:
	ld l,a			;41cc
	ld a,(0fba6h)		;41cd
	and a			;41d0
	jr z,ADPCMB_Update_L41E8		;41d1
	inc l			;41d3
	jr z,ADPCMB_Update_L41E8		;41d4
	ex af,af'			;41d6
	dec l			;41d7
	ld a,l			;41d8
	jr z,ADPCMB_Update_L41E8		;41d9
	ld h,014h		;41db
	ld a,(hl)			;41dd
	ex af,af'			;41de
	ld l,a			;41df
	ex af,af'			;41e0
	add a,(hl)			;41e1
	ld l,a			;41e2
	sbc a,a			;41e3
	jr nc,ADPCMB_Update_L41E8		;41e4
	inc h			;41e6
	ld a,(hl)			;41e7
ADPCMB_Update_L41E8:
	ld l,(iy+012h)		;41e8
	and a			;41eb
	jr z,ADPCMB_Update_L4203		;41ec
	inc l			;41ee
	jr z,ADPCMB_Update_L4203		;41ef
	ex af,af'			;41f1
	dec l			;41f2
	ld a,l			;41f3
	jr z,ADPCMB_Update_L4203		;41f4
	ld h,014h		;41f6
	ld a,(hl)			;41f8
	ex af,af'			;41f9
	ld l,a			;41fa
	ex af,af'			;41fb
	add a,(hl)			;41fc
	ld l,a			;41fd
	sbc a,a			;41fe
	jr nc,ADPCMB_Update_L4203		;41ff
	inc h			;4201
	ld a,(hl)			;4202
ADPCMB_Update_L4203:
	ld l,a			;4203
	ld a,0ffh		;4204
	bit 1,(iy+009h)		;4206
	jp z,ADPCMB_Update_L4210		;420a
	ld a,(wMasterVolume_F805)		;420d
ADPCMB_Update_L4210:
	and a			;4210
	jr z,ADPCMB_Update_L4228		;4211
	inc l			;4213
	jr z,ADPCMB_Update_L4228		;4214
	ex af,af'			;4216
	dec l			;4217
	ld a,l			;4218
	jr z,ADPCMB_Update_L4228		;4219
	ld h,014h		;421b
	ld a,(hl)			;421d
	ex af,af'			;421e
	ld l,a			;421f
	ex af,af'			;4220
	add a,(hl)			;4221
	ld l,a			;4222
	sbc a,a			;4223
	jr nc,ADPCMB_Update_L4228		;4224
	inc h			;4226
	ld a,(hl)			;4227
ADPCMB_Update_L4228:
	ld hl,0fbabh		;4228
	cp (hl)			;422b
	ld (hl),a			;422c
	jr z,ADPCMB_Update_L423C		;422d
	add a,a			;422f
	ld e,080h		;4230
	add a,e			;4232
	adc a,e			;4233
	ld e,a			;4234
	ld d,01bh		;4235
	ld c,004h		;4237
	ld c,004h		;4239
	rst 8			;423b
ADPCMB_Update_L423C:
	ld c,001h		;423c
	bit 1,(iy+009h)		;423e
	jr z,ADPCMB_Update_L4248		;4242
	ld a,(wTempoStep_F809)		;4244
	ld c,a			;4247
ADPCMB_Update_L4248:
	ld hl,0fba1h		;4248
	ld a,(hl)			;424b
	add a,c			;424c
	sbc a,000h		;424d
	ld (hl),a			;424f
	ld hl,0fba0h		;4250
	ld a,(hl)			;4253
	and a			;4254
	jp z,ADPCMB_Update_L4269		;4255
	sub c			;4258
	ld (hl),a			;4259
	jr nz,ADPCMB_Update_L427E		;425a
	bit 4,(iy+009h)		;425c
	jr z,ADPCMB_Update_L4278		;4260
	set 4,(ix+003h)		;4262
	jp ADPCMB_Update_L427E		;4266
ADPCMB_Update_L4269:
	bit 4,(iy+009h)		;4269
	jr nz,ADPCMB_Update_L427E		;426d
	ld hl,0fb9eh		;426f
	bit 4,(hl)		;4272
	jr z,ADPCMB_Update_L427E		;4274
	res 4,(hl)		;4276
ADPCMB_Update_L4278:
	ld hl,0fbbeh		;4278
	call Env_Release_2219		;427b
ADPCMB_Update_L427E:
	ret			;427e
ADPCMB_LoadPatch_427F:
	exx			;427f
	ld (ix+011h),b		;4280
	push ix		;4283
	inc hl			;4285
	ld de,00023h		;4286  constant
	add ix,de		;4289
	call Seq_LoadOpPtrs_219B		;428b
	push iy		;428e
	ex de,hl			;4290
	defb 0fdh,06bh	;ld iyl,e		;4291
	defb 0fdh,062h	;ld iyh,d		;4293
	ld de,0000eh		;4295  constant
	add ix,de		;4298
	call Lfo_Load_22A8		;429a
	ld de,00006h		;429d  constant
	add iy,de		;42a0
	ld de,0000eh		;42a2  constant
	add ix,de		;42a5
	call Lfo_Load_22A8		;42a7
	ld de,00006h		;42aa  constant
	add iy,de		;42ad
	ld de,0000eh		;42af  constant
	add ix,de		;42b2
	call Pitch_Load_242E		;42b4
	defb 0fdh,05dh	;ld e,iyl		;42b7
	defb 0fdh,054h	;ld d,iyh		;42b9
	pop iy		;42bb
	pop ix		;42bd
	ld hl,00004h		;42bf
	add hl,de			;42c2
	ld (ix+012h),l		;42c3
	ld (ix+013h),h		;42c6
	ret			;42c9
ADPCMB_SetPitch_42CA:
	ld l,(ix+008h)		;42ca
	ld h,011h		;42cd
	ld c,(hl)			;42cf
	set 7,l		;42d0
	ld b,(hl)			;42d2
	ld e,c			;42d3
	ld d,b			;42d4
	ld l,(ix+016h)		;42d5
	ld h,011h		;42d8
	ld c,(hl)			;42da
	set 7,l		;42db
	ld b,(hl)			;42dd
	ex de,hl			;42de
	and a			;42df
	sbc hl,bc		;42e0
	ld (ix+009h),l		;42e2
	ld (ix+00ah),h		;42e5
	ld hl,0ffffh		;42e8
	ld (0fba9h),hl		;42eb
	ret			;42ee
ADPCMB_KeyOn_42EF:
	ld a,004h		;42ef
	ld (wPCMB_Shared_FBBA),a		;42f1
	ld a,(0fbach)		;42f4
	ld b,a			;42f7
	ld a,(wCurBank_F811)		;42f8
	cp b			;42fb
	jp z,ADPCMB_KeyOn_L4319		;42fc
	ld a,b			;42ff
	ld (wCurBank_F811),a		;4300
	ld c,00bh		;4303
	in a,(c)		;4305
	inc b			;4307
	sla b		;4308
	dec c			;430a
	in a,(c)		;430b
	inc b			;430d
	sla b		;430e
	dec c			;4310
	in a,(c)		;4311
	inc b			;4313
	sla b		;4314
	dec c			;4316
	in a,(c)		;4317
ADPCMB_KeyOn_L4319:
	ld a,(0fba3h)		;4319
	ld hl,(0fbadh)		;431c
	ld de,00006h		;431f  constant
ADPCMB_KeyOn_L4322:
	cp (hl)			;4322
	jr z,ADPCMB_KeyOn_L432B		;4323
	jr c,ADPCMB_KeyOn_L432B		;4325
	add hl,de			;4327
	jp ADPCMB_KeyOn_L4322		;4328
ADPCMB_KeyOn_L432B:
	inc hl			;432b
	ld a,(hl)			;432c
	ld (0fbb1h),a		;432d
	inc hl			;4330
	call Bank_ResolvePtr24_1FA6		;4331
	ld a,(hl)			;4334
	ld (0fbafh),a		;4335
	inc hl			;4338
	ld a,(hl)			;4339
	ld (0fbb0h),a		;433a
	inc hl			;433d
	ld a,(hl)			;433e
	ld (0fbbch),a		;433f
	inc hl			;4342
	ld a,(hl)			;4343
	ld (0fbbbh),a		;4344
	inc hl			;4347
	ld e,(hl)			;4348
	inc hl			;4349
	ld d,(hl)			;434a
	ld (0fbb2h),de		;434b
	inc hl			;434f
	ld e,(hl)			;4350
	inc hl			;4351
	ld d,(hl)			;4352
	ld (0fbb4h),de		;4353
	inc hl			;4357
	ld e,(hl)			;4358
	inc hl			;4359
	ld d,(hl)			;435a
	ld (0fbb6h),de		;435b
	inc hl			;435f
	ld e,(hl)			;4360
	inc hl			;4361
	ld d,(hl)			;4362
	ld (0fbb8h),de		;4363
	ld hl,0fbbeh		;4367
	call Env_Start_2208		;436a
	ld hl,0fbcch		;436d
	call Env_Init_22F4		;4370
	ld hl,0fbdah		;4373
	call Env_Init_22F4		;4376
	push ix		;4379
	ld de,0004dh		;437b  constant
	add ix,de		;437e
	ld c,(iy+00eh)		;4380
	call Pitch_Update_2461		;4383
	pop ix		;4386
	ret			;4388
Ch_AllocByType_4389:
	ld hl,(0fbf9h)		;4389
	ld a,(hl)			;438c
	cp 000h		;438d
	jp z,Ch_AllocByType_L46BB		;438f
	cp 001h		;4392
	jp z,Ch_AllocByType_L480E		;4394
	cp 002h		;4397
	jp z,Ch_AllocByType_L4424		;4399
	cp 003h		;439c
	jp z,Ch_AllocByType_L43A9		;439e
	ld hl,00000h		;43a1  constant
	push hl			;43a4
	pop ix		;43a5
	and a			;43a7
	ret			;43a8
Ch_AllocByType_L43A9:
	defb 0fdh,05dh	;ld e,iyl		;43a9
	defb 0fdh,054h	;ld d,iyh		;43ab
	exx			;43ad
	xor a			;43ae
	ld d,a			;43af
	ld e,a			;43b0
	ld l,a			;43b1
	ld h,a			;43b2
	ld b,000h		;43b3
	ld ix,wPCMB_Ch_FB9B		;43b5
	exx			;43b9
	ld hl,(wPCMB_Ch_FB9B)		;43ba
	and a			;43bd
	sbc hl,de		;43be
	exx			;43c0
	jr nz,Ch_AllocByType_L43DB		;43c1
	ld a,(wNoteTmp_FBF5)		;43c3
	sub (ix+008h)		;43c6
	jp nz,Ch_AllocByType_L43DB		;43c9
	ld a,(0fba0h)		;43cc
	and a			;43cf
	jp z,Ch_AllocByType_L43DB		;43d0
	ld hl,wPCMB_Ch_FB9B		;43d3
	ld d,003h		;43d6
	jp Ch_AllocByType_L49CA		;43d8
Ch_AllocByType_L43DB:
	bit 1,(iy+009h)		;43db
	jr z,Ch_AllocByType_L43E8		;43df
	bit 1,(ix+003h)		;43e1
	jp z,Ch_AllocByType_L4421		;43e5
Ch_AllocByType_L43E8:
	ld c,001h		;43e8
	ld a,(0fba0h)		;43ea
	and a			;43ed
	jr nz,Ch_AllocByType_L43FA		;43ee
	inc c			;43f0
	ld a,(0fba2h)		;43f1
	sub (ix+006h)		;43f4
	jr nc,Ch_AllocByType_L43FA		;43f7
	inc c			;43f9
Ch_AllocByType_L43FA:
	ld a,d			;43fa
	cp c			;43fb
	jp c,Ch_AllocByType_L4415		;43fc
	jp nz,Ch_AllocByType_L4421		;43ff
	ld a,(0fb9fh)		;4402
	cp e			;4405
	jr z,Ch_AllocByType_L440E		;4406
	jp nc,Ch_AllocByType_L4415		;4408
	jp Ch_AllocByType_L4421		;440b
Ch_AllocByType_L440E:
	ld a,(0fba1h)		;440e
	cp b			;4411
	jp c,Ch_AllocByType_L4421		;4412
Ch_AllocByType_L4415:
	ld d,c			;4415
	ld a,(0fb9fh)		;4416
	ld e,a			;4419
	ld a,(0fba1h)		;441a
	ld b,a			;441d
	ld hl,wPCMB_Ch_FB9B		;441e
Ch_AllocByType_L4421:
	jp Ch_AllocByType_L49CA		;4421
Ch_AllocByType_L4424:
	defb 0fdh,05dh	;ld e,iyl		;4424
	defb 0fdh,054h	;ld d,iyh		;4426
	exx			;4428
	xor a			;4429
	ld d,a			;442a
	ld e,a			;442b
	ld l,a			;442c
	ld h,a			;442d
	ld b,000h		;442e
	ld ix,wPCMA_Ch1_FAA5		;4430
	exx			;4434
	ld hl,(wPCMA_Ch1_FAA5)		;4435
	and a			;4438
	sbc hl,de		;4439
	exx			;443b
	jr nz,Ch_AllocByType_L4456		;443c
	ld a,(wNoteTmp_FBF5)		;443e
	sub (ix+008h)		;4441
	jp nz,Ch_AllocByType_L4456		;4444
	ld a,(0faaah)		;4447
	and a			;444a
	jp z,Ch_AllocByType_L4456		;444b
	ld hl,wPCMA_Ch1_FAA5		;444e
	ld d,003h		;4451
	jp Ch_AllocByType_L49CA		;4453
Ch_AllocByType_L4456:
	bit 1,(iy+009h)		;4456
	jr z,Ch_AllocByType_L4463		;445a
	bit 1,(ix+003h)		;445c
	jp z,Ch_AllocByType_L449C		;4460
Ch_AllocByType_L4463:
	ld c,001h		;4463
	ld a,(0faaah)		;4465
	and a			;4468
	jr nz,Ch_AllocByType_L4475		;4469
	inc c			;446b
	ld a,(0faach)		;446c
	sub (ix+006h)		;446f
	jr nc,Ch_AllocByType_L4475		;4472
	inc c			;4474
Ch_AllocByType_L4475:
	ld a,d			;4475
	cp c			;4476
	jp c,Ch_AllocByType_L4490		;4477
	jp nz,Ch_AllocByType_L449C		;447a
	ld a,(0faa9h)		;447d
	cp e			;4480
	jr z,Ch_AllocByType_L4489		;4481
	jp nc,Ch_AllocByType_L4490		;4483
	jp Ch_AllocByType_L449C		;4486
Ch_AllocByType_L4489:
	ld a,(0faabh)		;4489
	cp b			;448c
	jp c,Ch_AllocByType_L449C		;448d
Ch_AllocByType_L4490:
	ld d,c			;4490
	ld a,(0faa9h)		;4491
	ld e,a			;4494
	ld a,(0faabh)		;4495
	ld b,a			;4498
	ld hl,wPCMA_Ch1_FAA5		;4499
Ch_AllocByType_L449C:
	ld ix,wPCMA_Ch2_FACE		;449c
	exx			;44a0
	ld hl,(wPCMA_Ch2_FACE)		;44a1
	and a			;44a4
	sbc hl,de		;44a5
	exx			;44a7
	jr nz,Ch_AllocByType_L44C2		;44a8
	ld a,(wNoteTmp_FBF5)		;44aa
	sub (ix+008h)		;44ad
	jp nz,Ch_AllocByType_L44C2		;44b0
	ld a,(0fad3h)		;44b3
	and a			;44b6
	jp z,Ch_AllocByType_L44C2		;44b7
	ld hl,wPCMA_Ch2_FACE		;44ba
	ld d,003h		;44bd
	jp Ch_AllocByType_L49CA		;44bf
Ch_AllocByType_L44C2:
	bit 1,(iy+009h)		;44c2
	jr z,Ch_AllocByType_L44CF		;44c6
	bit 1,(ix+003h)		;44c8
	jp z,Ch_AllocByType_L4508		;44cc
Ch_AllocByType_L44CF:
	ld c,001h		;44cf
	ld a,(0fad3h)		;44d1
	and a			;44d4
	jr nz,Ch_AllocByType_L44E1		;44d5
	inc c			;44d7
	ld a,(0fad5h)		;44d8
	sub (ix+006h)		;44db
	jr nc,Ch_AllocByType_L44E1		;44de
	inc c			;44e0
Ch_AllocByType_L44E1:
	ld a,d			;44e1
	cp c			;44e2
	jp c,Ch_AllocByType_L44FC		;44e3
	jp nz,Ch_AllocByType_L4508		;44e6
	ld a,(0fad2h)		;44e9
	cp e			;44ec
	jr z,Ch_AllocByType_L44F5		;44ed
	jp nc,Ch_AllocByType_L44FC		;44ef
	jp Ch_AllocByType_L4508		;44f2
Ch_AllocByType_L44F5:
	ld a,(0fad4h)		;44f5
	cp b			;44f8
	jp c,Ch_AllocByType_L4508		;44f9
Ch_AllocByType_L44FC:
	ld d,c			;44fc
	ld a,(0fad2h)		;44fd
	ld e,a			;4500
	ld a,(0fad4h)		;4501
	ld b,a			;4504
	ld hl,wPCMA_Ch2_FACE		;4505
Ch_AllocByType_L4508:
	ld ix,wPCMA_Ch3_FAF7		;4508
	exx			;450c
	ld hl,(wPCMA_Ch3_FAF7)		;450d
	and a			;4510
	sbc hl,de		;4511
	exx			;4513
	jr nz,Ch_AllocByType_L452E		;4514
	ld a,(wNoteTmp_FBF5)		;4516
	sub (ix+008h)		;4519
	jp nz,Ch_AllocByType_L452E		;451c
	ld a,(0fafch)		;451f
	and a			;4522
	jp z,Ch_AllocByType_L452E		;4523
	ld hl,wPCMA_Ch3_FAF7		;4526
	ld d,003h		;4529
	jp Ch_AllocByType_L49CA		;452b
Ch_AllocByType_L452E:
	bit 1,(iy+009h)		;452e
	jr z,Ch_AllocByType_L453B		;4532
	bit 1,(ix+003h)		;4534
	jp z,Ch_AllocByType_L4574		;4538
Ch_AllocByType_L453B:
	ld c,001h		;453b
	ld a,(0fafch)		;453d
	and a			;4540
	jr nz,Ch_AllocByType_L454D		;4541
	inc c			;4543
	ld a,(0fafeh)		;4544
	sub (ix+006h)		;4547
	jr nc,Ch_AllocByType_L454D		;454a
	inc c			;454c
Ch_AllocByType_L454D:
	ld a,d			;454d
	cp c			;454e
	jp c,Ch_AllocByType_L4568		;454f
	jp nz,Ch_AllocByType_L4574		;4552
	ld a,(0fafbh)		;4555
	cp e			;4558
	jr z,Ch_AllocByType_L4561		;4559
	jp nc,Ch_AllocByType_L4568		;455b
	jp Ch_AllocByType_L4574		;455e
Ch_AllocByType_L4561:
	ld a,(0fafdh)		;4561
	cp b			;4564
	jp c,Ch_AllocByType_L4574		;4565
Ch_AllocByType_L4568:
	ld d,c			;4568
	ld a,(0fafbh)		;4569
	ld e,a			;456c
	ld a,(0fafdh)		;456d
	ld b,a			;4570
	ld hl,wPCMA_Ch3_FAF7		;4571
Ch_AllocByType_L4574:
	ld ix,wPCMA_Ch4_FB20		;4574
	exx			;4578
	ld hl,(wPCMA_Ch4_FB20)		;4579
	and a			;457c
	sbc hl,de		;457d
	exx			;457f
	jr nz,Ch_AllocByType_L459A		;4580
	ld a,(wNoteTmp_FBF5)		;4582
	sub (ix+008h)		;4585
	jp nz,Ch_AllocByType_L459A		;4588
	ld a,(0fb25h)		;458b
	and a			;458e
	jp z,Ch_AllocByType_L459A		;458f
	ld hl,wPCMA_Ch4_FB20		;4592
	ld d,003h		;4595
	jp Ch_AllocByType_L49CA		;4597
Ch_AllocByType_L459A:
	bit 1,(iy+009h)		;459a
	jr z,Ch_AllocByType_L45A7		;459e
	bit 1,(ix+003h)		;45a0
	jp z,Ch_AllocByType_L45E0		;45a4
Ch_AllocByType_L45A7:
	ld c,001h		;45a7
	ld a,(0fb25h)		;45a9
	and a			;45ac
	jr nz,Ch_AllocByType_L45B9		;45ad
	inc c			;45af
	ld a,(0fb27h)		;45b0
	sub (ix+006h)		;45b3
	jr nc,Ch_AllocByType_L45B9		;45b6
	inc c			;45b8
Ch_AllocByType_L45B9:
	ld a,d			;45b9
	cp c			;45ba
	jp c,Ch_AllocByType_L45D4		;45bb
	jp nz,Ch_AllocByType_L45E0		;45be
	ld a,(0fb24h)		;45c1
	cp e			;45c4
	jr z,Ch_AllocByType_L45CD		;45c5
	jp nc,Ch_AllocByType_L45D4		;45c7
	jp Ch_AllocByType_L45E0		;45ca
Ch_AllocByType_L45CD:
	ld a,(0fb26h)		;45cd
	cp b			;45d0
	jp c,Ch_AllocByType_L45E0		;45d1
Ch_AllocByType_L45D4:
	ld d,c			;45d4
	ld a,(0fb24h)		;45d5
	ld e,a			;45d8
	ld a,(0fb26h)		;45d9
	ld b,a			;45dc
	ld hl,wPCMA_Ch4_FB20		;45dd
Ch_AllocByType_L45E0:
	ld ix,wPCMA_Ch5_FB49		;45e0
	exx			;45e4
	ld hl,(wPCMA_Ch5_FB49)		;45e5
	and a			;45e8
	sbc hl,de		;45e9
	exx			;45eb
	jr nz,Ch_AllocByType_L4606		;45ec
	ld a,(wNoteTmp_FBF5)		;45ee
	sub (ix+008h)		;45f1
	jp nz,Ch_AllocByType_L4606		;45f4
	ld a,(0fb4eh)		;45f7
	and a			;45fa
	jp z,Ch_AllocByType_L4606		;45fb
	ld hl,wPCMA_Ch5_FB49		;45fe
	ld d,003h		;4601
	jp Ch_AllocByType_L49CA		;4603
Ch_AllocByType_L4606:
	bit 1,(iy+009h)		;4606
	jr z,Ch_AllocByType_L4613		;460a
	bit 1,(ix+003h)		;460c
	jp z,Ch_AllocByType_L464C		;4610
Ch_AllocByType_L4613:
	ld c,001h		;4613
	ld a,(0fb4eh)		;4615
	and a			;4618
	jr nz,Ch_AllocByType_L4625		;4619
	inc c			;461b
	ld a,(0fb50h)		;461c
	sub (ix+006h)		;461f
	jr nc,Ch_AllocByType_L4625		;4622
	inc c			;4624
Ch_AllocByType_L4625:
	ld a,d			;4625
	cp c			;4626
	jp c,Ch_AllocByType_L4640		;4627
	jp nz,Ch_AllocByType_L464C		;462a
	ld a,(0fb4dh)		;462d
	cp e			;4630
	jr z,Ch_AllocByType_L4639		;4631
	jp nc,Ch_AllocByType_L4640		;4633
	jp Ch_AllocByType_L464C		;4636
Ch_AllocByType_L4639:
	ld a,(0fb4fh)		;4639
	cp b			;463c
	jp c,Ch_AllocByType_L464C		;463d
Ch_AllocByType_L4640:
	ld d,c			;4640
	ld a,(0fb4dh)		;4641
	ld e,a			;4644
	ld a,(0fb4fh)		;4645
	ld b,a			;4648
	ld hl,wPCMA_Ch5_FB49		;4649
Ch_AllocByType_L464C:
	ld ix,wPCMA_Ch6_FB72		;464c
	exx			;4650
	ld hl,(wPCMA_Ch6_FB72)		;4651
	and a			;4654
	sbc hl,de		;4655
	exx			;4657
	jr nz,Ch_AllocByType_L4672		;4658
	ld a,(wNoteTmp_FBF5)		;465a
	sub (ix+008h)		;465d
	jp nz,Ch_AllocByType_L4672		;4660
	ld a,(0fb77h)		;4663
	and a			;4666
	jp z,Ch_AllocByType_L4672		;4667
	ld hl,wPCMA_Ch6_FB72		;466a
	ld d,003h		;466d
	jp Ch_AllocByType_L49CA		;466f
Ch_AllocByType_L4672:
	bit 1,(iy+009h)		;4672
	jr z,Ch_AllocByType_L467F		;4676
	bit 1,(ix+003h)		;4678
	jp z,Ch_AllocByType_L46B8		;467c
Ch_AllocByType_L467F:
	ld c,001h		;467f
	ld a,(0fb77h)		;4681
	and a			;4684
	jr nz,Ch_AllocByType_L4691		;4685
	inc c			;4687
	ld a,(0fb79h)		;4688
	sub (ix+006h)		;468b
	jr nc,Ch_AllocByType_L4691		;468e
	inc c			;4690
Ch_AllocByType_L4691:
	ld a,d			;4691
	cp c			;4692
	jp c,Ch_AllocByType_L46AC		;4693
	jp nz,Ch_AllocByType_L46B8		;4696
	ld a,(0fb76h)		;4699
	cp e			;469c
	jr z,Ch_AllocByType_L46A5		;469d
	jp nc,Ch_AllocByType_L46AC		;469f
	jp Ch_AllocByType_L46B8		;46a2
Ch_AllocByType_L46A5:
	ld a,(0fb78h)		;46a5
	cp b			;46a8
	jp c,Ch_AllocByType_L46B8		;46a9
Ch_AllocByType_L46AC:
	ld d,c			;46ac
	ld a,(0fb76h)		;46ad
	ld e,a			;46b0
	ld a,(0fb78h)		;46b1
	ld b,a			;46b4
	ld hl,wPCMA_Ch6_FB72		;46b5
Ch_AllocByType_L46B8:
	jp Ch_AllocByType_L49CA		;46b8
Ch_AllocByType_L46BB:
	defb 0fdh,05dh	;ld e,iyl		;46bb
	defb 0fdh,054h	;ld d,iyh		;46bd
	exx			;46bf
	xor a			;46c0
	ld d,a			;46c1
	ld e,a			;46c2
	ld l,a			;46c3
	ld h,a			;46c4
	ld b,000h		;46c5
	ld ix,wSSG_ChA_F81F		;46c7
	exx			;46cb
	ld hl,(wSSG_ChA_F81F)		;46cc
	and a			;46cf
	sbc hl,de		;46d0
	exx			;46d2
	jr nz,Ch_AllocByType_L46ED		;46d3
	ld a,(wNoteTmp_FBF5)		;46d5
	sub (ix+008h)		;46d8
	jp nz,Ch_AllocByType_L46ED		;46db
	ld a,(0f824h)		;46de
	and a			;46e1
	jp z,Ch_AllocByType_L46ED		;46e2
	ld hl,wSSG_ChA_F81F		;46e5
	ld d,003h		;46e8
	jp Ch_AllocByType_L49CA		;46ea
Ch_AllocByType_L46ED:
	bit 1,(iy+009h)		;46ed
	jr z,Ch_AllocByType_L46FA		;46f1
	bit 1,(ix+003h)		;46f3
	jp z,Ch_AllocByType_L4733		;46f7
Ch_AllocByType_L46FA:
	ld c,001h		;46fa
	ld a,(0f824h)		;46fc
	and a			;46ff
	jr nz,Ch_AllocByType_L470C		;4700
	inc c			;4702
	ld a,(0f826h)		;4703
	sub (ix+006h)		;4706
	jr nc,Ch_AllocByType_L470C		;4709
	inc c			;470b
Ch_AllocByType_L470C:
	ld a,d			;470c
	cp c			;470d
	jp c,Ch_AllocByType_L4727		;470e
	jp nz,Ch_AllocByType_L4733		;4711
	ld a,(0f823h)		;4714
	cp e			;4717
	jr z,Ch_AllocByType_L4720		;4718
	jp nc,Ch_AllocByType_L4727		;471a
	jp Ch_AllocByType_L4733		;471d
Ch_AllocByType_L4720:
	ld a,(0f825h)		;4720
	cp b			;4723
	jp c,Ch_AllocByType_L4733		;4724
Ch_AllocByType_L4727:
	ld d,c			;4727
	ld a,(0f823h)		;4728
	ld e,a			;472b
	ld a,(0f825h)		;472c
	ld b,a			;472f
	ld hl,wSSG_ChA_F81F		;4730
Ch_AllocByType_L4733:
	ld ix,wSSG_ChB_F86B		;4733
	exx			;4737
	ld hl,(wSSG_ChB_F86B)		;4738
	and a			;473b
	sbc hl,de		;473c
	exx			;473e
	jr nz,Ch_AllocByType_L4759		;473f
	ld a,(wNoteTmp_FBF5)		;4741
	sub (ix+008h)		;4744
	jp nz,Ch_AllocByType_L4759		;4747
	ld a,(0f870h)		;474a
	and a			;474d
	jp z,Ch_AllocByType_L4759		;474e
	ld hl,wSSG_ChB_F86B		;4751
	ld d,003h		;4754
	jp Ch_AllocByType_L49CA		;4756
Ch_AllocByType_L4759:
	bit 1,(iy+009h)		;4759
	jr z,Ch_AllocByType_L4766		;475d
	bit 1,(ix+003h)		;475f
	jp z,Ch_AllocByType_L479F		;4763
Ch_AllocByType_L4766:
	ld c,001h		;4766
	ld a,(0f870h)		;4768
	and a			;476b
	jr nz,Ch_AllocByType_L4778		;476c
	inc c			;476e
	ld a,(0f872h)		;476f
	sub (ix+006h)		;4772
	jr nc,Ch_AllocByType_L4778		;4775
	inc c			;4777
Ch_AllocByType_L4778:
	ld a,d			;4778
	cp c			;4779
	jp c,Ch_AllocByType_L4793		;477a
	jp nz,Ch_AllocByType_L479F		;477d
	ld a,(0f86fh)		;4780
	cp e			;4783
	jr z,Ch_AllocByType_L478C		;4784
	jp nc,Ch_AllocByType_L4793		;4786
	jp Ch_AllocByType_L479F		;4789
Ch_AllocByType_L478C:
	ld a,(0f871h)		;478c
	cp b			;478f
	jp c,Ch_AllocByType_L479F		;4790
Ch_AllocByType_L4793:
	ld d,c			;4793
	ld a,(0f86fh)		;4794
	ld e,a			;4797
	ld a,(0f871h)		;4798
	ld b,a			;479b
	ld hl,wSSG_ChB_F86B		;479c
Ch_AllocByType_L479F:
	ld ix,wSSG_ChC_F8B7		;479f
	exx			;47a3
	ld hl,(wSSG_ChC_F8B7)		;47a4
	and a			;47a7
	sbc hl,de		;47a8
	exx			;47aa
	jr nz,Ch_AllocByType_L47C5		;47ab
	ld a,(wNoteTmp_FBF5)		;47ad
	sub (ix+008h)		;47b0
	jp nz,Ch_AllocByType_L47C5		;47b3
	ld a,(0f8bch)		;47b6
	and a			;47b9
	jp z,Ch_AllocByType_L47C5		;47ba
	ld hl,wSSG_ChC_F8B7		;47bd
	ld d,003h		;47c0
	jp Ch_AllocByType_L49CA		;47c2
Ch_AllocByType_L47C5:
	bit 1,(iy+009h)		;47c5
	jr z,Ch_AllocByType_L47D2		;47c9
	bit 1,(ix+003h)		;47cb
	jp z,Ch_AllocByType_L480B		;47cf
Ch_AllocByType_L47D2:
	ld c,001h		;47d2
	ld a,(0f8bch)		;47d4
	and a			;47d7
	jr nz,Ch_AllocByType_L47E4		;47d8
	inc c			;47da
	ld a,(0f8beh)		;47db
	sub (ix+006h)		;47de
	jr nc,Ch_AllocByType_L47E4		;47e1
	inc c			;47e3
Ch_AllocByType_L47E4:
	ld a,d			;47e4
	cp c			;47e5
	jp c,Ch_AllocByType_L47FF		;47e6
	jp nz,Ch_AllocByType_L480B		;47e9
	ld a,(0f8bbh)		;47ec
	cp e			;47ef
	jr z,Ch_AllocByType_L47F8		;47f0
	jp nc,Ch_AllocByType_L47FF		;47f2
	jp Ch_AllocByType_L480B		;47f5
Ch_AllocByType_L47F8:
	ld a,(0f8bdh)		;47f8
	cp b			;47fb
	jp c,Ch_AllocByType_L480B		;47fc
Ch_AllocByType_L47FF:
	ld d,c			;47ff
	ld a,(0f8bbh)		;4800
	ld e,a			;4803
	ld a,(0f8bdh)		;4804
	ld b,a			;4807
	ld hl,wSSG_ChC_F8B7		;4808
Ch_AllocByType_L480B:
	jp Ch_AllocByType_L49CA		;480b
Ch_AllocByType_L480E:
	defb 0fdh,05dh	;ld e,iyl		;480e
	defb 0fdh,054h	;ld d,iyh		;4810
	exx			;4812
	xor a			;4813
	ld d,a			;4814
	ld e,a			;4815
	ld l,a			;4816
	ld h,a			;4817
	ld b,000h		;4818
	ld ix,wFM_Ch1_F907		;481a
	exx			;481e
	ld hl,(wFM_Ch1_F907)		;481f
	and a			;4822
	sbc hl,de		;4823
	exx			;4825
	jr nz,Ch_AllocByType_L4840		;4826
	ld a,(wNoteTmp_FBF5)		;4828
	sub (ix+008h)		;482b
	jp nz,Ch_AllocByType_L4840		;482e
	ld a,(0f90ch)		;4831
	and a			;4834
	jp z,Ch_AllocByType_L4840		;4835
	ld hl,wFM_Ch1_F907		;4838
	ld d,003h		;483b
	jp Ch_AllocByType_L49CA		;483d
Ch_AllocByType_L4840:
	bit 1,(iy+009h)		;4840
	jr z,Ch_AllocByType_L484D		;4844
	bit 1,(ix+003h)		;4846
	jp z,Ch_AllocByType_L4886		;484a
Ch_AllocByType_L484D:
	ld c,001h		;484d
	ld a,(0f90ch)		;484f
	and a			;4852
	jr nz,Ch_AllocByType_L485F		;4853
	inc c			;4855
	ld a,(0f90eh)		;4856
	sub (ix+006h)		;4859
	jr nc,Ch_AllocByType_L485F		;485c
	inc c			;485e
Ch_AllocByType_L485F:
	ld a,d			;485f
	cp c			;4860
	jp c,Ch_AllocByType_L487A		;4861
	jp nz,Ch_AllocByType_L4886		;4864
	ld a,(0f90bh)		;4867
	cp e			;486a
	jr z,Ch_AllocByType_L4873		;486b
	jp nc,Ch_AllocByType_L487A		;486d
	jp Ch_AllocByType_L4886		;4870
Ch_AllocByType_L4873:
	ld a,(0f90dh)		;4873
	cp b			;4876
	jp c,Ch_AllocByType_L4886		;4877
Ch_AllocByType_L487A:
	ld d,c			;487a
	ld a,(0f90bh)		;487b
	ld e,a			;487e
	ld a,(0f90dh)		;487f
	ld b,a			;4882
	ld hl,wFM_Ch1_F907		;4883
Ch_AllocByType_L4886:
	ld ix,wFM_Ch2_F96E		;4886
	exx			;488a
	ld hl,(wFM_Ch2_F96E)		;488b
	and a			;488e
	sbc hl,de		;488f
	exx			;4891
	jr nz,Ch_AllocByType_L48AC		;4892
	ld a,(wNoteTmp_FBF5)		;4894
	sub (ix+008h)		;4897
	jp nz,Ch_AllocByType_L48AC		;489a
	ld a,(0f973h)		;489d
	and a			;48a0
	jp z,Ch_AllocByType_L48AC		;48a1
	ld hl,wFM_Ch2_F96E		;48a4
	ld d,003h		;48a7
	jp Ch_AllocByType_L49CA		;48a9
Ch_AllocByType_L48AC:
	bit 1,(iy+009h)		;48ac
	jr z,Ch_AllocByType_L48B9		;48b0
	bit 1,(ix+003h)		;48b2
	jp z,Ch_AllocByType_L48F2		;48b6
Ch_AllocByType_L48B9:
	ld c,001h		;48b9
	ld a,(0f973h)		;48bb
	and a			;48be
	jr nz,Ch_AllocByType_L48CB		;48bf
	inc c			;48c1
	ld a,(0f975h)		;48c2
	sub (ix+006h)		;48c5
	jr nc,Ch_AllocByType_L48CB		;48c8
	inc c			;48ca
Ch_AllocByType_L48CB:
	ld a,d			;48cb
	cp c			;48cc
	jp c,Ch_AllocByType_L48E6		;48cd
	jp nz,Ch_AllocByType_L48F2		;48d0
	ld a,(0f972h)		;48d3
	cp e			;48d6
	jr z,Ch_AllocByType_L48DF		;48d7
	jp nc,Ch_AllocByType_L48E6		;48d9
	jp Ch_AllocByType_L48F2		;48dc
Ch_AllocByType_L48DF:
	ld a,(0f974h)		;48df
	cp b			;48e2
	jp c,Ch_AllocByType_L48F2		;48e3
Ch_AllocByType_L48E6:
	ld d,c			;48e6
	ld a,(0f972h)		;48e7
	ld e,a			;48ea
	ld a,(0f974h)		;48eb
	ld b,a			;48ee
	ld hl,wFM_Ch2_F96E		;48ef
Ch_AllocByType_L48F2:
	ld ix,wFM_Ch3_F9D5		;48f2
	exx			;48f6
	ld hl,(wFM_Ch3_F9D5)		;48f7
	and a			;48fa
	sbc hl,de		;48fb
	exx			;48fd
	jr nz,Ch_AllocByType_L4918		;48fe
	ld a,(wNoteTmp_FBF5)		;4900
	sub (ix+008h)		;4903
	jp nz,Ch_AllocByType_L4918		;4906
	ld a,(0f9dah)		;4909
	and a			;490c
	jp z,Ch_AllocByType_L4918		;490d
	ld hl,wFM_Ch3_F9D5		;4910
	ld d,003h		;4913
	jp Ch_AllocByType_L49CA		;4915
Ch_AllocByType_L4918:
	bit 1,(iy+009h)		;4918
	jr z,Ch_AllocByType_L4925		;491c
	bit 1,(ix+003h)		;491e
	jp z,Ch_AllocByType_L495E		;4922
Ch_AllocByType_L4925:
	ld c,001h		;4925
	ld a,(0f9dah)		;4927
	and a			;492a
	jr nz,Ch_AllocByType_L4937		;492b
	inc c			;492d
	ld a,(0f9dch)		;492e
	sub (ix+006h)		;4931
	jr nc,Ch_AllocByType_L4937		;4934
	inc c			;4936
Ch_AllocByType_L4937:
	ld a,d			;4937
	cp c			;4938
	jp c,Ch_AllocByType_L4952		;4939
	jp nz,Ch_AllocByType_L495E		;493c
	ld a,(0f9d9h)		;493f
	cp e			;4942
	jr z,Ch_AllocByType_L494B		;4943
	jp nc,Ch_AllocByType_L4952		;4945
	jp Ch_AllocByType_L495E		;4948
Ch_AllocByType_L494B:
	ld a,(0f9dbh)		;494b
	cp b			;494e
	jp c,Ch_AllocByType_L495E		;494f
Ch_AllocByType_L4952:
	ld d,c			;4952
	ld a,(0f9d9h)		;4953
	ld e,a			;4956
	ld a,(0f9dbh)		;4957
	ld b,a			;495a
	ld hl,wFM_Ch3_F9D5		;495b
Ch_AllocByType_L495E:
	ld ix,wFM_Ch4_FA3C		;495e
	exx			;4962
	ld hl,(wFM_Ch4_FA3C)		;4963
	and a			;4966
	sbc hl,de		;4967
	exx			;4969
	jr nz,Ch_AllocByType_L4984		;496a
	ld a,(wNoteTmp_FBF5)		;496c
	sub (ix+008h)		;496f
	jp nz,Ch_AllocByType_L4984		;4972
	ld a,(0fa41h)		;4975
	and a			;4978
	jp z,Ch_AllocByType_L4984		;4979
	ld hl,wFM_Ch4_FA3C		;497c
	ld d,003h		;497f
	jp Ch_AllocByType_L49CA		;4981
Ch_AllocByType_L4984:
	bit 1,(iy+009h)		;4984
	jr z,Ch_AllocByType_L4991		;4988
	bit 1,(ix+003h)		;498a
	jp z,Ch_AllocByType_L49CA		;498e
Ch_AllocByType_L4991:
	ld c,001h		;4991
	ld a,(0fa41h)		;4993
	and a			;4996
	jr nz,Ch_AllocByType_L49A3		;4997
	inc c			;4999
	ld a,(0fa43h)		;499a
	sub (ix+006h)		;499d
	jr nc,Ch_AllocByType_L49A3		;49a0
	inc c			;49a2
Ch_AllocByType_L49A3:
	ld a,d			;49a3
	cp c			;49a4
	jp c,Ch_AllocByType_L49BE		;49a5
	jp nz,Ch_AllocByType_L49CA		;49a8
	ld a,(0fa40h)		;49ab
	cp e			;49ae
	jr z,Ch_AllocByType_L49B7		;49af
	jp nc,Ch_AllocByType_L49BE		;49b1
	jp Ch_AllocByType_L49CA		;49b4
Ch_AllocByType_L49B7:
	ld a,(0fa42h)		;49b7
	cp b			;49ba
	jp c,Ch_AllocByType_L49CA		;49bb
Ch_AllocByType_L49BE:
	ld d,c			;49be
	ld a,(0fa40h)		;49bf
	ld e,a			;49c2
	ld a,(0fa42h)		;49c3
	ld b,a			;49c6
	ld hl,wFM_Ch4_FA3C		;49c7
Ch_AllocByType_L49CA:
	ex de,hl			;49ca
	defb 0ddh,06bh	;ld ixl,e		;49cb
	defb 0ddh,062h	;ld ixh,d		;49cd
	ld a,h			;49cf
	and a			;49d0
	ret z			;49d1
	cp 002h		;49d2
	jr nc,Ch_AllocByType_L49E0		;49d4
	ld a,l			;49d6
	cp (iy+002h)		;49d7
	ccf			;49da
	ret nc			;49db
	ld (ix+005h),000h		;49dc
Ch_AllocByType_L49E0:
	ld l,(iy+000h)		;49e0
	ld h,(iy+001h)		;49e3
	ld (iy+000h),e		;49e6
	ld (iy+001h),d		;49e9
	defb 0fdh,05dh	;ld e,iyl		;49ec
	defb 0fdh,054h	;ld d,iyh		;49ee
	ld (ix+000h),e		;49f0
	ld (ix+001h),d		;49f3
	ld a,l			;49f6
	or h			;49f7
	jp z,Ch_AllocByType_L4A01		;49f8
	ld de,00004h		;49fb
	add hl,de			;49fe
	ld (hl),0ffh		;49ff
Ch_AllocByType_L4A01:
	scf			;4a01
	ret			;4a02
Ch_NoteOn_4A03:
	call Instr_ResolvePtr_4B5D		;4a03
	ld (0fbf9h),hl		;4a06
	ld a,b			;4a09
	ld (0fbfbh),a		;4a0a
	bit 0,(iy+009h)		;4a0d
	jr z,Ch_NoteOn_L4A30		;4a11
	call Ch_KeyOn_4AE2		;4a13
	jr c,Ch_NoteOn_L4A26		;4a16
	bit 1,(iy+009h)		;4a18
	jr nz,Ch_NoteOn_L4A30		;4a1c
	bit 5,(iy+009h)		;4a1e
	jr z,Ch_NoteOn_L4A30		;4a22
	and a			;4a24
	ret			;4a25
Ch_NoteOn_L4A26:
	ld hl,(0fbf9h)		;4a26
	ld a,(hl)			;4a29
	cp (ix+002h)		;4a2a
	jp z,Ch_NoteOn_L4A34		;4a2d
Ch_NoteOn_L4A30:
	call Ch_AllocByType_4389		;4a30
	ret nc			;4a33
Ch_NoteOn_L4A34:
	set 5,(iy+009h)		;4a34
	ld a,(iy+002h)		;4a38
	ld (ix+004h),a		;4a3b
	ld a,(wNoteTmp_FBF5)		;4a3e
	ld (ix+008h),a		;4a41
	ld a,(wNoteGate_FBF8)		;4a44
	ld (ix+00bh),a		;4a47
	ld a,(wNoteLen_FBF6)		;4a4a
	ld (ix+007h),a		;4a4d
	ld (ix+006h),000h		;4a50
	ld b,(ix+005h)		;4a54
	ld a,(wNoteVol_FBF7)		;4a57
	sub 001h		;4a5a
	adc a,001h		;4a5c
	ld (ix+005h),a		;4a5e
	ld a,(iy+009h)		;4a61
	ld (ix+003h),a		;4a64
	inc b			;4a67
	dec b			;4a68
	jp nz,Ch_DispatchPitch_4AC4		;4a69
	ld e,(iy+00ah)		;4a6c
	ld d,(iy+00bh)		;4a6f
	ld l,(ix+00ch)		;4a72
	ld h,(ix+00dh)		;4a75
	and a			;4a78
	sbc hl,de		;4a79
	jp z,Ch_DispatchKeyOn_4AA8		;4a7b
	ld (ix+00ch),e		;4a7e
	ld (ix+00dh),d		;4a81
	ld hl,(0fbf9h)		;4a84
	ld a,(0fbfbh)		;4a87
	ld b,a			;4a8a
	exx			;4a8b
	ld hl,Ch_DispatchKeyOn_4AA8		;4a8c
	push hl			;4a8f
	ld a,(ix+002h)		;4a90
	add a,a			;4a93
	ld l,a			;4a94
	ld h,000h		;4a95
	ld de,ChTypeJumpTable0_4AA0		;4a97
	add hl,de			;4a9a
	ld e,(hl)			;4a9b
	inc hl			;4a9c
	ld d,(hl)			;4a9d
	ex de,hl			;4a9e
	jp (hl)			;4a9f
