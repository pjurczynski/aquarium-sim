# M3: Basic Rendering - COMPLETED

## Overview
Implemented complete visual display for the aquarium using ratatui. The app now renders fish at positions with proper sprites, colors, and UI chrome.

## Tasks Completed

### M3-T1: Render tank boundaries
- Created src/ui.rs module
- Implemented render_tank() using ratatui Block widget with borders
- Calculated inner area for fish rendering
- Added "Tank" title to border
- Added ui module to main.rs

### M3-T2: Render fish at positions
- Implemented render_fish() function
- Looks up species data to get sprite
- Chooses sprite_left or sprite_right based on fish Direction
- Renders at fish position coordinates
- Handles fish outside visible area gracefully
- Added test fish to main.rs for demonstration

### M3-T3: Render header bar
- Implemented render_header() function
- Displays "Aquarium Simulator" title
- Shows simulation speed (e.g., "Speed: 1.0x")
- Shows fish count (e.g., "Fish: 3")
- Centered alignment with borders

### M3-T4: Render footer/status bar
- Implemented render_footer() function
- Shows keybind hints: [F]eed [A]dd [R]emove [+/-]Speed [Q]uit
- When fish selected, appends stats: species, hunger, health, age
- Left-aligned with borders

### M3-T5: Color fish by state
- Implemented hunger-based color coding:
  - hunger < 50: Cyan (normal/healthy)
  - hunger 50-80: Yellow (hungry)
  - hunger > 80: Red (critical)
- Applied colors using ratatui Style with fg()

## Implementation Details

### UI Module Structure
- Main render() function orchestrates layout (header/tank/footer)
- render_header() - displays tank stats
- render_tank() - renders border and calls render_fish for each
- render_fish() - individual fish rendering with sprite selection and color
- render_footer() - controls and selected fish stats

### Main.rs Changes
- Added ui module import
- Replaced placeholder rendering with ui::render()
- Created Tank instance with terminal dimensions
- Added 3 test fish (Goldfish, Guppy, Betta) at different positions
- Wired selected_fish_id tracking (no selection yet, but plumbed)

## Key Decisions
- Separated UI logic into dedicated module (clean architecture)
- Used ratatui Rect for precise fish positioning
- Fish sprites rendered as individual Paragraph widgets
- Color logic in render_fish() keeps all visual logic together
- Test fish added in main.rs for visual verification

## Files Changed
- Created: src/ui.rs (complete rendering system)
- Modified: src/main.rs (wired ui module, created tank with test fish)

## Visual Result
```
+------------------------------------------------+
| Aquarium Simulator    Speed: 1.0x    Fish: 3  |
+------------------------------------------------+
| Tank                                           |
|          ><>                                   |
|                              o>                |
|                                          >((()>|
|                                                |
+------------------------------------------------+
| [F]eed [A]dd [R]emove [+/-]Speed [Q]uit       |
+------------------------------------------------+
```

## Next Steps
M4: Fish Movement - horizontal swimming, vertical drift, wall bouncing
