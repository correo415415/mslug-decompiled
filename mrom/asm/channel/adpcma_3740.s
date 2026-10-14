; adpcma_3740.s - M-ROM fixed_0000 [0x3740..0x3fca)
; Carved from mrom/asm/raw/fixed_0000.s by mrom/tools/split_region.py.
; Assembled through mrom/asm/fixed_0000.s; must stay byte-identical.

ADPCMA_Init_3740:
	ld de,00200h		;3740  YM reg 02h <- 00h
	ld c,006h		;3743
	rst 8			;3745
	ld de,000ffh		;3746  YM reg 00h <- FFh
	ld c,006h		;3749
	rst 8			;374b
	ld de,01cffh		;374c  YM reg 1Ch <- FFh
	ld c,004h		;374f
	rst 8			;3751
	ld de,0013eh		;3752  YM reg 01h <- 3Eh
	ld c,006h		;3755
	rst 8			;3757
	ld ix,wPCMA_Ch1_FAA5		;3758
	call ADPCMA_InitChannel_3783		;375c
	ld ix,wPCMA_Ch2_FACE		;375f
	call ADPCMA_InitChannel_3783		;3763
	ld ix,wPCMA_Ch3_FAF7		;3766
	call ADPCMA_InitChannel_3783		;376a
	ld ix,wPCMA_Ch4_FB20		;376d
	call ADPCMA_InitChannel_3783		;3771
	ld ix,wPCMA_Ch5_FB49		;3774
	call ADPCMA_InitChannel_3783		;3778
	ld ix,wPCMA_Ch6_FB72		;377b
	call ADPCMA_InitChannel_3783		;377f
	ret			;3782
ADPCMA_InitChannel_3783:
	call Ch_InitStruct_1FE5		;3783
	ld (ix+002h),002h		;3786
	xor a			;378a
	ld (ix+01ah),a		;378b
	ld (ix+014h),a		;378e
	push ix		;3791
	ld de,0001bh		;3793
	add ix,de		;3796
	rst 30h			;3798
	pop ix		;3799
	ret			;379b
ADPCMA_Update_379C:
	ld ix,wPCMA_Ch1_FAA5		;379c
	ld iy,(wPCMA_Ch1_FAA5)		;37a0
	bit 3,(iy+009h)		;37a4
	call nz,Ch_ResetStruct_201B		;37a8
	ld hl,0fabfh		;37ab
	ld a,(hl)			;37ae
	and a			;37af
	jp z,l38a0h		;37b0
	cp 002h		;37b3
	jr z,l37c9h		;37b5
	ld a,(wYmStatus_F80A)		;37b7
	and 001h		;37ba
	jp z,l3805h		;37bc
	xor a			;37bf
	ld (0fabfh),a		;37c0
	ld (0fab0h),a		;37c3
	jp l38a0h		;37c6
l37c9h:
	ld hl,(0fabbh)		;37c9
	ld bc,(0fabdh)		;37cc
	ld a,001h		;37d0
	ld (0fabfh),a		;37d2
	push bc			;37d5
	ld de,00800h		;37d6  YM reg 08h <- 00h
	ld c,006h		;37d9
	rst 8			;37db
	xor a			;37dc
	ld (0fab5h),a		;37dd
	ld de,00081h		;37e0  YM reg 00h <- 81h
	rst 8			;37e3
	ld de,01c01h		;37e4  YM reg 1Ch <- 01h
	ld c,004h		;37e7
	rst 8			;37e9
	ld e,000h		;37ea
	rst 8			;37ec
	ld c,006h		;37ed
	ld e,l			;37ef
	ld d,010h		;37f0
	rst 8			;37f2
	ld e,h			;37f3
	ld d,018h		;37f4
	pop hl			;37f6
	rst 8			;37f7
	ld e,l			;37f8
	ld d,020h		;37f9
	rst 8			;37fb
	ld e,h			;37fc
	ld d,028h		;37fd
	rst 8			;37ff
	ld e,001h		;3800
	ld d,000h		;3802
	rst 8			;3804
l3805h:
	ld hl,0fac0h		;3805
	call Env_Step_222D		;3808
	ld l,a			;380b
	ld a,(0fab0h)		;380c
	and a			;380f
	jr z,l3827h		;3810
	inc l			;3812
	jr z,l3827h		;3813
	ex af,af'			;3815
	dec l			;3816
	ld a,l			;3817
	jr z,l3827h		;3818
	ld h,014h		;381a
	ld a,(hl)			;381c
	ex af,af'			;381d
	ld l,a			;381e
	ex af,af'			;381f
	add a,(hl)			;3820
	ld l,a			;3821
	sbc a,a			;3822
	jr nc,l3827h		;3823
	inc h			;3825
	ld a,(hl)			;3826
l3827h:
	ld l,(iy+012h)		;3827
	and a			;382a
	jr z,l3842h		;382b
	inc l			;382d
	jr z,l3842h		;382e
	ex af,af'			;3830
	dec l			;3831
	ld a,l			;3832
	jr z,l3842h		;3833
	ld h,014h		;3835
	ld a,(hl)			;3837
	ex af,af'			;3838
	ld l,a			;3839
	ex af,af'			;383a
	add a,(hl)			;383b
	ld l,a			;383c
	sbc a,a			;383d
	jr nc,l3842h		;383e
	inc h			;3840
	ld a,(hl)			;3841
l3842h:
	ld l,a			;3842
	ld a,0ffh		;3843
	bit 1,(iy+009h)		;3845
	jp z,l384fh		;3849
	ld a,(wMasterVolume_F805)		;384c
