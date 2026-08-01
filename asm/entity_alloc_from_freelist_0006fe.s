| ============================================================================
|  Metal Slug 1 - asm/entity_alloc_from_freelist_0006fe.s
|  ----------------------------------------------------------------------------
|  Wave W (Sprite slot allocator) - funcion #16
|
|  Entity_AllocFromFreeList_0006FE  @ $0006FE  (108 bytes, 2 callers directos
|                                                + 1 caller indirecto via
|                                                JsrAbsThunk_XXX de Wave I)
|
|  Reserva un entity del free-list en $106E80 (linked list simple con next
|  en offset $8), lo inicializa completamente, y lo inserta en la lista
|  de hijos del entity padre (a6) manteniendo el orden por prioridad
|  $10(entity) (byte).
|
|  Detalle forense CRITICO: la rama "free-list vacio" NO retorna con un rts
|  propio. En su lugar, `beq.w` salta a la FUNCION CONTIGUA en $076A que
|  reasigna a0 al puntero de un DUMMY_ENTITY global ($1009E0) antes de su
|  propio rts. Idioma clasico de asm hand-coded para evitar que el caller
|  tenga que comprobar por NULL: siempre recibe un puntero valido, aunque
|  sea a un placeholder.
|
|  Firma C conceptual:
|
|      /* Reserva un entity del free-list, lo inicializa por completo, y
|       * lo inserta en la lista de hijos del padre (a6) en orden por
|       * prioridad ascendente segun $10(entity). Retorna con a0 = nuevo
|       * entity o a0 = DUMMY_ENTITY_$1009E0 si el free-list esta vacio. */
|      struct Entity *Entity_AllocFromFreeList(void *script /*a1*/,
|                                              struct Entity *parent /*a6*/);
|
|  Notas forenses (por que NO es rederivable por GCC 1:1):
|    1. movem.l d0-d7/a0-a6, -(a7) preserva TODOS los registros ANTES del
|       jsr a helpers hermanos. Convencion muy poco C: GCC solo preservaria
|       callee-saved.
|    2. Doble jsr a $5DC34 (Entity_InitFields) - antes y despues de la
|       insercion en la linked list.
|    3. bls.b reentra en la MISMA instruccion `movea.l $8(a1),a1` - do-while
|       natural con `unsigned <=` como predicado, patron clasico de linked
|       list traversal.
|    4. Rama "empty" tail-calls a la funcion CONTIGUA ($076A) en vez de
|       tener rts propio. GCC nunca emitiria un branch condicional cuya
|       salida "nula" es la siguiente funcion definida.
|    5. Absorbio JsrAbsThunk_000762 (Wave I): los ultimos 8 B del helper
|       (`jsr $5DC34.l; rts`) fueron erroneamente contabilizados como
|       thunk independiente. Septimo falso positivo del proyecto.
|
|  Toolchain:  m68k-linux-gnu-gcc -m68000 -Wa,--register-prefix-optional
|  ============================================================================

        .text
        .globl  Entity_AllocFromFreeList_0006FE
        .type   Entity_AllocFromFreeList_0006FE, @function
        .section .text.Entity_AllocFromFreeList_0006FE, "ax", @progbits

Entity_AllocFromFreeList_0006FE:
        movea.l 0x106e80.l, a0                | +00  a0 = free-list head
        cmpa.l  #0xffffffff, a0                | +06  ¿ free-list vacio ?
        beq.w   EmptyEntity_Init_00076A        | +0c  si: tail-call a $076A
        move.l  0x8(a0), 0x106e80.l            | +10  head = a0->next
        movem.l d0-d7/a0-a6, -(a7)             | +18  push TODOS los registros
        jsr     Entity_ClearPtrSlots_05DC1C    | +1c  (W#15) limpia $10..$9C con NIL
        jsr     Entity_InitFields_05DC34       | +22  (V#5) init 24 campos + reserve
        movem.l (a7)+, d0-d7/a0-a6             | +28  pop registros
        move.l  a1, (a0)                       | +2c  a0->script_ptr = a1
        move.l  a6, 0x0c(a0)                   | +2e  a0->parent = a6
        move.b  0x10(a6), d0                   | +32  d0 = parent->prio_byte
        addq.b  #0x1, d0                       | +36  d0 += 1
        move.b  d0, 0x10(a0)                   | +38  a0->prio_byte = d0
        movea.l a6, a1                         | +3c  a1 = parent (cursor de busqueda)
.Lsearch:
        movea.l 0x8(a1), a1                    | +3e  a1 = a1->next
        cmp.b   0x10(a1), d0                   | +40  ¿ d0 vs a1->prio_byte ?
        bls.b   .Lsearch                       | +44  d0 <= a1->prio: seguir
        movem.l a2, -(a7)                      | +46  push a2
        movea.l 0x4(a1), a2                    | +4a  a2 = a1->prev
        move.l  a0, 0x8(a2)                    | +4e  a2->next = a0
        move.l  a2, 0x4(a0)                    | +52  a0->prev = a2
        move.l  a1, 0x8(a0)                    | +56  a0->next = a1
        move.l  a0, 0x4(a1)                    | +5a  a1->prev = a0
        movem.l (a7)+, a2                      | +5e  pop a2
        jsr     Entity_InitFields_05DC34       | +62  (V#5) SEGUNDA pasada de init
        rts                                    | +68
        .size   Entity_AllocFromFreeList_0006FE, .-Entity_AllocFromFreeList_0006FE
