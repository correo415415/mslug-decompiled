; opcodes_5362.s - M-ROM fixed_0000 [0x5362..0x57a1)
; Carved from mrom/asm/raw/fixed_0000.s by mrom/tools/split_region.py.
; Assembled through mrom/asm/fixed_0000.s; must stay byte-identical.

Seq_TrackFetch_5362:
	ld b,(iy+004h)		;5362
	ld a,(wCurBank_F811)		;5365
	cp b			;5368
	jp z,l5386h		;5369
	ld a,b			;536c
	ld (wCurBank_F811),a		;536d
	ld c,00bh		;5370
	in a,(c)		;5372
	inc b			;5374
	sla b		;5375
	dec c			;5377
	in a,(c)		;5378
	inc b			;537a
	sla b		;537b
	dec c			;537d
	in a,(c)		;537e
	inc b			;5380
	sla b		;5381
	dec c			;5383
	in a,(c)		;5384
l5386h:
	ld de,(wSeqPtr_FF0D)		;5386
	ld a,(de)			;538a
	inc de			;538b
	cp 0dah		;538c
	jp nc,Seq_TrackStop_578D		;538e
	cp 0c0h		;5391
	jp c,Seq_NoteEvent_56EA		;5393
	sub 0c0h		;5396
	ld l,a			;5398
	add a,a			;5399
	add a,l			;539a
	ld l,a			;539b
	ld h,000h		;539c
	ld bc,SeqOpJumpTable_53A3		;539e
	add hl,bc			;53a1
	jp (hl)			;53a2
SeqOpJumpTable_53A3:
	jp SeqOp_C0_SetInstr_56B1		;53a3
	jp SeqOp_C1_SetInstr2_56BD		;53a6
	jp SeqOp_C2_Jump_56CE		;53a9
	jp SeqOp_C3_SetBit0_569A		;53ac
	jp SeqOp_C4_CallSub_565D		;53af
	jp SeqOp_C5_Return_559E		;53b2
	jp SeqOp_C6_SetVol_5574		;53b5
	jp SeqOp_C7_SetPan_5582		;53b8
	jp SeqOp_C8_SetParam_5590		;53bb
	jp SeqOp_C9_SetSustain_555C		;53be
	jp SeqOp_CA_SetDetune_5541		;53c1
	jp SeqOp_CB_SetWait_5535		;53c4
	jp SeqOp_CC_SetLength_551D		;53c7
	jp SeqOp_CD_SetTempo_550F		;53ca
	jp SeqOp_CE_SetTranspose_54FC		;53cd
	jp SeqOp_CF_LoopStart_55D4		;53d0
	jp SeqOp_D0_LoopEnd_5605		;53d3
	jp SeqOp_D1_Portamento_54C5		;53d6
	jp SeqOp_D2_SetLfo_54F0		;53d9
	jp SeqOp_D3_Note_5420		;53dc
	jp SeqOp_D4_Note2_5434		;53df
	jp SeqOp_D5_Note3_5448		;53e2
	jp SeqOp_D6_Note4_545C		;53e5
	jp SeqOp_D7_PitchDown_5470		;53e8
	jp SeqOp_D8_PitchUp_5499		;53eb
	jp SeqOp_D9_Reply68k_53F4		;53ee
	jp SeqOp_DA_SetVolLevel_540F		;53f1
SeqOp_D9_Reply68k_53F4:
	ld a,(de)			;53f4
	inc de			;53f5
	ld (wSeqPtr_FF0D),de		;53f6
	and 03fh		;53fa
	add a,a			;53fc
	add a,080h		;53fd
	ld b,a			;53ff
	ld a,(wReplyToggle_FF0C)		;5400
	inc a			;5403
	ld (wReplyToggle_FF0C),a		;5404
	and 001h		;5407
	or b			;5409
	out (00ch),a		;540a
	jp Seq_TrackNextOp_533E		;540c
SeqOp_DA_SetVolLevel_540F:
	ld a,001h		;540f
	ld (wFadeFlag_F808),a		;5411
	ex de,hl			;5414
	ld a,(hl)			;5415
	inc hl			;5416
	ld (wSeqPtr_FF0D),hl		;5417
	call Seq_SetVolumeLevel_4F88		;541a
	jp Seq_TrackNextOp_533E		;541d
