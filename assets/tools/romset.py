"""romset.py - acceso a las ROMs del cartucho para las herramientas de assets.

Las herramientas (copiadas de la rama de la decompilación) leían las ROMs
sueltas de `rom/` y la P-ROM procesada de `build/mslug_prom.bin`. Aquí se
leen directamente del romset MAME (`internal/mslug.zip` por defecto, o el
que diga la variable `MSLUG_ROM`), sin extraer nada a disco. Si existe
`rom/<nombre>` se usa ese fichero (para probar ROMs modificadas).

`prom()` devuelve la P-ROM en el orden de la CPU (bytes intercambiados por
words y los dos bancos de 1 MiB permutados, como `scripts/setup.sh` de la
decompilación) y la cachea en `build/mslug_prom.bin`.
"""
import hashlib
import os
import zipfile

HERE = os.path.dirname(os.path.abspath(__file__))
ROOT = os.path.dirname(os.path.dirname(HERE))
ZIP = os.environ.get("MSLUG_ROM", os.path.join(ROOT, "internal", "mslug.zip"))
PROM_CACHE = os.path.join(ROOT, "build", "mslug_prom.bin")
PROM_MD5 = "816b3f74c76b3373993407615f1850fe"

_cache = {}


def rom_file(name):
    """Contenido de una ROM del romset (p. ej. "201-c1.bin")."""
    if name in _cache:
        return _cache[name]
    loose = os.path.join(ROOT, "rom", name)
    if os.path.isfile(loose):
        data = open(loose, "rb").read()
    else:
        if not os.path.isfile(ZIP):
            raise SystemExit(f"no encuentro el romset {ZIP} (usa MSLUG_ROM=ruta/mslug.zip)")
        with zipfile.ZipFile(ZIP) as z:
            data = z.read(name)
    _cache[name] = data
    return data


def prom_path():
    """Ruta de la P-ROM en orden de CPU (se genera si falta)."""
    if os.path.isfile(PROM_CACHE) and hashlib.md5(open(PROM_CACHE, "rb").read()).hexdigest() == PROM_MD5:
        return PROM_CACHE
    raw = rom_file("201-p1.bin")
    if len(raw) != 0x200000:
        raise SystemExit(f"201-p1.bin: esperaba 2 MiB, hay {len(raw)} bytes")
    sw = bytearray(len(raw))
    sw[0::2], sw[1::2] = raw[1::2], raw[0::2]
    p = bytes(sw[0x100000:] + sw[:0x100000])
    md5 = hashlib.md5(p).hexdigest()
    if md5 != PROM_MD5:
        print(f"[romset] aviso: P-ROM con MD5 {md5} (esperaba {PROM_MD5}); ¿otra versión del juego?")
    os.makedirs(os.path.dirname(PROM_CACHE), exist_ok=True)
    open(PROM_CACHE, "wb").write(p)
    return PROM_CACHE


def prom():
    return open(prom_path(), "rb").read()
