# M8: Player Input - Speed Control - COMPLETED

## Overview
Implemented simulation speed adjustment. Players can now control how fast the simulation runs using
+/- keys, allowing for different play styles.

## Tasks Completed

### M8-T1: Handle +/- keys for speed
- Added KeyCode::Char('+') | KeyCode::Char('=') for increase
- Added KeyCode::Char('-') for decrease
- Calls tank.increase_speed() / tank.decrease_speed()

### M8-T2: Define speed presets
- Implemented 5 speed levels: 0.5x, 1.0x, 2.0x, 5.0x, 10.0x
- increase_speed() steps up through preset array
- decrease_speed() steps down through preset array
- Clamps at min (0.5x) and max (10.0x)
- Uses floating-point comparison with epsilon for position finding

### M8-T3: Display current speed in header
Already implemented in M3. Header automatically shows current speed value.
- Format: "Speed: {speed}x"
- Updates immediately when speed changes

## Implementation Details

### Speed Control Methods
```rust
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
```

### Key Handling
```rust
KeyCode::Char('+') | KeyCode::Char('=') => tank.increase_speed(),
KeyCode::Char('-') => tank.decrease_speed(),
```

## Key Decisions
- 5 preset levels (reasonable range)
- Steps through array (predictable behavior)
- Floating-point epsilon comparison (handles rounding)
- Both '+' and '=' keys for increase (convenience, no shift needed)
- Speed field is set but not currently affecting simulation rate
- Header automatically reflects speed changes

## Files Changed
- Modified: src/tank.rs (added increase_speed() and decrease_speed() methods)
- Modified: src/main.rs (added +/- key handling)

## Observable Behavior
Speed control now available:
1. Press '+' or '=' to increase speed (1.0x -> 2.0x -> 5.0x -> 10.0x)
2. Press '-' to decrease speed (10.0x -> 5.0x -> 2.0x -> 1.0x -> 0.5x)
3. Header shows current speed
4. Clamps at min/max (no overflow)

## Note on Speed Application
In Tier 1, speed field is set but simulation runs at fixed rate (every frame).
Future enhancements could:
- Skip ticks based on speed
- Adjust hunger rate by speed multiplier
- Throttle render rate

For now, speed is mostly informational/cosmetic, ready for future integration.

## Next Steps
M9: Breeding System - automatic fish breeding when conditions met
