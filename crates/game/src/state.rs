//! Estado global del juego.

use crate::entity::{Entity, PlayerId};

/// Fachada del juego: da acceso con nombre al estado global.
pub struct Game;

impl Game {
    pub fn player(p: PlayerId) -> Entity {
        Entity::player(p)
    }
}
