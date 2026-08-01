"""
diff_settings.py — configuracion de asm-differ para Metal Slug 1
================================================================
Consumido por decomp/tools/asm_differ/diff.py.

Uso tipico:
    python3 decomp/tools/asm_differ/diff.py -mwo Entity_HasLinkedSlots

Flags:  m = objdump con simbolos, w = watch (rebuild al guardar), o = objdump
"""
import os

def apply(config, args):
    root = os.path.dirname(os.path.abspath(__file__))
    config["arch"] = "m68k"
    config["baseimg"] = os.path.join(root, "decomp", "build", "mslug_prom.bin")
    config["myimg"]   = os.path.join(root, "decomp", "build", "mslug_built.bin")
    config["mapfile"] = os.path.join(root, "decomp", "build", "mslug.map")
    config["source_directories"] = [
        os.path.join(root, "decomp", "src"),
        os.path.join(root, "decomp", "asm"),
    ]
    config["objdump_executable"] = "m68k-linux-gnu-objdump"
    config["make_command"] = ["python3", "decomp/tools/match_batch.py"]
    # base y my son ambos el mismo binario procesado con secciones
    # extraidas por objcopy; el diff se hace sobre el ELF intermedio.
    config["expected_dir"] = os.path.join(root, "decomp", "build", "expected")
    config["map_format"] = "gnu"
    config["mw_build_dir"] = os.path.join(root, "decomp", "build")
    config["build_command"] = ["python3", "decomp/tools/match_batch.py"]