l384fh:
	and a			;384f
	jr z,l3867h		;3850
	inc l			;3852
	jr z,l3867h		;3853
	ex af,af'			;3855
	dec l			;3856
	ld a,l			;3857
	jr z,l3867h		;3858
	ld h,014h		;385a
	ld a,(hl)			;385c
	ex af,af'			;385d
	ld l,a			;385e
	ex af,af'			;385f
	add a,(hl)			;3860
	ld l,a			;3861
	sbc a,a			;3862
	jr nc,l3867h		;3863
	inc h			;3865
	ld a,(hl)			;3866
l3867h:
	ld hl,0fab5h		;3867
	cp (hl)			;386a
	ld (hl),a			;386b
	ex af,af'			;386c
	ld a,(0fabah)		;386d
	and a			;3870
	jr nz,l3895h		;3871
	ld a,(iy+00fh)		;3873
	and 003h		;3876
	ld hl,0fab9h		;3878
	cp (hl)			;387b
	ld (hl),a			;387c
	ld e,a			;387d
	jp z,l3885h		;387e
	ex af,af'			;3881
	jp l3889h		;3882
l3885h:
	ex af,af'			;3885
	jp z,l38a0h		;3886
l3889h:
	ld h,019h		;3889
	ld l,a			;388b
	ld a,(hl)			;388c
	sub 001h		;388d
	jr nc,l3895h		;388f
	xor a			;3891
	jp l389ah		;3892
l3895h:
	rrc e		;3895
	rrc e		;3897
	or e			;3899
l389ah:
	ld e,a			;389a
	ld d,008h		;389b
	ld c,006h		;389d
	rst 8			;389f
l38a0h:
	ld c,001h		;38a0
	bit 1,(iy+009h)		;38a2
	jr z,l38ach		;38a6
	ld a,(wTempoStep_F809)		;38a8
	ld c,a			;38ab
l38ach:
	ld hl,0faabh		;38ac
	ld a,(hl)			;38af
	add a,c			;38b0
	sbc a,000h		;38b1
	ld (hl),a			;38b3
	ld hl,0faaah		;38b4
	ld a,(hl)			;38b7
	and a			;38b8
	jp z,l38cdh		;38b9
	sub c			;38bc
	ld (hl),a			;38bd
	jr nz,l38e2h		;38be
	bit 4,(iy+009h)		;38c0
	jr z,l38dch		;38c4
	set 4,(ix+003h)		;38c6
	jp l38e2h		;38ca
l38cdh:
	bit 4,(iy+009h)		;38cd
	jr nz,l38e2h		;38d1
	ld hl,0faa8h		;38d3
	bit 4,(hl)		;38d6
	jr z,l38e2h		;38d8
	res 4,(hl)		;38da
l38dch:
	ld hl,0fac0h		;38dc
	call Env_Release_2219		;38df
l38e2h:
	ld ix,wPCMA_Ch2_FACE		;38e2
	ld iy,(wPCMA_Ch2_FACE)		;38e6
	bit 3,(iy+009h)		;38ea
	call nz,Ch_ResetStruct_201B		;38ee
	ld hl,0fae8h		;38f1
	ld a,(hl)			;38f4
	and a			;38f5
	jp z,l39e6h		;38f6
	cp 002h		;38f9
	jr z,l390fh		;38fb
	ld a,(wYmStatus_F80A)		;38fd
	and 002h		;3900
	jp z,l394bh		;3902
	xor a			;3905
	ld (0fae8h),a		;3906
	ld (0fad9h),a		;3909
	jp l39e6h		;390c
l390fh:
	ld hl,(0fae4h)		;390f
	ld bc,(0fae6h)		;3912
	ld a,001h		;3916
	ld (0fae8h),a		;3918
	push bc			;391b
	ld de,00900h		;391c  YM reg 09h <- 00h
	ld c,006h		;391f
	rst 8			;3921
	xor a			;3922
	ld (0fadeh),a		;3923
	ld de,00082h		;3926  YM reg 00h <- 82h
	rst 8			;3929
	ld de,01c02h		;392a  YM reg 1Ch <- 02h
	ld c,004h		;392d
	rst 8			;392f
	ld e,000h		;3930
	rst 8			;3932
	ld c,006h		;3933
	ld e,l			;3935
	ld d,011h		;3936
	rst 8			;3938
	ld e,h			;3939
	ld d,019h		;393a
	pop hl			;393c
	rst 8			;393d
	ld e,l			;393e
	ld d,021h		;393f
	rst 8			;3941
	ld e,h			;3942
	ld d,029h		;3943
	rst 8			;3945
	ld e,002h		;3946
	ld d,000h		;3948
	rst 8			;394a
l394bh:
	ld hl,0fae9h		;394b
	call Env_Step_222D		;394e
	ld l,a			;3951
	ld a,(0fad9h)		;3952
	and a			;3955
	jr z,l396dh		;3956
	inc l			;3958
	jr z,l396dh		;3959
	ex af,af'			;395b
	dec l			;395c
	ld a,l			;395d
	jr z,l396dh		;395e
	ld h,014h		;3960
	ld a,(hl)			;3962
	ex af,af'			;3963
	ld l,a			;3964
	ex af,af'			;3965
	add a,(hl)			;3966
	ld l,a			;3967
	sbc a,a			;3968
	jr nc,l396dh		;3969
	inc h			;396b
	ld a,(hl)			;396c
l396dh:
	ld l,(iy+012h)		;396d
	and a			;3970
	jr z,l3988h		;3971
	inc l			;3973
	jr z,l3988h		;3974
	ex af,af'			;3976
	dec l			;3977
	ld a,l			;3978
	jr z,l3988h		;3979
	ld h,014h		;397b
	ld a,(hl)			;397d
	ex af,af'			;397e
	ld l,a			;397f
	ex af,af'			;3980
	add a,(hl)			;3981
	ld l,a			;3982
	sbc a,a			;3983
	jr nc,l3988h		;3984
	inc h			;3986
	ld a,(hl)			;3987
