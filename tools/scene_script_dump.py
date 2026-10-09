#!/usr/bin/env python3
"""
scene_script_dump.py — transcripción estructurada de la tabla de escenas
$0916C8 y de sus scripts (SceneScriptVM_Frame_0437DA + SceneLoader_Main_043568).
=============================================================================

La región $0916C8..$0967B4 son DATOS del subsistema de escena/scroll:

  * SceneDescTable_0916C8[32]  : pares {script_vm_ptr, entry_script_ptr}
                                 (8 B; sólo 0..15 válidos, el resto es el
                                 arranque del primer bytecode: la tabla y los
                                 datos están solapados en ROM).
  * SceneEntities_XXXXXX       : registros de 14 B {type, subop, tmpl.l,
                                 payload[8]} terminados en type==2
                                 (SceneLoader_Main_043568, pasada 1 y 2).
  * SceneScript_XXXXXX         : bytecode de la VM de scroll (23 opcodes,
                                 strides exactos de scene_script_vm_0437da.s)
                                 con CALLBACKS 68000 EMBEBIDOS (op $06: el cb
                                 devuelve en a1 el nuevo PC; op $14: en a0).
  * SceneTrig_XXXXXX           : tablas de words referenciadas por op $11
                                 (`move.l a1,$12(a0)`), terminadas en $FFFF.

Emite el .s con `.dc.w` por registro y un comentario por línea, en el mismo
estilo que asm/mission_streams_0e8524.s, y las líneas de REGISTRY.

Uso:
    python3 tools/scene_script_dump.py -o asm/scene_scripts_0916c8.s --registry
"""
import argparse
import os
import struct
import subprocess
import sys
import tempfile

HERE = os.path.dirname(os.path.abspath(__file__))
ROOT = os.path.abspath(os.path.join(HERE, ".."))
PROM = os.path.join(ROOT, "build", "mslug_prom.bin")
SRC_NAME = "scene_scripts_0916c8.s"
WAVE = "Wave AAAAA"

rom = open(PROM, "rb").read()


def W(a): return struct.unpack(">H", rom[a:a + 2])[0]
def L(a): return struct.unpack(">I", rom[a:a + 4])[0]
def SW(a): return struct.unpack(">h", rom[a:a + 2])[0]


TABLE = 0x0916C8
REGION_END = 0x0967B4
N_DESC = 16

# op -> (longitud total en bytes, nombre corto, formateador de parámetros)
OPS = {
    0x00: (22, "bind_path",  lambda a: f"ent=${L(a+2):06X} ruta=${L(a+6):06X}.. cont=${L(a+0xE):06X} ancla=({SW(a+0x12)},{SW(a+0x14)})"),
    0x01: (12, "set_fields", lambda a: f"ent=${L(a+2):06X} +72={rom[a+6]:02X} +74=${W(a+8):04X} +76=${W(a+10):04X}"),
    0x02: (2,  "END_FRAME",  lambda a: "cede el frame (integra scroll+camara)"),
    0x03: (6,  "warp",       lambda a: f"camara=({SW(a+2)},{SW(a+4)}) modo_eje=1 high_water=0"),
    0x04: (8,  "spawn",      lambda a: f"tmpl=${L(a+2):06X} a={rom[a+6]:02X} b={rom[a+7]:02X}  ($51B1C)"),
    0x05: (6,  "detach",     lambda a: f"ent=${L(a+2):06X}  (+$0E=0, $51ED6, commit MMIO)"),
    0x06: (2,  "wait_cb",    lambda a: "callback embebido: d0!=0 -> re-poll; a1 = nuevo PC"),
    0x07: (6,  "call_ece",   lambda a: f"ent=${L(a+2):06X}  ($51ECE)"),
    0x08: (6,  "call_ed6",   lambda a: f"ent=${L(a+2):06X}  ($51ED6)"),
    0x09: (6,  "probe_ece",  lambda a: f"ent=${L(a+2):06X}  (CameraHook_Probe08 + $51ECE)"),
    0x0A: (None, "slot_pairs", None),
    0x0B: (6,  "cond_clear", lambda a: f"ent=${L(a+2):06X}  ($51F02; si +78 -> clear collmap)"),
    0x0C: (4,  "set_6eac",   lambda a: f"$106EAC.b = {rom[a+2]:02X}"),
    0x0D: (14, "call_args",  lambda a: f"fn=${L(a+2):06X} args={rom[a+6:a+14].hex()}"),
    0x0E: (6,  "alloc_task", lambda a: f"handler=${L(a+2):06X}  (Task_AllocFromFreeList)"),
    0x0F: (12, "set_limits", lambda a: f"minX={SW(a+2)} minY={SW(a+4)} maxX={SW(a+6)} maxY={SW(a+8)} slope={SW(a+10)}"),
    0x10: (6,  "set_6466",   lambda a: f"$108164={W(a+2):04X} $108166={W(a+4):04X}"),
    0x11: (10, "set_trig",   lambda a: f"ent=${L(a+2):06X} tabla=${L(a+6):06X}  (-> $12(ent))"),
    0x12: (4,  "set_axis",   lambda a: f"modo_eje=${rom[a+2]:02X}"),
    0x13: (4,  "clamp",      lambda a: f"d0={W(a+2):04X}  (ClampD0ToRange)"),
    0x14: (2,  "call_newpc", lambda a: "callback embebido: a0 = nuevo PC"),
    0x15: (6,  "set_campos", lambda a: f"camara=({SW(a+2)},{SW(a+4)})"),
    0x16: (4,  "set_progress", lambda a: f"high_water/base=${W(a+2):04X}"),
}


