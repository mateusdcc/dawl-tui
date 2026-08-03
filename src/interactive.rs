use std::path::{Path, PathBuf};

use dawl_tui::error::{Error, Result};

pub fn view(path: &Path) -> Result<()> {
    let diagram = dawl_tui::load_diagram(path)?;
    let layout = dawl_tui::layout_diagram(&diagram, &Default::default())?;
    let grid = dawl_tui::render_diagram(&diagram, &layout, &Default::default())?;
    print!("{}", dawl_tui::export::ansi(&grid));
    Ok(())
}

pub fn watch(graph: PathBuf, events: PathBuf, headless: bool) -> Result<()> {
    let diagram = dawl_tui::load_diagram(&graph)?;
    let source = std::fs::read_to_string(events)?;
    let mut state = dawl_tui::DiagramState::default();
    for line in source.lines().filter(|line| !line.trim().is_empty()) {
        state.apply_json(line)?;
    }
    let layout = dawl_tui::layout_diagram(&diagram, &Default::default())?;
    let grid = dawl_tui::render_diagram(&diagram, &layout, &state)?;
    let body = if headless { dawl_tui::export::text(&grid) } else { dawl_tui::export::ansi(&grid) };
    print!("{body}");
    if source.is_empty() {
        return Err(Error::input("EVENT_EMPTY", "event stream is empty"));
    }
    Ok(())
}
