use crate::fish::{Direction, Fish};
use crate::species;
use crate::tank::Tank;
use ratatui::{
    layout::{Alignment, Constraint, Layout, Rect},
    style::{Color, Style},
    text::{Line, Span},
    widgets::{Block, Borders, Clear, Paragraph},
    Frame,
};

pub fn render(f: &mut Frame, tank: &Tank, selected_fish_id: Option<uuid::Uuid>, paused: bool, species_picker_open: bool, help_open: bool) {
    let chunks = Layout::vertical([
        Constraint::Length(3), // Header
        Constraint::Min(0),    // Tank area
        Constraint::Length(3), // Footer
    ])
    .split(f.area());

    render_header(f, chunks[0], tank, paused);
    render_tank(f, chunks[1], tank);
    render_footer(f, chunks[2], tank, selected_fish_id);

    // Render modals on top
    if species_picker_open {
        render_species_picker(f);
    }

    if help_open {
        render_help(f);
    }
}

fn render_header(f: &mut Frame, area: Rect, tank: &Tank, paused: bool) {
    let pause_indicator = if paused { " [PAUSED]" } else { "" };
    let header_text = format!(
        "Aquarium Simulator{}    Speed: {:.1}x    Fish: {}",
        pause_indicator,
        tank.simulation_speed,
        tank.fish.len()
    );

    let header = Paragraph::new(header_text)
        .block(Block::default().borders(Borders::ALL))
        .alignment(Alignment::Center);

    f.render_widget(header, area);
}

fn render_tank(f: &mut Frame, area: Rect, tank: &Tank) {
    let block = Block::default().borders(Borders::ALL).title("Tank");
    let inner = block.inner(area);
    f.render_widget(block, area);

    // Render each fish
    for fish in &tank.fish {
        render_fish(f, inner, fish);
    }
}

fn render_fish(f: &mut Frame, area: Rect, fish: &Fish) {
    // Get species to determine sprite
    let species_data = species::get_species(&fish.species);
    if species_data.is_none() {
        return;
    }
    let species_data = species_data.unwrap();

    // Choose sprite based on direction
    let sprite = match fish.direction {
        Direction::Left => &species_data.sprite_left,
        Direction::Right => &species_data.sprite_right,
    };

    // Determine color based on hunger level
    let color = if fish.hunger < 50 {
        Color::Cyan
    } else if fish.hunger < 80 {
        Color::Yellow
    } else {
        Color::Red
    };

    // Calculate absolute position
    let (x, y) = fish.position;

    // Check if fish is within visible area
    if x < area.width && y < area.height {
        let fish_para = Paragraph::new(sprite.as_str())
            .style(Style::default().fg(color));

        let fish_area = Rect {
            x: area.x + x,
            y: area.y + y,
            width: sprite.len() as u16,
            height: 1,
        };

        f.render_widget(fish_para, fish_area);
    }
}

fn render_footer(f: &mut Frame, area: Rect, tank: &Tank, selected_fish_id: Option<uuid::Uuid>) {
    let mut footer_text = String::from("[F]eed [A]dd [R]emove [Tab]Select [Space]Pause [+/-]Speed [?]Help [Q]uit");

    // If a fish is selected, show its stats
    if let Some(fish_id) = selected_fish_id {
        if let Some(fish) = tank.get_fish(fish_id) {
            footer_text = format!(
                "Selected: {} | Hunger: {} | Health: {} | Age: {} | {}",
                fish.species, fish.hunger, fish.health, fish.age, footer_text
            );
        }
    }

    let footer = Paragraph::new(footer_text)
        .block(Block::default().borders(Borders::ALL))
        .alignment(Alignment::Left);

    f.render_widget(footer, area);
}

fn centered_rect(percent_x: u16, percent_y: u16, area: Rect) -> Rect {
    let popup_layout = Layout::vertical([
        Constraint::Percentage((100 - percent_y) / 2),
        Constraint::Percentage(percent_y),
        Constraint::Percentage((100 - percent_y) / 2),
    ])
    .split(area);

    Layout::horizontal([
        Constraint::Percentage((100 - percent_x) / 2),
        Constraint::Percentage(percent_x),
        Constraint::Percentage((100 - percent_x) / 2),
    ])
    .split(popup_layout[1])[1]
}

fn render_species_picker(f: &mut Frame) {
    let area = centered_rect(50, 40, f.area());

    let species_text = vec![
        "Select a species:",
        "",
        "1 - Goldfish (Slow hunger, high health, long life)",
        "2 - Guppy (Fast hunger, low health, short life)",
        "3 - Betta (Medium hunger, medium health, medium life)",
        "4 - Tetra (Fast hunger, low health, medium life)",
        "5 - Angelfish (Slow hunger, high health, long life)",
        "",
        "Press Esc to cancel",
    ];

    let text = species_text.join("\n");
    let block = Block::default()
        .title("Add Fish")
        .borders(Borders::ALL)
        .style(Style::default().bg(Color::Black));

    let paragraph = Paragraph::new(text)
        .block(block)
        .alignment(Alignment::Left);

    f.render_widget(Clear, area);
    f.render_widget(paragraph, area);
}

fn render_help(f: &mut Frame) {
    let area = centered_rect(60, 50, f.area());

    let help_text = vec![
        "Aquarium Simulator - Help",
        "",
        "Controls:",
        "  F - Feed all fish",
        "  A - Add fish (opens species picker)",
        "  R - Remove selected fish",
        "  Tab - Select next fish",
        "  Space - Pause/Resume simulation",
        "  + / = - Increase simulation speed",
        "  - - Decrease simulation speed",
        "  ? - Toggle this help",
        "  Q - Quit (auto-saves)",
        "",
        "Fish Care:",
        "  - Feed fish regularly to keep hunger low",
        "  - Fish turn yellow when hungry, red when critical",
        "  - Unfed fish will starve and die",
        "  - Healthy, fed fish breed automatically",
        "",
        "Press ? or Esc to close",
    ];

    let text = help_text.join("\n");
    let block = Block::default()
        .title("Help")
        .borders(Borders::ALL)
        .style(Style::default().bg(Color::Black));

    let paragraph = Paragraph::new(text)
        .block(block)
        .alignment(Alignment::Left);

    f.render_widget(Clear, area);
    f.render_widget(paragraph, area);
}
