| ============================================================================
|  Metal Slug 1 (Neo Geo, M68000) — decompilación matching
|  Wave YYY — disparo del player por arma, casquillos, escombros, brazo Slug
|  Región: $03C62A..$03DA98  (5,230 B, 82 entradas, 1 huecos)
| ============================================================================
|
|  A. RESUMEN
|  ----------
|  Región de 5,230 B justo detrás de los handlers de brazo (Waves WWW/XXX):
|
|   1. Tres handlers de brazo residuales: PlayerArm_SlugRideA/B_03c632/03c67e
|      (anim $02, montado en el Slug; refs $279F04/$279F18) y
|      PlayerArm_Fall2_03c6c4 ($27968C, anim $26); PlayerArm_JsrAttack_03c62a
|      (`jsr $283CA; rts`, antigua isla C).
|   2. Spawners de proyectil del player, uno por arma (+$71):
|        PlayerFire_Pistol_*   ($03C710): template $3093A vía $6FE, música
|          $10F7, +$7A contador de ráfaga -> +$9A, +$98 = ángulo<<3, +$9B.
|        PlayerFire_HMG_*      ($03C872): template $9C4D4 vía $5EAA4, snd
|          $10F6, decrementa munición +$82 y +$85 del player.
|        PlayerFire_Shotgun_*  ($03C9C4): template $9C25E vía $5EAA4, $10F3.
|        PlayerFire_Rocket_*   ($03CB16): template $9BEC2 vía $5EAB6, $10F5;
|          los `_Fwd/_Up/_Back/_FwdLow` crean antes el casquillo
|          ShellCasing_Rocket_03d4ea/_RocketB_03d4f8.
|        PlayerFire_Flame_*    ($03CCD2): template $308C2 vía $6FE, $10F4;
|          variantes por dirección (Fwd/Up/Back/Ang17/Down) x desviación
|          (Neg=-2 / 0 / Pos=+2 en +$9B) y `_SpreadN_M` que emiten dos
|          llamas con ángulos N y M (1..$1F, abanico del lanzallamas).
|      Todos: a2 = player (a6 si es slot $100440/$1004E0, si no +$C);
|      si +$82 (munición) == 0 abortan sacando la dirección de retorno
|      (`movem.l (a7)+,a0; rts` = salta al rts del llamador). El
|      proyectil hereda +$22/+$24 (+$1C/+$14 de offset Y) y +$38; +$98/+$99
|      = ángulo y signo (facing +$3A).
|   3. ShellCasing_Pistol_03d396 (snd $184, $28134 con $D000, prio +$38
|      |= $10 + $200, anim ShellCasing_Pistol_Anim/AnimB) y
|      ShellCasing_Rocket (snd $17C): casquillos que caen con $2783A.
|   4. Scene3Debris_Spawn*_03d616..03d72a: sólo en escena 3 ($106ECE == 3)
|      crean Scene3Debris_Task_03d76e (snd $1BD, anim $3D7A0) con distintos
|      offsets; referenciados desde datos de nivel $17Axxx/$17Exxx.
|   5. Player_DebugMarker_03d842 (creado por Player_SpawnDebugTask_033358
|      si $10FD8F != 0; snd $17A) y Player_SpawnFx3D8FA_03d8fa (snd $17C,
|      anim $2F7D3A) creados desde player_core.
|   6. SlugCannon_ArmOverlay_03d944: brazo del player montado en el Slug
|      ($100580): elige slot vivo ($2AC0E), snd $176/$177+$190+$191/$192,
|      tabla SlugCannon_ArmSpriteTbl_03da02 por arma/jugador, offset por
|      $2FF8E y tabla Sub_0003DAA8 (siguiente región), sigue la posición
|      del Slug; al terminar música $108B.
|
|  B. EVIDENCIAS
|  -------------
|   - Refs ROM de los PlayerFire_*: $17Bxxx (pistola), $180Bxx (HMG),
|     $1814xx..$1818xx (shotgun), $181Exx..$1822xx (rocket), $17Exxx/
|     $17Fxxx/$180xxx (flame) = tablas de anim del player en banco alto,
|     indexadas por el mismo orden de armas que PlayerArm_SpriteTbl_*
|     (0 pistola, 1 HMG, 2 shotgun, 3 rocket, 4 flame).
|   - Música $10F3..$10F7 = un id por arma; $184/$17C = casquillo.
|   - player_core: `lea $3D842,a1; jsr $4AE` tras `tst.b $10FD8F`; `lea
|     $3D8FA,a1`.
|
|  C. CAMPOS (a2 = player, a0 = proyectil nuevo)
|  ---------------------------------------------
|   player: +$3A facing, +$71 arma, +$7A índice de ráfaga, +$82 munición,
|   +$85 cadencia. proyectil: +$22/+$24 pos, +$38 prio, +$98 ángulo<<3 /
|   tipo, +$99 signo ($00/$FF), +$9A ráfaga, +$9B desviación ($FE/0/2).
|
|  D. HELPERS EXTERNOS
|  -------------------
|   $4AE alloc, $518 free, $6FE/$5EAA4/$5EAB6 alloc desde template,
|   $517FE Entity_CopyField68AndCall, $236E snd, $2352 music, $28134
|   setup física proyectil, $283CA ataque, $28CD4 sprite, $28D70 paso
|   anim, $2783A física, $2AC0E slot vivo, $2FF8E, $5DD02 copia transform.
|
|  E. HIPÓTESIS ABIERTAS
|  ---------------------
|   - Nombres de arma (Pistol/HMG/Shotgun/Rocket/Flame) por el orden 0..4
|     del índice +$71; el id 4 podría ser el lanzallamas o la escopeta
|     según la convención de MS1 (HMG=1, Rocket=2, Flame=3, Shotgun=4?).
|   - "Scene3Debris" es provisional (entidad decorativa de la escena 3).
|
|  F. SIGUIENTE
|  ------------
|   `$03DAA8..` (tabla de offsets del cañón + islas), `$02E000..$032A00`.
|
|  Verificación: cada sección .text.<Sym> se coloca en su dirección CPU
|  absoluta y reensambla byte-exacta contra build/mslug_prom.bin
|  (MD5 816b3f74c76b3373993407615f1850fe).
| ============================================================================

        .text

| ----------------------------------------------------------------------------
|  PlayerArm_JsrAttack_03c62a  @ $03C62A  (8 B)
| ----------------------------------------------------------------------------
        .section .text.PlayerArm_JsrAttack_03c62a, "ax", @progbits
        .global PlayerArm_JsrAttack_03c62a
PlayerArm_JsrAttack_03c62a:
        jsr     0x283ca.l                       | +000
        rts                                     | +006

| ----------------------------------------------------------------------------
|  PlayerArm_SlugRideA_03c632  @ $03C632  (34 B)
| ----------------------------------------------------------------------------
        .section .text.PlayerArm_SlugRideA_03c632, "ax", @progbits
        .global PlayerArm_SlugRideA_03c632
PlayerArm_SlugRideA_03c632:
        bclr    #0x2,0x8c(a6)                   | +000
        bclr    #0x2,0x8c(a6)                   | +006
        jsr     PlayerArm_WeaponTableIndex_03933a(pc) | +00c
        lea     PlayerArm_SpriteTbl_03c654(pc),a0 | +010
        movea.l (a0,d0.w),a0                    | +014
        jsr     0x28cd4.l                       | +018
        bra.w   PlayerArm_SpriteTbl_03c654__L03c67c | +01e

| ----------------------------------------------------------------------------
|  PlayerArm_SpriteTbl_03c654  @ $03C654  (42 B)
| ----------------------------------------------------------------------------
        .section .text.PlayerArm_SpriteTbl_03c654, "ax", @progbits
        .global PlayerArm_SpriteTbl_03c654
PlayerArm_SpriteTbl_03c654:
        .dc.w   0x0027                        | +000  (dato / opcode no decodificado)
        .dc.w   0xc7e4                        | +002  (dato / opcode no decodificado)
        .dc.w   0x0027                        | +004  (dato / opcode no decodificado)
        .dc.w   0xfe26                        | +006  (dato / opcode no decodificado)
        .dc.w   0x0027                        | +008  (dato / opcode no decodificado)
        .dc.w   0xfe26                        | +00a  (dato / opcode no decodificado)
        .dc.w   0x0027                        | +00c  (dato / opcode no decodificado)
        .dc.w   0xfe26                        | +00e  (dato / opcode no decodificado)
        .dc.w   0x0027                        | +010  (dato / opcode no decodificado)
        .dc.w   0xfe26                        | +012  (dato / opcode no decodificado)
        .dc.w   0x0027                        | +014  (dato / opcode no decodificado)
        .dc.w   0xc7e4                        | +016  (dato / opcode no decodificado)
        .dc.w   0x0027                        | +018  (dato / opcode no decodificado)
        .dc.w   0xfe26                        | +01a  (dato / opcode no decodificado)
        .dc.w   0x0027                        | +01c  (dato / opcode no decodificado)
        .dc.w   0xfe26                        | +01e  (dato / opcode no decodificado)
        .dc.w   0x0027                        | +020  (dato / opcode no decodificado)
        .dc.w   0xfe26                        | +022  (dato / opcode no decodificado)
        .dc.w   0x0027                        | +024  (dato / opcode no decodificado)
        .dc.w   0xfe26                        | +026  (dato / opcode no decodificado)
        .global PlayerArm_SpriteTbl_03c654__L03c67c
PlayerArm_SpriteTbl_03c654__L03c67c:
        rts                                     | +028

| ----------------------------------------------------------------------------
|  PlayerArm_SlugRideB_03c67e  @ $03C67E  (28 B)
| ----------------------------------------------------------------------------
        .section .text.PlayerArm_SlugRideB_03c67e, "ax", @progbits
        .global PlayerArm_SlugRideB_03c67e
PlayerArm_SlugRideB_03c67e:
        bclr    #0x2,0x8c(a6)                   | +000
        jsr     PlayerArm_WeaponTableIndex_03933a(pc) | +006
        lea     PlayerArm_SpriteTbl_03c69a(pc),a0 | +00a
        movea.l (a0,d0.w),a0                    | +00e
        jsr     0x28cd4.l                       | +012
        bra.w   PlayerArm_SpriteTbl_03c69a__L03c6c2 | +018

| ----------------------------------------------------------------------------
|  PlayerArm_SpriteTbl_03c69a  @ $03C69A  (42 B)
| ----------------------------------------------------------------------------
        .section .text.PlayerArm_SpriteTbl_03c69a, "ax", @progbits
        .global PlayerArm_SpriteTbl_03c69a
PlayerArm_SpriteTbl_03c69a:
        .dc.w   0x0027                        | +000  (dato / opcode no decodificado)
        .dc.w   0xc800                        | +002  (dato / opcode no decodificado)
        .dc.w   0x0027                        | +004  (dato / opcode no decodificado)
        .dc.w   0xfe42                        | +006  (dato / opcode no decodificado)
        .dc.w   0x0027                        | +008  (dato / opcode no decodificado)
        .dc.w   0xfe42                        | +00a  (dato / opcode no decodificado)
        .dc.w   0x0027                        | +00c  (dato / opcode no decodificado)
        .dc.w   0xfe42                        | +00e  (dato / opcode no decodificado)
        .dc.w   0x0027                        | +010  (dato / opcode no decodificado)
        .dc.w   0xfe42                        | +012  (dato / opcode no decodificado)
        .dc.w   0x0027                        | +014  (dato / opcode no decodificado)
        .dc.w   0xc800                        | +016  (dato / opcode no decodificado)
        .dc.w   0x0027                        | +018  (dato / opcode no decodificado)
        .dc.w   0xfe42                        | +01a  (dato / opcode no decodificado)
        .dc.w   0x0027                        | +01c  (dato / opcode no decodificado)
        .dc.w   0xfe42                        | +01e  (dato / opcode no decodificado)
        .dc.w   0x0027                        | +020  (dato / opcode no decodificado)
        .dc.w   0xfe42                        | +022  (dato / opcode no decodificado)
        .dc.w   0x0027                        | +024  (dato / opcode no decodificado)
        .dc.w   0xfe42                        | +026  (dato / opcode no decodificado)
        .global PlayerArm_SpriteTbl_03c69a__L03c6c2
PlayerArm_SpriteTbl_03c69a__L03c6c2:
        rts                                     | +028

| ----------------------------------------------------------------------------
|  PlayerArm_Fall2_03c6c4  @ $03C6C4  (34 B)
| ----------------------------------------------------------------------------
        .section .text.PlayerArm_Fall2_03c6c4, "ax", @progbits
        .global PlayerArm_Fall2_03c6c4
PlayerArm_Fall2_03c6c4:
        bclr    #0x2,0x8c(a6)                   | +000
        bclr    #0x2,0x8c(a6)                   | +006
        jsr     PlayerArm_WeaponTableIndex_03933a(pc) | +00c
        lea     PlayerArm_SpriteTbl_03c6e6(pc),a0 | +010
        movea.l (a0,d0.w),a0                    | +014
        jsr     0x28cd4.l                       | +018
        bra.w   PlayerArm_SpriteTbl_03c6e6__L03c70e | +01e

| ----------------------------------------------------------------------------
|  PlayerArm_SpriteTbl_03c6e6  @ $03C6E6  (42 B)
| ----------------------------------------------------------------------------
        .section .text.PlayerArm_SpriteTbl_03c6e6, "ax", @progbits
        .global PlayerArm_SpriteTbl_03c6e6
PlayerArm_SpriteTbl_03c6e6:
        .dc.w   0x0027                        | +000  (dato / opcode no decodificado)
        .dc.w   0xa30c                        | +002  (dato / opcode no decodificado)
        .dc.w   0x0027                        | +004  (dato / opcode no decodificado)
        .dc.w   0xa30c                        | +006  (dato / opcode no decodificado)
        .dc.w   0x0027                        | +008  (dato / opcode no decodificado)
        .dc.w   0xa30c                        | +00a  (dato / opcode no decodificado)
        .dc.w   0x0027                        | +00c  (dato / opcode no decodificado)
        .dc.w   0xa30c                        | +00e  (dato / opcode no decodificado)
        .dc.w   0x0027                        | +010  (dato / opcode no decodificado)
        .dc.w   0xa30c                        | +012  (dato / opcode no decodificado)
        .dc.w   0x0027                        | +014  (dato / opcode no decodificado)
        .dc.w   0xa30c                        | +016  (dato / opcode no decodificado)
        .dc.w   0x0027                        | +018  (dato / opcode no decodificado)
        .dc.w   0xa30c                        | +01a  (dato / opcode no decodificado)
        .dc.w   0x0027                        | +01c  (dato / opcode no decodificado)
        .dc.w   0xa30c                        | +01e  (dato / opcode no decodificado)
        .dc.w   0x0027                        | +020  (dato / opcode no decodificado)
        .dc.w   0xa30c                        | +022  (dato / opcode no decodificado)
        .dc.w   0x0027                        | +024  (dato / opcode no decodificado)
        .dc.w   0xa30c                        | +026  (dato / opcode no decodificado)
        .global PlayerArm_SpriteTbl_03c6e6__L03c70e
PlayerArm_SpriteTbl_03c6e6__L03c70e:
        rts                                     | +028

| ----------------------------------------------------------------------------
|  PlayerFire_Pistol_Spawn_03c710  @ $03C710  (102 B)
| ----------------------------------------------------------------------------
        .section .text.PlayerFire_Pistol_Spawn_03c710, "ax", @progbits
        .global PlayerFire_Pistol_Spawn_03c710
PlayerFire_Pistol_Spawn_03c710:
        cmpa.l  #0x100440,a6                    | +000
        beq.b   .L03c722                        | +006
        cmpa.l  #0x1004e0,a6                    | +008
        bne.w   .L03c728                        | +00e