SeqOp_D3_Note_5420:
	ex de,hl			;5420
	ld c,(hl)			;5421
	inc hl			;5422
	ld a,(hl)			;5423
	ld (iy+016h),a		;5424
	inc hl			;5427
	ld (wSeqPtr_FF0D),hl		;5428
	call Ch_SetNote_4B0B		;542b
	jp nc,Seq_TrackEndCheck_5786		;542e
	jp Seq_TrackNextOp_533E		;5431
SeqOp_D4_Note2_5434:
	ex de,hl			;5434
	ld c,(hl)			;5435
	inc hl			;5436
	ld a,(hl)			;5437
	ld (iy+016h),a		;5438
	inc hl			;543b
	ld (wSeqPtr_FF0D),hl		;543c
	call Ch_SetNote2_4B26		;543f
	jp nc,Seq_TrackEndCheck_5786		;5442
	jp Seq_TrackNextOp_533E		;5445
SeqOp_D5_Note3_5448:
	ex de,hl			;5448
	ld c,(hl)			;5449
	inc hl			;544a
	ld a,(hl)			;544b
	ld (iy+016h),a		;544c
	inc hl			;544f
	ld (wSeqPtr_FF0D),hl		;5450
	call Ch_SetNote3_4B41		;5453
	jp nc,Seq_TrackEndCheck_5786		;5456
	jp Seq_TrackNextOp_533E		;5459
SeqOp_D6_Note4_545C:
	ex de,hl			;545c
	ld c,(hl)			;545d
	inc hl			;545e
	ld a,(hl)			;545f
	ld (iy+016h),a		;5460
	inc hl			;5463
	ld (wSeqPtr_FF0D),hl		;5464
	call Ch_SetNote4_4B4F		;5467
	jp nc,Seq_TrackEndCheck_5786		;546a
	jp Seq_TrackNextOp_533E		;546d
SeqOp_D7_PitchDown_5470:
	ex de,hl			;5470
	ld c,(hl)			;5471
	inc hl			;5472
	ld a,(hl)			;5473
	ld (iy+016h),a		;5474
	inc hl			;5477
	ld (wSeqPtr_FF0D),hl		;5478
	ld b,000h		;547b
	ld l,(iy+010h)		;547d
	ld h,(iy+011h)		;5480
	ld de,l1000h		;5483
	and a			;5486
	sbc hl,de		;5487
	add hl,bc			;5489
	jr nc,l548fh		;548a
	ld hl,0ffffh		;548c
l548fh:
	add hl,de			;548f
	ld (iy+010h),l		;5490
	ld (iy+011h),h		;5493
	jp Seq_TrackNextOp_533E		;5496
SeqOp_D8_PitchUp_5499:
	ex de,hl			;5499
	ld c,(hl)			;549a
	inc hl			;549b
	ld a,(hl)			;549c
	ld (iy+016h),a		;549d
	inc hl			;54a0
	ld (wSeqPtr_FF0D),hl		;54a1
	ld b,000h		;54a4
	ld l,(iy+010h)		;54a6
	ld h,(iy+011h)		;54a9
	ld de,l1000h		;54ac
	add hl,de			;54af
	and a			;54b0
	sbc hl,bc		;54b1
	jp nc,l54b9h		;54b3
	ld hl,Reset_0000		;54b6
l54b9h:
	and a			;54b9
	sbc hl,de		;54ba
	ld (iy+010h),l		;54bc
	ld (iy+011h),h		;54bf
	jp Seq_TrackNextOp_533E		;54c2
SeqOp_D1_Portamento_54C5:
	ex de,hl			;54c5
	ld a,(hl)			;54c6
	inc hl			;54c7
	ld (wSeqPtr_FF0D),hl		;54c8
	cp 040h		;54cb
	jr c,l54e2h		;54cd
	ld c,(iy+009h)		;54cf
	set 6,(iy+009h)		;54d2
	bit 6,c		;54d6
	jp nz,Seq_TrackNextOp_533E		;54d8
	ld (iy+01bh),0ffh		;54db
	jp Seq_TrackNextOp_533E		;54df
l54e2h:
	res 6,(iy+009h)		;54e2
	xor a			;54e6
	ld (iy+010h),a		;54e7
	ld (iy+011h),a		;54ea
	jp Seq_TrackNextOp_533E		;54ed
SeqOp_D2_SetLfo_54F0:
	ex de,hl			;54f0
	ld a,(hl)			;54f1
	inc hl			;54f2
	ld (wSeqPtr_FF0D),hl		;54f3
	ld (iy+013h),a		;54f6
	jp Seq_TrackNextOp_533E		;54f9
