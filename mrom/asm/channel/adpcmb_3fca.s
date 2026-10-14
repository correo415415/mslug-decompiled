; adpcmb_3fca.s - M-ROM fixed_0000 [0x3fca..0x4aa0)
; Carved from mrom/asm/raw/fixed_0000.s by mrom/tools/split_region.py.
; Assembled through mrom/asm/fixed_0000.s; must stay byte-identical.

ADPCMB_Init_3FCA:
	ld ix,wPCMB_Ch_FB9B		;3fca
	call sub_3fe0h		;3fce
	ld c,004h		;3fd1
	ld de,01b00h		;3fd3  YM reg 1Bh <- 00h
	rst 8			;3fd6
	ld de,01001h		;3fd7  YM reg 10h <- 01h
	rst 8			;3fda
	ld de,01100h		;3fdb  YM reg 11h <- 00h
	rst 8			;3fde
	ret			;3fdf
sub_3fe0h:
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
	jp z,l423ch		;4037
	cp 004h		;403a
	jp z,l4054h		;403c
	ex af,af'			;403f
	ld a,(wYmStatus_F80A)		;4040
	and 080h		;4043
	jp z,l410ah		;4045
	ex af,af'			;4048
	dec a			;4049
	jp z,l40fah		;404a
	dec a			;404d
	jp z,l4083h		;404e
	jp l4069h		;4051
l4054h:
	ld hl,(0fbb2h)		;4054
	ld bc,(0fbb4h)		;4057
	ld a,b			;405b
	or c			;405c
	jp z,l4071h		;405d
	dec bc			;4060
	ld a,003h		;4061
	ld (wPCMB_Shared_FBBA),a		;4063
	jp l40b9h		;4066
l4069h:
	ld hl,(0fbb2h)		;4069
	ld bc,(0fbb4h)		;406c
	add hl,bc			;4070
l4071h:
	ld bc,(0fbb6h)		;4071
	ld a,b			;4075
	or c			;4076
	jp z,l40abh		;4077
	dec bc			;407a
	ld a,002h		;407b
	ld (wPCMB_Shared_FBBA),a		;407d
	jp l40b9h		;4080
l4083h:
	ld a,(0fbbbh)		;4083
	cp 0ffh		;4086
	jp z,l410ah		;4088
	and a			;408b
	jr z,l409eh		;408c
	dec a			;408e
	ld (0fbbbh),a		;408f
	ld c,004h		;4092
	ld de,01c80h		;4094  YM reg 1Ch <- 80h
	rst 8			;4097
	ld e,000h		;4098
	rst 8			;409a
	jp l410ah		;409b
l409eh:
	ld hl,(0fbb2h)		;409e
	ld bc,(0fbb4h)		;40a1
	add hl,bc			;40a5
	ld bc,(0fbb6h)		;40a6
	add hl,bc			;40aa
l40abh:
	ld bc,(0fbb8h)		;40ab
	ld a,b			;40af
	or c			;40b0
	jp z,l40fah		;40b1
	ld a,001h		;40b4
	ld (wPCMB_Shared_FBBA),a		;40b6
l40b9h:
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
	jr nz,l40f4h		;40f0
	ld e,090h		;40f2
l40f4h:
	ld d,010h		;40f4
	rst 8			;40f6
	jp l410ah		;40f7
l40fah:
	ld de,01b00h		;40fa  YM reg 1Bh <- 00h
	ld c,004h		;40fd
	rst 8			;40ff
	xor a			;4100
	ld (wPCMB_Shared_FBBA),a		;4101
	ld (0fbcbh),a		;4104
	jp l423ch		;4107
l410ah:
	ld a,(0fbbch)		;410a
	and a			;410d
	jr nz,l4113h		;410e
	ld a,(iy+00fh)		;4110
l4113h:
	rrca			;4113
	rrca			;4114
	ld hl,0fbbdh		;4115
	cp (hl)			;4118
	jr z,l4122h		;4119
	ld (hl),a			;411b
	ld e,a			;411c
	ld d,011h		;411d
	ld c,004h		;411f
	rst 8			;4121
l4122h:
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
	jr z,l419ch		;415c
	ld (0fba9h),bc		;415e
	ld de,(0fbafh)		;4162
	ld h,017h		;4166
	ld l,c			;4168
	ld c,(hl)			;4169
	push de			;416a
	call Util_TableLookup_1F54		;416b
	pop de			;416e
	add hl,de			;416f
	jr nc,l4177h		;4170
	rr h		;4172
	rr l		;4174
	inc b			;4176
l4177h:
	ld a,b			;4177
	and a			;4178
	jp z,l4193h		;4179
	jp p,l418ah		;417c
l417fh:
	srl h		;417f
	rr l		;4181
	inc a			;4183
	jp nz,l417fh		;4184
	jp l4193h		;4187