.L03c722:
        movea.l a6,a2                           | +012
        bra.w   .L03c72c                        | +014
.L03c728:
        movea.l 0xc(a6),a2                      | +018
.L03c72c:
        addq.b  #0x1,0x7a(a6)                   | +01c
        cmpi.w  #0x0,0x82(a2)                   | +020
        beq.w   .L03c740                        | +026
        subi.w  #0x1,0x82(a2)                   | +02a
.L03c740:
        move.w  #0x10f7,d0                      | +030
        jsr     0x2352.l                        | +034
        lea     0x3093a.l,a1                    | +03a
        jsr     0x6fe.l                         | +040
        move.w  0x22(a2),0x22(a0)               | +046
        move.w  0x24(a2),0x24(a0)               | +04c
        move.w  0x38(a2),0x38(a0)               | +052
        jsr     0x517fe.l                       | +058
        addi.w  #0x1c,0x24(a0)                  | +05e
        rts                                     | +064

| ----------------------------------------------------------------------------
|  PlayerFire_Pistol_Fwd_03c776  @ $03C776  (52 B)
| ----------------------------------------------------------------------------
        .section .text.PlayerFire_Pistol_Fwd_03c776, "ax", @progbits
        .global PlayerFire_Pistol_Fwd_03c776
PlayerFire_Pistol_Fwd_03c776:
        jsr     PlayerFire_Pistol_Spawn_03c710(pc) | +000
        btst    #0x0,0x3a(a6)                   | +004
        bne.w   .L03c790                        | +00a
        move.b  #0x0,d0                         | +00e
        move.b  #0x0,d1                         | +012
        bra.w   .L03c798                        | +016
.L03c790:
        move.b  #0x0,d0                         | +01a
        move.b  #0x2,d1                         | +01e
.L03c798:
        lsl.b   #0x3,d0                         | +022
        move.b  d0,0x98(a0)                     | +024
        move.b  0x7a(a6),0x9a(a0)               | +028
        clr.b   0x9b(a0)                        | +02e
        rts                                     | +032

| ----------------------------------------------------------------------------
|  PlayerFire_Pistol_Up_03c7aa  @ $03C7AA  (30 B)
| ----------------------------------------------------------------------------
        .section .text.PlayerFire_Pistol_Up_03c7aa, "ax", @progbits
        .global PlayerFire_Pistol_Up_03c7aa
PlayerFire_Pistol_Up_03c7aa:
        jsr     PlayerFire_Pistol_Spawn_03c710(pc) | +000
        move.b  #0x8,d0                         | +004
        move.b  #0x1,d1                         | +008
        lsl.b   #0x3,d0                         | +00c
        move.b  d0,0x98(a0)                     | +00e
        move.b  0x7a(a6),0x9a(a0)               | +012
        clr.b   0x9b(a0)                        | +018
        rts                                     | +01c

| ----------------------------------------------------------------------------
|  PlayerFire_Pistol_Back_03c7c8  @ $03C7C8  (52 B)
| ----------------------------------------------------------------------------
        .section .text.PlayerFire_Pistol_Back_03c7c8, "ax", @progbits
        .global PlayerFire_Pistol_Back_03c7c8
PlayerFire_Pistol_Back_03c7c8:
        jsr     PlayerFire_Pistol_Spawn_03c710(pc) | +000
        btst    #0x0,0x3a(a6)                   | +004
        beq.w   .L03c7e2                        | +00a
        move.b  #0x10,d0                        | +00e
        move.b  #0x0,d1                         | +012
        bra.w   .L03c7ea                        | +016
.L03c7e2:
        move.b  #0x10,d0                        | +01a
        move.b  #0x2,d1                         | +01e
.L03c7ea:
        lsl.b   #0x3,d0                         | +022
        move.b  d0,0x98(a0)                     | +024
        move.b  0x7a(a6),0x9a(a0)               | +028
        clr.b   0x9b(a0)                        | +02e
        rts                                     | +032

| ----------------------------------------------------------------------------
|  PlayerFire_Pistol_FwdLow_03c7fc  @ $03C7FC  (58 B)
| ----------------------------------------------------------------------------
        .section .text.PlayerFire_Pistol_FwdLow_03c7fc, "ax", @progbits
        .global PlayerFire_Pistol_FwdLow_03c7fc
PlayerFire_Pistol_FwdLow_03c7fc:
        jsr     PlayerFire_Pistol_Spawn_03c710(pc) | +000
        subi.w  #0xc,0x24(a0)                   | +004
        btst    #0x0,0x3a(a6)                   | +00a
        bne.w   .L03c81c                        | +010
        move.b  #0x0,d0                         | +014
        move.b  #0x0,d1                         | +018
        bra.w   .L03c824                        | +01c
.L03c81c:
        move.b  #0x0,d0                         | +020
        move.b  #0x2,d1                         | +024
.L03c824:
        lsl.b   #0x3,d0                         | +028
        move.b  d0,0x98(a0)                     | +02a
        move.b  0x7a(a6),0x9a(a0)               | +02e
        clr.b   0x9b(a0)                        | +034
        rts                                     | +038

| ----------------------------------------------------------------------------
|  PlayerFire_Pistol_Down_03c836  @ $03C836  (30 B)
| ----------------------------------------------------------------------------
        .section .text.PlayerFire_Pistol_Down_03c836, "ax", @progbits
        .global PlayerFire_Pistol_Down_03c836
PlayerFire_Pistol_Down_03c836:
        jsr     PlayerFire_Pistol_Spawn_03c710(pc) | +000
        move.b  #0x18,d0                        | +004
        move.b  #0x0,d1                         | +008
        lsl.b   #0x3,d0                         | +00c
        move.b  d0,0x98(a0)                     | +00e
        move.b  0x7a(a6),0x9a(a0)               | +012
        clr.b   0x9b(a0)                        | +018
        rts                                     | +01c

| ----------------------------------------------------------------------------
|  PlayerFire_Pistol_DownB_03c854  @ $03C854  (30 B)
| ----------------------------------------------------------------------------
        .section .text.PlayerFire_Pistol_DownB_03c854, "ax", @progbits
        .global PlayerFire_Pistol_DownB_03c854
PlayerFire_Pistol_DownB_03c854:
        jsr     PlayerFire_Pistol_Spawn_03c710(pc) | +000
        move.b  #0x18,d0                        | +004
        move.b  #0x0,d1                         | +008
        lsl.b   #0x3,d0                         | +00c
        move.b  d0,0x98(a0)                     | +00e
        move.b  0x7a(a6),0x9a(a0)               | +012
        clr.b   0x9b(a0)                        | +018
        rts                                     | +01c

| ----------------------------------------------------------------------------
|  PlayerFire_HMG_Spawn_03c872  @ $03C872  (108 B)
| ----------------------------------------------------------------------------
        .section .text.PlayerFire_HMG_Spawn_03c872, "ax", @progbits
        .global PlayerFire_HMG_Spawn_03c872
PlayerFire_HMG_Spawn_03c872:
        cmpa.l  #0x100440,a6                    | +000
        beq.b   .L03c884                        | +006
        cmpa.l  #0x1004e0,a6                    | +008
        bne.w   .L03c88a                        | +00e
.L03c884:
        movea.l a6,a2                           | +012
        bra.w   .L03c88e                        | +014
.L03c88a:
        movea.l 0xc(a6),a2                      | +018
.L03c88e:
        cmpi.w  #0x0,0x82(a2)                   | +01c
        beq.w   .L03c8d8                        | +022
        subi.w  #0x1,0x82(a2)                   | +026
        cmpi.b  #0x0,0x85(a2)                   | +02c
        beq.w   .L03c8ae                        | +032
        subi.b  #0x1,0x85(a2)                   | +036
.L03c8ae:
        move.w  #0x10f6,d0                      | +03c
        jsr     0x2352.l                        | +040
        lea     0x9c4d4.l,a1                    | +046
        jsr     0x5eaa4.l                       | +04c
        move.w  0x22(a2),0x22(a0)               | +052
        move.w  0x24(a2),0x24(a0)               | +058
        jsr     0x517fe.l                       | +05e
        rts                                     | +064
.L03c8d8:
        movem.l (a7)+,a0                        | +066
        rts                                     | +06a

| ----------------------------------------------------------------------------
|  PlayerFire_HMG_Fwd_03c8de  @ $03C8DE  (38 B)
| ----------------------------------------------------------------------------
        .section .text.PlayerFire_HMG_Fwd_03c8de, "ax", @progbits
        .global PlayerFire_HMG_Fwd_03c8de
PlayerFire_HMG_Fwd_03c8de:
        jsr     PlayerFire_HMG_Spawn_03c872(pc) | +000
        btst    #0x0,0x3a(a6)                   | +004
        bne.w   .L03c8f4                        | +00a
        move.b  #0x0,d0                         | +00e
        bra.w   .L03c8f8                        | +012
.L03c8f4:
        move.b  #0xff,d0                        | +016
.L03c8f8:
        clr.b   d1                              | +01a
        move.b  d1,0x98(a0)                     | +01c
        move.b  d0,0x99(a0)                     | +020
        rts                                     | +024

| ----------------------------------------------------------------------------
|  PlayerFire_HMG_Up_03c904  @ $03C904  (22 B)
| ----------------------------------------------------------------------------
        .section .text.PlayerFire_HMG_Up_03c904, "ax", @progbits
        .global PlayerFire_HMG_Up_03c904
PlayerFire_HMG_Up_03c904:
        jsr     PlayerFire_HMG_Spawn_03c872(pc) | +000
        move.b  #0x1,d1                         | +004
        move.b  #0x0,d0                         | +008
        move.b  d1,0x98(a0)                     | +00c
        move.b  d0,0x99(a0)                     | +010
        rts                                     | +014

| ----------------------------------------------------------------------------
|  PlayerFire_HMG_Down_03c91a  @ $03C91A  (40 B)
| ----------------------------------------------------------------------------
        .section .text.PlayerFire_HMG_Down_03c91a, "ax", @progbits
        .global PlayerFire_HMG_Down_03c91a
PlayerFire_HMG_Down_03c91a:
        jsr     PlayerFire_HMG_Spawn_03c872(pc) | +000
        btst    #0x0,0x3a(a6)                   | +004
        bne.w   .L03c930                        | +00a
        move.b  #0x0,d0                         | +00e
        bra.w   .L03c934                        | +012
.L03c930:
        move.b  #0xff,d0                        | +016
.L03c934:
        move.b  #0x2,d1                         | +01a
        move.b  d1,0x98(a0)                     | +01e
        move.b  d0,0x99(a0)                     | +022
        rts                                     | +026

| ----------------------------------------------------------------------------
|  PlayerFire_HMG_FwdB_03c942  @ $03C942  (38 B)
| ----------------------------------------------------------------------------
        .section .text.PlayerFire_HMG_FwdB_03c942, "ax", @progbits
        .global PlayerFire_HMG_FwdB_03c942
PlayerFire_HMG_FwdB_03c942:
        jsr     PlayerFire_HMG_Spawn_03c872(pc) | +000
        btst    #0x0,0x3a(a6)                   | +004
        bne.w   .L03c958                        | +00a
        move.b  #0x0,d0                         | +00e
        bra.w   .L03c95c                        | +012
.L03c958:
        move.b  #0xff,d0                        | +016
.L03c95c:
        clr.b   d1                              | +01a
        move.b  d1,0x98(a0)                     | +01c
        move.b  d0,0x99(a0)                     | +020
        rts                                     | +024

| ----------------------------------------------------------------------------
|  PlayerFire_HMG_Diag_03c968  @ $03C968  (46 B)
| ----------------------------------------------------------------------------
        .section .text.PlayerFire_HMG_Diag_03c968, "ax", @progbits
        .global PlayerFire_HMG_Diag_03c968
PlayerFire_HMG_Diag_03c968:
        jsr     PlayerFire_HMG_Spawn_03c872(pc) | +000
        subi.w  #0xc,0x24(a0)                   | +004
        btst    #0x0,0x3a(a6)                   | +00a
        bne.w   .L03c984                        | +010
        move.b  #0x0,d0                         | +014
        bra.w   .L03c988                        | +018
.L03c984:
        move.b  #0xff,d0                        | +01c
.L03c988:
        move.b  #0x3,d1                         | +020
        move.b  d1,0x98(a0)                     | +024
        move.b  d0,0x99(a0)                     | +028
        rts                                     | +02c

| ----------------------------------------------------------------------------
|  PlayerFire_HMG_DiagB_03c996  @ $03C996  (46 B)
| ----------------------------------------------------------------------------
        .section .text.PlayerFire_HMG_DiagB_03c996, "ax", @progbits
        .global PlayerFire_HMG_DiagB_03c996
PlayerFire_HMG_DiagB_03c996:
        jsr     PlayerFire_HMG_Spawn_03c872(pc) | +000
        subi.w  #0xc,0x24(a0)                   | +004
        btst    #0x0,0x3a(a6)                   | +00a
        bne.w   .L03c9b2                        | +010
        move.b  #0x0,d0                         | +014
        bra.w   .L03c9b6                        | +018
.L03c9b2:
        move.b  #0xff,d0                        | +01c
.L03c9b6:
        move.b  #0x3,d1                         | +020
        move.b  d1,0x98(a0)                     | +024
        move.b  d0,0x99(a0)                     | +028
        rts                                     | +02c

| ----------------------------------------------------------------------------
|  PlayerFire_Shotgun_Spawn_03c9c4  @ $03C9C4  (98 B)
| ----------------------------------------------------------------------------
        .section .text.PlayerFire_Shotgun_Spawn_03c9c4, "ax", @progbits
        .global PlayerFire_Shotgun_Spawn_03c9c4
PlayerFire_Shotgun_Spawn_03c9c4:
        cmpa.l  #0x100440,a6                    | +000
        beq.b   .L03c9d6                        | +006
        cmpa.l  #0x1004e0,a6                    | +008
        bne.w   .L03c9dc                        | +00e
.L03c9d6:
        movea.l a6,a2                           | +012
        bra.w   .L03c9e0                        | +014
.L03c9dc:
        movea.l 0xc(a6),a2                      | +018
.L03c9e0:
        cmpi.w  #0x0,0x82(a2)                   | +01c
        beq.w   .L03ca20                        | +022
        subi.w  #0x1,0x82(a2)                   | +026
        move.w  #0x10f3,d0                      | +02c
        jsr     0x2352.l                        | +030
        lea     0x9c25e.l,a1                    | +036
        jsr     0x5eaa4.l                       | +03c
        move.w  0x22(a2),0x22(a0)               | +042
        move.w  0x24(a2),0x24(a0)               | +048
        jsr     0x517fe.l                       | +04e
        addi.w  #0x1c,0x24(a0)                  | +054
        rts                                     | +05a
.L03ca20:
        movem.l (a7)+,a0                        | +05c
        rts                                     | +060

| ----------------------------------------------------------------------------
|  PlayerFire_Shotgun_Fwd_03ca26  @ $03CA26  (40 B)
| ----------------------------------------------------------------------------
        .section .text.PlayerFire_Shotgun_Fwd_03ca26, "ax", @progbits
        .global PlayerFire_Shotgun_Fwd_03ca26
PlayerFire_Shotgun_Fwd_03ca26:
        jsr     PlayerFire_Shotgun_Spawn_03c9c4(pc) | +000
        btst    #0x0,0x3a(a6)                   | +004
        bne.w   .L03ca3c                        | +00a
        move.b  #0x0,d0                         | +00e
        bra.w   .L03ca40                        | +012
.L03ca3c:
        move.b  #0xff,d0                        | +016
