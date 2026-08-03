use std::time::Duration;

use crossterm::event::{self, Event, KeyCode, KeyEventKind};
use dawl_tui::canvas::Grid;
use dawl_tui::error::Result;
use dawl_tui::theme::{Palette, Style as SemanticStyle};
use ratatui::style::{Color, Style};
use ratatui::text::{Line, Span, Text};
use ratatui::widgets::{Block, Paragraph};

#[derive(Default)]
struct ViewState {
    x: u16,
    y: u16,
}

pub fn show(grid: &Grid, title: &str) -> Result<()> {
    let mut terminal = ratatui::try_init()?;
    let result = run(&mut terminal, grid, title);
    let restored = ratatui::try_restore();
    result.and(restored.map_err(Into::into))
}

fn run(terminal: &mut ratatui::DefaultTerminal, grid: &Grid, title: &str) -> Result<()> {
    let mut state = ViewState::default();
    loop {
        draw(terminal, grid, title, &state)?;
        if read_key(&mut state)? {
            return Ok(());
        }
    }
}

fn draw(
    terminal: &mut ratatui::DefaultTerminal,
    grid: &Grid,
    title: &str,
    state: &ViewState,
) -> Result<()> {
    let content = grid_text(grid);
    terminal.draw(|frame| {
        let widget = Paragraph::new(content)
            .scroll((state.y, state.x))
            .block(Block::bordered().title(format!(" {title} · arrows pan · q quits ")));
        frame.render_widget(widget, frame.area());
    })?;
    Ok(())
}

fn read_key(state: &mut ViewState) -> Result<bool> {
    if !event::poll(Duration::from_millis(100))? {
        return Ok(false);
    }
    let Event::Key(key) = event::read()? else {
        return Ok(false);
    };
    if key.kind != KeyEventKind::Press {
        return Ok(false);
    }
    Ok(handle_key(state, key.code))
}

fn handle_key(state: &mut ViewState, key: KeyCode) -> bool {
    match key {
        KeyCode::Char('q') | KeyCode::Esc => return true,
        KeyCode::Left => state.x = state.x.saturating_sub(2),
        KeyCode::Right => state.x = state.x.saturating_add(2),
        KeyCode::Up => state.y = state.y.saturating_sub(1),
        KeyCode::Down => state.y = state.y.saturating_add(1),
        KeyCode::Char('0') => *state = ViewState::default(),
        _ => {}
    }
    false
}

fn grid_text(grid: &Grid) -> Text<'static> {
    Text::from(
        (0..grid.height)
            .map(|y| grid_line(grid, y))
            .collect::<Vec<_>>(),
    )
}

fn grid_line(grid: &Grid, y: u16) -> Line<'static> {
    let mut spans = Vec::new();
    let mut x = 0;
    while x < grid.width {
        let Some(cell) = grid.cell(x, y) else {
            break;
        };
        let semantic = cell.style;
        let mut run = String::new();
        while x < grid.width && grid.cell(x, y).is_some_and(|item| item.style == semantic) {
            append_cell(&mut run, grid, x, y);
            x = x.saturating_add(1);
        }
        spans.push(Span::styled(run, terminal_style(semantic)));
    }
    Line::from(spans)
}

fn append_cell(run: &mut String, grid: &Grid, x: u16, y: u16) {
    let Some(cell) = grid.cell(x, y) else {
        return;
    };
    if !cell.continuation {
        run.push(grid.visible_char(x, y));
    }
}

fn terminal_style(semantic: SemanticStyle) -> Style {
    let palette = Palette::midnight();
    Style::default()
        .fg(color(palette.foreground(semantic)))
        .bg(color(palette.background()))
}

fn color(value: dawl_tui::theme::Rgb) -> Color {
    Color::Rgb(value.red, value.green, value.blue)
}
