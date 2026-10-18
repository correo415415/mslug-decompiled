#!/usr/bin/env python3
"""
patch_menu.py — menú interactivo de scripts/apply_patch.sh.

Se abre al ejecutar `bash scripts/apply_patch.sh` sin parámetros. Lista los
parches (leídos de la cabecera "Parches disponibles" del propio .sh), deja
marcar uno o varios, pide los valores de los que llevan parámetro (=N, =HHHH,
=jp|intl) y escribe en el fichero de salida la línea de argumentos que el .sh
ejecutará a continuación.

Controles:
  números o rangos ("1 4 7", "2-5")  marcar / desmarcar
  z RUTA                             zip base (por defecto internal/mslug.zip si existe)
  a                                  aplicar los marcados
  u                                  deshacer parches (--undo)
  q                                  salir sin hacer nada
"""
import os
import re
import shlex
import sys

# Parches con parámetro: (pregunta, valor por defecto, regex de validación)
PARAMS = {
    "lives": ("vidas iniciales (1..99)", "5", r"^([1-9]|[1-9][0-9])$"),
    "bombs": ("bombas iniciales (1..99)", "30", r"^([1-9]|[1-9][0-9])$"),
    "pistol-ammo": ("munición de pistola (1..99)", "30", r"^([1-9]|[1-9][0-9])$"),
    "debug": ("flags de depuración en hex (4 dígitos)", "0001", r"^[0-9A-Fa-f]{4}$"),
    "mission": ("misión inicial (1..6)", "2", r"^[1-6]$"),
    "ending": ("final: jp | intl", "jp", r"^(jp|intl)$"),
}


def parse_list(sh):
    """[[nombre, parámetro|None, descripción]] desde la cabecera del .sh."""
    out = []
    on = False
    for ln in open(sh, encoding="utf-8"):
        if ln.startswith("# Parches disponibles"):
            on = True
            continue
        if on and ln.startswith("set -euo"):
            break
        if not on:
            continue
        m = re.match(r"^#   (\S+)\s+(.*)$", ln.rstrip("\n"))
        if m:
            name, desc = m.group(1), m.group(2)
            par = None
            if "=" in name:
                n2, p2 = name.split("=", 1)
                # =N, =HHHH, =jp|intl -> parámetro; boot=soundtest -> nombre fijo
                if p2.isupper() or "|" in p2:
                    name, par = n2, p2
            out.append([name, par, desc])
        elif out and ln.startswith("#      "):
            out[-1][2] += " " + ln[1:].strip()
    return out


def ask(prompt, default, pattern):
    while True:
        try:
            v = input("    %s [%s]: " % (prompt, default)).strip() or default
        except EOFError:
            sys.exit(1)
        if re.match(pattern, v):
            return v
        print("    valor no válido")


def main():
    sh, out = sys.argv[1], sys.argv[2]
    patches = parse_list(sh)
    sel = {}  # índice -> argumento final ("lives=5", "invincible"...)
    zipf = "internal/mslug.zip" if os.path.exists("internal/mslug.zip") else ""
    if not sys.stdin.isatty():
        print("[patch] sin terminal: usa bash scripts/apply_patch.sh <parche>... (--list)")
        open(out, "w").write("")
        return
    while True:
        print("\n=== Parches de Metal Slug (apply_patch.sh) ===")
        for i, (n, par, desc) in enumerate(patches, 1):
            mark = "x" if i in sel else " "
            shown = sel.get(i, n + ("=" + par if par else ""))
            short = desc[:58] + ("…" if len(desc) > 58 else "")
            print(" [%s] %2d  %-16s %s" % (mark, i, shown, short))
        print("\n zip base: %s" % (zipf or "(ninguno: sólo build/repack/201-p1.bin)"))
        print(" números/rangos = marcar · z RUTA = zip · a = aplicar · u = deshacer · q = salir")
        try:
            cmd = input("> ").strip()
        except EOFError:
            cmd = "q"
        if cmd in ("q", "quit", "salir"):
            open(out, "w").write("")
            return
        if cmd == "u":
            open(out, "w").write("--undo")
            return
        if cmd.startswith("z"):
            zipf = cmd[1:].strip()
            continue
        if cmd == "a":
            if not sel:
                print("  no hay parches marcados")
                continue
            args = [sel[i] for i in sorted(sel)]
            if zipf:
                args += ["--zip", zipf]
            line = " ".join(shlex.quote(a) for a in args)
            print("\n  -> bash scripts/apply_patch.sh %s" % line)
            open(out, "w").write(line)
            return
        nums = []
        for tok in cmd.replace(",", " ").split():
            m = re.match(r"^(\d+)(?:-(\d+))?$", tok)
            if not m:
                print("  no entiendo '%s'" % tok)
                nums = []
                break
            a = int(m.group(1))
            b = int(m.group(2) or a)
            nums += list(range(a, b + 1))
        for i in nums:
            if not 1 <= i <= len(patches):
                print("  %d fuera de rango" % i)
                continue
            if i in sel:
                del sel[i]
                continue
            n, par, desc = patches[i - 1]
            if par is not None:
                p, d, pat = PARAMS.get(n, ("valor", par, r".+"))
                print("  %s:" % n)
                sel[i] = "%s=%s" % (n, ask(p, d, pat))
            else:
                sel[i] = n


if __name__ == "__main__":
    main()
