#!/usr/bin/env python3
import sys, struct
from pathlib import Path
sys.path.insert(0, str(Path(__file__).resolve().parent))
from registry import REGISTRY

prom = Path(__file__).resolve().parents[1] / 'build' / 'mslug_prom.bin'
rom = prom.read_bytes()
reg_addrs = {addr for _name, addr, _size, _src in REGISTRY}
reg_ranges = [(addr, addr+size) for _name, addr, size, _src in REGISTRY]

def in_registry(addr):
    return addr in reg_addrs

def overlaps(addr, size):
    end = addr + size
    for a,b in reg_ranges:
        if not (end <= a or addr >= b):
            return True
    return False

cands = []
for addr in range(len(rom)-8):
    if overlaps(addr, 2):
        continue
    # rts stub
    if rom[addr:addr+2] == b'\x4e\x75' and not in_registry(addr):
        cands.append((addr, 2, 'rts'))
    # moveq #imm,d0 ; rts
    if rom[addr] == 0x70 and rom[addr+2:addr+4] == b'\x4e\x75' and not overlaps(addr,4):
        cands.append((addr, 4, f'moveq #{struct.unpack("b", rom[addr+1:addr+2])[0]},d0 ; rts'))
    # clr.w d0 ; rts
    if rom[addr:addr+4] == b'\x42\x40\x4e\x75' and not overlaps(addr,4):
        cands.append((addr, 4, 'clr.w d0 ; rts'))
    # jsr abs.l ; rts
    if rom[addr:addr+2] == b'\x4e\xb9' and rom[addr+6:addr+8] == b'\x4e\x75' and not overlaps(addr,8):
        target = int.from_bytes(rom[addr+2:addr+6], 'big')
        cands.append((addr, 8, f'jsr ${target:06X}.l ; rts'))
    # jmp abs.l
    if rom[addr:addr+2] == b'\x4e\xf9' and not overlaps(addr,6):
        target = int.from_bytes(rom[addr+2:addr+6], 'big')
        cands.append((addr, 6, f'jmp ${target:06X}.l'))

# de-dup exact addr/size by preferring larger pattern
best = {}
for addr,size,desc in cands:
    if addr not in best or size > best[addr][0]:
        best[addr] = (size, desc)
for addr in sorted(best):
    size, desc = best[addr]
    print(f'0x{addr:06X} {size:2d} {desc}')
