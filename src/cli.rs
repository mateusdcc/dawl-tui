use clap::{Parser, Subcommand, ValueEnum};
use std::path::PathBuf;

use dawl_tui::error::Result;

#[derive(Parser)]
#[command(name = "dawl-tui", version, about)]
struct Cli {
    #[command(subcommand)]
    command: Command,
}

#[derive(Subcommand)]
enum Command {
    Render(RenderArgs),
    Check { input: PathBuf },
    View { input: PathBuf },
    Watch(WatchArgs),
}

#[derive(clap::Args)]
struct RenderArgs {
    input: PathBuf,
    #[arg(long, value_enum, default_value_t = Format::Ansi)]
    format: Format,
    #[arg(long)]
    output: Option<PathBuf>,
    #[arg(long)]
    width: Option<u16>,
    #[arg(long)]
    height: Option<u16>,
}

#[derive(clap::Args)]
struct WatchArgs {
    #[arg(long)]
    graph: PathBuf,
    #[arg(long)]
    events: PathBuf,
    #[arg(long)]
    headless: bool,
}

#[derive(Clone, Copy, ValueEnum)]
enum Format {
    Text,
    Ansi,
    Svg,
}

pub fn run() -> Result<()> {
    match Cli::parse().command {
        Command::Render(args) => render(args),
        Command::Check { input } => check(input),
        Command::View { input } => super::interactive::view(&input),
        Command::Watch(args) => super::interactive::watch(args.graph, args.events, args.headless),
    }
}

fn check(path: PathBuf) -> Result<()> {
    dawl_tui::load_diagram(&path)?;
    println!("ok: {}", path.display());
    Ok(())
}

fn render(args: RenderArgs) -> Result<()> {
    let diagram = dawl_tui::load_diagram(&args.input)?;
    let options = dawl_tui::LayoutOptions::new(args.width, args.height);
    let layout = dawl_tui::layout_diagram(&diagram, &options)?;
    let grid = dawl_tui::render_diagram(&diagram, &layout, &Default::default())?;
    let body = export(&grid, args.format);
    write_output(args.output, &body)
}

fn export(grid: &dawl_tui::canvas::Grid, format: Format) -> String {
    match format {
        Format::Text => dawl_tui::export::text(grid),
        Format::Ansi => dawl_tui::export::ansi(grid),
        Format::Svg => dawl_tui::export::svg(grid, 6, 10),
    }
}

fn write_output(path: Option<PathBuf>, body: &str) -> Result<()> {
    if let Some(path) = path {
        std::fs::write(path, body)?;
    } else {
        print!("{body}");
    }
    Ok(())
}
