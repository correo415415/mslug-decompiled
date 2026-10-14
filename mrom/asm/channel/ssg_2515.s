; ssg_2515.s - M-ROM fixed_0000 [0x2515..0x2b43)
; Carved from mrom/asm/raw/fixed_0000.s by mrom/tools/split_region.py.
; Assembled through mrom/asm/fixed_0000.s; must stay byte-identical.

SSG_Init_2515:
	ld de,00800h		;2515  YM reg 08h <- 00h
	ld c,004h		;2518
	rst 8			;251a
	ld de,00900h		;251b  YM reg 09h <- 00h
	ld c,004h		;251e
	rst 8			;2520
	ld de,00a00h		;2521  YM reg 0Ah <- 00h
	ld c,004h		;2524
	rst 8			;2526
	ld a,038h		;2527
	ld (wSSG_EnvShape_F905),a		;2529
	ld (0f906h),a		;252c
	ld e,a			;252f
	ld d,007h		;2530
	ld c,004h		;2532
	rst 8			;2534
	ld ix,wSSG_ChA_F81F		;2535
	ld (ix+014h),009h		;2539
	call SSG_InitChannel_255D		;253d
	ld ix,wSSG_ChB_F86B		;2540
	ld (ix+014h),012h		;2544
	call SSG_InitChannel_255D		;2548
	ld ix,wSSG_ChC_F8B7		;254b
	ld (ix+014h),024h		;254f
	call SSG_InitChannel_255D		;2553
	ld hl,00000h		;2556  constant
	ld (wSSG_Mixer_F903),hl		;2559
	ret			;255c
SSG_InitChannel_255D:
	call Ch_InitStruct_1FE5		;255d
	ld (ix+002h),000h		;2560
	push ix		;2564
	ld de,00015h		;2566  constant
	add ix,de		;2569
	rst 30h			;256b
	pop ix		;256c
	push ix		;256e
	ld de,00031h		;2570  constant
	add ix,de		;2573
	call Lfo_Clear_229D		;2575
	pop ix		;2578
	push ix		;257a
	ld de,00023h		;257c  constant
	add ix,de		;257f
	call Lfo_Clear_229D		;2581
	pop ix		;2584
	push ix		;2586
	ld de,0003fh		;2588  constant
	add ix,de		;258b
	call Pitch_Clear_23EE		;258d
	pop ix		;2590
	ret			;2592
SSG_Update_2593:
	ld ix,wSSG_ChA_F81F		;2593
	ld iy,(wSSG_ChA_F81F)		;2597
	bit 3,(iy+009h)		;259b
	call nz,Ch_ResetStruct_201B		;259f
	ld hl,0f834h		;25a2
	call Env_Step_222D		;25a5
	ld (wTmpByte_F814),a		;25a8
	and a			;25ab
	jp z,l2686h		;25ac
	ld hl,0f842h		;25af
	ld d,(iy+00dh)		;25b2
	call Env_Update_231E		;25b5
	sub c			;25b8
	dec a			;25b9
	ld l,a			;25ba
	ld a,(wTmpByte_F814)		;25bb
	and a			;25be
	jr z,l25d6h		;25bf
	inc l			;25c1
	jr z,l25d6h		;25c2
	ex af,af'			;25c4
	dec l			;25c5
	ld a,l			;25c6
	jr z,l25d6h		;25c7
	ld h,014h		;25c9
	ld a,(hl)			;25cb
	ex af,af'			;25cc
	ld l,a			;25cd
	ex af,af'			;25ce
	add a,(hl)			;25cf
	ld l,a			;25d0
	sbc a,a			;25d1
	jr nc,l25d6h		;25d2
	inc h			;25d4
	ld a,(hl)			;25d5