def cb_end(p, limit, reg):
    """Fin del callback embebido: busca `lea d(pc),aN ; rts` (reg=0x43 a1 / 0x41 a0)."""
    while p < limit - 4:
        if rom[p] == reg and rom[p + 1] == 0xFA and rom[p + 4] == 0x4E and rom[p + 5] == 0x75:
            return p + 6, p + 2 + SW(p + 2)
        p += 2
    return None, None


def decode_script(start, limit):
    """Lista de (addr, kind, len, info). kind: op | code | tail | ??"""
    pc = start
    out = []
    while pc < limit:
        op = W(pc)
        if op not in OPS:
            out.append((pc, "??", None, op))
            return out, pc
        n, _, _ = OPS[op]
        if op == 0x0A:
            p = pc + 2
            while W(p) != 0xFFFF:
                p += 4
            out.append((pc, "op", p + 2 - pc, op))
            pc = p + 2
        elif op in (0x06, 0x14):
            e, nxt = cb_end(pc + 2, limit, 0x43 if op == 0x06 else 0x41)
            if e is None:
                out.append((pc, "??", None, op))
                return out, pc
            out.append((pc, "op", 2, op))
            out.append((pc + 2, "code", e - (pc + 2), nxt))
            if nxt != e:
                out.append((e, "tail", nxt - e, None))
            pc = nxt
        else:
            out.append((pc, "op", n, op))
            pc += n
    return out, pc


def disasm(addr, size):
    """Desensambla con capstone; devuelve lista de (addr, size, texto)."""
    from capstone import Cs, CS_ARCH_M68K, CS_MODE_M68K_000
    cs = Cs(CS_ARCH_M68K, CS_MODE_M68K_000)
    rows = []
    for ins in cs.disasm(rom[addr:addr + size], addr):
        rows.append((ins.address, ins.size, f"{ins.mnemonic} {ins.op_str}".strip()))
    return rows


def dcw(a, n):
    return ",".join(f"0x{W(a+i):04x}" for i in range(0, n, 2))