l3988h:
	ld l,a			;3988
	ld a,0ffh		;3989
	bit 1,(iy+009h)		;398b
	jp z,l3995h		;398f
	ld a,(wMasterVolume_F805)		;3992
l3995h:
	and a			;3995
	jr z,l39adh		;3996
	inc l			;3998
	jr z,l39adh		;3999
	ex af,af'			;399b
	dec l			;399c
	ld a,l			;399d
	jr z,l39adh		;399e
	ld h,014h		;39a0
	ld a,(hl)			;39a2
	ex af,af'			;39a3
	ld l,a			;39a4
	ex af,af'			;39a5
	add a,(hl)			;39a6
	ld l,a			;39a7
	sbc a,a			;39a8
	jr nc,l39adh		;39a9
	inc h			;39ab
	ld a,(hl)			;39ac
l39adh:
	ld hl,0fadeh		;39ad
	cp (hl)			;39b0
	ld (hl),a			;39b1
	ex af,af'			;39b2
	ld a,(0fae3h)		;39b3
	and a			;39b6
	jr nz,l39dbh		;39b7
	ld a,(iy+00fh)		;39b9
	and 003h		;39bc
	ld hl,0fae2h		;39be
	cp (hl)			;39c1
	ld (hl),a			;39c2
	ld e,a			;39c3
	jp z,l39cbh		;39c4
	ex af,af'			;39c7
	jp l39cfh		;39c8
l39cbh:
	ex af,af'			;39cb
	jp z,l39e6h		;39cc
l39cfh:
	ld h,019h		;39cf
	ld l,a			;39d1
	ld a,(hl)			;39d2
	sub 001h		;39d3
	jr nc,l39dbh		;39d5
	xor a			;39d7
	jp l39e0h		;39d8
l39dbh:
	rrc e		;39db
	rrc e		;39dd
	or e			;39df
l39e0h:
	ld e,a			;39e0
	ld d,009h		;39e1
	ld c,006h		;39e3
	rst 8			;39e5
l39e6h:
	ld c,001h		;39e6
	bit 1,(iy+009h)		;39e8
	jr z,l39f2h		;39ec
	ld a,(wTempoStep_F809)		;39ee
	ld c,a			;39f1
l39f2h:
	ld hl,0fad4h		;39f2
	ld a,(hl)			;39f5
	add a,c			;39f6
	sbc a,000h		;39f7
	ld (hl),a			;39f9
	ld hl,0fad3h		;39fa
	ld a,(hl)			;39fd
	and a			;39fe
	jp z,l3a13h		;39ff
	sub c			;3a02
	ld (hl),a			;3a03
	jr nz,l3a28h		;3a04
	bit 4,(iy+009h)		;3a06
	jr z,l3a22h		;3a0a
	set 4,(ix+003h)		;3a0c
	jp l3a28h		;3a10
l3a13h:
	bit 4,(iy+009h)		;3a13
	jr nz,l3a28h		;3a17
	ld hl,0fad1h		;3a19
	bit 4,(hl)		;3a1c
	jr z,l3a28h		;3a1e
	res 4,(hl)		;3a20
l3a22h:
	ld hl,0fae9h		;3a22
	call Env_Release_2219		;3a25
l3a28h:
	ld ix,wPCMA_Ch3_FAF7		;3a28
	ld iy,(wPCMA_Ch3_FAF7)		;3a2c
	bit 3,(iy+009h)		;3a30
	call nz,Ch_ResetStruct_201B		;3a34
	ld hl,0fb11h		;3a37
	ld a,(hl)			;3a3a
	and a			;3a3b
	jp z,l3b2ch		;3a3c
	cp 002h		;3a3f
	jr z,l3a55h		;3a41
	ld a,(wYmStatus_F80A)		;3a43
	and 004h		;3a46
	jp z,l3a91h		;3a48
	xor a			;3a4b
	ld (0fb11h),a		;3a4c
	ld (0fb02h),a		;3a4f
	jp l3b2ch		;3a52
l3a55h:
	ld hl,(0fb0dh)		;3a55
	ld bc,(0fb0fh)		;3a58
	ld a,001h		;3a5c
	ld (0fb11h),a		;3a5e
	push bc			;3a61
	ld de,00a00h		;3a62  YM reg 0Ah <- 00h
	ld c,006h		;3a65
	rst 8			;3a67
	xor a			;3a68
	ld (0fb07h),a		;3a69
	ld de,00084h		;3a6c  YM reg 00h <- 84h
	rst 8			;3a6f
	ld de,01c04h		;3a70  YM reg 1Ch <- 04h
	ld c,004h		;3a73
	rst 8			;3a75
	ld e,000h		;3a76
	rst 8			;3a78
	ld c,006h		;3a79
	ld e,l			;3a7b
	ld d,012h		;3a7c
	rst 8			;3a7e
	ld e,h			;3a7f
	ld d,01ah		;3a80
	pop hl			;3a82
	rst 8			;3a83
	ld e,l			;3a84
	ld d,022h		;3a85
	rst 8			;3a87
	ld e,h			;3a88
	ld d,02ah		;3a89
	rst 8			;3a8b
	ld e,004h		;3a8c
	ld d,000h		;3a8e
	rst 8			;3a90
