| ============================================================================
|  Metal Slug 1 - asm/scheduler_compare_field10_0006f0.s
|  ----------------------------------------------------------------------------
|  Wave Y (Scheduler central + arranque + colas + constructores de entities) - #2
|
|  Scheduler_CompareField10_0006F0  @ $0006F0  (14 bytes, callers via
|                                                fall-through desde $65C
|                                                del Scheduler_MainLoop y
|                                                probablemente otros callers
|                                                directos aun no matcheados)
|
|  Compara el campo prio_byte (offset $10) del task node actual (a6) con el
|  del task node siguiente (a6->next en $8(a6)). Deja el resultado en CCR
|  y retorna: Z=1 si iguales, N/C reflejan la resta con signo/sin signo.
|
|  Firma C conceptual:
|
|      /* Retorna CCR con la comparacion prio(a6) vs prio(a6->next).
|       * Callers la usan como predicado directo con bcc/bls/beq/etc.
|       * sin transferencia a d0. */
|      /* void */ int Scheduler_CompareField10(struct TaskNode *cur /*a6*/);
|
|  Notas forenses (por que NO es rederivable por GCC 1:1):
|    1. Retorno por CCR (no por d0). GCC ABI siempre devuelve el resultado
|       de una comparacion en d0/tst, nunca dejando implicitamente el CCR.
|    2. Funcion contigua al Scheduler_MainLoop_000656: la salida "lista
|       agotada" del bucle principal cae AQUI por diseno cuando a6 alcanza
|       el centinela cuyo campo $10 satisface el predicado del caller.
|       Idioma "fall-through al vecino" clasico de asm hand-coded, ya visto
|       en Wave S#2 (Entity_HasLinkedSlots -> Script_DispatchOpcode) y en
|       Wave X#2 (Decimal_Clamp99999999 -> Sub_BinToDecimalDecoder).
|
|  Toolchain:  m68k-linux-gnu-gcc -m68000 -Wa,--register-prefix-optional
|  ============================================================================

        .text
        .globl  Scheduler_CompareField10_0006F0
        .type   Scheduler_CompareField10_0006F0, @function
        .section .text.Scheduler_CompareField10_0006F0, "ax", @progbits

Scheduler_CompareField10_0006F0:
        movea.l 0x8(a6), a0                    | +00  a0 = cur->next
        move.b  0x10(a6), d0                   | +04  d0 = cur->prio_byte
        cmp.b   0x10(a0), d0                   | +08  compare vs a0->prio_byte
        rts                                    | +0c  return with CCR set

        .size   Scheduler_CompareField10_0006F0, .-Scheduler_CompareField10_0006F0