l418ah:
	add hl,hl			;418a
	jr nc,l418fh		;418b
	sbc hl,hl		;418d
l418fh:
	dec a			;418f
	jp nz,l418ah		;4190
l4193h:
	ld e,l			;4193
	ld d,019h		;4194
	ld c,004h		;4196
	rst 8			;4198
	inc d			;4199
	ld e,h			;419a
	rst 8			;419b
l419ch:
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
	jr z,l41cch		;41b5
	inc l			;41b7
	jr z,l41cch		;41b8
	ex af,af'			;41ba
	dec l			;41bb
	ld a,l			;41bc
	jr z,l41cch		;41bd
	ld h,014h		;41bf
	ld a,(hl)			;41c1
	ex af,af'			;41c2
	ld l,a			;41c3
	ex af,af'			;41c4
	add a,(hl)			;41c5
	ld l,a			;41c6
	sbc a,a			;41c7
	jr nc,l41cch		;41c8
	inc h			;41ca
	ld a,(hl)			;41cb
l41cch:
	ld l,a			;41cc
	ld a,(0fba6h)		;41cd
	and a			;41d0
	jr z,l41e8h		;41d1
	inc l			;41d3
	jr z,l41e8h		;41d4
	ex af,af'			;41d6
	dec l			;41d7
	ld a,l			;41d8
	jr z,l41e8h		;41d9
	ld h,014h		;41db
	ld a,(hl)			;41dd
	ex af,af'			;41de
	ld l,a			;41df
	ex af,af'			;41e0
	add a,(hl)			;41e1
	ld l,a			;41e2
	sbc a,a			;41e3
	jr nc,l41e8h		;41e4
	inc h			;41e6
	ld a,(hl)			;41e7
l41e8h:
	ld l,(iy+012h)		;41e8
	and a			;41eb
	jr z,l4203h		;41ec
	inc l			;41ee
	jr z,l4203h		;41ef
	ex af,af'			;41f1
	dec l			;41f2
	ld a,l			;41f3
	jr z,l4203h		;41f4
	ld h,014h		;41f6
	ld a,(hl)			;41f8
	ex af,af'			;41f9
	ld l,a			;41fa
	ex af,af'			;41fb
	add a,(hl)			;41fc
	ld l,a			;41fd
	sbc a,a			;41fe
	jr nc,l4203h		;41ff
	inc h			;4201
	ld a,(hl)			;4202
l4203h:
	ld l,a			;4203
	ld a,0ffh		;4204
	bit 1,(iy+009h)		;4206
	jp z,l4210h		;420a
	ld a,(wMasterVolume_F805)		;420d
l4210h:
	and a			;4210
	jr z,l4228h		;4211
	inc l			;4213
	jr z,l4228h		;4214
	ex af,af'			;4216
	dec l			;4217
	ld a,l			;4218
	jr z,l4228h		;4219
	ld h,014h		;421b
	ld a,(hl)			;421d
	ex af,af'			;421e
	ld l,a			;421f
	ex af,af'			;4220
	add a,(hl)			;4221
	ld l,a			;4222
	sbc a,a			;4223
	jr nc,l4228h		;4224
	inc h			;4226
	ld a,(hl)			;4227
l4228h:
	ld hl,0fbabh		;4228
	cp (hl)			;422b
	ld (hl),a			;422c
	jr z,l423ch		;422d
	add a,a			;422f
	ld e,080h		;4230
	add a,e			;4232
	adc a,e			;4233
	ld e,a			;4234
	ld d,01bh		;4235
	ld c,004h		;4237
	ld c,004h		;4239
	rst 8			;423b
l423ch:
	ld c,001h		;423c
	bit 1,(iy+009h)		;423e
	jr z,l4248h		;4242
	ld a,(wTempoStep_F809)		;4244
	ld c,a			;4247
l4248h:
	ld hl,0fba1h		;4248
	ld a,(hl)			;424b
	add a,c			;424c
	sbc a,000h		;424d
	ld (hl),a			;424f
	ld hl,0fba0h		;4250
	ld a,(hl)			;4253
	and a			;4254
	jp z,l4269h		;4255
	sub c			;4258
	ld (hl),a			;4259
	jr nz,l427eh		;425a
	bit 4,(iy+009h)		;425c
	jr z,l4278h		;4260
	set 4,(ix+003h)		;4262
	jp l427eh		;4266
l4269h:
	bit 4,(iy+009h)		;4269
	jr nz,l427eh		;426d
	ld hl,0fb9eh		;426f
	bit 4,(hl)		;4272
	jr z,l427eh		;4274
	res 4,(hl)		;4276
l4278h:
	ld hl,0fbbeh		;4278
	call Env_Release_2219		;427b