l25d6h:
	ld (wTmpByte_F814),a		;25d6
	and a			;25d9
	jp z,l2686h		;25da
	ld a,(0f82ah)		;25dd
	and a			;25e0
	jp z,l2686h		;25e1
	ld a,(0f832h)		;25e4
	and a			;25e7
	jp z,l261eh		;25e8
	ld hl,0f850h		;25eb
	ld d,(iy+00ch)		;25ee
	call Env_Update_231E		;25f1
	ld l,a			;25f4
	ld h,000h		;25f5
	ld b,h			;25f7
	add hl,hl			;25f8
	and a			;25f9
	sbc hl,bc		;25fa
	ld a,l			;25fc
	sra h		;25fd
	rra			;25ff
	sra h		;2600
	rra			;2602
	sra h		;2603
	rra			;2605
	ld hl,0f827h		;2606
	ld b,a			;2609
	ld a,(hl)			;260a
	and 01fh		;260b
	add a,b			;260d
	jp p,l2612h		;260e
	xor a			;2611
l2612h:
	cp 020h		;2612
	jr c,l2618h		;2614
	ld a,01fh		;2616
l2618h:
	ld (wSSG_Mixer_F903),a		;2618
	jp l2686h		;261b
l261eh:
	ld hl,(0f828h)		;261e
	ld de,(0f830h)		;2621
	add hl,de			;2625
	ld e,(iy+010h)		;2626
	ld d,(iy+011h)		;2629
	add hl,de			;262c
	ld (wTmpWord_F812),hl		;262d
	ld hl,0f850h		;2630
	ld d,(iy+00ch)		;2633
	call Env_Update_231E		;2636
	ld l,a			;2639
	ld h,000h		;263a
	ld b,h			;263c
	add hl,hl			;263d
	and a			;263e
	sbc hl,bc		;263f
	ld de,(wTmpWord_F812)		;2641
	add hl,de			;2645
	ld (wTmpWord_F812),hl		;2646
	ld hl,0f85eh		;2649
	call Vib_Update_248E		;264c
	ld de,(wTmpWord_F812)		;264f
	add hl,de			;2653
	ex de,hl			;2654
	ld hl,(0f82dh)		;2655
	and a			;2658
	sbc hl,de		;2659
	jp z,l2686h		;265b
	ld (0f82dh),de		;265e
	ld a,d			;2662
	cp 010h		;2663
	jp c,l266ah		;2665
	ld a,00fh		;2668
l266ah:
	ld b,a			;266a
	and 007h		;266b
	add a,a			;266d
	add a,001h		;266e
	ld h,a			;2670
	ld l,e			;2671
	ld e,(hl)			;2672
	inc h			;2673
	ld d,(hl)			;2674
	ld a,b			;2675
	and 008h		;2676
	jr z,l267dh		;2678
	ld e,d			;267a
	ld d,000h		;267b
l267dh:
	ld b,d			;267d
	ld d,000h		;267e
	ld c,004h		;2680
	rst 8			;2682
	ld e,b			;2683
	inc d			;2684
	rst 8			;2685
l2686h:
	ld a,(wTmpByte_F814)		;2686
	ld l,a			;2689
	ld a,(0f82ah)		;268a
	and a			;268d
	jr z,l26a5h		;268e
	inc l			;2690
	jr z,l26a5h		;2691
	ex af,af'			;2693
	dec l			;2694
	ld a,l			;2695
	jr z,l26a5h		;2696
	ld h,014h		;2698
	ld a,(hl)			;269a
	ex af,af'			;269b
	ld l,a			;269c
	ex af,af'			;269d
	add a,(hl)			;269e
	ld l,a			;269f
	sbc a,a			;26a0
	jr nc,l26a5h		;26a1
	inc h			;26a3
	ld a,(hl)			;26a4
