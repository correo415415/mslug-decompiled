#!/usr/bin/env python3
"""Metal Slug 1 — Wave J: JsrPcThunk (jsr pc+d,pc ; rts, 6 B)"""
import os, importlib.util

ROOT = os.path.dirname(os.path.dirname(os.path.dirname(os.path.abspath(__file__))))
PROM = os.path.join(ROOT, "decomp", "build", "mslug_prom.bin")
OUT  = os.path.join(ROOT, "decomp", "src", "jsr_pc_thunks.c")
REG  = os.path.join(ROOT, "decomp", "tools", "registry.py")
SYM  = os.path.join(ROOT, "decomp", "tools", "symbols.py")

def load_cov_names():
    spec = importlib.util.spec_from_file_location("reg", REG)
    m = importlib.util.module_from_spec(spec); spec.loader.exec_module(m)
    cov = set(); name_at = {}
    for n,a,s,src in m.REGISTRY: cov.update(range(a,a+s)); name_at[a]=n
    spec2 = importlib.util.spec_from_file_location("sym", SYM)
    ms = importlib.util.module_from_spec(spec2); spec2.loader.exec_module(ms)
    for a,nm in ms.SYMBOLS.items():
        if a not in name_at: name_at[a]=nm
    return cov, name_at

HEADER = '''/*
 * Metal Slug 1 — Familia JsrPcThunk (trampolines PC-relativos, 6 B)
 * Auto-generado por gen_jsr_pc_thunks.py. Requiere -mpcrel.
 */
#include "mslug.h"

'''

def main():
    with open(PROM,'rb') as f: prom=f.read()
    cov, name_at = load_cov_names()
    hits = []
    for i in range(0, len(prom)-6, 2):
        if prom[i:i+2]!=b'\x4E\xBA': continue
        if prom[i+4:i+6]!=b'\x4E\x75': continue
        if any(k in cov for k in range(i,i+6)): continue
        disp = int.from_bytes(prom[i+2:i+4],'big',signed=True)
        tgt = (i+2+disp)&0xFFFFFFFF
        hits.append((i,tgt))
    print(f"JsrPcThunk: {len(hits)} sitios")
    new_targets = sorted({t for _,t in hits if t not in name_at})
    with open(OUT,'w') as f:
        f.write(HEADER)
        for a,t in hits:
            sym=f"JsrPcThunk_{a:06x}"; nm=name_at.get(t,f"PcThunkTarget_{t:06x}")
            f.write(f'__attribute__((section(".text.{sym}")))\n')
            f.write(f'void {sym}(void) {{\n')
            f.write(f'    extern void {nm}(void);\n')
            f.write(f'    __asm__ volatile("jsr {nm}(%%pc)" ::: "memory","cc","d0","d1","a0","a1");\n')
            f.write(f'}}\n\n')
    print(f"Escrito {OUT} ({len(hits)} funciones, {len(new_targets)} targets nuevos)")
    # Fragmento registry
    print("\n# Registry:")
    for a,t in hits:
        print(f'    ("JsrPcThunk_{a:06x}", 0x{a:06X},  6, "jsr_pc_thunks.c"),')
    if new_targets:
        print("\n# Symbols nuevos:")
        for t in new_targets:
            print(f'    0x{t:08X}: "PcThunkTarget_{t:06x}",')

if __name__=="__main__": main()
