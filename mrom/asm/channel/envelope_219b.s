; envelope_219b.s - M-ROM fixed_0000 [0x219b..0x2515)
; Carved from mrom/asm/raw/fixed_0000.s by mrom/tools/split_region.py.
; Assembled through mrom/asm/fixed_0000.s; must stay byte-identical.

Seq_LoadOpPtrs_219B:
	xor a			;219b
	ld (ix+000h),a		;219c
	ld (ix+00ch),a		;219f
	ld (ix+00dh),a		;21a2
	ld a,(hl)			;21a5
	exx			;21a6
	ld c,000h		;21a7
	ld b,c			;21a9
	and a			;21aa
	jp z,l21b4h		;21ab
	ld l,a			;21ae
	ld h,01bh		;21af
	ld c,(hl)			;21b1
	inc h			;21b2
	ld b,(hl)			;21b3
l21b4h:
	ld (ix+002h),c		;21b4
	ld (ix+003h),b		;21b7
	exx			;21ba
	inc hl			;21bb
	ld a,(hl)			;21bc
	exx			;21bd
	ld c,000h		;21be
	ld b,c			;21c0
	and a			;21c1
	jp z,l21cbh		;21c2
	ld l,a			;21c5
	ld h,01bh		;21c6
	ld c,(hl)			;21c8
	inc h			;21c9
	ld b,(hl)			;21ca
l21cbh:
	ld (ix+004h),c		;21cb
	ld (ix+005h),b		;21ce
	exx			;21d1
	inc hl			;21d2
	ld a,(hl)			;21d3
	cpl			;21d4
	ld (ix+001h),a		;21d5
	inc hl			;21d8
	ld a,(hl)			;21d9
	exx			;21da
	ld c,000h		;21db
	ld b,c			;21dd
	and a			;21de
	jp z,l21e8h		;21df
	ld l,a			;21e2
	ld h,01bh		;21e3
	ld c,(hl)			;21e5
	inc h			;21e6
	ld b,(hl)			;21e7
l21e8h:
	ld (ix+006h),c		;21e8
	ld (ix+007h),b		;21eb
	exx			;21ee
	inc hl			;21ef
	ld a,(hl)			;21f0
	exx			;21f1
	ld c,000h		;21f2
	ld b,c			;21f4
	and a			;21f5
	jp z,l21ffh		;21f6
	ld l,a			;21f9
	ld h,01bh		;21fa
	ld c,(hl)			;21fc
	inc h			;21fd
	ld b,(hl)			;21fe
l21ffh:
	ld (ix+008h),c		;21ff
	ld (ix+009h),b		;2202
	exx			;2205
	inc hl			;2206
	ret			;2207
Env_Start_2208:
	ld (hl),001h		;2208
	ld de,l0001h+1		;220a
	add hl,de			;220d
	ex de,hl			;220e
	ld hl,YM_Write_RST8_0008		;220f
	add hl,de			;2212
	ex de,hl			;2213
	ldi		;2214
	ld a,(hl)			;2216
	ld (de),a			;2217
	ret			;2218
Env_Release_2219:
	ld a,(hl)			;2219
	and a			;221a
	ret z			;221b
	ld (hl),004h		;221c
	ld de,YM_Write_RST8_0008		;221e
	add hl,de			;2221
	ex de,hl			;2222
	ld hl,l0001h+1		;2223
	add hl,de			;2226
	ex de,hl			;2227
	ldi		;2228
	ld a,(hl)			;222a
	ld (de),a			;222b
	ret			;222c
Env_Step_222D:
	ld a,(hl)			;222d
	and a			;222e
	ret z			;222f
	ex de,hl			;2230
	ld hl,l000ah		;2231
	add hl,de			;2234
	ld c,(hl)			;2235
	inc hl			;2236
	ld b,(hl)			;2237
	inc hl			;2238
	dec a			;2239
	jr z,l2253h		;223a
	dec a			;223c
	jr z,l226ch		;223d
	ld a,c			;223f
	or b			;2240
	jp nz,l2247h		;2241
	inc hl			;2244
	ld a,(hl)			;2245
	ret			;2246
