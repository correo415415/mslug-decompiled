| ============================================================================
|  Metal Slug 1 - asm/list_apply_sentinel_047888.s
|  ----------------------------------------------------------------------------
|  Wave Z - #7
|
|  List_ApplyWithSentinelFF_047888  @ $047888  (38 bytes, 1 caller)
|
|  Recorre una lista de words apuntada por a5 hasta el centinela `$00FF`,
|  invocando la callback PC-rel en $47872 con cada elemento. En cada
|  iteracion:
|    - lee un word con `(a5)+`
|    - si es $00FF, sale del bucle
|    - restaura los originales (d1=d4, a1=a4)
|    - llama a $47872(pc) con esos parametros
|    - avanza a4 en $40 bytes (proximo slot de 64 B)
|
|  Firma C conceptual:
|
|      /* Recorre lista de words desde a5, invocando callback $47872 con
|       * (d1_orig=d4, a1_orig=a4) para cada elemento, hasta encontrar el
|       * centinela $00FF. Avanza a4 en $40 bytes por iteracion. */
|      void List_ApplyWithSentinelFF(uint16_t *list /*a5*/,
|                                    void *table /*a4=a1_orig*/,
|                                    uint16 d1_saved /*d1*/);
|
|  Notas forenses (por que NO es rederivable por GCC 1:1):
|    1. Al inicio del bucle guarda d1->d4 y a1->a4 y los reusa para invocar
|       la callback: `move.w d1,d4` y `movea.l a1,a4` como save/restore.
|       Idioma clasico de asm hand-coded, GCC habria usado pila o registros
|       callee-saved.
|    2. `06 80 00 00 00 40 28 40` = `addi.l #$40, d0; movea.l d0, a4`
|       en vez de `adda.l #$40, a4` que seria mas corto (4 B vs 8 B).
|       GCC habria elegido `adda.l`. La eleccion `addi.l+movea.l` sugiere
|       que el resultado se usa tambien como d0 en algun otro lugar
|       (bucle acumulador oculto).
|    3. El offset entre elementos es $40 (64 B) = tamano tipico de un
|       task node del scheduler central (Y#1). Es decir, la lista apunta
|       a task nodes y $47872 es una operacion PER-TASK.
|
|  Toolchain:  m68k-linux-gnu-gcc -m68000 -Wa,--register-prefix-optional
|  ============================================================================

        .text
        .globl  List_ApplyWithSentinelFF_047888
        .type   List_ApplyWithSentinelFF_047888, @function
        .section .text.List_ApplyWithSentinelFF_047888, "ax", @progbits

List_ApplyWithSentinelFF_047888:
        move.w  d1, d4                         | +00  d4 = d1_original (save)
        movea.l a1, a4                         | +02  a4 = a1_original (save)
        movea.l a2, a5                         | +04  a5 = list cursor
.Lloop:
        move.w  (a5)+, d0                      | +06  d0 = *cursor++, list[i]
        cmpi.w  #0xff, d0                      | +08  if (d0 == $00FF)
        beq.w   .Lend                          | +0c     goto end
        move.w  d4, d1                         | +10  restore d1 for callback
        movea.l a4, a1                         | +12  restore a1 for callback
        jsr     .Lcallback(pc)                 | +14  Sub_00047872(d1, a1, d0)
        move.l  a4, d0                         | +18  d0 = a4 as long
        addi.l  #0x40, d0                      | +1a  d0 += 0x40 (task-node stride)
        movea.l d0, a4                         | +20  a4 = d0 (advance to next)
        bra.b   .Lloop                         | +22  loop back
.Lend:
        rts                                    | +24

        .equ    .Lcallback, Sub_00047872

        .size   List_ApplyWithSentinelFF_047888, .-List_ApplyWithSentinelFF_047888