.L03ca40:
        move.b  #0x0,d1                         | +01a
        move.b  d1,0x98(a0)                     | +01e
        move.b  d0,0x99(a0)                     | +022
        rts                                     | +026

| ----------------------------------------------------------------------------
|  PlayerFire_Shotgun_Up_03ca4e  @ $03CA4E  (22 B)
| ----------------------------------------------------------------------------
        .section .text.PlayerFire_Shotgun_Up_03ca4e, "ax", @progbits
        .global PlayerFire_Shotgun_Up_03ca4e
PlayerFire_Shotgun_Up_03ca4e:
        jsr     PlayerFire_Shotgun_Spawn_03c9c4(pc) | +000
        move.b  #0x1,d1                         | +004
        move.b  #0x0,d0                         | +008
        move.b  d1,0x98(a0)                     | +00c
        move.b  d0,0x99(a0)                     | +010
        rts                                     | +014

| ----------------------------------------------------------------------------
|  PlayerFire_Shotgun_Down_03ca64  @ $03CA64  (40 B)
| ----------------------------------------------------------------------------
        .section .text.PlayerFire_Shotgun_Down_03ca64, "ax", @progbits
        .global PlayerFire_Shotgun_Down_03ca64
PlayerFire_Shotgun_Down_03ca64:
        jsr     PlayerFire_Shotgun_Spawn_03c9c4(pc) | +000
        btst    #0x0,0x3a(a6)                   | +004
        bne.w   .L03ca7a                        | +00a
        move.b  #0x0,d0                         | +00e
        bra.w   .L03ca7e                        | +012
.L03ca7a:
        move.b  #0xff,d0                        | +016
.L03ca7e:
        move.b  #0x2,d1                         | +01a
        move.b  d1,0x98(a0)                     | +01e
        move.b  d0,0x99(a0)                     | +022
        rts                                     | +026

| ----------------------------------------------------------------------------
|  PlayerFire_Shotgun_FwdLow_03ca8c  @ $03CA8C  (46 B)
| ----------------------------------------------------------------------------
        .section .text.PlayerFire_Shotgun_FwdLow_03ca8c, "ax", @progbits
        .global PlayerFire_Shotgun_FwdLow_03ca8c
PlayerFire_Shotgun_FwdLow_03ca8c:
        jsr     PlayerFire_Shotgun_Spawn_03c9c4(pc) | +000
        subi.w  #0xc,0x24(a0)                   | +004
        btst    #0x0,0x3a(a6)                   | +00a
        bne.w   .L03caa8                        | +010
        move.b  #0x0,d0                         | +014
        bra.w   .L03caac                        | +018
.L03caa8:
        move.b  #0xff,d0                        | +01c
.L03caac:
        move.b  #0x0,d1                         | +020
        move.b  d1,0x98(a0)                     | +024
        move.b  d0,0x99(a0)                     | +028
        rts                                     | +02c

| ----------------------------------------------------------------------------
|  PlayerFire_Shotgun_Diag_03caba  @ $03CABA  (46 B)
| ----------------------------------------------------------------------------
        .section .text.PlayerFire_Shotgun_Diag_03caba, "ax", @progbits
        .global PlayerFire_Shotgun_Diag_03caba
PlayerFire_Shotgun_Diag_03caba:
        jsr     PlayerFire_Shotgun_Spawn_03c9c4(pc) | +000
        subi.w  #0xc,0x24(a0)                   | +004
        btst    #0x0,0x3a(a6)                   | +00a
        bne.w   .L03cad6                        | +010
        move.b  #0x0,d0                         | +014
        bra.w   .L03cada                        | +018
.L03cad6:
        move.b  #0xff,d0                        | +01c
.L03cada:
        move.b  #0x3,d1                         | +020
        move.b  d1,0x98(a0)                     | +024
        move.b  d0,0x99(a0)                     | +028
        rts                                     | +02c

| ----------------------------------------------------------------------------
|  PlayerFire_Shotgun_DiagB_03cae8  @ $03CAE8  (46 B)
| ----------------------------------------------------------------------------
        .section .text.PlayerFire_Shotgun_DiagB_03cae8, "ax", @progbits
        .global PlayerFire_Shotgun_DiagB_03cae8
PlayerFire_Shotgun_DiagB_03cae8:
        jsr     PlayerFire_Shotgun_Spawn_03c9c4(pc) | +000
        subi.w  #0xc,0x24(a0)                   | +004
        btst    #0x0,0x3a(a6)                   | +00a
        bne.w   .L03cb04                        | +010
        move.b  #0x0,d0                         | +014
        bra.w   .L03cb08                        | +018
.L03cb04:
        move.b  #0xff,d0                        | +01c
.L03cb08:
        move.b  #0x3,d1                         | +020
        move.b  d1,0x98(a0)                     | +024
        move.b  d0,0x99(a0)                     | +028
        rts                                     | +02c

| ----------------------------------------------------------------------------
|  PlayerFire_Rocket_Spawn_03cb16  @ $03CB16  (98 B)
| ----------------------------------------------------------------------------
        .section .text.PlayerFire_Rocket_Spawn_03cb16, "ax", @progbits
        .global PlayerFire_Rocket_Spawn_03cb16
PlayerFire_Rocket_Spawn_03cb16:
        cmpa.l  #0x100440,a6                    | +000
        beq.b   .L03cb28                        | +006
        cmpa.l  #0x1004e0,a6                    | +008
        bne.w   .L03cb2e                        | +00e
.L03cb28:
        movea.l a6,a2                           | +012
        bra.w   .L03cb32                        | +014
.L03cb2e:
        movea.l 0xc(a6),a2                      | +018
.L03cb32:
        cmpi.w  #0x0,0x82(a2)                   | +01c
        beq.w   .L03cb72                        | +022
        subi.w  #0x1,0x82(a2)                   | +026
        move.w  #0x10f5,d0                      | +02c
        jsr     0x2352.l                        | +030
        lea     0x9bec2.l,a1                    | +036
        jsr     0x5eab6.l                       | +03c
        move.w  0x22(a2),0x22(a0)               | +042
        move.w  0x24(a2),0x24(a0)               | +048
        jsr     0x517fe.l                       | +04e
        addi.w  #0x1c,0x24(a0)                  | +054
        rts                                     | +05a
.L03cb72:
        movem.l (a7)+,a0                        | +05c
        rts                                     | +060

| ----------------------------------------------------------------------------
|  PlayerFire_Rocket_Fwd_03cb78  @ $03CB78  (68 B)
| ----------------------------------------------------------------------------
        .section .text.PlayerFire_Rocket_Fwd_03cb78, "ax", @progbits
        .global PlayerFire_Rocket_Fwd_03cb78
PlayerFire_Rocket_Fwd_03cb78:
        lea     ShellCasing_Rocket_03d4ea(pc),a1 | +000
        jsr     0x4ae.l                         | +004
        jsr     0x5dd02.l                       | +00a
        jsr     PlayerFire_Rocket_Spawn_03cb16(pc) | +010
        btst    #0x0,0x3a(a6)                   | +014
        bne.w   .L03cba4                        | +01a
        addi.w  #0x10,0x22(a0)                  | +01e
        move.b  #0x0,d0                         | +024
        bra.w   .L03cbae                        | +028
.L03cba4:
        subi.w  #0x10,0x22(a0)                  | +02c
        move.b  #0xff,d0                        | +032
.L03cbae:
        move.b  #0x0,d1                         | +036
        move.b  d1,0x98(a0)                     | +03a
        move.b  d0,0x99(a0)                     | +03e
        rts                                     | +042

| ----------------------------------------------------------------------------
|  PlayerFire_Rocket_Up_03cbbc  @ $03CBBC  (38 B)
| ----------------------------------------------------------------------------
        .section .text.PlayerFire_Rocket_Up_03cbbc, "ax", @progbits
        .global PlayerFire_Rocket_Up_03cbbc
PlayerFire_Rocket_Up_03cbbc:
        lea     ShellCasing_RocketB_03d4f8(pc),a1 | +000
        jsr     0x4ae.l                         | +004
        jsr     0x5dd02.l                       | +00a
        jsr     PlayerFire_Rocket_Spawn_03cb16(pc) | +010
        move.b  #0x1,d1                         | +014
        move.b  #0x0,d0                         | +018
        move.b  d1,0x98(a0)                     | +01c
        move.b  d0,0x99(a0)                     | +020
        rts                                     | +024

| ----------------------------------------------------------------------------
|  PlayerFire_Rocket_Back_03cbe2  @ $03CBE2  (80 B)
| ----------------------------------------------------------------------------
        .section .text.PlayerFire_Rocket_Back_03cbe2, "ax", @progbits
        .global PlayerFire_Rocket_Back_03cbe2
PlayerFire_Rocket_Back_03cbe2:
        lea     ShellCasing_Rocket_03d4ea(pc),a1 | +000
        jsr     0x4ae.l                         | +004
        jsr     0x5dd02.l                       | +00a
        move.b  0x3a(a6),d0                     | +010
        eori.b  #0x1,d0                         | +014
        move.b  d0,0x3a(a0)                     | +018
        jsr     PlayerFire_Rocket_Spawn_03cb16(pc) | +01c
        btst    #0x0,0x3a(a6)                   | +020
        bne.w   .L03cc1a                        | +026
        subi.w  #0x10,0x22(a0)                  | +02a
        move.b  #0x0,d0                         | +030
        bra.w   .L03cc24                        | +034
.L03cc1a:
        addi.w  #0x10,0x22(a0)                  | +038
        move.b  #0xff,d0                        | +03e
.L03cc24:
        move.b  #0x2,d1                         | +042
        move.b  d1,0x98(a0)                     | +046
        move.b  d0,0x99(a0)                     | +04a
        rts                                     | +04e

| ----------------------------------------------------------------------------
|  PlayerFire_Rocket_FwdLow_03cc32  @ $03CC32  (68 B)
| ----------------------------------------------------------------------------
        .section .text.PlayerFire_Rocket_FwdLow_03cc32, "ax", @progbits
        .global PlayerFire_Rocket_FwdLow_03cc32
PlayerFire_Rocket_FwdLow_03cc32:
        lea     ShellCasing_Rocket_03d4ea(pc),a1 | +000
        jsr     0x4ae.l                         | +004
        jsr     0x5dd02.l                       | +00a
        subi.w  #0xc,0x24(a0)                   | +010
        jsr     PlayerFire_Rocket_Spawn_03cb16(pc) | +016
        subi.w  #0xc,0x24(a0)                   | +01a
        btst    #0x0,0x3a(a6)                   | +020
        bne.w   .L03cc64                        | +026
        move.b  #0x0,d0                         | +02a
        bra.w   .L03cc68                        | +02e
.L03cc64:
        move.b  #0xff,d0                        | +032
.L03cc68:
        move.b  #0x0,d1                         | +036
        move.b  d1,0x98(a0)                     | +03a
        move.b  d0,0x99(a0)                     | +03e
        rts                                     | +042

| ----------------------------------------------------------------------------
|  PlayerFire_Rocket_Diag_03cc76  @ $03CC76  (46 B)
| ----------------------------------------------------------------------------
        .section .text.PlayerFire_Rocket_Diag_03cc76, "ax", @progbits
        .global PlayerFire_Rocket_Diag_03cc76
PlayerFire_Rocket_Diag_03cc76:
        jsr     PlayerFire_Rocket_Spawn_03cb16(pc) | +000
        subi.w  #0xc,0x24(a0)                   | +004
        btst    #0x0,0x3a(a6)                   | +00a
        bne.w   .L03cc92                        | +010
        move.b  #0x0,d0                         | +014
        bra.w   .L03cc96                        | +018
.L03cc92:
        move.b  #0xff,d0                        | +01c
.L03cc96:
        move.b  #0x3,d1                         | +020
        move.b  d1,0x98(a0)                     | +024
        move.b  d0,0x99(a0)                     | +028
        rts                                     | +02c

| ----------------------------------------------------------------------------
|  PlayerFire_Rocket_DiagB_03cca4  @ $03CCA4  (46 B)
| ----------------------------------------------------------------------------
        .section .text.PlayerFire_Rocket_DiagB_03cca4, "ax", @progbits
        .global PlayerFire_Rocket_DiagB_03cca4
PlayerFire_Rocket_DiagB_03cca4:
        jsr     PlayerFire_Rocket_Spawn_03cb16(pc) | +000
        subi.w  #0xc,0x24(a0)                   | +004
        btst    #0x0,0x3a(a6)                   | +00a
        bne.w   .L03ccc0                        | +010
        move.b  #0x0,d0                         | +014
        bra.w   .L03ccc4                        | +018
.L03ccc0:
        move.b  #0xff,d0                        | +01c
.L03ccc4:
        move.b  #0x3,d1                         | +020
        move.b  d1,0x98(a0)                     | +024
        move.b  d0,0x99(a0)                     | +028
        rts                                     | +02c

| ----------------------------------------------------------------------------
|  PlayerFire_Flame_Spawn_03ccd2  @ $03CCD2  (108 B)
| ----------------------------------------------------------------------------
        .section .text.PlayerFire_Flame_Spawn_03ccd2, "ax", @progbits
        .global PlayerFire_Flame_Spawn_03ccd2
PlayerFire_Flame_Spawn_03ccd2:
        cmpa.l  #0x100440,a6                    | +000
        beq.b   .L03cce4                        | +006
        cmpa.l  #0x1004e0,a6                    | +008
        bne.w   .L03ccea                        | +00e
.L03cce4:
        movea.l a6,a2                           | +012
        bra.w   .L03ccee                        | +014
.L03ccea:
        movea.l 0xc(a6),a2                      | +018
.L03ccee:
        clr.b   0x7a(a6)                        | +01c
        cmpi.w  #0x0,0x82(a2)                   | +020
        beq.w   .L03cd38                        | +026
        subi.w  #0x1,0x82(a2)                   | +02a
        move.w  #0x10f4,d0                      | +030
        jsr     0x2352.l                        | +034
        lea     0x308c2.l,a1                    | +03a
        jsr     0x6fe.l                         | +040
        move.w  0x22(a2),0x22(a0)               | +046
        move.w  0x24(a2),0x24(a0)               | +04c
        move.w  0x38(a2),0x38(a0)               | +052
        jsr     0x517fe.l                       | +058
        addi.w  #0x14,0x24(a0)                  | +05e
        rts                                     | +064
.L03cd38:
        movem.l (a7)+,a0                        | +066
        rts                                     | +06a

| ----------------------------------------------------------------------------
|  PlayerFire_Flame_FwdNeg_03cd3e  @ $03CD3E  (50 B)
| ----------------------------------------------------------------------------
        .section .text.PlayerFire_Flame_FwdNeg_03cd3e, "ax", @progbits
        .global PlayerFire_Flame_FwdNeg_03cd3e
PlayerFire_Flame_FwdNeg_03cd3e:
        jsr     PlayerFire_Flame_Spawn_03ccd2(pc) | +000
        btst    #0x0,0x3a(a6)                   | +004
        bne.w   .L03cd54                        | +00a
        move.b  #0x0,d1                         | +00e
        bra.w   .L03cd58                        | +012
.L03cd54:
        move.b  #0x2,d1                         | +016
.L03cd58:
        move.b  #0x0,d0                         | +01a
        lsl.b   #0x3,d0                         | +01e
        move.b  d0,0x98(a0)                     | +020
        move.b  0x7a(a6),0x9a(a0)               | +024
        move.b  #0xfe,0x9b(a0)                  | +02a
        rts                                     | +030

| ----------------------------------------------------------------------------
|  PlayerFire_Flame_Fwd0_03cd70  @ $03CD70  (50 B)
| ----------------------------------------------------------------------------
        .section .text.PlayerFire_Flame_Fwd0_03cd70, "ax", @progbits
        .global PlayerFire_Flame_Fwd0_03cd70