l26a5h:
	ld l,(iy+012h)		;26a5
	and a			;26a8
	jr z,l26c0h		;26a9
	inc l			;26ab
	jr z,l26c0h		;26ac
	ex af,af'			;26ae
	dec l			;26af
	ld a,l			;26b0
	jr z,l26c0h		;26b1
	ld h,014h		;26b3
	ld a,(hl)			;26b5
	ex af,af'			;26b6
	ld l,a			;26b7
	ex af,af'			;26b8
	add a,(hl)			;26b9
	ld l,a			;26ba
	sbc a,a			;26bb
	jr nc,l26c0h		;26bc
	inc h			;26be
	ld a,(hl)			;26bf
l26c0h:
	ld l,a			;26c0
	ld a,0ffh		;26c1
	bit 1,(iy+009h)		;26c3
	jp z,l26cdh		;26c7
	ld a,(wMasterVolume_F805)		;26ca
l26cdh:
	and a			;26cd
	jr z,l26e5h		;26ce
	inc l			;26d0
	jr z,l26e5h		;26d1
	ex af,af'			;26d3
	dec l			;26d4
	ld a,l			;26d5
	jr z,l26e5h		;26d6
	ld h,014h		;26d8
	ld a,(hl)			;26da
	ex af,af'			;26db
	ld l,a			;26dc
	ex af,af'			;26dd
	add a,(hl)			;26de
	ld l,a			;26df
	sbc a,a			;26e0
	jr nc,l26e5h		;26e1
	inc h			;26e3
	ld a,(hl)			;26e4
l26e5h:
	ld hl,0f82fh		;26e5
	cp (hl)			;26e8
	ld (hl),a			;26e9
	jp z,l26f6h		;26ea
	ld l,a			;26ed
	ld h,018h		;26ee
	ld e,(hl)			;26f0
	ld d,008h		;26f1
	ld c,004h		;26f3
	rst 8			;26f5
l26f6h:
	ld c,001h		;26f6
	bit 1,(iy+009h)		;26f8
	jr z,l2702h		;26fc
	ld a,(wTempoStep_F809)		;26fe
	ld c,a			;2701
l2702h:
	ld hl,0f825h		;2702
	ld a,(hl)			;2705
	add a,c			;2706
	sbc a,000h		;2707
	ld (hl),a			;2709
	ld hl,0f824h		;270a
	ld a,(hl)			;270d
	and a			;270e
	jp z,l2723h		;270f
	sub c			;2712
	ld (hl),a			;2713
	jr nz,SSG_UpdateChB_2738		;2714
	bit 4,(iy+009h)		;2716
	jr z,l2732h		;271a
	set 4,(ix+003h)		;271c
	jp SSG_UpdateChB_2738		;2720
l2723h:
	bit 4,(iy+009h)		;2723
	jr nz,SSG_UpdateChB_2738		;2727
	ld hl,0f822h		;2729
	bit 4,(hl)		;272c
	jr z,SSG_UpdateChB_2738		;272e
	res 4,(hl)		;2730
l2732h:
	ld hl,0f834h		;2732
	call Env_Release_2219		;2735
SSG_UpdateChB_2738:
	ld ix,wSSG_ChB_F86B		;2738
	ld iy,(wSSG_ChB_F86B)		;273c
	bit 3,(iy+009h)		;2740
	call nz,Ch_ResetStruct_201B		;2744
	ld hl,0f880h		;2747
	call Env_Step_222D		;274a
	ld (wTmpByte_F814),a		;274d
	and a			;2750
	jp z,l282bh		;2751
	ld hl,0f88eh		;2754
	ld d,(iy+00dh)		;2757
	call Env_Update_231E		;275a
	sub c			;275d
	dec a			;275e
	ld l,a			;275f
	ld a,(wTmpByte_F814)		;2760
	and a			;2763
	jr z,l277bh		;2764
	inc l			;2766
	jr z,l277bh		;2767
	ex af,af'			;2769
	dec l			;276a
	ld a,l			;276b
	jr z,l277bh		;276c
	ld h,014h		;276e
	ld a,(hl)			;2770
	ex af,af'			;2771
	ld l,a			;2772
	ex af,af'			;2773
	add a,(hl)			;2774
	ld l,a			;2775
	sbc a,a			;2776
	jr nc,l277bh		;2777
	inc h			;2779
	ld a,(hl)			;277a
