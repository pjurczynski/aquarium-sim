# Aquarium Simulator - Playbook

## Development Workflow

### TDD Approach
Not strict unit testing for this project, but:
1. Implement feature
2. Run the app manually to verify behavior
3. Fix bugs immediately
4. Commit when working

### Commit Strategy
Granular commits per task:
- M1-T1: "Initialize Rust project structure"
- M1-T2: "Add ratatui, crossterm, serde, uuid dependencies"
- M1-T3: "Create basic event loop with clean exit"

### Code Organization
```
aquarium-sim/
  src/
    main.rs           # Entry point, event loop
    app.rs            # App state and main logic
    tank.rs           # Tank struct and simulation logic
    fish.rs           # Fish struct and behavior
    species.rs        # Species definitions
    ui.rs             # ratatui rendering
    save.rs           # Persistence (save/load)
  Cargo.toml
  save.json          # Persistent state file
```

### Testing Strategy
Manual testing focus:
- Run app after each feature
- Verify UI renders correctly
- Test keyboard inputs work
- Check state persists across restarts
- Watch for panics/crashes

### Documentation Updates After Each Milestone
1. Update docs/tasks.md with completion notes
2. Archive completed task to docs/done/NN-milestone-name.md
3. Update docs/scope.md if capabilities change
4. Append to docs/decisions.md for significant choices
5. Rewrite CONTEXT.md with new state (keep <200 lines)
6. Remove completed item from docs/backlog.json
7. Commit all changes

## Rust-Specific Guidelines

### Error Handling
- Use `Result` for operations that can fail
- Use `expect()` with clear messages for setup code
- Use `?` operator for propagating errors
- Don't panic in simulation logic

### Ownership
- Clone sparingly (only when necessary)
- Use references where possible
- Keep data model simple to avoid lifetime complexity

### Dependencies
Keep minimal:
- ratatui: TUI framework
- crossterm: Terminal manipulation
- serde + serde_json: Serialization
- uuid: Unique fish IDs

### Performance
Not critical for this project, but:
- Use `Vec` for fish collection
- Simple linear searches acceptable (small N)
- No need for complex spatial indexing

## ratatui Patterns

### Event Loop Structure
```rust
loop {
    terminal.draw(|f| ui::render(f, &app))?;

    if event::poll(Duration::from_millis(100))? {
        if let Event::Key(key) = event::read()? {
            match key.code {
                KeyCode::Char('q') => break,
                // ... handle other keys
            }
        }
    }

    app.tick(); // Update simulation
}
```

### Rendering Pattern
Separate UI logic from app logic:
- `ui.rs` contains all ratatui widget code
- `app.rs` contains state and simulation logic
- Pass immutable references to render functions

## Common Pitfalls to Avoid

1. **Don't** use floating-point for critical simulation state (use integers)
2. **Don't** forget to restore terminal on panic (use proper cleanup)
3. **Don't** block the event loop (keep tick logic fast)
4. **Don't** forget terminal resize handling (will be in M11)
5. **Don't** make fish movement too fast (tune movement speed)

## Decision Log Reference

Major technical decisions documented in docs/decisions.md:
- Choice of Rust + ratatui over other frameworks
- Tier 1 simulation scope (Hunger + Health only)
- Simple ASCII visuals (no complex animations)
- Global scatter feeding (not targeted)
- Pause-on-close (not background daemon)
- Same-species breeding only
