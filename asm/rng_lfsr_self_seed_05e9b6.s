| ============================================================================
|  Metal Slug 1 - asm/rng_lfsr_self_seed_05e9b6.s
|  ----------------------------------------------------------------------------
|  Wave CC batch 1 - #13
|
|  RNG_LFSRStep_SelfSeed_05E9B6  @ $05E9B6  (46 B, 2 callers)
|
|  Variante "self-seeded" del LFSR global compartida con RNG_LFSRStep_05E9E4
|  (Wave Z1). Buffer LFSR en $10E230..$10E26F (32 words), puntero circular
|  en $10E270 (mascara $3E).
|
|  Diferencia con $05E9E4: no toma seed en d0, solo avanza el LFSR. Deja
|  el flag N del CCR utilizable por el caller (idioma retorno-por-CCR).
|
|  Callers: EntityState_PublishByProbeN_05717A (BB2#3),
|           EntityState_PublishByProbeN_ClearSub75_05719C (BB2#4).
|
|  Toolchain:  m68k-linux-gnu-gcc -m68000 -Wa,--register-prefix-optional
|  ============================================================================

        .text
        .globl  RNG_LFSRStep_SelfSeed_05E9B6
        .type   RNG_LFSRStep_SelfSeed_05E9B6, @function
        .section .text.RNG_LFSRStep_SelfSeed_05E9B6, "ax", @progbits

RNG_LFSRStep_SelfSeed_05E9B6:
        lea.l   0x10e230.l, a4
        move.w  0x10e270.l, d7
        addi.w  #0x2, d7
        move.w  #0x3e, d6
        and.w   d6, d7
        move.w  d7, 0x10e270.l
        move.w  (a4, d7.w), d0
        subi.w  #0x15, d7
        and.w   d6, d7
        adda.w  d7, a4
        eor.w   d0, (a4)
        move.w  (a4), d0
        rts
        .size   RNG_LFSRStep_SelfSeed_05E9B6, .-RNG_LFSRStep_SelfSeed_05E9B6
