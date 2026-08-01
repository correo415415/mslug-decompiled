| ============================================================================
|  Metal Slug 1 - asm/entity_build3_chain_circular_03060a.s
|  ----------------------------------------------------------------------------
|  Wave Y - #10
|
|  Entity_Build3ChainCircular_03060A  @ $03060A  (140 bytes, 1 caller)
|
|  Reserva tres entities hijos desde tres templates PC-rel contiguos y los
|  cablea en una linked-list circular de 3 nodos donde cada nodo tiene tres
|  punteros ($70=prev, $74=other, $78=next) que apuntan al mismo trio en
|  orden fijo. El primer entity va al slot $7C(padre), el segundo a $74,
|  el tercero a $78; cada uno recibe copia de $82(padre) en $24 y limpia $98.
|
|  Templates apuntados (pc-rel calculado desde el lea):
|      $3010C  = template #1  -> parent->$7C
|      $30068  = template #2  -> parent->$74
|      $300BA  = template #3  -> parent->$78
|
|  Firma C conceptual:
|
|      /* Reserva tres entities hijos, publica sus punteros en tres slots
|       * del padre ($7C, $74, $78), copia $82(parent) a cada uno y limpia
|       * $98. Finaliza cableando entre los tres una lista circular por
|       * los campos $70 (=prev), $74 (=other), $78 (=next) con orientacion
|       * identica en cada nodo. Todos los templates y todos los offsets
|       * son literales; funcion NO parametrica. */
|      void Entity_Build3ChainCircular(struct Entity *parent /*a6*/);
|
|  Notas forenses (por que NO es rederivable por GCC 1:1):
|    1. Tres bloques identicos de 30 B (lea+jsr+jsr+move+move+clr) sin
|       loop unrolled a mano: GCC preferiria un bucle o hoisting comun.
|    2. La fase de cableado usa TRES registros de direccion en paralelo
|       (a2, a3, a0) para escribir simultaneamente los mismos 3 valores
|       en los mismos 3 offsets de cada uno. GCC serializaria las
|       cargas y escrituras via un unico registro.
|    3. Los tres templates estan hacia atras en la ROM ($3010C, $30068,
|       $300BA) todos ANTES de la propia funcion ($3060A) - es un patron
|       "templates delante, code detras" clasico de asm hand-coded.
|    4. `21 4A 00 70` (`move.l a2, $70(a0)`) genera el pareo especifico
|       de la triada circular. Si intercambiaramos a2 por a3 o el offset
|       $70 por $74 el juego rompe la sincronizacion de sub-entities.
|
|  Toolchain:  m68k-linux-gnu-gcc -m68000 -Wa,--register-prefix-optional
|  ============================================================================

        .text
        .globl  Entity_Build3ChainCircular_03060A
        .type   Entity_Build3ChainCircular_03060A, @function
        .section .text.Entity_Build3ChainCircular_03060A, "ax", @progbits

Entity_Build3ChainCircular_03060A:
                                              | ---- Reserva entity #1 en slot $7C ----
        lea     .LTpl1(pc), a1                 | +00  a1 = &Template1
        jsr     0x4ae.l                        | +04  Task_AllocFromFreeList (T#4)
        jsr     0x5dd02.l                      | +0a  Entity_CopyTransform (S#4)
        move.l  a0, 0x7c(a6)                   | +10  parent->slot7C = new1
        move.w  0x82(a6), 0x24(a0)             | +14  new1->field24 = parent->field82
        clr.b   0x98(a0)                       | +1a  new1->field98 = 0
                                              |
                                              | ---- Reserva entity #2 en slot $74 ----
        lea     .LTpl2(pc), a1                 | +1e  a1 = &Template2
        jsr     0x4ae.l                        | +22  Task_AllocFromFreeList (T#4)
        jsr     0x5dd02.l                      | +28  Entity_CopyTransform (S#4)
        move.l  a0, 0x74(a6)                   | +2e  parent->slot74 = new2
        move.w  0x82(a6), 0x24(a0)             | +32  new2->field24 = parent->field82
        clr.b   0x98(a0)                       | +38  new2->field98 = 0
                                              |
                                              | ---- Reserva entity #3 en slot $78 ----
        lea     .LTpl3(pc), a1                 | +3c  a1 = &Template3
        jsr     0x4ae.l                        | +40  Task_AllocFromFreeList (T#4)
        jsr     0x5dd02.l                      | +46  Entity_CopyTransform (S#4)
        move.l  a0, 0x78(a6)                   | +4c  parent->slot78 = new3
        move.w  0x82(a6), 0x24(a0)             | +50  new3->field24 = parent->field82
        clr.b   0x98(a0)                       | +56  new3->field98 = 0
                                              |
                                              | ---- Recupera los tres punteros en a2/a3/a0 ----
        movea.l 0x74(a6), a2                   | +5a  a2 = parent->slot74 (= new2)
        movea.l 0x78(a6), a3                   | +5e  a3 = parent->slot78 (= new3)
        movea.l 0x7c(a6), a0                   | +62  a0 = parent->slot7C (= new1)
                                              |
                                              | ---- Cablea la lista circular con orientacion
                                              |      identica en los tres nodos ----
        move.l  a2, 0x70(a2)                   | +66  new2->prev  = new2   (self-ref)
        move.l  a0, 0x78(a2)                   | +6a  new2->next  = new1
        move.l  a3, 0x74(a2)                   | +6e  new2->other = new3
                                              |
        move.l  a2, 0x70(a3)                   | +72  new3->prev  = new2
        move.l  a0, 0x78(a3)                   | +76  new3->next  = new1
        move.l  a3, 0x74(a3)                   | +7a  new3->other = new3   (self-ref)
                                              |
        move.l  a2, 0x70(a0)                   | +7e  new1->prev  = new2
        move.l  a0, 0x78(a0)                   | +82  new1->next  = new1   (self-ref)
        move.l  a3, 0x74(a0)                   | +86  new1->other = new3
                                              |
        rts                                    | +8a

        .equ    .LTpl1, Template_03010C
        .equ    .LTpl2, Template_030068
        .equ    .LTpl3, Template_0300BA

        .size   Entity_Build3ChainCircular_03060A, .-Entity_Build3ChainCircular_03060A
