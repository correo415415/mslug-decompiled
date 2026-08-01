| ============================================================================
|  Metal Slug 1 - asm/camera_list_ctx_helpers_wave_ii.s
|  ----------------------------------------------------------------------------
|  Wave II batch 2 - callees pendientes de Waves HH#1 / HH#3 / II#1.
|
|  Contenido (6 funciones, 236 bytes):
|
|      $043D6C   Reset4CameraLongs_043D6C          26 B  reset 4 slots camera
|      $043D86   CameraApplyAll4_043D86            36 B  aplica a 4 sistemas
|      $05DBC2   ListCursor_Reinit_05DBC2          26 B  reinit cursor lista
|      $05DBDC   ListCursor_ReinitClipped_05DBDC   36 B  idem + guarda $FFFF
|      $05DC00   CompareField10_CCR_05DC00         28 B  probe CCR (clon #2)
|      $025012   PlayerCtx_InitExtended_025012     84 B  init ctx multi-player
|
|  Todas son callees DIRECTAS de funciones ya matcheadas en oleadas previas,
|  lo que las hace cierres naturales del grafo de llamadas:
|
|      Reset4CameraLongs   <- SceneLoader_Main_043568        (HH#1, bsr.w)
|      ListCursor_Reinit   <- InstallListPubHead_05DB58      (II#1, jsr pc)
|      PlayerCtx_InitExt   <- PlayerCtx_ResetTwoBlocks_024FEC(HH#3, bcs.w)
|
|  ---------- NOTA ARQUITECTONICA: correccion de la hipotesis de HH#2 -------
|
|  Wave HH#2 documento la jump-table `$096B9C[8]` de `ClampAndLookup8_096B7E`
|  como "8 handlers de un sub-dispatcher attract pendiente" y la propuso como
|  objetivo principal de Wave II. **Esa hipotesis era INCORRECTA y queda aqui
|  corregida**: los 8 punteros NO apuntan a codigo sino a TABLAS DE DATOS.
|
|  Verificacion (dump de $096BBC interpretado con stride $14):
|
|      entry[0] @$096BBC: 0040 04B0 0170 0004 D70C 0030 F000 003E 0029 498E
|      entry[1] @$096BD0: 0040 0550 0170 0004 D70C 0090 F000 0043 0029 4214
|      entry[2] @$096BE4: 0100 0560 0140 0004 F46A 0000 0000 FFFF FFFF FFFF
|      ...
|
|  El stride de $14 (20 B) y el centinela $FFFF coinciden EXACTAMENTE con el
|  formato que iteran `AttractCuller_Cam0_0969C2` / `_Cam1_096A0E` (HH#2):
|  ambos hacen `cmpi.w #$FFFF, (a0,d4.w)` y `addi.w #$14, $70(a6)`.
|
|  Conclusion: `ClampAndLookup8_096B7E` es un **table-of-tables** — dado un
|  indice de escena attract (0..7) devuelve en `a0` la LISTA DE SPRITES de esa
|  escena. Los 8 targets son datos de la ROM del usuario y, conforme a las
|  reglas del proyecto, NO son decompilables: no se incorporan al registry.
|  El campo `$21(a6)` que alimenta el clamp es por tanto el "indice de escena
|  attract", no un opcode de dispatch.
|
|  ---------- Reset4CameraLongs_043D6C ---------------------------------------
|
|  Limpia el primer long de los CUATRO sistemas de camara del engine. Los
|  offsets confirman el layout ya documentado en Wave CC#1 (4 sistemas con
|  stride $7C entre bases $106F6C / $107FE8 / $108064 / $1080E0):
|
|      $106F7A = $106F6C + $E    (camara principal)
|      $107FF6 = $107FE8 + $E    (camara secundaria)
|      $108072 = $108064 + $E    (camara terciaria)
|      $1080EE = $1080E0 + $E    (camara cuaternaria)
|
|  El offset comun $E es el mismo campo que `CameraApplyAll4_043D86` testea
|  con `tst.l $E(a0)` como gate "sistema activo". Es decir: esta funcion
|  DESACTIVA los cuatro sistemas de camara de golpe.
|
|      /* Desactiva los 4 sistemas de camara (campo activo = 0). */
|      void Reset4CameraLongs(void) {
|          camera[0].active = 0;   /* $106F7A */
|          camera[1].active = 0;   /* $107FF6 */
|          camera[2].active = 0;   /* $108072 */
|          camera[3].active = 0;   /* $1080EE */
|      }
|
|  Callers conocidos: 1 (SceneLoader_Main_043568, HH#1, via bsr.w $43D6C
|  justo tras publicar el puntero de configuracion de escena).
|
|  Por que NO es rederivable por GCC 1:1:
|    1. Cuatro `clr.l abs.l` (6 B c/u) con direcciones absolutas largas en
|       lugar de `lea base,a0` + `clr.l $E(a0)` con stride. GCC habria
|       detectado el patron regular (stride $7C) y emitido un bucle o al
|       menos direccionamiento indexado. Aqui son 4 escrituras literales.
|    2. Mismo idioma "reset explicito con clr.l abs.l" ya documentado en
|       Attract_SoftReset (FF#6) y Camera_ResetSmoothing (HH#1).
|
|  ---------- CameraApplyAll4_043D86 -----------------------------------------
|
|  Aplica la rutina de actualizacion de camara ($043DAA) a los CUATRO
|  sistemas, en orden. Los tres primeros por `bsr.w` explicito y el CUARTO
|  por FALL-THROUGH: tras `lea $1080E0.l, a0` la ejecucion cae directamente
|  en $043DAA sin branch.
|
|      /* Actualiza los 4 sistemas de camara. */
|      void CameraApplyAll4(void) {
|          CameraApplyOne(&camera[0]);   /* bsr.w  $106F6C */
|          CameraApplyOne(&camera[1]);   /* bsr.w  $107FE8 */
|          CameraApplyOne(&camera[2]);   /* bsr.w  $108064 */
|          CameraApplyOne(&camera[3]);   /* FALL-THROUGH $1080E0 */
|      }
|
|  El fall-through final es una micro-optimizacion clasica: ahorra 4 B del
|  `bsr.w` y 2 B del `rts` propio (reutiliza el `rts` de $043DAA), a costa
|  de que la funcion no tenga epilogo. Es la SEXTA aparicion del idioma
|  "salida por fall-through a la funcion vecina" (Waves T, DD, EE#3, GG#2,
|  HH#3 y ahora II#2), y la PRIMERA en que el fall-through no es una salida
|  alternativa sino la ULTIMA ITERACION de un bucle desenrollado.
|
|  Por que NO es rederivable por GCC 1:1:
|    1. Bucle de 4 iteraciones DESENROLLADO con la ultima por fall-through.
|       GCC habria emitido un bucle real (`dbra` sobre stride $7C) o 4
|       llamadas completas con `rts` propio. Nunca omite el epilogo.
|    2. Los cuatro `lea abs.l, a0` cargan bases con stride REGULAR ($7C)
|       pero se escriben literales, sin aprovechar `adda.w #$7C, a0`.
|       Consistente con la hipotesis de macro `CAMERA_APPLY base`.
|
|  ---------- ListCursor_Reinit_05DBC2 ---------------------------------------
|
|  Reinicia el cursor de la lista activa del contexto y blitea su primera
|  entrada. Es el callee de `InstallListPubHead_05DB58` (II#1) y completa
|  ese pipeline: instalar lista -> publicar tamanyo -> reiniciar cursor.
|
|      /* Reinicia cursor de lista activa y blitea la entrada actual. */
|      void ListCursor_Reinit(void) {
|          ListEntry *e = ctx->list_ptr;      /* $3C(a6) */
|          u16 *vram    = (u16*) ctx->vram;   /* $22(a6) */
|          Fix_BlitRow(vram, e->tile, e->cols, e->rows);   /* $5DA56 */
|      }
|
|  Layout de ListEntry confirmado (coherente con II#1):
|      $0 : u16 tile   (codigo de tile + atributos)
|      $2 : u16 cols
|      $4 : u16 rows
|
|  Absorbe el FP `JsrAbsThunk_05dbd4` (Wave I): los 8 B en $05DBD4..$05DBDB
|  son la cola `jsr $5DA56.l; rts` de esta funcion.
|
|  Por que NO es rederivable por GCC 1:1:
|    1. `movea.w $22(a6), a1` carga un puntero VRAM desde un campo WORD con
|       sign-extend a 32 bits. GCC habria usado `movea.l` sobre un campo
|       long, o `move.w` + `ext.l`. Aqui se explota que los offsets del Fix
|       Layer ($7000..$74FF) caben en 16 bits con signo.
|    2. Tres lecturas consecutivas `(a0)`, `$2(a0)`, `$4(a0)` a d0/d1/d2 en
|       el orden EXACTO de los parametros del backend: es paso de argumentos
|       por registro fijo, incompatible con el ABI de GCC.
|
|  ---------- ListCursor_ReinitClipped_05DBDC --------------------------------
|
|  Variante de la anterior con dos diferencias:
|    (a) guarda de lista vacia: si `*list_ptr == $FFFF` retorna sin hacer nada;
|    (b) fuerza `d0 = $FF` (tile de borrado) y llama al backend de RELLENO
|        `$5DA9C` (Fix_BlitRect, Wave W) en vez del de fila `$5DA56`.
|
|  Es decir: **es el "borrar la lista actual"** frente al "pintar la lista
|  actual" de la variante anterior. Par simetrico pintar/borrar, idioma ya
|  visto en Wave Z#5/#6 (probe/revert) y CC#2 (apply/restore).
|
|      /* Borra el area de la lista activa (tile $FF) si la lista no esta vacia. */
|      void ListCursor_ReinitClipped(void) {
|          ListEntry *e = ctx->list_ptr;
|          if (e->tile == 0xFFFF) return;              /* lista vacia */
|          Fix_BlitRect(ctx->vram, 0xFF, e->cols, e->rows);   /* $5DA9C */
|      }
|
|  Absorbe el FP `JsrAbsThunk_05dbf8` (Wave I): los 8 B en $05DBF8..$05DBFF
|  son la cola `jsr $5DA9C.l; rts` de esta funcion.
|
|  Por que NO es rederivable por GCC 1:1:
|    1. `move.w #$FF, d0` (4 B) en lugar de `moveq #$FF, d0` (2 B) — que
|       ademas daria $FFFFFFFF por sign-extend, no $000000FF. La eleccion
|       del encoding largo es SEMANTICA, no estilistica: GCC habria usado
|       `moveq #0,d0; move.b #$FF,d0` o similar.
|    2. La guarda `cmpi.w #$FFFF, (a0); beq.w` salta al `rts` COMPARTIDO
|       del final ($05DBFE), no a un `rts` propio. Salida unica via branch.
|
|  ---------- CompareField10_CCR_05DC00 --------------------------------------
|
|  **CLON BYTE-A-BYTE de `CompareField10_CCR_05DB3C` (Wave II#1)**, situado
|  $C4 bytes mas adelante. Mismo cuerpo, mismos offsets, misma semantica:
|
|      self.f10 <  linked.f10  ->  CCR-C SET    (`ori.b #$11, ccr`)
|      self.f10 >= linked.f10  ->  CCR-C CLEAR  (`andi.b #$EE, ccr`)
|
|  Es el QUINTO par de "clones no factorizados" del proyecto (tras BB#2,
|  Z#5/#6, HH#2 Cam0/Cam1 y II#1 SlotExtractCoords paths A/B), y refuerza
|  definitivamente la hipotesis de macros ASM pesadas: una macro
|  `CMP_FIELD10_CCR` expandida en dos puntos distintos del mismo fichero
|  fuente original.
|
|  Absorbe los FPs `ClearXN_05dc10` y `SetXN_05dc16` (Wave N): son las dos
|  ramas del probe, exactamente igual que los FPs #33/#34 de II#1.
|
|  ---------- PlayerCtx_InitExtended_025012 ----------------------------------
|
|  Rama "player_count >= 2" de `PlayerCtx_ResetTwoBlocks_024FEC` (HH#3), a la
|  que se llega por el `bcs.w $025012` de aquella. Inicializa el contexto
|  extendido multi-jugador:
|
|    1. Marca `ctx_mode = MULTI` ($106ECA = 1). Nota: HH#3 documento que la
|       rama corta publica 0 aqui; se confirma la semantica 0=SINGLE/1=MULTI.
|    2. Indexa la tabla de buffers `$E7C00[]` con `player_count * 4` y publica
|       el puntero resultante en `$106EBE` (buffer activo del jugador).
|    3. Limpia cuatro campos de estado ($106EC2, $106EC4, $106EBC, $106EC6).
|    4. Gate de arranque: si el flag `$025118` esta armado, rellena 512 bytes
|       del buffer recien seleccionado con $FF (bucle `dbra` de $1FF+1 iters).
|
|      /* Inicializa el contexto extendido para 2+ jugadores. */
|      void PlayerCtx_InitExtended(u16 player_count) {
|          ctx_mode      = 1;                          /* $106ECA */
|          active_buffer = buffer_table[player_count]; /* $E7C00[] */
|          state_a = state_b = state_c = 0;            /* $106EC2/EC4/EBC */
|          flags_d = 0;                                /* $106EC6 */
|          if (init_flag)                              /* $025118 */
|              memset(active_buffer, 0xFF, 512);
|      }
|
|  Por que NO es rederivable por GCC 1:1:
|    1. `add.w d0, d0` DOS VECES para multiplicar por 4, en lugar de
|       `lsl.w #2, d0` (una instruccion, mismo tamanyo). Idioma de
|       ensamblador antiguo optimizado para 68000 temprano.
|    2. `tst.b $25118.l` lee un byte de la propia ROM de programa como flag
|       de configuracion (direccion dentro del rango de codigo $025xxx).
|       Es una constante de build embebida en el binario; GCC la habria
|       resuelto en tiempo de compilacion y eliminado la rama muerta.
|    3. `move.b #$FF, d1` + `move.w #$1FF, d5` + bucle `move.b d1,(a0)+ /
|       dbra` es un memset hand-coded byte a byte. GCC habria emitido un
|       bucle de longs (4x mas rapido) o llamado a memset.
|    4. Reutiliza `d0` como cero tras el `clr.w d0` para las cuatro
|       publicaciones siguientes (3 words + 1 byte) en vez de usar `clr.w`
|       / `clr.b` directos sobre cada destino. Ahorra 8 B a costa de
|       legibilidad — decision de programador humano.
|
|  Toolchain:  m68k-linux-gnu-gcc -m68000 -Wa,--register-prefix-optional
|  ============================================================================

        .text

|
| ---------------------------------------------------------------------------
|  Reset4CameraLongs_043D6C  @ $043D6C  (26 bytes)
|  Callee de SceneLoader_Main_043568 (Wave HH#1).
| ---------------------------------------------------------------------------
|
        .globl  Reset4CameraLongs_043D6C
        .type   Reset4CameraLongs_043D6C, @function
        .section .text.Reset4CameraLongs_043D6C, "ax", @progbits
Reset4CameraLongs_043D6C:
        clr.l   0x106f7a.l                      | +00  camera[0].active = 0
        clr.l   0x107ff6.l                      | +06  camera[1].active = 0
        clr.l   0x108072.l                      | +0c  camera[2].active = 0
        clr.l   0x1080ee.l                      | +12  camera[3].active = 0
        rts                                     | +18

        .size   Reset4CameraLongs_043D6C, .-Reset4CameraLongs_043D6C

|
| ---------------------------------------------------------------------------
|  CameraApplyAll4_043D86  @ $043D86  (36 bytes)
|  Bucle desenrollado de 4 iteraciones; la 4a por FALL-THROUGH a $043DAA.
| ---------------------------------------------------------------------------
|
        .globl  CameraApplyAll4_043D86
        .type   CameraApplyAll4_043D86, @function
        .section .text.CameraApplyAll4_043D86, "ax", @progbits
CameraApplyAll4_043D86:
        lea.l   0x106f6c.l, a0                  | +00  a0 = &camera[0]
        bsr.w   CameraApplyOne_043DAA                     | +06  CameraApplyOne()
        lea.l   0x107fe8.l, a0                  | +0a  a0 = &camera[1]
        bsr.w   CameraApplyOne_043DAA                     | +10  CameraApplyOne()
        lea.l   0x108064.l, a0                  | +14  a0 = &camera[2]
        bsr.w   CameraApplyOne_043DAA                     | +1a  CameraApplyOne()
        lea.l   0x1080e0.l, a0                  | +1e  a0 = &camera[3]
                                                | +24  FALL-THROUGH a $043DAA
                                                |      (reutiliza su rts)

        .size   CameraApplyAll4_043D86, .-CameraApplyAll4_043D86

|
| ---------------------------------------------------------------------------
|  ListCursor_Reinit_05DBC2  @ $05DBC2  (26 bytes)
|  Callee de InstallListPubHead_05DB58 (Wave II#1).
|  Absorbe FP: JsrAbsThunk_05dbd4 (Wave I) = cola jsr+rts.
| ---------------------------------------------------------------------------
|
        .globl  ListCursor_Reinit_05DBC2
        .type   ListCursor_Reinit_05DBC2, @function
        .section .text.ListCursor_Reinit_05DBC2, "ax", @progbits
ListCursor_Reinit_05DBC2:
        movea.l 0x3c(a6), a0                    | +00  a0 = ctx->list_ptr
        movea.w 0x22(a6), a1                    | +04  a1 = ctx->vram (sign-ext)
        move.w  (a0), d0                        | +08  d0 = entry->tile
        move.w  0x2(a0), d1                     | +0a  d1 = entry->cols
        move.w  0x4(a0), d2                     | +0e  d2 = entry->rows
        jsr     ThunkTarget_05da56              | +12  Fix_BlitRow()
        rts                                     | +18

        .size   ListCursor_Reinit_05DBC2, .-ListCursor_Reinit_05DBC2

|
| ---------------------------------------------------------------------------
|  ListCursor_ReinitClipped_05DBDC  @ $05DBDC  (36 bytes)
|  Contrapartida "borrar" de la anterior (tile $FF via Fix_BlitRect).
|  Absorbe FP: JsrAbsThunk_05dbf8 (Wave I) = cola jsr+rts.
| ---------------------------------------------------------------------------
|
        .globl  ListCursor_ReinitClipped_05DBDC
        .type   ListCursor_ReinitClipped_05DBDC, @function
        .section .text.ListCursor_ReinitClipped_05DBDC, "ax", @progbits
ListCursor_ReinitClipped_05DBDC:
        movea.l 0x3c(a6), a0                    | +00  a0 = ctx->list_ptr
        cmpi.w  #0xffff, (a0)                   | +04  if (entry->tile == $FFFF)
        beq.w   .Lclip_done                     | +08    lista vacia: return
        movea.w 0x22(a6), a1                    | +0c  a1 = ctx->vram (sign-ext)
        move.w  #0xff, d0                       | +10  d0 = $FF (tile borrado)
        move.w  0x2(a0), d1                     | +14  d1 = entry->cols
        move.w  0x4(a0), d2                     | +18  d2 = entry->rows
        jsr     ThunkTarget_05da9c              | +1c  Fix_BlitRect()
.Lclip_done:                                    | $05DBFE
        rts                                     | +22

        .size   ListCursor_ReinitClipped_05DBDC, .-ListCursor_ReinitClipped_05DBDC

|
| ---------------------------------------------------------------------------
|  CompareField10_CCR_05DC00  @ $05DC00  (28 bytes)
|  CLON byte-a-byte de CompareField10_CCR_05DB3C (Wave II#1).
|  Absorbe FPs: ClearXN_05dc10 y SetXN_05dc16 (Wave N) = las dos ramas.
| ---------------------------------------------------------------------------
|
        .globl  CompareField10_CCR_05DC00
        .type   CompareField10_CCR_05DC00, @function
        .section .text.CompareField10_CCR_05DC00, "ax", @progbits
CompareField10_CCR_05DC00:
        movea.l 0x8(a6), a1                     | +00  a1 = ctx->linked
        move.b  0x10(a6), d0                    | +04  d0 = self.f10
        cmp.b   0x10(a1), d0                    | +08  compara self vs linked
        bcs.w   .Lcmp2_less                     | +0c  if (self < linked)
        andi.b  #0xee, ccr                      | +10  CCR: C=0 (greater-equal)
        rts                                     | +14
.Lcmp2_less:                                    | $05DC16
        ori.b   #0x11, ccr                      | +16  CCR: C=1 (less-than)
        rts                                     | +1a

        .size   CompareField10_CCR_05DC00, .-CompareField10_CCR_05DC00

|
| ---------------------------------------------------------------------------
|  PlayerCtx_InitExtended_025012  @ $025012  (84 bytes)
|  Rama "player_count >= 2" de PlayerCtx_ResetTwoBlocks_024FEC (Wave HH#3).
| ---------------------------------------------------------------------------
|
        .globl  PlayerCtx_InitExtended_025012
        .type   PlayerCtx_InitExtended_025012, @function
        .section .text.PlayerCtx_InitExtended_025012, "ax", @progbits
PlayerCtx_InitExtended_025012:
        move.b  #0x1, 0x106eca.l                | +00  ctx_mode = MULTI
        add.w   d0, d0                          | +08  d0 *= 2
        add.w   d0, d0                          | +0a  d0 *= 2   (=count*4)
        lea.l   0xe7c00.l, a0                   | +0c  a0 = &buffer_table[0]
        move.l  (a0, d0.w), 0x106ebe.l          | +12  active_buffer = tbl[n]
        clr.w   d0                              | +1a  d0 = 0 (cero reutilizado)
        move.w  d0, 0x106ec2.l                  | +1c  state_a = 0
        move.w  d0, 0x106ec4.l                  | +22  state_b = 0
        move.w  d0, 0x106ebc.l                  | +28  state_c = 0
        move.b  d0, 0x106ec6.l                  | +2e  flags_d = 0
        tst.b   0x25118.l                       | +34  if (!init_flag)
        beq.w   .Lctx_done                      | +3a    skip memset
        movea.l 0x106ebe.l, a0                  | +3e  a0 = active_buffer
        move.b  #0xff, d1                       | +44  d1 = $FF (fill)
        move.w  #0x1ff, d5                      | +48  d5 = 511 (dbra)
.Lctx_fill:                                     | $02505E
        move.b  d1, (a0)+                       | +4c  *a0++ = $FF
        dbra    d5, .Lctx_fill                  | +4e  loop 512 veces
.Lctx_done:                                     | $025064
        rts                                     | +52

        .size   PlayerCtx_InitExtended_025012, .-PlayerCtx_InitExtended_025012