def gas_insn(addr, size):
    """Instrucción del callback en GAS del proyecto (solo los 5 patrones vistos)."""
    b = rom[addr:addr + size]
    w = W(addr)
    if w == 0x0C79 and size == 8:   # cmpi.w #imm, abs.l
        return f"cmpi.w  #0x{W(addr+2):x}, 0x{L(addr+4):x}.l"
    if w == 0x0C39 and size == 8:   # cmpi.b #imm, abs.l
        return f"cmpi.b  #0x{W(addr+2) & 0xFF:x}, 0x{L(addr+4):x}.l"
    if w == 0x13FC and size == 8:   # move.b #imm, abs.l
        return f"move.b  #0x{W(addr+2) & 0xFF:x}, 0x{L(addr+4):x}.l"
    if w == 0x23FC and size == 10:  # move.l #imm, abs.l
        return f"move.l  #0x{L(addr+2):x}, 0x{L(addr+6):x}.l"
    if (w & 0xF0F8) == 0x50C0 and size == 2:  # scc d0
        cc = ["t", "f", "hi", "ls", "cc", "cs", "ne", "eq", "vc", "vs", "pl", "mi", "ge", "lt", "gt", "le"][(w >> 8) & 0xF]
        return f"s{cc}     d{w & 7}"
    if w in (0x43FA, 0x41FA) and size == 4:
        return None  # lea d(pc),aN -> label
    if w == 0x4E75:
        return "rts"
    return None