l427eh:
	ret			;427e
l427fh:
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
l42cah:
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
l42efh:
	ld a,004h		;42ef
	ld (wPCMB_Shared_FBBA),a		;42f1
	ld a,(0fbach)		;42f4
	ld b,a			;42f7
	ld a,(wCurBank_F811)		;42f8
	cp b			;42fb
	jp z,l4319h		;42fc
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
l4319h:
	ld a,(0fba3h)		;4319
	ld hl,(0fbadh)		;431c
	ld de,00006h		;431f  constant
l4322h:
	cp (hl)			;4322
	jr z,l432bh		;4323
	jr c,l432bh		;4325
	add hl,de			;4327
	jp l4322h		;4328
l432bh:
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
sub_4389h:
	ld hl,(0fbf9h)		;4389
	ld a,(hl)			;438c
	cp 000h		;438d
	jp z,l46bbh		;438f
	cp 001h		;4392
	jp z,l480eh		;4394
	cp 002h		;4397
	jp z,l4424h		;4399
	cp 003h		;439c
	jp z,l43a9h		;439e
	ld hl,00000h		;43a1  constant
	push hl			;43a4
	pop ix		;43a5
	and a			;43a7
	ret			;43a8
l43a9h:
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
	jr nz,l43dbh		;43c1
	ld a,(wNoteTmp_FBF5)		;43c3
	sub (ix+008h)		;43c6
	jp nz,l43dbh		;43c9
	ld a,(0fba0h)		;43cc
	and a			;43cf
	jp z,l43dbh		;43d0
	ld hl,wPCMB_Ch_FB9B		;43d3
	ld d,003h		;43d6
	jp l49cah		;43d8
l43dbh:
	bit 1,(iy+009h)		;43db
	jr z,l43e8h		;43df
	bit 1,(ix+003h)		;43e1
	jp z,l4421h		;43e5
l43e8h:
	ld c,001h		;43e8
	ld a,(0fba0h)		;43ea
	and a			;43ed
	jr nz,l43fah		;43ee
	inc c			;43f0
	ld a,(0fba2h)		;43f1
	sub (ix+006h)		;43f4
	jr nc,l43fah		;43f7
	inc c			;43f9
l43fah:
	ld a,d			;43fa
	cp c			;43fb
	jp c,l4415h		;43fc
	jp nz,l4421h		;43ff
	ld a,(0fb9fh)		;4402
	cp e			;4405
	jr z,l440eh		;4406
	jp nc,l4415h		;4408
	jp l4421h		;440b
l440eh:
	ld a,(0fba1h)		;440e
	cp b			;4411
	jp c,l4421h		;4412
l4415h:
	ld d,c			;4415
	ld a,(0fb9fh)		;4416
	ld e,a			;4419
	ld a,(0fba1h)		;441a
	ld b,a			;441d
	ld hl,wPCMB_Ch_FB9B		;441e
l4421h:
	jp l49cah		;4421
l4424h:
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
	jr nz,l4456h		;443c
	ld a,(wNoteTmp_FBF5)		;443e
	sub (ix+008h)		;4441
	jp nz,l4456h		;4444
	ld a,(0faaah)		;4447
	and a			;444a
	jp z,l4456h		;444b
	ld hl,wPCMA_Ch1_FAA5		;444e
	ld d,003h		;4451
	jp l49cah		;4453
l4456h:
	bit 1,(iy+009h)		;4456
	jr z,l4463h		;445a
	bit 1,(ix+003h)		;445c
	jp z,l449ch		;4460
l4463h:
	ld c,001h		;4463
	ld a,(0faaah)		;4465
	and a			;4468
	jr nz,l4475h		;4469
	inc c			;446b
	ld a,(0faach)		;446c
	sub (ix+006h)		;446f
	jr nc,l4475h		;4472
	inc c			;4474
l4475h:
	ld a,d			;4475
	cp c			;4476
	jp c,l4490h		;4477
	jp nz,l449ch		;447a
	ld a,(0faa9h)		;447d
	cp e			;4480
	jr z,l4489h		;4481
	jp nc,l4490h		;4483
	jp l449ch		;4486
l4489h:
	ld a,(0faabh)		;4489
	cp b			;448c
	jp c,l449ch		;448d
l4490h:
	ld d,c			;4490
	ld a,(0faa9h)		;4491
	ld e,a			;4494
	ld a,(0faabh)		;4495
	ld b,a			;4498
	ld hl,wPCMA_Ch1_FAA5		;4499
