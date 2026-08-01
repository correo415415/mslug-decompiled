#!/usr/bin/env python3
"""Dumpea disassembly + hex bytes de N candidatos hasta rts/rte/jmp incondicional.

Uso: python3 tools/inspect_candidates.py 0x028D8E 0x043fac 0x05da56 0x0004ae 0x077c7e 0x00236e 0x027cee
"""
import os, sys, importlib.util
from capstone import Cs, CS_ARCH_M68K, CS_MODE_M68K_000
HERE = os.path.dirname(os.path.abspath(__file__))
ROOT = os.path.abspath(os.path.join(HERE, ".."))
PROM = os.path.join(ROOT, "build", "mslug_prom.bin")

def load_reg():
    spec = importlib.util.spec_from_file_location("registry", os.path.join(HERE,"registry.py"))
    m = importlib.util.module_from_spec(spec); spec.loader.exec_module(m); return m.REGISTRY

def matched_starts(reg):
    return {off: (name, size) for name, off, size, _ in reg}

def next_matched(reg, start):
    s = sorted(off for name,off,size,_ in reg if off > start)
    return s[0] if s else 0x200000

def disasm(rom, start, hard_end):
    md = Cs(CS_ARCH_M68K, CS_MODE_M68K_000); md.detail = True
    out = []; off = start
    while off < hard_end and off - start < 512:
        chunk = bytes(rom[off:off+16])
        gen = md.disasm(chunk, off)
        try: ins = next(gen)
        except StopIteration: break
        out.append(ins)
        mn = ins.mnemonic
        off += ins.size
        if mn in ("rts","rte","rtr"): break
        if mn == "jmp" or mn == "bra": break
        if ins.size == 0: break
    return out, off

def main():
    rom = open(PROM,"rb").read()
    reg = load_reg()
    for arg in sys.argv[1:]:
        start = int(arg, 16)
        hard_end = next_matched(reg, start)
        insts, end = disasm(rom, start, hard_end)
        size = end - start
        print(f"\n===== ${start:06X}  next_matched=${hard_end:06X}  candidate_size={size}B =====")
        # hex block
        hexb = rom[start:end].hex()
        # 16 bytes/line
        for i in range(0, len(hexb), 32):
            addr = start + i//2
            print(f"  {addr:06x}: {hexb[i:i+32]}")
        print(f"---- disasm ({len(insts)} insts) ----")
        for ins in insts:
            print(f"  {ins.address:06x}:  {ins.bytes.hex():<12s} {ins.mnemonic:<8s} {ins.op_str}")

if __name__ == "__main__":
    main()
