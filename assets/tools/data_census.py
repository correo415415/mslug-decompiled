#!/usr/bin/env python3
"""
data_census.py - censo de los módulos de datos del P-ROM (asm/data/*.s).

Clasifica cada byte de las 14 regiones de datos según el tipo de asset que
el código del juego le da, usando los mismos decodificadores que los
extractores (sprites.py, levels.py, scene_script_dump.py):

  palette     paletas de 64 B ($14E00 x1024, $1CE00 x512)
  frame       frames de sprite {npiezas, attr, piezas{dx,dy,w,h,tiles}}
              (+ rachas sin puntero que se recorren enteras como frames)
  anim        bytecode de animación (op 02 y demás ops recorridos; más los
              scripts que el código instala con EntitySetSpriteMap, recorridos
              con walk_struct: sprite oculto $FFFFFFFF y tablas de 1B/1C)
  tilemap     mapas de celdas de fondo {tile.w, attr.w} de los bind_path
  collision   mapas de colisión de los bind_path (4 B por celda, w*h celdas)
              (+ parches por Sprite_InvokeBlit8Params: `lea rec,a2; jsr $5022A`,
              rec = {ent.l, desc.l, 6 w}, desc = {w, h, tiles.l}; sólo las
              columnas que los registros llegan a copiar)
  struct      estructuras decodificadas con formato documentado: bytecode de
              misión (MissionStream_/MissionAux_), plantillas (TemplateIndex_),
              tabla/scripts/entidades/triggers de escena (Scene*_), registros
              y descriptores de parche de Sprite_InvokeBlit8Params, cajas de
              Entity_CheckOnScreenBox, rejillas de $43FAC y registros de 12 B
              del op 19 de animación, diccionarios
              de nombres de Results_RosterPick (ROSTER_NAMES)
  lut         tablas generadas por fórmula (ColorRGB555ToNeoGeo, ColorRamp,
              SinTab256)
  zero        rachas de $00 >= 16 B
  other       resto (tablas de código, parámetros, strings, etc.)

    python3 assets/tools/data_census.py            # tabla por módulo
    python3 assets/tools/data_census.py --json F   # además vuelca a JSON
"""
import json, os, sys
from collections import Counter

HERE = os.path.dirname(os.path.abspath(__file__))
ROOT = os.path.dirname(os.path.dirname(HERE))
sys.path.insert(0, HERE)
sys.path.insert(0, os.path.join(ROOT, "tools"))
import sprites as S          # noqa: E402
import levels as LV          # noqa: E402
import registry              # noqa: E402

KINDS = ("palette", "frame", "anim", "tilemap", "collision", "struct", "lut", "zero", "other")

# prefijos de registry cuyo contenido entero ya está decodificado por su parser
STRUCT_PREFIXES = ("MissionStream_", "MissionAux_", "TemplateIndex_", "SceneDescTable_",
                   "SceneScript_", "SceneEntities_", "SceneTrig_", "TaskBootTab_", "SoftDip_", "PropItemPtrs_", "HudText_",
                   "CollOffsets_", "CollSlopes_", "CollTileTypes_", "CollMasks_", "HitShapeTab_",
                   "InputDemo")

# tablas de color del motor de paletas (asm/data/sprite_tables_002f30.s)
LUTS = ((0x002F30, 0x012F30), (0x012F30, 0x012FB0), (0x012FB0, 0x0133B0),
        (0x2C072C, 0x2C09AC),     # SinTab256_1c072c (CPU $2C072C, 320 palabras)
        # puntería por dx (Bazooka_AimStep_06b5ea / RocketVehicle_AimStep_06d340):
        # dirección 0..15 = T[(clamp(dx,-256,256)+256)>>4] (33 B por tabla, stride
        # $22) y la tabla de espejo dir -> (16-dir)&15 (16 B)
        (0x2CA5F0, 0x2CA644),     # Bazooka: arriba, abajo, espejo
        (0x2CD4EA, 0x2CD606))     # RocketVehicle: 2 x 8 punteros, 6 tablas, espejo

# tablas de registros con usuario conocido (CPU, [lo, hi)) -> "struct"
STRUCTS = (
    # RocketVehicle_Fire_06d484: 4 tablas de 4 punteros (+$48/+$60/+$4C)
    (0x2CCFEA, 0x2CD02A),
    # RocketVehicle_FireRocket_06d4c2: $2CD03A = 4 punteros a 16 x {dx, dy} de
    # boca de cañón; RocketVehicle_Rocket_06cf06: $2CD10A/$2CD11A = [4][8]
    # punteros a 16 x {vx, vy, ay} (6 B) por dirección
    (0x2CD03A, 0x2CD4EA),
    # AimTurret_AnimScript_0614e6: 7 listas de 16 x {.l frame, .b dur, pad}
    # (+ -1) por nivel de daño y la tabla de 7 punteros
    (0x2C2E76, 0x2C322E),
    # para_squad_module_081816: $2E54A2 7 punteros a 10 x {dx, dy} de salida,
    # $2E55D6 7 x 4 offsets X, $2E560E 7 punteros a 16 x {vx, vy, ay} de salto,
    # $2E571A 7 punteros a 6 x {dx, dy, dz} de caída (TaskHandler_08267c)
    (0x2E54A2, 0x2E5832),
    # gemela sin referencia directa de $2D4A4E (jefe de la misión 5): 4 punteros a
    # 4 tablas Tbl_Decode2D largas contiguas ($2B8FF6, 4 x 130 B)
    (0x2D4274, 0x2D4284), (0x2B8FF6, 0x2B91FE),
    # escuadrón (death_handlers_080736 / deploy_module_07fbd2): $2E3D80 8 x
    # {dx, dy, prio, pad}, $2E3DC0 7 punteros a 8 handlers de muerte, $2E3EBC
    # 4 x {.l offsets {dx, dy} x 6, .l alturas 6 B} de despliegue
    (0x2E3D80, 0x2E3F54),
    # Bazooka (bazooka_rocketvehicle_walker_06a000): $2CA44C 4 punteros a
    # secuencias de dirección (5 B), $2CA470 16 x {dx, dy} de boca de cañón,
    # $2CA4B0 8 punteros a 16 x {vx, vy, ay} del cohete (3 tablas de 96 B)
    (0x2CA44C, 0x2CA5F0),
    # jefe de la misión 5 / walker (gunship_m5boss_finalboss_06dfe8): $2D4186 5
    # punteros a 4 x {vx, vy, ay} por distancia, $2D4212 4 x {vx, vy, ay}
    (0x2D4186, 0x2D422A),
    # escuadrón (spawn_states_041fb4 / children_handlers_041408): $2861D4 6 x
    # {dx, dy} de hijos, $2861EC 12 B de alturas, $2861F8 4 punteros a 11 x
    # {vx, vy, ay}, $286310 3 x {dx, dy}, $28631C patrón de fases
    (0x2861D4, 0x28632C),
    # resultados (results_squadleader_03daa8): $282A9C 16 offsets de columna,
    # $282ABC 72 abreviaturas de rango de 4 B (PVT..GA), $282BBC 72 tiles de
    # insignia por rango
    (0x282A9C, 0x282C4C),
    # jefe 2 (boss2_crab_carrier_07a00a): $2DF888 7 x {dx, dy} de piezas,
    # $2DF8A4 {dx.b, dy.b} por fase ($72 >> 8) hasta el $FFFF
    (0x2DF888, 0x2DFA06),
    # jefe de la misión 4 (m4_carrier_boss_helpers_085134 / miniboss_finale /
    # rescue_squad): $2EAF1C/$2EAF7E listas de guardias {x, y, flags} de 8 B
    # hasta $FFFF, $2EAFE2 lista de pasos (8 B), $2EB02C/$2EB04C 2 x 8 handlers,
    # $2EB06C registro de 14 B
    (0x2EAF1C, 0x2EB07A),
    # jefe de la misión 5 (gunship_m5boss_finalboss_06dfe8): $2D547E/$2D549E
    # destinos {x, y} por lado, $2D54BE.. rutas {x, y}, $2D552E SFX por fase,
    # $2D553E/$2D554E offsets {dx, dy} por dirección (16)
    (0x2D547E, 0x2D55CE),
    # torreta del jefe 2 (turret_boss2_04580c): 5 patrones de 16 longs (ángulo
    # de giro o $FFFFFFFF = pausa) instalados en +$74 por dirección
    (0x289504, 0x289644),
    # mortero (sniper_camper_mortar_062008): $2C56A8 4 punteros a listas de
    # {frame.l, dx, dy} hasta -1, $2C5718 4 x {vx, vy, ay}, $2C5730/$2C576C/
    # $2C57A8/$2C57E4 rejillas de Sprite_MultiBlitClippedX ({dx, dy, w, h} + w*h B)
    (0x2C56A8, 0x2C57E4),
    # popups de puntuación y munición (items_score_crates_09a0bc): $2F792C/
    # $2F794C valores BCD, $2F798C/$2F7994 anchos en dígitos, $2F79A4 4 punteros
    # a cadenas de dígitos, $2F79CE 11 punteros a secuencias de munición ($FF)
    (0x2F792C, 0x2F7A28),
    # cangrejo del jefe 2 (boss2_crab_carrier_07a00a): $2E035A 7 punteros a
    # 16 fases (bytes) de la pinza, $2E03E6 9 alturas, $2E03F8 3 punteros a
    # 9 velocidades, $2E044E 5 handlers de estado
    (0x2E035A, 0x2E0462),
    # coche torreta (turret_car_props_04cbd4): $294144 32 x {dx, dy} por
    # dirección, $2941C4 32 offsets de altura
    (0x294144, 0x294204),
    # game over / continue (gameover_continue_08f6d2): $2F4A32.. listas de
    # paletas y offsets por letra, $2F4A7C punteros, tablas de pasos {dx, dy}
    # ($2F4B24/$2F4B48/$2F4B68/$2F4BA8/$2F4BE8/$2F4BF8/$2F4C48)
    (0x2F4A32, 0x2F4C98),
    # continue: $2F4CB2 stream fix, $2F4CB8 5 punteros a filas de tiles ($FFFF)
    (0x2F4CB2, 0x2F4D9E),
    # hiscore (hiscore_memcard_options_09773c): $2F53B2 10 x 12 B de récords por
    # defecto (copiados a $100002), $2F542A tiles de rango, $2F5440 11 punteros
    # a sufijos ST/ND/RD/TH, $2F547C.. cabeceras ($FFFF), $2F54B2 tabla de
    # caracteres de entrada de nombre (hasta $0CE6)
    (0x2F53B2, 0x2F54A0), (0x2F54B2, 0x2F5504),
    # jefe final (gunship_m5boss_finalboss_06dfe8): listas de pasos {dx, dy} de
    # +$94/+$80 ($2D9130, $2D9138, $2D9168, $2D9198, $2D91C8)
    (0x2D9130, 0x2D91F8),
    # Patrol_Register0..4 (sniper_camper_mortar_062008): listas de 8 B
    # {.l handler, dx, dy} hasta $FFFFFFFF
    (0x2C6460, 0x2C6510),
    # Tank (barrel_paratrooper_shield_tank_066000): $2C8BFE/$2C8C16 2 x 4 x
    # {vx, vy, ay} de restos (Tank_Debris_069af8, +$5C), $2C8C2E 16 B de
    # frame por ángulo y $2C8C3E.. tablas de palabras por ángulo
    (0x2C8BFE, 0x2C8C84),
    # proyectil del jefe final (FinalBoss_CannonShell_071c84): 2 x 16 x
    # {vx, vy, ay} indexadas por +$98 & 15
    (0x2BAE38, 0x2BAEF8),
    # restos del barco (Prop_BoatDebris_04f992, fortress_04e580): bloque de
    # 16 B tras el script en bucle `01 00 .l $2949DE` (sin decodificar)
    (0x294B34, 0x294B44),
    # segundo $FFFF que cierra la última lista antes del relleno final
    (0x2F88C2, 0x2F88C4),
    # marca $12345678 en $2FF800 (sin referencias del código; firma/sentinela
    # del relleno final de la ROM)
    (0x2FF800, 0x2FF804),
)


