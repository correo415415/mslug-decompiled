| ============================================================================
|  Metal Slug 1 - asm/camera_reset_center_05cace.s
|  ----------------------------------------------------------------------------
|  Wave W (Sprite slot allocator) - funcion #14
|
|  Camera_ResetCenter_05CACE  @ $05CACE  (16 bytes, ? callers)
|
|  Resetea el centro de la camara a los valores por defecto de MSLUG1
|  (X=$A0=160, Y=$178=376). Estas coordenadas coinciden con el centro
|  del "safe area" del Neo Geo (320x224 con 40 columnas x 28 filas de
|  tiles del Fix Layer, aunque las coords absolutas de $10E1E4/$10E1E6
|  usan una escala interna diferente).
|
|  Firma C conceptual:
|
|      /* Resetea camera_x = 0xA0 (160) y camera_y = 0x178 (376). */
|      void Camera_ResetCenter(void);
|
|  Notas forenses:
|    1. Dos `move.w #imm, abs.l` seguidos (10 B cada uno) con misma
|       codificacion opcode 33fc para escrituras absolutas de word.
|       GCC habria consolidado en `move.l #<64bit>,$10E1E4.l` si los
|       valores fueran contiguos, pero $10E1E4 y $10E1E6 son adyacentes
|       (word offset), asi que se podria emitir como move.l tambien.
|       El original emite 2 x move.w porque el hardware puede reordenar
|       writes de 32 bit (aunque en 68000 no habria diferencia).
|    2. El helper no depende de estado - es un initializer puro.
|
|  Toolchain:  m68k-linux-gnu-gcc -m68000 -Wa,--register-prefix-optional
|  ============================================================================

        .text
        .globl  Camera_ResetCenter_05CACE
        .type   Camera_ResetCenter_05CACE, @function
        .section .text.Camera_ResetCenter_05CACE, "ax", @progbits

Camera_ResetCenter_05CACE:
        move.w  #0xa0, 0x10e1e4.l           | +00  camera_x = 160
        move.w  #0x178, 0x10e1e6.l          | +08  camera_y = 376
        rts                                  | +10
        .size   Camera_ResetCenter_05CACE, .-Camera_ResetCenter_05CACE
