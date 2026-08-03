use std::path::{Path, PathBuf};

use dawl_tui::error::{Error, Result};

pub fn view(path: &Path) -> Result<()> {
    let diagram = dawl_tui::load_diagram(path)?;
    let layout = dawl_tui::layout_diagram(&diagram, &Default::default())?;
    let grid = dawl_tui::render_diagram(&diagram, &layout, &Default::default())?;
    super::tui::show(&grid, &diagram.title)
}

pub fn watch(graph: PathBuf, events: PathBuf, headless: bool) -> Result<()> {
    let diagram = dawl_tui::load_diagram(&graph)?;
    let source = std::fs::read_to_string(events)?;
    if source.trim().is_empty() { return Err(Error::input("EVENT_EMPTY", "event stream is empty")); }
    let state = apply_events(&source, &diagram)?;
    let layout = dawl_tui::layout_diagram(&diagram, &Default::default())?;
    let grid = dawl_tui::render_diagram(&diagram, &layout, &state)?;
    if headless { print!("{}", dawl_tui::export::text(&grid)); return Ok(()); }
    super::tui::show(&grid, &diagram.title)
}

fn apply_events(source: &str, graph: &dawl_tui::Diagram) -> Result<dawl_tui::DiagramState> {
    let mut state = dawl_tui::DiagramState::default();
    for line in source.lines().filter(|line| !line.trim().is_empty()) {
        state.apply_json_with_graph(line, graph)?;
    }
    Ok(state)
}
