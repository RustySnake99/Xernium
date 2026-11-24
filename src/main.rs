mod app;
mod models;

use app::App;
use crossterm::event::{self, Event, KeyCode, KeyEventKind};
use ratatui::{
    Frame,
    layout::{Constraint, Layout, Rect},
    style::{Color, Modifier, Style},
    text::{Line, Span},
    widgets::{Block, Paragraph},
};

fn parse_hex(s: &str) -> Option<Color> {
    let s = s.trim().trim_start_matches('#');
    if s.len() != 6 {
        return None;
    }
    let n = u32::from_str_radix(s, 16).ok()?;
    Some(Color::Rgb((n >> 16) as u8, (n >> 8) as u8, n as u8))
}

fn render_ui(f: &mut Frame, app: &App) {
    let chunks = Layout::vertical([Constraint::Length(20), Constraint::Min(10)]).split(f.area());
    let (cw, ch) = (5u16, 2u16);

    for (&(r, c), &i) in &app.grid {
        let e = &app.elements[i];
        let area = Rect::new(
            chunks[0].x + c as u16 * cw,
            chunks[0].y + r as u16 * ch,
            cw,
            ch,
        )
        .intersection(chunks[0]);

        let bg = e
            .cpk_hex_color
            .as_deref()
            .and_then(parse_hex)
            .unwrap_or(Color::DarkGray);
        let mut style = Style::new().bg(bg).fg(Color::Black);

        if (r, c) == app.cursor {
            style = style.add_modifier(Modifier::REVERSED | Modifier::BOLD);
        }
        let text = vec![
            Line::from(format!("{:^5}", e.atomic_number)),
            Line::from(format!("{:^5}", e.symbol)),
        ];
        f.render_widget(Paragraph::new(text).style(style), area);
    }

    let detail = match app.get_selected_element() {
        Some(e) => {
            let row = |k: &str, v: &Option<String>| {
                Line::from(vec![
                    Span::styled(format!("{k:<22}"), Style::new().bold()),
                    Span::raw(v.clone().unwrap_or_else(|| "-".into())),
                ])
            };
            vec![
                Line::from(format!(
                    "{} ({}) [Z = {}]",
                    e.name, e.symbol, e.atomic_number
                ))
                .style(Style::new().bold().fg(Color::Cyan)),
                row("Atomic Mass (u): ", &e.atomic_mass),
                row("Group Block: ", &e.group_block),
                row("Electron Configuration: ", &e.electron_configuration),
                row("Electronegativity: ", &e.electronegativity),
                row("Oxidation States: ", &e.oxidation_states),
                row("Standard State: ", &e.standard_state),
                row("Melting Point (K): ", &e.melting_point),
                row("Boiling Point (K): ", &e.boiling_point),
                row("Density (g/cm^3): ", &e.density),
                row("Atomic Radius (van der Waals; in pm): ", &e.atomic_radius),
                row("Ionization Energy (in eV): ", &e.ionization_energy),
                row("Electron Affinity (in eV): ", &e.electron_affinity),
                row("Year of Discovery: ", &e.year_discovered),
            ]
        }
        None => vec![Line::from("")],
    };
    f.render_widget(
        Paragraph::new(detail)
            .block(Block::bordered().title("Details [Arrow-keys: Navigation | Esc: Quit]")),
        chunks[1],
    );
}

fn main() -> std::io::Result<()> {
    let mut terminal = ratatui::init();
    let mut app = App::new();

    while !app.is_quit {
        terminal.draw(|f| render_ui(f, &app))?;
        if let Event::Key(k) = event::read()? {
            if k.kind != KeyEventKind::Press {
                continue;
            }
            match k.code {
                KeyCode::Esc => app.is_quit = true,
                KeyCode::Left => app.step(0, -1),
                KeyCode::Right => app.step(0, 1),
                KeyCode::Up => app.step(-1, 0),
                KeyCode::Down => app.step(1, 0),
                _ => {}
            }
        }
    }
    ratatui::restore();
    Ok(())
}
