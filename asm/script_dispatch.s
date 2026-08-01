| ============================================================================
|  Metal Slug 1 - asm/script_dispatch.s
|  ----------------------------------------------------------------------------
|  Wave T (Script / Draw / Task primitives) - funcion #1
|
|  Script_DispatchOpcode  @ $028d8e  (70 bytes)
|
|  Despachador central del intérprete de scripts de entidad. Es el
|  destino directo del bne.w final de Entity_HasLinkedSlots ($028d70):
|  cuando la entidad tiene un slot_parent activo, la ejecución cae
|  literalmente en el primer byte de esta función (no hay padding entre
|  ambas, ni jsr, ni jmp - fall-through puro).
|
|  Flujo:
|      1. Compara slot_parent ($3c) con slot_child ($40).
|         Si son iguales, salta al bloque de lectura de opcode.
|      2. Si difieren, incrementa el contador de "ticks" $3b(a6).
|         Si tras el incremento el byte pasa por cero (overflow $FF->$00),
|         también salta al bloque de opcode. En otro caso, guarda el
|         nuevo tick y continúa.
|      3. Lee el opcode: d2 = *a1 (donde a1 = slot_parent).
|         Si opcode >= $20, dispara trap #15 (assertion de rango de
|         opcode, con tres nops de padding manual y un cmpi.b duplicado
|         entre medias - firma inequívoca de ensamblador escrito a mano).
|      4. Multiplica opcode * 4 y salta a  jump_table[opcode]  donde
|         jump_table = $28CF0 (tabla de 32 handlers de 4 bytes).
|
|  Entrada (registros absolutos, convención no-C):
|      a6 : entidad actual
|      a1 : puntero al bytecode (heredado del código que cae al fall-through:
|           en Entity_HasLinkedSlots ese a1 quedó cargado con $3c(a6))
|
|  Salida: jmp indirecto al handler seleccionado (no vuelve por rts).
|
|  Firma C conceptual (no reproducible por GCC 1:1):
|      void Script_DispatchOpcode(struct Entity *a6);
|      // Efectos: a2 = &script_jump_table (=$28CF0),
|      //         a3 = script_jump_table[opcode], y salta a (a3).
|
|  Hallazgos forenses (asm a mano):
|    1. Fall-through desde función anterior sin padding ni jmp.
|    2. cmpi.b #$20,d2 aparece DOS VECES ($028dac y $028db8) con tres
|       nop $4E71 entre medias. Un compilador jamás emite esta secuencia.
|    3. trap #15 como assertion fatal de opcode fuera de rango.
|    4. movea.l #$28cf0,a2 con dirección absoluta empotrada (los C-thunks
|       del proyecto usarían jsr $XXXXX.l, no movea.l #imm).
|
|  Referencias posteriores para completar el mapa:
|    - $028CF0: tabla de 32 punteros a handlers de opcode (128 bytes).
|      Cubrirá el rango justo antes de $028D8E, será una entrada de
|      "data" en decomp/asm/data/ cuando se aborde.
|  ============================================================================

        .text
        .globl  Script_DispatchOpcode
        .type   Script_DispatchOpcode, @function
        .section .text.Script_DispatchOpcode, "ax", @progbits

Script_DispatchOpcode:
        movea.l 0x3c(a6), a1            | +00  22 6e 00 3c   a1 = slot_parent
        cmpa.l  0x40(a6), a1            | +04  b3 ee 00 40   a1 == slot_child ?
        beq.w   .Lread_opcode           | +08  67 00 00 10   si iguales, leer opcode
        move.b  0x3b(a6), d1            | +0c  12 2e 00 3b   d1 = tick counter
        addq.b  #1, d1                  | +10  52 01         d1++ (byte)
        beq.w   .Lread_opcode           | +12  67 00 00 06   si overflow $FF->$00, opcode
        move.b  d1, 0x3b(a6)            | +16  1d 41 00 3b   guarda tick
.Lread_opcode:
        moveq   #0, d2                  | +1a  74 00         d2 = 0
        move.b  (a1), d2                | +1c  14 11         d2 = *slot_parent
        cmpi.b  #0x20, d2               | +1e  0c 02 00 20   opcode < $20 ?
        bcs.w   .Ldo_dispatch           | +22  65 00 00 0e   si, saltar assertion
        nop                             | +26  4e 71         padding manual
        nop                             | +28  4e 71         padding manual
        cmpi.b  #0x20, d2               | +2a  0c 02 00 20   re-check (deliberado)
        nop                             | +2e  4e 71         padding manual
        trap    #15                     | +30  4e 4f         ASSERTION: opcode >= $20
.Ldo_dispatch:
        andi.l  #0xff, d2               | +32  02 82 00 00 00 ff   d2 &= $FF (byte)
        lsl.l   #2, d2                  | +38  e5 8a         d2 *= 4 (index de 32-bit)
        movea.l #0x28cf0, a2            | +3a  24 7c 00 02 8c f0   a2 = &script_jump_table
        movea.l (a2, d2.w), a3          | +40  26 72 20 00   a3 = table[opcode]
        jmp     (a3)                    | +44  4e d3         salto al handler
        .size   Script_DispatchOpcode, .-Script_DispatchOpcode
