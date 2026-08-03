use std::collections::HashSet;

use crate::error::{Error, Result};

use super::Diagram;

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
