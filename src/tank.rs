use crate::fish::{Direction, Fish};
use crate::species;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct Tank {
    pub fish: Vec<Fish>,
    pub simulation_speed: f32,
    pub tick_count: u64,
    pub dimensions: (u16, u16),
}

impl Tank {
    pub fn new(width: u16, height: u16) -> Self {
        Tank {
            fish: Vec::new(),
            simulation_speed: 1.0,
            tick_count: 0,
            dimensions: (width, height),
        }
    }

    pub fn add_fish(&mut self, fish: Fish) {
        self.fish.push(fish);
    }

    pub fn remove_fish(&mut self, id: Uuid) {
        self.fish.retain(|f| f.id != id);
    }

    pub fn get_fish(&self, id: Uuid) -> Option<&Fish> {
        self.fish.iter().find(|f| f.id == id)
    }

    pub fn feed(&mut self) {
        for fish in &mut self.fish {
            fish.hunger = 0;
        }
    }

    pub fn increase_speed(&mut self) {
        const SPEEDS: [f32; 5] = [0.5, 1.0, 2.0, 5.0, 10.0];
        if let Some(idx) = SPEEDS.iter().position(|&s| (s - self.simulation_speed).abs() < 0.01) {
            if idx < SPEEDS.len() - 1 {
                self.simulation_speed = SPEEDS[idx + 1];
            }
        }
    }

    pub fn decrease_speed(&mut self) {
        const SPEEDS: [f32; 5] = [0.5, 1.0, 2.0, 5.0, 10.0];
        if let Some(idx) = SPEEDS.iter().position(|&s| (s - self.simulation_speed).abs() < 0.01) {
            if idx > 0 {
                self.simulation_speed = SPEEDS[idx - 1];
            }
        }
    }

    pub fn tick(&mut self) {
        self.tick_count += 1;

        // Update each fish
        for fish in &mut self.fish {
            // Horizontal movement
            match fish.direction {
                Direction::Left => {
                    if fish.position.0 > 0 {
                        fish.position.0 = fish.position.0.saturating_sub(1);
                    } else {
                        fish.direction = Direction::Right;
                    }
                }
                Direction::Right => {
                    // Leave some margin for fish sprite width (approximate 5 chars)
                    if fish.position.0 < self.dimensions.0.saturating_sub(8) {
                        fish.position.0 = fish.position.0.saturating_add(1);
                    } else {
                        fish.direction = Direction::Left;
                    }
                }
            }

            // Vertical drift (30% chance to drift up or down)
            if self.tick_count % 3 == 0 {
                let drift = (self.tick_count % 7) as i32 - 3;
                if drift < 0 && fish.position.1 > 0 {
                    fish.position.1 = fish.position.1.saturating_sub(1);
                } else if drift > 0 && fish.position.1 < self.dimensions.1.saturating_sub(5) {
                    fish.position.1 = fish.position.1.saturating_add(1);
                }
            }

            // Hunger increase
            if let Some(species_data) = species::get_species(&fish.species) {
                let hunger_increase = species_data.hunger_rate * 0.1;
                fish.hunger = ((fish.hunger as f32 + hunger_increase).min(100.0)) as u8;
            }

            // Health drain if starving
            if fish.hunger >= 100 {
                fish.health = fish.health.saturating_sub(2);
            }

            // Age increment
            fish.age += 1;

            // Breeding cooldown decrement
            if fish.breeding_cooldown > 0 {
                fish.breeding_cooldown -= 1;
            }
        }

        // Check for breeding pairs
        let mut new_fish = Vec::new();
        let mut bred_indices = std::collections::HashSet::new();

        for i in 0..self.fish.len() {
            if bred_indices.contains(&i) {
                continue;
            }

            for j in (i + 1)..self.fish.len() {
                if bred_indices.contains(&j) {
                    continue;
                }

                let fish_a = &self.fish[i];
                let fish_b = &self.fish[j];

                // Check breeding conditions
                if fish_a.species == fish_b.species
                    && fish_a.health > 50
                    && fish_b.health > 50
                    && fish_a.hunger < 50
                    && fish_b.hunger < 50
                    && fish_a.breeding_cooldown == 0
                    && fish_b.breeding_cooldown == 0
                {
                    // Spawn offspring near first parent
                    let position = (
                        fish_a.position.0.saturating_add(5).min(self.dimensions.0.saturating_sub(10)),
                        fish_a.position.1.saturating_add(2).min(self.dimensions.1.saturating_sub(5)),
                    );
                    let baby = Fish::new(fish_a.species.clone(), position);
                    new_fish.push(baby);

                    // Mark for cooldown application
                    bred_indices.insert(i);
                    bred_indices.insert(j);
                    break; // Each fish can only breed once per tick
                }
            }
        }

        // Apply cooldowns to bred fish
        for &idx in &bred_indices {
            if let Some(fish) = self.fish.get_mut(idx) {
                if let Some(species_data) = species::get_species(&fish.species) {
                    fish.breeding_cooldown = species_data.breeding_cooldown;
                }
            }
        }

        // Add new fish to tank
        self.fish.extend(new_fish);

        // Remove dead fish (health = 0 or age >= lifespan)
        let all_species = species::get_all_species();
        self.fish.retain(|fish| {
            let species_data = all_species.iter().find(|s| s.name == fish.species);
            let alive_by_health = fish.health > 0;
            let alive_by_age = species_data.map_or(true, |s| fish.age < s.lifespan);
            alive_by_health && alive_by_age
        });
    }
}