l2247h:
	ld a,(hl)			;2247
	sub c			;2248
	ld (hl),a			;2249
	inc hl			;224a
	ld a,(hl)			;224b
	sbc a,b			;224c
	ld (hl),a			;224d
	ret nc			;224e
	xor a			;224f
	ld (hl),a			;2250
	ld (de),a			;2251
	ret			;2252
l2253h:
	ld a,(hl)			;2253
	add a,c			;2254
	ld (hl),a			;2255
	inc hl			;2256
	ld a,(hl)			;2257
	adc a,b			;2258
	ld (hl),a			;2259
	ret nc			;225a
	sbc a,a			;225b
	ld (hl),a			;225c
	dec hl			;225d
	dec hl			;225e
	ex de,hl			;225f
	inc (hl)			;2260
	ld bc,00005h		;2261
	add hl,bc			;2264
	ldd		;2265
	ld a,(hl)			;2267
	ld (de),a			;2268
	ld a,0ffh		;2269
	ret			;226b
l226ch:
	ld a,c			;226c
	or b			;226d
	jp nz,l2274h		;226e
	inc hl			;2271
	ld a,(hl)			;2272
	ret			;2273
l2274h:
	ld a,(hl)			;2274
	sub c			;2275
	ld (hl),a			;2276
	inc hl			;2277
	ld a,(hl)			;2278
	sbc a,b			;2279
	ex de,hl			;227a
	inc hl			;227b
	ld c,(hl)			;227c
	jr c,l2284h		;227d
	cp c			;227f
	jr c,l2284h		;2280
	ld (de),a			;2282
	ret			;2283
l2284h:
	ex de,hl			;2284
	dec de			;2285
	ld (hl),c			;2286
	ld a,c			;2287
	and a			;2288
	jp nz,l228eh		;2289
	ld (de),a			;228c
	ret			;228d
l228eh:
	ex af,af'			;228e
	ex de,hl			;228f
	inc (hl)			;2290
	ld bc,l0007h		;2291
	add hl,bc			;2294
	dec de			;2295
	dec de			;2296
	ldd		;2297
	ld a,(hl)			;2299
	ld (de),a			;229a
	ex af,af'			;229b
	ret			;229c
Lfo_Clear_229D:
	xor a			;229d
	ld (ix+000h),a		;229e
	ld (ix+001h),a		;22a1
	ld (ix+002h),a		;22a4
	ret			;22a7
Lfo_Load_22A8:
	ld a,(iy+000h)		;22a8
	ld b,a			;22ab
	and 07fh		;22ac
	ld (ix+007h),a		;22ae
	ld a,b			;22b1
	rlca			;22b2
	and 001h		;22b3
	ld (ix+000h),a		;22b5
	ld a,(iy+001h)		;22b8
	ld (ix+00dh),a		;22bb
	ld a,(iy+002h)		;22be
	ld (ix+001h),a		;22c1
	ld a,(iy+003h)		;22c4
	add a,a			;22c7
	add a,080h		;22c8
	adc a,080h		;22ca
	ld (ix+002h),a		;22cc
	ld a,(iy+005h)		;22cf
	add a,a			;22d2
	add a,080h		;22d3
	adc a,080h		;22d5
	ld (ix+003h),a		;22d7
	ld l,(iy+004h)		;22da
	ld h,014h		;22dd
	ld a,(hl)			;22df
	inc h			;22e0
	cpl			;22e1
	ld l,a			;22e2
	ld b,(hl)			;22e3
	inc h			;22e4
	ld c,(hl)			;22e5
	ld (ix+009h),c		;22e6
	ld (ix+00bh),b		;22e9
	xor a			;22ec
	ld (ix+00ah),a		;22ed
	ld (ix+00ch),a		;22f0
	ret			;22f3
Env_Init_22F4:
	ld de,00004h		;22f4
	add hl,de			;22f7
	ld e,l			;22f8
	ld d,h			;22f9
	ld bc,YM_Write_RST8_0008+1		;22fa
	add hl,bc			;22fd
	ldi		;22fe
