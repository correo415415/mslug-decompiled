#!/usr/bin/env python3
"""Metal Slug 1 — Wave L: JmpToScheduler (jmp $000518; rts, 8 B)"""
import os, importlib.util

ROOT = os.path.dirname(os.path.dirname(os.path.dirname(os.path.abspath(__file__))))
PROM = os.path.join(ROOT, "decomp", "build", "mslug_prom.bin")
OUT  = os.path.join(ROOT, "decomp", "src", "jmp_to_scheduler.c")
REG  = os.path.join(ROOT, "decomp", "tools", "registry.py")

def load_covered():
    spec = importlib.util.spec_from_file_location("reg", REG)
    m = importlib.util.module_from_spec(spec); spec.loader.exec_module(m)
    cov = set()
    for n,a,s,src in m.REGISTRY: cov.update(range(a,a+s))
    return cov

HEADER = '''/*
 * Metal Slug 1 — Familia JmpToScheduler (48 thunks tail-call a $000518)
 * Auto-generado por gen_jmp_scheduler.py.
 */
#include "mslug.h"

extern void FUN_00000518(void);

'''

def main():
    with open(PROM,'rb') as f: prom=f.read()
    cov = load_covered()
    target = bytes.fromhex("4ef9000005184e75")
    hits = []
    i = 0
    while True:
        idx = prom.find(target, i)
        if idx < 0: break
        if idx%2==0 and idx not in cov:
            if not any(k in cov for k in range(idx,idx+8)):
                if idx==0 or (idx-2) in cov or prom[idx-2:idx]==b'\x4E\x75':
                    hits.append(idx)
        i = idx+1
    print(f"JmpToScheduler: {len(hits)} sitios")
    with open(OUT,'w') as f:
        f.write(HEADER)
        for a in hits:
            sym=f"JmpToScheduler_{a:06x}"
            f.write(f'__attribute__((section(".text.{sym}"), noreturn))\n')
            f.write(f'void {sym}(void) {{\n')
            f.write(f'    __asm__ volatile("jmp FUN_00000518 \\n rts" ::: "memory");\n')
            f.write(f'    __builtin_unreachable();\n')
            f.write(f'}}\n\n')
    print(f"Escrito {OUT} ({len(hits)} funciones)")
    print("\n# Registry:")
    for a in hits:
        print(f'    ("JmpToScheduler_{a:06x}", 0x{a:06X},  8, "jmp_to_scheduler.c"),')

if __name__=="__main__": main()
