/* macros.inc - sound-data macros for the Metal Slug 1 M-ROM (GNU as, binutils-z80)
 *
 * mrom/sound/ describes file 0x08000-0x1FFFF of 201-m1.bin: the data the Z80
 * driver reads through its bank window.  Labels are *linear file addresses*
 * relative to SD_Base (= file 0x08000), so 24-bit pointers and the relative
 * offsets of the sequence opcodes are written symbolically.
 * Field meanings: mrom/docs/FORMATS.md.
 */

/* ---- pointers ------------------------------------------------------------ */

/* 24-bit linear file pointer padded to 4 bytes (lo, hi, bank-ish, 0).
 * Bank_ResolvePtr24_1FA6: bank = addr >> 14, window = (addr & 0x3FFF) | 0x8000.
 * PTR24 0 = empty slot. */
        .macro PTR24 target
        .ifc \target,0
        .long 0
        .else
        .byte ((\target) - SD_Base + 0x8000) & 0xff
        .byte (((\target) - SD_Base + 0x8000) >> 8) & 0xff
        .byte (((\target) - SD_Base + 0x8000) >> 16) & 0xff
        .byte 0
        .endif
        .endm

/* ---- headers ------------------------------------------------------------- */

/* Music header (Seq_StartMusic_4FA3): first byte indexes NoteMapTable_4B84 to
 * give the initial tempo (FDE7/FDE8/FDEF); then up to 14 TRACKs. */
        .macro MUSIC_HDR tempo
        .byte \tempo
        .endm

/* SFX header (Seq_StartSfx_507B): four 16-bit sound ids (68k command | type<<8)
 * whose tracks are stopped first, then the priority compared by
 * Seq_FindFreeTrack_50DB, then up to 8 TRACKs. */
        .macro SFX_HDR s0, s1, s2, s3, prio
        .word \s0, \s1, \s2, \s3
        .byte \prio
        .endm

/* Track entry: PTR24 to the sequence (3 bytes + pad) + channel byte
 * (stored in track +02; for SFX it selects / prioritises the channel). */
        .macro TRACK seq, chan
        PTR24 \seq
        .byte \chan
        .endm

/* End of a header's track list (null pointer, 4 bytes). */
        .macro TRACKS_END
        .long 0
        .endm

/* ---- sequence byte-code (Seq_TrackFetch_5362) ----------------------------
 * The interpreter keeps a "current note" per track (+17) that relative note
 * events add to.  It is reset to 0x40 when a track starts and by jumps,
 * calls, returns, loop starts and taken loop ends; the macros mirror it in
 * the assembler symbol _sd_note so notes are written as absolute values.
 */
        _sd_note = 0x40

        .macro S_TRACK                  /* sequence entry point */
        _sd_note = 0x40
        .endm

        .macro _S_REL16 target          /* 16-bit offset from the end of the field */
        .word ((\target) - . - 2) & 0xffff
        .endm

        .macro _S_MASK len, vol, gate   /* bit set = byte omitted (keep current) */
        _sd_m = 0
        .ifb \len
        _sd_m = _sd_m | 1
        .endif
        .ifb \vol
        _sd_m = _sd_m | 2
        .endif
        .ifb \gate
        _sd_m = _sd_m | 4
        .endif
        .endm

        .macro _S_OPT len, vol, gate
        .ifnb \len
        .byte \len
        .endif
        .ifnb \vol
        .byte \vol
        .endif
        .ifnb \gate
        .byte \gate
        .endif
        .endm

/* Note event, relative form (op 00..B7): op = ((delta+11) << 3) | mask,
 * delta = note - current note in -11..+11.
 * N note [, len [, vol [, gate]]]  (empty argument = keep current value) */
        .macro N note, len, vol, gate
        _sd_d = ((((\note) - _sd_note) + 128) & 255) - 128
        .if (_sd_d < -11) | (_sd_d > 11)
        .error "N: note out of relative range, use NX"
        .endif
        _S_MASK \len, \vol, \gate
        .byte ((_sd_d + 11) << 3) | _sd_m
        _S_OPT \len, \vol, \gate
        _sd_note = (\note) & 255
        .endm