l277bh:
	ld (wTmpByte_F814),a		;277b
	and a			;277e
	jp z,l282bh		;277f
	ld a,(0f876h)		;2782
	and a			;2785
	jp z,l282bh		;2786
	ld a,(0f87eh)		;2789
	and a			;278c
	jp z,l27c3h		;278d
	ld hl,0f89ch		;2790
	ld d,(iy+00ch)		;2793
	call Env_Update_231E		;2796
	ld l,a			;2799
	ld h,000h		;279a
	ld b,h			;279c
	add hl,hl			;279d
	and a			;279e
	sbc hl,bc		;279f
	ld a,l			;27a1
	sra h		;27a2
	rra			;27a4
	sra h		;27a5
	rra			;27a7
	sra h		;27a8
	rra			;27aa
	ld hl,0f873h		;27ab
	ld b,a			;27ae
	ld a,(hl)			;27af
	and 01fh		;27b0
	add a,b			;27b2
	jp p,l27b7h		;27b3
	xor a			;27b6
l27b7h:
	cp 020h		;27b7
	jr c,l27bdh		;27b9
	ld a,01fh		;27bb
l27bdh:
	ld (wSSG_Mixer_F903),a		;27bd
	jp l282bh		;27c0
l27c3h:
	ld hl,(0f874h)		;27c3
	ld de,(0f87ch)		;27c6
	add hl,de			;27ca
	ld e,(iy+010h)		;27cb
	ld d,(iy+011h)		;27ce
	add hl,de			;27d1
	ld (wTmpWord_F812),hl		;27d2
	ld hl,0f89ch		;27d5
	ld d,(iy+00ch)		;27d8
	call Env_Update_231E		;27db
	ld l,a			;27de
	ld h,000h		;27df
	ld b,h			;27e1
	add hl,hl			;27e2
	and a			;27e3
	sbc hl,bc		;27e4
	ld de,(wTmpWord_F812)		;27e6
	add hl,de			;27ea
	ld (wTmpWord_F812),hl		;27eb
	ld hl,0f8aah		;27ee
	call Vib_Update_248E		;27f1
	ld de,(wTmpWord_F812)		;27f4
	add hl,de			;27f8
	ex de,hl			;27f9
	ld hl,(0f879h)		;27fa
	and a			;27fd
	sbc hl,de		;27fe
	jp z,l282bh		;2800
	ld (0f879h),de		;2803
	ld a,d			;2807
	cp 010h		;2808
	jp c,l280fh		;280a
	ld a,00fh		;280d
l280fh:
	ld b,a			;280f
	and 007h		;2810
	add a,a			;2812
	add a,001h		;2813
	ld h,a			;2815
	ld l,e			;2816
	ld e,(hl)			;2817
	inc h			;2818
	ld d,(hl)			;2819
	ld a,b			;281a
	and 008h		;281b
	jr z,l2822h		;281d
	ld e,d			;281f
	ld d,000h		;2820
l2822h:
	ld b,d			;2822
	ld d,002h		;2823
	ld c,004h		;2825
	rst 8			;2827
	ld e,b			;2828
	inc d			;2829
	rst 8			;282a
l282bh:
	ld a,(wTmpByte_F814)		;282b
	ld l,a			;282e
	ld a,(0f876h)		;282f
	and a			;2832
	jr z,l284ah		;2833
	inc l			;2835
	jr z,l284ah		;2836
	ex af,af'			;2838
	dec l			;2839
	ld a,l			;283a
	jr z,l284ah		;283b
	ld h,014h		;283d
	ld a,(hl)			;283f
	ex af,af'			;2840
	ld l,a			;2841
	ex af,af'			;2842
	add a,(hl)			;2843
	ld l,a			;2844
	sbc a,a			;2845
	jr nc,l284ah		;2846
	inc h			;2848
	ld a,(hl)			;2849