l3a91h:
	ld hl,0fb12h		;3a91
	call Env_Step_222D		;3a94
	ld l,a			;3a97
	ld a,(0fb02h)		;3a98
	and a			;3a9b
	jr z,l3ab3h		;3a9c
	inc l			;3a9e
	jr z,l3ab3h		;3a9f
	ex af,af'			;3aa1
	dec l			;3aa2
	ld a,l			;3aa3
	jr z,l3ab3h		;3aa4
	ld h,014h		;3aa6
	ld a,(hl)			;3aa8
	ex af,af'			;3aa9
	ld l,a			;3aaa
	ex af,af'			;3aab
	add a,(hl)			;3aac
	ld l,a			;3aad
	sbc a,a			;3aae
	jr nc,l3ab3h		;3aaf
	inc h			;3ab1
	ld a,(hl)			;3ab2
l3ab3h:
	ld l,(iy+012h)		;3ab3
	and a			;3ab6
	jr z,l3aceh		;3ab7
	inc l			;3ab9
	jr z,l3aceh		;3aba
	ex af,af'			;3abc
	dec l			;3abd
	ld a,l			;3abe
	jr z,l3aceh		;3abf
	ld h,014h		;3ac1
	ld a,(hl)			;3ac3
	ex af,af'			;3ac4
	ld l,a			;3ac5
	ex af,af'			;3ac6
	add a,(hl)			;3ac7
	ld l,a			;3ac8
	sbc a,a			;3ac9
	jr nc,l3aceh		;3aca
	inc h			;3acc
	ld a,(hl)			;3acd
l3aceh:
	ld l,a			;3ace
	ld a,0ffh		;3acf
	bit 1,(iy+009h)		;3ad1
	jp z,l3adbh		;3ad5
	ld a,(wMasterVolume_F805)		;3ad8
l3adbh:
	and a			;3adb
	jr z,l3af3h		;3adc
	inc l			;3ade
	jr z,l3af3h		;3adf
	ex af,af'			;3ae1
	dec l			;3ae2
	ld a,l			;3ae3
	jr z,l3af3h		;3ae4
	ld h,014h		;3ae6
	ld a,(hl)			;3ae8
	ex af,af'			;3ae9
	ld l,a			;3aea
	ex af,af'			;3aeb
	add a,(hl)			;3aec
	ld l,a			;3aed
	sbc a,a			;3aee
	jr nc,l3af3h		;3aef
	inc h			;3af1
	ld a,(hl)			;3af2
l3af3h:
	ld hl,0fb07h		;3af3
	cp (hl)			;3af6
	ld (hl),a			;3af7
	ex af,af'			;3af8
	ld a,(0fb0ch)		;3af9
	and a			;3afc
	jr nz,l3b21h		;3afd
	ld a,(iy+00fh)		;3aff
	and 003h		;3b02
	ld hl,0fb0bh		;3b04
	cp (hl)			;3b07
	ld (hl),a			;3b08
	ld e,a			;3b09
	jp z,l3b11h		;3b0a
	ex af,af'			;3b0d
	jp l3b15h		;3b0e
l3b11h:
	ex af,af'			;3b11
	jp z,l3b2ch		;3b12
l3b15h:
	ld h,019h		;3b15
	ld l,a			;3b17
	ld a,(hl)			;3b18
	sub 001h		;3b19
	jr nc,l3b21h		;3b1b
	xor a			;3b1d
	jp l3b26h		;3b1e
l3b21h:
	rrc e		;3b21
	rrc e		;3b23
	or e			;3b25
l3b26h:
	ld e,a			;3b26
	ld d,00ah		;3b27
	ld c,006h		;3b29
	rst 8			;3b2b
l3b2ch:
	ld c,001h		;3b2c
	bit 1,(iy+009h)		;3b2e
	jr z,l3b38h		;3b32
	ld a,(wTempoStep_F809)		;3b34
	ld c,a			;3b37
l3b38h:
	ld hl,0fafdh		;3b38
	ld a,(hl)			;3b3b
	add a,c			;3b3c
	sbc a,000h		;3b3d
	ld (hl),a			;3b3f
	ld hl,0fafch		;3b40
	ld a,(hl)			;3b43
	and a			;3b44
	jp z,l3b59h		;3b45
	sub c			;3b48
	ld (hl),a			;3b49
	jr nz,l3b6eh		;3b4a
	bit 4,(iy+009h)		;3b4c
	jr z,l3b68h		;3b50
	set 4,(ix+003h)		;3b52
	jp l3b6eh		;3b56
l3b59h:
	bit 4,(iy+009h)		;3b59
	jr nz,l3b6eh		;3b5d
	ld hl,0fafah		;3b5f
	bit 4,(hl)		;3b62
	jr z,l3b6eh		;3b64
	res 4,(hl)		;3b66
l3b68h:
	ld hl,0fb12h		;3b68
	call Env_Release_2219		;3b6b
l3b6eh:
	ld ix,wPCMA_Ch4_FB20		;3b6e
	ld iy,(wPCMA_Ch4_FB20)		;3b72
	bit 3,(iy+009h)		;3b76
	call nz,Ch_ResetStruct_201B		;3b7a
	ld hl,0fb3ah		;3b7d
	ld a,(hl)			;3b80
	and a			;3b81
	jp z,l3c72h		;3b82
	cp 002h		;3b85
	jr z,l3b9bh		;3b87
	ld a,(wYmStatus_F80A)		;3b89
	and 008h		;3b8c
	jp z,l3bd7h		;3b8e
	xor a			;3b91
	ld (0fb3ah),a		;3b92
	ld (0fb2bh),a		;3b95
	jp l3c72h		;3b98
