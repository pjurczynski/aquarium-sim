# M10: Persistence - COMPLETED

## Overview
Implemented save/load functionality. Tank state now persists between sessions in a JSON file.
Time pauses when the app closes (no background simulation).

## Tasks Completed

### M10-T1: Implement save state to JSON
- Created save.rs module
- Implemented save_tank(tank: &Tank) function
- Serializes Tank using serde_json::to_string_pretty()
- Writes to "tank.json" in current directory
- Returns io::Result for error handling

### M10-T2: Implement load state from JSON
- Implemented load_tank() -> Option<Tank> function
- Checks if tank.json exists
- Deserializes JSON using serde_json::from_str()
- Returns None if file missing or invalid
- Handles errors gracefully

### M10-T3: Auto-save on clean exit
- Added save call in main.rs after event loop breaks
- Saves before terminal cleanup
- Logs error to stderr if save fails (doesn't crash)
- Works when quitting with 'q'

### M10-T4: Handle missing/corrupt save file
- load_tank() returns None for missing/corrupt files
- main.rs creates fresh tank if load returns None
- Initializes with 3 test fish for new tanks
- Loads existing state if available

## Implementation Details

### save.rs Module
```rust
use crate::tank::Tank;
use std::fs;
use std::io;

const SAVE_FILE: &str = "tank.json";

pub fn save_tank(tank: &Tank) -> io::Result<()> {
    let json = serde_json::to_string_pretty(tank)?;
    fs::write(SAVE_FILE, json)?;
    Ok(())
}

pub fn load_tank() -> Option<Tank> {
    if !std::path::Path::new(SAVE_FILE).exists() {
        return None;
    }

    let json = fs::read_to_string(SAVE_FILE).ok()?;
    serde_json::from_str(&json).ok()
}
```

### main.rs Integration
```rust
// At start of run_app
let mut tank = if let Some(mut loaded_tank) = save::load_tank() {
    // Update dimensions to current terminal size
    loaded_tank.dimensions = (size.width, size.height);
    loaded_tank
} else {
    // Create new tank with test fish
    let mut tank = tank::Tank::new(size.width, size.height);
    // ... add test fish
    tank
};

// Before returning from run_app
if let Err(e) = save::save_tank(&tank) {
    eprintln!("Failed to save tank: {}", e);
}
```

## Key Decisions
- Simple save location: "./tank.json" (no extra dependencies)
- Pretty JSON format (human-readable, debuggable)
- Update dimensions on load (adapt to current terminal size)
- Test fish only for new tanks (not on every load)
- No background simulation (time pauses on close)
- Graceful error handling (doesn't crash on save failure)

## Files Changed
- Created: src/save.rs (save/load functions)
- Modified: src/main.rs (integrated save/load, added save module)

## Observable Behavior
Persistence now working:
1. Start app, add/feed/manage fish
2. Press 'q' to quit
3. tank.json created in current directory
4. Restart app
5. Previous fish restored with same stats
6. Time continues from where it left off
7. Delete tank.json to start fresh

## Data Persisted
Everything in Tank struct:
- All fish (id, species, position, direction, hunger, health, age, cooldown)
- simulation_speed
- tick_count (time elapsed)
- dimensions (updated on load)

## Next Steps
M11: Polish & UX - terminal resize, pause, help overlay, species picker improvements
