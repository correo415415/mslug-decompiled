//! # Metal Slug — el juego en Rust legible
//!
//! Reescritura **a mano** a partir de la decompilación (rama principal del
//! repositorio): cada función se escribe entendiendo la rutina original, con
//! nombre propio y tipos de Rust, **sin** recompilar instrucciones, sin
//! registros del 68000 y sin direcciones de la ROM en la lógica.
//!
//! Organización por sistemas:
//!
//! * [`player`] — el jugador (armas, munición, bombas, invulnerabilidad, ...)
//! * [`state`] / [`entity`] — estado global y entidades con nombre
//!
//! Mientras el juego convive con la ROM original (migración por sistemas),
//! el estado se lee/escribe a través del trait [`mem::Memory`]; los offsets
//! de cada campo están **sólo** en [`layout`]. Cuando todo esté portado,
//! `layout` se sustituirá por structs nativos.
//!
//! Plan y criterios: `docs/PORT_MANUAL.md`.

pub mod entity;
pub mod layout;
pub mod mem;
pub mod player;
pub mod state;

pub use entity::{Entity, PlayerId};
pub use mem::Memory;
pub use state::Game;