l449ch:
	ld ix,wPCMA_Ch2_FACE		;449c
	exx			;44a0
	ld hl,(wPCMA_Ch2_FACE)		;44a1
	and a			;44a4
	sbc hl,de		;44a5
	exx			;44a7
	jr nz,l44c2h		;44a8
	ld a,(wNoteTmp_FBF5)		;44aa
	sub (ix+008h)		;44ad
	jp nz,l44c2h		;44b0
	ld a,(0fad3h)		;44b3
	and a			;44b6
	jp z,l44c2h		;44b7
	ld hl,wPCMA_Ch2_FACE		;44ba
	ld d,003h		;44bd
	jp l49cah		;44bf
l44c2h:
	bit 1,(iy+009h)		;44c2
	jr z,l44cfh		;44c6
	bit 1,(ix+003h)		;44c8
	jp z,l4508h		;44cc
l44cfh:
	ld c,001h		;44cf
	ld a,(0fad3h)		;44d1
	and a			;44d4
	jr nz,l44e1h		;44d5
	inc c			;44d7
	ld a,(0fad5h)		;44d8
	sub (ix+006h)		;44db
	jr nc,l44e1h		;44de
	inc c			;44e0
l44e1h:
	ld a,d			;44e1
	cp c			;44e2
	jp c,l44fch		;44e3
	jp nz,l4508h		;44e6
	ld a,(0fad2h)		;44e9
	cp e			;44ec
	jr z,l44f5h		;44ed
	jp nc,l44fch		;44ef
	jp l4508h		;44f2
l44f5h:
	ld a,(0fad4h)		;44f5
	cp b			;44f8
	jp c,l4508h		;44f9
l44fch:
	ld d,c			;44fc
	ld a,(0fad2h)		;44fd
	ld e,a			;4500
	ld a,(0fad4h)		;4501
	ld b,a			;4504
	ld hl,wPCMA_Ch2_FACE		;4505
l4508h:
	ld ix,wPCMA_Ch3_FAF7		;4508
	exx			;450c
	ld hl,(wPCMA_Ch3_FAF7)		;450d
	and a			;4510
	sbc hl,de		;4511
	exx			;4513
	jr nz,l452eh		;4514
	ld a,(wNoteTmp_FBF5)		;4516
	sub (ix+008h)		;4519
	jp nz,l452eh		;451c
	ld a,(0fafch)		;451f
	and a			;4522
	jp z,l452eh		;4523
	ld hl,wPCMA_Ch3_FAF7		;4526
	ld d,003h		;4529
	jp l49cah		;452b
l452eh:
	bit 1,(iy+009h)		;452e
	jr z,l453bh		;4532
	bit 1,(ix+003h)		;4534
	jp z,l4574h		;4538
l453bh:
	ld c,001h		;453b
	ld a,(0fafch)		;453d
	and a			;4540
	jr nz,l454dh		;4541
	inc c			;4543
	ld a,(0fafeh)		;4544
	sub (ix+006h)		;4547
	jr nc,l454dh		;454a
	inc c			;454c
l454dh:
	ld a,d			;454d
	cp c			;454e
	jp c,l4568h		;454f
	jp nz,l4574h		;4552
	ld a,(0fafbh)		;4555
	cp e			;4558
	jr z,l4561h		;4559
	jp nc,l4568h		;455b
	jp l4574h		;455e
l4561h:
	ld a,(0fafdh)		;4561
	cp b			;4564
	jp c,l4574h		;4565
l4568h:
	ld d,c			;4568
	ld a,(0fafbh)		;4569
	ld e,a			;456c
	ld a,(0fafdh)		;456d
	ld b,a			;4570
	ld hl,wPCMA_Ch3_FAF7		;4571
l4574h:
	ld ix,wPCMA_Ch4_FB20		;4574
	exx			;4578
	ld hl,(wPCMA_Ch4_FB20)		;4579
	and a			;457c
	sbc hl,de		;457d
	exx			;457f
	jr nz,l459ah		;4580
	ld a,(wNoteTmp_FBF5)		;4582
	sub (ix+008h)		;4585
	jp nz,l459ah		;4588
	ld a,(0fb25h)		;458b
	and a			;458e
	jp z,l459ah		;458f
	ld hl,wPCMA_Ch4_FB20		;4592
	ld d,003h		;4595
	jp l49cah		;4597
l459ah:
	bit 1,(iy+009h)		;459a
	jr z,l45a7h		;459e
	bit 1,(ix+003h)		;45a0
	jp z,l45e0h		;45a4
l45a7h:
	ld c,001h		;45a7
	ld a,(0fb25h)		;45a9
	and a			;45ac
	jr nz,l45b9h		;45ad
	inc c			;45af
	ld a,(0fb27h)		;45b0
	sub (ix+006h)		;45b3
	jr nc,l45b9h		;45b6
	inc c			;45b8