def modules():
    m = {}
    for name, a, n, f in registry.REGISTRY:
        if f.startswith("data/"):
            lo, hi = m.get(f, (1 << 30, 0))
            m[f] = (min(lo, a), max(hi, a + n))
    return sorted(m.items(), key=lambda t: t[1][0])


def code_holes():
    """Offsets de fichero de entradas de código incrustadas dentro del rango de
    un módulo de datos (p. ej. SlugRiderMarker_Task_18dd3c): no cuentan."""
    mods = [r for f, r in modules()]
    h = set()
    for name, a, n, f in registry.REGISTRY:
        if not f.startswith("data/") and any(lo <= a < hi for lo, hi in mods):
            h.update(range(a, a + n))
    return h


def _cells_ok(p, a, h):
    """Una columna de h celdas {tile.w, attr.w} plausible: byte bajo del attr
    sólo con bits de tile_msb (b4) / flip (b0..b1) en >= 95 % de las celdas."""
    return sum(1 for r in range(h) if p.B(a + 4 * r + 3) & 0xEC == 0) >= 0.95 * h


def blit_patches(p, code_end=0x9C608):
    """-> [(rec, desc, w, h, tiles, ncol)] de los registros de 20 B que
    Sprite_InvokeBlit8Params pasa a CellCommit_MMIO_051DE2 (a0 = ent, a1 =
    desc, d0..d5 = col_dest, fila_dest, col_origen, fila_origen, ancho, alto).
    ncol = máxima columna de origen copiada (col_origen + ancho), ampliada
    mientras las columnas siguientes del descriptor sigan siendo celdas
    válidas (el w de algunos descriptores excede los datos reales)."""
    recs = set()
    for o in range(0, code_end, 2):
        if p.W(o) != 0x45F9:
            continue
        x = p.W(o + 6)
        if (x == 0x4EB9 and p.L(o + 8) == 0x5022A or
                x in (0x4EBA, 0x6100) and o + 8 + p.SW(o + 8) == 0x5022A):
            recs.add(p.L(o + 2))
        # Prop_BlitSequence_04f1f4: `lea X,a2; move.l a2,$70/$74(a0)` (registros
        # de parche que la tarea hija aplica en secuencia)
        elif x == 0x214A and p.W(o + 8) in (0x70, 0x74):
            recs.add(p.L(o + 2))
    recs = {r for r in recs if r & 1 == 0 and p.valid(r, 20) and p.valid(p.L(r + 4), 8)}
    by_desc = {}
    for r in recs:
        by_desc.setdefault(p.L(r + 4), []).append(r)
    out = []
    for desc, rs in sorted(by_desc.items()):
        w, h, tiles = p.W(desc), p.W(desc + 2), p.L(desc + 4)
        ncol = min(w, max(p.W(r + 12) + p.W(r + 16) for r in rs))
        while ncol < w and _cells_ok(p, tiles + 4 * h * ncol, h):
            ncol += 1                       # columnas no copiadas pero válidas
        for r in rs:
            out.append((r, desc, w, h, tiles, ncol))
    return out


def blit_records(p, descs):
    """Registros de 20 B de Sprite_InvokeBlit8Params sin `lea ...,a2 ; jsr`
    directo (llamadas pc-rel, tablas): {l entidad ($10xxxx), l descriptor
    conocido, w col_dest, w fila_dest, w col_origen, w fila_origen, w ancho,
    w alto} con el rectángulo de origen dentro del descriptor -> [(addr, 20)]."""
    descs = set(descs)
    out = []
    for x in range(0x200000, 0x300000, 2):
        if not p.valid(x, 20) or p.L(x + 4) not in descs or not 0x100000 <= p.L(x) < 0x110000:
            continue
        ds = p.L(x + 4)
        w, h = p.W(ds), p.W(ds + 2)
        c1, r1, ww, hh = p.W(x + 12), p.W(x + 14), p.W(x + 16), p.W(x + 18)
        if 1 <= ww and 1 <= hh and c1 + ww <= w + 1 and r1 + hh <= h + 1:
            out.append((x, 20))
    return out



# ---- PathScript (VM de movimiento, PathScript_Step_078f90, tabla $078FC0) ----
# opcode = 1 byte, tamaños fijos; 04 salta (.l), 0A fin del tramo (espera), 08
# llama a código (.l); 05/06/07 escriben un campo de la tarea (< $A0).
PATH_OPSZ = {0: 2, 1: 4, 2: 6, 3: 4, 5: 8, 6: 6, 7: 4, 8: 6, 9: 4}


def path_walk(p, c, code_end=0xF0000):
    """Recorre un PathScript (+saltos 04) -> [(addr, n)] o None si no encaja."""
    spans, seen, todo = [], set(), [c]
    while todo:
        a = todo.pop()
        while a not in seen:
            if a & 1 or not p.valid(a, 2) or a < code_end:
                return None
            op = p.B(a)
            if op > 10:
                return None
            seen.add(a)
            if op == 4:
                spans.append((a, 6))
                todo.append(p.L(a + 2))
                break
            if op == 10:
                spans.append((a, 2))
                break
            n = PATH_OPSZ[op]
            if not p.valid(a, n):
                return None
            if op == 8 and (not 0x400 <= p.L(a + 2) < code_end or p.L(a + 2) & 1):
                return None
            if op in (5, 7) and p.W(a + 2) >= 0xA0 or op == 6 and p.W(a + 4) >= 0xA0:
                return None
            spans.append((a, n))
            a += n
            if len(spans) > 600:
                return None
    return spans


def path_scripts(p, code_end=0xF0000):
    """PathScripts instalados por el código: `move.l #X,$90(a6)` y tablas
    `lea T,a0 ... move.l (a0,dN.w),$90(a6)` (se leen mientras cada entrada
    sea un script válido) -> [(addr, n)] de scripts + [(tabla, n)]."""
    ent, tabs, out = set(), [], []
    for o in range(0, code_end - 8, 2):
        w = p.W(o)
        if w == 0x2D7C and p.W(o + 6) == 0x0090:
            ent.add(p.L(o + 2))
        elif w == 0x2D70 and p.W(o + 4) == 0x0090:
            for b in range(o - 2, o - 40, -2):
                if p.W(b) == 0x41F9:
                    tabs.append(p.L(b + 2))
                    break
    for t in sorted(set(tabs)):
        k = 0
        while p.valid(t + 4 * k, 4) and path_walk(p, p.L(t + 4 * k)):
            ent.add(p.L(t + 4 * k))
            k += 1
        if k:
            out.append((t, 4 * k, "struct"))
    for e in sorted(ent):
        r = path_walk(p, e)
        if r:
            out += [(a, n, "struct") for a, n in r]
    return out


# ---- textos con centinela ----------------------------------------------------
LIST_APPLY_FF = (0x0477FC, 0x04784C, 0x047888)   # List_ApplyWithSentinelFF_*: a2


def ff_strings(p, code_end=0xF0000):
    """Cadenas de glifos (1 B, fin $FF) pasadas en a2 a List_ApplyWithSentinelFF_*
    (`lea X,a2` + jsr/bsr <= 16 B, sin otro `lea ..,a2`) -> [(addr, n)]."""
    out = set()
    for o in range(0, code_end - 8, 2):
        w = p.W(o)
        if w == 0x45F9:
            v, n = p.L(o + 2), 6
        elif w == 0x45FA:
            v, n = o + 2 + p.SW(o + 2), 4
        else:
            continue
        for q in range(o + n, o + n + 16, 2):
            x = p.W(q)
            if x & 0xFFC0 == 0x45C0:
                break
            t = (p.L(q + 2) if x in (0x4EB9, 0x4EF9) else
                 q + 2 + p.SW(q + 2) if x in (0x4EBA, 0x4EFA, 0x6100) else None)
            if t is None:
                continue
            if t in LIST_APPLY_FF and p.valid(v, 1):
                k = 0
                while p.valid(v + k, 1) and p.B(v + k) != 0xFF and k < 256:
                    k += 1
                if k < 256:
                    out.add((v, k + 1))
            break
    return sorted(out)


FIX_TEXTROW_DRAW = 0x08C9A6   # Fix_TextRow_Draw_08c9a6: +$80(a6) -> filas fix


def fix_row_texts(p, code_end=0xF0000):
    """Textos de Fix_TextRow_Draw_08c9a6 (Cut_TextPanel): palabras fix 2x2
    (`$0Bxx`/`$0Cxx` paleta 4, resto 9), 0000 = fin de fila, FFFE = fila alta,
    FFFD = salta, FFFF = fin; instalados con `lea X,a1 ; move.l a1,$80(a6)`."""
    out = []
    for o in range(0, code_end - 10, 2):
        if p.W(o) == 0x43F9 and p.W(o + 6) == 0x2D49 and p.W(o + 8) == 0x0080:
            v, k = p.L(o + 2), 0
            while (p.valid(v + k, 2) and p.W(v + k) != 0xFFFF and k < 0x2000
                   and (p.W(v + k) < 0x1000 or p.W(v + k) >= 0xFFFD)):
                k += 2
            if k >= 8 and p.valid(v + k, 2) and p.W(v + k) == 0xFFFF:
                out.append((v, k + 2))
    return out