Rng_Next_2300:
	ex de,hl			;2300
	xor a			;2301
	ld (hl),a			;2302
	inc hl			;2303
	ld (hl),a			;2304
	inc hl			;2305
	inc hl			;2306
	ld de,(wRngState_F81D)		;2307
	ld a,r		;230b
	add a,e			;230d
	ld e,d			;230e
	xor d			;230f
	ld d,a			;2310
	ld (wRngState_F81D),de		;2311
	ld (hl),a			;2315
	inc hl			;2316
	inc hl			;2317
	xor a			;2318
	ld (hl),a			;2319
	inc hl			;231a
	inc hl			;231b
	ld (hl),a			;231c
	ret			;231d
Env_Update_231E:
	ld a,(hl)			;231e
	inc hl			;231f
	ld b,(hl)			;2320
	inc hl			;2321
	ld c,(hl)			;2322
	inc hl			;2323
	ld e,(hl)			;2324
	inc hl			;2325
	and a			;2326
	jr z,l2357h		;2327
	ld a,b			;2329
	and a			;232a
	jp z,l2369h		;232b
	ld a,d			;232e
	add a,a			;232f
	ex de,hl			;2330
	and a			;2331
	jr z,l2349h		;2332
	inc l			;2334
	jr z,l2349h		;2335
	ex af,af'			;2337
	dec l			;2338
	ld a,l			;2339
	jr z,l2349h		;233a
	ld h,014h		;233c
	ld a,(hl)			;233e
	ex af,af'			;233f
	ld l,a			;2340
	ex af,af'			;2341
	add a,(hl)			;2342
	ld l,a			;2343
	sbc a,a			;2344
	jr nc,l2349h		;2345
	inc h			;2347
	ld a,(hl)			;2348
l2349h:
	ex de,hl			;2349
	add a,c			;234a
	ld c,a			;234b
	jp z,l2369h		;234c
	jp nc,l2362h		;234f
	ld c,0ffh		;2352
	jp l2362h		;2354
l2357h:
	ld a,c			;2357
	and a			;2358
	jp z,l2369h		;2359
	ld a,d			;235c
	add a,b			;235d
	ld b,a			;235e
	jp z,l2369h		;235f
l2362h:
	ld a,(hl)			;2362
	and a			;2363
	jp z,l236ch		;2364
	dec a			;2367
	ld (hl),a			;2368
l2369h:
	xor a			;2369
	ld c,a			;236a
	ret			;236b
l236ch:
	inc hl			;236c
	ld a,(hl)			;236d
	scf			;236e
	adc a,b			;236f
	ld e,a			;2370
	ld (hl),a			;2371
	inc hl			;2372
	sbc a,a			;2373
	ld b,a			;2374
	ld a,000h		;2375
	adc a,(hl)			;2377
	ld (hl),a			;2378
	inc hl			;2379
	ld d,(hl)			;237a
	inc hl			;237b
	dec d			;237c
	jr z,l23adh		;237d
	dec d			;237f
	jr z,l238eh		;2380
	dec d			;2382
	jr z,l2393h		;2383
	dec d			;2385
	jr z,l2399h		;2386
	rra			;2388
	sbc a,a			;2389
	xor e			;238a
	jp l23ach		;238b
l238eh:
	ld a,e			;238e
	cpl			;238f
	jp l23ach		;2390
l2393h:
	rra			;2393
	ccf			;2394
	sbc a,a			;2395
	jp l23ach		;2396
l2399h:
	inc b			;2399
	jr nz,l23abh		;239a
	ld de,(wRngState_F81D)		;239c
	ld a,r		;23a0
	add a,e			;23a2
	ld e,d			;23a3
	xor d			;23a4
	ld d,a			;23a5
	ld (wRngState_F81D),de		;23a6
	ld (hl),a			;23aa
l23abh:
	ld a,(hl)			;23ab
l23ach:
	ld e,a			;23ac