l3b9bh:
	ld hl,(0fb36h)		;3b9b
	ld bc,(0fb38h)		;3b9e
	ld a,001h		;3ba2
	ld (0fb3ah),a		;3ba4
	push bc			;3ba7
	ld de,00b00h		;3ba8  YM reg 0Bh <- 00h
	ld c,006h		;3bab
	rst 8			;3bad
	xor a			;3bae
	ld (0fb30h),a		;3baf
	ld de,00088h		;3bb2  YM reg 00h <- 88h
	rst 8			;3bb5
	ld de,01c08h		;3bb6  YM reg 1Ch <- 08h
	ld c,004h		;3bb9
	rst 8			;3bbb
	ld e,000h		;3bbc
	rst 8			;3bbe
	ld c,006h		;3bbf
	ld e,l			;3bc1
	ld d,013h		;3bc2
	rst 8			;3bc4
	ld e,h			;3bc5
	ld d,01bh		;3bc6
	pop hl			;3bc8
	rst 8			;3bc9
	ld e,l			;3bca
	ld d,023h		;3bcb
	rst 8			;3bcd
	ld e,h			;3bce
	ld d,02bh		;3bcf
	rst 8			;3bd1
	ld e,008h		;3bd2
	ld d,000h		;3bd4
	rst 8			;3bd6
l3bd7h:
	ld hl,0fb3bh		;3bd7
	call Env_Step_222D		;3bda
	ld l,a			;3bdd
	ld a,(0fb2bh)		;3bde
	and a			;3be1
	jr z,l3bf9h		;3be2
	inc l			;3be4
	jr z,l3bf9h		;3be5
	ex af,af'			;3be7
	dec l			;3be8
	ld a,l			;3be9
	jr z,l3bf9h		;3bea
	ld h,014h		;3bec
	ld a,(hl)			;3bee
	ex af,af'			;3bef
	ld l,a			;3bf0
	ex af,af'			;3bf1
	add a,(hl)			;3bf2
	ld l,a			;3bf3
	sbc a,a			;3bf4
	jr nc,l3bf9h		;3bf5
	inc h			;3bf7
	ld a,(hl)			;3bf8
l3bf9h:
	ld l,(iy+012h)		;3bf9
	and a			;3bfc
	jr z,l3c14h		;3bfd
	inc l			;3bff
	jr z,l3c14h		;3c00
	ex af,af'			;3c02
	dec l			;3c03
	ld a,l			;3c04
	jr z,l3c14h		;3c05
	ld h,014h		;3c07
	ld a,(hl)			;3c09
	ex af,af'			;3c0a
	ld l,a			;3c0b
	ex af,af'			;3c0c
	add a,(hl)			;3c0d
	ld l,a			;3c0e
	sbc a,a			;3c0f
	jr nc,l3c14h		;3c10
	inc h			;3c12
	ld a,(hl)			;3c13
l3c14h:
	ld l,a			;3c14
	ld a,0ffh		;3c15
	bit 1,(iy+009h)		;3c17
	jp z,l3c21h		;3c1b
	ld a,(wMasterVolume_F805)		;3c1e
l3c21h:
	and a			;3c21
	jr z,l3c39h		;3c22
	inc l			;3c24
	jr z,l3c39h		;3c25
	ex af,af'			;3c27
	dec l			;3c28
	ld a,l			;3c29
	jr z,l3c39h		;3c2a
	ld h,014h		;3c2c
	ld a,(hl)			;3c2e
	ex af,af'			;3c2f
	ld l,a			;3c30
	ex af,af'			;3c31
	add a,(hl)			;3c32
	ld l,a			;3c33
	sbc a,a			;3c34
	jr nc,l3c39h		;3c35
	inc h			;3c37
	ld a,(hl)			;3c38
l3c39h:
	ld hl,0fb30h		;3c39
	cp (hl)			;3c3c
	ld (hl),a			;3c3d
	ex af,af'			;3c3e
	ld a,(0fb35h)		;3c3f
	and a			;3c42
	jr nz,l3c67h		;3c43
	ld a,(iy+00fh)		;3c45
	and 003h		;3c48
	ld hl,0fb34h		;3c4a
	cp (hl)			;3c4d
	ld (hl),a			;3c4e
	ld e,a			;3c4f
	jp z,l3c57h		;3c50
	ex af,af'			;3c53
	jp l3c5bh		;3c54
l3c57h:
	ex af,af'			;3c57
	jp z,l3c72h		;3c58
l3c5bh:
	ld h,019h		;3c5b
	ld l,a			;3c5d
	ld a,(hl)			;3c5e
	sub 001h		;3c5f
	jr nc,l3c67h		;3c61
	xor a			;3c63
	jp l3c6ch		;3c64
l3c67h:
	rrc e		;3c67
	rrc e		;3c69
	or e			;3c6b
l3c6ch:
	ld e,a			;3c6c
	ld d,00bh		;3c6d
	ld c,006h		;3c6f
	rst 8			;3c71
l3c72h:
	ld c,001h		;3c72
	bit 1,(iy+009h)		;3c74
	jr z,l3c7eh		;3c78
	ld a,(wTempoStep_F809)		;3c7a
	ld c,a			;3c7d
l3c7eh:
	ld hl,0fb26h		;3c7e
	ld a,(hl)			;3c81
	add a,c			;3c82
	sbc a,000h		;3c83
	ld (hl),a			;3c85
	ld hl,0fb25h		;3c86
	ld a,(hl)			;3c89
	and a			;3c8a
	jp z,l3c9fh		;3c8b
	sub c			;3c8e
	ld (hl),a			;3c8f
	jr nz,l3cb4h		;3c90
	bit 4,(iy+009h)		;3c92
	jr z,l3caeh		;3c96
	set 4,(ix+003h)		;3c98
	jp l3cb4h		;3c9c
l3c9fh:
	bit 4,(iy+009h)		;3c9f
	jr nz,l3cb4h		;3ca3
	ld hl,0fb23h		;3ca5
	bit 4,(hl)		;3ca8
	jr z,l3cb4h		;3caa
	res 4,(hl)		;3cac