PlayerFire_Flame_Fwd0_03cd70:
        jsr     PlayerFire_Flame_Spawn_03ccd2(pc) | +000
        btst    #0x0,0x3a(a6)                   | +004
        bne.w   .L03cd86                        | +00a
        move.b  #0x0,d1                         | +00e
        bra.w   .L03cd8a                        | +012
.L03cd86:
        move.b  #0x2,d1                         | +016
.L03cd8a:
        move.b  #0x0,d0                         | +01a
        lsl.b   #0x3,d0                         | +01e
        move.b  d0,0x98(a0)                     | +020
        move.b  0x7a(a6),0x9a(a0)               | +024
        move.b  #0x0,0x9b(a0)                   | +02a
        rts                                     | +030

| ----------------------------------------------------------------------------
|  PlayerFire_Flame_FwdPos_03cda2  @ $03CDA2  (50 B)
| ----------------------------------------------------------------------------
        .section .text.PlayerFire_Flame_FwdPos_03cda2, "ax", @progbits
        .global PlayerFire_Flame_FwdPos_03cda2
PlayerFire_Flame_FwdPos_03cda2:
        jsr     PlayerFire_Flame_Spawn_03ccd2(pc) | +000
        btst    #0x0,0x3a(a6)                   | +004
        bne.w   .L03cdb8                        | +00a
        move.b  #0x0,d1                         | +00e
        bra.w   .L03cdbc                        | +012
.L03cdb8:
        move.b  #0x2,d1                         | +016
.L03cdbc:
        move.b  #0x0,d0                         | +01a
        lsl.b   #0x3,d0                         | +01e
        move.b  d0,0x98(a0)                     | +020
        move.b  0x7a(a6),0x9a(a0)               | +024
        move.b  #0x2,0x9b(a0)                   | +02a
        rts                                     | +030

| ----------------------------------------------------------------------------
|  PlayerFire_Flame_FwdNegLow_03cdd4  @ $03CDD4  (26 B)
| ----------------------------------------------------------------------------
        .section .text.PlayerFire_Flame_FwdNegLow_03cdd4, "ax", @progbits
        .global PlayerFire_Flame_FwdNegLow_03cdd4
PlayerFire_Flame_FwdNegLow_03cdd4:
        jsr     PlayerFire_Flame_FwdNeg_03cd3e(pc) | +000
        btst    #0x0,0x3a(a6)                   | +004
        bne.w   .L03cde6                        | +00a
        bra.w   .L03cde6                        | +00e
.L03cde6:
        subi.w  #0xc,0x24(a0)                   | +012
        rts                                     | +018

| ----------------------------------------------------------------------------
|  PlayerFire_Flame_Fwd0Low_03cdee  @ $03CDEE  (26 B)
| ----------------------------------------------------------------------------
        .section .text.PlayerFire_Flame_Fwd0Low_03cdee, "ax", @progbits
        .global PlayerFire_Flame_Fwd0Low_03cdee
PlayerFire_Flame_Fwd0Low_03cdee:
        jsr     PlayerFire_Flame_Fwd0_03cd70(pc) | +000
        btst    #0x0,0x3a(a6)                   | +004
        bne.w   .L03ce00                        | +00a
        bra.w   .L03ce00                        | +00e
.L03ce00:
        subi.w  #0xc,0x24(a0)                   | +012
        rts                                     | +018

| ----------------------------------------------------------------------------
|  PlayerFire_Flame_FwdPosLow_03ce08  @ $03CE08  (26 B)
| ----------------------------------------------------------------------------
        .section .text.PlayerFire_Flame_FwdPosLow_03ce08, "ax", @progbits
        .global PlayerFire_Flame_FwdPosLow_03ce08
PlayerFire_Flame_FwdPosLow_03ce08:
        jsr     PlayerFire_Flame_FwdPos_03cda2(pc) | +000
        btst    #0x0,0x3a(a6)                   | +004
        bne.w   .L03ce1a                        | +00a
        bra.w   .L03ce1a                        | +00e
.L03ce1a:
        subi.w  #0xc,0x24(a0)                   | +012
        rts                                     | +018

| ----------------------------------------------------------------------------
|  PlayerFire_Flame_UpNeg_03ce22  @ $03CE22  (50 B)
| ----------------------------------------------------------------------------
        .section .text.PlayerFire_Flame_UpNeg_03ce22, "ax", @progbits
        .global PlayerFire_Flame_UpNeg_03ce22
PlayerFire_Flame_UpNeg_03ce22:
        jsr     PlayerFire_Flame_Spawn_03ccd2(pc) | +000
        btst    #0x0,0x3a(a6)                   | +004
        bne.w   .L03ce38                        | +00a
        move.b  #0x0,d1                         | +00e
        bra.w   .L03ce3c                        | +012
.L03ce38:
        move.b  #0x2,d1                         | +016
.L03ce3c:
        move.b  #0x8,d0                         | +01a
        lsl.b   #0x3,d0                         | +01e
        move.b  d0,0x98(a0)                     | +020
        move.b  0x7a(a6),0x9a(a0)               | +024
        move.b  #0xfe,0x9b(a0)                  | +02a
        rts                                     | +030

| ----------------------------------------------------------------------------
|  PlayerFire_Flame_Up0_03ce54  @ $03CE54  (50 B)
| ----------------------------------------------------------------------------
        .section .text.PlayerFire_Flame_Up0_03ce54, "ax", @progbits
        .global PlayerFire_Flame_Up0_03ce54
PlayerFire_Flame_Up0_03ce54:
        jsr     PlayerFire_Flame_Spawn_03ccd2(pc) | +000
        btst    #0x0,0x3a(a6)                   | +004
        bne.w   .L03ce6a                        | +00a
        move.b  #0x0,d1                         | +00e
        bra.w   .L03ce6e                        | +012
.L03ce6a:
        move.b  #0x2,d1                         | +016
.L03ce6e:
        move.b  #0x8,d0                         | +01a
        lsl.b   #0x3,d0                         | +01e
        move.b  d0,0x98(a0)                     | +020
        move.b  0x7a(a6),0x9a(a0)               | +024
        move.b  #0x0,0x9b(a0)                   | +02a
        rts                                     | +030

| ----------------------------------------------------------------------------
|  PlayerFire_Flame_UpPos_03ce86  @ $03CE86  (50 B)
| ----------------------------------------------------------------------------
        .section .text.PlayerFire_Flame_UpPos_03ce86, "ax", @progbits
        .global PlayerFire_Flame_UpPos_03ce86
PlayerFire_Flame_UpPos_03ce86:
        jsr     PlayerFire_Flame_Spawn_03ccd2(pc) | +000
        btst    #0x0,0x3a(a6)                   | +004
        bne.w   .L03ce9c                        | +00a
        move.b  #0x0,d1                         | +00e
        bra.w   .L03cea0                        | +012
.L03ce9c:
        move.b  #0x2,d1                         | +016
.L03cea0:
        move.b  #0x8,d0                         | +01a
        lsl.b   #0x3,d0                         | +01e
        move.b  d0,0x98(a0)                     | +020
        move.b  0x7a(a6),0x9a(a0)               | +024
        move.b  #0x2,0x9b(a0)                   | +02a
        rts                                     | +030

| ----------------------------------------------------------------------------
|  PlayerFire_Flame_BackNeg_03ceb8  @ $03CEB8  (50 B)
| ----------------------------------------------------------------------------
        .section .text.PlayerFire_Flame_BackNeg_03ceb8, "ax", @progbits
        .global PlayerFire_Flame_BackNeg_03ceb8
PlayerFire_Flame_BackNeg_03ceb8:
        jsr     PlayerFire_Flame_Spawn_03ccd2(pc) | +000
        btst    #0x0,0x3a(a6)                   | +004
        bne.w   .L03cece                        | +00a
        move.b  #0x0,d1                         | +00e
        bra.w   .L03ced2                        | +012
.L03cece:
        move.b  #0x2,d1                         | +016
.L03ced2:
        move.b  #0x10,d0                        | +01a
        lsl.b   #0x3,d0                         | +01e
        move.b  d0,0x98(a0)                     | +020
        move.b  0x7a(a6),0x9a(a0)               | +024
        move.b  #0xfe,0x9b(a0)                  | +02a
        rts                                     | +030

| ----------------------------------------------------------------------------
|  PlayerFire_Flame_Back0_03ceea  @ $03CEEA  (58 B)
| ----------------------------------------------------------------------------
        .section .text.PlayerFire_Flame_Back0_03ceea, "ax", @progbits
        .global PlayerFire_Flame_Back0_03ceea
PlayerFire_Flame_Back0_03ceea:
        jsr     PlayerFire_Flame_Spawn_03ccd2(pc) | +000
        btst    #0x0,0x3a(a6)                   | +004
        bne.w   .L03cf04                        | +00a
        move.b  #0x10,d0                        | +00e
        move.b  #0x0,d1                         | +012
        bra.w   .L03cf0c                        | +016
.L03cf04:
        move.b  #0x10,d0                        | +01a
        move.b  #0x2,d1                         | +01e
.L03cf0c:
        move.b  #0x10,d0                        | +022
        lsl.b   #0x3,d0                         | +026
        move.b  d0,0x98(a0)                     | +028
        move.b  0x7a(a6),0x9a(a0)               | +02c
        move.b  #0x0,0x9b(a0)                   | +032
        rts                                     | +038

| ----------------------------------------------------------------------------
|  PlayerFire_Flame_BackPos_03cf24  @ $03CF24  (50 B)
| ----------------------------------------------------------------------------
        .section .text.PlayerFire_Flame_BackPos_03cf24, "ax", @progbits
        .global PlayerFire_Flame_BackPos_03cf24
PlayerFire_Flame_BackPos_03cf24:
        jsr     PlayerFire_Flame_Spawn_03ccd2(pc) | +000
        btst    #0x0,0x3a(a6)                   | +004
        bne.w   .L03cf3a                        | +00a
        move.b  #0x0,d1                         | +00e
        bra.w   .L03cf3e                        | +012
.L03cf3a:
        move.b  #0x2,d1                         | +016
.L03cf3e:
        move.b  #0x10,d0                        | +01a
        lsl.b   #0x3,d0                         | +01e
        move.b  d0,0x98(a0)                     | +020
        move.b  0x7a(a6),0x9a(a0)               | +024
        move.b  #0x2,0x9b(a0)                   | +02a
        rts                                     | +030

| ----------------------------------------------------------------------------
|  PlayerFire_Flame_Ang17Neg_03cf56  @ $03CF56  (62 B)
| ----------------------------------------------------------------------------
        .section .text.PlayerFire_Flame_Ang17Neg_03cf56, "ax", @progbits
        .global PlayerFire_Flame_Ang17Neg_03cf56
PlayerFire_Flame_Ang17Neg_03cf56:
        jsr     PlayerFire_Flame_Spawn_03ccd2(pc) | +000
        btst    #0x0,0x3a(a6)                   | +004
        bne.w   .L03cf72                        | +00a
        subi.w  #0x10,0x22(a0)                  | +00e
        move.b  #0x0,d1                         | +014
        bra.w   .L03cf7c                        | +018
.L03cf72:
        addi.w  #0x10,0x22(a0)                  | +01c
        move.b  #0x2,d1                         | +022
.L03cf7c:
        move.b  #0x17,d0                        | +026
        lsl.b   #0x3,d0                         | +02a
        move.b  d0,0x98(a0)                     | +02c
        move.b  0x7a(a6),0x9a(a0)               | +030
        move.b  #0xfe,0x9b(a0)                  | +036
        rts                                     | +03c

| ----------------------------------------------------------------------------
|  PlayerFire_Flame_Ang17_0_03cf94  @ $03CF94  (62 B)
| ----------------------------------------------------------------------------
        .section .text.PlayerFire_Flame_Ang17_0_03cf94, "ax", @progbits
        .global PlayerFire_Flame_Ang17_0_03cf94
PlayerFire_Flame_Ang17_0_03cf94:
        jsr     PlayerFire_Flame_Spawn_03ccd2(pc) | +000
        btst    #0x0,0x3a(a6)                   | +004
        bne.w   .L03cfb0                        | +00a
        subi.w  #0x10,0x22(a0)                  | +00e
        move.b  #0x0,d1                         | +014
        bra.w   .L03cfba                        | +018
.L03cfb0:
        addi.w  #0x10,0x22(a0)                  | +01c
        move.b  #0x2,d1                         | +022
.L03cfba:
        move.b  #0x17,d0                        | +026
        lsl.b   #0x3,d0                         | +02a
        move.b  d0,0x98(a0)                     | +02c
        move.b  0x7a(a6),0x9a(a0)               | +030
        move.b  #0x0,0x9b(a0)                   | +036
        rts                                     | +03c

| ----------------------------------------------------------------------------
|  PlayerFire_Flame_Ang17Pos_03cfd2  @ $03CFD2  (62 B)
| ----------------------------------------------------------------------------
        .section .text.PlayerFire_Flame_Ang17Pos_03cfd2, "ax", @progbits
        .global PlayerFire_Flame_Ang17Pos_03cfd2
PlayerFire_Flame_Ang17Pos_03cfd2:
        jsr     PlayerFire_Flame_Spawn_03ccd2(pc) | +000
        btst    #0x0,0x3a(a6)                   | +004
        bne.w   .L03cfee                        | +00a
        subi.w  #0x10,0x22(a0)                  | +00e
        move.b  #0x0,d1                         | +014
        bra.w   .L03cff8                        | +018
.L03cfee:
        addi.w  #0x10,0x22(a0)                  | +01c
        move.b  #0x2,d1                         | +022
.L03cff8:
        move.b  #0x17,d0                        | +026
        lsl.b   #0x3,d0                         | +02a
        move.b  d0,0x98(a0)                     | +02c
        move.b  0x7a(a6),0x9a(a0)               | +030
        move.b  #0x2,0x9b(a0)                   | +036
        rts                                     | +03c

| ----------------------------------------------------------------------------
|  PlayerFire_Flame_DownNeg_03d010  @ $03D010  (50 B)
| ----------------------------------------------------------------------------
        .section .text.PlayerFire_Flame_DownNeg_03d010, "ax", @progbits
        .global PlayerFire_Flame_DownNeg_03d010
PlayerFire_Flame_DownNeg_03d010:
        jsr     PlayerFire_Flame_Spawn_03ccd2(pc) | +000
        btst    #0x0,0x3a(a6)                   | +004
        bne.w   .L03d026                        | +00a
        move.b  #0x0,d1                         | +00e
        bra.w   .L03d02a                        | +012
.L03d026:
        move.b  #0x2,d1                         | +016
.L03d02a:
        move.b  #0x18,d0                        | +01a
        lsl.b   #0x3,d0                         | +01e
        move.b  d0,0x98(a0)                     | +020
        move.b  0x7a(a6),0x9a(a0)               | +024
        move.b  #0xfe,0x9b(a0)                  | +02a
        rts                                     | +030

| ----------------------------------------------------------------------------
|  PlayerFire_Flame_Down0_03d042  @ $03D042  (50 B)
| ----------------------------------------------------------------------------
        .section .text.PlayerFire_Flame_Down0_03d042, "ax", @progbits
        .global PlayerFire_Flame_Down0_03d042
PlayerFire_Flame_Down0_03d042:
        jsr     PlayerFire_Flame_Spawn_03ccd2(pc) | +000
        btst    #0x0,0x3a(a6)                   | +004
        bne.w   .L03d058                        | +00a
        move.b  #0x0,d1                         | +00e
        bra.w   .L03d05c                        | +012
.L03d058:
        move.b  #0x2,d1                         | +016
