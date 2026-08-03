mod utils;

use crate::error::{Error, Result};
use crate::model::*;

use self::utils::{
    at_line, edge_kind, enum_value, enum_value_or, finish, group_kind, node_kind, option_value,
    parse_size, point_value, points_after, port_value, relation, require, size_value,
    strip_comment, text_kind, tokenize,
};

pub fn parse(source: &str) -> Result<Diagram> {
    let mut diagram = Diagram::default();
    for (index, raw) in source.lines().enumerate() {
        let line = strip_comment(raw).trim();
        if line.is_empty() || line == "{" || line == "}" {
            continue;
        }
        parse_line(&mut diagram, line).map_err(|error| at_line(error, index + 1))?;
    }
    finish(diagram)
}

fn parse_line(diagram: &mut Diagram, line: &str) -> Result<()> {
    let tokens = tokenize(line)?;
    let head = tokens.first().map(String::as_str).unwrap_or_default();
    match head {
        "diagram" => parse_diagram(diagram, &tokens)?,
        "viewport" => parse_viewport(diagram, &tokens)?,
        "theme" => set_theme(diagram, &tokens)?,
        "direction" => set_direction(diagram, &tokens)?,
        "group" => diagram.groups.push(parse_group(&tokens)?),
        "node" | "decision" => diagram.nodes.push(parse_node(&tokens, head)?),
        "edge" => diagram.edges.push(parse_edge(&tokens)?),
        "text" => diagram.texts.push(parse_text(&tokens)?),
        "align" => diagram.constraints.push(parse_align(&tokens)?),
        "place" => diagram.constraints.push(parse_place(&tokens)?),
        _ => return Err(Error::input("SYNTAX_ITEM", format!("unknown item {head}"))),
    };
    Ok(())
}

fn parse_diagram(diagram: &mut Diagram, tokens: &[String]) -> Result<()> {
    require(tokens, 3, "diagram ID and title")?;
    diagram.id = tokens[1].clone();
    diagram.title = tokens[2].clone();
    Ok(())
}

fn parse_viewport(diagram: &mut Diagram, tokens: &[String]) -> Result<()> {
    require(tokens, 2, "viewport WIDTHxHEIGHT")?;
    diagram.viewport = parse_size(&tokens[1])?.into();
    Ok(())
}

fn set_theme(diagram: &mut Diagram, tokens: &[String]) -> Result<()> {
    require(tokens, 2, "theme NAME")?;
    diagram.theme = tokens[1].clone();
    Ok(())
}

fn set_direction(diagram: &mut Diagram, tokens: &[String]) -> Result<()> {
    require(tokens, 2, "direction right|down")?;
    diagram.direction = match tokens[1].as_str() {
        "right" => Direction::Right,
        "down" => Direction::Down,
        value => return Err(Error::input("SYNTAX_DIRECTION", value)),
    };
    Ok(())
}

fn parse_group(tokens: &[String]) -> Result<Group> {
    require(tokens, 3, "group ID LABEL")?;
    Ok(Group {
        id: tokens[1].clone(),
        label: tokens[2].clone(),
        kind: enum_value(tokens, "kind", group_kind)?,
        parent: option_value(tokens, "in"),
        at: point_value(tokens, "at")?,
        size: size_value(tokens, "size")?,
        dashed: tokens.iter().any(|v| v == "dashed"),
    })
}

fn parse_node(tokens: &[String], head: &str) -> Result<Node> {
    require(tokens, 3, "node ID LABEL")?;
    let fallback = if head == "decision" {
        NodeKind::Decision
    } else {
        NodeKind::Activity
    };
    Ok(Node {
        id: tokens[1].clone(),
        label: tokens[2].clone(),
        kind: enum_value_or(tokens, "kind", node_kind, fallback)?,
        group: option_value(tokens, "in"),
        at: point_value(tokens, "at")?,
        size: size_value(tokens, "size")?,
    })
}

fn parse_edge(tokens: &[String]) -> Result<Edge> {
    require(tokens, 5, "edge ID FROM -> TO")?;
    if tokens[3] != "->" {
        return Err(Error::input("SYNTAX_EDGE", "expected ->"));
    }
    Ok(Edge {
        id: tokens[1].clone(),
        from: tokens[2].clone(),
        to: tokens[4].clone(),
        label: option_value(tokens, "label").unwrap_or_default(),
        kind: enum_value(tokens, "kind", edge_kind)?,
        via: points_after(tokens, "via")?,
        from_port: port_value(tokens, "from_port")?,
        to_port: port_value(tokens, "to_port")?,
    })
}

fn parse_text(tokens: &[String]) -> Result<TextItem> {
    require(tokens, 3, "text ID TEXT")?;
    let at =
        point_value(tokens, "at")?.ok_or_else(|| Error::input("SYNTAX_AT", "text needs at"))?;
    Ok(TextItem {
        id: tokens[1].clone(),
        text: tokens[2].clone(),
        at,
        kind: enum_value(tokens, "kind", text_kind)?,
    })
}

fn parse_align(tokens: &[String]) -> Result<Constraint> {
    require(tokens, 4, "align horizontal|vertical ID...")?;
    let axis = match tokens[1].as_str() {
        "horizontal" => Axis::Horizontal,
        "vertical" => Axis::Vertical,
        value => return Err(Error::input("SYNTAX_AXIS", value)),
    };
    Ok(Constraint::Align {
        axis,
        ids: tokens[2..].to_vec(),
    })
}

fn parse_place(tokens: &[String]) -> Result<Constraint> {
    require(tokens, 4, "place A RELATION B")?;
    let relation = relation(&tokens[2])?;
    Ok(Constraint::Place {
        first: tokens[1].clone(),
        relation,
        second: tokens[3].clone(),
    })
}

impl From<Size> for Viewport {
    fn from(value: Size) -> Self {
        Self {
            width: value.width,
            height: value.height,
        }
    }
}
