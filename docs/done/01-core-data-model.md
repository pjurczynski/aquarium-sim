# M2: Core Data Model - COMPLETED

## Overview
Implemented all foundational data structures for the aquarium simulation.

## Tasks Completed

### M2-T1: Define Species struct
- Created `src/species.rs` with Species struct
- Fields: name, sprite_left, sprite_right, hunger_rate, base_health, lifespan, breeding_cooldown
- Added Serialize, Deserialize, Clone, Debug derives
- Implemented constructor method

### M2-T2: Define Fish struct
- Created `src/fish.rs` with Fish struct and Direction enum
- Fields: id (UUID), species, position, direction, hunger, health, age, breeding_cooldown
- Direction enum: Left, Right with PartialEq
- Constructor creates fish with UUID, default stats (hunger=0, health=100)

### M2-T3: Define Tank struct
- Created `src/tank.rs` with Tank struct
- Fields: fish (Vec<Fish>), simulation_speed, tick_count, dimensions
- Methods: new(), add_fish(), remove_fish(), get_fish()
- Added module to main.rs

### M2-T4: Create initial species roster
- Implemented `get_all_species()` returning Vec<Species>
- Added 5 starter species with mechanical variety:
  - Goldfish: slow hunger (0.5), high health (100), long life (10000)
  - Guppy: fast hunger (2.0), low health (60), short life (3000)
  - Betta: medium hunger (1.0), medium health (80), medium life (6000)
  - Tetra: medium hunger (1.5), low health (50), medium life (5000)
  - Angelfish: slow hunger (0.8), high health (90), long life (8000)
- Implemented `get_species(name)` for species lookup

## Key Decisions
- Used String for species reference in Fish (simpler than lifetimes)
- Position as (u16, u16) tuples for ratatui compatibility
- Stats as u8 for 0-100 percentages (memory efficient)
- UUID v4 for fish IDs (unique, serializable)
- Species roster provides mechanical variety (hunger rates, health, lifespans differ)

## Files Changed
- Created: src/tank.rs
- Modified: src/species.rs (added roster functions)
- Modified: src/main.rs (added tank module)

## Next Steps
M3: Basic Rendering - render tank, fish, header, footer with colors