SeqOp_CE_SetTranspose_54FC:
	ex de,hl			;54fc
	ld a,(hl)			;54fd
	and 07fh		;54fe
	add a,a			;5500
	add a,080h		;5501
	adc a,080h		;5503
	ld (iy+012h),a		;5505
	inc hl			;5508
	ld (wSeqPtr_FF0D),hl		;5509
	jp Seq_TrackNextOp_533E		;550c
SeqOp_CD_SetTempo_550F:
	ex de,hl			;550f
	ld b,(hl)			;5510
	inc hl			;5511
	ld a,(hl)			;5512
	inc hl			;5513
	ld (wSeqPtr_FF0D),hl		;5514
	call Seq_SetTempo_4F39		;5517
	jp Seq_TrackNextOp_533E		;551a
SeqOp_CC_SetLength_551D:
	ex de,hl			;551d
	ld a,(hl)			;551e
	inc hl			;551f
	ld (wSeqPtr_FF0D),hl		;5520
	ld b,003h		;5523
	cp 040h		;5525
	jp z,l552fh		;5527
	dec b			;552a
	jp c,l552fh		;552b
	dec b			;552e
l552fh:
	ld (iy+00fh),b		;552f
	jp Seq_TrackNextOp_533E		;5532
SeqOp_CB_SetWait_5535:
	ex de,hl			;5535
	ld a,(hl)			;5536
	inc hl			;5537
	ld (wSeqPtr_FF0D),hl		;5538
	ld (iy+016h),a		;553b
	jp Seq_TrackNextOp_533E		;553e
SeqOp_CA_SetDetune_5541:
	ex de,hl			;5541
	ld e,(hl)			;5542
	inc hl			;5543
	ld d,(hl)			;5544
	inc hl			;5545
	ld a,(hl)			;5546
	ld (iy+016h),a		;5547
	inc hl			;554a
	ld (wSeqPtr_FF0D),hl		;554b
	ex de,hl			;554e
	add hl,hl			;554f
	add hl,hl			;5550
	add hl,hl			;5551
	sbc a,a			;5552
	ld (iy+010h),h		;5553
	ld (iy+011h),a		;5556
	jp Seq_TrackNextOp_533E		;5559
SeqOp_C9_SetSustain_555C:
	ex de,hl			;555c
	ld a,(hl)			;555d
	inc hl			;555e
	ld (wSeqPtr_FF0D),hl		;555f
	and a			;5562
	jp z,l556dh		;5563
	set 4,(iy+009h)		;5566
	jp Seq_TrackNextOp_533E		;556a
l556dh:
	res 4,(iy+009h)		;556d
	jp Seq_TrackNextOp_533E		;5571
SeqOp_C6_SetVol_5574:
	ex de,hl			;5574
	ld a,(hl)			;5575
	inc hl			;5576
	ld (wSeqPtr_FF0D),hl		;5577
	and 07fh		;557a
	ld (iy+00ch),a		;557c
	jp Seq_TrackNextOp_533E		;557f
SeqOp_C7_SetPan_5582:
	ex de,hl			;5582
	ld a,(hl)			;5583
	inc hl			;5584
	ld (wSeqPtr_FF0D),hl		;5585
	and 07fh		;5588
	ld (iy+00dh),a		;558a
	jp Seq_TrackNextOp_533E		;558d
SeqOp_C8_SetParam_5590:
	ex de,hl			;5590
	ld a,(hl)			;5591
	inc hl			;5592
	ld (wSeqPtr_FF0D),hl		;5593
	and 07fh		;5596
	ld (iy+00eh),a		;5598
	jp Seq_TrackNextOp_533E		;559b
SeqOp_C5_Return_559E:
	ld a,(iy+022h)		;559e
	cp 006h		;55a1
	jp c,l55a9h		;55a3
	jp l5791h		;55a6
l55a9h:
	ld e,a			;55a9
	add a,002h		;55aa
	ld (iy+022h),a		;55ac
	ld d,000h		;55af
	ld hl,l001ch		;55b1
	add hl,de			;55b4
	defb 0fdh,05dh	;ld e,iyl		;55b5
	defb 0fdh,054h	;ld d,iyh		;55b7
	add hl,de			;55b9
	ld e,(hl)			;55ba
	inc hl			;55bb
	ld d,(hl)			;55bc
	ld (wSeqPtr_FF0D),de		;55bd
	ld (iy+017h),040h		;55c1
	ld (iy+018h),0c0h		;55c5
	ld (iy+019h),0c0h		;55c9
	ld (iy+01ah),07fh		;55cd
	jp Seq_TrackNextOp_533E		;55d1
