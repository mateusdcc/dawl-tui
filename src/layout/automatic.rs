use std::collections::HashMap;

use indexmap::IndexMap;

use super::place::place_layers;
use crate::canvas::Rect;
use crate::error::{Error, Result};
use crate::model::{Diagram, EdgeKind};

pub(super) fn place(diagram: &Diagram, nodes: &mut IndexMap<String, Rect>) -> Result<()> {
    let ranks = assign_ranks(diagram)?;
    let mut layers = build_layers(diagram, &ranks);
    reduce_crossings(diagram, &mut layers);
    place_layers(diagram, &layers, nodes);
    Ok(())
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

fn build_layers(diagram: &Diagram, ranks: &HashMap<String, usize>) -> Vec<Vec<String>> {
    let count = ranks.values().copied().max().unwrap_or(0).saturating_add(1);
    let mut layers = vec![Vec::new(); count];
    for node in &diagram.nodes {
        let rank = ranks.get(&node.id).copied().unwrap_or(0);
        layers[rank].push(node.id.clone());
    }
    layers
}

fn reduce_crossings(diagram: &Diagram, layers: &mut [Vec<String>]) {
    for _ in 0..4 {
        sweep_forward(diagram, layers);
        sweep_backward(diagram, layers);
    }
}

fn sweep_forward(diagram: &Diagram, layers: &mut [Vec<String>]) {
    for rank in 1..layers.len() {
        let positions = positions(&layers[rank - 1]);
        order_layer(diagram, &mut layers[rank], &positions, true);
    }
}

fn sweep_backward(diagram: &Diagram, layers: &mut [Vec<String>]) {
    for rank in (0..layers.len().saturating_sub(1)).rev() {
        let positions = positions(&layers[rank + 1]);
        order_layer(diagram, &mut layers[rank], &positions, false);
    }
}

fn positions(layer: &[String]) -> HashMap<String, usize> {
    layer
        .iter()
        .cloned()
        .enumerate()
        .map(|(index, id)| (id, index))
        .collect()
}

fn order_layer(
    diagram: &Diagram,
    layer: &mut [String],
    adjacent: &HashMap<String, usize>,
    incoming: bool,
) {
    let original = positions(layer);
    layer.sort_by_key(|id| order_key(diagram, id, adjacent, &original, incoming));
}

fn order_key(
    diagram: &Diagram,
    id: &str,
    adjacent: &HashMap<String, usize>,
    original: &HashMap<String, usize>,
    incoming: bool,
) -> (bool, usize, usize) {
    let mut values = neighbor_positions(diagram, id, adjacent, incoming);
    values.sort_unstable();
    let empty = values.is_empty();
    let median = values.get(values.len() / 2).copied().unwrap_or(0);
    (empty, median, original.get(id).copied().unwrap_or(0))
}

fn neighbor_positions(
    diagram: &Diagram,
    id: &str,
    adjacent: &HashMap<String, usize>,
    incoming: bool,
) -> Vec<usize> {
    diagram
        .edges
        .iter()
        .filter(|edge| edge.kind != EdgeKind::Back)
        .filter_map(|edge| neighbor(edge, id, adjacent, incoming))
        .collect()
}

fn neighbor(
    edge: &crate::model::Edge,
    id: &str,
    adjacent: &HashMap<String, usize>,
    incoming: bool,
) -> Option<usize> {
    if incoming && edge.to == id {
        return adjacent.get(&edge.from).copied();
    }
    if !incoming && edge.from == id {
        return adjacent.get(&edge.to).copied();
    }
    None
}
