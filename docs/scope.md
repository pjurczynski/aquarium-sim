# Aquarium Simulator - Scope Document

## Problem Statement

Build a terminal-based aquarium simulator that goes beyond existing decorative solutions by
implementing real virtual pet mechanics. Fish require active care (feeding), have health states
(can die from starvation), and lifecycle progression (breeding). The simulation runs at
user-configurable speed and pauses when the application is closed.

## Existing Solutions

Before building something new, here's what already exists:

### 1. Asciiquarium (Perl)
- **Repo:** https://github.com/cmatsuoka/asciiquarium
- **Pros:** Classic, lightweight, single Perl script
- **Cons:** No customization, no interactivity beyond pause, requires Curses/Perl, no Windows
- **Features:** Random fish, whales, sharks, castle, bubbles - purely decorative

### 2. Freefish (Rust)
- **Repo:** https://github.com/daviddwk/freefish
- **Pros:** Highly customizable via JSON, modern Rust, colored ASCII art
- **Cons:** Still purely visual, no simulation/gameplay
- **Features:** Custom tanks, fish, ducks, crabs - animation frames in JSON

### 3. ASCII-Aquarium (Python)
- **Repo:** https://github.com/ahoink/ASCII-Aquarium
- **Pros:** Python accessibility
- **Cons:** Less mature, limited features

**Key Gap Analysis:** All existing solutions are purely visual screensavers. None offer:
- Simulation mechanics (health, hunger, breeding)
- Persistent state
- Meaningful user interaction beyond viewing
- Economy/progression systems

## Stakeholders

- Primary: Developer (learning/fun project)
- Secondary: Terminal enthusiasts, lo-fi aesthetic fans

## Success Criteria

1. Fish require feeding and will die if neglected (starvation mechanic works)
2. Multiple species with distinct mechanical behaviors (different hunger rates, health, lifespans)
3. Fish breed automatically when conditions are met
4. State persists between sessions (save/load works)
5. Configurable simulation speed
6. Clean, responsive terminal UI with ratatui
7. Simple ASCII visuals that convey fish state clearly

## Core Features (In Scope)

### Simulation Mechanics
- **Hunger system**: Fish hunger increases over time (rate varies by species)
- **Health system**: Health drains when hunger maxes out; death at 0 health
- **Breeding**: Automatic when fish are healthy + fed, same species only, instant adults
- **Persistence**: State saves on exit, loads on start; time pauses when closed

### Fish Variety
- Multiple species with unique stats:
  - Hunger rate (how fast they get hungry)
  - Base health
  - Lifespan (max age before natural death - simple countdown)
- Simple ASCII sprites (1-3 characters, e.g., `><>`, `<><`, `><>>`)

### Player Interactions
- **Feed**: Global scatter - press key, food appears, all hungry fish eat
- **Spawn fish**: Sandbox mode - freely add any species
- **Remove fish**: Delete selected fish from tank
- **View stats**: Select fish to see individual hunger/health
- **Adjust speed**: Change simulation speed at runtime

### Technical
- **Language**: Rust
- **UI Framework**: ratatui
- **Visuals**: Simple ASCII (`><>` style, 1-3 chars per fish)
- **Colors**: Basic terminal colors to indicate fish state (hungry = yellow, critical = red)

## Out of Scope

### Excluded Mechanics (Tier 2+ - Future Expansion)
- Water quality / tank cleanliness
- Temperature / pH management
- Fish stress / happiness
- Individual fish personalities
- Disease / illness mechanics
- Growth stages (fry -> adult)
- Cross-species breeding
- Visual death stages

### Excluded Features
- Economy / currency system
- Shop / unlock mechanics
- Achievements / goals / structured progression
- Multi-character detailed fish sprites
- Animation frames for fish movement
- Sound effects
- Background daemon mode (time continues when closed)
- Network features / multiplayer
- Real-time simulation catch-up after closing

### Excluded Technical
- True color support (stick to basic 16 colors)
- Custom fish sprite editor
- Plugin / mod system

## Constraints

- Terminal-based only (no GUI)
- Must work on macOS (primary development platform)
- Should work on Linux (secondary)
- Windows support: nice-to-have, not required
- Minimum terminal size: 80x24

