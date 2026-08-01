| ============================================================================
|  Metal Slug 1 - asm/scene_loader_cluster_043xxx.s
|  ----------------------------------------------------------------------------
|  Wave HH batch 1 - cluster de reset/integrador + scene loader en $043xxx.
|
|  Contenido (5 funciones, 546 bytes):
|
|      $0434EA   Camera_ResetSmoothing_0434EA      12 B  stub reset
|      $0434F8   Camera_SmoothingIntegrate_0434F8 104 B  integrador+clamp
|      $043562   SceneLoader_ByIndex_043562        6 B  entry alt (d0=0)
|      $043568   SceneLoader_Main_043568         382 B  scene loader
|
|  El fichero cubre 3 helpers contiguos del subsistema camera/scene que se
|  cruzan por bsr.w y comparten los slots globales $10817A/$10817E/$108182
|  (delta camera-Y smoothing) y $10815C (puntero de configuracion de escena
|  activa).
|
|  Ademas la funcion "scene loader" invoca por bsr.w tres helpers vecinos
|  ($43D6C stub-reset y $43EDA buffer-clear W#CC1) que quedan expuestos por
|  otras entradas del registry.
|
|  ---------- Camera_ResetSmoothing_0434EA -----------------------------------
|
|  Reset simetrico de los dos slots long del "smoothing accumulator" de
|  camera-Y en $10817A y $10817E. El primero (=camera_y_target) y el segundo
|  (=camera_y_velocity) se usan por Camera_SmoothingIntegrate para producir
|  el efecto de "look-ahead" con inercia que se ve en attract/gameplay.
|
|      /* Reset del acumulador de smoothing de camera Y. */
|      void Camera_ResetSmoothing(void) {
|          camera_y_target   = 0;      /* $10817A.l */
|          camera_y_velocity = 0;      /* $10817E.l */
|      }
|
|  Callers conocidos: $043568 (SceneLoader_Main).
|
|  Por que NO es rederivable por GCC 1:1:
|    - Dos clr.l consecutivos con direccion absoluta larga. GCC habria
|      emitido lea $10817A,a0; clr.l (a0)+; clr.l (a0)  (10 B vs 12 B),
|      pero aqui el patron `clr.l abs.l` se repite exactamente (mismo
|      idioma de "reset explicito" ya observado en Attract_SoftReset FF#6).
|
|  ---------- Camera_SmoothingIntegrate_0434F8 -------------------------------
|
|  Actualiza el acumulador de smoothing de camera Y con damping asimetrico
|  y clamp saturado a [-$80000, +$80000]. Publica el delta hi-word en
|  $108182 (offset visible que se lee mas adelante en el pipeline por
|  Geom_Proj_Clamp FF#2 via `sub.w $10817a.l, d4` en fase 2).
|
|  Semantica reconstruida:
|
|      /* Integra un paso del smoothing de camera Y.
|       *
|       * Sea t = camera_y_target y v = camera_y_velocity.
|       *
|       * 1) Si |v| < $4000, damping fuerte (v/8), si no damping nulo.
|       *    Esto crea el "asentamiento suave" cuando la velocidad es
|       *    pequenya y "movimiento libre" cuando es grande.
|       *
|       *    v_damped = (v/8) + t     (delta a restar)
|       *
|       * 2) v_new = v - v_damped        [nuevo velocity]
|       *    t_new = t + v_new           [tentative target]
|       *
|       * 3) Clamp saturado: t_new = clamp(t_new, -$80000, +$80000).
|       *
|       * 4) Publica delta:
|       *      camera_y_delta = (t_new - t) hi_word    [$108182.w] */
|      void Camera_SmoothingIntegrate(void);
|
|  Callers conocidos: pipeline principal de camera-Y (llamado desde el
|  "camera per-frame update", aun no identificado).
|
|  Por que NO es rederivable por GCC 1:1:
|    1. `cmpi.l #$4000, d3; bge.b; cmpi.l #$FFFFC000, d3; bgt.w; asr.l #$3, d3`
|       es un test |v|<$4000 hand-coded. GCC habria emitido cmp.l abs con
|       secuencia distinta (por ejemplo tst.l + bpl/bmi mask por sign bit).
|    2. Dos ramas de clamp saturado (ble/bgt) con `move.l #$80000, d2` /
|       `move.l #$FFF80000, d2` inline en lugar de tabular como constante.
|       GCC no factoriza el clamp en un helper, cada rama es explicita.
|    3. `swap d2; swap d0; sub.w d0, d2; move.w d2, d0` es un "extraer y
|       restar hi-words" hand-coded. GCC habria emitido shifts o typecast
|       long->short con distinta secuencia.
|
|  ---------- SceneLoader_ByIndex_043562 (entry alt) + Main_043568 -----------
|
|  DUAL-ENTRY hand-coded (patron ya visto en Wave AA#3 Player_Dispatch).
|
|      $043562  entry corta (d0=0): salta al fetch del puntero
|      $043568  entry larga (d0=idx): descarta byte alto, escala x8
|
|  La entry corta se usa cuando el caller ya tiene el indice implicito
|  (siempre escena 0). La entry larga clampea el indice a 8 bits y lo
|  escala a offset de 8 B en la tabla de descriptores de escena $916C8:
|
|      struct SceneDescriptor {
|          void   *config_ptr;      /* +0: puntero a config global */
|          Entry  *entry_script;    /* +4: puntero al bytecode de entities */
|      };
|      struct SceneDescriptor  scene_table[256];   /* $916C8[256] */
|
|  Del script se sabe:
|
|      struct SceneEntry {
|          u8   type;               /* +0: 0/1/other; 2 = terminador */
|          u8   subop;              /* +1: pasado a helper $1390E */
|          u32  template_ptr;       /* +2: template para Entity alloc */
|          u8   payload[8];         /* +6: params extra pasados a $51ABE */
|      };  /* sizeof = 14 = $E */
|
|  El loader hace DOS pasadas sobre el bytecode:
|
|    PASADA 1 (conteo): recorre desde a0 hasta type==2 (terminador).
|         type==0  -> d1 += $20     (contador de "entities tipo A")
|         else     -> d2 += $20     (contador de "entities tipo B")
|         a0       += $E cada iter
|
|    Al final llama al asignador de scratch:
|         jsr $1390E(bank_a=d0=d2, bank_b=d2=subop, ...);
|
|    PASADA 2 (spawn real): restaura a0 desde la pila, recorre otra vez,
|         type==0  -> jsr $13982  (branch spawn tipo A)
|         else     -> jsr $13952  (branch spawn tipo B)
|         Luego para cada entrada:
|           - a0 = *(a2+2)      (template_ptr del script)
|           - a1 = a2+6         (payload)
|           - jsr $51ABE        (Entity_AllocAndInit)
|           - inicializa flags fijos del entity resultante en a0:
|               $74(a0) = $100    (scale/anim base)
|               $76(a0) = $100
|               $72(a0) = 0       (flags)
|               $78(a0) = 0L      (linked entity ptr)
|
|    POST-LOAD (a partir de $043610): reset masivo del contexto de camera
|    y HUD, seguido de 5 jsr a inicializadores globales:
|         $7707c   (subsistema HUD)
|         $8f158   (subsistema audio-scene)
|         $3ee3a   (subsistema puntuaciones)
|         $997b8   (subsistema attract-hooks)
|         $4cb5c   (subsistema misc / final)
|
|  Firma C conceptual:
|
|      /* Carga la escena `idx` (0..255) de scene_table[]. */
|      void SceneLoader_Main(u8 idx);
|      void SceneLoader_ByIndex(void);        /* == SceneLoader_Main(0) */
|
|  Callers conocidos: 8 (segun scan_unmatched_callees) — muy alto caller
|  count. Es el punto de entrada canonico de carga de escena del juego.
|
|  Por que NO es rederivable por GCC 1:1:
|    1. Dual-entry: `$043562: clr.w d0; bra.w $43574` salta al `move.l`
|       INTERNO de la funcion larga (offset +$C), no al inicio. GCC nunca
|       emite fall-through cross-function.
|    2. La tabla $916C8 se indexa con `move.l (a0, d0.w), $10815c.l` y
|       `movea.l $4(a0, d0.w), a0`. GCC habria cargado struct member por
|       member con `lea` + `move.l` separados.
|    3. Doble pasada sobre el mismo bytecode con `move.l a0, -(a7)` +
|       `movea.l (a7), a0` + `movea.l (a7)+, a0` para restaurar. GCC lo
|       habria hecho con una variable local en frame pointer.
|    4. 5 `jsr abs.l` finales sin agrupar en un array de function-pointers.
|       Es una lista literal de hooks de init estilo ensamblador antiguo.
|    5. `move.w #$0, $108168.l` (10 B) en vez de `clr.w $108168.l` (6 B):
|       el codigo esta escrito para EJECUTAR igual, no para optimizar
|       tamanyo. GCC habria emitido clr.w en las 4 posiciones donde
|       aparece este patron.
|
|  Toolchain:  m68k-linux-gnu-gcc -m68000 -Wa,--register-prefix-optional
|  ============================================================================

        .text

|
| ---------------------------------------------------------------------------
|  Camera_ResetSmoothing_0434EA  @ $0434EA  (12 bytes)
| ---------------------------------------------------------------------------
|
        .globl  Camera_ResetSmoothing_0434EA
        .type   Camera_ResetSmoothing_0434EA, @function
        .section .text.Camera_ResetSmoothing_0434EA, "ax", @progbits
Camera_ResetSmoothing_0434EA:
        clr.l   0x10817a.l                     | +00  camera_y_target   = 0
        clr.l   0x10817e.l                     | +06  camera_y_velocity = 0
        rts                                    | +0c

|
| ---------------------------------------------------------------------------
|  Camera_SmoothingIntegrate_0434F8  @ $0434F8  (104 bytes)
| ---------------------------------------------------------------------------
|
        .globl  Camera_SmoothingIntegrate_0434F8
        .type   Camera_SmoothingIntegrate_0434F8, @function
        .section .text.Camera_SmoothingIntegrate_0434F8, "ax", @progbits
Camera_SmoothingIntegrate_0434F8:
        move.l  0x10817a.l, d0                 | +00  d0 = t (target)
        move.l  d0, d1                         | +06  d1 = t
        move.l  0x10817e.l, d2                 | +08  d2 = v (velocity)
        move.l  d2, d3                         | +0e  d3 = v
        cmpi.l  #0x4000, d3                    | +10  if (v >= $4000)
        bge.b   .Lno_damp                      | +16    skip damping
        cmpi.l  #0xffffc000, d3                | +18  if (v > -$4000)
        bgt.w   .Ldamp                         | +1e    apply damping
.Lno_damp:                                     | $04351A
        asr.l   #0x3, d3                       | +22  d3 = v / 8  (damped)
.Ldamp:                                        | $04351C
        add.l   d1, d3                         | +24  d3 = (v/8) + t
        sub.l   d3, d2                         | +26  v_new = v - (v/8) - t
        move.l  d2, 0x10817e.l                 | +28  camera_y_velocity = v_new
        add.l   d0, d2                         | +2e  d2 = t + v_new
        cmpi.l  #0x80000, d2                   | +30  if (d2 > +$80000)
        ble.w   .Lclamp_lo                     | +36
        move.l  #0x80000, d2                   | +3a    d2 = +$80000
        bra.w   .Lstore                        | +40
.Lclamp_lo:                                    | $04353C
        cmpi.l  #0xfff80000, d2                | +44  if (d2 > -$80000)
        bgt.w   .Lstore                        | +4a    skip
        move.l  #0xfff80000, d2                | +4e    d2 = -$80000
.Lstore:                                       | $04354C
        move.l  d2, 0x10817a.l                 | +54  camera_y_target = d2
        swap    d2                             | +5a  d2 = t_new hi_word
        swap    d0                             | +5c  d0 = t     hi_word
        sub.w   d0, d2                         | +5e  d2 = (t_new - t) hi_word
        move.w  d2, d0                         | +60
        move.w  d0, 0x108182.l                 | +62  camera_y_delta = delta
        rts                                    | +68

|
| ---------------------------------------------------------------------------
|  SceneLoader_ByIndex_043562  @ $043562  (6 bytes, entry alt de _Main)
| ---------------------------------------------------------------------------
|
        .globl  SceneLoader_ByIndex_043562
        .type   SceneLoader_ByIndex_043562, @function
        .section .text.SceneLoader_ByIndex_043562, "ax", @progbits
SceneLoader_ByIndex_043562:
        clr.w   d0                             | +00  d0 = 0  (fuerza escena 0)
        bra.w   .Lfetch_desc                   | +02  goto $043574

|
| ---------------------------------------------------------------------------
|  SceneLoader_Main_043568  @ $043568  (382 bytes)
| ---------------------------------------------------------------------------
|
        .globl  SceneLoader_Main_043568
        .type   SceneLoader_Main_043568, @function
        .section .text.SceneLoader_Main_043568, "ax", @progbits
SceneLoader_Main_043568:
        andi.w  #0xff, d0                      | +00  d0 = idx & $FF
        lsl.w   #0x3, d0                       | +04  d0 <<= 3  (=idx*8)
        lea.l   0x916c8.l, a0                  | +06  a0 = &scene_table[0]
.Lfetch_desc:                                  | $043574
        move.l  (a0, d0.w), 0x10815c.l         | +0c  cfg_ptr = desc.config_ptr
        movea.l 0x4(a0, d0.w), a0              | +14  a0 = desc.entry_script
        move.l  a0, -(a7)                      | +18  push script base
        bsr.w   Reset4CameraLongs_043D6C       | +1a  clr scratch camera
        movea.l (a7), a0                       | +1e  peek script base
        moveq   #0x0, d1                       | +20  d1 = 0
        move.l  d1, d2                         | +22  d2 = 0
.Lpass1_loop:                                  | $04358C
        cmpi.b  #0x2, (a0)                     | +24  if (type == 2)
        beq.w   .Lpass1_end                    | +28    goto end pasada 1
        move.b  (a0), d0                       | +2c  d0 = type
        cmpi.b  #0x0, d0                       | +2e  if (type == 0)
        bne.w   .Lpass1_typeB                  | +32
        addi.w  #0x20, d1                      | +36    d1 += $20
        bra.w   .Lpass1_next                   | +3a
.Lpass1_typeB:                                 | $0435A6
        addi.w  #0x20, d2                      | +3e    d2 += $20
.Lpass1_next:                                  | $0435AA
        adda.w  #0xe, a0                       | +42  a0 += sizeof(SceneEntry)
        bra.b   .Lpass1_loop                   | +46
.Lpass1_end:                                   | $0435B0
        move.w  d2, d0                         | +48  d0 = counter B
        moveq   #0x0, d2                       | +4a
        move.b  0x1(a0), d2                    | +4c  d2 = subop del terminador
        jsr     Scratch_Alloc_01390E           | +50  (jsr abs.l, 6 B)
        movea.l (a7)+, a0                      | +56  pop; a0 = script base
.Lpass2_loop:                                  | $0435C0
        cmpi.b  #0x2, (a0)                     | +58  if (type == 2)
        beq.w   .Lpass2_end                    | +5c    goto post-load
        move.l  a0, -(a7)                      | +60  push script cursor
        moveq   #0x20, d0                      | +62  d0 = $20 (unused?)
        cmpi.b  #0x0, (a0)                     | +64  if (type == 0)
        bne.w   .Lpass2_typeB                  | +68
        jsr     Spawn_TypeA_013982             | +6c    (jsr abs.l)
        bra.w   .Lpass2_after_spawn            | +72
.Lpass2_typeB:                                 | $0435DE
        jsr     Spawn_TypeB_013952             | +76    (jsr abs.l)
.Lpass2_after_spawn:                           | $0435E4
        movea.l (a7), a2                       | +7c  peek; a2 = script cursor
        movea.l 0x2(a2), a0                    | +7e  a0 = entry.template_ptr
        lea.l   0x6(a2), a1                    | +82  a1 = &entry.payload[0]
        jsr     Entity_AllocAndInit_051ABE     | +86  (jsr abs.l)
        move.w  #0x100, 0x74(a0)               | +8c  scale_x = $100
        move.w  #0x100, 0x76(a0)               | +92  scale_y = $100
        clr.b   0x72(a0)                       | +98  flags = 0
        clr.l   0x78(a0)                       | +9c  linked = NULL
        movea.l (a7)+, a0                      | +a0  pop; restore a0
        adda.w  #0xe, a0                       | +a2  a0 += sizeof(SceneEntry)
        bra.b   .Lpass2_loop                   | +a6
.Lpass2_end:                                   | $043610
        clr.w   0x106f5e.l                     | +a8  camera_flag = 0
        clr.l   0x106f50.l                     | +ae  camera0_x = 0
        clr.l   0x106f54.l                     | +b4  camera0_y = 0
        clr.l   0x106f58.l                     | +ba  camera0_z = 0
        clr.w   0x108160.l                     | +c0  hud_x = 0
        clr.w   0x108162.l                     | +c6  hud_y = 0
        move.w  #0xa0,   0x108164.l            | +cc  hud_center_x = $A0
        move.w  #0x180,  0x108166.l            | +d4  hud_center_y = $180
        move.w  #0x0,    0x108168.l            | +dc  hud_scroll_x = 0
        move.w  #0x0,    0x10816a.l            | +e4  hud_scroll_y = 0
        move.w  #0xffff, 0x10816c.l            | +ec  hud_bound_min = -1
        move.w  #0xffff, 0x10816e.l            | +f4  hud_bound_max = -1
        move.w  #0x0,    0x108170.l            | +fc  hud_extra = 0
        clr.w   0x106f5c.l                     | +104 camera1_flag = 0
        clr.b   0x108179.l                     | +10a hud_gate = 0
        move.w  0x106f5c.l, 0x108172.l         | +110 hud_cam1 = 0
        move.w  0x106f50.l, 0x108174.l         | +11a hud_cam0hi = 0
        move.w  0x106f58.l, 0x108176.l         | +124 hud_cam0z = 0
        lea.l   0x106f6c.l, a0                 | +12e a0 = &camera_scratch
        bsr.w   Buffer_ClearBlock1024L_043EDA  | +134 (bsr.w a $43EDA, W#CC1)
        bsr.w   Camera_ResetSmoothing_0434EA   | +138 (bsr.w a $434EA)
        clr.b   0x10e39c.l                     | +13c current_level     = 0
        clr.b   0x10e39d.l                     | +142 current_sublevel  = 0
        clr.w   0x106f42.l                     | +148 misc_flag         = 0
        move.b  #0x1,    0x106ed3.l            | +14e system_state      = 1
        jsr     Subsystem_HudInit_07707C       | +156 (jsr abs.l)
        jsr     Subsystem_AudioSceneInit_08F158| +15c (jsr abs.l)
        jsr     Subsystem_ScoresInit_03EE3A    | +162 (jsr abs.l)
        jsr     Subsystem_AttractHookInit_997B8| +168 (jsr abs.l)
        jsr     Subsystem_MiscInit_04CB5C      | +16e (jsr abs.l)
        rts                                    | +174

        .size   SceneLoader_Main_043568, .-SceneLoader_Main_043568
