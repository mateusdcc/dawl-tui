#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct Rgb {
    pub red: u8,
    pub green: u8,
    pub blue: u8,
}

#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub enum Style {
    Background,
    Structure,
    Group,
    Edge,
    Agent,
    Reviewer,
    Decision,
    Phase,
    Input,
    Output,
    Success,
    Failure,
    Running,
    Dim,
    Title,
    Metric,
    Shell,
    Join,
    Muted,
    #[default]
    Plain,
}

#[derive(Clone, Copy, Debug)]
pub struct Palette {
    background: Rgb,
}

impl Rgb {
    pub const fn new(red: u8, green: u8, blue: u8) -> Self {
        Self { red, green, blue }
    }
}

impl Palette {
    pub const fn midnight() -> Self {
        Self {
            background: Rgb::new(13, 27, 47),
        }
    }

    pub const fn background(self) -> Rgb {
        self.background
    }

    pub const fn foreground(self, style: Style) -> Rgb {
        match style {
            Style::Structure | Style::Group | Style::Phase | Style::Input | Style::Output => CYAN,
            Style::Agent => BLUE,
            Style::Reviewer => PURPLE,
            Style::Success => GREEN,
            Style::Failure => RED,
            Style::Running | Style::Metric => AMBER,
            Style::Dim | Style::Muted => DIM,
            Style::Title => WHITE,
            Style::Shell => TEAL,
            Style::Join | Style::Decision => LIGHT,
            Style::Background => self.background,
            Style::Edge | Style::Plain => LIGHT,
        }
    }
}

const CYAN: Rgb = Rgb::new(68, 211, 242);
const BLUE: Rgb = Rgb::new(91, 168, 255);
const PURPLE: Rgb = Rgb::new(180, 113, 255);
const GREEN: Rgb = Rgb::new(116, 226, 145);
const RED: Rgb = Rgb::new(255, 101, 111);
const AMBER: Rgb = Rgb::new(255, 190, 92);
const DIM: Rgb = Rgb::new(102, 130, 157);
const WHITE: Rgb = Rgb::new(235, 242, 250);
const LIGHT: Rgb = Rgb::new(197, 217, 235);
const TEAL: Rgb = Rgb::new(95, 210, 200);