/* Note event, explicit form (op B8..BF + absolute note byte). */
        .macro NX note, len, vol, gate
        _S_MASK \len, \vol, \gate
        .byte 0xb8 | _sd_m, \note
        _S_OPT \len, \vol, \gate
        _sd_note = (\note) & 255
        .endm

        .macro S_INSTR num              /* C0: instrument number (+0A) */
        .byte 0xc0, \num
        .endm
        .macro S_INSTRG num, grp        /* C1: instrument number + group (+0A/+0B) */
        .byte 0xc1, \num, \grp
        .endm
        .macro S_JUMP target            /* C2: goto */
        .byte 0xc2
        _S_REL16 \target
        _sd_note = 0x40
        .endm
        .macro S_KEYMODE v              /* C3: track flag +09 bit 0 (legato/re-key) (?) */
        .byte 0xc3, \v
        .endm
        .macro S_CALL target            /* C4: call subsequence (3-level stack at +1C) */
        .byte 0xc4
        _S_REL16 \target
        _sd_note = 0x40
        .endm
        .macro S_RET                    /* C5: return from subsequence */
        .byte 0xc5
        _sd_note = 0x40
        .endm
        .macro S_PMOD v                 /* C6: pitch-envelope depth (+0C) */
        .byte 0xc6, \v
        .endm
        .macro S_VMOD v                 /* C7: volume-envelope depth (+0D) */
        .byte 0xc7, \v
        .endm
        .macro S_PARAM v                /* C8: pitch-update parameter (+0E) (?) */
        .byte 0xc8, \v
        .endm
        .macro S_SUSTAIN v              /* C9: sustain flag (+09 bit 4) */
        .byte 0xc9, \v
        .endm
        .macro S_DETUNE w, wait         /* CA: detune word (x8 -> +10/+11) + wait */
        .byte 0xca
        .word \w
        .byte \wait
        .endm
        .macro S_WAIT n                 /* CB: rest n ticks */
        .byte 0xcb, \n
        .endm
        .macro S_PAN v                  /* CC: 0x40 = L+R, <0x40 = one side, >0x40 = other (+0F) */
        .byte 0xcc, \v
        .endm
        .macro S_TEMPO a, b             /* CD: tempo change (Seq_SetTempo_4F39) */
        .byte 0xcd, \a, \b
        .endm
        .macro S_TRKVOL v               /* CE: track attenuation (+12) */
        .byte 0xce, \v
        .endm
        .macro S_LOOPBEGIN              /* CF: push a loop counter */
        .byte 0xcf
        _sd_note = 0x40
        .endm
        .macro S_LOOPEND count, target  /* D0: repeat from target, count times */
        .byte 0xd0, \count
        _S_REL16 \target
        .endm
        .macro S_PORTA v                /* D1: portamento on (>=0x40) / off */
        .byte 0xd1, \v
        .endm
        .macro S_LFO v                  /* D2: LFO / vibrato preset (+13) */
        .byte 0xd2, \v
        .endm
        .macro S_D3 a, wait             /* D3..D6: per-channel-type key-on helpers (?) */
        .byte 0xd3, \a, \wait
        .endm
        .macro S_D4 a, wait
        .byte 0xd4, \a, \wait
        .endm
        .macro S_D5 a, wait
        .byte 0xd5, \a, \wait
        .endm
        .macro S_D6 a, wait
        .byte 0xd6, \a, \wait
        .endm
        .macro S_PITCHDN amt, wait      /* D7: pitch bend down */
        .byte 0xd7, \amt, \wait
        .endm
        .macro S_PITCHUP amt, wait      /* D8: pitch bend up */
        .byte 0xd8, \amt, \wait
        .endm
        .macro S_REPLY code             /* D9: reply (code&3F)*2+80 | toggle to the 68k */
        .byte 0xd9, \code
        .endm