def emit():
    descs = [(L(TABLE + 8 * i), L(TABLE + 8 * i + 4)) for i in range(N_DESC)]
    # Segmentación: todos los punteros únicos ordenados delimitan bloques.
    starts = sorted(set([p for d in descs for p in d]))
    bounds = {}
    for i, s in enumerate(starts):
        bounds[s] = starts[i + 1] if i + 1 < len(starts) else REGION_END
    scripts = {c for c, _ in descs}
    entities = {e for _, e in descs}

    lines = []
    reg = []
    sym = {}

    def section(name, addr, size, comment):
        lines.append("")
        lines.append(f"        .globl  {name}")
        lines.append(f'        .section .text.{name}, "ax", @progbits')
        lines.append(f"{name}:" + " " * max(1, 44 - len(name) - 1) + f"| {comment}")
        reg.append((name, addr, size))
        sym[addr] = name

    # ---- tabla de descriptores -------------------------------------------
    tsize = starts[0] - TABLE
    section("SceneDescTable_0916C8", TABLE, tsize,
            f"{N_DESC} descriptores x 8 B = {tsize} B (escenas 0..15)")
    callers = {0: "attract state 0", 1: "attract state 1", 2: "attract state 2", 3: "attract state 3",
               4: "attract state 4", 5: "attract state 5", 6: "= escena 1 (alias)", 7: "attract state 7 / = escena 2",
               9: "anim_state_machine_08cxxx (+42)", 10: "GameOver_Boot_08f91a", 11: "HiScore_Tpl_Common_097816",
               14: "cutscene_anim_08baxx (+014)", 15: "cutscene_anim_08baxx (+028/+06c)"}
    for i, (c, e) in enumerate(descs):
        lines.append(f"        .dc.l   0x{c:08x},0x{e:08x}" + " " * 12 +
                     f"| [{i:2}] vm=${c:06X} entities=${e:06X}  {callers.get(i, '')}")

    # ---- bloques ------------------------------------------------------------
    for s in starts:
        e = bounds[s]
        if s in entities:
            n = 0
            a = s
            recs = []
            while rom[a] != 2:
                recs.append(a)
                a += 14
            term = a
            size = e - s
            section(f"SceneEntities_{s:06X}", s, size,
                    f"{len(recs)} registros de 14 B + terminador ({size} B)")
            for a in recs:
                lines.append("        .dc.w   " + dcw(a, 14))
                lines.append(f"                | ${a:06X} type={rom[a]} subop={rom[a+1]:02X} tmpl=${L(a+2):06X} "
                             f"payload={rom[a+6:a+14].hex()}")
            lines.append("        .dc.w   " + dcw(term, e - term) +
                         f"{'':<24}| ${term:06X} terminador type=2" +
                         (f" (+{e-term-2} B de relleno)" if e - term > 2 else ""))
        elif s in scripts:
            out, stop = decode_script(s, e)
            size = stop - s
            # tablas de trigger (op $11) que caen tras el script
            trig_ptrs = sorted({L(a + 6) for a, k, n, op in out if k == "op" and op == 0x11
                                and s <= L(a + 6) < e})
            section(f"SceneScript_{s:06X}", s, size,
                    f"bytecode VM de escena ({size} B, {sum(1 for o in out if o[1]=='op')} ops)")
            depth = 0
            for a, k, n, info in out:
                if k == "op":
                    op = info
                    _, nm, fmt = OPS[op]
                    if op == 0x0A:
                        lines.append(f"        .dc.w   0x000a" + " " * 30 + f"| ${a:06X} op $0A slot_pairs ($2B58): {{slot,bank}} hasta $FFFF")
                        p = a + 2
                        while W(p) != 0xFFFF:
                            lines.append(f"        .dc.w   0x{W(p):04x},0x{W(p+2):04x}" + " " * 17 +
                                         f"|   slot ${W(p):02X} <- bank ${W(p+2):04X}")
                            p += 4
                        lines.append(f"        .dc.w   0xffff" + " " * 30 + f"| ${p:06X} fin de pares")
                        continue
                    lines.append("        .dc.w   " + dcw(a, n))
                    lines.append(f"                | ${a:06X} op ${op:02X} {nm}: {fmt(a)}")
                elif k == "code":
                    nxt = info
                    lines.append(f"                | ${a:06X} --- callback 68000 embebido ({n} B) -> PC=${nxt:06X} ---")
                    for ia, sz, txt in disasm(a, n):
                        g = gas_insn(ia, sz)
                        if g is None and W(ia) in (0x43FA, 0x41FA):
                            tgt = ia + 2 + SW(ia + 2)
                            an = "a1" if W(ia) == 0x43FA else "a0"
                            lines.append(f"        lea     .L{tgt:06x}(pc), {an}" + " " * 14 + f"| ${ia:06X}  {an} = nuevo PC")
                            continue
                        if g is None:
                            lines.append("        .dc.w   " + dcw(ia, sz) + f"   | ${ia:06X}  {txt}")
                        else:
                            lines.append(f"        {g:<38} | ${ia:06X}  {txt}")
                    if nxt < e:
                        lines.append(f".L{nxt:06x}:")
                elif k == "tail":
                    lines.append("        .dc.w   " + dcw(a, n) + f"   | ${a:06X} datos entre callback y PC")
            # cola huerfana de la region: comparador CCR (codigo, sin callers)
            if stop < e and e == REGION_END and e - stop == 16 and W(stop) == 0x226E:
                nm = f"ChildRank_CmpByte10_{stop:06X}"
                section(nm, stop, 16, "CCR: C=1 si child.rank(+$10) > ent.rank(+$10). Sin callers (huerfana)")
                lines += ["        movea.l 0x8(a6), a1                    | +00  a1 = entidad hija (+$08)",
                          "        move.b  0x10(a6), d0                   | +04  d0 = rango propio",
                          "        cmp.b   0x10(a1), d0                   | +08  vs rango de la hija",
                          f"        bcs.w   SetXN_{stop+0x16:06x}{'':<19}| +0c  menor -> C=1 (isla SetXN)",
                          f"                                               | +10  cae en ClearXN_{stop+0x10:06x}"]
                stop = e
            # resto del bloque: tablas de trigger
            if stop < e:
                p = stop
                idx = 0
                while p < e:
                    q = p
                    while q < e and W(q) != 0xFFFF:
                        q += 2
                    n = min(q + 4, e) - p
                    nm = f"SceneTrig_{p:06X}"
                    section(nm, p, n, f"tabla de trigger (op $11), {n} B" +
                            ("" if p in trig_ptrs else "  (sin referencia directa op $11 en este script)"))
                    words = [W(p + i) for i in range(0, n, 2)]
                    # pares {valor, umbral}: emitir de 2 en 2 words
                    i = 0
                    while i < len(words):
                        chunk = words[i:i + 8]
                        lines.append("        .dc.w   " + ",".join(f"0x{w:04x}" for w in chunk) +
                                     " " * max(1, 40 - 7 * len(chunk)) + f"| ${p+2*i:06X}")
                        i += 8
                    p += n
                    idx += 1
        else:
            section(f"SceneData_{s:06X}", s, e - s, "datos sin clasificar")
            for a in range(s, e, 16):
                n = min(16, e - a)
                lines.append("        .dc.w   " + dcw(a, n) + f"   | ${a:06X}")
    return lines, reg