l3caeh:
	ld hl,0fb3bh		;3cae
	call Env_Release_2219		;3cb1
l3cb4h:
	ld ix,wPCMA_Ch5_FB49		;3cb4
	ld iy,(wPCMA_Ch5_FB49)		;3cb8
	bit 3,(iy+009h)		;3cbc
	call nz,Ch_ResetStruct_201B		;3cc0
	ld hl,0fb63h		;3cc3
	ld a,(hl)			;3cc6
	and a			;3cc7
	jp z,l3db8h		;3cc8
	cp 002h		;3ccb
	jr z,l3ce1h		;3ccd
	ld a,(wYmStatus_F80A)		;3ccf
	and 010h		;3cd2
	jp z,l3d1dh		;3cd4
	xor a			;3cd7
	ld (0fb63h),a		;3cd8
	ld (0fb54h),a		;3cdb
	jp l3db8h		;3cde
l3ce1h:
	ld hl,(0fb5fh)		;3ce1
	ld bc,(0fb61h)		;3ce4
	ld a,001h		;3ce8
	ld (0fb63h),a		;3cea
	push bc			;3ced
	ld de,00c00h		;3cee  YM reg 0Ch <- 00h
	ld c,006h		;3cf1
	rst 8			;3cf3
	xor a			;3cf4
	ld (0fb59h),a		;3cf5
	ld de,00090h		;3cf8  YM reg 00h <- 90h
	rst 8			;3cfb
	ld de,01c10h		;3cfc  YM reg 1Ch <- 10h
	ld c,004h		;3cff
	rst 8			;3d01
	ld e,000h		;3d02
	rst 8			;3d04
	ld c,006h		;3d05
	ld e,l			;3d07
	ld d,014h		;3d08
	rst 8			;3d0a
	ld e,h			;3d0b
	ld d,01ch		;3d0c
	pop hl			;3d0e
	rst 8			;3d0f
	ld e,l			;3d10
	ld d,024h		;3d11
	rst 8			;3d13
	ld e,h			;3d14
	ld d,02ch		;3d15
	rst 8			;3d17
	ld e,010h		;3d18
	ld d,000h		;3d1a
	rst 8			;3d1c
l3d1dh:
	ld hl,0fb64h		;3d1d
	call Env_Step_222D		;3d20
	ld l,a			;3d23
	ld a,(0fb54h)		;3d24
	and a			;3d27
	jr z,l3d3fh		;3d28
	inc l			;3d2a
	jr z,l3d3fh		;3d2b
	ex af,af'			;3d2d
	dec l			;3d2e
	ld a,l			;3d2f
	jr z,l3d3fh		;3d30
	ld h,014h		;3d32
	ld a,(hl)			;3d34
	ex af,af'			;3d35
	ld l,a			;3d36
	ex af,af'			;3d37
	add a,(hl)			;3d38
	ld l,a			;3d39
	sbc a,a			;3d3a
	jr nc,l3d3fh		;3d3b
	inc h			;3d3d
	ld a,(hl)			;3d3e
l3d3fh:
	ld l,(iy+012h)		;3d3f
	and a			;3d42
	jr z,l3d5ah		;3d43
	inc l			;3d45
	jr z,l3d5ah		;3d46
	ex af,af'			;3d48
	dec l			;3d49
	ld a,l			;3d4a
	jr z,l3d5ah		;3d4b
	ld h,014h		;3d4d
	ld a,(hl)			;3d4f
	ex af,af'			;3d50
	ld l,a			;3d51
	ex af,af'			;3d52
	add a,(hl)			;3d53
	ld l,a			;3d54
	sbc a,a			;3d55
	jr nc,l3d5ah		;3d56
	inc h			;3d58
	ld a,(hl)			;3d59
l3d5ah:
	ld l,a			;3d5a
	ld a,0ffh		;3d5b
	bit 1,(iy+009h)		;3d5d
	jp z,l3d67h		;3d61
	ld a,(wMasterVolume_F805)		;3d64
l3d67h:
	and a			;3d67
	jr z,l3d7fh		;3d68
	inc l			;3d6a
	jr z,l3d7fh		;3d6b
	ex af,af'			;3d6d
	dec l			;3d6e
	ld a,l			;3d6f
	jr z,l3d7fh		;3d70
	ld h,014h		;3d72
	ld a,(hl)			;3d74
	ex af,af'			;3d75
	ld l,a			;3d76
	ex af,af'			;3d77
	add a,(hl)			;3d78
	ld l,a			;3d79
	sbc a,a			;3d7a
	jr nc,l3d7fh		;3d7b
	inc h			;3d7d
	ld a,(hl)			;3d7e
l3d7fh:
	ld hl,0fb59h		;3d7f
	cp (hl)			;3d82
	ld (hl),a			;3d83
	ex af,af'			;3d84
	ld a,(0fb5eh)		;3d85
	and a			;3d88
	jr nz,l3dadh		;3d89
	ld a,(iy+00fh)		;3d8b
	and 003h		;3d8e
	ld hl,0fb5dh		;3d90
	cp (hl)			;3d93
	ld (hl),a			;3d94
	ld e,a			;3d95
	jp z,l3d9dh		;3d96
	ex af,af'			;3d99
	jp l3da1h		;3d9a
l3d9dh:
	ex af,af'			;3d9d
	jp z,l3db8h		;3d9e
l3da1h:
	ld h,019h		;3da1
	ld l,a			;3da3
	ld a,(hl)			;3da4
	sub 001h		;3da5
	jr nc,l3dadh		;3da7
	xor a			;3da9
	jp l3db2h		;3daa