l284ah:
	ld l,(iy+012h)		;284a
	and a			;284d
	jr z,l2865h		;284e
	inc l			;2850
	jr z,l2865h		;2851
	ex af,af'			;2853
	dec l			;2854
	ld a,l			;2855
	jr z,l2865h		;2856
	ld h,014h		;2858
	ld a,(hl)			;285a
	ex af,af'			;285b
	ld l,a			;285c
	ex af,af'			;285d
	add a,(hl)			;285e
	ld l,a			;285f
	sbc a,a			;2860
	jr nc,l2865h		;2861
	inc h			;2863
	ld a,(hl)			;2864
l2865h:
	ld l,a			;2865
	ld a,0ffh		;2866
	bit 1,(iy+009h)		;2868
	jp z,l2872h		;286c
	ld a,(wMasterVolume_F805)		;286f
l2872h:
	and a			;2872
	jr z,l288ah		;2873
	inc l			;2875
	jr z,l288ah		;2876
	ex af,af'			;2878
	dec l			;2879
	ld a,l			;287a
	jr z,l288ah		;287b
	ld h,014h		;287d
	ld a,(hl)			;287f
	ex af,af'			;2880
	ld l,a			;2881
	ex af,af'			;2882
	add a,(hl)			;2883
	ld l,a			;2884
	sbc a,a			;2885
	jr nc,l288ah		;2886
	inc h			;2888
	ld a,(hl)			;2889
l288ah:
	ld hl,0f87bh		;288a
	cp (hl)			;288d
	ld (hl),a			;288e
	jp z,l289bh		;288f
	ld l,a			;2892
	ld h,018h		;2893
	ld e,(hl)			;2895
	ld d,009h		;2896
	ld c,004h		;2898
	rst 8			;289a
l289bh:
	ld c,001h		;289b
	bit 1,(iy+009h)		;289d
	jr z,l28a7h		;28a1
	ld a,(wTempoStep_F809)		;28a3
	ld c,a			;28a6
l28a7h:
	ld hl,0f871h		;28a7
	ld a,(hl)			;28aa
	add a,c			;28ab
	sbc a,000h		;28ac
	ld (hl),a			;28ae
	ld hl,0f870h		;28af
	ld a,(hl)			;28b2
	and a			;28b3
	jp z,l28c8h		;28b4
	sub c			;28b7
	ld (hl),a			;28b8
	jr nz,SSG_UpdateChC_28DD		;28b9
	bit 4,(iy+009h)		;28bb
	jr z,l28d7h		;28bf
	set 4,(ix+003h)		;28c1
	jp SSG_UpdateChC_28DD		;28c5
l28c8h:
	bit 4,(iy+009h)		;28c8
	jr nz,SSG_UpdateChC_28DD		;28cc
	ld hl,0f86eh		;28ce
	bit 4,(hl)		;28d1
	jr z,SSG_UpdateChC_28DD		;28d3
	res 4,(hl)		;28d5
l28d7h:
	ld hl,0f880h		;28d7
	call Env_Release_2219		;28da
SSG_UpdateChC_28DD:
	ld ix,wSSG_ChC_F8B7		;28dd
	ld iy,(wSSG_ChC_F8B7)		;28e1
	bit 3,(iy+009h)		;28e5
	call nz,Ch_ResetStruct_201B		;28e9
	ld hl,0f8cch		;28ec
	call Env_Step_222D		;28ef
	ld (wTmpByte_F814),a		;28f2
	and a			;28f5
	jp z,l29d0h		;28f6
	ld hl,0f8dah		;28f9
	ld d,(iy+00dh)		;28fc
	call Env_Update_231E		;28ff
	sub c			;2902
	dec a			;2903
	ld l,a			;2904
	ld a,(wTmpByte_F814)		;2905
	and a			;2908
	jr z,l2920h		;2909
	inc l			;290b
	jr z,l2920h		;290c
	ex af,af'			;290e
	dec l			;290f
	ld a,l			;2910
	jr z,l2920h		;2911
	ld h,014h		;2913
	ld a,(hl)			;2915
	ex af,af'			;2916
	ld l,a			;2917
	ex af,af'			;2918
	add a,(hl)			;2919
	ld l,a			;291a
	sbc a,a			;291b
	jr nc,l2920h		;291c
	inc h			;291e
	ld a,(hl)			;291f
