| ============================================================================
|  Metal Slug 1 - asm/handler_timer_and_replace_001bcc.s
|  ----------------------------------------------------------------------------
|  Wave Z batch 2 - #13
|
|  Handler_TimerAndReplace_001BCC  @ $001BCC  (104 bytes)
|
|  Handler complejo de zona baja. Estructura general:
|    1. Guarda d0 con `and.b d0,d0` + move a d0=$10D7 y `jsr $2352` (guard);
|       recupera d0/d1 con `movem (a7)+, d0-d1`; clr.b d1.
|    2. Si contador global $106E92 >= $64 (=100): salta a $1C02 (skip decrement).
|    3. Si (d0 & $0F) != 0: tail-call a $47482 directo (bne.w $1C32).
|    4. Si (d0 & $7F) != 0: skip a $1C02 (skip decrement).
|    5. En otro caso: --$106E92, sigue a $1C02.
|    6. En $1C02: si $106E92 > 0: salta a $1C2C (tail-call $47482).
|    7. Si $106E92 <= 0: publica $FF en $21(a6), reserva entity desde
|       template $46A48, resetea $106E92 a 0, publica handler continuacion
|       en (a6) = &Sub_00001C34.
|    8. En $1C2C: `jsr $47482.l; rts` = 20 FP absorbido (JsrAbsThunk_001c2c).
|
|  Firma C conceptual:
|
|      /* Handler de timer con reset condicional: si counter global
|       * $106E92 llega a 0, spawna entity desde template $46A48 e instala
|       * el handler continuacion $1C34; en cualquier caso llama al post-hook
|       * $47482. */
|      void Handler_TimerAndReplace(struct Entity *self /*a6*/);
|
|  Toolchain:  m68k-linux-gnu-gcc -m68000 -Wa,--register-prefix-optional
|  ============================================================================

        .text
        .globl  Handler_TimerAndReplace_001BCC
        .type   Handler_TimerAndReplace_001BCC, @function
        .section .text.Handler_TimerAndReplace_001BCC, "ax", @progbits

Handler_TimerAndReplace_001BCC:
        and.b   d0, d0                         | +00  fija CCR = flags(d0)
        move.w  #0x10d7, d0                    | +02  d0 = $10D7 (guard token)
        jsr     0x2352.l                       | +06  InputGuardCall219c (Wave A)
        movem.l (a7)+, d0-d1                   | +0c  restaura d0/d1
        clr.b   d1                             | +10  d1 low = 0
        cmpi.w  #0x64, 0x106e92.l              | +12  if (counter >= 100)
        bcc.w   .Lat_1c02                      | +1a     skip decrement
        move.b  d0, d2                         | +1e  d2 = d0
        andi.b  #0xf, d2                       | +20  d2 &= 0x0F (low nibble)
        bne.w   .Lexit_rts                     | +24  if (low nibble != 0) exit directo
        andi.b  #0x7f, d0                      | +28  d0 &= 0x7F
        bne.w   .Lat_1c02                      | +2c  if (d0 != 0) skip decrement
        subq.w  #0x1, 0x106e92.l               | +30  --counter
.Lat_1c02:
        tst.w   0x106e92.l                     | +36  if (counter > 0)
        bgt.w   .Ltail_47482                   | +3c     tail-call
                                              |
                                              | ---- counter agotado: instala continuacion ----
        move.b  #0xff, 0x21(a6)                | +40  self->field21 = $FF
        lea.l   0x46a48.l, a1                  | +46  a1 = &Template_46A48
        jsr     0x4ae.l                        | +4c  Task_AllocFromFreeList (T#4)
        clr.w   0x106e92.l                     | +52  counter = 0
        clr.b   d1                             | +58  d1 low = 0
        lea     .Lnext_handler(pc), a1         | +5a  a1 = &Sub_00001C34
        move.l  a1, (a6)                       | +5e  self->handler = &Sub_00001C34
                                              |
.Ltail_47482:
        jsr     0x47482.l                      | +60  Sub_00047482 (post-hook)
.Lexit_rts:
        rts                                    | +66  (target del bne.w low-nibble)

        .equ    .Lnext_handler, Sub_00001C34

        .size   Handler_TimerAndReplace_001BCC, .-Handler_TimerAndReplace_001BCC