.L03d05c:
        move.b  #0x18,d0                        | +01a
        lsl.b   #0x3,d0                         | +01e
        move.b  d0,0x98(a0)                     | +020
        move.b  0x7a(a6),0x9a(a0)               | +024
        move.b  #0x0,0x9b(a0)                   | +02a
        rts                                     | +030

| ----------------------------------------------------------------------------
|  PlayerFire_Flame_DownPos_03d074  @ $03D074  (50 B)
| ----------------------------------------------------------------------------
        .section .text.PlayerFire_Flame_DownPos_03d074, "ax", @progbits
        .global PlayerFire_Flame_DownPos_03d074
PlayerFire_Flame_DownPos_03d074:
        jsr     PlayerFire_Flame_Spawn_03ccd2(pc) | +000
        btst    #0x0,0x3a(a6)                   | +004
        bne.w   .L03d08a                        | +00a
        move.b  #0x0,d1                         | +00e
        bra.w   .L03d08e                        | +012
.L03d08a:
        move.b  #0x2,d1                         | +016
.L03d08e:
        move.b  #0x18,d0                        | +01a
        lsl.b   #0x3,d0                         | +01e
        move.b  d0,0x98(a0)                     | +020
        move.b  0x7a(a6),0x9a(a0)               | +024
        move.b  #0x2,0x9b(a0)                   | +02a
        rts                                     | +030

| ----------------------------------------------------------------------------
|  PlayerFire_Flame_Spread1_3_03d0a6  @ $03D0A6  (94 B)
| ----------------------------------------------------------------------------
        .section .text.PlayerFire_Flame_Spread1_3_03d0a6, "ax", @progbits
        .global PlayerFire_Flame_Spread1_3_03d0a6
PlayerFire_Flame_Spread1_3_03d0a6:
        jsr     PlayerFire_Flame_Spawn_03ccd2(pc) | +000
        btst    #0x0,0x3a(a6)                   | +004
        bne.w   .L03d0bc                        | +00a
        move.b  #0x0,d1                         | +00e
        bra.w   .L03d0c0                        | +012
.L03d0bc:
        move.b  #0x2,d1                         | +016
.L03d0c0:
        move.b  #0x1,d0                         | +01a
        lsl.b   #0x3,d0                         | +01e
        move.b  d0,0x98(a0)                     | +020
        move.b  0x7a(a6),0x9a(a0)               | +024
        clr.b   0x9b(a0)                        | +02a
        jsr     PlayerFire_Flame_Spawn_03ccd2(pc) | +02e
        btst    #0x0,0x3a(a6)                   | +032
        bne.w   .L03d0ea                        | +038
        move.b  #0x0,d1                         | +03c
        bra.w   .L03d0ee                        | +040
.L03d0ea:
        move.b  #0x2,d1                         | +044
.L03d0ee:
        move.b  #0x3,d0                         | +048
        lsl.b   #0x3,d0                         | +04c
        move.b  d0,0x98(a0)                     | +04e
        move.b  0x7a(a6),0x9a(a0)               | +052
        clr.b   0x9b(a0)                        | +058
        rts                                     | +05c

| ----------------------------------------------------------------------------
|  PlayerFire_Flame_Spread5_7_03d104  @ $03D104  (94 B)
| ----------------------------------------------------------------------------
        .section .text.PlayerFire_Flame_Spread5_7_03d104, "ax", @progbits
        .global PlayerFire_Flame_Spread5_7_03d104
PlayerFire_Flame_Spread5_7_03d104:
        jsr     PlayerFire_Flame_Spawn_03ccd2(pc) | +000
        btst    #0x0,0x3a(a6)                   | +004
        bne.w   .L03d11a                        | +00a
        move.b  #0x0,d1                         | +00e
        bra.w   .L03d11e                        | +012
.L03d11a:
        move.b  #0x2,d1                         | +016
.L03d11e:
        move.b  #0x5,d0                         | +01a
        lsl.b   #0x3,d0                         | +01e
        move.b  d0,0x98(a0)                     | +020
        move.b  0x7a(a6),0x9a(a0)               | +024
        clr.b   0x9b(a0)                        | +02a
        jsr     PlayerFire_Flame_Spawn_03ccd2(pc) | +02e
        btst    #0x0,0x3a(a6)                   | +032
        bne.w   .L03d148                        | +038
        move.b  #0x0,d1                         | +03c
        bra.w   .L03d14c                        | +040
.L03d148:
        move.b  #0x2,d1                         | +044
.L03d14c:
        move.b  #0x7,d0                         | +048
        lsl.b   #0x3,d0                         | +04c
        move.b  d0,0x98(a0)                     | +04e
        move.b  0x7a(a6),0x9a(a0)               | +052
        clr.b   0x9b(a0)                        | +058
        rts                                     | +05c

| ----------------------------------------------------------------------------
|  PlayerFire_Flame_Spread9_B_03d162  @ $03D162  (94 B)
| ----------------------------------------------------------------------------
        .section .text.PlayerFire_Flame_Spread9_B_03d162, "ax", @progbits
        .global PlayerFire_Flame_Spread9_B_03d162
PlayerFire_Flame_Spread9_B_03d162:
        jsr     PlayerFire_Flame_Spawn_03ccd2(pc) | +000
        btst    #0x0,0x3a(a6)                   | +004
        bne.w   .L03d178                        | +00a
        move.b  #0x0,d1                         | +00e
        bra.w   .L03d17c                        | +012
.L03d178:
        move.b  #0x2,d1                         | +016
.L03d17c:
        move.b  #0x9,d0                         | +01a
        lsl.b   #0x3,d0                         | +01e
        move.b  d0,0x98(a0)                     | +020
        move.b  0x7a(a6),0x9a(a0)               | +024
        clr.b   0x9b(a0)                        | +02a
        jsr     PlayerFire_Flame_Spawn_03ccd2(pc) | +02e
        btst    #0x0,0x3a(a6)                   | +032
        bne.w   .L03d1a6                        | +038
        move.b  #0x0,d1                         | +03c
        bra.w   .L03d1aa                        | +040
.L03d1a6:
        move.b  #0x2,d1                         | +044
.L03d1aa:
        move.b  #0xb,d0                         | +048
        lsl.b   #0x3,d0                         | +04c
        move.b  d0,0x98(a0)                     | +04e
        move.b  0x7a(a6),0x9a(a0)               | +052
        clr.b   0x9b(a0)                        | +058
        rts                                     | +05c

| ----------------------------------------------------------------------------
|  PlayerFire_Flame_SpreadD_F_03d1c0  @ $03D1C0  (94 B)
| ----------------------------------------------------------------------------
        .section .text.PlayerFire_Flame_SpreadD_F_03d1c0, "ax", @progbits
        .global PlayerFire_Flame_SpreadD_F_03d1c0
PlayerFire_Flame_SpreadD_F_03d1c0:
        jsr     PlayerFire_Flame_Spawn_03ccd2(pc) | +000
        btst    #0x0,0x3a(a6)                   | +004
        bne.w   .L03d1d6                        | +00a
        move.b  #0x0,d1                         | +00e
        bra.w   .L03d1da                        | +012
.L03d1d6:
        move.b  #0x2,d1                         | +016
.L03d1da:
        move.b  #0xd,d0                         | +01a
        lsl.b   #0x3,d0                         | +01e
        move.b  d0,0x98(a0)                     | +020
        move.b  0x7a(a6),0x9a(a0)               | +024
        clr.b   0x9b(a0)                        | +02a
        jsr     PlayerFire_Flame_Spawn_03ccd2(pc) | +02e
        btst    #0x0,0x3a(a6)                   | +032
        bne.w   .L03d204                        | +038
        move.b  #0x0,d1                         | +03c
        bra.w   .L03d208                        | +040
.L03d204:
        move.b  #0x2,d1                         | +044
.L03d208:
        move.b  #0xf,d0                         | +048
        lsl.b   #0x3,d0                         | +04c
        move.b  d0,0x98(a0)                     | +04e
        move.b  0x7a(a6),0x9a(a0)               | +052
        clr.b   0x9b(a0)                        | +058
        rts                                     | +05c

| ----------------------------------------------------------------------------
|  PlayerFire_Flame_Spread11_13_03d21e  @ $03D21E  (94 B)
| ----------------------------------------------------------------------------
        .section .text.PlayerFire_Flame_Spread11_13_03d21e, "ax", @progbits
        .global PlayerFire_Flame_Spread11_13_03d21e
PlayerFire_Flame_Spread11_13_03d21e:
        jsr     PlayerFire_Flame_Spawn_03ccd2(pc) | +000
        btst    #0x0,0x3a(a6)                   | +004
        bne.w   .L03d234                        | +00a
        move.b  #0x0,d1                         | +00e
        bra.w   .L03d238                        | +012
.L03d234:
        move.b  #0x2,d1                         | +016
.L03d238:
        move.b  #0x11,d0                        | +01a
        lsl.b   #0x3,d0                         | +01e
        move.b  d0,0x98(a0)                     | +020
        move.b  0x7a(a6),0x9a(a0)               | +024
        clr.b   0x9b(a0)                        | +02a
        jsr     PlayerFire_Flame_Spawn_03ccd2(pc) | +02e
        btst    #0x0,0x3a(a6)                   | +032
        bne.w   .L03d262                        | +038
        move.b  #0x0,d1                         | +03c
        bra.w   .L03d266                        | +040
.L03d262:
        move.b  #0x2,d1                         | +044
.L03d266:
        move.b  #0x13,d0                        | +048
        lsl.b   #0x3,d0                         | +04c
        move.b  d0,0x98(a0)                     | +04e
        move.b  0x7a(a6),0x9a(a0)               | +052
        clr.b   0x9b(a0)                        | +058
        rts                                     | +05c

| ----------------------------------------------------------------------------
|  PlayerFire_Flame_Spread15_17_03d27c  @ $03D27C  (94 B)
| ----------------------------------------------------------------------------
        .section .text.PlayerFire_Flame_Spread15_17_03d27c, "ax", @progbits
        .global PlayerFire_Flame_Spread15_17_03d27c
PlayerFire_Flame_Spread15_17_03d27c:
        jsr     PlayerFire_Flame_Spawn_03ccd2(pc) | +000
        btst    #0x0,0x3a(a6)                   | +004
        bne.w   .L03d292                        | +00a
        move.b  #0x0,d1                         | +00e
        bra.w   .L03d296                        | +012
.L03d292:
        move.b  #0x2,d1                         | +016
.L03d296:
        move.b  #0x15,d0                        | +01a
        lsl.b   #0x3,d0                         | +01e
        move.b  d0,0x98(a0)                     | +020
        move.b  0x7a(a6),0x9a(a0)               | +024
        clr.b   0x9b(a0)                        | +02a
        jsr     PlayerFire_Flame_Spawn_03ccd2(pc) | +02e
        btst    #0x0,0x3a(a6)                   | +032
        bne.w   .L03d2c0                        | +038
        move.b  #0x0,d1                         | +03c
        bra.w   .L03d2c4                        | +040
.L03d2c0:
        move.b  #0x2,d1                         | +044
.L03d2c4:
        move.b  #0x17,d0                        | +048
        lsl.b   #0x3,d0                         | +04c
        move.b  d0,0x98(a0)                     | +04e
        move.b  0x7a(a6),0x9a(a0)               | +052
        clr.b   0x9b(a0)                        | +058
        rts                                     | +05c

| ----------------------------------------------------------------------------
|  PlayerFire_Flame_Spread19_1B_03d2da  @ $03D2DA  (94 B)
| ----------------------------------------------------------------------------
        .section .text.PlayerFire_Flame_Spread19_1B_03d2da, "ax", @progbits
        .global PlayerFire_Flame_Spread19_1B_03d2da
PlayerFire_Flame_Spread19_1B_03d2da:
        jsr     PlayerFire_Flame_Spawn_03ccd2(pc) | +000
        btst    #0x0,0x3a(a6)                   | +004
        bne.w   .L03d2f0                        | +00a
        move.b  #0x0,d1                         | +00e
        bra.w   .L03d2f4                        | +012
.L03d2f0:
        move.b  #0x2,d1                         | +016
.L03d2f4:
        move.b  #0x19,d0                        | +01a
        lsl.b   #0x3,d0                         | +01e
        move.b  d0,0x98(a0)                     | +020
        move.b  0x7a(a6),0x9a(a0)               | +024
        clr.b   0x9b(a0)                        | +02a
        jsr     PlayerFire_Flame_Spawn_03ccd2(pc) | +02e
        btst    #0x0,0x3a(a6)                   | +032
        bne.w   .L03d31e                        | +038
        move.b  #0x0,d1                         | +03c
        bra.w   .L03d322                        | +040
.L03d31e:
        move.b  #0x2,d1                         | +044
.L03d322:
        move.b  #0x1b,d0                        | +048
        lsl.b   #0x3,d0                         | +04c
        move.b  d0,0x98(a0)                     | +04e
        move.b  0x7a(a6),0x9a(a0)               | +052
        clr.b   0x9b(a0)                        | +058
        rts                                     | +05c

| ----------------------------------------------------------------------------
|  PlayerFire_Flame_Spread1D_1F_03d338  @ $03D338  (94 B)
| ----------------------------------------------------------------------------
        .section .text.PlayerFire_Flame_Spread1D_1F_03d338, "ax", @progbits
        .global PlayerFire_Flame_Spread1D_1F_03d338
PlayerFire_Flame_Spread1D_1F_03d338:
        jsr     PlayerFire_Flame_Spawn_03ccd2(pc) | +000
        btst    #0x0,0x3a(a6)                   | +004
        bne.w   .L03d34e                        | +00a
        move.b  #0x0,d1                         | +00e
        bra.w   .L03d352                        | +012
.L03d34e:
        move.b  #0x2,d1                         | +016
.L03d352:
        move.b  #0x1d,d0                        | +01a
        lsl.b   #0x3,d0                         | +01e
        move.b  d0,0x98(a0)                     | +020
        move.b  0x7a(a6),0x9a(a0)               | +024
        clr.b   0x9b(a0)                        | +02a
        jsr     PlayerFire_Flame_Spawn_03ccd2(pc) | +02e
        btst    #0x0,0x3a(a6)                   | +032
        bne.w   .L03d37c                        | +038
        move.b  #0x0,d1                         | +03c
        bra.w   .L03d380                        | +040
.L03d37c:
        move.b  #0x2,d1                         | +044
.L03d380:
        move.b  #0x1f,d0                        | +048
        lsl.b   #0x3,d0                         | +04c
        move.b  d0,0x98(a0)                     | +04e
        move.b  0x7a(a6),0x9a(a0)               | +052
        clr.b   0x9b(a0)                        | +058
        rts                                     | +05c

| ----------------------------------------------------------------------------
|  ShellCasing_Pistol_03d396  @ $03D396  (96 B)
| ----------------------------------------------------------------------------
        .section .text.ShellCasing_Pistol_03d396, "ax", @progbits
        .global ShellCasing_Pistol_03d396
ShellCasing_Pistol_03d396:
        lea     ShellCasing_Pistol_Anim_03d3f6(pc),a0 | +000
        jsr     0x28cd4.l                       | +004
        bra.w   .L03d3b2                        | +00a
        lea     ShellCasing_Pistol_AnimB_03d470(pc),a0 | +00e
        jsr     0x28cd4.l                       | +012
        bra.w   .L03d3b2                        | +018
.L03d3b2:
        move.w  #0x184,d1                       | +01c
        jsr     0x236e.l                        | +020
        move.w  #0xd000,d0                      | +026
        jsr     0x28134.l                       | +02a
        andi.w  #0xffe3,0x38(a6)                | +030
        ori.w   #0x10,0x38(a6)                  | +036
        addi.w  #0x200,0x38(a6)                 | +03c
        lea     .L03d3de(pc),a1                 | +042
        move.l  a1,(a6)                         | +046
