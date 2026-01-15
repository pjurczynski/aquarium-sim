mod fish;
mod save;
mod species;
mod tank;
mod ui;

use crossterm::{
    event::{self, Event, KeyCode},
    execute,
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
};
use ratatui::{
    backend::CrosstermBackend,
    Terminal,
};
use std::io;

fn main() -> Result<(), io::Error> {
    // Setup terminal
    enable_raw_mode()?;
    let mut stdout = io::stdout();
    execute!(stdout, EnterAlternateScreen)?;
    let backend = CrosstermBackend::new(stdout);
    let mut terminal = Terminal::new(backend)?;

    // Run app
    let result = run_app(&mut terminal);

    // Restore terminal
    disable_raw_mode()?;
    execute!(terminal.backend_mut(), LeaveAlternateScreen)?;
    terminal.show_cursor()?;

    result
}

fn run_app<B: ratatui::backend::Backend>(terminal: &mut Terminal<B>) -> Result<(), io::Error> {
    // Initialize tank - load from save or create new
    let size = terminal.size()?;
    let mut tank = if let Some(mut loaded_tank) = save::load_tank() {
        // Update dimensions to current terminal size
        loaded_tank.dimensions = (size.width, size.height);
        loaded_tank
    } else {
        // Create new tank with test fish
        let mut tank = tank::Tank::new(size.width, size.height);
        let test_fish1 = fish::Fish::new("Goldfish".to_string(), (10, 5));
        let test_fish2 = fish::Fish::new("Guppy".to_string(), (30, 10));
        let test_fish3 = fish::Fish::new("Betta".to_string(), (50, 8));
        tank.add_fish(test_fish1);
        tank.add_fish(test_fish2);
        tank.add_fish(test_fish3);
        tank
    };
    let mut selected_fish_index: Option<usize> = None;
    let mut paused = false;
    let mut species_picker_open = false;
    let mut help_open = false;

    loop {
        // Calculate selected_fish_id from index
        let selected_fish_id = selected_fish_index
            .and_then(|idx| tank.fish.get(idx))
            .map(|f| f.id);

        // Render UI
        terminal.draw(|f| {
            ui::render(f, &tank, selected_fish_id, paused, species_picker_open, help_open);
        })?;

        // Handle input
        if event::poll(std::time::Duration::from_millis(100))? {
            let event = event::read()?;
            match event {
                Event::Resize(width, height) => {
                    tank.dimensions = (width, height);
                    // Clamp fish positions to new boundaries
                    for fish in &mut tank.fish {
                        fish.position.0 = fish.position.0.min(width.saturating_sub(10));
                        fish.position.1 = fish.position.1.min(height.saturating_sub(5));
                    }
                }
                Event::Key(key) => {
                    // Handle modal-specific keys first
                    if help_open {
                        match key.code {
                            KeyCode::Char('?') | KeyCode::Esc => help_open = false,
                            _ => {}
                        }
                        continue;
                    }

                    if species_picker_open {
                        match key.code {
                            KeyCode::Esc => species_picker_open = false,
                            KeyCode::Char('1') => {
                                let position = (10 + (tank.fish.len() as u16 * 3) % 60, 5 + (tank.fish.len() as u16 % 10));
                                tank.add_fish(fish::Fish::new("Goldfish".to_string(), position));
                                species_picker_open = false;
                            }
                            KeyCode::Char('2') => {
                                let position = (10 + (tank.fish.len() as u16 * 3) % 60, 5 + (tank.fish.len() as u16 % 10));
                                tank.add_fish(fish::Fish::new("Guppy".to_string(), position));
                                species_picker_open = false;
                            }
                            KeyCode::Char('3') => {
                                let position = (10 + (tank.fish.len() as u16 * 3) % 60, 5 + (tank.fish.len() as u16 % 10));
                                tank.add_fish(fish::Fish::new("Betta".to_string(), position));
                                species_picker_open = false;
                            }
                            KeyCode::Char('4') => {
                                let position = (10 + (tank.fish.len() as u16 * 3) % 60, 5 + (tank.fish.len() as u16 % 10));
                                tank.add_fish(fish::Fish::new("Tetra".to_string(), position));
                                species_picker_open = false;
                            }
                            KeyCode::Char('5') => {
                                let position = (10 + (tank.fish.len() as u16 * 3) % 60, 5 + (tank.fish.len() as u16 % 10));
                                tank.add_fish(fish::Fish::new("Angelfish".to_string(), position));
                                species_picker_open = false;
                            }
                            _ => {}
                        }
                        continue;
                    }

                    // Normal keys
                    match key.code {
                    KeyCode::Char('q') | KeyCode::Char('Q') => break,
                    KeyCode::Char(' ') => paused = !paused,
                    KeyCode::Char('?') => help_open = true,
                    KeyCode::Char('f') | KeyCode::Char('F') => tank.feed(),
                    KeyCode::Char('+') | KeyCode::Char('=') => tank.increase_speed(),
                    KeyCode::Char('-') => tank.decrease_speed(),
                    KeyCode::Tab => {
                        if !tank.fish.is_empty() {
                            selected_fish_index = Some(match selected_fish_index {
                                None => 0,
                                Some(idx) => (idx + 1) % tank.fish.len(),
                            });
                        }
                    }
                    KeyCode::Char('a') | KeyCode::Char('A') => {
                        species_picker_open = true;
                    }
                    KeyCode::Char('r') | KeyCode::Char('R') => {
                        if let Some(fish_id) = selected_fish_id {
                            tank.remove_fish(fish_id);
                            selected_fish_index = None;
                        }
                    }
                    _ => {}
                }
                }
                _ => {}
            }
        }

        // Update simulation (only if not paused)
        if !paused {
            tank.tick();
        }
    }

    // Save tank state on exit
    if let Err(e) = save::save_tank(&tank) {
        eprintln!("Failed to save tank: {}", e);
    }

    Ok(())
}