SeqOp_CF_LoopStart_55D4:
	ld (wSeqPtr_FF0D),de		;55d4
	ld a,(iy+022h)		;55d8
	dec a			;55db
	cp 006h		;55dc
	jp nc,Seq_TrackStop_578D		;55de
	ld (iy+022h),a		;55e1
	ld l,a			;55e4
	ld h,000h		;55e5
	ld bc,l001ch		;55e7
	add hl,bc			;55ea
	defb 0fdh,04dh	;ld c,iyl		;55eb
	defb 0fdh,044h	;ld b,iyh		;55ed
	add hl,bc			;55ef
	ld (hl),000h		;55f0
	ld (iy+017h),040h		;55f2
	ld (iy+018h),0c0h		;55f6
	ld (iy+019h),0c0h		;55fa
	ld (iy+01ah),07fh		;55fe
	jp Seq_TrackNextOp_533E		;5602
SeqOp_D0_LoopEnd_5605:
	ld a,(iy+022h)		;5605
	cp 006h		;5608
	jp nc,Seq_TrackStop_578D		;560a
	ld l,a			;560d
	ld h,000h		;560e
	ld bc,l001ch		;5610
	add hl,bc			;5613
	defb 0fdh,04dh	;ld c,iyl		;5614
	defb 0fdh,044h	;ld b,iyh		;5616
	add hl,bc			;5618
	ex de,hl			;5619
	ld a,(de)			;561a
	and a			;561b
	jr z,l5624h		;561c
	dec a			;561e
	jr z,l5648h		;561f
	ld (de),a			;5621
	jr l562ch		;5622
l5624h:
	ld a,(hl)			;5624
	and a			;5625
	jr z,l562ch		;5626
	dec a			;5628
	jr z,l5648h		;5629
	ld (de),a			;562b
l562ch:
	inc hl			;562c
	ld e,(hl)			;562d
	inc hl			;562e
	ld d,(hl)			;562f
	inc hl			;5630
	add hl,de			;5631
	ld (wSeqPtr_FF0D),hl		;5632
	ld (iy+017h),040h		;5635
	ld (iy+018h),0c0h		;5639
	ld (iy+019h),0c0h		;563d
	ld (iy+01ah),07fh		;5641
	jp Seq_TrackNextOp_533E		;5645
l5648h:
	ld a,(iy+022h)		;5648
	cp 006h		;564b
	jp nc,Seq_TrackStop_578D		;564d
	inc a			;5650
	ld (iy+022h),a		;5651
	inc hl			;5654
	inc hl			;5655
	inc hl			;5656
	ld (wSeqPtr_FF0D),hl		;5657
	jp Seq_TrackNextOp_533E		;565a
SeqOp_C4_CallSub_565D:
	ex de,hl			;565d
	ld e,(hl)			;565e
	inc hl			;565f
	ld d,(hl)			;5660
	inc hl			;5661
	ex de,hl			;5662
	add hl,de			;5663
	ld (wSeqPtr_FF0D),hl		;5664
	ld a,(iy+022h)		;5667
	and a			;566a
	jp z,Seq_TrackNextOp_533E		;566b
	sub 002h		;566e
	cp 006h		;5670
	jp nc,Seq_TrackStop_578D		;5672
	ld (iy+022h),a		;5675
	ld l,a			;5678
	ld h,000h		;5679
	ld bc,l001ch		;567b
	add hl,bc			;567e
	defb 0fdh,04dh	;ld c,iyl		;567f
	defb 0fdh,044h	;ld b,iyh		;5681
	add hl,bc			;5683
	ld (hl),e			;5684
	inc hl			;5685
	ld (hl),d			;5686
	ld (iy+017h),040h		;5687
	ld (iy+018h),0c0h		;568b
	ld (iy+019h),0c0h		;568f
	ld (iy+01ah),07fh		;5693
	jp Seq_TrackNextOp_533E		;5697
SeqOp_C3_SetBit0_569A:
	ex de,hl			;569a
	ld a,(hl)			;569b
	or a			;569c
	jr z,l56a6h		;569d
	set 0,(iy+009h)		;569f
	jp l56aah		;56a3
l56a6h:
	res 0,(iy+009h)		;56a6
l56aah:
	inc hl			;56aa
	ld (wSeqPtr_FF0D),hl		;56ab
	jp Seq_TrackNextOp_533E		;56ae
