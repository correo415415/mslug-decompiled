| ============================================================================
|  Metal Slug 1 - asm/camera_apply_cluster_043daa.s
|  ----------------------------------------------------------------------------
|  Wave JJ batch 1 - cluster de aplicacion de camara $043DAA..$043E39.
|
|  Contenido (4 funciones, 144 bytes):
|
|      $043DAA   CameraApplyOne_043DAA      74 B  aplica transform a 1 camara
|      $043DF4   CameraHook_Probe08_043DF4  26 B  hook probe $51C08
|      $043E0E   CameraHook_Probe82_043E0E  22 B  hook probe $51C82
|      $043E24   CameraHook_ProbeF6_043E24  22 B  hook probe $51CF6
|
|  Cierra el subsistema de camara iniciado en Wave CC#1 (coordenadas) y
|  continuado en II#2 (`Reset4CameraLongs_043D6C`, `CameraApplyAll4_043D86`).
|  `CameraApplyOne` es precisamente el callee que `CameraApplyAll4` invoca
|  3 veces por `bsr.w` y una cuarta por FALL-THROUGH.
|
|  ---------- HALLAZGO FORENSE: EPILOGO COMPARTIDO ---------------------------
|
|  Los tres hooks son estructuralmente identicos salvo por el probe que
|  invocan ($51C08 / $51C82 / $51CF6). Pero la comparacion byte-a-byte
|  revela algo mas profundo:
|
|      $043E0C   rts              <- epilogo del hook A
|      ...
|      $043E14   bcc.b $43E0C     <- hook B salta al rts de HOOK A
|      $043E1C   beq.b $43E0C     <- idem
|      $043E22   rts              <- epilogo propio de B: INALCANZABLE
|      ...
|      $043E2A   bcc.b $43E0C     <- hook C salta al rts de HOOK A
|      $043E32   beq.b $43E0C     <- idem
|      $043E38   rts              <- epilogo propio de C: INALCANZABLE
|
|  Es decir: **los hooks B y C reutilizan el `rts` del hook A** como salida
|  temprana, y sus propios `rts` finales son CODIGO MUERTO que nunca se
|  ejecuta (la instruccion previa es siempre `bra.w $43E8C`, incondicional).
|
|  Esto confirma de forma directa la hipotesis fundacional del proyecto:
|  "epilogos compartidos entre funciones aparentemente independientes".
|  Es la PRIMERA evidencia del proyecto en que el reuso de epilogo es
|  CRUZADO (B y C dependen de A) y ademas deja bytes muertos preservados
|  por la macro que emite el epilogo incondicionalmente.
|
|  Consecuencia practica para el matcher: los `bcc.b`/`beq.b` de B y C
|  deben ensamblarse contra un simbolo GLOBAL exportado desde la seccion
|  de A (`CameraHook_SharedRts_043E0C`), no contra una etiqueta local.
|
|  ---------- CameraApplyOne_043DAA ------------------------------------------
|
|  Aplica la transformacion de escala/posicion a UN sistema de camara.
|
|      /* Aplica transform al sistema de camara apuntado por a0.
|       * No-op si el sistema esta inactivo (campo +$E == 0). */
|      void CameraApplyOne(CameraSystem *cam) {
|          if (cam->active == 0) return;              /* $E(a0) */
|          long x = hud_x * cam->scale_x << 8;        /* $108160 * $74(a0) */
|          long y = hud_y * cam->scale_y << 8;        /* $108162 * $76(a0) */
|          Transform_Publish(x, y);                   /* $51B80 */
|          if (cam->flags & 0x01) CameraHook_Probe82();  /* bsr $43E0E */
|          if (cam->flags & 0x02) CameraHook_ProbeF6();  /* bsr $43E24 */
|          Transform_Commit();                        /* $51F30 */
|      }
|
|  Layout de CameraSystem confirmado (coherente con CC#1 y II#2):
|      +$0E : u32  active     (gate; puesto a 0 por Reset4CameraLongs)
|      +$72 : u8   flags      (bit0 -> hook B, bit1 -> hook C)
|      +$74 : u16  scale_x
|      +$76 : u16  scale_y
|      +$78 : void *linked    (leido por los tres hooks)
|
|  Absorbe el FP `JsrAbsThunk_043dec` (Wave I): los 8 B en $043DEC..$043DF3
|  son la cola `jsr $51F30.l; rts` de esta funcion, y ademas el destino del
|  `beq.w` del segundo test de flags.
|
|  Por que NO es rederivable por GCC 1:1:
|    1. `muls.w $74(a0), d0` seguido de `asl.l #8, d0` es un producto
|       16x16->32 con reescalado a punto fijo 8.8 hand-coded. GCC habria
|       emitido `muls.w` + `lsl.l #8` (mismo opcode) pero con distinto
|       orden de operandos y sin intercalar el segundo producto.
|    2. Los dos tests de flags usan `btst.b #N, $72(a0)` + `beq.w` con
|       desplazamiento largo (6+4 B) para saltar 4 y 6 bytes. GCC habria
|       emitido `beq.b` (2 B). Es el idioma "branch largo por convencion"
|       del cluster $043xxx.
|    3. El `beq.w` del segundo test salta a $043DEC, que es exactamente el
|       inicio del epilogo `jsr $51F30.l; rts` — o sea, la salida normal y
|       la salida "sin hook C" convergen en el mismo punto sin duplicar.
|
|  ---------- CameraHook_Probe08 / Probe82 / ProbeF6 -------------------------
|
|  Los tres comparten cuerpo exacto salvo el probe invocado:
|
|      /* Ejecuta el probe correspondiente; si devuelve CCR-C set y el
|       * sistema tiene un enlace no nulo, salta al procesador $43E8C.
|       * En cualquier otro caso retorna. */
|      void CameraHook_ProbeNN(void) {
|          if (!Probe_NN()) return;              /* bcc -> rts compartido */
|          void *lnk = cam->linked;              /* $78(a0) */
|          if (lnk == NULL) return;              /* beq -> rts compartido */
|          goto Process_043E8C;                  /* bra.w, no retorna */
|      }
|
|  El `bra.w $43E8C` es un salto a otra funcion SIN retorno (tail-jump con
|  el marco de pila del caller intacto), por lo que el `rts` que le sigue
|  en B y C es inalcanzable. Solo el de A se usa, y lo usan los tres.
|
|  Callers conocidos: `CameraApplyOne_043DAA` invoca B (bit0) y C (bit1).
|  El hook A ($043DF4, probe $51C08) no tiene caller identificado todavia
|  dentro del codigo matcheado — probablemente lo invoca el procesador
|  $43E8C o una rama del pipeline de colisiones aun sin cerrar.
|
|  Por que NO son rederivables por GCC 1:1:
|    1. Reuso cruzado del epilogo de A por parte de B y C (ver arriba).
|       GCC jamas genera un branch a la instruccion `rts` de otra funcion.
|    2. `rts` muertos preservados al final de B y C: GCC eliminaria el
|       codigo inalcanzable con -Os.
|    3. Hook A usa `bcc.w`/`beq.w` (4 B) mientras B y C usan `bcc.b`/`beq.b`
|       (2 B) para el MISMO salto logico. La diferencia se explica porque
|       en A el destino esta 18 B por delante (aun sin resolver cuando el
|       ensamblador procesa la macro) y en B/C esta ya definido hacia atras.
|       Es un artefacto de ensamblado en una sola pasada.
|    4. `move.l a1, d7` solo para poner a cero el flag Z sobre un registro
|       de direcciones (en 68000 `movea` no altera CCR). Idioma clasico
|       hand-coded; GCC habria usado `cmpa.w #0, a1`.
|
|  Toolchain:  m68k-linux-gnu-gcc -m68000 -Wa,--register-prefix-optional
|  ============================================================================

        .text

|
| ---------------------------------------------------------------------------
|  CameraApplyOne_043DAA  @ $043DAA  (74 bytes)
|  Callee de CameraApplyAll4_043D86 (Wave II#2): 3x bsr.w + 1x fall-through.
|  Absorbe FP: JsrAbsThunk_043dec (Wave I) = cola jsr $51F30.l + rts.
| ---------------------------------------------------------------------------
|
        .globl  CameraApplyOne_043DAA
        .type   CameraApplyOne_043DAA, @function
        .section .text.CameraApplyOne_043DAA, "ax", @progbits
CameraApplyOne_043DAA:
        tst.l   0xe(a0)                         | +00  if (!cam->active)
        beq.w   .Lcam_done                      | +04    return
        move.w  0x108160.l, d0                  | +08  d0 = hud_x
        move.w  0x108162.l, d1                  | +0e  d1 = hud_y
        muls.w  0x74(a0), d0                    | +14  d0 *= cam->scale_x
        asl.l   #0x8, d0                        | +18  d0 <<= 8  (fixed 8.8)
        muls.w  0x76(a0), d1                    | +1a  d1 *= cam->scale_y
        asl.l   #0x8, d1                        | +1e  d1 <<= 8  (fixed 8.8)
        jsr     Integrator_XY_051B80                     | +20  Transform_Publish(d0,d1)
        btst.b  #0x0, 0x72(a0)                  | +26  if (!(flags & 1))
        beq.w   .Lcam_skip_b                    | +2c    skip hook B
        bsr.w   CameraHook_Probe82_043E0E       | +30  hook B
.Lcam_skip_b:                                   | $043DDE
        btst.b  #0x1, 0x72(a0)                  | +34  if (!(flags & 2))
        beq.w   .Lcam_commit                    | +3a    skip hook C
        bsr.w   CameraHook_ProbeF6_043E24       | +3e  hook C
.Lcam_commit:                                   | $043DEC
        jsr     TransformCommit_MMIO_051F30              | +42  Transform_Commit()
.Lcam_done:                                     | $043DF2
        rts                                     | +48

        .size   CameraApplyOne_043DAA, .-CameraApplyOne_043DAA

|
| ---------------------------------------------------------------------------
|  CameraHook_Probe08_043DF4  @ $043DF4  (26 bytes)
|  Exporta CameraHook_SharedRts_043E0C: el `rts` que reutilizan B y C.
| ---------------------------------------------------------------------------
|
        .globl  CameraHook_Probe08_043DF4
        .globl  CameraHook_SharedRts_043E0C
        .type   CameraHook_Probe08_043DF4, @function
        .section .text.CameraHook_Probe08_043DF4, "ax", @progbits
CameraHook_Probe08_043DF4:
        jsr     Fn_00051C08                     | +00  probe A
        bcc.w   CameraHook_SharedRts_043E0C     | +06  if (!C) return
        movea.l 0x78(a0), a1                    | +0a  a1 = cam->linked
        move.l  a1, d7                          | +0e  set CCR-Z sobre a1
        beq.w   CameraHook_SharedRts_043E0C     | +10  if (linked == NULL) ret
        bra.w   Fn_00043E8C                     | +14  tail-jump (no retorna)
CameraHook_SharedRts_043E0C:                    | $043E0C
        rts                                     | +18  <- epilogo COMPARTIDO

        .size   CameraHook_Probe08_043DF4, .-CameraHook_Probe08_043DF4

|
| ---------------------------------------------------------------------------
|  CameraHook_Probe82_043E0E  @ $043E0E  (22 bytes)
|  Invocado por CameraApplyOne cuando flags & bit0.
|  Su `rts` final ($043E22) es CODIGO MUERTO: usa el epilogo de Probe08.
| ---------------------------------------------------------------------------
|
        .globl  CameraHook_Probe82_043E0E
        .type   CameraHook_Probe82_043E0E, @function
        .section .text.CameraHook_Probe82_043E0E, "ax", @progbits
CameraHook_Probe82_043E0E:
        jsr     Fn_00051C82                     | +00  probe B
        bcc.b   CameraHook_SharedRts_043E0C     | +06  if (!C) -> rts de A
        movea.l 0x78(a0), a1                    | +08  a1 = cam->linked
        move.l  a1, d7                          | +0c  set CCR-Z sobre a1
        beq.b   CameraHook_SharedRts_043E0C     | +0e  if (NULL) -> rts de A
        bra.w   Fn_00043E8C                     | +10  tail-jump (no retorna)
        rts                                     | +14  INALCANZABLE (macro)

        .size   CameraHook_Probe82_043E0E, .-CameraHook_Probe82_043E0E

|
| ---------------------------------------------------------------------------
|  CameraHook_ProbeF6_043E24  @ $043E24  (22 bytes)
|  Invocado por CameraApplyOne cuando flags & bit1.
|  Clon byte-a-byte de Probe82 salvo el probe ($51CF6 vs $51C82).
|  Su `rts` final ($043E38) es igualmente CODIGO MUERTO.
| ---------------------------------------------------------------------------
|
        .globl  CameraHook_ProbeF6_043E24
        .type   CameraHook_ProbeF6_043E24, @function
        .section .text.CameraHook_ProbeF6_043E24, "ax", @progbits
CameraHook_ProbeF6_043E24:
        jsr     Fn_00051CF6                     | +00  probe C
        bcc.b   CameraHook_SharedRts_043E0C     | +06  if (!C) -> rts de A
        movea.l 0x78(a0), a1                    | +08  a1 = cam->linked
        move.l  a1, d7                          | +0c  set CCR-Z sobre a1
        beq.b   CameraHook_SharedRts_043E0C     | +0e  if (NULL) -> rts de A
        bra.w   Fn_00043E8C                     | +10  tail-jump (no retorna)
        rts                                     | +14  INALCANZABLE (macro)

        .size   CameraHook_ProbeF6_043E24, .-CameraHook_ProbeF6_043E24