l2920h:
	ld (wTmpByte_F814),a		;2920
	and a			;2923
	jp z,l29d0h		;2924
	ld a,(0f8c2h)		;2927
	and a			;292a
	jp z,l29d0h		;292b
	ld a,(0f8cah)		;292e
	and a			;2931
	jp z,l2968h		;2932
	ld hl,0f8e8h		;2935
	ld d,(iy+00ch)		;2938
	call Env_Update_231E		;293b
	ld l,a			;293e
	ld h,000h		;293f
	ld b,h			;2941
	add hl,hl			;2942
	and a			;2943
	sbc hl,bc		;2944
	ld a,l			;2946
	sra h		;2947
	rra			;2949
	sra h		;294a
	rra			;294c
	sra h		;294d
	rra			;294f
	ld hl,0f8bfh		;2950
	ld b,a			;2953
	ld a,(hl)			;2954
	and 01fh		;2955
	add a,b			;2957
	jp p,l295ch		;2958
	xor a			;295b
l295ch:
	cp 020h		;295c
	jr c,l2962h		;295e
	ld a,01fh		;2960
l2962h:
	ld (wSSG_Mixer_F903),a		;2962
	jp l29d0h		;2965
l2968h:
	ld hl,(0f8c0h)		;2968
	ld de,(0f8c8h)		;296b
	add hl,de			;296f
	ld e,(iy+010h)		;2970
	ld d,(iy+011h)		;2973
	add hl,de			;2976
	ld (wTmpWord_F812),hl		;2977
	ld hl,0f8e8h		;297a
	ld d,(iy+00ch)		;297d
	call Env_Update_231E		;2980
	ld l,a			;2983
	ld h,000h		;2984
	ld b,h			;2986
	add hl,hl			;2987
	and a			;2988
	sbc hl,bc		;2989
	ld de,(wTmpWord_F812)		;298b
	add hl,de			;298f
	ld (wTmpWord_F812),hl		;2990
	ld hl,0f8f6h		;2993
	call Vib_Update_248E		;2996
	ld de,(wTmpWord_F812)		;2999
	add hl,de			;299d
	ex de,hl			;299e
	ld hl,(0f8c5h)		;299f
	and a			;29a2
	sbc hl,de		;29a3
	jp z,l29d0h		;29a5
	ld (0f8c5h),de		;29a8
	ld a,d			;29ac
	cp 010h		;29ad
	jp c,l29b4h		;29af
	ld a,00fh		;29b2
l29b4h:
	ld b,a			;29b4
	and 007h		;29b5
	add a,a			;29b7
	add a,001h		;29b8
	ld h,a			;29ba
	ld l,e			;29bb
	ld e,(hl)			;29bc
	inc h			;29bd
	ld d,(hl)			;29be
	ld a,b			;29bf
	and 008h		;29c0
	jr z,l29c7h		;29c2
	ld e,d			;29c4
	ld d,000h		;29c5
l29c7h:
	ld b,d			;29c7
	ld d,004h		;29c8
	ld c,004h		;29ca
	rst 8			;29cc
	ld e,b			;29cd
	inc d			;29ce
	rst 8			;29cf
