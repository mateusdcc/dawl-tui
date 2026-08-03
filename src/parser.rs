use crate::error::{Error, Result};
use crate::model::*;

pub fn parse(source: &str) -> Result<Diagram> {
    let mut diagram = Diagram::default();
    for (index, raw) in source.lines().enumerate() {
        let line = strip_comment(raw).trim();
        if line.is_empty() || line == "{" || line == "}" { continue; }
        parse_line(&mut diagram, line).map_err(|error| at_line(error, index + 1))?;
    }
    finish(diagram)
}

fn parse_line(diagram: &mut Diagram, line: &str) -> Result<()> {
    let tokens = tokenize(line)?;
    let head = tokens.first().map(String::as_str).unwrap_or_default();
    match head {
        "diagram" => parse_diagram(diagram, &tokens),
        "viewport" => parse_viewport(diagram, &tokens),
        "theme" => set_theme(diagram, &tokens),
        "direction" => set_direction(diagram, &tokens),
        "group" => diagram.groups.push(parse_group(&tokens)?),
        "node" | "decision" => diagram.nodes.push(parse_node(&tokens, head)?),
        "edge" => diagram.edges.push(parse_edge(&tokens)?),
        "text" => diagram.texts.push(parse_text(&tokens)?),
        "align" => diagram.constraints.push(parse_align(&tokens)?),
        "place" => diagram.constraints.push(parse_place(&tokens)?),
        _ => return Err(Error::input("SYNTAX_ITEM", format!("unknown item {head}"))),
    }
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
        id: tokens[1].clone(), label: tokens[2].clone(),
        kind: enum_value(tokens, "kind", group_kind)?,
        parent: option_value(tokens, "in"), at: point_value(tokens, "at")?,
        size: size_value(tokens, "size")?, dashed: tokens.iter().any(|v| v == "dashed"),
    })
}

fn parse_node(tokens: &[String], head: &str) -> Result<Node> {
    require(tokens, 3, "node ID LABEL")?;
    let fallback = if head == "decision" { NodeKind::Decision } else { NodeKind::Activity };
    Ok(Node {
        id: tokens[1].clone(), label: tokens[2].clone(),
        kind: enum_value_or(tokens, "kind", node_kind, fallback)?,
        group: option_value(tokens, "in"), at: point_value(tokens, "at")?,
        size: size_value(tokens, "size")?,
    })
}

fn parse_edge(tokens: &[String]) -> Result<Edge> {
    require(tokens, 5, "edge ID FROM -> TO")?;
    if tokens[3] != "->" { return Err(Error::input("SYNTAX_EDGE", "expected ->")); }
    Ok(Edge {
        id: tokens[1].clone(), from: tokens[2].clone(), to: tokens[4].clone(),
        label: option_value(tokens, "label").unwrap_or_default(),
        kind: enum_value(tokens, "kind", edge_kind)?, via: points_after(tokens, "via")?,
        from_port: port_value(tokens, "from_port")?, to_port: port_value(tokens, "to_port")?,
    })
}

fn parse_text(tokens: &[String]) -> Result<TextItem> {
    require(tokens, 3, "text ID TEXT")?;
    let at = point_value(tokens, "at")?.ok_or_else(|| Error::input("SYNTAX_AT", "text needs at"))?;
    Ok(TextItem { id: tokens[1].clone(), text: tokens[2].clone(), at,
        kind: enum_value(tokens, "kind", text_kind)? })
}

fn parse_align(tokens: &[String]) -> Result<Constraint> {
    require(tokens, 4, "align horizontal|vertical ID...")?;
    let axis = match tokens[1].as_str() {
        "horizontal" => Axis::Horizontal,
        "vertical" => Axis::Vertical,
        value => return Err(Error::input("SYNTAX_AXIS", value)),
    };
    Ok(Constraint::Align { axis, ids: tokens[2..].to_vec() })
}

fn parse_place(tokens: &[String]) -> Result<Constraint> {
    require(tokens, 4, "place A RELATION B")?;
    let relation = relation(&tokens[2])?;
    Ok(Constraint::Place { first: tokens[1].clone(), relation, second: tokens[3].clone() })
}

fn tokenize(line: &str) -> Result<Vec<String>> {
    let mut tokens = Vec::new();
    let mut chars = line.chars().peekable();
    while chars.peek().is_some() {
        skip_space(&mut chars);
        if chars.peek().is_none() { break; }
        tokens.push(read_token(&mut chars)?);
    }
    Ok(tokens)
}

fn read_token(chars: &mut std::iter::Peekable<std::str::Chars<'_>>) -> Result<String> {
    if chars.peek() == Some(&'"') { return read_string(chars); }
    let mut value = String::new();
    while chars.peek().is_some_and(|ch| !ch.is_whitespace() && *ch != '{' && *ch != '}') {
        if let Some(ch) = chars.next() { value.push(ch); }
    }
    Ok(value)
}

