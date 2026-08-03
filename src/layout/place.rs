use indexmap::IndexMap;

use crate::canvas::Rect;
use crate::model::{Diagram, Direction, Size};

pub fn place_layers(diagram: &Diagram, layers: &[Vec<String>], nodes: &mut IndexMap<String, Rect>) {
    match diagram.direction {
        Direction::Right => place_right(diagram, layers, nodes),
        Direction::Down => place_down(diagram, layers, nodes),
    }
}

fn place_right(diagram: &Diagram, layers: &[Vec<String>], nodes: &mut IndexMap<String, Rect>) {
    let mut x = 2;
    for layer in layers {
        let mut y = 4;
        for id in layer {
            let size = node_size(diagram, id);
            nodes.entry(id.clone()).or_insert(Rect {
                x,
                y,
                width: size.width,
                height: size.height,
            });
            y = y.saturating_add(size.height).saturating_add(2);
        }
        x = x
            .saturating_add(layer_width(diagram, layer))
            .saturating_add(6);
    }
}

fn place_down(diagram: &Diagram, layers: &[Vec<String>], nodes: &mut IndexMap<String, Rect>) {
    let mut y = 3;
    for layer in layers {
        let mut x = 2;
        for id in layer {
            let size = node_size(diagram, id);
            nodes.entry(id.clone()).or_insert(Rect {
                x,
                y,
                width: size.width,
                height: size.height,
            });
            x = x.saturating_add(size.width).saturating_add(4);
        }
        y = y
            .saturating_add(layer_height(diagram, layer))
            .saturating_add(3);
    }
}

fn node_size(diagram: &Diagram, id: &str) -> Size {
    diagram
        .nodes
        .iter()
        .find(|node| node.id == id)
        .map(|node| node.size.unwrap_or_else(|| super::measure(&node.label)))
        .unwrap_or(Size {
            width: 5,
            height: 3,
        })
}

fn layer_width(diagram: &Diagram, layer: &[String]) -> u16 {
    layer
        .iter()
        .map(|id| node_size(diagram, id).width)
        .max()
        .unwrap_or(1)
}

fn layer_height(diagram: &Diagram, layer: &[String]) -> u16 {
    layer
        .iter()
        .map(|id| node_size(diagram, id).height)
        .max()
        .unwrap_or(1)
}
