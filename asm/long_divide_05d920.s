| ============================================================================
|  Metal Slug 1 - asm/long_divide_05d920.s
|  ----------------------------------------------------------------------------
|  Wave X (post-allocator: HUD debug + comparadores + arranque) - funcion #4
|
|  Sub_LongDivide_05D920  @ $05D920  (36 bytes, ≥1 caller via X#3)
|
|  Divisor long/word por shift-and-subtract (32 iteraciones). Divide d0
|  por d1 devolviendo:
|      d0 = d0 / d1  (cociente, 32 bits)
|      d2 = d0 mod d1  (resto)
|
|  Es el sustituto manual del `divul.l`/`divu.l` del 68020+ (MSLUG1 corre
|  en 68000 puro donde solo existe `divu.w`/`divs.w` de 32/16 bits). Este
|  helper es necesario para el pipeline decimal display porque los
|  scores/timers exceden 16 bits (99_999_999 > 0xFFFF).
|
|  Algoritmo (shift-and-subtract clasico de division sin restauracion):
|      d2 = 0                          -- resto acumulador
|      if (d0 == 0)  goto .Lreturn     -- 0/x = 0, salir con d2 = 0
|      if (d1 == 0)  goto TRAP #15      -- x/0 -> halt sistema (funcion externa)
|      d3 = 31                         -- 32 iteraciones
|      do:
|          d0 <<= 1                    -- shift izq del cociente en construccion
|          d2 <<= 1 (con carry de d0)  -- shift izq del resto
|          if (d2 >= d1):
|              d0 |= 1                 -- bit 0 del cociente = 1
|              d2 -= d1                -- restar divisor
|      while (d3-- >= 0)
|      rts
|
|  Firma C conceptual:
|
|      /* Divide d0 por d1 (division 32/32 sin overflow). Retorna
|       * cociente en d0 y resto en d2. Si d1 == 0, hace TRAP #15 (halt
|       * del sistema) - NO retorna con valor centinela.
|       * Si d0 == 0, retorna directamente (d0=0, d2=0). */
|      uint32_t Sub_LongDivide(uint32_t dividend /*d0*/,
|                              uint32_t divisor  /*d1*/,
|                              uint32_t *remainder /*d2*/);
|
|  Descubrimiento forense CRITICO:
|      El brazo "d1 == 0" NO retorna con un valor centinela. Salta a
|      Trap15_DivByZero_05D944 (funcion externa contigua) que ejecuta
|      trap #15. El TRAP dispara el vector table del 68000 que rutea a
|      un handler de excepcion (probablemente "System Halt" o "BIOS Panic").
|      Idioma de asm defensivo hand-coded que ningun compilador GCC emite:
|      se aprovecha del hardware para senalar el error en vez de un valor
|      centinela como -1 o UINT_MAX.
|
|  Notas forenses (por que NO es rederivable por GCC 1:1):
|    1. add.l d0,d0 + addx.l d2,d2 es el patron cannonico de "shift left
|       de 64 bits" en 68000 sin instruccion nativa: d0 aporta el bit
|       nuevo desde el MSB, addx.l propaga el carry al d2. GCC habria
|       emitido __ashldi3 (libgcc runtime) con overhead de call+ret.
|    2. dbra d3, .Lloop con d3=31 hace 32 iteraciones exactas (dbra sale
|       cuando d3 == -1). Corresponde a los 32 bits del cociente.
|    3. Los dos beq.w tempranos con destinos DIFERENTES:
|         beq.w #$1C -> $5D942 = .Lreturn (rts, brazo d0==0)
|         beq.w #$18 -> $5D944 = Trap15_DivByZero (funcion externa, brazo d1==0)
|       Es la misma tecnica que hemos identificado en W#16 (tail-call
|       a funcion contigua): dos brazos con salidas distintas que aterrizan
|       en direcciones adyacentes con semantica opuesta (retorno normal
|       vs halt del sistema).
|    4. addq.w #1, d0 (2 B) en vez de addq.l (2 B tambien pero opcode
|       distinto): .w es suficiente porque el bit 0 se acumula desde
|       el bottom del long y no desborda la palabra baja en una iter.
|
|  Toolchain:  m68k-linux-gnu-gcc -m68000 -Wa,--register-prefix-optional
|  ============================================================================

        .text
        .globl  Sub_LongDivide_05D920
        .type   Sub_LongDivide_05D920, @function
        .section .text.Sub_LongDivide_05D920, "ax", @progbits

Sub_LongDivide_05D920:
        moveq   #0x0, d2                    | +00  d2 = 0 (resto)
        tst.l   d0                          | +02  ¿ dividendo = 0 ?
        beq.w   .Lreturn                    | +04  si: 0/x = 0, ir al rts propio
        tst.l   d1                          | +08  ¿ divisor = 0 ?
        beq.w   Trap15_DivByZero_05D944     | +0a  si: TRAP #15 (halt sistema, extern)
        moveq   #0x1f, d3                   | +0e  d3 = 31 (32 iteraciones)
.Lloop:
        add.l   d0, d0                      | +10  d0 <<= 1
        addx.l  d2, d2                      | +12  d2 <<= 1, bit desde d0
        cmp.l   d1, d2                      | +14  ¿ d2 >= d1 ?
        bcs.w   .Lskip                      | +16  no: skip
        addq.w  #0x1, d0                    | +1a  si: bit 0 del cociente = 1
        sub.l   d1, d2                      | +1c  d2 -= d1
.Lskip:
        dbra    d3, .Lloop                  | +1e  d3--; loop mientras d3 >= 0
.Lreturn:
        rts                                  | +22
        .size   Sub_LongDivide_05D920, .-Sub_LongDivide_05D920
