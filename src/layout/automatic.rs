use std::collections::HashMap;

use indexmap::IndexMap;

use crate::canvas::Rect;
use crate::error::{Error, Result};
use crate::model::{Diagram, EdgeKind};

pub(super) fn place(diagram: &Diagram) -> Result<IndexMap<String, Rect>> {
    assign_ranks(diagram)?;
    let graph = super::topology::layout(diagram);
    super::topology::node_rects(diagram, &graph)
}

fn assign_ranks(diagram: &Diagram) -> Result<HashMap<String, usize>> {
    if diagram.nodes.is_empty() {
        return Ok(HashMap::new());
    }
    let mut ranks = diagram
        .nodes
        .iter()
        .map(|node| (node.id.clone(), 0))
        .collect();
    for _ in 0..diagram.nodes.len() {
        let changed = relax_all(diagram, &mut ranks);
        if !changed {
            return Ok(ranks);
        }
    }
    Err(
        Error::layout("LAYOUT_CYCLE", "forward edges contain a cycle")
            .hint("mark retry or feedback edges as kind back"),
    )
}

fn relax_all(diagram: &Diagram, ranks: &mut HashMap<String, usize>) -> bool {
    let mut changed = false;
    for edge in diagram
        .edges
        .iter()
        .filter(|edge| edge.kind != EdgeKind::Back)
    {
        changed |= relax(ranks, &edge.from, &edge.to);
    }
    changed
}

fn relax(ranks: &mut HashMap<String, usize>, from: &str, to: &str) -> bool {
    let next = ranks.get(from).copied().unwrap_or(0).saturating_add(1);
    let current = ranks.get(to).copied().unwrap_or(0);
    if next <= current {
        return false;
    }
    ranks.insert(to.into(), next);
    true
}
