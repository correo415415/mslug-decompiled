; vectors_0000.s - M-ROM fixed_0000 [0x0000..0x0080)
; Carved from mrom/asm/raw/fixed_0000.s by mrom/tools/split_region.py.
; Assembled through mrom/asm/fixed_0000.s; must stay byte-identical.

Reset_0000:
	di			;0000
	out (018h),a		;0001
	jp Main_Init_1E00		;0003
	defb 000h,000h		;0006
YM_Write_RST8_0008:
	in a,(004h)		;0008
	and a			;000a
	jp m,YM_Write_RST8_0008		;000b
	ld a,e			;000e
	ld e,c			;000f
	ld (wYmPortLatch_F80B),de		;0010
	out (c),d		;0014
	nop			;0016
	nop			;0017
	ld e,a			;0018
	inc c			;0019
	out (c),e		;001a
	dec c			;001c
	ret			;001d
	defb 000h,000h		;001e
	defb 000h,000h,000h,000h,000h,000h,000h,000h,000h,000h,000h,000h,000h,000h,000h,000h		;0020
Ch_ClearActive_RST30_0030:
	xor a			;0030
	ld (ix+000h),a		;0031
	ld (ix+00dh),a		;0034
	ret			;0037
IRQ_Vector_0038:
	jp IRQ_Timer_1E7A		;0038
	defb 000h,000h,000h,000h,000h		;003b
	defb 000h,000h,000h,000h,000h,000h,000h,000h,000h,000h,000h,000h,000h,000h,000h,000h		;0040
	defb 000h,000h,000h,000h,000h,000h,000h,000h,000h,000h,000h,000h,000h,000h,000h,000h		;0050
	defb 000h,000h,000h,000h,000h,000h		;0060
NMI_Vector_0066:
	jp NMI_Command_1ED6		;0066
	defb 000h,000h,000h,000h,000h,000h,000h		;0069
	defb 000h,000h,000h,000h,000h,000h,000h,000h,000h,000h,000h,000h,000h,000h,000h,000h		;0070
