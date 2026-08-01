| ============================================================================
|  Metal Slug 1 - asm/player_route_publish_033xxx.s
|  ----------------------------------------------------------------------------
|  Wave NN batch 1 - helper micro + publicador estado (Top-1 del scan
|  corregido: PlayerRoute_PublishState_033522 con 18 callers reales).
|
|  Contenido (3 funciones, 186 bytes, 4 FPs absorbidos):
|
|      $0334A2   Probe_Bit3At100001_0334A2         36 B  probe DIP switch bit
|      $033522   PlayerRoute_PublishState_033522   72 B  Top-1 del scan
|      $027EBA   SpritePubEffect_027EBA            78 B  publicador efecto sprite
|
|  ---------- Descubrimientos arquitectonicos Wave NN batch 1 --------------
|
|  1. **Top-1 del scan corregido** materializado. Tras el bug fix del
|     escaner (que retiro los falsos "callers" a la vector table del
|     68000), el nuevo Top-1 con 18 callers reales resulto ser
|     PlayerRoute_PublishState_033522: un publicador de estado que
|     ilustra el patron canonico "publica siguiente handler en (a6) +
|     invoca InputGuardCall219c" que impregna todo el sistema.
|
|  2. **CUATRO falsos positivos absorbidos** en un solo batch (record del
|     proyecto tras Wave KK#2 con 7):
|
|     - ClearXN_0334c0 (6 B) = cola CCR-clear final de Probe_Bit3At100001.
|       Wave N clasifico `andi.b #$EE, ccr; rts` como funcion
|       independiente cuando en realidad es la salida "colision detectada"
|       del probe.
|     - JsrAbsThunk_03356a (8 B) = cola tail-call final de PlayerRoute_
|       PublishState. Wave I clasifico `jsr $2352.l; rts` como thunk
|       independiente cuando es la 2a salida del publicador (rama P2).
|     - ClearXN_027efc (6 B) = cola CCR-clear "colision detectada" de
|       SpritePubEffect. `andi.b #$EE, ccr; rts` clasificado por Wave N
|       como setter independiente.
|     - SetXN_027f02 (6 B) = cola CCR-set "sin colision" de
|       SpritePubEffect. `ori.b #$11, ccr; rts` clasificado por Wave N
|       como setter independiente.
|
|     Los 4 FPs (26 B totales) reafirman por 3a vez el patron ya visto en
|     Wave KK#2: **Wave N sobre-cuenta sistematicamente las colas CCR
|     como funciones independientes cuando en realidad son epilogos con
|     retorno por flag** del handler que las precede.
|
|  3. **Idioma "publica-siguiente-handler-en-(a6) + invoca-input-guard"**
|     documentado por primera vez. PlayerRoute_PublishState_033522 hace:
|
|       jsr Probe_Bit3At100001(pc)           ; test DIP bit 3
|       bcc.w .Lret_no_dip                    ; if bit clear, exit
|       jsr SpritePubEffect_027EBA.l          ; effect w/ CCR-C return
|       bcs.w .Lroute_b                        ; if C set (colision),
|                                              ;   goto ruta B
|       lea PlayerHandlerA(pc), a1            ; ruta A: publica handler A
|       move.l a1, (a6)                        ;
|       bra.w .Ldispatch                       ;
|      .Lroute_b:
|       lea PlayerHandlerB(pc), a1            ; ruta B: publica handler B
|       move.l a1, (a6)                        ;
|      .Ldispatch:
|       cmpi.b #5, $58(a6)                    ; validar slot state
|       beq.w .Lret                            ; if ya terminado, exit
|       cmpa.l #$100440, a6                   ; testear si P1 o P2 slot
|       bne.w .Lp2                             ; if P2, saltar
|       move.w #$1123, d0                     ; P1 input guard code
|       jsr InputGuardCall219c.l              ; publicar
|       bra.w .Lret                            ;
|      .Lp2:
|       move.w #$1087, d0                     ; P2 input guard code
|       ; fall-through natural a JsrAbsThunk_03356a (absorbido):
|       ;   jsr InputGuardCall219c.l
|       ;   rts
|
|     Es el "state-machine setter con doble ruta y publicacion de codigo
|     de entrada al input guard". Reaparece en ~18 sitios del ROM. GCC no
|     genera este patron - usaria un switch/case + call convencional.
|
|  4. **`$100440` = P1 slot base**. Deducido del `cmpa.l #$100440, a6`
|     como test de discriminacion P1 vs P2. Confirmado indirectamente:
|     $100440 = $100000 (RAM base) + $440 = offset del TCB del player 1
|     activo. El TCB del player 2 estara en $100440 + N donde N es el
|     tamano del TCB (aun por medir).
|
|  5. **`$106F44`** identificado como registro MMIO de estado de efecto
|     de sprite (publicado por SpritePubEffect_027EBA con `d6` que viene
|     del helper `$27DB2` / `$9993C`). Es el "current effect id" que
|     leen otros subsistemas.
|
|  6. **Dual-CCR-return por rts explicito** documentado en SpritePubEffect
|     con doble epilogo: `andi.b #$EE, ccr; rts` (Carry clear = sin
|     colision) y `ori.b #$11, ccr; rts` (Carry+Extend set = colision).
|     El caller (PlayerRoute) usa `bcs.w` para discriminar. Es el mismo
|     idioma que las Waves KK#2 (`Collision_ProbeRange/X/Y`) y N/S/T
|     (probe CCR helpers).
|
|  Toolchain: m68k-linux-gnu-gcc -m68000 -Wa,--register-prefix-optional
|  ============================================================================

        .text

|
| ---------------------------------------------------------------------------
|  Probe_Bit3At100001_0334A2  @ $0334A2  (36 bytes)
|
|  Probe de bit 3 del byte $100001.l (probable DIP switch de config -
|  Neo Geo mapea los soft-dips en la region $100000..$100010 tras el
|  arranque). Retorna por CCR-C:
|
|      C clear = bit set (dip NO activo) -- caller sigue por bcc
|      C set   = bit clear (dip activo)  -- caller salta por bcc
|
|  Efecto lateral: si bit 3 = 1, bclr bit 3 y 0 del byte $13(a6) (flag
|  de estado) y setea $66(a6) = 1 (contador de eventos).
|
|  Absorbe FP: ClearXN_0334c0 (6 B, cola `andi.b #$EE, ccr; rts` que
|  Wave N clasifico como funcion CCR-clear independiente).
| ---------------------------------------------------------------------------
|
        .globl  Probe_Bit3At100001_0334A2
        .globl  PcThunkTarget_0334a2            | alias legacy (Wave I callers)
        .type   Probe_Bit3At100001_0334A2, @function
        .section .text.Probe_Bit3At100001_0334A2, "ax", @progbits
Probe_Bit3At100001_0334A2:
PcThunkTarget_0334a2:
        btst.b  #0x3, 0x100001.l                | +00  test DIP bit 3
        beq.w   Probe_Bit3At100001_End          | +08  if clear -> exit (fall-out to $0334C6)
        bclr.b  #0x3, 0x13(a6)                  | +0c  clear flag bit 3
        bclr.b  #0x0, 0x13(a6)                  | +12  clear flag bit 0
        move.w  #0x1, 0x66(a6)                  | +18  event_ctr = 1
        andi.b  #0xee, ccr                      | +1e  CCR = X clr, N=?, Z=?, V clr, C clr
        rts                                     | +22

        .size   Probe_Bit3At100001_0334A2, .-Probe_Bit3At100001_0334A2

|
| ---------------------------------------------------------------------------
|  PlayerRoute_PublishState_033522  @ $033522  (72 bytes)
|
|  El Top-1 del scan corregido (18 callers reales). State-machine setter
|  con doble ruta que publica el siguiente handler en (a6) y dispatcha
|  hacia el input guard con el codigo de entrada apropiado segun sea el
|  slot P1 ($100440) o P2 (offset del anterior).
|
|  Firma conceptual:
|    void PlayerRoute_PublishState(void)
|      con a6 = puntero al TCB activo del player
|      efectos: probe DIP -> effect probe -> publica handler en (a6) ->
|               invoca InputGuardCall219c con codigo por-player
|
|  Absorbe FP: JsrAbsThunk_03356a (8 B, cola `jsr $2352.l; rts` que
|  Wave I clasifico como thunk independiente pero es la 2a salida del
|  publicador tras publicar el codigo P2).
| ---------------------------------------------------------------------------
|
        .globl  PlayerRoute_PublishState_033522
        .type   PlayerRoute_PublishState_033522, @function
        .section .text.PlayerRoute_PublishState_033522, "ax", @progbits
PlayerRoute_PublishState_033522:
        jsr     Probe_Bit3At100001_0334A2(pc)   | +00  probe DIP bit 3 (pc-rel)
        bcc.w   .Lroute_ret_early               | +04  if !C, salta al rts
        jsr     SpritePubEffect_027EBA          | +08  effect w/ dual-CCR return
        bcs.w   .Lroute_b                       | +0e  if C=set, ruta B
| ---- Ruta A: no colision, publica handler A ($033572)
        lea.l   PlayerHandlerA_033572(pc), a1   | +12  a1 = &handlerA
        move.l  a1, (a6)                        | +16  (a6) = a1  (publish)
        bra.w   .Lroute_dispatch                | +18
.Lroute_b:                                      | $03353E
        lea.l   PlayerHandlerB_033578(pc), a1   | +1c  a1 = &handlerB
        move.l  a1, (a6)                        | +20  (a6) = a1  (publish)
.Lroute_dispatch:                               | $033544
        cmpi.b  #0x5, 0x58(a6)                  | +22  if slot.state == 5
        beq.w   .Lroute_ret_early               | +28    goto ret
        cmpa.l  #0x100440, a6                   | +2c  test P1 slot base
        bne.w   .Lroute_p2                      | +32  if !=, ruta P2
| ---- Ruta P1: publicar code $1123
        move.w  #0x1123, d0                     | +36  d0 = P1 code
        jsr     InputGuardCall219c              | +3a
        bra.w   .Lroute_ret_early               | +40
.Lroute_p2:                                     | $033566
        move.w  #0x1087, d0                     | +44  d0 = P2 code
| ---- Fall-through natural a "JsrAbsThunk_03356a" (FP absorbido):
| $03356A: jsr InputGuardCall219c ; $033570: rts
        jsr     InputGuardCall219c              | +48
.Lroute_ret_early:                              | $033570
        rts                                     | +4e

        .size   PlayerRoute_PublishState_033522, .-PlayerRoute_PublishState_033522

|
| PlayerHandlerA_033572 y PlayerHandlerB_033578 son externals resueltos
| por --defsym en symbols.py (leccion Wave MM#1: .set XXX, 0xNNN NO deja
| reubicacion PC-rel; hay que usar externals no definidos).

|
| ---------------------------------------------------------------------------
|  SpritePubEffect_027EBA  @ $027EBA  (78 bytes)
|
|  Publicador de efecto de sprite con retorno CCR-C dual. Publica el
|  effect id en $106F44 y setea $5a(a6) bit 6 si detecta colision.
|
|  Firma conceptual:
|    bool_ccr SpritePubEffect(void)
|      con a6 = puntero al TCB activo
|      leyendo $22(a6) = x, $24(a6) = y
|      efectos: probe intersecion via $27DB2/$9993C -> publica effect id
|      retorna CCR-C: 0 = sin colision (default), 1 = colision detectada
|
|  Absorbe DOS FPs de Wave N:
|    ClearXN_027efc (6 B) = rama "colision", CCR = $EE (C clear + otros)
|    SetXN_027f02   (6 B) = rama "sin colision", CCR = $11 (C set + X set)
|
|  Los dos "setters CCR independientes" son en realidad los dos epilogos
|  con retorno por flag del mismo publicador. La razon del CCR "invertido"
|  (colision = C clear, sin colision = C set) es historica: el caller
|  usa `bcs.w .Lroute_b` para elegir la ruta B tras COLISION -- pero eso
|  significa que si NO hay colision (path del `bset.b #6, $5a(a6)` NO
|  tomado), la salida `ori.b #$11, ccr; rts` PONE C set indicando
|  "ejecuto sin problemas, sigue por ruta A". Es un idioma poco
|  ortodoxo pero consistente: **el bit C se usa como "usa la ruta que
|  el helper te propone"** en el caller de PlayerRoute.
| ---------------------------------------------------------------------------
|
        .globl  SpritePubEffect_027EBA
        .globl  Sub_00027EBA                    | alias legacy (Entity_ProbeAndInstallHandler_049FD0 caller)
        .type   SpritePubEffect_027EBA, @function
        .section .text.SpritePubEffect_027EBA, "ax", @progbits
SpritePubEffect_027EBA:
Sub_00027EBA:
        move.w  0x22(a6), d1                    | +00  d1 = x_coord
        move.w  0x24(a6), d2                    | +04  d2 = y_coord
        subq.w  #0x1, d2                        | +08  d2 -= 1
        jsr     Sub_00027DB2(pc)                | +0a  helper $27DB2 (pc-rel)
        jsr     Sub_0009993C                    | +0e  helper $9993C (abs.l)
        move.b  d6, 0x106f44.l                  | +14  publish effect_id
        cmpi.b  #0xf, d6                        | +1a  if d6 == $F
        beq.w   .Lspe_probe                     | +1e    skip setup
        moveq   #0x0, d0                        | +22  d0 = 0
        moveq   #0x0, d3                        | +24  d3 = 0
        move.w  d5, d4                          | +26  d4 = d5
        moveq   #0x11, d0                       | +28  d0 = $11
        lea.l   Data_00278BA8, a1               | +2a  a1 = &data $278BA8
        bra.w   .Lspe_probe                     | +30  (nop entry)
.Lspe_probe:                                    | $027EEE
        jsr     Sub_00027E28(pc)                | +34  probe $27E28 (pc-rel)
        bcc.w   .Lspe_no_collision              | +38  if !C, no collision
| ---- Rama COLISION detectada
        bset.b  #0x6, 0x5a(a6)                  | +3c  flags bit 6 = 1
        andi.b  #0xee, ccr                      | +42  CCR = X_clr, V_clr, C_clr
        rts                                     | +46
| ---- Rama SIN colision (fall-through target de bcc.w arriba)
.Lspe_no_collision:                             | $027F02
        ori.b   #0x11, ccr                      | +48  CCR = X_set, C_set
        rts                                     | +4c

        .size   SpritePubEffect_027EBA, .-SpritePubEffect_027EBA

|
| Externals resueltos por --defsym en symbols.py (Wave NN batch 1).
