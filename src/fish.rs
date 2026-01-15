use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
pub enum Direction {
    Left,
    Right,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct Fish {
    pub id: Uuid,
    pub species: String,
    pub position: (u16, u16),
    pub direction: Direction,
    pub hunger: u8,
    pub health: u8,
    pub age: u64,
    pub breeding_cooldown: u32,
}

impl Fish {
    pub fn new(species: String, position: (u16, u16)) -> Self {
        Fish {
            id: Uuid::new_v4(),
            species,
            position,
            direction: Direction::Right,
            hunger: 0,
            health: 100,
            age: 0,
            breeding_cooldown: 0,
        }
    }
}
