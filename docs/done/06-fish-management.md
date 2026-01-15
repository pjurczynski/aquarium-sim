# M7: Player Input - Fish Management - COMPLETED

## Overview
Implemented fish selection, adding, and removing. Players can now manage their aquarium population
in full sandbox mode.

## Tasks Completed

### M7-T1: Implement fish selection
- Changed selected_fish_id from Option<Uuid> to selected_fish_index: Option<usize>
- Added Tab key handling to cycle through fish
- Selection wraps around (modulo fish count)
- Starts at None, Tab selects first fish, then cycles forward
- Converts index to ID for rendering

### M7-T2: Show selected fish stats
Already implemented in M3, just needed selection wiring:
- Footer displays species, hunger, health, age
- Shows when selected_fish_id is Some

### M7-T3: Handle 'A' key for add fish
- Added 'A'/'a' key handling
- Cycles through species list deterministically
- Species selected based on current fish count modulo 5
- Spawns at calculated position (avoids stacking)
- Position formula: x = 10 + (count * 3) % 60, y = 5 + (count % 10)

### M7-T4: Handle 'R' key for remove fish
- Added 'R'/'r' key handling
- Removes currently selected fish if any
- Clears selection after removal (avoids dangling reference)
- Fish disappears immediately

## Implementation Details

### Selection Tracking
```rust
let mut selected_fish_index: Option<usize> = None;

// In event loop, convert index to ID
let selected_fish_id = selected_fish_index
    .and_then(|idx| tank.fish.get(idx))
    .map(|f| f.id);
```

### Key Handling
```rust
KeyCode::Tab => {
    if !tank.fish.is_empty() {
        selected_fish_index = Some(match selected_fish_index {
            None => 0,
            Some(idx) => (idx + 1) % tank.fish.len(),
        });
    }
}

KeyCode::Char('a') | KeyCode::Char('A') => {
    let species_names = vec!["Goldfish", "Guppy", "Betta", "Tetra", "Angelfish"];
    let species = species_names[tank.fish.len() % species_names.len()];
    let position = (10 + (tank.fish.len() as u16 * 3) % 60, 5 + (tank.fish.len() as u16 % 10));
    tank.add_fish(fish::Fish::new(species.to_string(), position));
}

KeyCode::Char('r') | KeyCode::Char('R') => {
    if let Some(fish_id) = selected_fish_id {
        tank.remove_fish(fish_id);
        selected_fish_index = None;
    }
}
```

## Key Decisions
- Index-based selection instead of direct ID (simpler cycling logic)
- Deterministic species rotation (predictable, no randomness needed)
- Calculated spawn positions (avoid overlapping)
- Clear selection on remove (prevent invalid state)
- No species picker modal in Tier 1 (can add in M11)

## Files Changed
- Modified: src/main.rs (selection tracking, Tab/A/R key handling)

## Observable Behavior
Full fish management now available:
1. Press Tab to select fish (cycles through all)
2. Footer shows selected fish stats
3. Press 'A' to add fish (cycles species: Goldfish, Guppy, Betta, Tetra, Angelfish)
4. Press 'R' to remove selected fish
5. Complete sandbox mode working

## Next Steps
M8: Player Input - Speed Control - adjust simulation speed with +/-
