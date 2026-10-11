; command_tables_2076.s - M-ROM fixed_0000 [0x2076..0x20d3)
; Carved from mrom/asm/raw/fixed_0000.s by mrom/tools/split_region.py.
; Assembled through mrom/asm/fixed_0000.s; must stay byte-identical.

CmdJumpTable_2076:
	defw Cmd_Nop_20D3		;2076
	defw Cmd_Nop_20D3		;2078
	defw Cmd_StopMusic_2101		;207a
	defw Cmd_Nop_20D3		;207c
	defw Cmd_StopAll_215D		;207e
	defw Cmd_StopAll2_2163		;2080
	defw Cmd_StopSeq_2166		;2082
	defw Cmd_Nop_20D3		;2084
	defw Cmd_Nop_20D3		;2086
	defw Cmd_Nop_20D3		;2088
	defw Cmd_Nop_20D3		;208a
	defw Cmd_Nop_20D3		;208c
	defw Cmd_SetFlag_210F		;208e
	defw Cmd_Nop_20D3		;2090
	defw Cmd_Nop_20D3		;2092
	defw Cmd_StopSfx_2156		;2094
Cmd_DispatchHi_2096:
	ld (wSfxId_F81B),a		;2096
	sub 020h		;2099
	ld (wSoundId_F815),a		;209b
	ld a,(wCmdType_F81C)		;209e
	cp 010h		;20a1
	jp nc,Cmd_PlaySfx_2115		;20a3
	add a,a			;20a6
	ld e,a			;20a7
	ld d,000h		;20a8
	ld hl,CmdHiJumpTable_20B3		;20aa
	add hl,de			;20ad
	ld a,(hl)			;20ae
	inc hl			;20af
	ld h,(hl)			;20b0
	ld l,a			;20b1
	jp (hl)			;20b2
CmdHiJumpTable_20B3:
	defw Cmd_PlayMusic_20D4		;20b3
	defw Cmd_Nop_20D3		;20b5
	defw Cmd_Nop_20D3		;20b7
	defw Cmd_Nop_20D3		;20b9
	defw Cmd_Nop_20D3		;20bb
	defw Cmd_Nop_20D3		;20bd
	defw Cmd_Nop_20D3		;20bf
	defw Cmd_SetParam_2169		;20c1
	defw Cmd_SetTempo_2170		;20c3
	defw Cmd_SetFade_217E		;20c5
	defw Cmd_FadeOutOn_2185		;20c7
	defw Cmd_FadeOutOff_2190		;20c9
	defw Cmd_Nop_20D3		;20cb
	defw Cmd_Nop_20D3		;20cd
	defw Cmd_Nop_20D3		;20cf
	defw Cmd_Nop_20D3		;20d1
