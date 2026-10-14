; instruments_57a1.s - M-ROM fixed_0000 [0x57a1..0x8000)
; Carved from mrom/asm/raw/fixed_0000.s by mrom/tools/split_region.py.
; Assembled through mrom/asm/fixed_0000.s; must stay byte-identical.

InstrPtrTable_57A1:
	incbin "instruments_fixed.bin"		; 0x285f bytes, 57a1-8000
