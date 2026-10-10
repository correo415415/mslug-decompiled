| ============================================================================
|  Metal Slug 1 - decomp/asm/sprite_invoke.s
|  ----------------------------------------------------------------------------
|  Wave S (Entity/Sprite helpers) - funcion #1
|
|  Sprite_InvokeBlit8Params  @ $05022a  (20 bytes, 279 callers)
|
|  Trampolin de 8 parametros: recibe en a2 un puntero a un bloque de
|  comandos de sprite y carga los 8 parametros (a0, a1, d0..d5) desde
|  campos consecutivos antes de saltar a la rutina de blit ($51de2).
|
|  Firma C conceptual:
|      struct SpriteCmd {
|          void  *ptr0;   // +0
|          void  *ptr1;   // +4
|          short  w0;     // +8
|          short  w1;     // +A
|          short  w2;     // +C
|          short  w3;     // +E
|          short  w4;     // +10
|          short  w5;     // +12
|      };
|      void Sprite_InvokeBlit8Params(struct SpriteCmd *a2);
|
|  Se codifica como .s (no .c) porque el llamador pasa parametros en
|  registros absolutos concretos (a2 entrada; a0/a1/d0..d5 al callee),
|  convencion incompatible con el ABI de GCC.
|
|  Toolchain:  m68k-linux-gnu-gcc -m68000 (modo bare-metal freestanding)
|              GAS m68k con --register-prefix-optional.
|  ============================================================================

        .text
        .globl  Sprite_InvokeBlit8Params
        .type   Sprite_InvokeBlit8Params, @function
        .section .text.Sprite_InvokeBlit8Params, "ax", @progbits

Sprite_InvokeBlit8Params:
        movea.l (a2), a0            | +00  0x2052
        movea.l 4(a2), a1           | +02  0x226a 0x0004
        move.w  8(a2), d0            | +06  0x302a 0x0008
        move.w  10(a2), d1           | +0a  0x322a 0x000a
        move.w  12(a2), d2           | +0e  0x342a 0x000c
        move.w  14(a2), d3           | +12  0x362a 0x000e
        move.w  16(a2), d4           | +16  0x382a 0x0010
        move.w  18(a2), d5           | +1a  0x3a2a 0x0012
        jsr     0x51de2              | +1e  0x4eb9 0x0005 0x1de2
        rts                          | +24  0x4e75
        .size   Sprite_InvokeBlit8Params, .-Sprite_InvokeBlit8Params