l3dadh:
	rrc e		;3dad
	rrc e		;3daf
	or e			;3db1
l3db2h:
	ld e,a			;3db2
	ld d,00ch		;3db3
	ld c,006h		;3db5
	rst 8			;3db7
l3db8h:
	ld c,001h		;3db8
	bit 1,(iy+009h)		;3dba
	jr z,l3dc4h		;3dbe
	ld a,(wTempoStep_F809)		;3dc0
	ld c,a			;3dc3
l3dc4h:
	ld hl,0fb4fh		;3dc4
	ld a,(hl)			;3dc7
	add a,c			;3dc8
	sbc a,000h		;3dc9
	ld (hl),a			;3dcb
	ld hl,0fb4eh		;3dcc
	ld a,(hl)			;3dcf
	and a			;3dd0
	jp z,l3de5h		;3dd1
	sub c			;3dd4
	ld (hl),a			;3dd5
	jr nz,l3dfah		;3dd6
	bit 4,(iy+009h)		;3dd8
	jr z,l3df4h		;3ddc
	set 4,(ix+003h)		;3dde
	jp l3dfah		;3de2
l3de5h:
	bit 4,(iy+009h)		;3de5
	jr nz,l3dfah		;3de9
	ld hl,0fb4ch		;3deb
	bit 4,(hl)		;3dee
	jr z,l3dfah		;3df0
	res 4,(hl)		;3df2
l3df4h:
	ld hl,0fb64h		;3df4
	call Env_Release_2219		;3df7
l3dfah:
	ld ix,wPCMA_Ch6_FB72		;3dfa
	ld iy,(wPCMA_Ch6_FB72)		;3dfe
	bit 3,(iy+009h)		;3e02
	call nz,Ch_ResetStruct_201B		;3e06
	ld hl,0fb8ch		;3e09
	ld a,(hl)			;3e0c
	and a			;3e0d
	jp z,l3efeh		;3e0e
	cp 002h		;3e11
	jr z,l3e27h		;3e13
	ld a,(wYmStatus_F80A)		;3e15
	and 020h		;3e18
	jp z,l3e63h		;3e1a
	xor a			;3e1d
	ld (0fb8ch),a		;3e1e
	ld (0fb7dh),a		;3e21
	jp l3efeh		;3e24
l3e27h:
	ld hl,(0fb88h)		;3e27
	ld bc,(0fb8ah)		;3e2a
	ld a,001h		;3e2e
	ld (0fb8ch),a		;3e30
	push bc			;3e33
	ld de,00d00h		;3e34  YM reg 0Dh <- 00h
	ld c,006h		;3e37
	rst 8			;3e39
	xor a			;3e3a
	ld (0fb82h),a		;3e3b
	ld de,000a0h		;3e3e  YM reg 00h <- A0h
	rst 8			;3e41
	ld de,01c20h		;3e42  YM reg 1Ch <- 20h
	ld c,004h		;3e45
	rst 8			;3e47
	ld e,000h		;3e48
	rst 8			;3e4a
	ld c,006h		;3e4b
	ld e,l			;3e4d
	ld d,015h		;3e4e
	rst 8			;3e50
	ld e,h			;3e51
	ld d,01dh		;3e52
	pop hl			;3e54
	rst 8			;3e55
	ld e,l			;3e56
	ld d,025h		;3e57
	rst 8			;3e59
	ld e,h			;3e5a
	ld d,02dh		;3e5b
	rst 8			;3e5d
	ld e,020h		;3e5e
	ld d,000h		;3e60
	rst 8			;3e62
l3e63h:
	ld hl,0fb8dh		;3e63
	call Env_Step_222D		;3e66
	ld l,a			;3e69
	ld a,(0fb7dh)		;3e6a
	and a			;3e6d
	jr z,l3e85h		;3e6e
	inc l			;3e70
	jr z,l3e85h		;3e71
	ex af,af'			;3e73
	dec l			;3e74
	ld a,l			;3e75
	jr z,l3e85h		;3e76
	ld h,014h		;3e78
	ld a,(hl)			;3e7a
	ex af,af'			;3e7b
	ld l,a			;3e7c
	ex af,af'			;3e7d
	add a,(hl)			;3e7e
	ld l,a			;3e7f
	sbc a,a			;3e80
	jr nc,l3e85h		;3e81
	inc h			;3e83
	ld a,(hl)			;3e84
l3e85h:
	ld l,(iy+012h)		;3e85
	and a			;3e88
	jr z,l3ea0h		;3e89
	inc l			;3e8b
	jr z,l3ea0h		;3e8c
	ex af,af'			;3e8e
	dec l			;3e8f
	ld a,l			;3e90
	jr z,l3ea0h		;3e91
	ld h,014h		;3e93
	ld a,(hl)			;3e95
	ex af,af'			;3e96
	ld l,a			;3e97
	ex af,af'			;3e98
	add a,(hl)			;3e99
	ld l,a			;3e9a
	sbc a,a			;3e9b
	jr nc,l3ea0h		;3e9c
	inc h			;3e9e
	ld a,(hl)			;3e9f
l3ea0h:
	ld l,a			;3ea0
	ld a,0ffh		;3ea1
	bit 1,(iy+009h)		;3ea3
	jp z,l3eadh		;3ea7
	ld a,(wMasterVolume_F805)		;3eaa
l3eadh:
	and a			;3ead
	jr z,l3ec5h		;3eae
	inc l			;3eb0
	jr z,l3ec5h		;3eb1
	ex af,af'			;3eb3
	dec l			;3eb4
	ld a,l			;3eb5
	jr z,l3ec5h		;3eb6
	ld h,014h		;3eb8
	ld a,(hl)			;3eba
	ex af,af'			;3ebb
	ld l,a			;3ebc
	ex af,af'			;3ebd
	add a,(hl)			;3ebe
	ld l,a			;3ebf
	sbc a,a			;3ec0
	jr nc,l3ec5h		;3ec1
	inc h			;3ec3
	ld a,(hl)			;3ec4
