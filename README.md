# Aquarium Simulator

A terminal-based virtual pet aquarium with real simulation mechanics. Unlike decorative
screensavers, fish here need feeding, can breed, and will die if neglected.

## Quick Start

```bash
cargo run
```

## Controls

| Key | Action |
|-----|--------|
| `F` | Feed all fish |
| `A` | Add a new fish (opens species picker) |
| `R` | Remove selected fish |
| `Tab` | Cycle through fish selection |
| `+` / `-` | Speed up / slow down simulation |
| `Space` | Pause simulation |
| `S` | Save tank to file |
| `L` | Load tank from file |
| `?` | Show help overlay |
| `Q` | Quit |

## Fish Species

| Species | Hunger Rate | Lifespan | Notes |
|---------|-------------|----------|-------|
| Goldfish | Slow | Long | Hardy beginner fish |
| Guppy | Fast | Short | Breeds quickly |
| Betta | Medium | Medium | Colorful |
| Tetra | Fast | Short | Small and quick |
| Angelfish | Slow | Long | Elegant swimmers |

## Mechanics

### Hunger & Health
- Fish get hungry over time (rate varies by species)
- When hunger maxes out, health starts draining
- Health reaches 0 = fish dies and disappears
- Feed regularly to keep them alive

### Breeding
- Automatic when two same-species fish are:
  - Healthy (health > 50%)
  - Well-fed (hunger < 50%)
- Babies spawn as instant adults
- Cooldown prevents breeding spam

### Persistence
- Tank state saves to `tank.json`
- Time pauses when app closes (no dead fish surprises)
- Load to continue where you left off

## Simulation Speed

5 presets available:
- 0.5x - Relaxed watching
- 1x - Normal (default)
- 2x - Faster action
- 5x - Quick breeding cycles
- 10x - Speed run

## Requirements

### System
- **Rust toolchain** - Install via [rustup](https://rustup.rs/):
  ```bash
  curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
  ```
- **macOS** or **Linux** (Windows: untested but may work)

### Terminal
- 16-color support (most modern terminals)
- Minimum size: 80x24 characters
