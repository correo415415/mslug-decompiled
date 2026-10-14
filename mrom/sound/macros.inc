/* macros.inc - sound-data macros for the Metal Slug 1 M-ROM (GNU as, binutils-z80)
 *
 * mrom/sound/ describes file 0x057A1-0x1FFFF of 201-m1.bin: the data the Z80
 * driver reads through its bank window.  Labels are *linear file addresses*
 * relative to SD_Base (= file address 0), so 24-bit pointers and the relative
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
        .byte ((\target) - SD_Base) & 0xff
        .byte (((\target) - SD_Base) >> 8) & 0xff
        .byte (((\target) - SD_Base) >> 16) & 0xff
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

/* ---- instruments (InstrPtrTable_57A1 targets; mrom/docs/FORMATS.md §6) ----
 * Byte 0 = channel type, dispatched through ChTypeJumpTable0_4AA0 when a note
 * is keyed with a new instrument.  Sub-blocks shared by several types:
 *   I_ENV   5 B  -> Seq_LoadOpPtrs_219B: attack, decay, sustain (stored cpl),
 *                  release, extra rate; each rate !=0 indexes the 1Bxx/1Cxx
 *                  rate tables (0 = instant)
 *   I_LFO   6 B  -> Lfo_Load_22A8: +0 bit7 on, bits0-6 delay; +1 speed;
 *                  +2 depth; +3/+5 signed shapes; +4 index into 14xx/15xx/16xx
 *   I_PITCH 4 B  -> Pitch_Load_242E: +0 bit0 enable, bits4-7 mode; +1 rate;
 *                  +2 fine; +3 note offset
 * The first I_LFO drives volume, the second pitch (vibrato). */
        .macro I_ENV a, d, s, r, x
        .byte \a, \d, \s, \r, \x
        .endm
        .macro I_LFO a, b, c, d, e, f
        .byte \a, \b, \c, \d, \e, \f
        .endm
        .macro I_PITCH a, b, c, d
        .byte \a, \b, \c, \d
        .endm

/* Type 0, SSG (24 B, loader l2aa8h): +1 noise period / tone flags (+11/+12 of
 * the channel), +2 mixer: 0 = tone only, else tone+noise (FB05 bits);
 * then I_ENV, I_LFO volume, I_LFO pitch, I_PITCH. */
        .macro INSTR_SSG noise, mix
        .byte 0, \noise, \mix
        .endm

/* Type 1, FM (50 B, loader l3568h): +1 bits4-7 slot mask (key-on reg 28h),
 * +2 FB/ALG (reg B0h, bit 7 forced), +3 LFO enable/freq (reg 22h if bit 3),
 * +4 AMS/PMS (reg B4h), +5 transpose; then 4 FM_OP, I_LFO x2, I_PITCH. */
        .macro INSTR_FM slots, fbalg, lfo, amspms, transp
        .byte 1, \slots, \fbalg, \lfo, \amspms, \transp
        .endm
/* FM operator: DT/MUL (30h), TL (40h), KS/AR (50h), AM/DR (60h), SR (70h),
 * SL/RR (80h), SSG-EG (90h)  - copied 7 bytes to the channel, TL patched later */
        .macro FM_OP dtmul, tl, ksar, amdr, sr, slrr, ssgeg
        .byte \dtmul, \tl, \ksar, \amdr, \sr, \slrr, \ssgeg
        .endm

/* Type 2, ADPCM-A (loader l3f41h / key-on l3f60h): I_ENV, then PCMA_MAP:
 * 6 unused bytes, PTR24 to a PcmaSampleTab, base note; then one byte per
 * note (note - base) = index into the sample table. */
        .macro INSTR_PCMA
        .byte 2
        .endm
        .macro PCMA_MAP u0, u1, u2, u3, u4, u5, tab, base
        .byte \u0, \u1, \u2, \u3, \u4, \u5
        PTR24 \tab
        .byte \base
        .endm
/* ADPCM-A sample (5 B, copied to channel +15): volume/pan byte, start and
 * end address in V-ROM 256-byte units (regs 10h-2Dh). */
        .macro PCMA_SAMPLE vol, start, end
        .byte \vol
        .word \start, \end
        .endm

/* Type 3, ADPCM-B (loader l427fh / key-on l42efh): I_ENV, I_LFO x2, I_PITCH,
 * then PCMB_KEY ranges (highest note, root note, sample) ended by PCMB_END. */
        .macro INSTR_PCMB
        .byte 3
        .endm
        .macro PCMB_KEY top, root, smp
        .byte \top, \root
        PTR24 \smp                      /* 3 bytes + 0 pad */
        .endm
        .macro PCMB_END
        .byte 0xff
        .endm
/* ADPCM-B sample (12 B, read at l432bh): delta-N base, pan/mode, loop count
 * (FF = none), then 4 V-ROM ranges (start, attack length, loop length, tail
 * length, in 256-byte units). */
        .macro PCMB_SAMPLE deltan, pan, loops, start, len1, len2, len3
        .word \deltan
        .byte \pan, \loops
        .word \start, \len1, \len2, \len3
        .endm
