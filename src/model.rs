use std::collections::HashSet;

use serde::{Deserialize, Serialize};

use crate::error::{Error, Result};

#[derive(Clone, Debug, Default, Deserialize, Serialize)]
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

#[derive(Clone, Copy, Debug, Default, Deserialize, Serialize)]
#[serde(rename_all = "snake_case")]
pub enum Direction {
    Down,
    #[default]
    Right,
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

#[derive(Clone, Copy, Debug, Default, Deserialize, Serialize)]
#[serde(rename_all = "snake_case")]
pub enum GroupKind {
    Parallel,
    Lane,
    Repeat,
    Scope,
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
pub enum Port { North, East, South, West }

#[derive(Clone, Copy, Debug, Deserialize, Serialize)]
#[serde(rename_all = "snake_case")]
pub enum Axis { Horizontal, Vertical }

#[derive(Clone, Copy, Debug, Deserialize, Serialize)]
#[serde(rename_all = "snake_case")]
pub enum Relation { Before, After, Above, Below }

impl Diagram {
    pub fn validate(&self) -> Result<()> {
        self.validate_schema()?;
        let groups = unique(self.groups.iter().map(|item| item.id.as_str()), "group")?;
        let nodes = unique(self.nodes.iter().map(|item| item.id.as_str()), "node")?;
        self.validate_groups(&groups)?;
        self.validate_nodes(&groups)?;
        self.validate_edges(&nodes)
    }

    fn validate_schema(&self) -> Result<()> {
        if self.schema == "dawl.diagram/v1" { return Ok(()); }
        Err(Error::input("MODEL_SCHEMA", format!("unsupported schema {}", self.schema)))
    }

    fn validate_groups(&self, groups: &HashSet<&str>) -> Result<()> {
        for group in &self.groups {
            if group.parent.as_deref().is_some_and(|id| !groups.contains(id)) {
                return Err(Error::input("MODEL_UNKNOWN_GROUP", group.id.clone()));
            }
        }
        Ok(())
    }

    fn validate_nodes(&self, groups: &HashSet<&str>) -> Result<()> {
        for node in &self.nodes {
            if node.group.as_deref().is_some_and(|id| !groups.contains(id)) {
                return Err(Error::input("MODEL_UNKNOWN_GROUP", node.id.clone()));
            }
        }
        Ok(())
    }

    fn validate_edges(&self, nodes: &HashSet<&str>) -> Result<()> {
        for edge in &self.edges {
            if !nodes.contains(edge.from.as_str()) || !nodes.contains(edge.to.as_str()) {
                return Err(Error::input("MODEL_UNKNOWN_NODE", edge.id.clone()));
            }
        }
        Ok(())
    }
}

fn unique<'a>(values: impl Iterator<Item = &'a str>, kind: &str) -> Result<HashSet<&'a str>> {
    let mut seen = HashSet::new();
    for value in values {
        if !seen.insert(value) {
            return Err(Error::input("MODEL_DUPLICATE_ID", format!("duplicate {kind} id {value}")));
        }
    }
    Ok(seen)
}

fn schema() -> String { "dawl.diagram/v1".into() }
fn theme() -> String { "midnight".into() }
