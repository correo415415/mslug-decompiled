//! Nombres de las rutinas originales (etiquetas de la decompilación,
//! `assets/map/labels.tsv`), para informes: dónde se paró el modo sólo
//! manual, qué rutinas faltan por portar...

use std::sync::OnceLock;

static TSV: &str = include_str!("../../../assets/map/labels.tsv");

fn table() -> &'static Vec<(u32, &'static str)> {
    static T: OnceLock<Vec<(u32, &'static str)>> = OnceLock::new();
    T.get_or_init(|| {
        let mut v: Vec<(u32, &str)> = TSV
            .lines()
            .filter(|l| !l.starts_with('#'))
            .filter_map(|l| {
                let (a, n) = l.split_once('\t')?;
                Some((u32::from_str_radix(a.trim(), 16).ok()?, n.trim()))
            })
            .collect();
        v.sort_by_key(|&(a, _)| a);
        v
    })
}

/// Etiqueta que contiene `addr` (la última que empieza en o antes de ella):
/// `(dirección de la etiqueta, nombre)`.
pub fn containing(addr: u32) -> Option<(u32, &'static str)> {
    let t = table();
    let i = t.partition_point(|&(a, _)| a <= addr);
    (i > 0).then(|| t[i - 1])
}

/// `Nombre+0xNN` (o la dirección en hex si no hay etiqueta).
pub fn describe(addr: u32) -> String {
    match containing(addr) {
        Some((a, n)) if a == addr => n.to_string(),
        Some((a, n)) => format!("{n}+0x{:X}", addr - a),
        None => format!("${addr:06X}"),
    }
}

/// Nombre sin el sufijo `_XXXXXX` de dirección.
pub fn short(name: &str) -> &str {
    match name.rsplit_once('_') {
        Some((n, hex)) if hex.len() == 6 && hex.chars().all(|c| c.is_ascii_hexdigit()) => n,
        _ => name,
    }
}