def verify(src_text, reg):
    with tempfile.TemporaryDirectory() as td:
        s = os.path.join(td, "d.s"); open(s, "w").write(src_text)
        o = os.path.join(td, "d.o")
        r = subprocess.run(["m68k-linux-gnu-gcc", "-m68000", "-nostdlib", "-nostartfiles", "-ffreestanding",
                            "-Wa,--register-prefix-optional", "-c", s, "-o", o], capture_output=True, text=True)
        if r.returncode:
            print(r.stderr); return False
        ld = os.path.join(td, "d.ld")
        Ls = ["OUTPUT_FORMAT(elf32-m68k)", "OUTPUT_ARCH(m68k)", "SECTIONS {"]
        for name, addr, size in sorted(reg, key=lambda x: x[1]):
            Ls += [f"  . = 0x{addr:X};", f"  .text.{name} : {{ KEEP(*(.text.{name})) }}"]
        Ls += ["  /DISCARD/ : { *(.text) *(.data*) *(.bss*) *(.comment) *(.note.*) }", "}"]
        open(ld, "w").write("\n".join(Ls))
        elf = os.path.join(td, "d.elf")
        import importlib.util
        spec = importlib.util.spec_from_file_location("symbols", os.path.join(HERE, "symbols.py"))
        m = importlib.util.module_from_spec(spec); spec.loader.exec_module(m)
        spec2 = importlib.util.spec_from_file_location("registry", os.path.join(HERE, "registry.py"))
        m2 = importlib.util.module_from_spec(spec2); spec2.loader.exec_module(m2)
        ours = {n for n, _, _ in reg}
        defs = [f"--defsym={n}=0x{a:X}" for a, n in m.SYMBOLS.items() if n not in ours]
        defs += [f"--defsym={n}=0x{a:X}" for n, a, _, _ in m2.REGISTRY if n not in ours and a not in m.SYMBOLS]
        r = subprocess.run(["m68k-linux-gnu-ld", "-T", ld, *defs, o, "-o", elf], capture_output=True, text=True)
        if r.returncode:
            print(r.stderr); return False
        ok = True
        for name, addr, size in reg:
            b = os.path.join(td, name + ".bin")
            subprocess.run(["m68k-linux-gnu-objcopy", "-O", "binary", f"--only-section=.text.{name}", elf, b])
            got = open(b, "rb").read()[:size]
            if got != rom[addr:addr + size]:
                print(f"!! MISMATCH {name} @ ${addr:06X} ({size} B) got {len(got)} B"); ok = False
        return ok


def main():
    ap = argparse.ArgumentParser()
    ap.add_argument("-o", "--output")
    ap.add_argument("--registry", action="store_true")
    a = ap.parse_args()
    lines, reg = emit()
    header = [
        "| " + "=" * 76,
        "|  Metal Slug 1 (Neo Geo, M68000) — decompilación matching",
        f"|  {WAVE} — (borrador)",
        f"|  Región: ${TABLE:06X}..${REGION_END:06X}  ({REGION_END-TABLE:,} B, {len(reg)} entradas)",
        "| " + "=" * 76,
        "|",
        "|  BORRADOR generado por tools/scene_script_dump.py — pendiente de análisis",
        "|  semántico.",
        "|",
        "|  Verificación: cada sección .text.<Sym> se coloca en su dirección CPU",
        "|  absoluta y reensambla byte-exacta contra build/mslug_prom.bin",
        "|  (MD5 816b3f74c76b3373993407615f1850fe).",
        "| " + "=" * 76,
        "",
        "        .text",
    ]
    text = "\n".join(header + lines) + "\n"
    total = sum(s for _, _, s in reg)
    covered_end = max(a + s for _, a, s in reg)
    print(f"entradas={len(reg)} bytes={total} fin=${covered_end:06X} (region end ${REGION_END:06X})", file=sys.stderr)
    ok = verify(text, reg)
    print("VERIFY", "OK" if ok else "FAIL", file=sys.stderr)
    if a.output:
        open(a.output, "w").write(text)
    if a.registry:
        for name, addr, size in sorted(reg, key=lambda x: x[1]):
            print(f'    ("{name}",{"":<{max(1, 45-len(name))}}0x{addr:06X}, {size:5d}, "{SRC_NAME}"),')


if __name__ == "__main__":
    main()
