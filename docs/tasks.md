# Current Tasks: M11 - Polish & UX (FINAL MILESTONE)

## Overview
Add quality-of-life features to improve user experience. This is the final milestone for Tier 1.

## Detailed Steps

### M11-T1: Handle terminal resize
1. Detect terminal resize events (Event::Resize in crossterm)
2. Update tank.dimensions on resize
3. Clamp fish positions to new boundaries if needed
4. Test: Resize terminal, fish should stay within bounds
5. Commit: "Handle terminal resize gracefully"

### M11-T2: Add pause functionality
1. Add paused: bool field to app state
2. Handle Space key to toggle pause
3. Skip tank.tick() when paused
4. Show "PAUSED" indicator in header
5. Test: Press Space, simulation stops
6. Commit: "Add pause/resume with Space key"

### M11-T3: Improve species picker UI
1. Add species_picker_open: bool to app state
2. When 'A' pressed, open picker modal
3. Render modal with numbered species list
4. Handle 1-5 keys to select species
5. Esc to cancel
6. Test: Press 'A', see modal, select species
7. Commit: "Add species picker modal"

### M11-T4: Add help overlay
1. Add help_open: bool to app state
2. Handle '?' key to toggle help
3. Render modal showing all keybindings
4. Esc or '?' to close
5. Test: Press '?', see help, close with Esc
6. Commit: "Add help overlay with keybindings"

## Expected Outcome
After M11 completion:
- Terminal resize handled smoothly
- Pause/resume with Space
- Better species selection UX
- Help available for new users
- Tier 1 COMPLETE!

## Implementation Approach

### App State Structure
```rust
struct AppState {
    tank: Tank,
    selected_fish_index: Option<usize>,
    paused: bool,
    species_picker_open: bool,
    help_open: bool,
}
```

### Terminal Resize
```rust
Event::Resize(width, height) => {
    tank.dimensions = (width, height);
    // Clamp fish positions
    for fish in &mut tank.fish {
        fish.position.0 = fish.position.0.min(width.saturating_sub(10));
        fish.position.1 = fish.position.1.min(height.saturating_sub(5));
    }
}
```

### Pause
```rust
KeyCode::Char(' ') => {
    paused = !paused;
}

// In loop
if !paused {
    tank.tick();
}
```

### Species Picker Modal
```rust
if species_picker_open {
    // Render centered modal
    let modal_area = centered_rect(40, 50, f.area());
    let block = Block::default()
        .title("Select Species")
        .borders(Borders::ALL);
    f.render_widget(Clear, modal_area); // Clear background
    f.render_widget(block, modal_area);

    // List species with numbers
    // Handle 1-5 keys to select
}
```

### Help Overlay
```rust
if help_open {
    // Render large centered modal with keybindings
    let help_text = vec![
        "Keybindings:",
        "q - Quit",
        "f - Feed all fish",
        "a - Add fish",
        // ... etc
    ];
}
```

## Blockers
None expected. Final polish work.

## Notes
- Modal rendering requires Clear widget from ratatui
- Centered rect helper function useful for modals
- Pause should also pause breeding/death (just skip tick())
- Help text should be comprehensive but concise
- Species picker makes 'A' more intuitive (no more cycling)
