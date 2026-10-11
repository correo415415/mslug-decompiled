; neogeo_z80.inc - Neo Geo Z80 side hardware equates (ports + bank windows)
; Shared by the M-ROM sources.  z80asm syntax (`name: equ value`).

; ---- I/O ports ---------------------------------------------------------------
PORT_CMD_IN:        equ 000h   ; in : command latch written by the 68000 (REG_SOUND)
PORT_YM_ADDR0:      equ 004h   ; out: YM2610 address, part 1 (regs 000-0FF) / in: status 0
PORT_YM_DATA0:      equ 005h   ; out: YM2610 data, part 1
PORT_YM_ADDR1:      equ 006h   ; out: YM2610 address, part 2 (regs 100-1FF) / in: status 1
PORT_YM_DATA1:      equ 007h   ; out: YM2610 data, part 2
PORT_BANK_8000:     equ 00bh   ; in with B = bank: map 16 KiB bank at 8000-BFFF
PORT_BANK_C000:     equ 00ah   ; in with B = bank: map  8 KiB bank at C000-DFFF
PORT_BANK_E000:     equ 009h   ; in with B = bank: map  4 KiB bank at E000-EFFF
PORT_BANK_F000:     equ 008h   ; in with B = bank: map  2 KiB bank at F000-F7FF
PORT_NMI_ENABLE:    equ 008h   ; out: enable NMI (any value)
PORT_REPLY_OUT:     equ 00ch   ; out: reply byte readable by the 68000 (REG_SOUND)
PORT_NMI_DISABLE:   equ 018h   ; out: disable NMI (any value)

; ---- memory windows ----------------------------------------------------------
WIN_8000:           equ 08000h ; 16 KiB window, driver uses file banks 2..7 here
WIN_C000:           equ 0c000h
WIN_E000:           equ 0e000h
WIN_F000:           equ 0f000h
WORK_RAM:           equ 0f800h ; 2 KiB work RAM F800-FFFF

; ---- YM2610 registers used by the driver (part 1 unless noted) --------------
YM_SSG_FINE_A:      equ 000h
YM_SSG_MIXER:       equ 007h
YM_SSG_VOL_A:       equ 008h
YM_SSG_VOL_B:       equ 009h
YM_SSG_VOL_C:       equ 00ah
YM_SSG_ENV_SHAPE:   equ 00dh
YM_ADPCMB_CTRL:     equ 010h
YM_ADPCMB_LR:       equ 011h
YM_ADPCMB_START_L:  equ 012h
YM_ADPCMB_START_H:  equ 013h
YM_ADPCMB_END_L:    equ 014h
YM_ADPCMB_END_H:    equ 015h
YM_ADPCMB_DELTA_L:  equ 019h
YM_ADPCMB_DELTA_H:  equ 01ah
YM_ADPCMB_VOL:      equ 01bh
YM_IRQ_FLAGS:       equ 01ch   ; flag control / ADPCM end flags
YM_TIMER_A_H:       equ 024h
YM_TIMER_A_L:       equ 025h
YM_TIMER_B:         equ 026h
YM_TIMER_CTRL:      equ 027h
YM_KEY_ON:          equ 028h
YM_FM_DT_MUL:       equ 030h   ; +op*4 +ch
YM_FM_TL:           equ 040h
YM_FM_KS_AR:        equ 050h
YM_FM_AM_DR:        equ 060h
YM_FM_SR:           equ 070h
YM_FM_SL_RR:        equ 080h
YM_FM_SSG_EG:       equ 090h
YM_FM_FNUM_L:       equ 0a0h
YM_FM_FNUM_H:       equ 0a4h
YM_FM_FB_ALGO:      equ 0b0h
YM_FM_LR_AMS_PMS:   equ 0b4h
YM_ADPCMA_DUMP:     equ 000h   ; part 2: key on/off (bit 7 = dump)
YM_ADPCMA_TL:       equ 001h   ; part 2: total level
YM_ADPCMA_CH_VOL:   equ 008h   ; part 2: +ch, pan + instrument level
YM_ADPCMA_START_L:  equ 010h   ; part 2: +ch
YM_ADPCMA_START_H:  equ 018h   ; part 2: +ch
YM_ADPCMA_END_L:    equ 020h   ; part 2: +ch
YM_ADPCMA_END_H:    equ 028h   ; part 2: +ch
