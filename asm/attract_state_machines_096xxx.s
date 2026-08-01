| ============================================================================
|  Metal Slug 1 - asm/attract_state_machines_096xxx.s
|  ----------------------------------------------------------------------------
|  Wave GG batch 1 - cluster de 7 handlers "attract state machine" en $096xxx
|
|  Estos 7 handlers son los targets exactos de los `move.l #$967FE/$96840/
|  $96882/$968C4/$96906/$96948/$9698A, (a0)` publicados por
|  Dispatcher_ModeTable_001922 (EE#3) en $100260 - es decir, la
|  implementacion CONCRETA de cada uno de los 7 estados del subsistema
|  attract cuyo indice se guarda en $106ECE.
|
|  Todos siguen la misma macro-plantilla `ATTRACT_HANDLER n`:
|
|      move.b  #n, d0                       | +00  state index
|      move.b  d0, $21(a6)                   | +04  entity.flag_21 = n
|      jsr     $43568.l                      | +08  Sub_00043568 (setup pesado)
|      move.b  #$0, $10e39e.l                | +0e  clear global flag
|     [move.w  #$0, $70(a6)                  | +16  entity.timer_70 = 0  (n=0..5)
|      lea     $<next+2>(pc), a1             | +1c  ptr a "continuation"
|      move.l  a1, (a6)                      | +20  entity.handler = a1
|      jsr     $967c0(pc)                    | +22  Attract_Sub_setup_967C0
|      jsr     $436de.l                      | +26  Geom_Proj_Clamp (FF#2)
|     [jsr     $969c2(pc)                    | +2c  Attract_Sub_969C2  (n=0..5)
|      jsr     $96a80(pc)                    | +30  ThunkTarget_096a80
|      jsr     $96b24(pc)                    | +34  Attract_Sub_96B24
|      move.b  #$1, $10e39a.l                | +38  set trigger flag
|      rts                                   | +40
|
|  Handler #7 (n=7, $09698A) tiene una variante compacta (56 B) que OMITE
|  el `move.w #$0, $70(a6)` y el `jsr $969c2(pc)`. Los otros 6 (n=0..5, 66 B)
|  llevan la plantilla completa.
|
|  Firma C conceptual:
|
|      void Attract_State<N>_Handler(void);    // N in [0..6]
|
|  Cross-links con oleadas anteriores:
|    - Los 7 handlers son los targets de Dispatcher_ModeTable_001922 (EE#3)
|      en su tabla de estados 0..6. State 7-A del EE#3 son variantes.
|    - jsr $436de.l es Geom_Proj_Clamp_0436DE (Wave FF#2) - primera vez
|      que documentamos callers CONCRETOS de ese helper geometrico.
|    - jsr $2352 (aparece via $43568 -> ...) es InputGuardCall219c (Wave A#4).
|
|  Notas forenses (por que NO es rederivable por GCC 1:1):
|    1. 7 handlers estructuralmente identicos indican expansion de macro
|       ASM `ATTRACT_HANDLER n[, no_timer_reset, no_969c2_call]`. GCC
|       nunca emite 7 copias literales - factorizaria en subrutina + tabla.
|    2. El puntero `lea $<self+2>(pc), a1; move.l a1, (a6)` re-instala
|       EL PROPIO HANDLER como continuacion. Es un "self-loop handler",
|       patron hand-coded que requiere conocer explicitamente el offset
|       del propio codigo.
|    3. El helper $967c0 llamado en 7 handlers consecutivos con la MISMA
|       secuencia (jsr $967c0; jsr $436de; ...) es candidato a factoring
|       en GCC, pero aqui aparece 7 veces literal.
|
|  Toolchain:  m68k-linux-gnu-gcc -m68000 -Wa,--register-prefix-optional
|  ============================================================================

        .text

| ============================================================================
| Handler #0 (state=0) @ $0967FE  (66 B) - full template
| ============================================================================
        .globl  Attract_State0_Handler_0967FE
        .type   Attract_State0_Handler_0967FE, @function
        .section .text.Attract_State0_Handler_0967FE, "ax", @progbits

Attract_State0_Handler_0967FE:
        move.b  #0x0, d0                       | +00  state index
        move.b  d0, 0x21(a6)                   | +04  entity.flag_21 = 0
        jsr     0x43568.l                      | +08  Sub_00043568
        move.b  #0x0, 0x10e39e.l               | +0e  clear global flag
        move.w  #0x0, 0x70(a6)                 | +16  entity.timer_70 = 0
        lea.l   .Lh0_cont(pc), a1              | +1c  ptr a continuation
        move.l  a1, (a6)                       | +20  entity.handler = a1
.Lh0_cont:                                     | $096820
        jsr     Attract_Sub_setup_967C0(pc)    | +22  jsr $967c0(pc)
        jsr     0x436de.l                      | +26  Geom_Proj_Clamp
        jsr     Attract_Sub_969C2(pc)          | +2c  jsr $969c2(pc)
        jsr     ThunkTarget_096a80(pc)         | +30  jsr $96a80(pc)
        jsr     Attract_Sub_96B24(pc)          | +34  jsr $96b24(pc)
        move.b  #0x1, 0x10e39a.l               | +38  set trigger flag
        rts                                    | +40

        .size   Attract_State0_Handler_0967FE, .-Attract_State0_Handler_0967FE

| ============================================================================
| Handler #1 (state=1) @ $096840  (66 B) - full template
| ============================================================================
        .globl  Attract_State1_Handler_096840
        .type   Attract_State1_Handler_096840, @function
        .section .text.Attract_State1_Handler_096840, "ax", @progbits

Attract_State1_Handler_096840:
        move.b  #0x1, d0
        move.b  d0, 0x21(a6)
        jsr     0x43568.l
        move.b  #0x0, 0x10e39e.l
        move.w  #0x0, 0x70(a6)
        lea.l   .Lh1_cont(pc), a1
        move.l  a1, (a6)
.Lh1_cont:
        jsr     Attract_Sub_setup_967C0(pc)
        jsr     0x436de.l
        jsr     Attract_Sub_969C2(pc)
        jsr     ThunkTarget_096a80(pc)
        jsr     Attract_Sub_96B24(pc)
        move.b  #0x1, 0x10e39a.l
        rts

        .size   Attract_State1_Handler_096840, .-Attract_State1_Handler_096840

| ============================================================================
| Handler #2 (state=2) @ $096882  (66 B) - full template
| ============================================================================
        .globl  Attract_State2_Handler_096882
        .type   Attract_State2_Handler_096882, @function
        .section .text.Attract_State2_Handler_096882, "ax", @progbits

Attract_State2_Handler_096882:
        move.b  #0x2, d0
        move.b  d0, 0x21(a6)
        jsr     0x43568.l
        move.b  #0x0, 0x10e39e.l
        move.w  #0x0, 0x70(a6)
        lea.l   .Lh2_cont(pc), a1
        move.l  a1, (a6)
.Lh2_cont:
        jsr     Attract_Sub_setup_967C0(pc)
        jsr     0x436de.l
        jsr     Attract_Sub_969C2(pc)
        jsr     ThunkTarget_096a80(pc)
        jsr     Attract_Sub_96B24(pc)
        move.b  #0x1, 0x10e39a.l
        rts

        .size   Attract_State2_Handler_096882, .-Attract_State2_Handler_096882

| ============================================================================
| Handler #3 (state=3) @ $0968C4  (66 B) - full template
| ============================================================================
        .globl  Attract_State3_Handler_0968C4
        .type   Attract_State3_Handler_0968C4, @function
        .section .text.Attract_State3_Handler_0968C4, "ax", @progbits

Attract_State3_Handler_0968C4:
        move.b  #0x3, d0
        move.b  d0, 0x21(a6)
        jsr     0x43568.l
        move.b  #0x0, 0x10e39e.l
        move.w  #0x0, 0x70(a6)
        lea.l   .Lh3_cont(pc), a1
        move.l  a1, (a6)
.Lh3_cont:
        jsr     Attract_Sub_setup_967C0(pc)
        jsr     0x436de.l
        jsr     Attract_Sub_969C2(pc)
        jsr     ThunkTarget_096a80(pc)
        jsr     Attract_Sub_96B24(pc)
        move.b  #0x1, 0x10e39a.l
        rts

        .size   Attract_State3_Handler_0968C4, .-Attract_State3_Handler_0968C4

| ============================================================================
| Handler #4 (state=4) @ $096906  (66 B) - full template, 3 refs
|
| Referenciado 3 veces desde EE#3 (states #4, #8, #9 comparten este handler).
| ============================================================================
        .globl  Attract_State4_Handler_096906
        .type   Attract_State4_Handler_096906, @function
        .section .text.Attract_State4_Handler_096906, "ax", @progbits

Attract_State4_Handler_096906:
        move.b  #0x4, d0
        move.b  d0, 0x21(a6)
        jsr     0x43568.l
        move.b  #0x0, 0x10e39e.l
        move.w  #0x0, 0x70(a6)
        lea.l   .Lh4_cont(pc), a1
        move.l  a1, (a6)
.Lh4_cont:
        jsr     Attract_Sub_setup_967C0(pc)
        jsr     0x436de.l
        jsr     Attract_Sub_969C2(pc)
        jsr     ThunkTarget_096a80(pc)
        jsr     Attract_Sub_96B24(pc)
        move.b  #0x1, 0x10e39a.l
        rts

        .size   Attract_State4_Handler_096906, .-Attract_State4_Handler_096906

| ============================================================================
| Handler #5 (state=5) @ $096948  (66 B) - full template
| ============================================================================
        .globl  Attract_State5_Handler_096948
        .type   Attract_State5_Handler_096948, @function
        .section .text.Attract_State5_Handler_096948, "ax", @progbits

Attract_State5_Handler_096948:
        move.b  #0x5, d0
        move.b  d0, 0x21(a6)
        jsr     0x43568.l
        move.b  #0x0, 0x10e39e.l
        move.w  #0x0, 0x70(a6)
        lea.l   .Lh5_cont(pc), a1
        move.l  a1, (a6)
.Lh5_cont:
        jsr     Attract_Sub_setup_967C0(pc)
        jsr     0x436de.l
        jsr     Attract_Sub_969C2(pc)
        jsr     ThunkTarget_096a80(pc)
        jsr     Attract_Sub_96B24(pc)
        move.b  #0x1, 0x10e39a.l
        rts

        .size   Attract_State5_Handler_096948, .-Attract_State5_Handler_096948

| ============================================================================
| Handler #6 (state=7) @ $09698A  (56 B) - compact template
|
| Note: state index es 7 (no 6). No hace `move.w #0, $70(a6)` ni
| `jsr $969c2(pc)` como los anteriores.
| ============================================================================
        .globl  Attract_State7_Handler_09698A
        .type   Attract_State7_Handler_09698A, @function
        .section .text.Attract_State7_Handler_09698A, "ax", @progbits

Attract_State7_Handler_09698A:
        move.b  #0x7, d0                       | +00  state index = 7
        move.b  d0, 0x21(a6)                   | +04
        jsr     0x43568.l                      | +08
        move.b  #0x0, 0x10e39e.l               | +0e
                                               | (NO timer reset)
        lea.l   .Lh7_cont(pc), a1              | +16
        move.l  a1, (a6)                       | +1a
.Lh7_cont:                                     | $0969A6
        jsr     Attract_Sub_setup_967C0(pc)    | +1c
        jsr     0x436de.l                      | +20  Geom_Proj_Clamp
                                               | (NO jsr $969c2)
        jsr     ThunkTarget_096a80(pc)         | +26
        jsr     Attract_Sub_96B24(pc)          | +2a
        move.b  #0x1, 0x10e39a.l               | +2e
        rts                                    | +36

        .size   Attract_State7_Handler_09698A, .-Attract_State7_Handler_09698A