.L03d3de:
        jsr     0x2783a.l                       | +048
        jsr     0x28d70.l                       | +04e
        bcc.w   .L03d3f4                        | +054
        jmp     0x518.l                         | +058
.L03d3f4:
        rts                                     | +05e

| ----------------------------------------------------------------------------
|  ShellCasing_Pistol_Anim_03d3f6  @ $03D3F6  (122 B)
| ----------------------------------------------------------------------------
        .section .text.ShellCasing_Pistol_Anim_03d3f6, "ax", @progbits
        .global ShellCasing_Pistol_Anim_03d3f6
ShellCasing_Pistol_Anim_03d3f6:
        .dc.w   0x0001                        | +000  (dato / opcode no decodificado)
        .dc.w   0x0208                        | +002  (dato / opcode no decodificado)
        .dc.w   0x0026                        | +004  (dato / opcode no decodificado)
        .dc.w   0x20e6                        | +006  (dato / opcode no decodificado)
        .dc.w   0xffff                        | +008  (dato / opcode no decodificado)
        .dc.w   0x0001                        | +00a  (dato / opcode no decodificado)
        .dc.w   0x0208                        | +00c  (dato / opcode no decodificado)
        .dc.w   0x0026                        | +00e  (dato / opcode no decodificado)
        .dc.w   0x2100                        | +010  (dato / opcode no decodificado)
        .dc.w   0xffff                        | +012  (dato / opcode no decodificado)
        .dc.w   0x0001                        | +014  (dato / opcode no decodificado)
        .dc.w   0x0208                        | +016  (dato / opcode no decodificado)
        .dc.w   0x0026                        | +018  (dato / opcode no decodificado)
        .dc.w   0x212c                        | +01a  (dato / opcode no decodificado)
        .dc.w   0xffff                        | +01c  (dato / opcode no decodificado)
        .dc.w   0x0001                        | +01e  (dato / opcode no decodificado)
        .dc.w   0x0208                        | +020  (dato / opcode no decodificado)
        .dc.w   0x0026                        | +022  (dato / opcode no decodificado)
        .dc.w   0x215e                        | +024  (dato / opcode no decodificado)
        .dc.w   0xffff                        | +026  (dato / opcode no decodificado)
        .dc.w   0x0001                        | +028  (dato / opcode no decodificado)
        .dc.w   0x0208                        | +02a  (dato / opcode no decodificado)
        .dc.w   0x0026                        | +02c  (dato / opcode no decodificado)
        .dc.w   0x2198                        | +02e  (dato / opcode no decodificado)
        .dc.w   0xffff                        | +030  (dato / opcode no decodificado)
        .dc.w   0x0001                        | +032  (dato / opcode no decodificado)
        .dc.w   0x0208                        | +034  (dato / opcode no decodificado)
        .dc.w   0x0026                        | +036  (dato / opcode no decodificado)
        .dc.w   0x21ce                        | +038  (dato / opcode no decodificado)
        .dc.w   0xffff                        | +03a  (dato / opcode no decodificado)
        .dc.w   0x0001                        | +03c  (dato / opcode no decodificado)
        .dc.w   0x0208                        | +03e  (dato / opcode no decodificado)
        .dc.w   0x0026                        | +040  (dato / opcode no decodificado)
        .dc.w   0x21fc                        | +042  (dato / opcode no decodificado)
        .dc.w   0xffff                        | +044  (dato / opcode no decodificado)
        .dc.w   0x0001                        | +046  (dato / opcode no decodificado)
        .dc.w   0x0208                        | +048  (dato / opcode no decodificado)
        .dc.w   0x0026                        | +04a  (dato / opcode no decodificado)
        .dc.w   0x2222                        | +04c  (dato / opcode no decodificado)
        .dc.w   0xffff                        | +04e  (dato / opcode no decodificado)
        .dc.w   0x0001                        | +050  (dato / opcode no decodificado)
        .dc.w   0x0208                        | +052  (dato / opcode no decodificado)
        .dc.w   0x0026                        | +054  (dato / opcode no decodificado)
        .dc.w   0x223c                        | +056  (dato / opcode no decodificado)
        .dc.w   0xffff                        | +058  (dato / opcode no decodificado)
        .dc.w   0x0001                        | +05a  (dato / opcode no decodificado)
        .dc.w   0x0208                        | +05c  (dato / opcode no decodificado)
        .dc.w   0x0026                        | +05e  (dato / opcode no decodificado)
        .dc.w   0x225a                        | +060  (dato / opcode no decodificado)
        .dc.w   0xffff                        | +062  (dato / opcode no decodificado)
        .dc.w   0x0001                        | +064  (dato / opcode no decodificado)
        .dc.w   0x0208                        | +066  (dato / opcode no decodificado)
        .dc.w   0x0026                        | +068  (dato / opcode no decodificado)
        .dc.w   0x227e                        | +06a  (dato / opcode no decodificado)
        .dc.w   0xffff                        | +06c  (dato / opcode no decodificado)
        .dc.w   0x0001                        | +06e  (dato / opcode no decodificado)
        .dc.w   0x0208                        | +070  (dato / opcode no decodificado)
        .dc.w   0x0026                        | +072  (dato / opcode no decodificado)
        .dc.w   0x229e                        | +074  (dato / opcode no decodificado)
        .dc.w   0xffff                        | +076  (dato / opcode no decodificado)
        .dc.w   0x1600                        | +078  (dato / opcode no decodificado)

| ----------------------------------------------------------------------------
|  ShellCasing_Pistol_AnimB_03d470  @ $03D470  (122 B)
| ----------------------------------------------------------------------------
        .section .text.ShellCasing_Pistol_AnimB_03d470, "ax", @progbits
        .global ShellCasing_Pistol_AnimB_03d470
ShellCasing_Pistol_AnimB_03d470:
        .dc.w   0x0001                        | +000  (dato / opcode no decodificado)
        .dc.w   0x0208                        | +002  (dato / opcode no decodificado)
        .dc.w   0x0026                        | +004  (dato / opcode no decodificado)
        .dc.w   0x22b2                        | +006  (dato / opcode no decodificado)
        .dc.w   0xffff                        | +008  (dato / opcode no decodificado)
        .dc.w   0x0001                        | +00a  (dato / opcode no decodificado)
        .dc.w   0x0208                        | +00c  (dato / opcode no decodificado)
        .dc.w   0x0026                        | +00e  (dato / opcode no decodificado)
        .dc.w   0x22cc                        | +010  (dato / opcode no decodificado)
        .dc.w   0xffff                        | +012  (dato / opcode no decodificado)
        .dc.w   0x0001                        | +014  (dato / opcode no decodificado)
        .dc.w   0x0208                        | +016  (dato / opcode no decodificado)
        .dc.w   0x0026                        | +018  (dato / opcode no decodificado)
        .dc.w   0x22f0                        | +01a  (dato / opcode no decodificado)
        .dc.w   0xffff                        | +01c  (dato / opcode no decodificado)
        .dc.w   0x0001                        | +01e  (dato / opcode no decodificado)
        .dc.w   0x0208                        | +020  (dato / opcode no decodificado)
        .dc.w   0x0026                        | +022  (dato / opcode no decodificado)
        .dc.w   0x2324                        | +024  (dato / opcode no decodificado)
        .dc.w   0xffff                        | +026  (dato / opcode no decodificado)
        .dc.w   0x0002                        | +028  (dato / opcode no decodificado)
        .dc.w   0x0208                        | +02a  (dato / opcode no decodificado)
        .dc.w   0x0026                        | +02c  (dato / opcode no decodificado)
        .dc.w   0x2356                        | +02e  (dato / opcode no decodificado)
        .dc.w   0xffff                        | +030  (dato / opcode no decodificado)
        .dc.w   0x0002                        | +032  (dato / opcode no decodificado)
        .dc.w   0x0208                        | +034  (dato / opcode no decodificado)
        .dc.w   0x0026                        | +036  (dato / opcode no decodificado)
        .dc.w   0x2388                        | +038  (dato / opcode no decodificado)
        .dc.w   0xffff                        | +03a  (dato / opcode no decodificado)
        .dc.w   0x0002                        | +03c  (dato / opcode no decodificado)
        .dc.w   0x0208                        | +03e  (dato / opcode no decodificado)
        .dc.w   0x0026                        | +040  (dato / opcode no decodificado)
        .dc.w   0x23b0                        | +042  (dato / opcode no decodificado)
        .dc.w   0xffff                        | +044  (dato / opcode no decodificado)
        .dc.w   0x0002                        | +046  (dato / opcode no decodificado)
        .dc.w   0x0208                        | +048  (dato / opcode no decodificado)
        .dc.w   0x0026                        | +04a  (dato / opcode no decodificado)
        .dc.w   0x23ca                        | +04c  (dato / opcode no decodificado)
        .dc.w   0xffff                        | +04e  (dato / opcode no decodificado)
        .dc.w   0x0002                        | +050  (dato / opcode no decodificado)
        .dc.w   0x0208                        | +052  (dato / opcode no decodificado)
        .dc.w   0x0026                        | +054  (dato / opcode no decodificado)
        .dc.w   0x23e4                        | +056  (dato / opcode no decodificado)
        .dc.w   0xffff                        | +058  (dato / opcode no decodificado)
        .dc.w   0x0002                        | +05a  (dato / opcode no decodificado)
        .dc.w   0x0208                        | +05c  (dato / opcode no decodificado)
        .dc.w   0x0026                        | +05e  (dato / opcode no decodificado)
        .dc.w   0x2402                        | +060  (dato / opcode no decodificado)
        .dc.w   0xffff                        | +062  (dato / opcode no decodificado)
        .dc.w   0x0001                        | +064  (dato / opcode no decodificado)
        .dc.w   0x0208                        | +066  (dato / opcode no decodificado)
        .dc.w   0x0026                        | +068  (dato / opcode no decodificado)
        .dc.w   0x2424                        | +06a  (dato / opcode no decodificado)
        .dc.w   0xffff                        | +06c  (dato / opcode no decodificado)
        .dc.w   0x0001                        | +06e  (dato / opcode no decodificado)
        .dc.w   0x0208                        | +070  (dato / opcode no decodificado)
        .dc.w   0x0026                        | +072  (dato / opcode no decodificado)
        .dc.w   0x2444                        | +074  (dato / opcode no decodificado)
        .dc.w   0xffff                        | +076  (dato / opcode no decodificado)
        .dc.w   0x1600                        | +078  (dato / opcode no decodificado)

| ----------------------------------------------------------------------------
|  ShellCasing_Rocket_03d4ea  @ $03D4EA  (14 B)
| ----------------------------------------------------------------------------
        .section .text.ShellCasing_Rocket_03d4ea, "ax", @progbits
        .global ShellCasing_Rocket_03d4ea
ShellCasing_Rocket_03d4ea:
        lea     ShellCasing_Rocket_Anim_03d54a(pc),a0 | +000
        jsr     0x28cd4.l                       | +004
        bra.w   ShellCasing_RocketB_03d4f8__L03d506 | +00a

| ----------------------------------------------------------------------------
|  ShellCasing_RocketB_03d4f8  @ $03D4F8  (82 B)
| ----------------------------------------------------------------------------
        .section .text.ShellCasing_RocketB_03d4f8, "ax", @progbits
        .global ShellCasing_RocketB_03d4f8
ShellCasing_RocketB_03d4f8:
        lea     ShellCasing_Rocket_AnimB_03d5b0(pc),a0 | +000
        jsr     0x28cd4.l                       | +004
        bra.w   ShellCasing_RocketB_03d4f8__L03d506 | +00a
        .global ShellCasing_RocketB_03d4f8__L03d506
ShellCasing_RocketB_03d4f8__L03d506:
        move.w  #0x17c,d1                       | +00e
        jsr     0x236e.l                        | +012
        move.w  #0xd000,d0                      | +018
        jsr     0x28134.l                       | +01c
        andi.w  #0xffe3,0x38(a6)                | +022
        ori.w   #0x10,0x38(a6)                  | +028
        addi.w  #0x200,0x38(a6)                 | +02e
        lea     .L03d532(pc),a1                 | +034
        move.l  a1,(a6)                         | +038
.L03d532:
        jsr     0x2783a.l                       | +03a
        jsr     0x28d70.l                       | +040
        bcc.w   .L03d548                        | +046
        jmp     0x518.l                         | +04a
.L03d548:
        rts                                     | +050

| ----------------------------------------------------------------------------
|  ShellCasing_Rocket_Anim_03d54a  @ $03D54A  (102 B)
| ----------------------------------------------------------------------------
        .section .text.ShellCasing_Rocket_Anim_03d54a, "ax", @progbits
        .global ShellCasing_Rocket_Anim_03d54a
ShellCasing_Rocket_Anim_03d54a:
        .dc.w   0x0001                        | +000  (dato / opcode no decodificado)
        .dc.w   0x0208                        | +002  (dato / opcode no decodificado)
        .dc.w   0x0025                        | +004  (dato / opcode no decodificado)
        .dc.w   0xc172                        | +006  (dato / opcode no decodificado)
        .dc.w   0xffff                        | +008  (dato / opcode no decodificado)
        .dc.w   0x0001                        | +00a  (dato / opcode no decodificado)
        .dc.w   0x0208                        | +00c  (dato / opcode no decodificado)
        .dc.w   0x0025                        | +00e  (dato / opcode no decodificado)
        .dc.w   0xc182                        | +010  (dato / opcode no decodificado)
        .dc.w   0xffff                        | +012  (dato / opcode no decodificado)
        .dc.w   0x0001                        | +014  (dato / opcode no decodificado)
        .dc.w   0x0208                        | +016  (dato / opcode no decodificado)
        .dc.w   0x0025                        | +018  (dato / opcode no decodificado)
        .dc.w   0xc19a                        | +01a  (dato / opcode no decodificado)
        .dc.w   0xffff                        | +01c  (dato / opcode no decodificado)
        .dc.w   0x0002                        | +01e  (dato / opcode no decodificado)
        .dc.w   0x0208                        | +020  (dato / opcode no decodificado)
        .dc.w   0x0025                        | +022  (dato / opcode no decodificado)
        .dc.w   0xc1b2                        | +024  (dato / opcode no decodificado)
        .dc.w   0xffff                        | +026  (dato / opcode no decodificado)
        .dc.w   0x0002                        | +028  (dato / opcode no decodificado)
        .dc.w   0x0208                        | +02a  (dato / opcode no decodificado)
        .dc.w   0x0025                        | +02c  (dato / opcode no decodificado)
        .dc.w   0xc1ca                        | +02e  (dato / opcode no decodificado)
        .dc.w   0xffff                        | +030  (dato / opcode no decodificado)
        .dc.w   0x0002                        | +032  (dato / opcode no decodificado)
        .dc.w   0x0208                        | +034  (dato / opcode no decodificado)
        .dc.w   0x0025                        | +036  (dato / opcode no decodificado)
        .dc.w   0xc1e2                        | +038  (dato / opcode no decodificado)
        .dc.w   0xffff                        | +03a  (dato / opcode no decodificado)
        .dc.w   0x0002                        | +03c  (dato / opcode no decodificado)
        .dc.w   0x0208                        | +03e  (dato / opcode no decodificado)
        .dc.w   0x0025                        | +040  (dato / opcode no decodificado)
        .dc.w   0xc1f6                        | +042  (dato / opcode no decodificado)
        .dc.w   0xffff                        | +044  (dato / opcode no decodificado)
        .dc.w   0x0003                        | +046  (dato / opcode no decodificado)
        .dc.w   0x0208                        | +048  (dato / opcode no decodificado)
        .dc.w   0x0025                        | +04a  (dato / opcode no decodificado)
        .dc.w   0xc20a                        | +04c  (dato / opcode no decodificado)
        .dc.w   0xffff                        | +04e  (dato / opcode no decodificado)
        .dc.w   0x0003                        | +050  (dato / opcode no decodificado)
        .dc.w   0x0208                        | +052  (dato / opcode no decodificado)
        .dc.w   0x0025                        | +054  (dato / opcode no decodificado)
        .dc.w   0xc21e                        | +056  (dato / opcode no decodificado)
        .dc.w   0xffff                        | +058  (dato / opcode no decodificado)
        .dc.w   0x0003                        | +05a  (dato / opcode no decodificado)
        .dc.w   0x0208                        | +05c  (dato / opcode no decodificado)
        .dc.w   0x0025                        | +05e  (dato / opcode no decodificado)
        .dc.w   0xc232                        | +060  (dato / opcode no decodificado)
        .dc.w   0xffff                        | +062  (dato / opcode no decodificado)
        .dc.w   0x1600                        | +064  (dato / opcode no decodificado)

