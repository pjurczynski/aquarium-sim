# Aquarium Simulator - Context

## Project State: TIER 1 COMPLETE
All 11 milestones completed. Fully functional aquarium simulator ready for use.

## Completed Milestones
M1: Project Setup ✓
M2: Core Data Model ✓
M3: Basic Rendering ✓
M4: Fish Movement ✓
M5: Simulation Loop ✓
M6: Player Input - Feeding ✓
M7: Player Input - Fish Management ✓
M8: Player Input - Speed Control ✓
M9: Breeding System ✓
M10: Persistence ✓
M11: Polish & UX ✓

## Core Features
- 5 fish species with mechanical variety (different hunger rates, health, lifespans)
- Autonomous fish movement (horizontal swimming with vertical drift)
- Hunger/health simulation (fish get hungry, starve if not fed)
- Color-coded fish states (cyan = healthy, yellow = hungry, red = critical)
- Breeding system (automatic when healthy and fed)
- Death conditions (starvation or old age)
- Full player interaction (feed, add, remove, select fish)
- Speed control (0.5x to 10x)
- Pause/resume functionality
- Save/load persistence (JSON)
- Terminal resize handling
- Species picker modal
- Help overlay

## Architecture
```
src/
  main.rs     - Entry point, event loop, input handling, state management
  fish.rs     - Fish struct (id, species, position, direction, hunger, health, age, cooldown)
  species.rs  - 5 species definitions with stats
  tank.rs     - Tank management, tick() simulation, breeding, death
  ui.rs       - Complete rendering (tank, fish, modals, header, footer)
  save.rs     - JSON persistence (save/load)
```

## Species
- Goldfish: slow hunger (0.5), high health (100), long life (10000), breed cooldown 500
- Guppy: fast hunger (2.0), low health (60), short life (3000), breed cooldown 200
- Betta: medium hunger (1.0), medium health (80), medium life (6000), breed cooldown 400
- Tetra: medium-fast hunger (1.5), low health (50), medium life (5000), breed cooldown 300
- Angelfish: slow hunger (0.8), high health (90), long life (8000), breed cooldown 450

## Controls
- Q: Quit (auto-saves to tank.json)
- F: Feed all fish
- A: Add fish (opens species picker modal)
- R: Remove selected fish
- Tab: Select next fish
- Space: Pause/Resume simulation
- +/=: Increase speed
- -: Decrease speed
- ?: Toggle help overlay
- 1-5: Select species (in picker modal)
- Esc: Close modal

## Tech Stack
- Rust 2024 edition
- ratatui 0.29 (TUI framework)
- crossterm 0.28 (terminal manipulation)
- serde 1.0 + serde_json (serialization)
- uuid (unique fish IDs)

## Files to Commit
All implementation work (M2-M11) needs to be committed:
- src/species.rs (created, species roster)
- src/fish.rs (created, Fish struct)
- src/tank.rs (created, Tank with full simulation)
- src/ui.rs (created, complete rendering system)
- src/save.rs (created, persistence)
- src/main.rs (updated, event loop and state management)
- docs/backlog.json (all milestones marked completed)
- docs/tasks.md (final state)
- docs/done/*.md (10 completion documents)
- CONTEXT.md (this file)

## How to Run
```bash
cargo run
```

## Next Steps (Optional Future Enhancements)
Tier 1 is complete. Possible future improvements:
- Water quality mechanics
- Disease system
- More fish species
- Decorations (plants, rocks)
- Schooling behavior
- Better graphics (Unicode fish)
- Sound effects
- Achievements
- Economy system

For now, project is feature-complete and ready for use!
