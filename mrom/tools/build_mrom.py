#!/usr/bin/env python3
"""
build_mrom.py - assemble the Z80 sound driver and compare it with rom/201-m1.bin.

Every region listed in mrom/tools/mrom_regions.py is assembled with z80asm
from mrom/asm/<region>.s (a top-level file that `include`s the pieces of that
region, in address order).  The resulting binaries are concatenated into the
128 KiB image build/mrom/201-m1_rebuilt.bin and compared byte-by-byte with
rom/201-m1.bin.

    python3 mrom/tools/build_mrom.py            # build + compare
    python3 mrom/tools/build_mrom.py --diff     # also list first differing bytes
"""
import hashlib, os, subprocess, sys

ROOT = os.path.dirname(os.path.dirname(os.path.dirname(os.path.abspath(__file__))))
sys.path.insert(0, os.path.join(ROOT, "mrom", "tools"))
from mrom_regions import REGIONS, EXPECT_MD5  # noqa: E402

ROM = os.path.join(ROOT, "rom", "201-m1.bin")
OUT = os.path.join(ROOT, "build", "mrom")
ASM = os.path.join(ROOT, "mrom", "asm")


def assemble(region):
    name, off, size, org = region
    src = os.path.join(ASM, name + ".s")
    if not os.path.exists(src):
        return None, f"missing {src}"
    obj = os.path.join(OUT, name + ".bin")
    r = subprocess.run(["z80asm", "-I", ASM, "-I", os.path.join(ROOT, "mrom", "include"),
                        "-o", obj, src], capture_output=True, text=True)
    if r.returncode != 0:
        return None, r.stderr.strip() or r.stdout.strip()
    blob = open(obj, "rb").read()
    # z80asm emits from the first org; pad/truncate to region size
    if len(blob) > size:
        return None, f"{name}: assembled {len(blob):#x} B > region size {size:#x}"
    blob += b"\xff" * (size - len(blob))
    return blob, None


def main():
    show_diff = "--diff" in sys.argv
    os.makedirs(OUT, exist_ok=True)
    orig = open(ROM, "rb").read() if os.path.exists(ROM) else None
    image = bytearray(b"\xff" * 0x20000)
    ok = True
    for reg in REGIONS:
        name, off, size, org = reg
        blob, err = assemble(reg)
        if err:
            print(f"[build_mrom] ✘ {name}: {err}")
            ok = False
            continue
        image[off:off + size] = blob
        if orig is not None:
            same = orig[off:off + size] == blob
            ndiff = sum(1 for a, b in zip(orig[off:off + size], blob) if a != b)
            print(f"[build_mrom] {'✔' if same else '✘'} {name:12s} {size:#07x} B  "
                  f"{'idéntico' if same else f'{ndiff} bytes distintos'}")
            if not same:
                ok = False
                if show_diff:
                    shown = 0
                    for i, (a, b) in enumerate(zip(orig[off:off + size], blob)):
                        if a != b:
                            print(f"    z80 {org + i:#06x} (file {off + i:#07x}): {a:02x} -> {b:02x}")
                            shown += 1
                            if shown >= 16:
                                break
    out = os.path.join(OUT, "201-m1_rebuilt.bin")
    open(out, "wb").write(image)
    md5 = hashlib.md5(image).hexdigest()
    print(f"[build_mrom] escrito {out}  ({len(image):,} B)  MD5 = {md5}")
    if md5 == EXPECT_MD5:
        print("[build_mrom] ✔ IDÉNTICA a 201-m1.bin")
    else:
        print(f"[build_mrom] ✘ difiere (esperado {EXPECT_MD5})")
        ok = False
    sys.exit(0 if ok else 1)


if __name__ == "__main__":
    main()