l45b9h:
	ld a,d			;45b9
	cp c			;45ba
	jp c,l45d4h		;45bb
	jp nz,l45e0h		;45be
	ld a,(0fb24h)		;45c1
	cp e			;45c4
	jr z,l45cdh		;45c5
	jp nc,l45d4h		;45c7
	jp l45e0h		;45ca
l45cdh:
	ld a,(0fb26h)		;45cd
	cp b			;45d0
	jp c,l45e0h		;45d1
l45d4h:
	ld d,c			;45d4
	ld a,(0fb24h)		;45d5
	ld e,a			;45d8
	ld a,(0fb26h)		;45d9
	ld b,a			;45dc
	ld hl,wPCMA_Ch4_FB20		;45dd
l45e0h:
	ld ix,wPCMA_Ch5_FB49		;45e0
	exx			;45e4
	ld hl,(wPCMA_Ch5_FB49)		;45e5
	and a			;45e8
	sbc hl,de		;45e9
	exx			;45eb
	jr nz,l4606h		;45ec
	ld a,(wNoteTmp_FBF5)		;45ee
	sub (ix+008h)		;45f1
	jp nz,l4606h		;45f4
	ld a,(0fb4eh)		;45f7
	and a			;45fa
	jp z,l4606h		;45fb
	ld hl,wPCMA_Ch5_FB49		;45fe
	ld d,003h		;4601
	jp l49cah		;4603
l4606h:
	bit 1,(iy+009h)		;4606
	jr z,l4613h		;460a
	bit 1,(ix+003h)		;460c
	jp z,l464ch		;4610
l4613h:
	ld c,001h		;4613
	ld a,(0fb4eh)		;4615
	and a			;4618
	jr nz,l4625h		;4619
	inc c			;461b
	ld a,(0fb50h)		;461c
	sub (ix+006h)		;461f
	jr nc,l4625h		;4622
	inc c			;4624
l4625h:
	ld a,d			;4625
	cp c			;4626
	jp c,l4640h		;4627
	jp nz,l464ch		;462a
	ld a,(0fb4dh)		;462d
	cp e			;4630
	jr z,l4639h		;4631
	jp nc,l4640h		;4633
	jp l464ch		;4636
l4639h:
	ld a,(0fb4fh)		;4639
	cp b			;463c
	jp c,l464ch		;463d
l4640h:
	ld d,c			;4640
	ld a,(0fb4dh)		;4641
	ld e,a			;4644
	ld a,(0fb4fh)		;4645
	ld b,a			;4648
	ld hl,wPCMA_Ch5_FB49		;4649
l464ch:
	ld ix,wPCMA_Ch6_FB72		;464c
	exx			;4650
	ld hl,(wPCMA_Ch6_FB72)		;4651
	and a			;4654
	sbc hl,de		;4655
	exx			;4657
	jr nz,l4672h		;4658
	ld a,(wNoteTmp_FBF5)		;465a
	sub (ix+008h)		;465d
	jp nz,l4672h		;4660
	ld a,(0fb77h)		;4663
	and a			;4666
	jp z,l4672h		;4667
	ld hl,wPCMA_Ch6_FB72		;466a
	ld d,003h		;466d
	jp l49cah		;466f
l4672h:
	bit 1,(iy+009h)		;4672
	jr z,l467fh		;4676
	bit 1,(ix+003h)		;4678
	jp z,l46b8h		;467c
l467fh:
	ld c,001h		;467f
	ld a,(0fb77h)		;4681
	and a			;4684
	jr nz,l4691h		;4685
	inc c			;4687
	ld a,(0fb79h)		;4688
	sub (ix+006h)		;468b
	jr nc,l4691h		;468e
	inc c			;4690
l4691h:
	ld a,d			;4691
	cp c			;4692
	jp c,l46ach		;4693
	jp nz,l46b8h		;4696
	ld a,(0fb76h)		;4699
	cp e			;469c
	jr z,l46a5h		;469d
	jp nc,l46ach		;469f
	jp l46b8h		;46a2
l46a5h:
	ld a,(0fb78h)		;46a5
	cp b			;46a8
	jp c,l46b8h		;46a9
l46ach:
	ld d,c			;46ac
	ld a,(0fb76h)		;46ad
	ld e,a			;46b0
	ld a,(0fb78h)		;46b1
	ld b,a			;46b4
	ld hl,wPCMA_Ch6_FB72		;46b5
l46b8h:
	jp l49cah		;46b8
l46bbh:
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
	jr nz,l46edh		;46d3
	ld a,(wNoteTmp_FBF5)		;46d5
	sub (ix+008h)		;46d8
	jp nz,l46edh		;46db
	ld a,(0f824h)		;46de
	and a			;46e1
	jp z,l46edh		;46e2
	ld hl,wSSG_ChA_F81F		;46e5
	ld d,003h		;46e8
	jp l49cah		;46ea
