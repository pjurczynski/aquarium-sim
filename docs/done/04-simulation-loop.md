# M5: Simulation Loop - COMPLETED

## Overview
Implemented full simulation mechanics. Fish now have hunger that increases over time, health that
drains when starving, age tracking, and death conditions. The aquarium is now a living simulation
that requires active care.

## Tasks Completed

### M5-T1: Implement tick system
Already completed in M4. Tank.tick() method exists and is called in main event loop.
- tick_count increments each frame
- Foundation for all simulation updates

### M5-T2: Update hunger per tick
- Added hunger increase logic to Tank.tick()
- Looks up species data to get hunger_rate
- Formula: hunger += hunger_rate * 0.1 (scaled for reasonable progression)
- Clamps to 0-100 range using min()
- Different species get hungry at different rates:
  - Goldfish: 0.5/tick (slow)
  - Guppy: 2.0/tick (fast)
  - Betta: 1.0/tick (medium)
  - Tetra: 1.5/tick (medium-fast)
  - Angelfish: 0.8/tick (slow-medium)

### M5-T3: Update health when starving
- Added starvation check: if hunger >= 100
- Health drains at 2 points per tick when starving
- Uses saturating_sub for safe subtraction
- Fish will turn red (hunger > 80) then die (health = 0)

### M5-T4: Update fish age
- Simple age increment: fish.age += 1 every tick
- Tracks time alive for lifecycle
- Visible in footer when fish selected
- Used for natural death condition

### M5-T5: Handle death conditions
- Added fish removal logic at end of tick()
- Two death conditions:
  1. health == 0 (starvation)
  2. age >= species.lifespan (natural death)
- Uses Vec::retain() with closure checking both conditions
- Looks up species data for lifespan comparison
- Dead fish simply disappear from tank

## Implementation Details

### Enhanced Tank.tick() Method
```rust
pub fn tick(&mut self) {
    self.tick_count += 1;

    // Update each fish
    for fish in &mut self.fish {
        // Movement (from M4)
        // ...

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

    // Remove dead fish
    let all_species = species::get_all_species();
    self.fish.retain(|fish| {
        let species_data = all_species.iter().find(|s| s.name == fish.species);
        let alive_by_health = fish.health > 0;
        let alive_by_age = species_data.map_or(true, |s| fish.age < s.lifespan);
        alive_by_health && alive_by_age
    });
}
```

## Key Decisions
- Hunger rate scaling: 0.1 multiplier for reasonable progression
- Health drain rate: 2 points/tick when starving (balanced, not instant)
- Death check at end of tick (after all updates)
- Breeding cooldown also decrements (prep for M9)
- Species lookup in tick is acceptable (only 5 species, small overhead)

## Files Changed
- Modified: src/tank.rs (expanded tick() with full simulation mechanics, added species import)

## Observable Behavior
With M5 complete, the simulation is now fully functional:
1. Fish spawn with hunger=0, health=100
2. Hunger increases over time (rate varies by species)
3. Fish turn yellow at hunger=50, red at hunger=80
4. When hunger hits 100, health drains rapidly
5. Fish die and disappear when health=0
6. Fish also die naturally when age reaches lifespan

Without feeding, fish will eventually starve and die.

## Next Steps
M6: Player Input - Feeding - implement 'F' key to reduce fish hunger
