| ============================================================================
|  Metal Slug 1 - asm/entity_install_handler_077c7e.s
|  ----------------------------------------------------------------------------
|  Wave T (Script / Draw / Task primitives) - funcion #6
|
|  Entity_InstallHandlerAndCopyXf  @ $077c7e  (26 bytes, 8 callers)
|
|  Envoltura que instala un handler literal ($77c98) sobre la entidad
|  destino a0, copia el transform desde la entidad fuente a6 mediante
|  Entity_CopyTransform (Wave S#4) y guarda el "a1 original del caller"
|  en dst->field_70.
|
|  Flujo:
|      1. Guarda a1 (parametro logico del caller: probable puntero a
|         datos animados, ver punto 4).
|      2. Carga a1 = &Handler_077c98 mediante lea PC-relativo corto (4 B).
|         El literal Handler_077c98 es codigo 68k valido (empieza con
|         movea.l $70(a6),a0 ; move.b $1(a0),$74(a0) ; ...) - es un
|         subprograma completo, no una tabla de datos.
|      3. jsr $6fe.l -> Task_ChangeHandler_XXX (aun sin nombre estable;
|         registrado provisionalmente como Fn_0000_06FE).
|         Le pasa como argumento el handler cargado en a1.
|      4. jsr $5dd02.l -> Entity_CopyTransform  (Wave S#4).
|         Copia los 5 campos pos_x, pos_y, flags38, flags3a, flags11
|         de a6 (src) a a0 (dst).
|      5. Restaura a1 original.
|      6. Guarda a1 (original del caller) en dst->field_70.
|         El offset $70 sugiere "puntero a datos" tardio en la struct
|         Entity - probablemente el proximo puntero de animacion.
|      7. rts.
|
|  Entrada (registros absolutos, convencion no-C):
|      a0 : entidad destino (donde se instalara el handler y el a1 original)
|      a1 : puntero logico (payload, se guarda en dst->field_70)
|      a6 : entidad fuente (para el copy_transform)
|
|  Salida: ninguna (retorna via rts).
|
|  Hallazgos forenses:
|    1. lea $77c98(pc),a1 usa PC-rel corto (43 fa 00 16 = 4 B), mientras
|       que los dos jsr son abs.l (6 B cada uno). Aunque este mix es
|       tolerable por -mpcrel de GCC (auto-fallback a abs.l cuando el
|       target no cabe en 16-bit), el patron combinado con el ABI de
|       registros absolutos (a0 vivo a la entrada, dst->field_70) no
|       es rederivable en C.
|    2. Handler_077c98 es codigo, no datos: los 16 primeros bytes son
|       instrucciones validas del 68000. Se registrara como funcion
|       independiente en su propio momento (probable candidato de
|       proxima ola tras completar Wave T).
|  ============================================================================

        .text
        .globl  Entity_InstallHandlerAndCopyXf
        .type   Entity_InstallHandlerAndCopyXf, @function
        .section .text.Entity_InstallHandlerAndCopyXf, "ax", @progbits

Entity_InstallHandlerAndCopyXf:
        move.l  a1, -(a7)               | +00  2f 09          push a1 original
        lea     .LHandler(pc), a1       | +02  43 fa 00 16    a1 = &Handler_077c98
        jsr     0x6fe.l                 | +06  4e b9 00 00 06 fe   Task_ChangeHandler
                                        |               (forma abs.l explicita, 6 B;
                                        |                GAS elegiria abs.w corto sin
                                        |                el sufijo .l porque $6fe cabe
                                        |                en 16-bit con signo)
        jsr     Entity_CopyTransform    | +0c  4e b9 00 05 dd 02   copia transform a6->a0
        movea.l (a7)+, a1               | +12  22 5f          pop a1 original
        move.l  a1, 0x70(a0)            | +14  21 49 00 70    dst->field_70 = a1
        rts                             | +18  4e 75
        .size   Entity_InstallHandlerAndCopyXf, .-Entity_InstallHandlerAndCopyXf

| Etiqueta PC-relativa hacia $77c98. Distancia:
|   $77c98 - ($77c80 + 2) = $16. Cabe en desplazamiento con signo 16-bit.
        .equ    .LHandler, Handler_077c98