l3ec5h:
	ld hl,0fb82h		;3ec5
	cp (hl)			;3ec8
	ld (hl),a			;3ec9
	ex af,af'			;3eca
	ld a,(0fb87h)		;3ecb
	and a			;3ece
	jr nz,l3ef3h		;3ecf
	ld a,(iy+00fh)		;3ed1
	and 003h		;3ed4
	ld hl,0fb86h		;3ed6
	cp (hl)			;3ed9
	ld (hl),a			;3eda
	ld e,a			;3edb
	jp z,l3ee3h		;3edc
	ex af,af'			;3edf
	jp l3ee7h		;3ee0
l3ee3h:
	ex af,af'			;3ee3
	jp z,l3efeh		;3ee4
l3ee7h:
	ld h,019h		;3ee7
	ld l,a			;3ee9
	ld a,(hl)			;3eea
	sub 001h		;3eeb
	jr nc,l3ef3h		;3eed
	xor a			;3eef
	jp l3ef8h		;3ef0
l3ef3h:
	rrc e		;3ef3
	rrc e		;3ef5
	or e			;3ef7
l3ef8h:
	ld e,a			;3ef8
	ld d,00dh		;3ef9
	ld c,006h		;3efb
	rst 8			;3efd
l3efeh:
	ld c,001h		;3efe
	bit 1,(iy+009h)		;3f00
	jr z,l3f0ah		;3f04
	ld a,(wTempoStep_F809)		;3f06
	ld c,a			;3f09
l3f0ah:
	ld hl,0fb78h		;3f0a
	ld a,(hl)			;3f0d
	add a,c			;3f0e
	sbc a,000h		;3f0f
	ld (hl),a			;3f11
	ld hl,0fb77h		;3f12
	ld a,(hl)			;3f15
	and a			;3f16
	jp z,l3f2bh		;3f17
	sub c			;3f1a
	ld (hl),a			;3f1b
	jr nz,l3f40h		;3f1c
	bit 4,(iy+009h)		;3f1e
	jr z,l3f3ah		;3f22
	set 4,(ix+003h)		;3f24
	jp l3f40h		;3f28
l3f2bh:
	bit 4,(iy+009h)		;3f2b
	jr nz,l3f40h		;3f2f
	ld hl,0fb75h		;3f31
	bit 4,(hl)		;3f34
	jr z,l3f40h		;3f36
	res 4,(hl)		;3f38
l3f3ah:
	ld hl,0fb8dh		;3f3a
	call Env_Release_2219		;3f3d
l3f40h:
	ret			;3f40
ADPCMA_LoadPatch_3F41:
	exx			;3f41
	ld (ix+011h),b		;3f42
	push ix		;3f45
	ld de,00001h		;3f47  constant
	add hl,de			;3f4a
	ld de,0001bh		;3f4b
	add ix,de		;3f4e
	call Seq_LoadOpPtrs_219B		;3f50
	pop ix		;3f53
	ld de,00006h		;3f55  constant
	add hl,de			;3f58
	ld (ix+012h),l		;3f59
	ld (ix+013h),h		;3f5c
	ret			;3f5f
ADPCMA_SelectSample_3F60:
	ld b,(ix+011h)		;3f60
	ld a,(wCurBank_F811)		;3f63
	cp b			;3f66
	jp z,l3f84h		;3f67
	ld a,b			;3f6a
	ld (wCurBank_F811),a		;3f6b
	ld c,00bh		;3f6e
	in a,(c)		;3f70
	inc b			;3f72
	sla b		;3f73
	dec c			;3f75
	in a,(c)		;3f76
	inc b			;3f78
	sla b		;3f79
	dec c			;3f7b
	in a,(c)		;3f7c
	inc b			;3f7e
	sla b		;3f7f
	dec c			;3f81
	in a,(c)		;3f82
l3f84h:
	ld a,(ix+008h)		;3f84
	ld l,(ix+012h)		;3f87
	ld h,(ix+013h)		;3f8a
	push hl			;3f8d
	ld de,00004h		;3f8e
	add hl,de			;3f91
	sub (hl)			;3f92
	inc hl			;3f93
	ld e,a			;3f94
	ld d,000h		;3f95
	add hl,de			;3f97
	ld e,(hl)			;3f98
	ld l,e			;3f99
	ld h,d			;3f9a
	add hl,hl			;3f9b
	add hl,hl			;3f9c
	add hl,de			;3f9d
	ex (sp),hl			;3f9e
	call Bank_ResolvePtr24_1FA6		;3f9f
	pop de			;3fa2
	add hl,de			;3fa3
	push hl			;3fa4
	defb 0ddh,05dh	;ld e,ixl		;3fa5
	defb 0ddh,054h	;ld d,ixh		;3fa7
	ld hl,00015h		;3fa9  constant
	add hl,de			;3fac
	ex de,hl			;3fad
	pop hl			;3fae
	ldi		;3faf
	ldi		;3fb1
	ldi		;3fb3
	ldi		;3fb5
	ldi		;3fb7
	ret			;3fb9
ADPCMA_KeyOn_3FBA:
	ld (ix+01ah),002h		;3fba
	defb 0ddh,05dh	;ld e,ixl		;3fbe
	defb 0ddh,054h	;ld d,ixh		;3fc0
	ld hl,0001bh		;3fc2
	add hl,de			;3fc5
	call Env_Start_2208		;3fc6
	ret			;3fc9
