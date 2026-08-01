| ============================================================================
|  Metal Slug 1 - asm/camera_sprite_helpers_wave_kk.s
|  ----------------------------------------------------------------------------
|  Wave KK batch 1 - 4 callees pendientes de Wave JJ (camara + sprites).
|
|  Contenido (4 funciones, 266 bytes):
|
|      $043E8C   BlitterTile_2D_043E8C            78 B  blit 2D tile buffer
|      $051B80   Integrator_XY_051B80             40 B  integrador 2D con hi_swap
|      $051F30   TransformCommit_MMIO_051F30     100 B  commit + jsr scratch
|      $05A88A   SpriteSubsystem_Reset_05A88A     48 B  reset lista sprites
|
|  Cierra 4 de los 5 callees pendientes documentados al final de Wave JJ:
|      $043E8C  <- CameraHook_Probe08/82/F6_043DF4 (tail-jump comun)
|      $051B80  <- CameraApplyOne_043DAA          (Transform_Publish)
|      $051F30  <- CameraApplyOne_043DAA          (Transform_Commit)
|      $05A88A  <- Scratch_Alloc_01390E           (SpriteSys_Reset)
|
|  ---------- BlitterTile_2D_043E8C ------------------------------------------
|
|  Es el procesador al que los TRES hooks de camara (JJ#1) hacen tail-jump
|  incondicional (`bra.w $43E8C`), no un simple callee. La convergencia de
|  los tres hooks en este punto identifica esta funcion como el CORE del
|  blit de un tile de camara: aplica offset por la posicion del sistema,
|  calcula direcciones en el buffer de camara ($7C(a0)..) y copia el tile
|  desde el ptr en a1.
|
|  Semantica reconstruida a partir del flujo de registros:
|
|      /* Blitea un tile 2D en el buffer del sistema de camara.
|       *   Entrada:
|       *     a0  = camera system base (offset $7C = buffer local)
|       *     a1  = ptr al tile data (words a copiar)
|       *     d0  = X (aditivo con $22(a0))
|       *     d1  = Y (aditivo con $24(a0))
|       *     d2  = pos inicial en row  (X inicial)
|       *     d3  = incremento de row   (X step)
|       *     d4  = numero de filas
|       *     d5  = numero de cols por fila
|       *     d6  = tile width (multiplicador de row start) */
|      void BlitterTile_2D(void);
|
|  Estructura:
|    1. Precompute address: aplica offset del sistema (d0+=$22, d1+=$24)
|       y calcula el punto inicial en a1 via `d2*d6 + d3, d2*4`.
|    2. Setup: a2 = &buffer_local, ajustes de shift para indexar por row.
|    3. Doble bucle:
|         outer:  d0 -> a3 = &buffer[row]     (indice por lsl.w #$7)
|         inner:  d3 = X in row (mask $7C), copia (a1)+ -> (a3, d3)
|                 d3 += 4, dbra d2
|         a1 += d6*4 (siguiente row en fuente)
|         d0 += $80  (siguiente row en destino)
|
|  Callers conocidos: **3** (los tres hooks de camara de JJ#1 hacen
|  `bra.w $43E8C` incondicional, no `jsr` — es el "cuerpo compartido" al
|  que convergen los tres cuando el probe pasa y el enlace no es NULL).
|
|  Por que NO es rederivable por GCC 1:1:
|    1. Cascada de 4 `add.w dX, dX` consecutivos ($043E9A..$043EA0) para
|       hacer `d2 <<= 2` y `d6 <<= 2` intercalados. GCC habria emitido
|       `lsl.w #2, d2` + `lsl.w #2, d6` (dos instrucciones) o mantenido
|       los operandos en registros distintos. Aqui se prefiere reutilizar
|       el mismo par de instrucciones aritmeticas.
|    2. `lea.l (a1, d2.w), a1` + `lea.l $7C(a0), a2` con desplazamientos
|       simetricos: el mismo idioma que ya vimos en HH#1 (SceneLoader) y
|       II#2 (Reset4CameraLongs) para publicar bases del buffer local.
|    3. `andi.w #$F80, d0` seguido de `andi.w #$7C, d3` en el doble bucle:
|       mascaras de bits que limitan Y a $F80 (32 rows * $80 stride) y X
|       a $7C (32 cols * 4 bytes). GCC habria usado shifts en lugar de
|       mascaras porque no conoce la naturaleza modular del tile-map.
|    4. `dbra d2, .Linner` seguido de `dbra d4, .Louter` sin epilogo
|       explicito entre ambos: el reset del inner (`move.w d5, d2`) se
|       hace al inicio del outer.
|
|  ---------- Integrator_XY_051B80 -------------------------------------------
|
|  Integrador de coordenadas 2D del subsistema de sprite dinamico. Toma
|  incrementos long (d0, d1) y los suma a los acumuladores en el struct
|  apuntado por a0. Publica ademas el WORD ALTO (hi_word) resultante en el
|  campo word visible a los layers de dibujo, calculandolo como
|  `hi_word_new - hi_word_old` — el delta de la coordenada tras la suma.
|
|      /* Integra (d0, d1) en los acumuladores long del sprite y publica
|       * el hi_word como delta visible. Usado por CameraApplyOne (JJ#1)
|       * como Transform_Publish tras escalar hud_x/hud_y por scale_x/y.
|       *   a0  = struct sprite       (long acumulador X en $4, Y en $8;
|       *                              word visible X en (a0), Y en $2)
|       *   d0  = delta X en 8.24 fixed (long)
|       *   d1  = delta Y en 8.24 fixed (long)  */
|      void Integrator_XY(void);
|
|  Estructura del struct sprite inferida (coherente con SceneLoader HH#1):
|      $00  u16   x_visible   (hi_word del acumulador X, para blit)
|      $02  u16   y_visible   (hi_word del acumulador Y, para blit)
|      $04  u32   x_accum     (acumulador X en 16.16 fixed point)
|      $08  u32   y_accum     (acumulador Y en 16.16 fixed point)
|
|  Callers conocidos: **1+** (`CameraApplyOne_043DAA` de JJ#1 con
|  `jsr Fn_00051B80` tras escalar hud_x/hud_y por scale_x/scale_y del
|  sistema de camara).
|
|  Por que NO es rederivable por GCC 1:1:
|    1. `swap d0 / swap d2 / sub.w d2, d0 / move.w d0, (a0)` es el idioma
|       "extraer hi_word y publicar delta" hand-coded: obtiene el word
|       alto de d0 (nuevo acumulador) y d2 (viejo), calcula la diferencia
|       en una word en d0 y la escribe. GCC habria emitido shifts (`asr.l
|       #16`) o accesos por miembro word directo, no una cadena de swaps.
|    2. La secuencia se repite DOS VECES sin factorizar (una para X, otra
|       para Y). El asm hand-coded prefiere copiar el bloque, GCC habria
|       inlinado un helper o emitido un bucle.
|    3. Estructura sub/swap simetrica es identica a la ya documentada en
|       Camera_SmoothingIntegrate (HH#1, $0434F8) — pertenece a la misma
|       familia de integradores del engine.
|
|  ---------- TransformCommit_MMIO_051F30 ------------------------------------
|
|  Consolida la transformacion aplicada por CameraApplyOne y la escribe al
|  hardware Neo Geo. La estructura tiene un gate por bit 0 de $C(a0) que
|  permite saltar la operacion (rama corta 8 B). Cuando el bit esta armado:
|
|    1. Calcula 4 valores intermedios ($2A, $2C, $2E, $30 del struct):
|         $2A = ((x_vis + tile_x*16) >> 4) - 1     [mod 32]
|         $2C = $2A + $16                          [mod 32]
|         $2E = -(x_vis + tile_x*16) << 7          [hardware Y offset]
|         $30 = (y_vis + tile_y*16) << 7           [hardware X offset]
|    2. Prepara a0 en a6 (movem preservado) y salta a un procesador
|       generico ($1F4A) con a0 = &handler_local ($51F94). El handler
|       inline recorre 32 celdas del tile-map ($8201.., $F0F.., ...)
|       y publica cada una via el MMIO $3C0000/$3C0002.
|    3. Restaura a0/a6 y retorna.
|
|      /* Confirma la transformacion pendiente escribiendo al MMIO del
|       * sprite hardware. No-op si el bit sticky del sprite esta apagado.
|       *   a0  = struct sprite (mismo que Integrator_XY_051B80) */
|      void TransformCommit_MMIO(void);
|
|  Callers conocidos: **1+** (`CameraApplyOne_043DAA` de JJ#1 con
|  `jsr ThunkTarget_051f30`, ya expuesto como Wave I).
|
|  El handler inline `$051F94` NO se incluye en este batch: es una funcion
|  distinta (aunque adyacente) que se decompilara en Wave KK batch 2 junto
|  con los tres probes grandes de camara ($051C08/$51C82/$51CF6).
|
|  Por que NO es rederivable por GCC 1:1:
|    1. `btst.b #0, $c(a0) / bne.w / rts` es una guarda con salida corta
|       de solo 2 B tras el `bne.w`. GCC habria emitido `beq.b .Ldone` +
|       cuerpo + `rts` (misma logica invertida), sin la rama corta al rts.
|    2. `movea.l a0, a6 / lea.l $51F94(pc), a0 / jsr $1F4A.l / movea.l a6,
|       a0` es un patron "call by continuation": el handler que sigue en
|       $51F94 se pasa como CODIGO a un dispatcher $1F4A (que no sabemos
|       aun que hace pero probablemente ejecuta el handler N veces con
|       distinto indice). GCC no genera este patron: usaria un puntero a
|       funcion pasado por registro convencional.
|    3. `movem.l a0/a6, -(a7)` para preservar el par completo sobre la
|       llamada indirecta: idioma "save-and-swap frames".
|    4. Multiplicacion por 16 via `lsl.w #4` seguida de `lsl.w #7` para
|       llegar a `<< 11` en dos pasos: GCC habria emitido `lsl.w #11` de
|       una vez. La separacion en dos shifts sugiere macros de tile-map
|       independientes (`TILE_TO_PIXEL` seguido de `PIXEL_TO_HW`).
|
|  ---------- SpriteSubsystem_Reset_05A88A -----------------------------------
|
|  Reset del subsistema de sprites dinamicos. Limpia los 3 slots de estado
|  en $10E1EC..$10E1EA (contadores/flags) y desactiva dos listas enlazadas
|  publicando el centinela $FFFF en su cabezal:
|
|      $10E1EC + $616C = $114E1F0    counter_flag  (byte)
|      $10E1EC + $6168 = $114E1EC    counter_word  (word)
|      $10E1EC + $616A = $114E1EE    counter_word  (word)
|      $10E1EC + $614C = $114E1D0    list_head_A   ($8 = flag, $4 = next=$FFFF)
|      $10E1EC + $6158 = $114E1DC    list_head_B   (idem)
|
|  Los offsets con la base $108080 apuntan a la zona $108080+$616x =
|  ~$10E1EX (tabla de contexto de sprite del engine). Los dos "list heads"
|  contienen un flag activo ($8) y un puntero al siguiente elemento ($4).
|  Publicar $FFFF en el next-ptr equivale a "lista vacia" en la convencion
|  del scheduler ya documentada en Wave S.
|
|      /* Resetea el subsistema de sprites: cero contadores + listas vacias. */
|      void SpriteSubsystem_Reset(void);
|
|  Callers conocidos: **1** (`Scratch_Alloc_01390E` de JJ#2, primera
|  instruccion `jsr Fn_0005A88A`).
|
|  Por que NO es rederivable por GCC 1:1:
|    1. `lea.l $108080.l, a5` para cargar una BASE COMUN, seguida de accesos
|       por desplazamiento word grande ($6168..$616C). GCC habria usado
|       direcciones absolutas largas directas en cada `clr.w abs.l` (2 B
|       mas por acceso pero sin mantener el a5 vivo). La eleccion sugiere
|       que a5 se preserva entre funciones del cluster de sprites.
|    2. Segundo cambio de base a `lea.l $614C(a5), a0` para dos escrituras
|       CONSECUTIVAS en el mismo slot, y luego otro `lea.l $6158(a5), a0`
|       para el segundo slot con la misma idea. GCC habria emitido dos
|       escrituras absolutas independientes o mantenido a0 con `adda.w`.
|       Aqui vemos el patron "preparar puntero, escribir dos campos,
|       cambiar puntero, escribir dos campos" que expone la existencia de
|       un helper `RESET_LIST_HEAD a5, offset` en el fuente original.
|    3. `clr.b $8(a0) / move.w #$FFFF, $4(a0)` en ORDEN INVERTIDO respecto
|       al layout struct (primero clr byte $8, despues move word $4). GCC
|       habria seguido el orden ascendente de los offsets.
|
|  Toolchain:  m68k-linux-gnu-gcc -m68000 -Wa,--register-prefix-optional
|  ============================================================================

        .text

|
| ---------------------------------------------------------------------------
|  BlitterTile_2D_043E8C  @ $043E8C  (78 bytes)
|  Target de tail-jump de los 3 hooks de camara (JJ#1).
| ---------------------------------------------------------------------------
|
        .globl  BlitterTile_2D_043E8C
        .type   BlitterTile_2D_043E8C, @function
        .section .text.BlitterTile_2D_043E8C, "ax", @progbits
BlitterTile_2D_043E8C:
        add.w   0x22(a0), d0                    | +00  d0 += cam.pos_x
        add.w   0x24(a0), d1                    | +04  d1 += cam.pos_y
        mulu.w  d6, d2                          | +08  d2 = row_start * tile_w
        add.w   d3, d2                          | +0a  d2 += row_step
        sub.w   d5, d6                          | +0c  d6 = tile_w - cols
        add.w   d2, d2                          | +0e  d2 <<= 1  (byte->word)
        add.w   d6, d6                          | +10  d6 <<= 1
        add.w   d2, d2                          | +12  d2 <<= 2  (word->long)
        add.w   d6, d6                          | +14  d6 <<= 2
        lea.l   (a1, d2.w), a1                  | +16  a1 = &tile[row_start*4]
        lea.l   0x7c(a0), a2                    | +1a  a2 = &cam.buffer_local
        lsl.w   #0x7, d0                        | +1e  d0 <<= 7  (row idx)
        lsl.w   #0x2, d1                        | +20  d1 <<= 2  (col long idx)
        subq.w  #0x1, d4                        | +22  --d4  (dbra)
        subq.w  #0x1, d5                        | +24  --d5  (dbra)
.Louter:                                        | $043EB2
        move.w  d5, d2                          | +26  d2 = cols per row
        move.w  d1, d3                          | +28  d3 = col base
        andi.w  #0xf80, d0                      | +2a  d0 = row idx mod 32
        lea.l   (a2, d0.w), a3                  | +2e  a3 = &row[d0]
.Linner:                                        | $043EBE
        andi.w  #0x7c, d3                       | +32  d3 = col idx mod 32
        move.l  (a1)+, (a3, d3.w)               | +36  row[d3] = *tile++
        addq.w  #0x4, d3                        | +3a  d3 += 4
        dbra    d2, .Linner                     | +3c  loop cols
        lea.l   (a1, d6.w), a1                  | +40  a1 += (tile_w-cols)*4
        addi.w  #0x80, d0                       | +44  d0 += next row
        dbra    d4, .Louter                     | +48  loop rows
        rts                                     | +4c

        .size   BlitterTile_2D_043E8C, .-BlitterTile_2D_043E8C

|
| ---------------------------------------------------------------------------
|  Integrator_XY_051B80  @ $051B80  (40 bytes)
|  Callee de CameraApplyOne_043DAA (JJ#1) como Transform_Publish.
| ---------------------------------------------------------------------------
|
        .globl  Integrator_XY_051B80
        .type   Integrator_XY_051B80, @function
        .section .text.Integrator_XY_051B80, "ax", @progbits
Integrator_XY_051B80:
        move.l  0x4(a0), d2                     | +00  d2 = old_x_accum
        add.l   d2, d0                          | +04  d0 = new_x_accum
        move.l  d0, 0x4(a0)                     | +06  store new_x_accum
        swap    d0                              | +0a  d0 hi = new_x_hi
        swap    d2                              | +0c  d2 hi = old_x_hi
        sub.w   d2, d0                          | +0e  d0 = delta hi word
        move.w  d0, (a0)                        | +10  store x_visible
        move.l  0x8(a0), d2                     | +12  d2 = old_y_accum
        add.l   d2, d1                          | +16  d1 = new_y_accum
        move.l  d1, 0x8(a0)                     | +18  store new_y_accum
        swap    d1                              | +1c  d1 hi = new_y_hi
        swap    d2                              | +1e  d2 hi = old_y_hi
        sub.w   d2, d1                          | +20  d1 = delta hi word
        move.w  d1, 0x2(a0)                     | +22  store y_visible
        rts                                     | +26

        .size   Integrator_XY_051B80, .-Integrator_XY_051B80

|
| ---------------------------------------------------------------------------
|  TransformCommit_MMIO_051F30  @ $051F30  (100 bytes)
|  Callee de CameraApplyOne_043DAA (JJ#1) como Transform_Commit.
|  El handler inline en $051F94 se cerrara en Wave KK batch 2.
| ---------------------------------------------------------------------------
|
        .globl  TransformCommit_MMIO_051F30
        .type   TransformCommit_MMIO_051F30, @function
        .section .text.TransformCommit_MMIO_051F30, "ax", @progbits
TransformCommit_MMIO_051F30:
        btst.b  #0x0, 0xc(a0)                   | +00  if (!sprite.active_bit)
        bne.w   .Ldo_commit                     | +06
        rts                                     | +0a  no-op
.Ldo_commit:                                    | $051F3C
        move.w  0x4(a0), d0                     | +0c  d0 = x_accum lo
        move.w  0x22(a0), d1                    | +10  d1 = tile_x
        lsl.w   #0x4, d1                        | +14  d1 <<= 4  (tile->pixel)
        add.w   d1, d0                          | +16  d0 = x_pos
        move.w  d0, d1                          | +18  d1 = x_pos
        lsr.w   #0x4, d1                        | +1a  d1 >>= 4
        subq.b  #0x1, d1                        | +1c  --d1
        andi.w  #0x1f, d1                       | +1e  d1 mod 32
        move.w  d1, 0x2a(a0)                    | +22  cam.tile_row_start = d1
        addi.w  #0x16, d1                       | +26  d1 += 22
        andi.w  #0x1f, d1                       | +2a  d1 mod 32
        move.w  d1, 0x2c(a0)                    | +2e  cam.tile_row_end = d1
        neg.w   d0                              | +32  d0 = -x_pos
        lsl.w   #0x7, d0                        | +34  d0 <<= 7  (pixel->hw)
        move.w  d0, 0x2e(a0)                    | +36  cam.hw_x_off = d0
        move.w  0x8(a0), d0                     | +3a  d0 = y_accum lo
        move.w  0x24(a0), d1                    | +3e  d1 = tile_y
        lsl.w   #0x4, d1                        | +42  d1 <<= 4
        add.w   d1, d0                          | +44  d0 = y_pos
        lsl.w   #0x7, d0                        | +46  d0 <<= 7
        move.w  d0, 0x30(a0)                    | +48  cam.hw_y_off = d0
        movem.l a0/a6, -(a7)                    | +4c  save frame ptrs
        movea.l a0, a6                          | +50  a6 = struct sprite
        lea.l   TileMap_HandlerInline_051F94(pc), a0             | +52  a0 = &handler_inline
        jsr     Fn_00001F4A                     | +56  Scratch_RunHandler(a0)
        movea.l a6, a0                          | +5c  restore a0
        movem.l (a7)+, a0/a6                    | +5e  restore frame ptrs
        rts                                     | +62

        .size   TransformCommit_MMIO_051F30, .-TransformCommit_MMIO_051F30

|
| ---------------------------------------------------------------------------
|  SpriteSubsystem_Reset_05A88A  @ $05A88A  (48 bytes)
|  Callee de Scratch_Alloc_01390E (JJ#2). Reset del contexto de sprites.
| ---------------------------------------------------------------------------
|
        .globl  SpriteSubsystem_Reset_05A88A
        .type   SpriteSubsystem_Reset_05A88A, @function
        .section .text.SpriteSubsystem_Reset_05A88A, "ax", @progbits
SpriteSubsystem_Reset_05A88A:
        lea.l   0x108080.l, a5                  | +00  a5 = &sprite_ctx_base
        clr.b   0x616c(a5)                      | +06  ctx.counter_flag = 0
        clr.w   0x6168(a5)                      | +0a  ctx.counter_w0   = 0
        clr.w   0x616a(a5)                      | +0e  ctx.counter_w1   = 0
        lea.l   0x614c(a5), a0                  | +12  a0 = &list_head_A
        clr.b   0x8(a0)                         | +18  head_A.flag = 0
        move.w  #0xffff, 0x4(a0)                | +1c  head_A.next = $FFFF
        lea.l   0x6158(a5), a0                  | +22  a0 = &list_head_B
        clr.b   0x8(a0)                         | +26  head_B.flag = 0
        move.w  #0xffff, 0x4(a0)                | +2a  head_B.next = $FFFF
        rts                                     | +2e

        .size   SpriteSubsystem_Reset_05A88A, .-SpriteSubsystem_Reset_05A88A