l46edh:
	bit 1,(iy+009h)		;46ed
	jr z,l46fah		;46f1
	bit 1,(ix+003h)		;46f3
	jp z,l4733h		;46f7
l46fah:
	ld c,001h		;46fa
	ld a,(0f824h)		;46fc
	and a			;46ff
	jr nz,l470ch		;4700
	inc c			;4702
	ld a,(0f826h)		;4703
	sub (ix+006h)		;4706
	jr nc,l470ch		;4709
	inc c			;470b
l470ch:
	ld a,d			;470c
	cp c			;470d
	jp c,l4727h		;470e
	jp nz,l4733h		;4711
	ld a,(0f823h)		;4714
	cp e			;4717
	jr z,l4720h		;4718
	jp nc,l4727h		;471a
	jp l4733h		;471d
l4720h:
	ld a,(0f825h)		;4720
	cp b			;4723
	jp c,l4733h		;4724
l4727h:
	ld d,c			;4727
	ld a,(0f823h)		;4728
	ld e,a			;472b
	ld a,(0f825h)		;472c
	ld b,a			;472f
	ld hl,wSSG_ChA_F81F		;4730
l4733h:
	ld ix,wSSG_ChB_F86B		;4733
	exx			;4737
	ld hl,(wSSG_ChB_F86B)		;4738
	and a			;473b
	sbc hl,de		;473c
	exx			;473e
	jr nz,l4759h		;473f
	ld a,(wNoteTmp_FBF5)		;4741
	sub (ix+008h)		;4744
	jp nz,l4759h		;4747
	ld a,(0f870h)		;474a
	and a			;474d
	jp z,l4759h		;474e
	ld hl,wSSG_ChB_F86B		;4751
	ld d,003h		;4754
	jp l49cah		;4756
l4759h:
	bit 1,(iy+009h)		;4759
	jr z,l4766h		;475d
	bit 1,(ix+003h)		;475f
	jp z,l479fh		;4763
l4766h:
	ld c,001h		;4766
	ld a,(0f870h)		;4768
	and a			;476b
	jr nz,l4778h		;476c
	inc c			;476e
	ld a,(0f872h)		;476f
	sub (ix+006h)		;4772
	jr nc,l4778h		;4775
	inc c			;4777
l4778h:
	ld a,d			;4778
	cp c			;4779
	jp c,l4793h		;477a
	jp nz,l479fh		;477d
	ld a,(0f86fh)		;4780
	cp e			;4783
	jr z,l478ch		;4784
	jp nc,l4793h		;4786
	jp l479fh		;4789
l478ch:
	ld a,(0f871h)		;478c
	cp b			;478f
	jp c,l479fh		;4790
l4793h:
	ld d,c			;4793
	ld a,(0f86fh)		;4794
	ld e,a			;4797
	ld a,(0f871h)		;4798
	ld b,a			;479b
	ld hl,wSSG_ChB_F86B		;479c
l479fh:
	ld ix,wSSG_ChC_F8B7		;479f
	exx			;47a3
	ld hl,(wSSG_ChC_F8B7)		;47a4
	and a			;47a7
	sbc hl,de		;47a8
	exx			;47aa
	jr nz,l47c5h		;47ab
	ld a,(wNoteTmp_FBF5)		;47ad
	sub (ix+008h)		;47b0
	jp nz,l47c5h		;47b3
	ld a,(0f8bch)		;47b6
	and a			;47b9
	jp z,l47c5h		;47ba
	ld hl,wSSG_ChC_F8B7		;47bd
	ld d,003h		;47c0
	jp l49cah		;47c2
l47c5h:
	bit 1,(iy+009h)		;47c5
	jr z,l47d2h		;47c9
	bit 1,(ix+003h)		;47cb
	jp z,l480bh		;47cf
l47d2h:
	ld c,001h		;47d2
	ld a,(0f8bch)		;47d4
	and a			;47d7
	jr nz,l47e4h		;47d8
	inc c			;47da
	ld a,(0f8beh)		;47db
	sub (ix+006h)		;47de
	jr nc,l47e4h		;47e1
	inc c			;47e3
l47e4h:
	ld a,d			;47e4
	cp c			;47e5
	jp c,l47ffh		;47e6
	jp nz,l480bh		;47e9
	ld a,(0f8bbh)		;47ec
	cp e			;47ef
	jr z,l47f8h		;47f0
	jp nc,l47ffh		;47f2
	jp l480bh		;47f5
l47f8h:
	ld a,(0f8bdh)		;47f8
	cp b			;47fb
	jp c,l480bh		;47fc
l47ffh:
	ld d,c			;47ff
	ld a,(0f8bbh)		;4800
	ld e,a			;4803
	ld a,(0f8bdh)		;4804
	ld b,a			;4807
	ld hl,wSSG_ChC_F8B7		;4808
