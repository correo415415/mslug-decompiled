| ============================================================================
|  Metal Slug 1 - asm/rng_lfsr_step_05e9e4.s
|  ----------------------------------------------------------------------------
|  Wave Z - #9  (GENERADOR ALEATORIO PRINCIPAL DEL JUEGO)
|
|  RNG_LFSRStep_05E9E4  @ $05E9E4  (56 bytes, callers multiples via wrappers)
|
|  Generador de numeros aleatorios estilo LFSR con tap Fibonacci sobre un
|  ring buffer de 32 words en $10E230 (buffer). Head del ring en $10E270
|  como indice byte-offset dentro del buffer (0..$3E paso 2).
|
|  Algoritmo:
|    1. head = ($10E270 + 2) & $3E                   avanza head 2 bytes
|    2. w0   = buffer[head]                          lee word actual
|    3. w0   = rol.w #1, w0                          rotacion 1-bit
|    4. tap  = (head - $15) & $3E                    tap Fibonacci -21 mod 64
|    5. buffer[tap] ^= w0                            XOR feedback
|    6. w0   = buffer[tap]                           re-lee post-XOR
|    7. producto = w0 * d5                           mulu.w d5, d0
|    8. retorna swap(producto) en d0 (high word)     dispersion post-mult
|
|  Los taps -1 y -21 con mask $3F sobre buffer de 32 words son consistentes
|  con un LFSR Fibonacci de periodo largo. El multiplicador d5 introducido
|  por el caller actua como salt (rareza para el juego: RNG parametrizado).
|
|  Firma C conceptual:
|
|      /* Avanza el estado del LFSR interno una vez y devuelve un word
|       * pseudo-aleatorio derivado del multiplicador d5. Estado global
|       * en $10E230 (32 words) + $10E270 (head, word offset dentro del
|       * buffer, paso 2, mask $3E). */
|      uint16 RNG_LFSRStep(uint16 salt /*d5*/);
|
|  Notas forenses (por que NO es rederivable por GCC 1:1):
|    1. `mulu.w d5, d0; swap d0` extrae la parte alta del producto de dos
|       words - idioma clasico de RNG multiplicativo. GCC habria emitido
|       `mulu.l` + shift si el objetivo fuera 32-bit puro.
|    2. `rol.w #1, w` (`e3 58`) rota 1 bit por instruccion de 2 B, con la
|       clave de que preserva el bit desplazado en el propio word. Es la
|       forma canonica hand-coded de "advance-then-feed" del LFSR.
|    3. Constante `-$15` (-21 signed) como tap Fibonacci sobre 32 words es
|       un polinomio primitivo x^32 + x^21 + 1 conocido en literatura de
|       RNG - el juego usa un LFSR estandar de la epoca.
|    4. Los offsets $10E230 (buffer) y $10E270 (head) son ADYACENTES en RAM,
|       sugiriendo layout `struct { uint16 buffer[32]; uint16 head; }` en
|       $10E230..$10E270.
|
|  Toolchain:  m68k-linux-gnu-gcc -m68000 -Wa,--register-prefix-optional
|  ============================================================================

        .text
        .globl  RNG_LFSRStep_05E9E4
        .type   RNG_LFSRStep_05E9E4, @function
        .section .text.RNG_LFSRStep_05E9E4, "ax", @progbits

RNG_LFSRStep_05E9E4:
        move.w  d0, d5                         | +00  d5 = salt (moved from d0 arg)
        lea.l   0x10e230.l, a4                 | +02  a4 = &lfsr_buffer[0]  (32 words)
        move.w  0x10e270.l, d7                 | +08  d7 = head (byte-offset in buffer)
        addi.w  #0x2, d7                       | +0e  d7 += 2  (advance one word)
        move.w  #0x3e, d6                      | +12  d6 = mask ($3E = 62 = 31*2)
        and.w   d6, d7                         | +16  d7 &= mask (wrap in 32 words)
        move.w  d7, 0x10e270.l                 | +18  publish head
        move.w  (a4, d7.w), d0                 | +1e  d0 = buffer[head]
        rol.w   #0x1, d0                       | +22  d0 = rotl(d0, 1)
        subi.w  #0x15, d7                      | +24  d7 = head - 21  (tap Fibonacci)
        and.w   d6, d7                         | +28  d7 &= mask (wrap)
        eor.w   d0, (a4, d7.w)                 | +2a  buffer[tap] ^= rotated
        move.w  (a4, d7.w), d0                 | +2e  d0 = buffer[tap] (post-XOR)
        mulu.w  d5, d0                         | +32  d0 = d0 * salt (unsigned)
        swap    d0                             | +34  d0 = high word of product
        rts                                    | +36

        .size   RNG_LFSRStep_05E9E4, .-RNG_LFSRStep_05E9E4
