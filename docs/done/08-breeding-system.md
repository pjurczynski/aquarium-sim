# M9: Breeding System - COMPLETED

## Overview
Implemented automatic fish breeding. When two fish of the same species are healthy and well-fed,
they produce offspring. Breeding is instant (no growth stages), with cooldown periods to prevent spam.

## Tasks Completed

### M9-T1: Check breeding conditions per tick
- Added breeding check logic in Tank.tick()
- Nested loop checks all fish pairs (O(n²))
- Conditions for breeding:
  - Same species
  - Both health > 50
  - Both hunger < 50
  - Both breeding_cooldown == 0
- Tracks bred fish to avoid multiple breeds per tick

### M9-T2: Spawn baby fish
- Creates new Fish with parent's species
- Position offset from parent (+5 x, +2 y)
- Clamps position to tank boundaries
- Baby is instant adult (hunger=0, health=100, age=0)
- Adds to tank after all pair checking

### M9-T3: Apply breeding cooldown
- Uses HashSet to track bred fish indices
- Applies species.breeding_cooldown to both parents
- Cooldown prevents immediate re-breeding
- Cooldown decrements each tick (implemented in M5)

### M9-T4: Optional: tank capacity limit
- Marked as skipped for Tier 1
- Population can grow unbounded (player responsibility)
- Can add limit in M11 if needed

## Implementation Details

### Breeding Logic
```rust
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

        // Check conditions
        if fish_a.species == fish_b.species
            && fish_a.health > 50
            && fish_b.health > 50
            && fish_a.hunger < 50
            && fish_b.hunger < 50
            && fish_a.breeding_cooldown == 0
            && fish_b.breeding_cooldown == 0
        {
            // Spawn offspring
            let position = (
                fish_a.position.0.saturating_add(5).min(self.dimensions.0 - 10),
                fish_a.position.1.saturating_add(2).min(self.dimensions.1 - 5),
            );
            let baby = Fish::new(fish_a.species.clone(), position);
            new_fish.push(baby);

            bred_indices.insert(i);
            bred_indices.insert(j);
            break; // Each fish breeds once per tick
        }
    }
}

// Apply cooldowns
for &idx in &bred_indices {
    if let Some(fish) = self.fish.get_mut(idx) {
        if let Some(species_data) = species::get_species(&fish.species) {
            fish.breeding_cooldown = species_data.breeding_cooldown;
        }
    }
}

// Add babies
self.fish.extend(new_fish);
```

## Key Decisions
- Same-species only (no cross-breeding)
- HashSet to track bred fish (prevents multiple breeds per tick)
- Position clamps to boundaries (avoid spawning outside tank)
- Instant adults (no growth stages)
- Each fish breeds once per tick maximum
- Cooldown prevents spam (species-specific durations)

## Breeding Cooldowns by Species
- Goldfish: 500 ticks (slow breeders)
- Guppy: 200 ticks (fast breeders)
- Betta: 400 ticks (medium)
- Tetra: 300 ticks (medium-fast)
- Angelfish: 450 ticks (slow-medium)

## Files Changed
- Modified: src/tank.rs (added breeding logic to tick() method)

## Observable Behavior
With M9 complete, population dynamics work:
1. Feed fish regularly (hunger < 50)
2. Keep fish healthy (health > 50)
3. Fish breed automatically when conditions met
4. Offspring spawn near parents
5. Parents get cooldown (visible in footer if selected)
6. Population grows naturally if well-cared-for

## Next Steps
M10: Persistence - save/load tank state to JSON file