l480bh:
	jp l49cah		;480b
l480eh:
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
	jr nz,l4840h		;4826
	ld a,(wNoteTmp_FBF5)		;4828
	sub (ix+008h)		;482b
	jp nz,l4840h		;482e
	ld a,(0f90ch)		;4831
	and a			;4834
	jp z,l4840h		;4835
	ld hl,wFM_Ch1_F907		;4838
	ld d,003h		;483b
	jp l49cah		;483d
l4840h:
	bit 1,(iy+009h)		;4840
	jr z,l484dh		;4844
	bit 1,(ix+003h)		;4846
	jp z,l4886h		;484a
l484dh:
	ld c,001h		;484d
	ld a,(0f90ch)		;484f
	and a			;4852
	jr nz,l485fh		;4853
	inc c			;4855
	ld a,(0f90eh)		;4856
	sub (ix+006h)		;4859
	jr nc,l485fh		;485c
	inc c			;485e
l485fh:
	ld a,d			;485f
	cp c			;4860
	jp c,l487ah		;4861
	jp nz,l4886h		;4864
	ld a,(0f90bh)		;4867
	cp e			;486a
	jr z,l4873h		;486b
	jp nc,l487ah		;486d
	jp l4886h		;4870
l4873h:
	ld a,(0f90dh)		;4873
	cp b			;4876
	jp c,l4886h		;4877
l487ah:
	ld d,c			;487a
	ld a,(0f90bh)		;487b
	ld e,a			;487e
	ld a,(0f90dh)		;487f
	ld b,a			;4882
	ld hl,wFM_Ch1_F907		;4883
l4886h:
	ld ix,wFM_Ch2_F96E		;4886
	exx			;488a
	ld hl,(wFM_Ch2_F96E)		;488b
	and a			;488e
	sbc hl,de		;488f
	exx			;4891
	jr nz,l48ach		;4892
	ld a,(wNoteTmp_FBF5)		;4894
	sub (ix+008h)		;4897
	jp nz,l48ach		;489a
	ld a,(0f973h)		;489d
	and a			;48a0
	jp z,l48ach		;48a1
	ld hl,wFM_Ch2_F96E		;48a4
	ld d,003h		;48a7
	jp l49cah		;48a9
l48ach:
	bit 1,(iy+009h)		;48ac
	jr z,l48b9h		;48b0
	bit 1,(ix+003h)		;48b2
	jp z,l48f2h		;48b6
l48b9h:
	ld c,001h		;48b9
	ld a,(0f973h)		;48bb
	and a			;48be
	jr nz,l48cbh		;48bf
	inc c			;48c1
	ld a,(0f975h)		;48c2
	sub (ix+006h)		;48c5
	jr nc,l48cbh		;48c8
	inc c			;48ca
l48cbh:
	ld a,d			;48cb
	cp c			;48cc
	jp c,l48e6h		;48cd
	jp nz,l48f2h		;48d0
	ld a,(0f972h)		;48d3
	cp e			;48d6
	jr z,l48dfh		;48d7
	jp nc,l48e6h		;48d9
	jp l48f2h		;48dc
l48dfh:
	ld a,(0f974h)		;48df
	cp b			;48e2
	jp c,l48f2h		;48e3
l48e6h:
	ld d,c			;48e6
	ld a,(0f972h)		;48e7
	ld e,a			;48ea
	ld a,(0f974h)		;48eb
	ld b,a			;48ee
	ld hl,wFM_Ch2_F96E		;48ef
l48f2h:
	ld ix,wFM_Ch3_F9D5		;48f2
	exx			;48f6
	ld hl,(wFM_Ch3_F9D5)		;48f7
	and a			;48fa
	sbc hl,de		;48fb
	exx			;48fd
	jr nz,l4918h		;48fe
	ld a,(wNoteTmp_FBF5)		;4900
	sub (ix+008h)		;4903
	jp nz,l4918h		;4906
	ld a,(0f9dah)		;4909
	and a			;490c
	jp z,l4918h		;490d
	ld hl,wFM_Ch3_F9D5		;4910
	ld d,003h		;4913
	jp l49cah		;4915
l4918h:
	bit 1,(iy+009h)		;4918
	jr z,l4925h		;491c
	bit 1,(ix+003h)		;491e
	jp z,l495eh		;4922
l4925h:
	ld c,001h		;4925
	ld a,(0f9dah)		;4927
	and a			;492a
	jr nz,l4937h		;492b
	inc c			;492d
	ld a,(0f9dch)		;492e
	sub (ix+006h)		;4931
	jr nc,l4937h		;4934
	inc c			;4936
