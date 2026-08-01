| ============================================================================
|  Metal Slug 1 - asm/probe_two_attempts_ccr_05e5a8.s
|  ----------------------------------------------------------------------------
|  Wave Z batch 2 - #1
|
|  ProbeTwoAttemptsCcr_05E5A8  @ $05E5A8  (56 bytes)
|
|  Realiza DOS intentos consecutivos de "probe + confirm" con d0=0 y d0=1
|  como parametros. Cada intento consiste en:
|    - jsr $5E3A2(pc)                    (probe primario)
|    - si bcc (probe fallo): fallback path (segundo intento o exit)
|    - jsr $5E618(pc)                    (confirm secundario)
|    - si bcs (confirm fallo): fallback path
|    - andi.b #$EE, ccr; rts             (exito: publica exito por CCR)
|
|  Si ambos intentos fallan, cae en $5E5DA (`ori #$11, ccr; rts`) = fallo
|  publicado por CCR. Absorbe SetXN_05e5da (Wave N-ccr_helpers). 16 FP.
|
|  Firma C conceptual:
|
|      /* Intenta dos veces (con d0=0 y d0=1) la secuencia probe-confirm.
|       * Retorna con CCR limpio si algun intento tuvo exito, o con X+C
|       * seteados si ambos fallaron. */
|      /* void */ int ProbeTwoAttemptsCcr(void);
|
|  Toolchain:  m68k-linux-gnu-gcc -m68000 -Wa,--register-prefix-optional
|  ============================================================================

        .text
        .globl  ProbeTwoAttemptsCcr_05E5A8
        .type   ProbeTwoAttemptsCcr_05E5A8, @function
        .section .text.ProbeTwoAttemptsCcr_05E5A8, "ax", @progbits

ProbeTwoAttemptsCcr_05E5A8:
        clr.w   d0                             | +00  d0 = 0 (primer intento)
        jsr     .Lprobe(pc)                    | +02  Sub_0005E3A2 (probe)
        bcc.w   .Lattempt2                     | +06  if (!C) fallo primer probe -> intento 2
        jsr     .Lconfirm(pc)                  | +0a  Sub_0005E618 (confirm)
        bcs.w   .Lattempt2                     | +0e  if (C) fallo confirm -> intento 2
        andi.b  #0xee, ccr                     | +12  CCR &= 0xEE (exito publico)
        rts                                    | +16
                                              |
.Lattempt2:
        move.w  #0x1, d0                       | +18  d0 = 1 (segundo intento)
        jsr     .Lprobe(pc)                    | +1c  Sub_0005E3A2 (probe)
        bcc.w   .Lfail                         | +20  if (!C) fallo -> exit fallido
        jsr     .Lconfirm(pc)                  | +24  Sub_0005E618 (confirm)
        bcs.w   .Lfail                         | +28  if (C) fallo -> exit fallido
        andi.b  #0xee, ccr                     | +2c  CCR &= 0xEE (exito publico)
        rts                                    | +30
                                              |
.Lfail:
        ori.b   #0x11, ccr                     | +32  CCR |= 0x11 (fallo publico)
        rts                                    | +36

        .equ    .Lprobe,   Sub_0005E3A2
        .equ    .Lconfirm, Sub_0005E618

        .size   ProbeTwoAttemptsCcr_05E5A8, .-ProbeTwoAttemptsCcr_05E5A8
