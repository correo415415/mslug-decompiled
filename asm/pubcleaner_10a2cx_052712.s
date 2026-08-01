| ============================================================================
|  Metal Slug 1 - asm/pubcleaner_10a2cx_052712.s
|  ----------------------------------------------------------------------------
|  Wave LL batch 1 - publisher-cleaner del bloque de flags $10A2C8..$10A2D0.
|
|  Contenido (1 funcion, 40 bytes):
|
|      $052712   Pubcleaner_10A2Cx_052712        40 B  clear bloque flags
|
|  Llamada desde el cluster attract-wait (Wave FF batch 1) via jsr abs.l
|  ($052712.l). Ya estaba expuesto como alias externo ThunkTarget_052712
|  en tools/symbols.py; tras Wave LL, el simbolo canonico se define aqui
|  y el alias antiguo queda comentado con nota Wave LL.
|
|  ---------- Mapa de callers ------------------------------------------------
|
|      attract_cluster_batch_ff.s:71     jsr $52712.l   (palette fade)
|      (attract_cluster_batch_ff.s:297 lo documenta como paso 3 de la
|      secuencia de wake-up del sistema attract)
|
|  ---------- Semantica ------------------------------------------------------
|
|  Pubcleaner es el idioma "clear-by-move.b-of-zeroed-register" tipico del
|  asm hand-coded: en vez de emitir 6 clr.b abs.l independientes (que
|  ocuparian 8 B c/u = 48 B), zeroea d0 una sola vez con moveq #0, d0 y
|  reutiliza d0 como origen para 6 move.b abs.l (6 B c/u = 36 B). GCC no
|  genera este patron - usaria clr.b directamente porque el clr.b sobre
|  memoria absoluta es una instruccion sencilla en el 68000.
|
|  Layout del bloque $10A2C8..$10A2D0 (bloque de flags attract con hueco):
|
|      $10A2C8   [written]   attract_state_lo
|      $10A2C9   [skipped]   -- reservado (no se limpia)
|      $10A2CA   [written]   attract_timer_hi
|      $10A2CB   [written]   attract_frame_ctr
|      $10A2CC   [written]   attract_scene_idx
|      $10A2CD   [skipped]   -- reservado
|      $10A2CE   [skipped]   -- reservado
|      $10A2CF   [written]   attract_pad_state
|      $10A2D0   [written]   attract_fade_lvl
|
|  Los tres huecos ($C9, $CD, $CE) sugieren que este bloque comparte
|  espacio con otro subsistema (probablemente palette fade o input) que
|  los usa como storage privado y no debe tocarse en el reset attract.
|
|  Toolchain: m68k-linux-gnu-gcc -m68000 -Wa,--register-prefix-optional
|  ============================================================================

        .text

|
| ---------------------------------------------------------------------------
|  Pubcleaner_10A2Cx_052712  @ $052712  (40 bytes)
|
|  Limpia 6 bytes no-contiguos en el bloque de flags $10A2C8..$10A2D0
|  (con huecos en $C9, $CD, $CE). No preserva d0 (lo usa como origen del
|  cero y no lo restaura).
|
|  Firma conceptual:
|      void Pubcleaner_10A2Cx(void);
|
|  Efecto (equivalente C, no rederivable byte-a-byte porque GCC emitiria
|  6x clr.b en lugar de moveq #0 + 6x move.b):
|      attract_state_lo   = 0;   // $10A2C8
|      attract_timer_hi   = 0;   // $10A2CA
|      attract_frame_ctr  = 0;   // $10A2CB
|      attract_scene_idx  = 0;   // $10A2CC
|      attract_pad_state  = 0;   // $10A2CF
|      attract_fade_lvl   = 0;   // $10A2D0
| ---------------------------------------------------------------------------
|
        .globl  Pubcleaner_10A2Cx_052712
        .globl  ThunkTarget_052712              | alias externo FF (jsr abs.l)
        .type   Pubcleaner_10A2Cx_052712, @function
        .section .text.Pubcleaner_10A2Cx_052712, "ax", @progbits
Pubcleaner_10A2Cx_052712:
ThunkTarget_052712:
        moveq   #0x0, d0                        | +00  d0 = 0 (source)
        move.b  d0, 0x10a2c8.l                  | +02  clear attract_state_lo
        move.b  d0, 0x10a2ca.l                  | +08  clear attract_timer_hi
        move.b  d0, 0x10a2cb.l                  | +0e  clear attract_frame_ctr
        move.b  d0, 0x10a2cc.l                  | +14  clear attract_scene_idx
        move.b  d0, 0x10a2cf.l                  | +1a  clear attract_pad_state
        move.b  d0, 0x10a2d0.l                  | +20  clear attract_fade_lvl
        rts                                     | +26

        .size   Pubcleaner_10A2Cx_052712, .-Pubcleaner_10A2Cx_052712
