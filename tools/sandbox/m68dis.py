"""Desensambla la P-ROM volcada: m68dis.py DIRECCION_HEX [N]."""
import sys, capstone
data = open('/tmp/prom.bin', 'rb').read()
addr = int(sys.argv[1], 16); n = int(sys.argv[2]) if len(sys.argv) > 2 else 40
md = capstone.Cs(capstone.CS_ARCH_M68K, capstone.CS_MODE_BIG_ENDIAN | capstone.CS_MODE_M68K_000)
c = 0
for i in md.disasm(data[addr:addr + n * 10], addr):
    print(f"{i.address:06X}: {i.mnemonic} {i.op_str}"); c += 1
    if c >= n: break
