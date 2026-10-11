; vectors_0000.s - M-ROM fixed_0000 [0x0000..0x0080)
; Carved from mrom/asm/raw/fixed_0000.s by mrom/tools/split_region.py.
; Assembled through mrom/asm/fixed_0000.s; must stay byte-identical.

Reset_0000:
	di			;0000
l0001h:
	out (018h),a		;0001
	jp Main_Init_1E00		;0003
l0006h:
	defb 000h		;0006
l0007h:
	defb 000h		;0007
YM_Write_RST8_0008:
	in a,(004h)		;0008
l000ah:
	and a			;000a
	jp m,YM_Write_RST8_0008		;000b
l000eh:
	ld a,e			;000e
	ld e,c			;000f
	ld (wYmPortLatch_F80B),de		;0010
l0014h:
	out (c),d		;0014
	nop			;0016
	nop			;0017
	ld e,a			;0018
	inc c			;0019
	out (c),e		;001a
l001ch:
	dec c			;001c
	ret			;001d
	defb 000h		;001e
	defb 000h		;001f
	defb 000h		;0020
	defb 000h		;0021
l0022h:
	defb 000h		;0022
l0023h:
	defb 000h		;0023
	defb 000h		;0024
	defb 000h		;0025
	defb 000h		;0026
	defb 000h		;0027
	defb 000h		;0028
	defb 000h		;0029
	defb 000h		;002a
	defb 000h		;002b
	defb 000h		;002c
	defb 000h		;002d
	defb 000h		;002e
	defb 000h		;002f
Ch_ClearActive_RST30_0030:
	xor a			;0030
l0031h:
	ld (ix+000h),a		;0031
	ld (ix+00dh),a		;0034
	ret			;0037
IRQ_Vector_0038:
	jp IRQ_Timer_1E7A		;0038
	defb 000h		;003b
	defb 000h		;003c
	defb 000h		;003d
l003eh:
	defb 000h		;003e
l003fh:
	defb 000h		;003f
l0040h:
	defb 000h		;0040
	defb 000h		;0041
	defb 000h		;0042
	defb 000h		;0043
	defb 000h		;0044
	defb 000h		;0045
	defb 000h		;0046
	defb 000h		;0047
	defb 000h		;0048
	defb 000h		;0049
	defb 000h		;004a
	defb 000h		;004b
l004ch:
	defb 000h		;004c
l004dh:
	defb 000h		;004d
	defb 000h		;004e
	defb 000h		;004f
	defb 000h		;0050
	defb 000h		;0051
	defb 000h		;0052
	defb 000h		;0053
	defb 000h		;0054
	defb 000h		;0055
	defb 000h		;0056
	defb 000h		;0057
	defb 000h		;0058
	defb 000h		;0059
l005ah:
	defb 000h		;005a
	defb 000h		;005b
	defb 000h		;005c
	defb 000h		;005d
	defb 000h		;005e
	defb 000h		;005f
	defb 000h		;0060
	defb 000h		;0061
	defb 000h		;0062
	defb 000h		;0063
	defb 000h		;0064
	defb 000h		;0065
NMI_Vector_0066:
	jp NMI_Command_1ED6		;0066
	defb 000h		;0069
	defb 000h		;006a
	defb 000h		;006b
	defb 000h		;006c
	defb 000h		;006d
	defb 000h		;006e
	defb 000h		;006f
	defb 000h		;0070
	defb 000h		;0071
	defb 000h		;0072
	defb 000h		;0073
	defb 000h		;0074
	defb 000h		;0075
	defb 000h		;0076
	defb 000h		;0077
	defb 000h		;0078
	defb 000h		;0079
	defb 000h		;007a
	defb 000h		;007b
	defb 000h		;007c
	defb 000h		;007d
	defb 000h		;007e
	defb 000h		;007f