l23adh:
	inc hl			;23ad
	ld a,(hl)			;23ae
	inc hl			;23af
	add a,(hl)			;23b0
	ld (hl),a			;23b1
	inc hl			;23b2
	ld a,(hl)			;23b3
	inc hl			;23b4
	adc a,(hl)			;23b5
	ld b,a			;23b6
	sbc a,a			;23b7
	or b			;23b8
	ld (hl),a			;23b9
	ld l,c			;23ba
	and a			;23bb
	jr z,l23d3h		;23bc
	inc l			;23be
	jr z,l23d3h		;23bf
	ex af,af'			;23c1
	dec l			;23c2
	ld a,l			;23c3
	jr z,l23d3h		;23c4
	ld h,014h		;23c6
	ld a,(hl)			;23c8
	ex af,af'			;23c9
	ld l,a			;23ca
	ex af,af'			;23cb
	add a,(hl)			;23cc
	ld l,a			;23cd
	sbc a,a			;23ce
	jr nc,l23d3h		;23cf
	inc h			;23d1
	ld a,(hl)			;23d2
l23d3h:
	ld c,a			;23d3
	ld l,e			;23d4
	and a			;23d5
	jr z,l23edh		;23d6
	inc l			;23d8
	jr z,l23edh		;23d9
	ex af,af'			;23db
	dec l			;23dc
	ld a,l			;23dd
	jr z,l23edh		;23de
	ld h,014h		;23e0
	ld a,(hl)			;23e2
	ex af,af'			;23e3
	ld l,a			;23e4
	ex af,af'			;23e5
	add a,(hl)			;23e6
	ld l,a			;23e7
	sbc a,a			;23e8
	jr nc,l23edh		;23e9
	inc h			;23eb
	ld a,(hl)			;23ec
l23edh:
	ret			;23ed
Pitch_Clear_23EE:
	xor a			;23ee
	ld (ix+000h),a		;23ef
	ld (ix+001h),a		;23f2
	ret			;23f5
Pitch_SetNote_23F6:
	ld l,a			;23f6
	add a,a			;23f7
	sbc a,a			;23f8
	ld h,a			;23f9
	xor (ix+006h)		;23fa
	and 0bfh		;23fd
	xor h			;23ff
	ld (ix+006h),a		;2400
	ld a,l			;2403
	xor h			;2404
	sub h			;2405
	cp 080h		;2406
	jr nz,l240bh		;2408
	dec a			;240a
l240bh:
	ld l,a			;240b
	ld h,011h		;240c
	ld c,(hl)			;240e
	set 7,l		;240f
	ld b,(hl)			;2411
	ld a,c			;2412
	sra b		;2413
	rra			;2415
	sra b		;2416
	rra			;2418
	ld (ix+007h),a		;2419
	ld (ix+008h),b		;241c
	ret			;241f
Pitch_SetFine_2420:
	ld l,a			;2420
	ld h,000h		;2421
	add hl,hl			;2423
	add hl,hl			;2424
	add hl,hl			;2425
	add hl,hl			;2426
	ld (ix+000h),l		;2427
	ld (ix+001h),h		;242a
	ret			;242d
Pitch_Load_242E:
	ld a,(iy+000h)		;242e
	ld b,a			;2431
	and 001h		;2432
	rrca			;2434
	ld (ix+006h),a		;2435
	ld a,b			;2438
	and 0f0h		;2439
	ld (ix+00ch),a		;243b
	ld a,(iy+001h)		;243e
	ld (ix+009h),a		;2441
	ld a,(iy+003h)		;2444
	ld (ix+00bh),a		;2447
	call Pitch_SetNote_23F6		;244a
	ld a,(iy+002h)		;244d
	ld (ix+00ah),a		;2450
	call Pitch_SetFine_2420		;2453
	xor a			;2456
	ld (ix+003h),a		;2457
	ld (ix+004h),a		;245a
	ld (ix+005h),a		;245d
	ret			;2460
Pitch_Update_2461:
	ld a,(ix+00ch)		;2461
	and a			;2464
	jr nz,l2474h		;2465
	ld a,(ix+00ah)		;2467
	add a,c			;246a
	call Pitch_SetFine_2420		;246b
	ld a,(ix+00bh)		;246e
	jp l247ah		;2471
l2474h:
	ld a,c			;2474
	sub 040h		;2475
	add a,(ix+00bh)		;2477
