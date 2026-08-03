use serde::{Deserialize, Serialize};

#[derive(Clone, Copy, Debug, Default, Deserialize, Serialize)]
#[serde(rename_all = "snake_case")]
pub enum Direction {
    Down,
    #[default]
    Right,
}

#[derive(Clone, Copy, Debug, Default, Deserialize, Serialize, Eq, PartialEq)]
#[serde(rename_all = "snake_case")]
pub enum NodeKind {
    Agent,
    Reviewer,
    Decision,
    Phase,
    Input,
    Output,
    Success,
    Failure,
    Shell,
    Join,
    Fork,
    Return,
    Value,
    #[default]
    Activity,
}

#[derive(Clone, Copy, Debug, Default, Deserialize, Serialize, Eq, PartialEq)]
#[serde(rename_all = "snake_case")]
pub enum EdgeKind {
    Success,
    Failure,
    Back,
    Muted,
    #[default]
    Forward,
}

#[derive(Clone, Copy, Debug, Default, Deserialize, Serialize, Eq, PartialEq)]
#[serde(rename_all = "snake_case")]
pub enum GroupKind {
    Parallel,
    Lane,
    Repeat,
    Scope,
    Function,
    Panel,
    #[default]
    Group,
}

#[derive(Clone, Copy, Debug, Default, Deserialize, Serialize)]
#[serde(rename_all = "snake_case")]
pub enum TextKind {
    Title,
    Metric,
    Dim,
    #[default]
    Plain,
}

#[derive(Clone, Copy, Debug, Default, Deserialize, Serialize, Eq, PartialEq, Hash)]
pub struct Point {
    pub x: u16,
    pub y: u16,
}

#[derive(Clone, Copy, Debug, Default, Deserialize, Serialize, Eq, PartialEq)]
pub struct Size {
    pub width: u16,
    pub height: u16,
}

#[derive(Clone, Copy, Debug, Deserialize, Serialize)]
#[serde(rename_all = "snake_case")]
pub enum Port {
    North,
    East,
    South,
    West,
}

#[derive(Clone, Copy, Debug, Deserialize, Serialize)]
#[serde(rename_all = "snake_case")]
pub enum Axis {
    Horizontal,
    Vertical,
}

#[derive(Clone, Copy, Debug, Deserialize, Serialize)]
#[serde(rename_all = "snake_case")]
pub enum Relation {
    Before,
    After,
    Above,
    Below,
}