| ----------------------------------------------------------------------------
|  ShellCasing_Rocket_AnimB_03d5b0  @ $03D5B0  (102 B)
| ----------------------------------------------------------------------------
        .section .text.ShellCasing_Rocket_AnimB_03d5b0, "ax", @progbits
        .global ShellCasing_Rocket_AnimB_03d5b0
ShellCasing_Rocket_AnimB_03d5b0:
        .dc.w   0x0001                        | +000  (dato / opcode no decodificado)
        .dc.w   0x0208                        | +002  (dato / opcode no decodificado)
        .dc.w   0x0025                        | +004  (dato / opcode no decodificado)
        .dc.w   0xc256                        | +006  (dato / opcode no decodificado)
        .dc.w   0xffff                        | +008  (dato / opcode no decodificado)
        .dc.w   0x0001                        | +00a  (dato / opcode no decodificado)
        .dc.w   0x0208                        | +00c  (dato / opcode no decodificado)
        .dc.w   0x0025                        | +00e  (dato / opcode no decodificado)
        .dc.w   0xc266                        | +010  (dato / opcode no decodificado)
        .dc.w   0xffff                        | +012  (dato / opcode no decodificado)
        .dc.w   0x0001                        | +014  (dato / opcode no decodificado)
        .dc.w   0x0208                        | +016  (dato / opcode no decodificado)
        .dc.w   0x0025                        | +018  (dato / opcode no decodificado)
        .dc.w   0xc27e                        | +01a  (dato / opcode no decodificado)
        .dc.w   0xffff                        | +01c  (dato / opcode no decodificado)
        .dc.w   0x0002                        | +01e  (dato / opcode no decodificado)
        .dc.w   0x0208                        | +020  (dato / opcode no decodificado)
        .dc.w   0x0025                        | +022  (dato / opcode no decodificado)
        .dc.w   0xc296                        | +024  (dato / opcode no decodificado)
        .dc.w   0xffff                        | +026  (dato / opcode no decodificado)
        .dc.w   0x0002                        | +028  (dato / opcode no decodificado)
        .dc.w   0x0208                        | +02a  (dato / opcode no decodificado)
        .dc.w   0x0025                        | +02c  (dato / opcode no decodificado)
        .dc.w   0xc2aa                        | +02e  (dato / opcode no decodificado)
        .dc.w   0xffff                        | +030  (dato / opcode no decodificado)
        .dc.w   0x0002                        | +032  (dato / opcode no decodificado)
        .dc.w   0x0208                        | +034  (dato / opcode no decodificado)
        .dc.w   0x0025                        | +036  (dato / opcode no decodificado)
        .dc.w   0xc2be                        | +038  (dato / opcode no decodificado)
        .dc.w   0xffff                        | +03a  (dato / opcode no decodificado)
        .dc.w   0x0002                        | +03c  (dato / opcode no decodificado)
        .dc.w   0x0208                        | +03e  (dato / opcode no decodificado)
        .dc.w   0x0025                        | +040  (dato / opcode no decodificado)
        .dc.w   0xc2d2                        | +042  (dato / opcode no decodificado)
        .dc.w   0xffff                        | +044  (dato / opcode no decodificado)
        .dc.w   0x0003                        | +046  (dato / opcode no decodificado)
        .dc.w   0x0208                        | +048  (dato / opcode no decodificado)
        .dc.w   0x0025                        | +04a  (dato / opcode no decodificado)
        .dc.w   0xc2e6                        | +04c  (dato / opcode no decodificado)
        .dc.w   0xffff                        | +04e  (dato / opcode no decodificado)
        .dc.w   0x0003                        | +050  (dato / opcode no decodificado)
        .dc.w   0x0208                        | +052  (dato / opcode no decodificado)
        .dc.w   0x0025                        | +054  (dato / opcode no decodificado)
        .dc.w   0xc2fa                        | +056  (dato / opcode no decodificado)
        .dc.w   0xffff                        | +058  (dato / opcode no decodificado)
        .dc.w   0x0003                        | +05a  (dato / opcode no decodificado)
        .dc.w   0x0208                        | +05c  (dato / opcode no decodificado)
        .dc.w   0x0025                        | +05e  (dato / opcode no decodificado)
        .dc.w   0xc30e                        | +060  (dato / opcode no decodificado)
        .dc.w   0xffff                        | +062  (dato / opcode no decodificado)
        .dc.w   0x1600                        | +064  (dato / opcode no decodificado)

| ----------------------------------------------------------------------------
|  Scene3Debris_Spawn_03d616  @ $03D616  (42 B)
| ----------------------------------------------------------------------------
        .section .text.Scene3Debris_Spawn_03d616, "ax", @progbits
        .global Scene3Debris_Spawn_03d616
Scene3Debris_Spawn_03d616:
        cmpi.b  #0x3,0x106ece.l                 | +000
        bne.w   .L03d63e                        | +008
        lea     Scene3Debris_Task_03d76e(pc),a1 | +00c
        jsr     0x4ae.l                         | +010
        jsr     0x5dd02.l                       | +016
        andi.w  #0xffe3,0x38(a0)                | +01c
        ori.w   #0x10,0x38(a0)                  | +022
.L03d63e:
        rts                                     | +028

| ----------------------------------------------------------------------------
|  Scene3Debris_SpawnLow8_03d640  @ $03D640  (46 B)
| ----------------------------------------------------------------------------
        .section .text.Scene3Debris_SpawnLow8_03d640, "ax", @progbits
        .global Scene3Debris_SpawnLow8_03d640
Scene3Debris_SpawnLow8_03d640:
        cmpi.b  #0x3,0x106ece.l                 | +000
        bne.w   .L03d66c                        | +008
        lea     Scene3Debris_Task_03d76e(pc),a1 | +00c
        jsr     0x4ae.l                         | +010
        jsr     0x5dd02.l                       | +016
        andi.w  #0xffe3,0x38(a0)                | +01c
        ori.w   #0x10,0x38(a0)                  | +022
        addq.w  #0x8,0x24(a0)                   | +028
.L03d66c:
        rts                                     | +02c

| ----------------------------------------------------------------------------
|  Scene3Debris_SpawnUpD_03d66e  @ $03D66E  (70 B)
| ----------------------------------------------------------------------------
        .section .text.Scene3Debris_SpawnUpD_03d66e, "ax", @progbits
        .global Scene3Debris_SpawnUpD_03d66e
Scene3Debris_SpawnUpD_03d66e:
        cmpi.b  #0x3,0x106ece.l                 | +000
        bne.w   .L03d6b2                        | +008
        lea     Scene3Debris_Task_03d76e(pc),a1 | +00c
        jsr     0x4ae.l                         | +010
        jsr     0x5dd02.l                       | +016
        andi.w  #0xffe3,0x38(a0)                | +01c
        ori.w   #0x10,0x38(a0)                  | +022
        subi.w  #0xd,0x24(a0)                   | +028
        btst    #0x0,0x3a(a6)                   | +02e
        bne.w   .L03d6ae                        | +034
        addq.w  #0x3,0x22(a0)                   | +038
        bra.w   .L03d6b2                        | +03c
.L03d6ae:
        subq.w  #0x3,0x22(a0)                   | +040
.L03d6b2:
        rts                                     | +044

| ----------------------------------------------------------------------------
|  Scene3Debris_SpawnUpC_03d6b4  @ $03D6B4  (48 B)
| ----------------------------------------------------------------------------
        .section .text.Scene3Debris_SpawnUpC_03d6b4, "ax", @progbits
        .global Scene3Debris_SpawnUpC_03d6b4
Scene3Debris_SpawnUpC_03d6b4:
        cmpi.b  #0x3,0x106ece.l                 | +000
        bne.w   .L03d6e2                        | +008
        lea     Scene3Debris_Task_03d76e(pc),a1 | +00c
        jsr     0x4ae.l                         | +010
        jsr     0x5dd02.l                       | +016
        andi.w  #0xffe3,0x38(a0)                | +01c
        ori.w   #0x10,0x38(a0)                  | +022
        subi.w  #0xc,0x24(a0)                   | +028
.L03d6e2:
        rts                                     | +02e

| ----------------------------------------------------------------------------
|  Scene3Debris_SpawnUpB_03d6e4  @ $03D6E4  (70 B)
| ----------------------------------------------------------------------------
        .section .text.Scene3Debris_SpawnUpB_03d6e4, "ax", @progbits
        .global Scene3Debris_SpawnUpB_03d6e4
Scene3Debris_SpawnUpB_03d6e4:
        cmpi.b  #0x3,0x106ece.l                 | +000
        bne.w   .L03d728                        | +008
        lea     Scene3Debris_Task_03d76e(pc),a1 | +00c
        jsr     0x4ae.l                         | +010
        jsr     0x5dd02.l                       | +016
        andi.w  #0xffe3,0x38(a0)                | +01c
        ori.w   #0x10,0x38(a0)                  | +022
        subi.w  #0xb,0x24(a0)                   | +028
        btst    #0x0,0x3a(a6)                   | +02e
        bne.w   .L03d724                        | +034
        subq.w  #0x4,0x22(a0)                   | +038
        bra.w   .L03d728                        | +03c
.L03d724:
        addq.w  #0x4,0x22(a0)                   | +040
.L03d728:
        rts                                     | +044

| ----------------------------------------------------------------------------
|  Scene3Debris_SpawnPrio4_03d72a  @ $03D72A  (68 B)
| ----------------------------------------------------------------------------
        .section .text.Scene3Debris_SpawnPrio4_03d72a, "ax", @progbits
        .global Scene3Debris_SpawnPrio4_03d72a
Scene3Debris_SpawnPrio4_03d72a:
        cmpi.b  #0x3,0x106ece.l                 | +000
        bne.w   .L03d76c                        | +008
        lea     Scene3Debris_Task_03d76e(pc),a1 | +00c
        jsr     0x4ae.l                         | +010
        jsr     0x5dd02.l                       | +016
        andi.w  #0xffe3,0x38(a0)                | +01c
        ori.w   #0x4,0x38(a0)                   | +022
        subq.w  #0x6,0x24(a0)                   | +028
        btst    #0x0,0x3a(a6)                   | +02c
        bne.w   .L03d768                        | +032
        subq.w  #0x3,0x22(a0)                   | +036
        bra.w   .L03d76c                        | +03a
.L03d768:
        addq.w  #0x3,0x22(a0)                   | +03e
.L03d76c:
        rts                                     | +042

| ----------------------------------------------------------------------------
|  Scene3Debris_Task_03d76e  @ $03D76E  (50 B)
| ----------------------------------------------------------------------------
        .section .text.Scene3Debris_Task_03d76e, "ax", @progbits
        .global Scene3Debris_Task_03d76e
Scene3Debris_Task_03d76e:
        move.w  #0x1bd,d1                       | +000
        jsr     0x236e.l                        | +004
        lea     Scene3Debris_Anim_03d7a0(pc),a0 | +00a
        jsr     0x28cd4.l                       | +00e
        lea     .L03d788(pc),a1                 | +014
        move.l  a1,(a6)                         | +018
.L03d788:
        jsr     0x2783a.l                       | +01a
        jsr     0x28d70.l                       | +020
        bcc.w   .L03d79e                        | +026
        jmp     0x518.l                         | +02a
.L03d79e:
        rts                                     | +030

| ----------------------------------------------------------------------------
|  Scene3Debris_Anim_03d7a0  @ $03D7A0  (162 B)
| ----------------------------------------------------------------------------
        .section .text.Scene3Debris_Anim_03d7a0, "ax", @progbits
        .global Scene3Debris_Anim_03d7a0
