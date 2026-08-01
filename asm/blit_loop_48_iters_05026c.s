| ============================================================================
|  Metal Slug 1 - asm/blit_loop_48_iters_05026c.s
|  ----------------------------------------------------------------------------
|  Wave Z - #12
|
|  Blit_Loop48Iters_05026C  @ $05026C  (48 bytes, 1 caller)
|
|  Ejecuta 48 iteraciones ($30 decimal) del blitter de fila $43F5E (Wave T,
|  ya expuesto como `Sub_00043F5E`) sobre coordenadas iniciales d0=0, d1=$F0,
|  incrementando d0 en 8 pixeles por iteracion.
|
|  Setup previo: `jsr $44022.l` (blit setup: probable `Sub_00044022`, ya
|  registrado en symbols.py Wave Z).
|
|  Layout de la pila durante el bucle (post-setup):
|      SP+0  : contador de iteraciones (word, arranca en $30)
|      SP+2  : d0 stashed
|      SP+4  : d1 stashed
|
|  Firma C conceptual:
|
|      /* Renderiza una fila completa de 48 tiles (384 pixeles = pantalla
|       * ancho estandar Neo Geo) llamando al blitter Sub_00043F5E con
|       * (d7=1, d0=x, d1=y) por cada tile, incrementando x en 8 por iter. */
|      void Blit_Loop48Iters(void);
|
|  Notas forenses (por que NO es rederivable por GCC 1:1):
|    1. Contador en la PILA como `-(a7)` en vez de en un registro:
|       `move.w #$30, -(a7)` y decremento con `subq.w #1, (a7); bne`.
|       Idioma clasico asm hand-coded para liberar todos los registros
|       de datos para el callee. GCC habria usado un registro dedicado.
|    2. Los operandos del jsr $43F5E van SEPARADOS en la pila
|       (`move.w d0,-(a7); move.w d1,-(a7)` DESPUES del contador),
|       y se recuperan al final del bucle con `move.w (a7)+,d1; move.w
|       (a7)+,d0`. Es decir: convencion "args via pila explicita,
|       callee-preserved, sin frame pointer".
|    3. `d0 += 8` por iteracion y $30 iters => 384 pixeles horizontal =
|       ancho exacto de la pantalla Neo Geo (320+padding). Confirma que
|       este helper renderiza UNA FILA COMPLETA del fondo.
|    4. Cleanup del stack con `adda.w #$2, a7` (2 B) en vez de `addq.w
|       #$2, a7` (2 B tambien): eleccion inusual - patente hand-coded.
|       Ambos son 2 B, pero addq es el idioma recomendado para <=8. Sin
|       embargo el codegen es `de fc 00 02` = adda.w #2,a7 (4 B). GCC
|       habria emitido `addq.w #$2, a7`.
|
|  Toolchain:  m68k-linux-gnu-gcc -m68000 -Wa,--register-prefix-optional
|  ============================================================================

        .text
        .globl  Blit_Loop48Iters_05026C
        .type   Blit_Loop48Iters_05026C, @function
        .section .text.Blit_Loop48Iters_05026C, "ax", @progbits

Blit_Loop48Iters_05026C:
        move.w  #0x0, d0                       | +00  d0 = 0    (x inicial)
        move.w  #0xf0, d1                      | +04  d1 = 240  (y inicial)
        jsr     0x44022.l                      | +08  Sub_00044022 (blit setup)
        move.w  #0x30, -(a7)                   | +0e  push counter = 48
.Lloop:
        move.w  d0, -(a7)                      | +12  push d0 (x)
        move.w  d1, -(a7)                      | +14  push d1 (y)
        move.b  #0x1, d7                       | +16  d7 = 1 (blit mode)
        jsr     0x43f5e.l                      | +1a  Sub_00043F5E (blitter fila)
        move.w  (a7)+, d1                      | +20  pop d1
        move.w  (a7)+, d0                      | +22  pop d0
        addq.w  #0x8, d0                       | +24  d0 += 8  (avance x)
        subq.w  #0x1, (a7)                     | +26  --counter (top of stack)
        bne.b   .Lloop                         | +28  loop back
        adda.w  #0x2, a7                       | +2a  cleanup counter (4 B, hand-coded)
        rts                                    | +2e

        .size   Blit_Loop48Iters_05026C, .-Blit_Loop48Iters_05026C
