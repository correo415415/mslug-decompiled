| ============================================================================
|  Metal Slug 1 - asm/player_dispatch_0335xx.s
|  ----------------------------------------------------------------------------
|  Wave NN batch 2 - dispatcher por indice + constructor de entity.
|  Cierra el grafo de PlayerRoute_PublishState_033522 (Wave NN batch 1).
|
|  Contenido (2 funciones, 190 bytes):
|
|      $033572   PlayerStateDispatch_033572    52 B  dispatcher doble indice
|      $0335A6   PlayerEntitySpawn_0335A6     138 B  constructor de entity
|
|  ---------- Mapa de callers -----------------------------------------------
|
|      PlayerRoute_PublishState_033522 (NN#1) -> lea $33572(pc), a1; (a6)=a1
|      PlayerRoute_PublishState_033522 (NN#1) -> lea $33578(pc), a1; (a6)=a1
|      PlayerStateDispatch_033572 -> jmp (a0)  (transfiere control al handler
|                                                seleccionado por la LUT)
|      PlayerEntitySpawn_0335A6 cae por fall-through en SetTaskHandler_033630
|      (Wave H, ya matcheado).
|
|  ---------- Descubrimientos arquitectonicos Wave NN batch 2 ---------------
|
|  1. **Dispatcher de doble indice con LUT anidada**. PlayerStateDispatch
|     usa d7 (0=PlayerHandlerA, 1=PlayerHandlerB, publicado por NN#1) como
|     indice en una LUT de 2 punteros en $3349A, y luego $58(a6) (slot
|     state, clampado a [0, $22] -> [0, $F]) como indice en una LUT de 16
|     punteros dentro del handler seleccionado. Resultado: 32 handlers
|     posibles organizados en 2x16 = tabla bi-dimensional.
|
|     Idioma "doble deref con jmp (a0)" ya visto en el bytecode virtual
|     continuation-passing de MM#1 pero aqui con LUT de punteros en
|     lugar de tabla de scripts. Es el State Pattern del juego con 32
|     estados por player.
|
|  2. **Clamp de slot state**: $58(a6) puede valer hasta $22 (34 decimales)
|     pero la LUT solo tiene 16 entries (indices 0..$F). El clamp
|     `cmpi.w #$22, d0; bcs.w skip; move.w #$F, d0` garantiza que indices
|     fuera de rango caigan al ultimo handler (probablemente un handler
|     "idle" o "error").
|
|  3. **Constructor de entity completo**. PlayerEntitySpawn inicializa un
|     TCB de player con:
|       - coords $22=$60, $24=$171 (posicion inicial del player)
|       - 3x jsr $236E (SpriteSetup) con d1=$176/$190/$191 (3 sprites:
|         body + 2 accesorios)
|       - $1C(a6) = $1B (tipo de entity)
|       - jsr $138FE (probable carga de config de nivel)
|       - jsr $32A02(pc) (helper local, pc-rel)
|       - bclr bit 4 de $12(a6) (flag de pausa)
|       - ori.w #$2, $38(a6) (flag de "spawn activo")
|       - lea $394A8, a1; jsr $4AE (Task_Alloc con template $394A8)
|       - jsr $5DD02 (Entity_CopyTransform, Wave S)
|       - jsr $517FE (probable init de sprites hardware)
|       - clr de 6 fields ($28-$2E, $26, $27) = reset de coords/flags
|       - bset bit 6 de $13(a6) = flag "entity lista"
|       - jsr $32FF2(pc) + jsr $27BC8 + jsr $32AA8(pc) = post-init hooks
|       - Fall-through natural a SetTaskHandler_033630 (Wave H):
|         lea $36D64(pc), a1; move.l a1, (a6); rts
|     Es la funcion de "spawn del player" que se ejecuta cuando el
|     juego arranca una nueva vida o el player respawnea.
|
|  4. **10a aparicion del idioma fall-through** (y 4a hacia matcheada
|     previa): PlayerEntitySpawn cae por fall-through directo en
|     SetTaskHandler_033630 (Wave H), que hace `lea $36D64(pc), a1;
|     move.l a1, (a6); rts` = publica el siguiente handler y retorna.
|     El constructor no termina con rts propio sino que delega el
|     retorno al SetTaskHandler vecino.
|
|  5. **LUT en $3349A**: 2 punteros u32 BE que apuntan a las 2 tablas
|     de 16 handlers cada una (P1 y P2). Se registra como external
|     en symbols.py para que el `lea $3349A(pc)` resuelva correctamente
|     (leccion MM#1: PC-rel con literal numerico NO deja reubicacion).
|
|  Toolchain: m68k-linux-gnu-gcc -m68000 -Wa,--register-prefix-optional
|  ============================================================================

        .text

|
| ---------------------------------------------------------------------------
|  PlayerStateDispatch_033572  @ $033572  (52 bytes)
|
|  Dispatcher de doble indice: d7 selecciona tabla P1/P2, $58(a6)
|  selecciona handler dentro de la tabla. jmp (a0) al handler final.
|
|  Es invocado como handler publicado por PlayerRoute_PublishState (NN#1)
|  en (a6): PlayerHandlerA_033572 (d7=0) o PlayerHandlerB_033578 (d7=1).
| ---------------------------------------------------------------------------
|
        .globl  PlayerStateDispatch_033572
        .globl  PlayerHandlerA_033572           | alias publicado por NN#1
        .globl  PlayerHandlerB_033578           | alias publicado por NN#1
        .type   PlayerStateDispatch_033572, @function
        .section .text.PlayerStateDispatch_033572, "ax", @progbits
PlayerStateDispatch_033572:
PlayerHandlerA_033572:
        moveq   #0x0, d7                        | +00  d7 = 0 (ruta A / P1)
        bra.w   .Ldispatch_resolve              | +02
PlayerHandlerB_033578:
        moveq   #0x1, d7                        | +06  d7 = 1 (ruta B / P2)
        bra.w   .Ldispatch_resolve              | +08
.Ldispatch_resolve:                             | $03357E
        andi.w  #0xff, d7                       | +0c  d7 &= 0xFF (sanitize)
        lsl.w   #0x2, d7                        | +10  d7 <<= 2 (index * 4)
        lea.l   PlayerStateLUT_03349A(pc), a0   | +12  a0 = &LUT_2ptrs[$3349A]
        movea.l (a0, d7.w), a0                  | +16  a0 = LUT[d7] (tabla P1 o P2)
        moveq   #0x0, d0                        | +1a  d0 = 0
        move.b  0x58(a6), d0                    | +1c  d0 = slot_state
        cmpi.w  #0x22, d0                       | +20  if d0 >= $22
        bcs.w   .Ldispatch_clamp_ok             | +24    skip clamp
        move.w  #0xf, d0                        | +28  d0 = $F (clamp to last)
.Ldispatch_clamp_ok:                            | $03359E
        lsl.w   #0x2, d0                        | +2c  d0 <<= 2 (index * 4)
        movea.l (a0, d0.w), a0                  | +2e  a0 = tabla[slot_state]
        jmp     (a0)                            | +32  jump to selected handler

        .size   PlayerStateDispatch_033572, .-PlayerStateDispatch_033572

|
| ---------------------------------------------------------------------------
|  PlayerEntitySpawn_0335A6  @ $0335A6  (138 bytes)
|
|  Constructor de entity de player. Inicializa coords, sprites, flags,
|  asigna task con template $394A8, copia transform, resetea campos, y
|  cae por fall-through en SetTaskHandler_033630 (Wave H) que publica
|  el siguiente handler en (a6) y retorna.
|
|  Es invocado por PlayerStateDispatch via jmp (a0) cuando slot_state
|  selecciona este handler como "spawn inicial".
| ---------------------------------------------------------------------------
|
        .globl  PlayerEntitySpawn_0335A6
        .type   PlayerEntitySpawn_0335A6, @function
        .section .text.PlayerEntitySpawn_0335A6, "ax", @progbits
PlayerEntitySpawn_0335A6:
        move.w  #0x60, 0x22(a6)                 | +00  x_coord = $60
        move.w  #0x171, 0x24(a6)                | +06  y_coord = $171
        move.w  #0x176, d1                      | +0c  d1 = sprite_id body
        jsr     ThunkTarget_00236e              | +10  SpriteSetup (body)
        move.w  #0x190, d1                      | +16  d1 = sprite_id acc1
        jsr     ThunkTarget_00236e              | +1a  SpriteSetup (acc1)
        move.w  #0x191, d1                      | +20  d1 = sprite_id acc2
        jsr     ThunkTarget_00236e              | +24  SpriteSetup (acc2)
        move.w  #0x1b, 0x1c(a6)                 | +2a  entity_type = $1B (player)
        jsr     ThunkTarget_0138fe              | +30  load level config
        jsr     PlayerEntity_InitAuxState_032A02(pc) | +36  Wave QQ#1
        bclr.b  #0x4, 0x12(a6)                  | +3a  clear pause flag
        ori.w   #0x2, 0x38(a6)                  | +40  set spawn-active flag
        lea.l   TaskTpl_0394A8, a1              | +46  a1 = &task_tpl_$394A8
        jsr     ThunkTarget_0004ae              | +4c  Task_Alloc
        jsr     ThunkTarget_05dd02              | +52  Entity_CopyTransform (S)
        jsr     ThunkTarget_0517fe              | +58  init sprites hardware
        clr.w   0x28(a6)                        | +5e  reset field $28
        clr.w   0x2a(a6)                        | +62  reset field $2A
        clr.w   0x2c(a6)                        | +66  reset field $2C
        clr.w   0x2e(a6)                        | +6a  reset field $2E
        clr.b   0x26(a6)                        | +6e  reset field $26
        clr.b   0x27(a6)                        | +72  reset field $27
        bset.b  #0x6, 0x13(a6)                  | +76  set "entity ready" flag
        jsr     Sub_00032FF2(pc)                | +7a  post-init hook 1 (pc-rel)
        jsr     Sub_00027BC8                    | +7e  post-init hook 2 (abs.l)
        jsr     Sub_00032AA8(pc)                | +84  post-init hook 3 (pc-rel)
| ---- Fall-through natural a SetTaskHandler_033630 (Wave H, ya matcheado):
|      $033630: lea $36D64(pc), a1; move.l a1, (a6); rts
|      No se incluye en esta funcion: los 8 B siguientes pertenecen al
|      SetTaskHandler vecino. 10a aparicion del fall-through del proyecto,
|      4a hacia matcheada previa (tras MM#1 x2 y NN#1 x1).

        .size   PlayerEntitySpawn_0335A6, .-PlayerEntitySpawn_0335A6