Scene3Debris_Anim_03d7a0:
        .dc.w   0x0001                        | +000  (dato / opcode no decodificado)
        .dc.w   0x0208                        | +002  (dato / opcode no decodificado)
        .dc.w   0x0025                        | +004  (dato / opcode no decodificado)
        .dc.w   0xc990                        | +006  (dato / opcode no decodificado)
        .dc.w   0xffff                        | +008  (dato / opcode no decodificado)
        .dc.w   0x0001                        | +00a  (dato / opcode no decodificado)
        .dc.w   0x0208                        | +00c  (dato / opcode no decodificado)
        .dc.w   0x0025                        | +00e  (dato / opcode no decodificado)
        .dc.w   0xc99a                        | +010  (dato / opcode no decodificado)
        .dc.w   0xffff                        | +012  (dato / opcode no decodificado)
        .dc.w   0x0001                        | +014  (dato / opcode no decodificado)
        .dc.w   0x0208                        | +016  (dato / opcode no decodificado)
        .dc.w   0x0025                        | +018  (dato / opcode no decodificado)
        .dc.w   0xc9a4                        | +01a  (dato / opcode no decodificado)
        .dc.w   0xffff                        | +01c  (dato / opcode no decodificado)
        .dc.w   0x0001                        | +01e  (dato / opcode no decodificado)
        .dc.w   0x0208                        | +020  (dato / opcode no decodificado)
        .dc.w   0x0025                        | +022  (dato / opcode no decodificado)
        .dc.w   0xc9ae                        | +024  (dato / opcode no decodificado)
        .dc.w   0xffff                        | +026  (dato / opcode no decodificado)
        .dc.w   0x0001                        | +028  (dato / opcode no decodificado)
        .dc.w   0x0208                        | +02a  (dato / opcode no decodificado)
        .dc.w   0x0025                        | +02c  (dato / opcode no decodificado)
        .dc.w   0xc9b8                        | +02e  (dato / opcode no decodificado)
        .dc.w   0xffff                        | +030  (dato / opcode no decodificado)
        .dc.w   0x0001                        | +032  (dato / opcode no decodificado)
        .dc.w   0x0208                        | +034  (dato / opcode no decodificado)
        .dc.w   0x0025                        | +036  (dato / opcode no decodificado)
        .dc.w   0xc9c2                        | +038  (dato / opcode no decodificado)
        .dc.w   0xffff                        | +03a  (dato / opcode no decodificado)
        .dc.w   0x0001                        | +03c  (dato / opcode no decodificado)
        .dc.w   0x0208                        | +03e  (dato / opcode no decodificado)
        .dc.w   0x0025                        | +040  (dato / opcode no decodificado)
        .dc.w   0xc9cc                        | +042  (dato / opcode no decodificado)
        .dc.w   0xffff                        | +044  (dato / opcode no decodificado)
        .dc.w   0x0001                        | +046  (dato / opcode no decodificado)
        .dc.w   0x0208                        | +048  (dato / opcode no decodificado)
        .dc.w   0x0025                        | +04a  (dato / opcode no decodificado)
        .dc.w   0xc9d6                        | +04c  (dato / opcode no decodificado)
        .dc.w   0xffff                        | +04e  (dato / opcode no decodificado)
        .dc.w   0x0002                        | +050  (dato / opcode no decodificado)
        .dc.w   0x0208                        | +052  (dato / opcode no decodificado)
        .dc.w   0x0025                        | +054  (dato / opcode no decodificado)
        .dc.w   0xc9e0                        | +056  (dato / opcode no decodificado)
        .dc.w   0xffff                        | +058  (dato / opcode no decodificado)
        .dc.w   0x0002                        | +05a  (dato / opcode no decodificado)
        .dc.w   0x0208                        | +05c  (dato / opcode no decodificado)
        .dc.w   0x0025                        | +05e  (dato / opcode no decodificado)
        .dc.w   0xc9ea                        | +060  (dato / opcode no decodificado)
        .dc.w   0xffff                        | +062  (dato / opcode no decodificado)
        .dc.w   0x0002                        | +064  (dato / opcode no decodificado)
        .dc.w   0x0208                        | +066  (dato / opcode no decodificado)
        .dc.w   0x0025                        | +068  (dato / opcode no decodificado)
        .dc.w   0xc9f4                        | +06a  (dato / opcode no decodificado)
        .dc.w   0xffff                        | +06c  (dato / opcode no decodificado)
        .dc.w   0x0002                        | +06e  (dato / opcode no decodificado)
        .dc.w   0x0208                        | +070  (dato / opcode no decodificado)
        .dc.w   0x0025                        | +072  (dato / opcode no decodificado)
        .dc.w   0xca00                        | +074  (dato / opcode no decodificado)
        .dc.w   0xffff                        | +076  (dato / opcode no decodificado)
        .dc.w   0x0002                        | +078  (dato / opcode no decodificado)
        .dc.w   0x0208                        | +07a  (dato / opcode no decodificado)
        .dc.w   0x0025                        | +07c  (dato / opcode no decodificado)
        .dc.w   0xca0a                        | +07e  (dato / opcode no decodificado)
        .dc.w   0xffff                        | +080  (dato / opcode no decodificado)
        .dc.w   0x0002                        | +082  (dato / opcode no decodificado)
        .dc.w   0x0208                        | +084  (dato / opcode no decodificado)
        .dc.w   0x0025                        | +086  (dato / opcode no decodificado)
        .dc.w   0xca14                        | +088  (dato / opcode no decodificado)
        .dc.w   0xffff                        | +08a  (dato / opcode no decodificado)
        .dc.w   0x0002                        | +08c  (dato / opcode no decodificado)
        .dc.w   0x0208                        | +08e  (dato / opcode no decodificado)
        .dc.w   0x0025                        | +090  (dato / opcode no decodificado)
        .dc.w   0xca1e                        | +092  (dato / opcode no decodificado)
        .dc.w   0xffff                        | +094  (dato / opcode no decodificado)
        .dc.w   0x0002                        | +096  (dato / opcode no decodificado)
        .dc.w   0x0208                        | +098  (dato / opcode no decodificado)
        .dc.w   0x0025                        | +09a  (dato / opcode no decodificado)
        .dc.w   0xca28                        | +09c  (dato / opcode no decodificado)
        .dc.w   0xffff                        | +09e  (dato / opcode no decodificado)
        .dc.w   0x1600                        | +0a0  (dato / opcode no decodificado)

| ----------------------------------------------------------------------------
|  Player_DebugMarker_03d842  @ $03D842  (82 B)
| ----------------------------------------------------------------------------
        .section .text.Player_DebugMarker_03d842, "ax", @progbits
        .global Player_DebugMarker_03d842
Player_DebugMarker_03d842:
        lea     Player_DebugMarker_Anim_03d894(pc),a0 | +000
        jsr     0x28cd4.l                       | +004
        bra.w   .L03d850                        | +00a
.L03d850:
        move.w  #0x17a,d1                       | +00e
        jsr     0x236e.l                        | +012
        move.w  #0xd000,d0                      | +018
        jsr     0x28134.l                       | +01c
        andi.w  #0xffe3,0x38(a6)                | +022
        ori.w   #0x10,0x38(a6)                  | +028
        addi.w  #0x200,0x38(a6)                 | +02e
        lea     .L03d87c(pc),a1                 | +034
        move.l  a1,(a6)                         | +038
.L03d87c:
        jsr     0x2783a.l                       | +03a
        jsr     0x28d70.l                       | +040
        bcc.w   .L03d892                        | +046
        jmp     0x518.l                         | +04a
.L03d892:
        rts                                     | +050

| ----------------------------------------------------------------------------
|  Player_DebugMarker_Anim_03d894  @ $03D894  (102 B)
| ----------------------------------------------------------------------------
        .section .text.Player_DebugMarker_Anim_03d894, "ax", @progbits
        .global Player_DebugMarker_Anim_03d894
Player_DebugMarker_Anim_03d894:
        .dc.w   0x0001                        | +000  (dato / opcode no decodificado)
        .dc.w   0x0208                        | +002  (dato / opcode no decodificado)
        .dc.w   0x0026                        | +004  (dato / opcode no decodificado)
        .dc.w   0x27b0                        | +006  (dato / opcode no decodificado)
        .dc.w   0xffff                        | +008  (dato / opcode no decodificado)
        .dc.w   0x0001                        | +00a  (dato / opcode no decodificado)
        .dc.w   0x0208                        | +00c  (dato / opcode no decodificado)
        .dc.w   0x0026                        | +00e  (dato / opcode no decodificado)
        .dc.w   0x27c4                        | +010  (dato / opcode no decodificado)
        .dc.w   0xffff                        | +012  (dato / opcode no decodificado)
        .dc.w   0x0001                        | +014  (dato / opcode no decodificado)
        .dc.w   0x0208                        | +016  (dato / opcode no decodificado)
        .dc.w   0x0026                        | +018  (dato / opcode no decodificado)
        .dc.w   0x27d4                        | +01a  (dato / opcode no decodificado)
        .dc.w   0xffff                        | +01c  (dato / opcode no decodificado)
        .dc.w   0x0001                        | +01e  (dato / opcode no decodificado)
        .dc.w   0x0208                        | +020  (dato / opcode no decodificado)
        .dc.w   0x0026                        | +022  (dato / opcode no decodificado)
        .dc.w   0x27e4                        | +024  (dato / opcode no decodificado)
        .dc.w   0xffff                        | +026  (dato / opcode no decodificado)
        .dc.w   0x0002                        | +028  (dato / opcode no decodificado)
        .dc.w   0x0208                        | +02a  (dato / opcode no decodificado)
        .dc.w   0x0026                        | +02c  (dato / opcode no decodificado)
        .dc.w   0x27fc                        | +02e  (dato / opcode no decodificado)
        .dc.w   0xffff                        | +030  (dato / opcode no decodificado)
        .dc.w   0x0002                        | +032  (dato / opcode no decodificado)
        .dc.w   0x0208                        | +034  (dato / opcode no decodificado)
        .dc.w   0x0026                        | +036  (dato / opcode no decodificado)
        .dc.w   0x2814                        | +038  (dato / opcode no decodificado)
        .dc.w   0xffff                        | +03a  (dato / opcode no decodificado)
        .dc.w   0x0002                        | +03c  (dato / opcode no decodificado)
        .dc.w   0x0208                        | +03e  (dato / opcode no decodificado)
        .dc.w   0x0026                        | +040  (dato / opcode no decodificado)
        .dc.w   0x282e                        | +042  (dato / opcode no decodificado)
        .dc.w   0xffff                        | +044  (dato / opcode no decodificado)
        .dc.w   0x0002                        | +046  (dato / opcode no decodificado)
        .dc.w   0x0208                        | +048  (dato / opcode no decodificado)
        .dc.w   0x0026                        | +04a  (dato / opcode no decodificado)
        .dc.w   0x284a                        | +04c  (dato / opcode no decodificado)
        .dc.w   0xffff                        | +04e  (dato / opcode no decodificado)
        .dc.w   0x0002                        | +050  (dato / opcode no decodificado)
        .dc.w   0x0208                        | +052  (dato / opcode no decodificado)
        .dc.w   0x0026                        | +054  (dato / opcode no decodificado)
        .dc.w   0x2862                        | +056  (dato / opcode no decodificado)
        .dc.w   0xffff                        | +058  (dato / opcode no decodificado)
        .dc.w   0x0002                        | +05a  (dato / opcode no decodificado)
        .dc.w   0x0208                        | +05c  (dato / opcode no decodificado)
        .dc.w   0x0026                        | +05e  (dato / opcode no decodificado)
        .dc.w   0x2872                        | +060  (dato / opcode no decodificado)
        .dc.w   0xffff                        | +062  (dato / opcode no decodificado)
        .dc.w   0x1600                        | +064  (dato / opcode no decodificado)

| ----------------------------------------------------------------------------
|  Player_SpawnFx3D8FA_03d8fa  @ $03D8FA  (74 B)
| ----------------------------------------------------------------------------
        .section .text.Player_SpawnFx3D8FA_03d8fa, "ax", @progbits
        .global Player_SpawnFx3D8FA_03d8fa
Player_SpawnFx3D8FA_03d8fa:
        move.w  #0x17c,d1                       | +000
        jsr     0x236e.l                        | +004
        move.w  #0x2000,d0                      | +00a
        jsr     0x28134.l                       | +00e
        andi.w  #0xffe3,0x38(a6)                | +014
        ori.w   #0x10,0x38(a6)                  | +01a
        lea     0x2f7d3a.l,a0                   | +020
        jsr     0x28cd4.l                       | +026
        lea     .L03d92c(pc),a1                 | +02c
        move.l  a1,(a6)                         | +030
.L03d92c:
        jsr     0x2783a.l                       | +032
        jsr     0x28d70.l                       | +038
        bcc.w   .L03d942                        | +03e
        jmp     0x518.l                         | +042
.L03d942:
        rts                                     | +048

| ----------------------------------------------------------------------------
|  SlugCannon_ArmOverlay_03d944  @ $03D944  (190 B)
| ----------------------------------------------------------------------------
        .section .text.SlugCannon_ArmOverlay_03d944, "ax", @progbits
        .global SlugCannon_ArmOverlay_03d944
SlugCannon_ArmOverlay_03d944:
        lea     0x100440.l,a0                   | +000
        jsr     0x2ac0e.l                       | +006
        bcc.w   .L03d98e                        | +00c
        move.w  #0x176,d1                       | +010
        jsr     0x236e.l                        | +014
        move.w  #0x190,d1                       | +01a
        jsr     0x236e.l                        | +01e
        move.w  #0x191,d1                       | +024
        jsr     0x236e.l                        | +028
        move.w  #0x1b,0x1c(a6)                  | +02e
        jsr     0x138fe.l                       | +034
        bclr    #0x4,0x12(a6)                   | +03a
        lea     0x100440.l,a0                   | +040
        bra.w   .L03d9dc                        | +046
.L03d98e:
        lea     0x1004e0.l,a0                   | +04a
        jsr     0x2ac0e.l                       | +050
        bcc.w   .L03d9d8                        | +056
        move.w  #0x177,d1                       | +05a
        jsr     0x236e.l                        | +05e
        move.w  #0x190,d1                       | +064
        jsr     0x236e.l                        | +068
        move.w  #0x192,d1                       | +06e
        jsr     0x236e.l                        | +072
        move.w  #0x1c,0x1c(a6)                  | +078
        jsr     0x138fe.l                       | +07e
        bset    #0x4,0x12(a6)                   | +084
        lea     0x1004e0.l,a0                   | +08a
        bra.w   .L03d9dc                        | +090
.L03d9d8:
        jmp     JmpToScheduler_03daa0(pc)       | +094
.L03d9dc:
        moveq   #0,d0                           | +098
        move.b  0x71(a0),d0                     | +09a
        cmpa.l  #0x1004e0,a0                    | +09e
        bne.w   .L03d9ee                        | +0a4
        addq.w  #0x5,d0                         | +0a8
.L03d9ee:
        lsl.w   #0x2,d0                         | +0aa
        lea     SlugCannon_ArmSpriteTbl_03da02(pc),a0 | +0ac
        movea.l (a0,d0.w),a0                    | +0b0
        jsr     0x28cd4.l                       | +0b4
        bra.w   SlugCannon_ArmSpriteTbl_03da02__L03da2a | +0ba

| ----------------------------------------------------------------------------
|  SlugCannon_ArmSpriteTbl_03da02  @ $03DA02  (150 B)
| ----------------------------------------------------------------------------
        .section .text.SlugCannon_ArmSpriteTbl_03da02, "ax", @progbits
        .global SlugCannon_ArmSpriteTbl_03da02
SlugCannon_ArmSpriteTbl_03da02:
        .dc.w   0x0027                        | +000  (dato / opcode no decodificado)
        .dc.w   0xd9e6                        | +002  (dato / opcode no decodificado)
        .dc.w   0x0028                        | +004  (dato / opcode no decodificado)
        .dc.w   0x28b4                        | +006  (dato / opcode no decodificado)
        .dc.w   0x0028                        | +008  (dato / opcode no decodificado)
        .dc.w   0x28b4                        | +00a  (dato / opcode no decodificado)
        .dc.w   0x0028                        | +00c  (dato / opcode no decodificado)
        .dc.w   0x28b4                        | +00e  (dato / opcode no decodificado)
        .dc.w   0x0028                        | +010  (dato / opcode no decodificado)
        .dc.w   0x28b4                        | +012  (dato / opcode no decodificado)
        .dc.w   0x0027                        | +014  (dato / opcode no decodificado)
        .dc.w   0xd9e6                        | +016  (dato / opcode no decodificado)
        .dc.w   0x0028                        | +018  (dato / opcode no decodificado)
        .dc.w   0x28b4                        | +01a  (dato / opcode no decodificado)
        .dc.w   0x0028                        | +01c  (dato / opcode no decodificado)
        .dc.w   0x28b4                        | +01e  (dato / opcode no decodificado)
        .dc.w   0x0028                        | +020  (dato / opcode no decodificado)
        .dc.w   0x28b4                        | +022  (dato / opcode no decodificado)
        .dc.w   0x0028                        | +024  (dato / opcode no decodificado)
        .dc.w   0x28b4                        | +026  (dato / opcode no decodificado)
        .global SlugCannon_ArmSpriteTbl_03da02__L03da2a
SlugCannon_ArmSpriteTbl_03da02__L03da2a:
        jsr     0x2ff8e.l                       | +028
        lsl.w   #0x2,d0                         | +02e
        lea     Sub_0003DAA8(pc),a0             | +030
        move.w  (a0,d0.w),d1                    | +034
        move.w  0x2(a0,d0.w),d2                 | +038
        move.w  d1,0x5c(a6)                     | +03c
        move.w  d2,0x5e(a6)                     | +040
        lea     .L03da4c(pc),a1                 | +044
        move.l  a1,(a6)                         | +048
.L03da4c:
        jsr     0x2783a.l                       | +04a
        lea     0x100580.l,a0                   | +050
        move.w  0x22(a0),d0                     | +056
        move.w  0x24(a0),d1                     | +05a
        move.w  0x38(a0),d2                     | +05e
        add.w   0x5c(a6),d0                     | +062
        add.w   0x5e(a6),d1                     | +066
        move.w  d0,0x22(a6)                     | +06a
        move.w  d1,0x24(a6)                     | +06e
        move.w  d2,0x38(a6)                     | +072
        andi.w  #0xffe3,0x38(a6)                | +076
        ori.w   #0x4,0x38(a6)                   | +07c
        jsr     0x28d70.l                       | +082
        bcc.w   JsrAbsRts_03da9e                | +088
        lea     JmpToScheduler_03daa0(pc),a1    | +08c
        move.l  a1,(a6)                         | +090
        move.w  #0x108b,d0                      | +092
