| ============================================================================
|  Metal Slug 1 - asm/task_walk_two_arenas_05dd2a.s
|  ----------------------------------------------------------------------------
|  Wave Z - #8
|
|  Task_WalkTwoArenas_05DD2A  @ $05DD2A  (34 bytes, 1 caller)
|
|  Ejecuta Task_WalkList_05B6 sobre dos arenas de tareas consecutivas:
|  primero $100800 (256 B despues de la arena principal $100080 del
|  scheduler central Y#1) y luego $1008A0 (160 B mas adelante).
|
|  Save/restore de a6 con `movem.l a6, -(a7)` porque el helper $5B6 modifica
|  a6 al recorrer la linked-list interna de la arena.
|
|  Firma C conceptual:
|
|      /* Recorre dos arenas de tareas secundarias ($100800 y $1008A0)
|       * invocando Task_WalkList_05B6 sobre cada una. Preserva a6
|       * original del caller. */
|      void Task_WalkTwoArenas(void);
|
|  Notas forenses (por que NO es rederivable por GCC 1:1):
|    1. `movem.l a6, -(a7)` para preservar UN SOLO registro (a6). Es mas
|       eficiente que `move.l a6, -(a7)`? No - ambos generan 4 B de
|       instruccion. Pero `movem` con un solo bit es CANONICO del asm
|       hand-coded para preservar registros durante llamadas: patente de
|       la convencion. GCC habria emitido `move.l a6, -(a7)` normal.
|    2. Dos `jsr abs.l` al mismo destino con formas identicas (6 B c/u).
|       GCC habria factorizado en un loop de 2 iteraciones si viera esta
|       simetria, o directamente en un array de 2 punteros.
|    3. $100800 y $1008A0 (diferencia de $A0 = 160 B) son direcciones de
|       arenas secundarias del scheduler central Y#1. El offset $A0 = 160
|       es multiplo de $40 (tamano de task node), sugiriendo que cada
|       arena tiene ~4 task nodes.
|
|  Toolchain:  m68k-linux-gnu-gcc -m68000 -Wa,--register-prefix-optional
|  ============================================================================

        .text
        .globl  Task_WalkTwoArenas_05DD2A
        .type   Task_WalkTwoArenas_05DD2A, @function
        .section .text.Task_WalkTwoArenas_05DD2A, "ax", @progbits

Task_WalkTwoArenas_05DD2A:
        movem.l a6, -(a7)                      | +00  push a6 via movem (idioma asm)
        lea.l   0x100800.l, a6                 | +04  a6 = &arena_secundaria_A
        jsr     0x5b6.l                        | +0a  Task_WalkList_05B6
        lea.l   0x1008a0.l, a6                 | +10  a6 = &arena_secundaria_B (+$A0)
        jsr     0x5b6.l                        | +16  Task_WalkList_05B6
        movem.l (a7)+, a6                      | +1c  pop a6
        rts                                    | +20

        .size   Task_WalkTwoArenas_05DD2A, .-Task_WalkTwoArenas_05DD2A