SeqOp_C0_SetInstr_56B1:
	ex de,hl			;56b1
	ld a,(hl)			;56b2
	ld (iy+00ah),a		;56b3
	inc hl			;56b6
	ld (wSeqPtr_FF0D),hl		;56b7
	jp Seq_TrackNextOp_533E		;56ba
SeqOp_C1_SetInstr2_56BD:
	ex de,hl			;56bd
	ld a,(hl)			;56be
	ld (iy+00ah),a		;56bf
	inc hl			;56c2
	ld a,(hl)			;56c3
	ld (iy+00bh),a		;56c4
	inc hl			;56c7
	ld (wSeqPtr_FF0D),hl		;56c8
	jp Seq_TrackNextOp_533E		;56cb
SeqOp_C2_Jump_56CE:
	ex de,hl			;56ce
	ld e,(hl)			;56cf
	inc hl			;56d0
	ld d,(hl)			;56d1
	inc hl			;56d2
	add hl,de			;56d3
	ld (wSeqPtr_FF0D),hl		;56d4
	ld (iy+017h),040h		;56d7
	ld (iy+018h),0c0h		;56db
	ld (iy+019h),0c0h		;56df
	ld (iy+01ah),07fh		;56e3
	jp Seq_TrackNextOp_533E		;56e7
Seq_NoteEvent_56EA:
	ld b,a			;56ea
	and 0f8h		;56eb
	cp 0b8h		;56ed
	jr z,l56fch		;56ef
	rrca			;56f1
	rrca			;56f2
	rrca			;56f3
	sub 00bh		;56f4
	add a,(iy+017h)		;56f6
	jp l56feh		;56f9
l56fch:
	ld a,(de)			;56fc
	inc de			;56fd
l56feh:
	ld (iy+017h),a		;56fe
	ld (wNoteTmp_FBF5),a		;5701
	ld a,(iy+018h)		;5704
	srl b		;5707
	jr c,l5710h		;5709
	ld a,(de)			;570b
	inc de			;570c
	ld (iy+018h),a		;570d
l5710h:
	ld (iy+016h),a		;5710
	ld (wNoteLen_FBF6),a		;5713
	ld a,(iy+019h)		;5716
	srl b		;5719
	jr c,l5722h		;571b
	ld a,(de)			;571d
	inc de			;571e
	ld (iy+019h),a		;571f
l5722h:
	ld (wNoteVol_FBF7),a		;5722
	ld a,(iy+01ah)		;5725
	srl b		;5728
	jr c,l5731h		;572a
	ld a,(de)			;572c
	inc de			;572d
	ld (iy+01ah),a		;572e
l5731h:
	ld (wNoteGate_FBF8),a		;5731
	ld (wSeqPtr_FF0D),de		;5734
	bit 6,(iy+009h)		;5738
	jp z,l576ah		;573c
	ld a,(iy+01bh)		;573f
	cp 0ffh		;5742
	jp z,l576ah		;5744
	ld l,a			;5747
	ld h,011h		;5748
	ld c,(hl)			;574a
	set 7,l		;574b
	ld b,(hl)			;574d
	ld e,c			;574e
	ld d,b			;574f
	ld l,(iy+017h)		;5750
	ld h,011h		;5753
	ld c,(hl)			;5755
	set 7,l		;5756
	ld b,(hl)			;5758
	ex de,hl			;5759
	and a			;575a
	sbc hl,bc		;575b
	ld e,(iy+010h)		;575d
	ld d,(iy+011h)		;5760
	add hl,de			;5763
	ld (iy+010h),l		;5764
	ld (iy+011h),h		;5767
l576ah:
	ld a,(iy+017h)		;576a
	ld (iy+01bh),a		;576d
	ld a,(wNoteVol_FBF7)		;5770
	and a			;5773
	jp z,Seq_TrackNextOp_533E		;5774
	ld a,(wNoteGate_FBF8)		;5777
	jp z,Seq_TrackNextOp_533E		;577a
	call sub_4a03h		;577d
	jp nc,Seq_TrackEndCheck_5786		;5780
	jp Seq_TrackNextOp_533E		;5783
Seq_TrackEndCheck_5786:
	bit 1,(iy+009h)		;5786
	jp nz,Seq_TrackNextOp_533E		;578a
Seq_TrackStop_578D:
	xor a			;578d
	ld (iy+012h),a		;578e
l5791h:
	ld (iy+009h),008h		;5791
	xor a			;5795
	ld (iy+000h),a		;5796
	ld (iy+001h),a		;5799
	dec a			;579c
	ld (iy+002h),a		;579d
	ret			;57a0