l4937h:
	ld a,d			;4937
	cp c			;4938
	jp c,l4952h		;4939
	jp nz,l495eh		;493c
	ld a,(0f9d9h)		;493f
	cp e			;4942
	jr z,l494bh		;4943
	jp nc,l4952h		;4945
	jp l495eh		;4948
l494bh:
	ld a,(0f9dbh)		;494b
	cp b			;494e
	jp c,l495eh		;494f
l4952h:
	ld d,c			;4952
	ld a,(0f9d9h)		;4953
	ld e,a			;4956
	ld a,(0f9dbh)		;4957
	ld b,a			;495a
	ld hl,wFM_Ch3_F9D5		;495b
l495eh:
	ld ix,wFM_Ch4_FA3C		;495e
	exx			;4962
	ld hl,(wFM_Ch4_FA3C)		;4963
	and a			;4966
	sbc hl,de		;4967
	exx			;4969
	jr nz,l4984h		;496a
	ld a,(wNoteTmp_FBF5)		;496c
	sub (ix+008h)		;496f
	jp nz,l4984h		;4972
	ld a,(0fa41h)		;4975
	and a			;4978
	jp z,l4984h		;4979
	ld hl,wFM_Ch4_FA3C		;497c
	ld d,003h		;497f
	jp l49cah		;4981
l4984h:
	bit 1,(iy+009h)		;4984
	jr z,l4991h		;4988
	bit 1,(ix+003h)		;498a
	jp z,l49cah		;498e
l4991h:
	ld c,001h		;4991
	ld a,(0fa41h)		;4993
	and a			;4996
	jr nz,l49a3h		;4997
	inc c			;4999
	ld a,(0fa43h)		;499a
	sub (ix+006h)		;499d
	jr nc,l49a3h		;49a0
	inc c			;49a2
l49a3h:
	ld a,d			;49a3
	cp c			;49a4
	jp c,l49beh		;49a5
	jp nz,l49cah		;49a8
	ld a,(0fa40h)		;49ab
	cp e			;49ae
	jr z,l49b7h		;49af
	jp nc,l49beh		;49b1
	jp l49cah		;49b4
l49b7h:
	ld a,(0fa42h)		;49b7
	cp b			;49ba
	jp c,l49cah		;49bb
l49beh:
	ld d,c			;49be
	ld a,(0fa40h)		;49bf
	ld e,a			;49c2
	ld a,(0fa42h)		;49c3
	ld b,a			;49c6
	ld hl,wFM_Ch4_FA3C		;49c7
l49cah:
	ex de,hl			;49ca
	defb 0ddh,06bh	;ld ixl,e		;49cb
	defb 0ddh,062h	;ld ixh,d		;49cd
	ld a,h			;49cf
	and a			;49d0
	ret z			;49d1
	cp 002h		;49d2
	jr nc,l49e0h		;49d4
	ld a,l			;49d6
	cp (iy+002h)		;49d7
	ccf			;49da
	ret nc			;49db
	ld (ix+005h),000h		;49dc
l49e0h:
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
	jp z,l4a01h		;49f8
	ld de,00004h		;49fb
	add hl,de			;49fe
	ld (hl),0ffh		;49ff
l4a01h:
	scf			;4a01
	ret			;4a02
sub_4a03h:
	call Instr_ResolvePtr_4B5D		;4a03
	ld (0fbf9h),hl		;4a06
	ld a,b			;4a09
	ld (0fbfbh),a		;4a0a
	bit 0,(iy+009h)		;4a0d
	jr z,l4a30h		;4a11
	call Ch_KeyOn_4AE2		;4a13
	jr c,l4a26h		;4a16
	bit 1,(iy+009h)		;4a18
	jr nz,l4a30h		;4a1c
	bit 5,(iy+009h)		;4a1e
	jr z,l4a30h		;4a22
	and a			;4a24
	ret			;4a25
l4a26h:
	ld hl,(0fbf9h)		;4a26
	ld a,(hl)			;4a29
	cp (ix+002h)		;4a2a
	jp z,l4a34h		;4a2d
l4a30h:
	call sub_4389h		;4a30
	ret nc			;4a33
l4a34h:
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
	jp nz,l4ac4h		;4a69
	ld e,(iy+00ah)		;4a6c
	ld d,(iy+00bh)		;4a6f
	ld l,(ix+00ch)		;4a72
	ld h,(ix+00dh)		;4a75
	and a			;4a78
	sbc hl,de		;4a79
	jp z,l4aa8h		;4a7b
	ld (ix+00ch),e		;4a7e
	ld (ix+00dh),d		;4a81
	ld hl,(0fbf9h)		;4a84
	ld a,(0fbfbh)		;4a87
	ld b,a			;4a8a
	exx			;4a8b
	ld hl,l4aa8h		;4a8c
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
