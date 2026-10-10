#!/usr/bin/env python3
"""
attract_sprites_dump.py — transcripción estructurada de las 8 listas de
sprites de las escenas attract ($096BBC..$097730), destino de la tabla
JumpTable_096B9C de ClampAndLookup8_096B7E (ver attract_sub_helpers_096xxx.s
y la corrección arquitectónica de camera_list_ctx_helpers_wave_ii.s).

Formato de registro (20 B, stride $14, iterado por AttractCuller_Cam0_0969C2
y AttractCuller_Cam1_096A0E; `Fn_0005DCCE` instancia el sprite):

    struct AttractSprite {
        u16  flags;      /* +0  $0040/$0100/$0140: tipo de instancia (b6/b8) */
        s16  x_world;    /* +2  comparado con camara0 + $140 (culling)        */
        s16  y_world;    /* +4                                               */
        u32  template;   /* +6  plantilla de sprite en ROM ($4D70C, $52A26..) */
        u8   params[10]; /* +A  parametros; $FFFF x3 = sin extension          */
    };
    terminador: u16 $FFFF

Uso:
    python3 tools/attract_sprites_dump.py -o asm/attract_sprite_lists_096bbc.s --registry
"""
import argparse, os, struct, sys

HERE = os.path.dirname(os.path.abspath(__file__))
ROOT = os.path.abspath(os.path.join(HERE, ".."))
rom = open(os.path.join(ROOT, "build", "mslug_prom.bin"), "rb").read()
SRC_NAME = "attract_sprite_lists_096bbc.s"


def W(a): return struct.unpack(">H", rom[a:a + 2])[0]
def SW(a): return struct.unpack(">h", rom[a:a + 2])[0]
def L(a): return struct.unpack(">I", rom[a:a + 4])[0]


TABLE = 0x096B9C  # JumpTable_096B9C[8]
END = 0x097730


def main():
    ap = argparse.ArgumentParser()
    ap.add_argument("-o", "--output"); ap.add_argument("--registry", action="store_true")
    a = ap.parse_args()
    ptrs = [L(TABLE + 4 * i) for i in range(8)] + [END]
    lines = ["        .text"]
    reg = []
    for i in range(8):
        s, e = ptrs[i], ptrs[i + 1]
        name = f"AttractSprites_List{i}_{s:06X}"
        n = 0; p = s
        while W(p) != 0xFFFF:
            p += 20; n += 1
        list_end = p + 2
        assert list_end <= e, (hex(p), hex(e))
        reg.append((name, s, list_end - s))
        lines += ["", f"        .globl  {name}",
                  f'        .section .text.{name}, "ax", @progbits',
                  f"{name}:" + " " * max(1, 44 - len(name) - 1) + f"| lista attract {i}: {n} sprites x 20 B + $FFFF ({list_end-s} B)"]
        p = s
        for _ in range(n):
            words = ",".join(f"0x{W(p+k):04x}" for k in range(0, 20, 2))
            lines.append(f"        .dc.w   {words}")
            lines.append(f"                | ${p:06X} flags=${W(p):04X} pos=({SW(p+2)},{SW(p+4)}) tmpl=${L(p+6):06X} params={rom[p+10:p+20].hex()}")
            p += 20
        lines.append(f"        .dc.w   0xffff{'':<30}| ${p:06X} fin de lista")
    # cola huerfana: comparador CCR identico al de $0967A4 (sin referencias)
    tail = ptrs[8] - 16
    name = f"ChildRank_CmpByte10_{tail:06X}"
    reg.append((name, tail, 16))
    lines += ["", f"        .globl  {name}",
              f'        .section .text.{name}, "ax", @progbits',
              f"{name}:" + " " * (44 - len(name) - 1) + "| CCR: C=1 si child.rank(+$10) > ent.rank(+$10). Sin callers (huerfana)",
              "        movea.l 0x8(a6), a1                    | +00  a1 = entidad hija (+$08)",
              "        move.b  0x10(a6), d0                   | +04  d0 = rango propio",
              "        cmp.b   0x10(a1), d0                   | +08  vs rango de la hija",
              f"        bcs.w   SetXN_{tail+0x16:06x}{'':<19}| +0c  menor -> C=1 (isla SetXN)",
              f"                                               | +10  cae en ClearXN_{tail+0x10:06x}"]
    text = "\n".join(lines) + "\n"
    if a.output:
        open(a.output, "w").write(text)
    if a.registry:
        for name, addr, size in reg:
            print(f'    ("{name}",{"":<{max(1, 45-len(name))}}0x{addr:06X}, {size:5d}, "{SRC_NAME}"),')
    print(f"{len(reg)} entradas, {sum(s for _,_,s in reg)} B", file=sys.stderr)


if __name__ == "__main__":
    main()