fn read_string(chars: &mut std::iter::Peekable<std::str::Chars<'_>>) -> Result<String> {
    chars.next();
    let mut value = String::new();
    while let Some(ch) = chars.next() {
        if ch == '"' { return Ok(value); }
        if ch == '\\' { value.push(read_escape(chars)?); } else { value.push(ch); }
    }
    Err(Error::input("SYNTAX_STRING", "unterminated string"))
}

fn read_escape(chars: &mut std::iter::Peekable<std::str::Chars<'_>>) -> Result<char> {
    match chars.next() {
        Some('n') => Ok('\n'), Some('t') => Ok('\t'), Some('"') => Ok('"'),
        Some('\\') => Ok('\\'), Some(value) => Ok(value),
        None => Err(Error::input("SYNTAX_ESCAPE", "unfinished escape")),
    }
}

fn skip_space(chars: &mut std::iter::Peekable<std::str::Chars<'_>>) {
    while chars.peek().is_some_and(|ch| ch.is_whitespace()) { chars.next(); }
}

fn strip_comment(line: &str) -> &str { line.split_once('#').map_or(line, |item| item.0) }
fn at_line(error: Error, line: usize) -> Error { Error::input(error.code, format!("line {line}: {}", error.message)).hint(error.hint.unwrap_or_else(|| "check the .dtui syntax".into())) }
fn finish(diagram: Diagram) -> Result<Diagram> { if diagram.title.is_empty() { Err(Error::input("SYNTAX_DIAGRAM", "missing diagram header")) } else { Ok(diagram) } }
fn require(tokens: &[String], count: usize, usage: &str) -> Result<()> { if tokens.len() >= count { Ok(()) } else { Err(Error::input("SYNTAX_ARITY", usage)) } }
fn option_value(tokens: &[String], key: &str) -> Option<String> { position(tokens, key).and_then(|i| tokens.get(i + 1)).cloned() }
fn position(tokens: &[String], key: &str) -> Option<usize> { tokens.iter().position(|item| item == key) }
fn point_value(tokens: &[String], key: &str) -> Result<Option<Point>> { option_value(tokens, key).map(|v| parse_point(&v)).transpose() }
fn size_value(tokens: &[String], key: &str) -> Result<Option<Size>> { option_value(tokens, key).map(|v| parse_size(&v)).transpose() }
fn port_value(tokens: &[String], key: &str) -> Result<Option<Port>> { option_value(tokens, key).map(|v| port(&v)).transpose() }
fn parse_point(value: &str) -> Result<Point> { let (x, y) = pair(value, ',')?; Ok(Point { x, y }) }
fn parse_size(value: &str) -> Result<Size> { let (width, height) = pair(value, 'x')?; Ok(Size { width, height }) }
fn pair(value: &str, separator: char) -> Result<(u16, u16)> { let (a, b) = value.split_once(separator).ok_or_else(|| Error::input("SYNTAX_PAIR", value))?; Ok((number(a)?, number(b)?)) }
fn number(value: &str) -> Result<u16> { value.parse().map_err(|_| Error::input("SYNTAX_NUMBER", value)) }
fn points_after(tokens: &[String], key: &str) -> Result<Vec<Point>> { let Some(start) = position(tokens, key) else { return Ok(vec![]); }; tokens[start + 1..].iter().take_while(|v| v.contains(',')).map(|v| parse_point(v)).collect() }
fn enum_value<T: Default>(tokens: &[String], key: &str, parse: fn(&str) -> Result<T>) -> Result<T> { option_value(tokens, key).map(|v| parse(&v)).transpose().map(|v| v.unwrap_or_default()) }
fn enum_value_or<T>(tokens: &[String], key: &str, parse: fn(&str) -> Result<T>, fallback: T) -> Result<T> { option_value(tokens, key).map(|v| parse(&v)).transpose().map(|v| v.unwrap_or(fallback)) }
fn node_kind(value: &str) -> Result<NodeKind> { serde_json::from_str(&format!("\"{value}\"")).map_err(Into::into) }
fn edge_kind(value: &str) -> Result<EdgeKind> { serde_json::from_str(&format!("\"{value}\"")).map_err(Into::into) }
fn group_kind(value: &str) -> Result<GroupKind> { serde_json::from_str(&format!("\"{value}\"")).map_err(Into::into) }
fn text_kind(value: &str) -> Result<TextKind> { serde_json::from_str(&format!("\"{value}\"")).map_err(Into::into) }
fn port(value: &str) -> Result<Port> { serde_json::from_str(&format!("\"{value}\"")).map_err(Into::into) }
fn relation(value: &str) -> Result<Relation> { serde_json::from_str(&format!("\"{value}\"")).map_err(Into::into) }

impl From<Size> for Viewport { fn from(value: Size) -> Self { Self { width: value.width, height: value.height } } }
