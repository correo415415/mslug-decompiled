; command_handlers_20d3.s - M-ROM fixed_0000 [0x20d3..0x219b)
; Carved from mrom/asm/raw/fixed_0000.s by mrom/tools/split_region.py.
; Assembled through mrom/asm/fixed_0000.s; must stay byte-identical.

Cmd_Nop_20D3:
	ret			;20d3
Cmd_PlayMusic_20D4:
	ld a,(wMusicId_F818)		;20d4
	call Seq_SetVolumeLevel_4F88		;20d7
	xor a			;20da
	ld (wMusicId_F818),a		;20db
	ld hl,BankPtrTable_0080+4		;20de
	call Bank_ResolvePtr24_1FA6		;20e1
	ex de,hl			;20e4
	ld a,(wSoundId_F815)		;20e5
	ld l,a			;20e8
	ld h,000h		;20e9
	add hl,hl			;20eb
	add hl,hl			;20ec
	add hl,de			;20ed
	ld a,(hl)			;20ee
	inc hl			;20ef
	or (hl)			;20f0
	inc hl			;20f1
	or (hl)			;20f2
	ret z			;20f3
	dec hl			;20f4
	dec hl			;20f5
	call Bank_ResolvePtr24_1FA6		;20f6
	ex de,hl			;20f9
	defb 0ddh,06bh	;ld ixl,e		;20fa
	defb 0ddh,062h	;ld ixh,d		;20fc
	jp Seq_StartMusic_4FA3		;20fe
Cmd_StopMusic_2101:
	xor a			;2101
	ld (wMusicId_F818),a		;2102
	call Seq_SetVolumeLevel_4F88		;2105
	ld ix,CmdStruct_24DE		;2108
	jp Seq_StartMusic_4FA3		;210c
Cmd_SetFlag_210F:
	ld a,0ffh		;210f
	ld (wSfxPending_F816),a		;2111
	ret			;2114
Cmd_PlaySfx_2115:
	ld a,(wSfxPending_F816)		;2115
	and a			;2118
	jr z,Cmd_PlaySfx_L2126		;2119
	xor a			;211b
	ld (wSfxPending_F816),a		;211c
	ld bc,(wSfxId_F81B)		;211f
	jp Seq_StopTrackById_5210		;2123
Cmd_PlaySfx_L2126:
	ld a,(wCmdType_F81C)		;2126
	sub 010h		;2129
	and 00fh		;212b
	add a,a			;212d
	ld e,a			;212e
	ld d,000h		;212f
	ld hl,BankPtrTable_0080+8		;2131
	add hl,de			;2134
	add hl,de			;2135
	call Bank_ResolvePtr24_1FA6		;2136
	ex de,hl			;2139
	ld a,(wSoundId_F815)		;213a
	ld l,a			;213d
	ld h,000h		;213e
	add hl,hl			;2140
	add hl,hl			;2141
	add hl,de			;2142
	ld a,(hl)			;2143
	inc hl			;2144
	or (hl)			;2145
	inc hl			;2146
	or (hl)			;2147
	ret z			;2148
	dec hl			;2149
	dec hl			;214a
	call Bank_ResolvePtr24_1FA6		;214b
	ex de,hl			;214e
	defb 0ddh,06bh	;ld ixl,e		;214f
	defb 0ddh,062h	;ld ixh,d		;2151
	jp Seq_StartSfx_507B		;2153
Cmd_StopSfx_2156:
	ld ix,CmdStruct_24F7		;2156
	jp Seq_StartSfx_507B		;215a
Cmd_StopAll_215D:
	call Seq_Init_4C84		;215d
	jp Seq_StopAllTracks_504E		;2160
Cmd_StopAll2_2163:
	jp Seq_Init_4C84		;2163
Cmd_StopSeq_2166:
	jp Seq_StopAllTracks_504E		;2166
Cmd_SetParam_2169:
	ld a,(wSoundId_F815)		;2169
	ld (wSoundParam_F817),a		;216c
	ret			;216f
Cmd_SetTempo_2170:
	ld a,(wSoundId_F815)		;2170
	add a,020h		;2173
	srl a		;2175
	ld b,a			;2177
	ld a,(wSoundParam_F817)		;2178
	jp Seq_TempoCurve_4F5B		;217b
Cmd_SetFade_217E:
	ld a,(wSoundId_F815)		;217e
	ld (wMusicId_F818),a		;2181
	ret			;2184
Cmd_FadeOutOn_2185:
	ld a,001h		;2185
	ld (wFadeFlag_F808),a		;2187
	ld a,(wSoundId_F815)		;218a
	jp Seq_SetVolumeLevel_4F88		;218d
Cmd_FadeOutOff_2190:
	ld a,000h		;2190
	ld (wFadeFlag_F808),a		;2192
	ld a,(wSoundId_F815)		;2195
	jp Seq_SetVolumeLevel_4F88		;2198
