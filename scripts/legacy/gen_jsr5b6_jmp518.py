#!/usr/bin/env python3
"""Metal Slug 1 — Wave M: Jsr5B6ThenJmpScheduler (jsr $5B6; jmp $518; rts, 14 B)"""
import os, importlib.util

ROOT = os.path.dirname(os.path.dirname(os.path.dirname(os.path.abspath(__file__))))
PROM = os.path.join(ROOT, "decomp", "build", "mslug_prom.bin")
OUT  = os.path.join(ROOT, "decomp", "src", "jsr5b6_jmp518.c")
REG  = os.path.join(ROOT, "decomp", "tools", "registry.py")

def load_covered():
    spec = importlib.util.spec_from_file_location("reg", REG)
    m = importlib.util.module_from_spec(spec); spec.loader.exec_module(m)
    cov = set()
    for n,a,s,src in m.REGISTRY: cov.update(range(a,a+s))
    return cov

HEADER = '''/*
 * Metal Slug 1 — Familia Jsr5B6ThenJmpScheduler (14 B, 87 sitios)
 * Auto-generado por gen_jsr5b6_jmp518.py.
 */
#include "mslug.h"

extern void FUN_000005B6(void);
extern void FUN_00000518(void);

'''

def main():
    with open(PROM,'rb') as f: prom=f.read()
    cov = load_covered()
    target = bytes.fromhex("4eb9000005b64ef9000005184e75")
    hits = []
    i = 0
    while True:
        idx = prom.find(target, i)
        if idx < 0: break
        if idx%2==0 and idx not in cov:
            if not any(k in cov for k in range(idx,idx+14)):
                hits.append(idx)
        i = idx+1
    print(f"Jsr5B6ThenJmpScheduler: {len(hits)} sitios")
    with open(OUT,'w') as f:
        f.write(HEADER)
        for a in hits:
            sym=f"Jsr5B6ThenJmpScheduler_{a:06x}"
            f.write(f'__attribute__((section(".text.{sym}"), noreturn))\n')
            f.write(f'void {sym}(void) {{\n')
            f.write(f'    __asm__ volatile(\n')
            f.write(f'        "jsr FUN_000005B6 \\n"\n')
            f.write(f'        "jmp FUN_00000518 \\n"\n')
            f.write(f'        "rts             \\n"\n')
            f.write(f'        ::: "memory","cc","d0","d1","a0","a1");\n')
            f.write(f'    __builtin_unreachable();\n')
            f.write(f'}}\n\n')
    print(f"Escrito {OUT} ({len(hits)} funciones)")
    print("\n# Registry:")
    for a in hits:
        print(f'    ("Jsr5B6ThenJmpScheduler_{a:06x}", 0x{a:06X}, 14, "jsr5b6_jmp518.c"),')

if __name__=="__main__": main()
