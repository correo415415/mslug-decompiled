| ============================================================================
|  Metal Slug 1 - asm/entity_copy_field68_call_0517fe.s
|  ----------------------------------------------------------------------------
|  Wave V (Entity flag/probe helpers) - funcion #3
|
|  Entity_CopyField68AndCall_0517FE  @ $0517FE  (14 bytes, 4 callers)
|
|  Copia el byte $68 de la entidad src (a6) a la entidad dst (a0) y
|  delega en el backend $5CCC8 (probablemente el instalador/refrescador
|  de la propiedad asociada a ese campo; los 4 callers son wrappers
|  cortos que preparan a0 antes de llamar aqui).
|
|  Firma C conceptual:
|
|      void Entity_CopyField68AndCall(struct Entity *a0 /*dst*/,
|                                     struct Entity *a6 /*src*/);
|
|  Entrada:
|      a0 : entidad destino
|      a6 : entidad origen
|
|  Salida: ninguna (delega en Sub_0005CCC8 via jsr, y retorna con rts).
|
|  Notas forenses:
|    - move.b memory-to-memory con offsets identicos (misma constante
|      literal $68) en src y dst no es rederivable por GCC 1:1: el
|      compilador emitiria un move.b (a6),d0 + move.b d0,(a0) o
|      colapsaria el field a un load/store PC-rel con offset distinto.
|    - jsr abs.l inmediato seguido de rts es el patron canonico de
|      "thunk que hace 1 accion + delega": ya identificado en toda la
|      Wave I. La diferencia aqui es que hay una operacion no trivial
|      previa al jsr, asi que NO cabe en la familia JsrAbsThunk.
|
|  Toolchain:  m68k-linux-gnu-gcc -m68000 -Wa,--register-prefix-optional
|  ============================================================================

        .text
        .globl  Entity_CopyField68AndCall_0517FE
        .type   Entity_CopyField68AndCall_0517FE, @function
        .section .text.Entity_CopyField68AndCall_0517FE, "ax", @progbits

Entity_CopyField68AndCall_0517FE:
        move.b  0x68(a6), 0x68(a0)     | +00  dst.field68 = src.field68
        jsr     ThunkTarget_05ccc8      | +06  delega en el backend $5CCC8
        rts                             | +0c
        .size   Entity_CopyField68AndCall_0517FE, .-Entity_CopyField68AndCall_0517FE
