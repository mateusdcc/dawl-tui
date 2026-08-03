#![forbid(unsafe_code)]

pub mod canvas;
pub mod error;
pub mod export;
pub mod input;
pub mod layout;
pub mod model;
pub mod parser;
pub mod render;
pub mod route;
pub mod state;
pub mod theme;

pub use error::{Error, Result};
pub use input::load_diagram;
pub use layout::{layout_diagram, Layout, LayoutOptions};
pub use model::Diagram;
pub use render::render_diagram;
pub use state::DiagramState;