SET_SPRITE_MAP = 0x028CD4          # EntitySetSpriteMap: a0 -> +$3C/+$40 (script)
_OPSZ = {**S.OP_SIZE, 0x00: 2, 0x01: 6, 0x03: 6, 0x16: 2, 0x17: 2, 0x18: 2}


def code_script_seeds(p, code_end=0x9C608):
    """Scripts de sprite que el código instala directamente:
    `lea X,a0` (abs.l o d16(pc)) seguido (<= 12 B) de jsr/bsr EntitySetSpriteMap,
    o `move.l #X,$3C(a6)` (puntero de script de la tarea)."""
    seeds = set()
    for o in range(0, code_end - 8, 2):
        w = p.W(o)
        if w == 0x41F9:
            v, n = p.L(o + 2), 6
        elif w == 0x41FA:
            v, n = o + 2 + p.SW(o + 2), 4
        elif w == 0x2D7C and p.W(o + 6) == 0x003C:
            seeds.add(p.L(o + 2))
            continue
        else:
            continue
        for q in range(o + n, o + n + 12, 2):
            x = p.W(q)
            t = (p.L(q + 2) if x in (0x4EB9, 0x4EF9) else
                 q + 2 + p.SW(q + 2) if x in (0x4EBA, 0x4EFA, 0x6100) else None)
            if t is not None:
                if t == SET_SPRITE_MAP:
                    seeds.add(v)
                break
    return {v for v in seeds if v & 1 == 0 and p.valid(v, 2)}


def walk_struct(p, a, seen, depth=0):
    """Recorrido estructural del bytecode de sprite (Script_OpcodeTable, entity/
    collmap_hitbox_script_ops_027400.s) -> [(addr, len)] o None si es inválido.
    A diferencia de sprites.walk: acepta `02` con frame $FFFFFFFF (sprite
    oculto: ScriptOp02 hace `movea.l #-1,a0`) y recorre las tablas de 1B/1C
    (entradas de $12/$1C B tras el opcode, indexadas por una palabra de la
    tarea) hasta la primera entrada inválida."""
    out = []
    for _ in range(4000):
        if a in seen or a & 1 or not p.valid(a, 2):
            break
        op = p.B(a)
        if op in (0x02, 0x1E):                # 1E = SpawnChild_Sprite: mismo formato que 02
            fl, fr = p.B(a + 1), p.L(a + 2)
            if fl & 0xE0 or not (fr == 0xFFFFFFFF or
                                 (fr & 1 == 0 and p.valid(fr, 2) and S.parse_frame(p, fr))):
                return None
            n = 8 if fl & 8 else 12
            seen.add(a)
            out.append((a, n))
            a += n
            continue
        if op in (0x1B, 0x1C):
            st = 0x12 if op == 0x1B else 0x1C
            seen.add(a)
            out.append((a, 2))
            for k in range(256):
                e = a + 2 + st * k
                if e in seen:
                    continue
                r = walk_struct(p, e, seen, depth + 1) if depth < 4 else None
                if not r:
                    break
                out += r
            break
        if op in (0x1D, 0x16):                # 16 = RepeatSprite (a1 = $40(a6))
            seen.add(a)
            out.append((a, 2))
            break
        if op not in _OPSZ or not p.valid(a, _OPSZ[op]):
            return out or None
        n = _OPSZ[op]
        seen.add(a)
        out.append((a, n))
        if op == 0x01:
            t = p.L(a + 2)
            if t & 1 or not p.valid(t, 2):
                return None
            a = t
            continue
        if op == 0x03:
            t = p.L(a + 2)
            if t & 1 == 0 and p.valid(t, 2) and t not in seen:
                out += walk_struct(p, t, seen, depth + 1) or []
        a += n
    return out or None