l29d0h:
	ld a,(wTmpByte_F814)		;29d0
	ld l,a			;29d3
	ld a,(0f8c2h)		;29d4
	and a			;29d7
	jr z,l29efh		;29d8
	inc l			;29da
	jr z,l29efh		;29db
	ex af,af'			;29dd
	dec l			;29de
	ld a,l			;29df
	jr z,l29efh		;29e0
	ld h,014h		;29e2
	ld a,(hl)			;29e4
	ex af,af'			;29e5
	ld l,a			;29e6
	ex af,af'			;29e7
	add a,(hl)			;29e8
	ld l,a			;29e9
	sbc a,a			;29ea
	jr nc,l29efh		;29eb
	inc h			;29ed
	ld a,(hl)			;29ee
l29efh:
	ld l,(iy+012h)		;29ef
	and a			;29f2
	jr z,l2a0ah		;29f3
	inc l			;29f5
	jr z,l2a0ah		;29f6
	ex af,af'			;29f8
	dec l			;29f9
	ld a,l			;29fa
	jr z,l2a0ah		;29fb
	ld h,014h		;29fd
	ld a,(hl)			;29ff
	ex af,af'			;2a00
	ld l,a			;2a01
	ex af,af'			;2a02
	add a,(hl)			;2a03
	ld l,a			;2a04
	sbc a,a			;2a05
	jr nc,l2a0ah		;2a06
	inc h			;2a08
	ld a,(hl)			;2a09
l2a0ah:
	ld l,a			;2a0a
	ld a,0ffh		;2a0b
	bit 1,(iy+009h)		;2a0d
	jp z,l2a17h		;2a11
	ld a,(wMasterVolume_F805)		;2a14
l2a17h:
	and a			;2a17
	jr z,l2a2fh		;2a18
	inc l			;2a1a
	jr z,l2a2fh		;2a1b
	ex af,af'			;2a1d
	dec l			;2a1e
	ld a,l			;2a1f
	jr z,l2a2fh		;2a20
	ld h,014h		;2a22
	ld a,(hl)			;2a24
	ex af,af'			;2a25
	ld l,a			;2a26
	ex af,af'			;2a27
	add a,(hl)			;2a28
	ld l,a			;2a29
	sbc a,a			;2a2a
	jr nc,l2a2fh		;2a2b
	inc h			;2a2d
	ld a,(hl)			;2a2e
l2a2fh:
	ld hl,0f8c7h		;2a2f
	cp (hl)			;2a32
	ld (hl),a			;2a33
	jp z,l2a40h		;2a34
	ld l,a			;2a37
	ld h,018h		;2a38
	ld e,(hl)			;2a3a
	ld d,00ah		;2a3b
	ld c,004h		;2a3d
	rst 8			;2a3f
l2a40h:
	ld c,001h		;2a40
	bit 1,(iy+009h)		;2a42
	jr z,l2a4ch		;2a46
	ld a,(wTempoStep_F809)		;2a48
	ld c,a			;2a4b
l2a4ch:
	ld hl,0f8bdh		;2a4c
	ld a,(hl)			;2a4f
	add a,c			;2a50
	sbc a,000h		;2a51
	ld (hl),a			;2a53
	ld hl,0f8bch		;2a54
	ld a,(hl)			;2a57
	and a			;2a58
	jp z,l2a6dh		;2a59
	sub c			;2a5c
	ld (hl),a			;2a5d
	jr nz,l2a82h		;2a5e
	bit 4,(iy+009h)		;2a60
	jr z,l2a7ch		;2a64
	set 4,(ix+003h)		;2a66
	jp l2a82h		;2a6a
l2a6dh:
	bit 4,(iy+009h)		;2a6d
	jr nz,l2a82h		;2a71
	ld hl,0f8bah		;2a73
	bit 4,(hl)		;2a76
	jr z,l2a82h		;2a78
	res 4,(hl)		;2a7a
l2a7ch:
	ld hl,0f8cch		;2a7c
	call Env_Release_2219		;2a7f
