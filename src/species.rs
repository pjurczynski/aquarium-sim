use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct Species {
    pub name: String,
    pub sprite_left: String,
    pub sprite_right: String,
    pub hunger_rate: f32,
    pub base_health: u8,
    pub lifespan: u64,
    pub breeding_cooldown: u32,
}

impl Species {
    pub fn new(
        name: String,
        sprite_left: String,
        sprite_right: String,
        hunger_rate: f32,
        base_health: u8,
        lifespan: u64,
        breeding_cooldown: u32,
    ) -> Self {
        Species {
            name,
            sprite_left,
            sprite_right,
            hunger_rate,
            base_health,
            lifespan,
            breeding_cooldown,
        }
    }
}

pub fn get_all_species() -> Vec<Species> {
    vec![
        Species::new(
            "Goldfish".to_string(),
            "<><".to_string(),
            "><>".to_string(),
            0.5,
            100,
            10000,
            500,
        ),
        Species::new(
            "Guppy".to_string(),
            "<o".to_string(),
            "o>".to_string(),
            2.0,
            60,
            3000,
            200,
        ),
        Species::new(
            "Betta".to_string(),
            "<)))<".to_string(),
            ">((()>".to_string(),
            1.0,
            80,
            6000,
            400,
        ),
        Species::new(
            "Tetra".to_string(),
            "<*".to_string(),
            "*>".to_string(),
            1.5,
            50,
            5000,
            300,
        ),
        Species::new(
            "Angelfish".to_string(),
            "<^>".to_string(),
            "<^>".to_string(),
            0.8,
            90,
            8000,
            450,
        ),
    ]
}

pub fn get_species(name: &str) -> Option<Species> {
    get_all_species().into_iter().find(|s| s.name == name)
}
