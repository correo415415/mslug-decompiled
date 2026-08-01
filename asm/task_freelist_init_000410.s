| ============================================================================
|  Metal Slug 1 - asm/task_freelist_init_000410.s
|  ----------------------------------------------------------------------------
|  Wave DD - #5
|
|  Task_FreeListInit_000410  @ $000410  (158 B, 1 caller)
|
|  Inicializador del task free-list del scheduler. Complementa Wave R
|  (Scheduler central) instalando los nodos iniciales.
|
|  Estructura:
|
|    1. FASE A: bucle inicial de $A0 = 160 nodos, cada uno de $A0 B, en el
|       rango $100A80..$100A80+$A0*$A0 = $100A80..$1104880 (aproximado).
|       Cada nodo se inicializa via los helpers ya matcheados $5DC1C
|       (Entity_ClearPtrSlots) y $5DC34 (Entity_InitFields):
|
|         a0 = $100A80              (first free node)
|         $106E80 = a0              (head of free-list)
|         d0 = $A0 = 160            (node count)
|         a1 = a0                   (a1 will trail a0 by one stride)
|       loop_A:
|         movem.l d0-d7/a0-a6, -(a7)   (save state around helpers)
|         jsr $5DC1C                    (Entity_ClearPtrSlots)
|         jsr $5DC34                    (Entity_InitFields)
|         movem.l (a7)+, d0-d7/a0-a6   (restore)
|         a0 += $A0                    (advance a0 to next node)
|         a1[$8] = a0                  (link a1 -> next)
|         --d0
|         bne loop_A
|         a1[$8] = $FFFFFFFF           (sentinel: last node's next = NIL)
|
|    2. FASE B: bucle de instalacion de handlers via tabla en $278000.
|       Este es el "task installer" que lee 32 entradas (2 longs c/u) desde
|       $278000..$278100 = 32 * $8 = 256 B de tabla:
|
|         a1 = $100940                 (segundo pool: 32 slots contiguos)
|         a2 = $278000                 (task descriptor table)
|       loop_B:
|         a6 = *a2                     (leer handler ptr)
|         a0 = a6                      (a6 = a0 = current task node)
|         movem.l d0-d7/a0-a6, -(a7)
|         jsr $5DC1C
|         jsr $5DC34
|         movem.l (a7)+, d0-d7/a0-a6
|         a6->flag10 = 0
|         a6->flag12 = 0
|         a6->flag12 |= 1              (set bit 0)
|         a6->slot0C = a6              (self-pointer at $C)
|         d0 = a2[4]                   (leer 2do long del descriptor)
|         a6[0] = d0                   (publicar como task_handler_ptr)
|         a6[4] = a1                   (link prev)
|         a1[8] = a6                   (link next)
|         a1 = a6
|         a2 += 8                      (avanzar en la tabla)
|         if (a6 != $100940) loop_B    (hasta completar los 32)
|
|         a6->slot8 = $100080          (tail-pointer al slot final)
|         rts
|
|    3. Segunda funcion contigua $0004AE (88 B) es el `Task_AllocFromFreeList`
|       propiamente dicho — se aparca para oleada dedicada (no forma parte
|       de esta funcion `Task_FreeListInit`).
|
|  Idiomas hand-coded:
|    - `movem.l d0-d7/a0-a6, -(a7)` + `movem.l (a7)+, d0-d7/a0-a6` para
|      preservar TODO el estado alrededor de dos jsr consecutivos. GCC
|      nunca emitiria esto porque conoce las clobber-lists.
|    - `d1.fc ff ff ff ff` = `move.l #$FFFFFFFF, ...` como sentinel de
|      "unused" en el ultimo nodo del free-list.
|    - Puntero self via `move.l a6, $C(a6)` (a6 = self, y $C(a6) = self-ptr).
|
|  Firma C conceptual:
|
|      /* Inicializa la free-list del scheduler:
|       *   1. $A0 nodos de $A0 B cada uno en $100A80..
|       *   2. 32 tareas iniciales via tabla en $278000
|       * Complementa Wave R (Scheduler_MainLoop) instalando los nodos que
|       * el scheduler consumira despues. */
|      void Task_FreeListInit(void);
|
|  Toolchain:  m68k-linux-gnu-gcc -m68000 -Wa,--register-prefix-optional
|  ============================================================================

        .text
        .globl  Task_FreeListInit_000410
        .type   Task_FreeListInit_000410, @function
        .section .text.Task_FreeListInit_000410, "ax", @progbits

Task_FreeListInit_000410:
                                              | ---- FASE A: $A0 nodos ----
        lea.l   0x100a80.l, a0                 | +00  a0 = first free node
        move.l  a0, 0x106e80.l                 | +06  $106E80 = a0
        move.w  #0xa0, d0                      | +0c  d0 = 160
.LA_loop:
        movea.l a0, a1                         | +10  a1 = a0 (loop body)
        movem.l d0-d7/a0-a6, -(a7)             | +12  save all
        jsr     0x5dc1c.l                      | +16  Entity_ClearPtrSlots
        jsr     0x5dc34.l                      | +1c  Entity_InitFields
        movem.l (a7)+, d0-d7/a0-a6             | +22  restore
        adda.l  #0xa0, a0                      | +26  a0 += $A0
        move.l  a0, 0x8(a1)                    | +2c  a1[8] = a0 (link fwd)
        subq.w  #0x1, d0                       | +30  --d0
        bne.b   .LA_loop                       | +32  loop
        move.l  #0xffffffff, 0x8(a1)           | +34  last->next = NIL
                                              |
                                              | ---- FASE B: tabla $278000 ----
        lea.l   0x100940.l, a1                 | +3c  a1 = $100940
        lea.l   0x278000.l, a2                 | +42  a2 = descriptor tbl
.LB_loop:
        movea.l (a2), a6                       | +48  a6 = *a2
        movea.l a6, a0                         | +4a  a0 = a6
        movem.l d0-d7/a0-a6, -(a7)             | +4c  save
        jsr     0x5dc1c.l                      | +50  Entity_ClearPtrSlots
        jsr     0x5dc34.l                      | +56  Entity_InitFields
        movem.l (a7)+, d0-d7/a0-a6             | +5c  restore
        move.b  #0x0, 0x10(a6)                 | +60  a6->flag10 = 0
        move.b  #0x0, 0x12(a6)                 | +66  a6->flag12 = 0
        bset.b  #0x0, 0x12(a6)                 | +6c  flag12 |= 1
        move.l  a6, 0xc(a6)                    | +72  a6->self at $C
        move.l  0x4(a2), d0                    | +76  d0 = *(a2+4)
        move.l  d0, (a6)                       | +7a  a6[0] = d0
        move.l  a1, 0x4(a6)                    | +7c  a6[4] = a1
        move.l  a6, 0x8(a1)                    | +80  a1[8] = a6
        movea.l a6, a1                         | +84  a1 = a6
        adda.l  #0x8, a2                       | +86  a2 += 8
        cmpa.l  #0x100940, a6                  | +8c  if (a6 != $100940)
        bne.b   .LB_loop                       | +92     continue
        move.l  #0x100080, 0x8(a6)             | +94  tail-pointer
        rts                                    | +9c

        .size   Task_FreeListInit_000410, .-Task_FreeListInit_000410
