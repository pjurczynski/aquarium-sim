# M4: Fish Movement - COMPLETED

## Overview
Implemented autonomous fish movement with horizontal swimming, wall bouncing, and vertical drift.
Fish now move naturally across the tank.

## Tasks Completed

### M4-T1: Implement basic horizontal movement
- Added tick() method to Tank struct
- Fish move horizontally based on Direction (Left/Right)
- Movement speed: 1 unit per tick
- Wall collision detection:
  - Left wall: position.0 == 0
  - Right wall: position.0 >= dimensions.0 - 8 (margin for sprite width)
- Direction reverses on wall hit
- Used saturating_add/sub for safety

### M4-T2: Add vertical drift
- Implemented subtle vertical movement
- Drift occurs roughly every 3rd tick (tick_count % 3 == 0)
- Pseudo-random direction using tick_count modulo
- Respects top/bottom boundaries
- Boundaries: position.1 > 0 and < dimensions.1 - 5

### M4-T3: Prevent fish overlap
- Marked as optional for Tier 1
- Skipped for now (can add in future if needed)
- Simple overlap is acceptable for initial version

## Implementation Details

### Tank.tick() Method
```rust
pub fn tick(&mut self) {
    self.tick_count += 1;

    for fish in &mut self.fish {
        // Horizontal movement with wall bouncing
        match fish.direction {
            Direction::Left => {
                if fish.position.0 > 0 {
                    fish.position.0 -= 1;
                } else {
                    fish.direction = Direction::Right;
                }
            }
            Direction::Right => {
                if fish.position.0 < self.dimensions.0 - 8 {
                    fish.position.0 += 1;
                } else {
                    fish.direction = Direction::Left;
                }
            }
        }

        // Vertical drift (subtle, ~30% of ticks)
        if self.tick_count % 3 == 0 {
            let drift = (self.tick_count % 7) as i32 - 3;
            if drift < 0 && fish.position.1 > 0 {
                fish.position.1 -= 1;
            } else if drift > 0 && fish.position.1 < self.dimensions.1 - 5 {
                fish.position.1 += 1;
            }
        }
    }
}
```

### Main Loop Integration
- Added tank.tick() call at end of main event loop
- Runs after input handling
- Movement updates happen every loop iteration (~100ms with poll timeout)

## Key Decisions
- Fixed movement speed (1 unit/tick) for all fish in Tier 1
- Simple pseudo-random drift using tick_count (no rand crate dependency)
- Wall margins account for sprite width (8 chars for safety)
- Vertical boundaries have margin to prevent fish at edges
- Skipped collision avoidance (optional, can add later)

## Files Changed
- Modified: src/tank.rs (added tick() method, imported Direction)
- Modified: src/main.rs (added tank.tick() call in event loop)

## Visual Result
Fish now swim autonomously:
- Move left/right continuously
- Bounce off walls and change direction
- Drift up/down slightly for natural feel
- Smooth, organic movement visible in terminal

## Next Steps
M5: Simulation Loop - hunger/health updates, age tracking, death conditions