l2a82h:
	ld hl,wSSG_Mixer_F903		;2a82
	ld e,(hl)			;2a85
	inc hl			;2a86
	ld a,(hl)			;2a87
	ld (hl),e			;2a88
	inc hl			;2a89
	cp e			;2a8a
	jp z,l2a93h		;2a8b
	ld d,006h		;2a8e
	ld c,004h		;2a90
	rst 8			;2a92
l2a93h:
	ld e,(hl)			;2a93
	inc hl			;2a94
	ld a,(hl)			;2a95
	ld (hl),e			;2a96
	cp e			;2a97
	ret z			;2a98
	ld a,e			;2a99
	cpl			;2a9a
	and 007h		;2a9b
	add a,a			;2a9d
	add a,a			;2a9e
	add a,a			;2a9f
	or e			;2aa0
	ld e,a			;2aa1
	ld d,007h		;2aa2
	ld c,004h		;2aa4
	rst 8			;2aa6
	ret			;2aa7
l2aa8h:
	exx			;2aa8
	inc hl			;2aa9
	ld a,(hl)			;2aaa
	ld (ix+011h),a		;2aab
	add a,a			;2aae
	sbc a,a			;2aaf
	ld (ix+012h),a		;2ab0
	inc hl			;2ab3
	ld a,(hl)			;2ab4
	ld (ix+013h),a		;2ab5
	sub 001h		;2ab8
	sbc a,a			;2aba
	xor 038h		;2abb
	exx			;2abd
	ld b,a			;2abe
	ld hl,wSSG_EnvShape_F905		;2abf
	xor (hl)			;2ac2
	or (ix+014h)		;2ac3
	xor b			;2ac6
	ld (hl),a			;2ac7
	exx			;2ac8
	push ix		;2ac9
	inc hl			;2acb
	ld de,00015h		;2acc  constant
	add ix,de		;2acf
	call Seq_LoadOpPtrs_219B		;2ad1
	push iy		;2ad4
	ex de,hl			;2ad6
	defb 0fdh,06bh	;ld iyl,e		;2ad7
	defb 0fdh,062h	;ld iyh,d		;2ad9
	ld de,0000eh		;2adb  constant
	add ix,de		;2ade
	call Lfo_Load_22A8		;2ae0
	ld de,00006h		;2ae3  constant
	add iy,de		;2ae6
	ld de,0000eh		;2ae8  constant
	add ix,de		;2aeb
	call Lfo_Load_22A8		;2aed
	ld de,00006h		;2af0  constant
	add iy,de		;2af3
	ld de,0000eh		;2af5  constant
	add ix,de		;2af8
	call Pitch_Load_242E		;2afa
	pop iy		;2afd
	pop ix		;2aff
	ret			;2b01
l2b02h:
	ld l,(ix+008h)		;2b02
	ld h,011h		;2b05
	ld c,(hl)			;2b07
	set 7,l		;2b08
	ld b,(hl)			;2b0a
	ld (ix+009h),c		;2b0b
	ld (ix+00ah),b		;2b0e
	ret			;2b11
l2b12h:
	defb 0ddh,05dh	;ld e,ixl		;2b12
	defb 0ddh,054h	;ld d,ixh		;2b14
	ld hl,00015h		;2b16  constant
	add hl,de			;2b19
	call Env_Start_2208		;2b1a
	defb 0ddh,05dh	;ld e,ixl		;2b1d
	defb 0ddh,054h	;ld d,ixh		;2b1f
	ld hl,00023h		;2b21  constant
	add hl,de			;2b24
	call Env_Init_22F4		;2b25
	defb 0ddh,05dh	;ld e,ixl		;2b28
	defb 0ddh,054h	;ld d,ixh		;2b2a
	ld hl,00031h		;2b2c  constant
	add hl,de			;2b2f
	call Env_Init_22F4		;2b30
	push ix		;2b33
	ld de,0003fh		;2b35  constant
	add ix,de		;2b38
	ld c,(iy+00eh)		;2b3a
	call Pitch_Update_2461		;2b3d
	pop ix		;2b40
	ret			;2b42
