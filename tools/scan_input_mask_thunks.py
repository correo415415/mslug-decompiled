#!/usr/bin/env python3
"""
scan_input_mask_thunks.py
-------------------------
Herramienta especifica para el cluster InputMask $05CDFC..$05CFA7.

Recorre linealmente la ROM entre START y END, y para cada "entrada"
extrae (addr, size, mask, layer, a2_target?, backend), basandose en
la deteccion del patron:
    move.b  #<mask>, d1     ; 12 3c 00 XX
    [ori.b  #<orimask>, d1] ; 00 01 00 XX
    move.w  #<layer>, d0    ; 30 3c 00 XX
    [lea    $<a2_target>,a2] ; 45 f9 XX XX XX XX
    bra.w   <backend>        ; 60 00 XX XX

Uso:
    python3 tools/scan_input_mask_thunks.py
"""
import os, sys, importlib.util
from capstone import Cs, CS_ARCH_M68K, CS_MODE_M68K_000

HERE = os.path.dirname(os.path.abspath(__file__))
ROOT = os.path.abspath(os.path.join(HERE, ".."))
PROM = os.path.join(ROOT, "build", "mslug_prom.bin")

START = 0x05CDFC
END   = 0x05CFA8   # exclusive: primer byte del backend $05CFA8

def parse_imm(op_str):
    """Devuelve el primer inmediato como entero (soporta '#$XX,YY' y '#N,YY')."""
    # op_str tipico: '#$f0, d1'  o  '#$3, d0'
    seg = op_str.split(',')[0].strip()
    assert seg.startswith('#'), seg
    seg = seg[1:]
    if seg.startswith('$'):
        return int(seg[1:], 16)
    return int(seg)

def parse_addr(op_str):
    """Extrae la primera direccion absoluta '$XXXXXX' del op_str."""
    hexpart = op_str.split('$', 1)[1]
    # cortar en el primer no-hex
    end = 0
    while end < len(hexpart) and hexpart[end] in "0123456789abcdefABCDEF":
        end += 1
    return int(hexpart[:end], 16)

def main():
    rom = open(PROM, "rb").read()
    md = Cs(CS_ARCH_M68K, CS_MODE_M68K_000); md.detail = True
    entries = []
    off = START
    while off < END:
        # Cada entrada empieza con  12 3c 00 XX  = move.b #<XX>, d1
        if rom[off] != 0x12 or rom[off+1] != 0x3C:
            print(f'! byte inesperado @ ${off:06x}: {rom[off]:02x}{rom[off+1]:02x}')
            break
        entry_start = off
        instrs = []
        # decodificar hasta encontrar bra.w
        while off < END:
            chunk = bytes(rom[off:off+16])
            try: ins = next(md.disasm(chunk, off))
            except StopIteration: break
            instrs.append(ins)
            off += ins.size
            if ins.mnemonic.startswith('bra'): break
        size = off - entry_start
        mask = None; ori_val = None; layer = None; a2 = None; backend = None
        for ins in instrs:
            m = ins.mnemonic
            if m == 'move.b' and ins.op_str.startswith('#'):
                mask = parse_imm(ins.op_str)
            elif m == 'ori.b' and ins.op_str.startswith('#'):
                ori_val = parse_imm(ins.op_str)
            elif m == 'move.w' and ins.op_str.startswith('#'):
                layer = parse_imm(ins.op_str)
            elif m == 'lea.l':
                a2 = parse_addr(ins.op_str)
            elif m.startswith('bra'):
                backend = parse_addr(ins.op_str)
        effective = mask if ori_val is None else (mask | ori_val)
        entries.append({
            'addr': entry_start, 'size': size,
            'mask_lo': mask, 'ori': ori_val,
            'mask_effective': effective,
            'layer': layer, 'a2': a2, 'backend': backend,
        })

    print(f'Entries: {len(entries)}')
    print(f'{"idx":>3} {"addr":<8} {"sz":>2} {"mask":>5} {"ori":>5} {"eff":>5} {"lyr":>3} {"a2":>8} {"backend":>8}')
    for i, e in enumerate(entries):
        a2s = f'${e["a2"]:06x}' if e['a2'] else '-'
        oris = f'${e["ori"]:02x}' if e['ori'] is not None else '-'
        print(f'{i:>3} ${e["addr"]:06x} {e["size"]:>2} ${e["mask_lo"]:02x} {oris:>5} ${e["mask_effective"]:02x} {e["layer"]:>3} {a2s:>8} ${e["backend"]:06x}')
    total = sum(e['size'] for e in entries)
    print(f'\ntotal bytes: {total}  (range ${START:x}..${END:x} = {END-START} B)')
    return entries

if __name__ == '__main__':
    main()