l247ah:
	call Pitch_SetNote_23F6		;247a
	ld (ix+004h),a		;247d
	ld (ix+005h),b		;2480
	ld (ix+003h),000h		;2483
	ld a,(ix+009h)		;2487
	ld (ix+002h),a		;248a
	ret			;248d
Vib_Update_248E:
	ld b,(hl)			;248e
	inc hl			;248f
	ld c,(hl)			;2490
	ld a,b			;2491
	or c			;2492
	jr nz,l2498h		;2493
	ld h,b			;2495
	ld l,c			;2496
	ret			;2497
l2498h:
	inc hl			;2498
	ld a,(hl)			;2499
	and a			;249a
	jr z,l24a8h		;249b
	dec a			;249d
	ld (hl),a			;249e
	inc hl			;249f
	inc hl			;24a0
	ld e,(hl)			;24a1
	inc hl			;24a2
	ld d,(hl)			;24a3
	inc hl			;24a4
	jp l24c1h		;24a5
l24a8h:
	ex af,af'			;24a8
	inc hl			;24a9
	ld a,(hl)			;24aa
	sub b			;24ab
l24ach:
	ld (hl),a			;24ac
	inc hl			;24ad
	ld e,(hl)			;24ae
	inc hl			;24af
	ld d,(hl)			;24b0
	ex de,hl			;24b1
	ld b,000h		;24b2
	sbc hl,bc		;24b4
	ex de,hl			;24b6
	jr nc,l24bch		;24b7
	ld de,Reset_0000		;24b9
l24bch:
	ld (hl),d			;24bc
	dec hl			;24bd
	ld (hl),e			;24be
	inc hl			;24bf
	inc hl			;24c0
l24c1h:
	ld a,(hl)			;24c1
	add a,a			;24c2
	jr c,l24d2h		;24c3
	inc hl			;24c5
	ld c,(hl)			;24c6
	inc hl			;24c7
	ld h,(hl)			;24c8
	ld l,c			;24c9
	add a,a			;24ca
	jr nc,l24cfh		;24cb
	ex de,hl			;24cd
	and a			;24ce
l24cfh:
	sbc hl,de		;24cf
	ret			;24d1
l24d2h:
	add a,a			;24d2
	jr nc,l24dch		;24d3
	ld hl,Reset_0000		;24d5
	and a			;24d8
	sbc hl,de		;24d9
	ret			;24db
l24dch:
	ex de,hl			;24dc
	ret			;24dd
CmdStruct_24DE:
	defb 08ch		;24de
	defb 0a8h		;24df
	defb 0d1h		;24e0
	defb 001h		;24e1
	defb 000h		;24e2
	defb 0ffh		;24e3
	defb 043h		;24e4
	defb 0d2h		;24e5
	defb 001h		;24e6
	defb 000h		;24e7
	defb 0ffh		;24e8
	defb 03fh		;24e9
	defb 0d2h		;24ea
	defb 001h		;24eb
	defb 000h		;24ec
	defb 0ffh		;24ed
	defb 038h		;24ee
	defb 0d2h		;24ef
	defb 001h		;24f0
	defb 000h		;24f1
	defb 0ffh		;24f2
	defb 000h		;24f3
	defb 000h		;24f4
	defb 000h		;24f5
	defb 000h		;24f6
CmdStruct_24F7:
	defb 020h		;24f7
	defb 010h		;24f8
	defb 000h		;24f9
	defb 000h		;24fa
	defb 000h		;24fb
	defb 000h		;24fc
	defb 000h		;24fd
	defb 000h		;24fe
	defb 0f7h		;24ff
	defb 062h		;2500
	defb 0d2h		;2501
	defb 001h		;2502
l2503h:
	defb 000h		;2503
	defb 000h		;2504
	defb 000h		;2505
	defb 000h		;2506
	defb 000h		;2507
	defb 000h		;2508
l2509h:
	defb 080h		;2509
	defb 019h		;250a
	defb 000h		;250b
	defb 000h		;250c
	defb 0b2h		;250d
	defb 019h		;250e
	defb 000h		;250f
	defb 000h		;2510
	defb 0e4h		;2511
	defb 019h		;2512
	defb 000h		;2513
	defb 000h		;2514