## Key Decisions

| Decision | Options | Chosen | Rationale |
|----------|---------|--------|-----------|
| Language | Rust, Go, Python, TypeScript | Rust | Performance, safety, ratatui ecosystem |
| UI Framework | ratatui, crossterm only, termion | ratatui | Best Rust TUI library, good abstractions |
| Visual style | ASCII, Unicode, Mix | ASCII | Simple `><>` style, 1-3 chars per fish |
| Color depth | Mono, 16, 256, True | 16 | Wide terminal compatibility |
| Simulation vs Decoration | Pure visual, Light sim, Full sim | Full sim | Differentiator from existing solutions |
| Time model | Real-time, Accelerated, Configurable | Configurable | User flexibility for different play styles |
| Offline behavior | Pause, Catch-up, Daemon | Pause | Simpler implementation, predictable state |
| Simulation depth | Tier 1, Tier 2, Tier 3 | Tier 1 | Start simple: Hunger + Health. Expand later |
| Fish variety | Visual only, Mechanical, Personalities | Mechanical | Species-level differences create gameplay |
| Breeding | Same species, Cross-breeding | Same species | Simpler, more realistic |
| Breeding complexity | Growth stages, Instant adults | Instant adults | Automatic trigger, instant output, cooldown |
| Death mechanics | Complex, Simple | Simple | Starvation only, fish disappears |
| Progression | Economy, Achievements, Sandbox | Sandbox | Pure sandbox - spawn any species freely |
| Feeding | Targeted, Global scatter | Global scatter | Press key, food appears for all |

## Core Mechanics Summary

### Fish Stats (Tier 1 - Light)
- **Hunger**: 0-100, increases over time (rate varies by species)
- **Health**: 0-100, drains when hunger hits 100
- **Age**: Tracks time alive (for display/lifespan tracking)

### Species Variety
Each species has unique:
- Hunger rate (how fast they get hungry)
- Base health (starting health)
- Lifespan (max simulation ticks before natural death)
- Visual representation (ASCII sprite)

### Breeding (Super Simple)
- **Trigger**: Automatic when both fish are healthy (>50) and fed (<50 hunger)
- **Compatibility**: Same species only
- **Output**: Babies are instant adults with full health
- **Limit**: Simple cooldown timer per fish (prevents spam)

### Death (Super Simple)
- **Starvation**: Hunger reaches 100 -> health drains rapidly
- **Death trigger**: Health reaches 0
- **Natural death**: Age reaches species lifespan
- **Visual**: Dead fish simply disappear from tank
- **No illness**, no critical visual states

### Feeding (Global Scatter)
- Player presses feed key (e.g., `f`)
- Food particles appear in tank
- All fish with hunger > 0 eat and reduce hunger
- Simple, no targeting needed

## Data Model (Conceptual)

```
Tank {
  fish: Vec<Fish>
  simulation_speed: f32
  tick_count: u64
}

Fish {
  id: Uuid
  species: Species
  position: (x, y)
  direction: Direction  // left or right
  hunger: u8           // 0-100
  health: u8           // 0-100
  age: u64             // ticks alive
  breeding_cooldown: u32
}

Species {
  name: String
  sprite_left: String   // e.g., "<><"
  sprite_right: String  // e.g., "><>"
  hunger_rate: f32      // hunger increase per tick
  base_health: u8
  lifespan: u64         // max age in ticks
  breeding_cooldown: u32
}
```

## UI Layout (Conceptual)

```
+--------------------------------------------------+
|  Aquarium Simulator          Speed: 1x   Fish: 5 |
+--------------------------------------------------+
|                                                  |
|        ><>                                       |
|                    <><                           |
|   ><>>                          ><>              |
|                                                  |
|              <><                                 |
|                                                  |
+--------------------------------------------------+
| Selected: Goldfish #3  | Hunger: 45 | Health: 80 |
| [F]eed [A]dd [R]emove [+/-]Speed [Q]uit         |
+--------------------------------------------------+
```

## Open Questions

None - all critical questions answered. Ready for implementation.