def code_script_tables(p, code_end=0x9C608):
    """Tablas de punteros a scripts que el código indexa antes de instalarlos:
    `lea T,a0` / `movea.l #T,a0`, luego `movea.l (a0,dN.w),a0` (<= 12 B) y un
    jsr/bsr EntitySetSpriteMap (<= 24 B). La tabla sigue mientras cada long sea
    $FFFFFFFF (sin sprite) o un script válido (walk_struct).
    -> ([(addr, len)] de tablas, [scripts])"""
    tabs, scripts = [], set()
    for o in range(0, code_end - 8, 2):
        w = p.W(o)
        if w in (0x41F9, 0x207C):
            v, n = p.L(o + 2), 6
        elif w == 0x41FA:
            v, n = o + 2 + p.SW(o + 2), 4
        else:
            continue
        if v & 1 or not p.valid(v, 4):
            continue
        idx = any(p.W(q) == 0x2070 for q in range(o + n, o + n + 12, 2))
        if not idx:
            continue
        hit = False
        for q in range(o + n, o + n + 24, 2):
            x = p.W(q)
            t = (p.L(q + 2) if x in (0x4EB9, 0x4EF9) else
                 q + 2 + p.SW(q + 2) if x in (0x4EBA, 0x4EFA, 0x6100) else None)
            if t == SET_SPRITE_MAP:
                hit = True
                break
        if not hit:
            continue
        k, ok = 0, []
        while k < 256 and p.valid(v + 4 * k, 4):
            e = p.L(v + 4 * k)
            if e == 0xFFFFFFFF:
                k += 1
                continue
            if e & 1 or not p.valid(e, 2) or not walk_struct(p, e, set()):
                break
            ok.append(e)
            k += 1
        if ok:
            tabs.append((v, 4 * k))
    # una tabla acaba donde empieza la siguiente conocida
    starts = sorted({v for v, n in tabs})
    out = []
    for i, v in enumerate(starts):
        n = max(n for t, n in tabs if t == v)
        if i + 1 < len(starts):
            n = min(n, starts[i + 1] - v)
        out.append((v, n))
        for k in range(n // 4):
            e = p.L(v + 4 * k)
            if e != 0xFFFFFFFF:
                scripts.add(e)
    return out, scripts


def pointed_scripts(p, mark):
    """Scripts sin clasificar a los que apunta algún long de la ROM y que
    walk_struct recorre con >= 2 frames (op 02) acabando en un op de salto o
    fin (01/16/1B/1C/1D/1E/00) -> [(addr, len)]."""
    d, cand = p.d, set()
    for o in range(0, len(d) - 3, 2):
        v = (d[o] << 24) | (d[o + 1] << 16) | (d[o + 2] << 8) | d[o + 3]
        if v & 1 == 0 and (0x90000 <= v < 0x100000 or 0x200000 <= v < 0x300000) \
                and mark.get(v) in (None, "anim") and p.valid(v, 2):
            cand.add(v)
    out = []
    for v in sorted(cand):
        r = walk_struct(p, v, set())
        if r and sum(1 for a, n in r if p.B(a) in (0x02, 0x1E)) >= 2 and \
                p.B(max(r)[0]) in (0x00, 0x01, 0x16, 0x1B, 0x1C, 0x1D) and \
                any(a not in mark for a, n in r):
            out += r
    return out


def gap_scripts(p, mark):
    """Scripts sin puntero pegados a otros datos: en cada racha sin clasificar,
    se intenta walk_struct desde su primera palabra (y tras cada script
    aceptado); se acepta si queda dentro de la racha, tiene >= 2 frames
    (02/1E) y acaba en un op de salto o fin (01/16/1B/1C/1D) -> [(addr, len)]."""
    out = []
    for lo, hi in _gaps(p, mark):
        a = p.cpu((lo + 1) & ~1)
        end = a + (hi - ((lo + 1) & ~1))
        while a < end - 8:
            if not p.valid(a, 2):
                break
            if p.B(a) == 0 and p.B(a + 1) == 0:
                a += 2
                continue
            r = walk_struct(p, a, set())
            nfr = sum(1 for x, n in r if p.B(x) in (0x02, 0x1E)) if r else 0
            if not (r and (nfr >= 2 or nfr == 1 and p.B(max(r)[0]) in (0x16, 0x1D))
                    and p.B(max(r)[0]) in (0x01, 0x16, 0x1B, 0x1C, 0x1D)
                    and all(a <= x and x + n <= end or mark.get(x) == "anim" for x, n in r)):
                break
            r = [(x, n) for x, n in r if a <= x and x + n <= end]
            out += r
            a = max(x + n for x, n in r)
    return out


HITBOX_REC, HITSHAPE_LEN = 0x50, 36


def _field_ptr_tables(p, field, code_end=0x9C608):
    """Tablas de punteros copiadas a un campo de la tarea:
    `lea T,aN` ... `move.l (aN,dM.w),field(a6)` (<= 48 B) -> entradas de T."""
    out = set()
    for o in range(0, code_end - 8, 2):
        w = p.W(o)
        if w & 0xF1FF != 0x41F9:
            continue
        reg, t = (w >> 9) & 7, p.L(o + 2)
        for q in range(o + 6, o + 54, 2):
            if p.W(q) == 0x2D70 | reg and p.B(q + 2) & 0x08 == 0 and p.W(q + 4) == field:
                k = 0
                while k < 32 and p.valid(t + 4 * k, 4):
                    v = p.L(t + 4 * k)
                    if v != 0xFFFFFFFF and (v & 1 or not 0x200000 <= v < 0x300000):
                        break
                    out.add(v)
                    k += 1
                break
    return out


def hitbox_lists(p, op_starts, code_end=0x9C608):
    """Listas de cajas de golpe de +$48(a6) (Hitbox_TestBoxes_028594): registros
    de $50 B {.w, .l tabla de formas (36 B, HitShapeTab_17928c) o -1, cajas}
    hasta un long $FFFFFFFF. Semillas: operando del op 09 (SetHitbox48) de los
    scripts recorridos y `move.l #X,$48(aN)` del código."""
    seeds = {p.L(a + 2) for a in op_starts if p.B(a) == 0x09 and p.valid(a + 2, 4)}
    seeds |= _field_ptr_tables(p, 0x48, code_end)
    for o in range(0, code_end - 8, 2):
        if p.W(o) & 0xF1FF == 0x217C and p.W(o + 6) == 0x48:
            seeds.add(p.L(o + 2))
        elif p.W(o) == 0x41F9 and p.W(o + 6) == 0x2D48 and p.W(o + 8) == 0x48:
            seeds.add(p.L(o + 2))
        # tabla de punteros: `lea T,aN; movea.l (aN,dM.w),a0; move.l a0,$48(a6)`
        elif p.W(o) & 0xF1FF == 0x41F9 and p.W(o + 6) & 0xFFF8 == 0x2070 and \
                p.W(o + 10) == 0x2D48 and p.W(o + 12) == 0x48:
            t, k = p.L(o + 2), 0
            while k < 32 and p.valid(t + 4 * k, 4) and p.L(t + 4 * k) & 1 == 0 and \
                    0x200000 <= p.L(t + 4 * k) < 0x300000:
                seeds.add(p.L(t + 4 * k))
                k += 1
    out = []
    for t in sorted(seeds):
        if t & 1 or not p.valid(t, 4) or t < 0x200000 and t < code_end:
            continue
        x, k, recs = t, 0, []
        while p.valid(x, 4) and p.L(x) != 0xFFFFFFFF and k < 32:
            sh = p.L(x + 2)
            if not p.valid(x, HITBOX_REC) or not (sh == 0xFFFFFFFF or (sh & 1 == 0 and p.valid(sh, HITSHAPE_LEN))):
                recs = None
                break
            recs.append(sh)
            x += HITBOX_REC
            k += 1
        if recs is None or not p.valid(x, 4):
            continue
        out.append((t, x + 4 - t))
        out += [(sh, HITSHAPE_LEN) for sh in recs if sh != 0xFFFFFFFF]
    # tablas de formas contiguas (36 B de bytes 0/1) tras una conocida
    shapes = sorted({a for a, n in out if n == HITSHAPE_LEN})
    for sh in shapes:
        x = sh + HITSHAPE_LEN
        while p.valid(x, HITSHAPE_LEN) and all(p.B(x + i) in (0, 1) for i in range(HITSHAPE_LEN)) \
                and any(p.B(x + i) for i in range(HITSHAPE_LEN)):
            out.append((x, HITSHAPE_LEN))
            x += HITSHAPE_LEN
    return out


def attack_lists(p, op_starts, code_end=0x9C608):
    """Listas de cajas de ataque de +$4C(a6) (Hitbox_RunList_0283ec): registros
    de $50 B {op.b 0..4 (Hitbox_ListOpTable_028428: a quién golpea), ...} hasta
    un byte $FF. Semillas: operando del op 0A (SetField4C) y `lea X,a0` +
    `move.l a0,$4C(a6)` / `move.l #X,$4C(aN)` del código."""
    seeds = {p.L(a + 2) for a in op_starts if p.B(a) == 0x0A and p.valid(a + 2, 4)}
    seeds |= _field_ptr_tables(p, 0x4C, code_end)
    for o in range(0, code_end - 10, 2):
        w = p.W(o)
        if w == 0x41F9 and p.W(o + 6) == 0x2D48 and p.W(o + 8) == 0x4C:
            seeds.add(p.L(o + 2))
        elif w & 0xF1FF == 0x217C and p.W(o + 6) == 0x4C:
            seeds.add(p.L(o + 2))
    # boss2_crab_carrier_07a00a: `lea T,a1; movea.l (a1,d),a1; movea.l (a1,d1),a0;
    # move.l a0,$4C(a6)` -> T = punteros a listas de punteros a listas de ataque
    # (cada sublista termina en $FFFFFFFF)
    for o in range(0, code_end - 40, 2):
        if p.W(o) == 0x43F9 and any(p.W(q) == 0x2D48 and p.W(q + 2) == 0x4C
                                    for q in range(o + 6, o + 40, 2)) and \
                any(p.W(q) == 0x2271 for q in range(o + 6, o + 20, 2)):
            t = p.L(o + 2)
            k = 0
            while k < 32 and p.valid(t + 4 * k, 4) and p.valid(p.L(t + 4 * k), 4):
                sub = p.L(t + 4 * k)
                j = 0
                while j < 32 and p.valid(sub + 4 * j, 4) and p.L(sub + 4 * j) != 0xFFFFFFFF:
                    seeds.add(p.L(sub + 4 * j))
                    j += 1
                k += 1
    out = []
    for t in sorted(seeds):
        if t & 1 or not p.valid(t, 1) or t < 0x100000:
            continue
        x, k = t, 0
        while p.valid(x, 1) and p.B(x) < 5 and k < 32 and p.valid(x, 0x50):
            x += 0x50
            k += 1
        if p.valid(x, 1) and p.B(x) == 0xFF:
            out.append((t, x + 2 - t))
    return out


INSTALL_FIX_LIST = 0x05DB58   # InstallListPubHead_05DB58: a0 -> +$3C (lista fix)


def fix_anim_lists(p, code_end=0x9C608):
    """Listas de ListCursor_Step_05db6a (animación de bloques del fix):
    entradas de 8 B {tile, cols, rows, duración}; tile 0 = salto (.l en +2),
    $FFFF = fin. Semillas: `lea X,a0` + jsr/jmp InstallListPubHead_05DB58
    (<= 12 B) -> [(addr, n)]."""
    seeds, out, seen = set(), [], set()
    for o in range(0, code_end - 8, 2):
        if p.W(o) != 0x41F9:
            continue
        v = p.L(o + 2)
        for q in range(o + 6, o + 18, 2):
            x = p.W(q)
            if x in (0x4EB9, 0x4EF9) and p.L(q + 2) == INSTALL_FIX_LIST:
                seeds.add(v)
                break
    for v in sorted(seeds):
        todo = [v]
        while todo:
            a = todo.pop()
            for _ in range(512):
                if a in seen or a & 1 or not p.valid(a, 2):
                    break
                seen.add(a)
                w = p.W(a)
                if w == 0xFFFF:
                    out.append((a, 2))
                    break
                if not p.valid(a, 8):
                    break
                out.append((a, 8))
                if w == 0:
                    todo.append(p.L(a + 2))
                    break
                a += 8
    return out


def motion_ptr_tables(p, mark, code_end=0x9C608):
    """Tablas de punteros a registros de movimiento: `lea T,aN` y, en <= 40 B,
    `movea.l (aN,dM.w),aK` seguido de `move.w (aK,..),$28/$2E/$2A(a6)`
    (velocidades {vx, vy[, ay]}). Las entradas son punteros a la ROM de datos
    contiguos (cada uno entre T y el siguiente bloque conocido); se marcan la
    tabla y el tramo [min(entradas), fin) hasta el primer byte ya marcado
    (<= 1 KB) -> [(addr, n)]."""
    out = []
    for o in range(0, code_end - 8, 2):
        w = p.W(o)
        if w & 0xF1FF != 0x41F9:
            continue
        t, reg = p.L(o + 2), (w >> 9) & 7
        if t & 1 or not 0x200000 <= t < 0x300000 or t in mark and mark[t] != "struct":
            continue
        ok_ind = ok_mv = False
        for q in range(o + 6, o + 46, 2):
            x = p.W(q)
            if x & 0xF1FF == 0x2070 | reg and p.B(q + 2) & 0x8F == 0:   # movea.l (aN,dM.w),aK
                ok_ind = True
                k = (x >> 9) & 7
                for r in range(q + 4, q + 30, 2):
                    y = p.W(r)
                    # move.w (aK,..)/(aK)/d(aK),$28/$2E/$2A(a6): 0x3D70|k, 0x3D50|k, 0x3D68|k
                    if y in (0x3D70 | k, 0x3D50 | k, 0x3D68 | k):
                        dst = p.W(r + (4 if y & 0x38 in (0x30, 0x28) else 2))
                        if dst in (0x28, 0x2E, 0x2A, 0x36):
                            ok_mv = True
                            break
                break
        if not (ok_ind and ok_mv):
            continue
        ents = []
        while len(ents) < 64 and p.valid(t + 4 * len(ents), 4):
            e = p.L(t + 4 * len(ents))
            if e & 1 or not 0x200000 <= e < 0x300000 or abs(e - t) > 0x800:
                break
            ents.append(e)
        if len(ents) < 2:
            continue
        lo = min(ents + [t])
        hi = max(ents)
        while (hi < lo + 0x1000 and p.valid(hi, 2) and hi not in mark
               and (p.W(hi) < 0x1000 or p.W(hi) > 0xF000)):
            hi += 2
        out.append((t, 4 * len(ents)))
        out.append((min(ents), hi - min(ents)))
    return out


ONSCREEN_BOX = (0x05DD56, 0x05DD5C)  # a0 = caja {dx0, dx1, dy0, dy1} (8 B)
PROBE_GRID = 0x043FAC                # a1 = {dx, dy, w, h} + w*h bytes
FIX_STREAM = 0x05DAD8                # a2 = bytes de tile del fix ($FD = línea, $FE = fin)
LIST_FF = (0x0477FC, 0x04784C, 0x047888)   # a2 = lista de bytes hasta $FF (glifos)


def code_arg_tables(p, code_end=0x9C608):
    """-> [(addr, len)] de tablas pasadas en a0/a1 a rutinas de formato fijo:
    `lea X,aN` seguido (<= 12 B) de jsr/bsr a ONSCREEN_BOX (caja de 8 B,
    a0) o a PROBE_GRID (rejilla de bytes, a1), y textos en a2 para
    Fix_BlitStream_05DAD8 (hasta $FE) o List_ApplyWithSentinelFF_* (hasta $FF).
    Los `lea` en a2 admiten también direcciones impares (texto)."""
    out = []
    for o in range(0, code_end - 8, 2):
        w = p.W(o)
        if w in (0x41F9, 0x43F9, 0x45F9):
            v, n, reg = p.L(o + 2), 6, (w >> 9) & 7
        elif w in (0x41FA, 0x43FA, 0x45FA):
            v, n, reg = o + 2 + p.SW(o + 2), 4, (w >> 9) & 7
        else:
            continue
        # tabla de punteros a rejillas: `lea T,a0; ...; movea.l (a0,dN.w),a1;
        # jsr PROBE_GRID` (gun_platform_04bb9a: rejilla por dirección)
        if reg == 0 and v & 1 == 0:
            for q in range(o + n, o + n + 16, 2):
                if p.W(q) == 0x2270 and p.B(q + 2) & 0x8F == 0 and \
                        p.W(q + 4) == 0x4EB9 and p.L(q + 6) == PROBE_GRID:
                    k = 0
                    while k < 16 and p.valid(v + 4 * k, 4):
                        g = p.L(v + 4 * k)
                        if g & 1 or not p.valid(g, 8) or not 0 < p.W(g + 4) * p.W(g + 6) <= 4096:
                            break
                        out.append((g, 8 + p.W(g + 4) * p.W(g + 6)))
                        k += 1
                    if k:
                        out.append((v, 4 * k))
                    break
        # rejillas guardadas en +$78 y dibujadas luego con `movea.l $78(a6),a1;
        # jsr PROBE_GRID` (fort_scene4_0865be)
        if reg == 1 and p.W(o + n) == 0x2D49 and p.W(o + n + 2) == 0x78 and \
                v & 1 == 0 and p.valid(v, 8) and 0 < p.W(v + 4) * p.W(v + 6) <= 4096:
            out.append((v, 8 + p.W(v + 4) * p.W(v + 6)))
            continue
        for q in range(o + n, o + n + 12, 2):
            x = p.W(q)
            t = (p.L(q + 2) if x in (0x4EB9, 0x4EF9) else
                 q + 2 + p.SW(q + 2) if x in (0x4EBA, 0x4EFA, 0x6100) else None)
            if t is None:
                continue
            if v & 1 == 0 and p.valid(v, 8):
                if t in ONSCREEN_BOX and reg == 0:
                    out.append((v, 8))
                elif t == PROBE_GRID and reg == 1:
                    gw, gh = p.W(v + 4), p.W(v + 6)
                    if 0 < gw * gh <= 4096:
                        out.append((v, 8 + gw * gh))
            if reg == 2 and (t == FIX_STREAM or t in LIST_FF) and p.valid(v, 1):
                end = 0xFE if t == FIX_STREAM else 0xFF
                k = 0
                while k < 4096 and p.valid(v + k, 1) and p.B(v + k) != end:
                    k += 1
                if k < 4096:
                    out.append((v, k + 1))
            break
    return out


TBL_DECODE2D = 0x0799DE   # Tbl_Decode2D_0799DE: a0 = tabla por dificultad/rango


def decode2d_tables(p, code_end=0xF0000):
    """-> [(addr, len)] de tablas pasadas en a0 a Tbl_Decode2D_0799DE
    (`lea X,a0` + jsr/bsr <= 12 B, sin otro `lea ..,a0` entre medias).
    Formato largo (palabra 0 != 2): 2 + 64 palabras indexadas por
    (flags11&3)*4 + ($106ED1&2) + (Rank_SubIndex&7)*16 -> 130 B.
    Formato corto (palabra 0 == 2): 2 + 64 bytes de índice + 16 B por índice
    (8 palabras elegidas con Random&$E) -> 2 + 64 + 16*(max+1)."""
    out = set()
    for o in range(0, code_end - 8, 2):
        w = p.W(o)
        if w & 0xF1FF == 0x41F9:          # lea X.l,aN
            v, n, reg = p.L(o + 2), 6, (w >> 9) & 7
        elif w & 0xF1FF == 0x41FA:        # lea X(pc),aN
            v, n, reg = o + 2 + p.SW(o + 2), 4, (w >> 9) & 7
        else:
            continue
        if reg == 7:
            continue
        ind = False
        for q in range(o + n, o + n + 20, 2):
            x = p.W(q)
            if x & 0xFFC0 == 0x41C0 or x == 0x41C0 | (reg << 9):   # otro lea ..,a0/aN
                # variante normal/espejo: `lea X,a0; cmpi/tst; bne/beq L; lea Y,a0;
                # L: jsr Decode2D` -> X también es tabla si lo es Y
                if reg == 0 and x == 0x41F9 and any(
                        p.W(r) & 0xFE00 == 0x6600 for r in range(o + n, q, 2)):
                    for r in range(q + 6, q + 18, 2):
                        if p.W(r) == 0x4EB9 and p.L(r + 2) == TBL_DECODE2D:
                            if _d2d_len(p, v) and _d2d_len(p, p.L(q + 2)):
                                out.add((v, _d2d_len(p, v)))
                            break
                        if p.W(r) in (0x4EBA, 0x6100) and r + 2 + p.SW(r + 2) == TBL_DECODE2D:
                            if _d2d_len(p, v) and _d2d_len(p, p.L(q + 2)):
                                out.add((v, _d2d_len(p, v)))
                            break
                break
            if x == 0x2070 | reg and p.B(q + 2) & 0x8F == 0:   # movea.l (aN,dM.w),a0
                ind = True
                continue
            if reg != 0 and not ind:
                continue
            t = (p.L(q + 2) if x in (0x4EB9, 0x4EF9) else
                 q + 2 + p.SW(q + 2) if x in (0x4EBA, 0x4EFA, 0x6100) else None)
            if t is None:
                continue
            if t == TBL_DECODE2D:
                if ind:
                    k = 0
                    while p.valid(v + 4 * k, 4) and _d2d_len(p, p.L(v + 4 * k)):
                        e = p.L(v + 4 * k)
                        out.add((e, _d2d_len(p, e)))
                        k += 1
                    if k:
                        out.add((v, 4 * k))
                elif _d2d_len(p, v):
                    out.add((v, _d2d_len(p, v)))
            break
    return sorted(out)


def _d2d_len(p, v):
    """Longitud de una tabla Tbl_Decode2D en v (0 si no encaja)."""
    if v & 1 or not 0x100000 <= v < 0x300000 or not p.valid(v, 130):
        return 0
    if p.W(v) == 2:
        mx = max(p.B(v + 2 + i) for i in range(64))
        return 66 + 16 * (mx + 1) if mx < 32 and p.valid(v, 66 + 16 * (mx + 1)) else 0
    return 130


INSTALL_SPAWNER = 0x077C7E   # Entity_InstallHandlerAndCopyXf: a1 -> $70(a0)


def spawner_records(p, code_end=0x9C608):
    """-> [(addr, len)] de registros de Spawner_Handler_077c98 (18 B):
    {b offset_por_padre, b ritmo ($FF = todo de golpe), w cuenta, w dx, w rx,
    w dy, w ry, b ?, b aleatorio, l handler}; si +$D != 0, +$E apunta a una
    tabla de 16 handlers elegidos con Random(16)*4 (64 B). Se siguen los
    `lea X,a1` + jsr/bsr Entity_InstallHandlerAndCopyXf (<= 16 B)."""
    out = set()
    for o in range(0, code_end - 8, 2):
        w = p.W(o)
        if w == 0x43F9:
            v, n = p.L(o + 2), 6
        elif w == 0x43FA:
            v, n = o + 2 + p.SW(o + 2), 4
        else:
            continue
        for q in range(o + n, o + n + 16, 2):
            x = p.W(q)
            if x & 0xFFC0 == 0x43C0:          # otro lea ..,a1
                break
            t = (p.L(q + 2) if x in (0x4EB9, 0x4EF9) else
                 q + 2 + p.SW(q + 2) if x in (0x4EBA, 0x4EFA, 0x6100) else None)
            if t is None:
                continue
            if t == INSTALL_SPAWNER and v & 1 == 0 and p.valid(v, 18):
                out.add((v, 18))
                h = p.L(v + 14)
                if p.B(v + 13) and h & 1 == 0 and p.valid(h, 64) and \
                        all(p.L(h + 4 * i) < code_end for i in range(16)):
                    out.add((h, 64))
            break
    return sorted(out)


# Textos "a máquina" de la presentación de misión (mission/cutscene_anim_08ba0c):
# Icon_Anchor_Run_08c678 elige por idioma ($8A(a6) = 0..3) una de 4 tablas de
# 44 pares {stream.l, atributo_fix.l} indexadas por la página $76 del padre;
# cada stream (Anim_ScriptStepFix_A/B, Anim_ScriptStep_08bc74) son registros
# de 8 B {w espera ($FFFF = fin), w carácter/comando, l script de sprite o 0}.
FIX_TYPE_TABS = (0x2EFACC, 0x2EFC2C, 0x2EFD8C, 0x2EFEEC)
FIX_TYPE_PAGES = 44
FIX_TYPE_STREAMS = (0x2EF8AA, 0x2F0056)


def fix_type_streams(p):
    """-> ([(addr, len)] tablas + streams, [scripts de sprite referenciados])."""
    out, streams, scripts = [], set(FIX_TYPE_STREAMS), set()
    for t in FIX_TYPE_TABS:
        out.append((t, 8 * FIX_TYPE_PAGES))
        for k in range(FIX_TYPE_PAGES):
            streams.add(p.L(t + 8 * k))
    for s in sorted(streams):
        n = 0
        while p.valid(s + 8 * n, 2) and p.W(s + 8 * n) != 0xFFFF and n < 4096:
            f = p.L(s + 8 * n + 4)
            if f and f & 1 == 0 and p.valid(f, 2):
                scripts.add(f)
            n += 1
        out.append((s, 8 * n + 2))
    return out, sorted(scripts)


# SpawnStream_ReadNext_04cac4: por misión (SpriteSetPtrTbl6_04cb44[$106ECE])
# registros de 16 B {b ?, b ?, b rel_x, b rel_y, w x, w y, w d2, w d3, b d4,
# b d5, b d6, b ?} y luego w umbral de cámara del siguiente; fin con $FFFF.
SPAWN_STREAM_TAB, SPAWN_STREAM_N = 0x04CB44, 6


def spawn_streams(p):
    """-> [(addr, len)] de los 6 streams de spawn por misión."""
    ss = sorted({p.L(SPAWN_STREAM_TAB + 4 * i) for i in range(SPAWN_STREAM_N)})
    out = []
    for i, s in enumerate(ss):
        lim = ss[i + 1] if i + 1 < len(ss) else s + 16 * 512
        k = 0
        while s + 16 * k + 18 <= lim and p.W(s + 16 * k + 16) != 0xFFFF:
            k += 1
        out.append((s, 16 * (k + 1) + 2))
    return out


# Diccionarios de nombres de Results_RosterPickA/B_03e6da/03e740 (pantalla de
# resultados): d0 = Random(n) -> longitud = LenTab[d0]; {ptr.l, cnt.l} =
# PtrTab[longitud - base]; nombre = ptr + longitud * Random(cnt).
#   (LenTab, n, PtrTab, base): A = katakana (b>=$80 -> tile $2A00+b-$80),
#   B = ASCII (modo $10FD83).
ROSTER_NAMES = ((0x182DE0, 0x3B2, 0x1831B2, 1), (0x182C3C, 0x14C, 0x182D88, 2))


def roster_names(p):
    """-> [(addr, len)] de las tablas de longitudes, de punteros y los pools."""
    out = []
    for lt, n, pt, base in ROSTER_NAMES:
        lt, pt = p.cpu(lt), p.cpu(pt)
        out.append((lt, n))
        kmax = max(p.B(lt + i) for i in range(n))
        # el PtrTab sigue con longitudes mayores (pools contiguos) aunque LenTab
        # no las use
        while p.L(pt + 8 * (kmax + 1 - base)) == \
                p.L(pt + 8 * (kmax - base)) + kmax * p.L(pt + 8 * (kmax - base) + 4):
            kmax += 1
        out.append((pt, 8 * (kmax - base + 1)))
        for k in range(base, kmax + 1):
            ptr, cnt = p.L(pt + 8 * (k - base)), p.L(pt + 8 * (k - base) + 4)
            if p.valid(ptr, k * cnt):
                out.append((ptr, k * cnt))
    return out


def _gaps(p, mark):
    """Rachas de offsets [lo, hi) de los módulos sin clasificar todavía."""
    out, cur = [], None
    for f, (lo, hi) in modules():
        for o in range(lo, hi):
            if p.cpu(o) not in mark:
                if cur and cur[1] == o:
                    cur[1] = o + 1
                else:
                    cur = [o, o + 1]
                    out.append(cur)
    return out


def _split_zeros(p, gaps, min_zero=4):
    """Corta cada racha [lo, hi) en los tramos de palabras a cero >= min_zero B
    (relleno entre bloques de frames) -> [(lo, hi)] sin esos ceros."""
    out = []
    for lo, hi in gaps:
        start, o = lo, lo + (lo & 1)
        while o < hi - 1:
            if p.W(p.cpu(o)) == 0:
                z = o
                while o < hi - 1 and p.W(p.cpu(o)) == 0:
                    o += 2
                if o - z >= min_zero:
                    if z > start:
                        out.append((start, z))
                    start = o
                continue
            o += 2
        if hi > start:
            out.append((start, hi))
    return out


def frame_runs(p, mark, min_cover=0.95):
    """Frames de sprite sin puntero directo: una racha sin clasificar que se
    recorre de punta a punta como frames consecutivos (parse_frame) en >= 95 %
    (si no, se prueba cortando los rellenos de ceros >= 16 B)
    (se tolera relleno final). Típico de animaciones cuyos frames se indexan
    por tabla calculada. -> [(cpu_addr, len)]"""
    def run(lo, hi):
        if lo & 1 or hi - lo < 16:
            return None
        o, fr = lo, []
        while o < hi - 2:
            r = S.parse_frame(p, p.cpu(o))
            if not r or o + r["size"] > hi:
                break
            fr.append((p.cpu(o), r["size"]))
            o += r["size"]
        return fr if fr and (o - lo) >= min_cover * (hi - lo) else None

    out = []
    for g in _gaps(p, mark):
        fr = run(*g)
        if fr is None:
            fr = []
            for sub in _split_zeros(p, [g], 16):
                fr += run(*sub) or []
        out += fr
    return out


def _cell_good(p, c):
    """Celda {tile.w, attr.w} típica: attr con byte bajo $10..$19 (tile_msb +
    flips) y paleta < $80, o celda vacía."""
    t, at = p.W(c), p.W(c + 2)
    return (t == 0 and at == 0) or (0x10 <= at & 0xFF <= 0x19 and at >> 8 < 0x80)


def desc_chains(p, min_n=8):
    """Tablas de descriptores de 8 B {w, h, tiles.l} (formato de los parches de
    Sprite_InvokeBlit8Params) en cadena, cuyos tilemaps son contiguos
    (tiles[i+1] == tiles[i] + 4*w*h) -> [(addr, len, kind)]."""
    out, a = [], 0
    while a < 0x200000 - 8:
        ds, q = [], a
        while True:
            c = p.cpu(q)
            if not p.valid(c, 8):
                break
            w, h, t = p.W(c), p.W(c + 2), p.L(c + 4)
            if not (1 <= w <= 128 and 1 <= h <= 32 and t & 3 == 0 and p.valid(t, 4 * w * h)):
                break
            if ds and ds[-1][2] + 4 * ds[-1][0] * ds[-1][1] != t:
                break
            ds.append((w, h, t))
            q += 8
        if len(ds) >= min_n and sum(_cell_good(p, t + 4 * i) for w, h, t in ds
                                    for i in range(w * h)) >= 0.95 * sum(w * h for w, h, t in ds):
            out.append((p.cpu(a), 8 * len(ds), "struct"))
            for w, h, t in ds:
                out.append((t, 4 * w * h, "tilemap"))
            a = q
        else:
            a += 2
    return out


def tilemap_gaps(p, mark, min_cells=64):
    """Rachas sin clasificar que son celdas de tilemap en >= 98 % (con >= 50 %
    no vacías): tilemaps de fondo sin descriptor conocido (extensiones de capas
    vecinas). -> [(addr, len)]"""
    out = []
    for lo, hi in _gaps(p, mark):
        o = (lo + 3) & ~3
        n = (hi - o) // 4
        if n < min_cells:
            continue
        cs = [p.cpu(o + 4 * i) for i in range(n)]
        good = sum(_cell_good(p, c) for c in cs)
        nz = sum(1 for c in cs if p.W(c + 2))
        if good >= 0.98 * n and nz >= 0.5 * n:
            out.append((p.cpu(o), 4 * n))
    return out


def pointer_tables(p, mark, min_n=3):
    """Rachas sin clasificar de longs que son todos punteros a datos ya
    clasificados ($09xxxx-$0Fxxxx / $2xxxxx) o $FFFFFFFF (entrada vacía), con
    >= 2 punteros reales: tablas de punteros por estado/dirección/fase.
    -> [(addr, len)]"""
    def ok(v):
        return v == 0xFFFFFFFF or ((0x90000 <= v < 0x100000 or 0x200000 <= v < 0x300000)
                                   and v in mark)
    out = []
    for lo, hi in _gaps(p, mark):
        o = lo + (lo & 1)
        while o + 4 <= hi:
            if not ok(p.L(p.cpu(o))):
                o += 2
                continue
            st, real = o, 0
            while o + 4 <= hi and ok(p.L(p.cpu(o))):
                real += p.L(p.cpu(o)) != 0xFFFFFFFF
                o += 4
            if (o - st) // 4 >= min_n and real >= 2:
                out.append((p.cpu(st), o - st))
    return out


def indexed_tables(p, mark, code_end=0x9C608, max_len=0x200):
    """Tablas que el código indexa: `lea T.l,aN` (o `movea.l #T,aN`) seguido en
    <= 24 B de un acceso `(aN,dM.w)` / `d(aN,dM.w)` (move/add/cmp/movea con
    índice). El tamaño no se puede saber del código: se marca desde T hasta
    la siguiente dirección referenciada por el código o el siguiente byte ya
    clasificado (máx. max_len) -> [(addr, len)]."""
    refs, seeds = set(), set()
    for o in range(0, code_end - 6, 2):
        w = p.W(o)
        if w & 0xF1FF in (0x41F9, 0x207C):       # lea X.l,aN / movea.l #X,aN
            v = p.L(o + 2)
            if not 0x200000 <= v < 0x300000:
                continue
            refs.add(v)
            reg = (w >> 9) & 7
            qs, q, hops = [], o + 6, 0
            while len(qs) < 12 and q < code_end:
                x = p.W(q)
                if x == 0x6000 and hops < 2:          # bra.w: seguir el salto
                    q, hops = q + 2 + p.SW(q + 2), hops + 1
                    continue
                qs.append(q)
                q += 2
            for q in qs:
                x = p.W(q)
                # modo de dirección fuente o destino (aN,Xi): ext. word con bit 8 = 0
                src = x & 0x3F == 0x30 | reg and x >> 12 in (1, 2, 3, 9, 0xB, 0xC, 0xD)
                dst = x >> 12 in (1, 2, 3) and (x >> 6) & 7 == 6 and (x >> 9) & 7 == reg
                # add.l aN,dM / add.w aN,dM (dirección calculada a mano), adda dM,aN
                addr = (x & 0xF1F8 in (0xD088, 0xD048) and x & 7 == reg) or \
                    (x & 0xF0F8 == 0xD0C0 and (x >> 9) & 7 == reg) or \
                    (x & 0xF1F8 == 0xD1C0 and (x >> 9) & 7 == reg)
                if addr:
                    seeds.add(v)
                    break
                if (src or dst) and p.B(q + 2) & 0x08 == 0:
                    seeds.add(v)
                    break
                if x & 0xF1FF == 0x41F9 and (x >> 9) & 7 == reg:
                    break
    out = []
    srefs = sorted(refs)
    import bisect
    for v in sorted(seeds):
        if v in mark or not p.valid(v, 2):
            continue
        i = bisect.bisect_right(srefs, v)
        lim = srefs[i] if i < len(srefs) else v + max_len
        x = v
        while x < lim and x - v < max_len and x not in mark and p.valid(x, 1):
            x += 1
        if x > v:
            out.append((v, x - v))
    return out


SCENE_BY_DESC = 0x043562   # SceneLoader_ByIndex_043562: a0 = {.l script VM, .l entidades}


def scene_descs(p, code_end=0x9C608):
    """Descriptores de escena sueltos (fuera de la tabla $0916C8) pasados en a0
    a SceneLoader_ByIndex_043562: {.l script de la VM de escena, .l lista de
    entidades de 14 B hasta tipo 2}; el script se decodifica con
    scene_script_dump.decode_script -> [(addr, len)]."""
    m = LV.load_ssd()
    out = []
    for o in range(0, code_end - 12, 2):
        if p.W(o) != 0x41F9 or p.W(o + 6) != 0x4EB9 or p.L(o + 8) != SCENE_BY_DESC:
            continue
        d = p.L(o + 2)
        if d & 1 or not p.valid(d, 8):
            continue
        out.append((d, 8))
        ent = p.L(d + 4)
        x = ent
        while p.valid(x, 1) and p.B(x) != 2 and x - ent < 14 * 64:
            x += 14
        if p.valid(x, 1) and p.B(x) == 2:
            out.append((ent, x + 2 - ent))
        vm = p.L(d)
        if p.valid(vm, 2):
            ops, end = m.decode_script(p.off(vm), p.off(vm) + 0x1000)
            if end > p.off(vm):
                out.append((vm, end - p.off(vm)))
    return out


def code_ref_gaps(p, mark, code_end=0x9C608, max_len=0x100):
    """Último recurso: huecos que empiezan exactamente en una dirección cargada
    por el código (`lea X.l,aN`, `movea.l #X,aN`, `move.l #X,d(aN)`) y van
    hasta el siguiente byte clasificado o la siguiente dirección referenciada
    (máx. max_len). El formato es propio de cada rutina (registros de piezas,
    offsets, parámetros); la evidencia es el puntero exacto -> [(addr, len)]."""
    refs = set()
    for o in range(0, code_end - 6, 2):
        w = p.W(o)
        if w & 0xF1FF in (0x41F9, 0x207C) or w & 0xF1FF == 0x217C or w == 0x23FC:
            v = p.L(o + 2)
            if 0x200000 <= v < 0x300000 and v & 1 == 0:
                refs.add(v)
    srefs = sorted(refs)
    import bisect
    out = []
    for v in srefs:
        if v in mark or not p.valid(v, 2):
            continue
        i = bisect.bisect_right(srefs, v)
        lim = min(srefs[i] if i < len(srefs) else v + max_len, v + max_len)
        x = v
        while x < lim and x not in mark and p.valid(x, 1):
            x += 1
        out.append((v, x - v))
    return out


def data_ref_gaps(p, mark, max_len=0x100):
    """Huecos que empiezan exactamente en un long par $2xxxxx guardado en datos
    ya clasificados como struct/anim/lut (tablas de punteros, operandos de
    scripts); van hasta el siguiente byte clasificado o la siguiente dirección
    referenciada (máx. max_len) -> [(addr, len)]."""
    refs = set()
    for a, k in mark.items():
        if a & 1 or k not in ("struct", "anim", "lut") or mark.get(a + 3) != k:
            continue
        if p.valid(a, 4):
            v = p.L(a)
            if 0x200000 <= v < 0x300000 and v & 1 == 0:
                refs.add(v)
    import bisect
    srefs = sorted(refs)
    out = []
    for v in srefs:
        if v in mark or (v - 1) not in mark or not p.valid(v, 2):
            continue
        i = bisect.bisect_right(srefs, v)
        lim = min(srefs[i] if i < len(srefs) else v + max_len, v + max_len)
        x = v
        while x < lim and x not in mark and p.valid(x, 1):
            x += 1
        out.append((v, x - v))
    return out


def record_tail_gaps(p, mark, code_end=0x9C608, max_gap=64, max_back=0x100):
    """Huecos ≤ max_gap B con datos clasificados a ambos lados cuya dirección
    referenciada más próxima por debajo (lea/movea/move.l #X del código o long
    $2xxxxx de datos clasificados) está a ≤ max_back -> [(addr, len)]."""
    import bisect
    refs = set()
    for o in range(0, code_end - 6, 2):
        w = p.W(o)
        if w & 0xF1FF in (0x41F9, 0x207C) or w & 0xF1FF == 0x217C or w == 0x23FC:
            refs.add(p.L(o + 2))
    for a, k in mark.items():
        if a & 1 == 0 and k in ("struct", "anim", "lut") and mark.get(a + 3) == k \
                and p.valid(a, 4):
            refs.add(p.L(a))
    srefs = sorted(v for v in refs if 0x200000 <= v < 0x300000 and v & 1 == 0)
    out = []
    for lo, hi in _gaps(p, mark):
        a, b = p.cpu(lo), p.cpu(hi)
        if b - a > max_gap or (a - 1) not in mark or b not in mark:
            continue
        i = bisect.bisect_right(srefs, a) - 1
        if i >= 0 and a - srefs[i] <= max_back:
            out.append((a, b - a))
    return out


def classify(p):
    """-> dict cpu_addr -> kind (sólo bytes marcados)."""
    mark = {}

    def put(a, n, k):
        for x in range(a, a + n):
            mark.setdefault(x, k)

    # código incrustado (no cuenta en el censo, pero corta las rachas)
    for o in code_holes():
        mark[p.cpu(o)] = "code"

    # tablas por fórmula
    for lo, hi in LUTS:
        put(lo, hi - lo, "lut")
    for lo, hi in STRUCTS:
        put(lo, hi - lo, "struct")
    # paletas
    put(S.PAL_LO, 1024 * 64, "palette")
    put(S.PAL_HI, 512 * 64, "palette")
    # tilemaps de fondo
    m = LV.load_ssd()
    for i in range(m.N_DESC):
        for L in LV.scene_layers(m, i):
            put(L["tiles"], 4 * L["w"] * L["h"], "tilemap")
            if L["collision"]:
                put(L["collision"], 4 * L["w"] * L["h"], "collision")
    # mapa de colisión enlazado a mano a una cámara (Camera0_RelinkAndWrapScroll
    # _06896A: `lea X,aN; move.l aN,$78(a0)`): columnas de $40 B (16 celdas de
    # 4 B) hasta el siguiente bloque ya clasificado (se resuelve al final)
    cam_colmaps = []
    for o in range(0, 0x9C608 - 12, 2):
        w = p.W(o)
        if w & 0xF1FF == 0x41F9:
            r = (w >> 9) & 7
            if p.W(o + 6) == 0x2148 | r and p.W(o + 8) == 0x78 and \
                    0x200000 <= p.L(o + 2) < 0x300000:
                cam_colmaps.append(p.L(o + 2))
    # parches de tilemap: lea rec.l,a2 ; jsr Sprite_InvokeBlit8Params ($5022A)
    for rec, desc, w, h, tiles, ncol in blit_patches(p):
        put(tiles, 4 * h * ncol, "tilemap")
        put(rec, 20, "struct")
        put(desc, 8, "struct")
    descs = {d for r, d, w, h, t, nc in blit_patches(p)}
    for a, n, k in desc_chains(p):        # descriptores de las cadenas
        if k == "struct":
            descs.update(range(a, a + n, 8))
    for a, n in blit_records(p, descs):
        put(a, n, "struct")
    # módulos con parser propio (mission_streams, scene_scripts, template_index)
    for name, a, n, f in registry.REGISTRY:
        if name.startswith(STRUCT_PREFIXES):
            put(p.cpu(a), n, "struct")
    # frames + scripts de animación
    ops, frames = S.find_frames(p)
    for fa in frames:
        fr = S.parse_frame(p, fa)
        if fr:
            put(fa, fr["size"], "frame")
    scripts, _ = S.find_scripts(p, ops)
    op_starts = set()
    for s, (pics, info) in scripts.items():
        for a in info["visited"]:
            op = p.B(a)
            n = ops[a][4] if a in ops else S.OP_SIZE.get(op, 2)
            if op == 0x01:
                n = 6
            put(a, n, "anim")
            op_starts.add(a)
    # streams de spawn por misión
    for a, n in spawn_streams(p):
        put(a, n, "struct")
    # diccionarios de nombres de la pantalla de resultados
    for a, n in roster_names(p):
        put(a, n, "struct")
    # tablas de argumentos de formato fijo (cajas en pantalla, rejillas de sondeo)
    for a, n in code_arg_tables(p):
        put(a, n, "struct")
    # tablas por dificultad/rango de Tbl_Decode2D_0799DE
    d2d = decode2d_tables(p)
    for a, n in d2d:
        put(a, n, "struct")
    # tablas largas sin referencia pegadas tras otra Decode2D conocida (bloques
    # de variantes por enemigo): palabra 0 = 0 y 64 palabras < $1000
    # ... y hacia atrás: tablas cortas/largas contiguas que terminan justo donde
    # empieza una Decode2D conocida (palabra 0 = 0 o 2, mismo tamaño)
    for a, n in sorted(d2d):
        x = a - n
        while (x not in mark and x + n - 1 not in mark and p.valid(x, n)
               and _d2d_len(p, x) == n and p.W(x) in (0, 2)
               and all(p.W(x + 2 + 2 * i) < 0x1000 for i in range(32))):
            put(x, n, "struct")
            x -= n
    for a, n in sorted(d2d):
        x = a + n
        while (x not in mark and p.valid(x, 130) and p.W(x) == 0
               and all(p.W(x + 2 + 2 * i) < 0x1000 for i in range(64))
               and any(p.W(x + 2 + 2 * i) for i in range(64))):
            put(x, 130, "struct")
            x += 130
    # tablas de punteros sin referencia directa cuyas >= 3 entradas son tablas
    # Decode2D de formato largo (palabra 0 = 0) contiguas, la primera ya conocida
    # por el código (p. ej. $2D4274 del jefe de la misión 5)
    d2d_known = {a for a, n in d2d}
    for x in range(0x200000, 0x300000 - 12, 2):
        if x in mark or not p.valid(x, 12) or p.L(x) not in d2d_known:
            continue
        k = 0
        while (k < 16 and p.valid(x + 4 * k, 4) and x + 4 * k not in mark
               and p.L(x + 4 * k) == p.L(x) + 130 * k and p.valid(p.L(x + 4 * k), 130)
               and p.W(p.L(x + 4 * k)) == 0):
            k += 1
        if k >= 3:
            put(x, 4 * k, "struct")
            put(p.L(x), 130 * k, "struct")
    # registros de Spawner_Handler_077c98 (+ tablas de 16 handlers)
    for a, n in spawner_records(p):
        put(a, n, "struct")
    # registros de spawner sin referencia contiguos (antes/después) a otros
    # conocidos: {b 0/1, b ritmo, ..., b +$D = 0, l handler de código}
    def _spw(x):
        return (p.valid(x, 18) and p.B(x) in (0, 1) and p.B(x + 13) == 0
                and 0x400 <= p.L(x + 14) < 0x9C608 and p.L(x + 14) & 1 == 0)
    known = [a for a, n in spawner_records(p) if n == 18]
    for _ in range(3):
        for a in sorted(known):
            for x in (a - 18, a + 18):
                if _spw(x) and all(y not in mark for y in range(x, x + 18)):
                    put(x, 18, "struct")
                    known.append(x)
    # registros de spawner sin referencia directa cuyo +$E apunta justo detrás a
    # una tabla de 16 handlers de código (+$D != 0)
    for x in range(0x200000, 0x300000 - 82, 2):
        if x in mark or x + 17 in mark or not p.valid(x, 82) or p.B(x + 13) == 0 or p.L(x + 14) != x + 18:
            continue
        k = 0
        while k < 16 and 0x400 <= p.L(x + 18 + 4 * k) < 0x9C608 and p.L(x + 18 + 4 * k) & 1 == 0:
            k += 1
        if k >= 4:
            put(x, 18 + 4 * k, "struct")
    # textos a máquina de la presentación de misión (+ sus scripts de sprite)
    fx_tabs, fx_scripts = fix_type_streams(p)
    for a, n in fx_tabs:
        put(a, n, "struct")
    # scripts instalados desde el código (EntitySetSpriteMap / move.l #,$3C(a6))
    seen = set()
    cs_tabs, cs_scripts = code_script_tables(p)
    for a, n in cs_tabs:
        put(a, n, "struct")
    for v in sorted(set(code_script_seeds(p)) | set(fx_scripts) | cs_scripts):
        for a, n in walk_struct(p, v, seen) or []:
            put(a, n, "anim")
            op_starts.add(a)
    # op 19 SetField70: $70(a6) = operando+2 -> registro de 12 B (6 palabras con
    # signo: caja de colisión / ataque por frame; destinos con stride 12)
    op19 = set()
    for a in op_starts:
        if p.B(a) == 0x19 and p.valid(a + 2, 4):
            t = p.L(a + 2)
            if t & 1 == 0 and p.valid(t, 12):
                put(t, 12, "struct")
                op19.add(t)
    # registros de 12 B sin referencia en bloques de op 19 (frames sin caja
    # propia): huecos múltiplos de 12 entre/tras registros conocidos con
    # palabras pequeñas (|v| < $200)
    def _small12(x):
        return p.valid(x, 12) and all(abs(p.SW(x + 2 * i)) < 0x200 for i in range(6))
    for t in sorted(op19):
        x = t + 12
        while x not in mark and x not in op19 and _small12(x):
            y = x
            while y not in mark and _small12(y) and y - x < 12 * 64:
                y += 12
            if y in op19 or (y - x <= 12 * 32 and y in mark and p.valid(y, 12) and y - 12 >= x):
                put(x, y - x, "struct")
            break
    # listas de cajas de golpe (+$48) y de ataque (+$4C) de los ops 09/0A y del código
    for a, n in hitbox_lists(p, op_starts) + attack_lists(p, op_starts):
        put(a, n, "struct")
    # textos: cadenas $FF de List_ApplyWithSentinelFF_* y filas fix de Cut_TextPanel
    for a, n in ff_strings(p) + fix_row_texts(p):
        put(a, n, "struct")
    for a, n in fix_anim_lists(p):
        put(a, n, "struct")
    for a, n in motion_ptr_tables(p, mark):
        put(a, n, "struct")
    # PathScripts (VM de movimiento, +$90 de la tarea) y sus tablas
    for a, n, k in path_scripts(p):
        put(a, n, k)
    # scripts apuntados por algún long y recorribles de punta a punta
    for a, n in pointed_scripts(p, mark):
        put(a, n, "anim")
    for _ in range(2):
        for a, n in gap_scripts(p, mark):
            put(a, n, "anim")
    # cadenas de descriptores {w, h, tiles} de parches (+ sus tilemaps)
    for a, n, k in desc_chains(p):
        put(a, n, k)
    # rachas completas de frames sin puntero directo
    for a, n in frame_runs(p, mark):
        put(a, n, "frame")
    # celdas de tilemap sin descriptor
    for a, n in tilemap_gaps(p, mark):
        put(a, n, "tilemap")
    for a, n in scene_descs(p):
        put(a, n, "struct")
    # tablas indexadas por el código (lea T,aN + acceso (aN,dM.w))
    for a, n in indexed_tables(p, mark):
        put(a, n, "struct")
    # tablas de punteros a datos ya clasificados (dos pasadas: tablas de tablas)
    for _ in range(2):
        for a, n in pointer_tables(p, mark):
            put(a, n, "struct")
    # rejillas de Sprite_MultiBlitClippedX contiguas a otras conocidas: {dx, dy,
    # w, h} (|dx|,|dy| < $200, 1 <= w,h <= 32) + w*h bytes (alineado a 2)
    grids = [a for a, n in code_arg_tables(p) if p.valid(a, 8) and
             8 + p.W(a + 4) * p.W(a + 6) == n]
    def _grid(x):
        if x & 1 or not p.valid(x, 8):
            return 0
        dx, dy, w, h = p.SW(x), p.SW(x + 2), p.W(x + 4), p.W(x + 6)
        if abs(dx) >= 0x200 or abs(dy) >= 0x200 or not (1 <= w <= 32 and 1 <= h <= 32):
            return 0
        n = 8 + w * h
        return n if p.valid(x, n) else 0
    for _ in range(2):
        for g in sorted(grids):
            n = 8 + p.W(g + 4) * p.W(g + 6)
            for x in (g + n, g + n + 1):
                x += x & 1
                k = _grid(x)
                while k and all(y not in mark for y in range(x, x + k)):
                    put(x, k, "struct")
                    grids.append(x)
                    x += k + (k & 1)
                    k = _grid(x)
    # registros de spawner sueltos de 18 B que llenan exactamente un hueco entre
    # datos ya clasificados (handler de efectos $077xxx..$080xxx)
    for lo, hi in _gaps(p, mark):
        if (hi - lo) % 18 == 0 and lo & 1 == 0 and hi - lo <= 18 * 4:
            xs = [p.cpu(o) for o in range(lo, hi, 18)]
            if all(_spw(x) and 0x76000 <= p.L(x + 14) < 0x82000 for x in xs):
                for x in xs:
                    put(x, 18, "struct")
    # Player_* (air_death_crouch_036632): `lea X,a0; move.l -4(a0),$74(a6)` -> el
    # long que precede al script es un parámetro; los scripts de estas tablas son
    # bloques de 10 B {.l parámetro, 01 00 .l salto}
    for o in range(0, 0x9C608 - 12, 2):
        if p.W(o) == 0x41F9 and p.W(o + 6) == 0x2D68 and p.W(o + 8) == 0xFFFC \
                and p.W(o + 10) == 0x74:
            x = p.L(o + 2)
            put(x - 4, 4, "struct")
            # bloques hermanos {.l, 01 00 .l} contiguos antes y después
            for step in (10, -10):
                y = x + step if step > 0 else x - 10
                while p.valid(y - 4, 10) and p.W(y) == 0x0100 and \
                        p.valid(p.L(y + 2), 2) and mark.get(p.L(y + 2)) == "anim":
                    put(y - 4, 4, "struct")
                    put(y, 6, "anim")
                    y += step
    # (sustituye al "struct" genérico que haya puesto la heurística de tablas
    # indexadas sobre la primera parte del mapa)
    for x0 in cam_colmaps:
        y = x0
        while y - x0 < 0x2000 and mark.get(y) in (None, "struct") and p.valid(y, 1):
            y += 1
        if (y - x0) % 0x40 == 0:
            for x in range(x0, y):
                mark[x] = "collision"
    # op 07 SetLong con valor en datos $2xxxxx: bloque referenciado hasta el
    # siguiente byte ya clasificado (≤ $100)
    refs7 = sorted({p.L(a + 2) for a in op_starts if p.B(a) == 0x07 and p.valid(a + 2, 6)
                    and 0x200000 <= p.L(a + 2) < 0x300000})
    for v in refs7:
        if v & 1 or v in mark:
            continue
        x = v
        while x - v < 0x100 and x not in mark and p.valid(x, 1):
            x += 1
        put(v, x - v, "struct")
    # tablas de saltos: huecos que empiezan con ≥ 3 longs pares a código
    for lo, hi in _gaps(p, mark):
        o = lo + (lo & 1)
        st = o
        while o + 4 <= hi and 0x400 <= p.L(p.cpu(o)) < 0x9C608 and p.L(p.cpu(o)) & 1 == 0:
            o += 4
        if (o - st) // 4 >= 3:
            put(p.cpu(st), o - st, "struct")
    # último recurso: datos que empiezan en un puntero exacto del código
    for a, n in code_ref_gaps(p, mark):
        put(a, n, "struct")
    # ídem para punteros guardados en datos ya clasificados (struct/anim/lut):
    # p. ej. la tabla de $2D545E (M5Boss_BombFall) -> scripts en $2D88CC...
    for a, n in data_ref_gaps(p, mark):
        put(a, n, "struct")
    # colas de registro: hueco pequeño (≤ 64 B) entre datos clasificados cuyo
    # registro empieza en una dirección referenciada (código o datos) a ≤ $100
    # antes del hueco (p. ej. los registros de 40 B de $2C8C7E/$2C8CA6 del Tank)
    for a, n in record_tail_gaps(p, mark):
        put(a, n, "struct")
    # continuación periódica: hueco ≤ 16 B cuyo periodo anterior (P = 4..64)
    # tiene exactamente un tramo de un solo tipo de la misma longitud, con los
    # mismos tipos de vecino a ambos lados (registros {anim 8 B, struct 6 B}
    # repetidos de fortress_04e580 en $294846...). Se hace en orden para que
    # cada hueco sirva de modelo al siguiente.
    for lo, hi in _gaps(p, mark):
        a, b = p.cpu(lo), p.cpu(hi)
        g = b - a
        if g > 16 or (a - 1) not in mark or b not in mark:
            continue
        for P in range(g + 2, 65):
            k = mark.get(a - P)
            if k is None or any(mark.get(x - P) != k for x in range(a, b)):
                continue
            if mark.get(a - P - 1) == k or mark.get(b - P) == k:
                continue
            if mark.get(a - P - 1) == mark.get(a - 1) and mark.get(b - P) == mark.get(b):
                put(a, g, k)
                break
    # terminadores sueltos: huecos de ≤ 4 B sólo de $FF entre datos ya
    # clasificados (fin de lista $FFFF / $FFFFFFFF tras un `1D 00` o un registro)
    for lo, hi in _gaps(p, mark):
        a, b = p.cpu(lo), p.cpu(hi)
        if b - a <= 4 and (a - 1) in mark and b in mark and \
                all(p.B(x) == 0xFF for x in range(a, b)):
            put(a, b - a, "struct")
    # el $FFFF que cierra el último script antes del relleno final de la ROM
    for lo, hi in _gaps(p, mark):
        a = p.cpu(lo)
        if (a - 1) in mark and p.valid(a, 2) and p.W(a) == 0xFFFF and \
                p.W(a - 2) == 0x1D00:
            put(a, 2, "struct")
    # colas de rampa: hueco cuyos bytes continúan sin saltos (paso 0/+1) una
    # rampa monótona ya clasificada justo antes (p. ej. la tabla 01..0A que
    # termina en $2831B2)
    for lo, hi in _gaps(p, mark):
        a, b = p.cpu(lo), p.cpu(hi)
        if (a - 1) not in mark or b - a > 64 or a - 16 < 0x200000:
            continue
        seq = [p.B(x) for x in range(a - 16, b)]
        if all(0 <= seq[i + 1] - seq[i] <= 1 for i in range(len(seq) - 1)) and \
                seq[-1] > seq[0]:
            put(a, b - a, "lut")
    return mark


def census(p, mark):
    rows = []
    holes = code_holes()
    for f, (lo, hi) in modules():
        c = Counter()
        zrun = 0
        for o in range(lo, hi):            # registry = offsets del fichero de 2 MiB
            if o in holes:
                continue
            k = mark.get(p.cpu(o))
            if k is None:
                if p.d[o] == 0:
                    zrun += 1
                    continue
                k = "other"
            if zrun:
                c["zero" if zrun >= 16 else "other"] += zrun
                zrun = 0
            c[k] += 1
        if zrun:
            c["zero" if zrun >= 16 else "other"] += zrun
        rows.append((f, lo, hi, c))
    return rows


def main():
    p = S.PRom()
    rows = census(p, classify(p))
    hdr = "%-34s %-17s %8s " % ("módulo", "región", "bytes") + " ".join("%9s" % k for k in KINDS)
    print(hdr)
    print("-" * len(hdr))
    tot = Counter()
    for f, lo, hi, c in rows:
        tot.update(c)
        n = sum(c.values())
        print("%-34s $%06X-$%06X %8d " % (f, lo, hi, n)
              + " ".join("%8.1f%%" % (100 * c[k] / n) for k in KINDS))
    n = sum(tot.values())
    print("%-34s %-17s %8d " % ("TOTAL", "", n) + " ".join("%8.1f%%" % (100 * tot[k] / n) for k in KINDS))
    if "--json" in sys.argv:
        out = sys.argv[sys.argv.index("--json") + 1]
        json.dump([dict(module=f, start="%06X" % lo, end="%06X" % hi,
                        bytes={k: c[k] for k in KINDS}) for f, lo, hi, c in rows],
                  open(out, "w"), indent=1)


if __name__ == "__main__":
    main()
