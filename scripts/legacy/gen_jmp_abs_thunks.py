#!/usr/bin/env python3
"""Metal Slug 1 — Wave K: JmpAbsThunk (jmp abs.l, tail-call puro, 6 B)"""
import os, importlib.util

ROOT = os.path.dirname(os.path.dirname(os.path.dirname(os.path.abspath(__file__))))
PROM = os.path.join(ROOT, "decomp", "build", "mslug_prom.bin")
OUT  = os.path.join(ROOT, "decomp", "src", "jmp_abs_thunks.c")
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
 * Metal Slug 1 — Familia JmpAbsThunk (tail-call thunks jmp abs.l, 6 B)
 * Auto-generado por gen_jmp_abs_thunks.py.
 */
#include "mslug.h"

'''

def main():
    with open(PROM,'rb') as f: prom=f.read()
    cov, name_at = load_cov_names()
    hits = []
    i = 0
    while i < len(prom)-6:
        if prom[i:i+2]==b'\x4E\x75':
            start=i+2
            while start<len(prom)-2 and prom[start:start+2]==b'\x00\x00': start+=2
            if start in cov: i=start; continue
            if prom[start:start+2]==b'\x4E\xF9':
                if not any(k in cov for k in range(start,start+6)):
                    if prom[start+6:start+8]!=b'\x4E\x75':
                        tgt=int.from_bytes(prom[start+2:start+6],'big')
                        hits.append((start,tgt))
            i=start+6
        else: i+=2
    print(f"JmpAbsThunk: {len(hits)} sitios")
    new_targets = sorted({t for _,t in hits if t not in name_at})
    with open(OUT,'w') as f:
        f.write(HEADER)
        for a,t in hits:
            sym=f"JmpAbsThunk_{a:06x}"; nm=name_at.get(t,f"JmpTarget_{t:06x}")
            f.write(f'__attribute__((section(".text.{sym}"), noreturn))\n')
            f.write(f'void {sym}(void) {{\n')
            f.write(f'    extern void {nm}(void);\n')
            f.write(f'    __asm__ volatile("jmp {nm}" ::: "memory");\n')
            f.write(f'    __builtin_unreachable();\n')
            f.write(f'}}\n\n')
    print(f"Escrito {OUT} ({len(hits)} funciones, {len(new_targets)} targets nuevos)")
    print("\n# Registry:")
    for a,t in hits:
        print(f'    ("JmpAbsThunk_{a:06x}", 0x{a:06X},  6, "jmp_abs_thunks.c"),')
    if new_targets:
        print("\n# Symbols nuevos:")
        for t in new_targets:
            print(f'    0x{t:08X}: "JmpTarget_{t:06x}",')

if __name__=="__main__": main()
