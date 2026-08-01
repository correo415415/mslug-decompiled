| ============================================================================
|  Metal Slug 1 - asm/entity_build4_from_templates_0818aa.s
|  ----------------------------------------------------------------------------
|  Wave Y - #11
|
|  Entity_Build4FromTemplates_0818AA  @ $0818AA  (66 bytes, 1 caller)
|
|  Spawn en batch de CUATRO entities independientes desde cuatro templates
|  PC-rel contiguos. A diferencia de Entity_Build3ChainCircular_03060A
|  (Y#10), no encadena los cuatro entities entre si ni escribe slot alguno
|  en el padre - simplemente los reserva y aplica Entity_CopyTransform,
|  descartando los punteros (a0 se sobreescribe en cada iteracion sin
|  guardar).
|
|  Templates apuntados (pc-rel calculado desde cada lea):
|      $8121C  = template #1
|      $8123C  = template #2
|      $81260  = template #3
|      $81284  = template #4
|
|  Firma C conceptual:
|
|      /* Reserva cuatro entities hijos independientes desde cuatro
|       * templates estaticos y les aplica CopyTransform. No guarda
|       * ni encadena los punteros. Uso tipico: HUD estatico, formacion
|       * fija o grupo de disparos simultaneos donde el padre no
|       * necesita reencontrar los hijos. */
|      void Entity_Build4FromTemplates(struct Entity *parent /*a6*/);
|
|  Notas forenses (por que NO es rederivable por GCC 1:1):
|    1. Cuatro bloques identicos de 16 B (lea+jsr+jsr) sin loop.
|       GCC hubiese emitido un bucle con un array de templates, o
|       hoisting comun. Aqui todo es unrolled a mano.
|    2. Los punteros devueltos por Task_AllocFromFreeList se pierden:
|       cada nuevo jsr sobreescribe a0. Es decir, el juego no necesita
|       reencontrar estos 4 hijos por indice - se autogestionan via
|       linked-list del scheduler central o por otro medio.
|    3. Los templates estan HACIA DELANTE (offsets pc-rel positivos:
|       $F970, $F980, $F994, $F9A8), lo cual es inusual - lo normal
|       es tener code delante y datos detras. Indica que el bloque de
|       templates $8121C..$812xx es una tabla estatica del ROM que
|       comparten multiples spawners.
|    4. Absorbio JsrAbsThunk_0818e4 (Wave I): los ultimos 8 B de la funcion
|       (`jsr $5dd02.l; rts`) fueron erroneamente contabilizados como thunk
|       independiente. Noveno falso positivo del proyecto, mismo patron que
|       los absorbidos por Entity_AllocFromFreeList_0006FE (W#16) y por
|       Init_MasterSubsystems_0020E2 (Y#8).
|
|  Toolchain:  m68k-linux-gnu-gcc -m68000 -Wa,--register-prefix-optional
|  ============================================================================

        .text
        .globl  Entity_Build4FromTemplates_0818AA
        .type   Entity_Build4FromTemplates_0818AA, @function
        .section .text.Entity_Build4FromTemplates_0818AA, "ax", @progbits

Entity_Build4FromTemplates_0818AA:
                                              | ---- Reserva entity #1 (template $8121C) ----
        lea     .LTpl1(pc), a1                 | +00  a1 = &Template1
        jsr     0x4ae.l                        | +04  Task_AllocFromFreeList (T#4)
        jsr     0x5dd02.l                      | +0a  Entity_CopyTransform (S#4)
                                              |
                                              | ---- Reserva entity #2 (template $8123C) ----
        lea     .LTpl2(pc), a1                 | +10  a1 = &Template2
        jsr     0x4ae.l                        | +14  Task_AllocFromFreeList (T#4)
        jsr     0x5dd02.l                      | +1a  Entity_CopyTransform (S#4)
                                              |
                                              | ---- Reserva entity #3 (template $81260) ----
        lea     .LTpl3(pc), a1                 | +20  a1 = &Template3
        jsr     0x4ae.l                        | +24  Task_AllocFromFreeList (T#4)
        jsr     0x5dd02.l                      | +2a  Entity_CopyTransform (S#4)
                                              |
                                              | ---- Reserva entity #4 (template $81284) ----
        lea     .LTpl4(pc), a1                 | +30  a1 = &Template4
        jsr     0x4ae.l                        | +34  Task_AllocFromFreeList (T#4)
        jsr     0x5dd02.l                      | +3a  Entity_CopyTransform (S#4)
                                              |
        rts                                    | +40

        .equ    .LTpl1, Template_08121C
        .equ    .LTpl2, Template_08123C
        .equ    .LTpl3, Template_081260
        .equ    .LTpl4, Template_081284

        .size   Entity_Build4FromTemplates_0818AA, .-Entity_Build4FromTemplates_0818AA
