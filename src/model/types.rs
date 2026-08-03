use serde::{Deserialize, Serialize};

use super::kinds::{Axis, Direction, EdgeKind, GroupKind, NodeKind, Point, Port, Relation, Size, TextKind};

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct Diagram {
    #[serde(default = "schema")]
    pub schema: String,
    #[serde(default)]
    pub id: String,
    pub title: String,
    #[serde(default)]
    pub viewport: Viewport,
    #[serde(default = "theme")]
    pub theme: String,
    #[serde(default)]
    pub direction: Direction,
    #[serde(default)]
    pub nodes: Vec<Node>,
    #[serde(default)]
    pub edges: Vec<Edge>,
    #[serde(default)]
    pub groups: Vec<Group>,
    #[serde(default)]
    pub texts: Vec<TextItem>,
    #[serde(default)]
    pub constraints: Vec<Constraint>,
}

#[derive(Clone, Copy, Debug, Default, Deserialize, Serialize)]
pub struct Viewport {
    pub width: u16,
    pub height: u16,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct Node {
    pub id: String,
    pub label: String,
    #[serde(default)]
    pub kind: NodeKind,
    #[serde(default, alias = "groupId")]
    pub group: Option<String>,
    #[serde(default)]
    pub at: Option<Point>,
    #[serde(default)]
    pub size: Option<Size>,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct Edge {
    pub id: String,
    pub from: String,
    pub to: String,
    #[serde(default)]
    pub label: String,
    #[serde(default)]
    pub kind: EdgeKind,
    #[serde(default)]
    pub via: Vec<Point>,
    #[serde(default)]
    pub from_port: Option<Port>,
    #[serde(default)]
    pub to_port: Option<Port>,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct Group {
    pub id: String,
    pub label: String,
    #[serde(default)]
    pub kind: GroupKind,
    #[serde(default, alias = "parentId")]
    pub parent: Option<String>,
    #[serde(default)]
    pub at: Option<Point>,
    #[serde(default)]
    pub size: Option<Size>,
    #[serde(default)]
    pub dashed: bool,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct TextItem {
    pub id: String,
    pub text: String,
    pub at: Point,
    #[serde(default)]
    pub kind: TextKind,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(tag = "type", rename_all = "snake_case")]
pub enum Constraint {
    Align { axis: Axis, ids: Vec<String> },
    Place { first: String, relation: Relation, second: String },
}

impl Default for Diagram {
    fn default() -> Self {
        Self {
            schema: schema(), id: String::new(), title: String::new(),
            viewport: Viewport::default(), theme: theme(),
            direction: Direction::default(), nodes: vec![], edges: vec![],
            groups: vec![], texts: vec![], constraints: vec![],
        }
    }
}

fn schema() -> String { "dawl.diagram/v1".into() }
fn theme() -> String { "midnight".into() }
