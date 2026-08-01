| ============================================================================
|  Metal Slug 1 - asm/init_entity_spawn_0018da.s
|  ----------------------------------------------------------------------------
|  Wave EE batch 1 - #2
|
|  Init_EntitySpawn_0018DA  @ $0018DA  (72 bytes)
|
|  Handler mini-script. Codificado en la tabla de descriptores $000BA2 como
|  puntero long de "entrada tipo 2" (junto con $001846 / $001A70 / $001A64
|  en su fila). Sirve para arrancar la fase que engancha tres task handlers
|  encadenados (uno de datos "$28DB6A", uno de logica "$29588", uno de
|  entidad activa "$469E2") y publica el timer master $3C en $44(a6).
|
|  Firma C conceptual:
|
|      /* Handler invocado desde el scheduler ($100080 arena) con a6
|       * apuntando al task node activo. Ejecuta la subrutina de probe
|       * $1DB8, arma timer_a = $3C, y llama tres veces a
|       * scheduler_add($4AE) con distintos handlers. Al terminar, el
|       * ultimo scheduler_add() deja a0 apuntando al nuevo task node y
|       * este handler publica $0D en el campo $98 de ese nodo, resetea
|       * el flag global $1081B1 a 0, lanza el init pesado $2A24A y hace
|       * tail al scheduler ($FE0). */
|      void Init_EntitySpawn(void);
|
|  Handlers encadenados (en orden de invocacion):
|      $28DB6A -> TaskHandler_00028DB6A    (loader de banco tabla de datos)
|      $029588 -> TaskHandler_00029588     (dispatcher de script logico)
|      $046982 -> TaskHandler_000469E2     (entidad activa: HUD y capa fija)
|
|  Globales tocadas:
|      $44(a6)   = timer_a del task actual := $3C
|      $98(a0)   = campo $98 del task recien creado := $0D
|      $1081B1   = flag global (limpiado a 0)
|
|  Notas forenses (por que NO es rederivable por GCC 1:1):
|    1. Los tres pares `lea.l imm.l, a1; jsr $4ae.l` usan la forma
|       absoluta larga (6+6 = 12 B por llamada) cuando la forma PC-rel
|       daria 4+6 = 10 B. El compilador elegiria PC-rel si -mpcrel esta
|       activo; el codigo original usa absoluta larga sistematicamente
|       para toda la fila del cluster, delatando codigo escrito a mano
|       (probablemente proviene de una tabla de templates cargada por
|       macro de ensamblador).
|    2. El `move.b #$0D, $98(a0)` presume que scheduler_add() ($4AE)
|       deja el puntero al nuevo task node en a0. Esta convencion no es
|       parte del ABI de GCC - GCC habria hecho `movea.l returnval, a0`
|       primero. Aqui se usa como side-effect de $4AE, un patron muy
|       comun en el proyecto (Wave DD Task_FreeListInit ya lo detecto).
|    3. `jsr $2A24A.l` es un tail-call disfrazado: el `bra.w $FE0` que
|       le sigue nunca vuelve al rts propio (la funcion no tiene rts).
|       GCC habria emitido directamente `jmp $2A24A.l` seguido de
|       `bra.w $FE0` inalcanzable.
|
|  Toolchain:  m68k-linux-gnu-gcc -m68000 -Wa,--register-prefix-optional
|  ============================================================================

        .text
        .globl  Init_EntitySpawn_0018DA
        .type   Init_EntitySpawn_0018DA, @function
        .section .text.Init_EntitySpawn_0018DA, "ax", @progbits

Init_EntitySpawn_0018DA:
                                              | +00  primer word es datos "raw" del
                                              |      opcode extended: 00046ac6 se
                                              |      interpreta como ori.b #$c6,d4
                                              |      SOLO si se entra por linear
                                              |      disasm. La entrada real es el
                                              |      bsr.w en $18DE (ver arriba).
                                              |      Emitimos los 4 bytes literales.
        .byte   0x00, 0x04, 0x6a, 0xc6         | +00  literal: 0004 6AC6

        bsr.w   Sub_00001DB8                   | +04  Sub_00001DB8() (probe/setup)
        move.b  #0x3c, 0x44(a6)                | +08  timer_a = $3C
        lea.l   0x28db6a.l, a1                 | +0e  a1 = TaskHandler_00028DB6A
        jsr     0x4ae.l                        | +14  scheduler_add(a1)
        lea.l   0x29588.l, a1                  | +1a  a1 = TaskHandler_00029588
        jsr     0x4ae.l                        | +20  scheduler_add(a1)
        lea.l   0x469e2.l, a1                  | +26  a1 = TaskHandler_000469E2
        jsr     0x4ae.l                        | +2c  scheduler_add(a1)  -> a0 = new task
        move.b  #0xd, 0x98(a0)                 | +32  new_task->field_98 = $0D
        clr.b   0x1081b1.l                     | +38  publica flag global = 0
        jsr     0x2a24a.l                      | +3e  Sub_0002A24A (init pesado)
        bra.w   Sub_00000FE0                   | +44  tail al scheduler (no rts)

        .size   Init_EntitySpawn_0018DA, .-Init_EntitySpawn_0018DA
