use crate::error::{Error, Result};
use crate::model::*;

pub fn tokenize(line: &str) -> Result<Vec<String>> {
    let mut tokens = Vec::new();
    let mut chars = line.chars().peekable();
    while chars.peek().is_some() {
        skip_space(&mut chars);
        if chars.peek().is_none() {
            break;
        }
        tokens.push(read_token(&mut chars)?);
    }
    Ok(tokens)
}

fn read_token(chars: &mut std::iter::Peekable<std::str::Chars<'_>>) -> Result<String> {
    if chars.peek() == Some(&'"') {
        return read_string(chars);
    }
    let mut value = String::new();
    while chars
        .peek()
        .is_some_and(|ch| !ch.is_whitespace() && *ch != '{' && *ch != '}')
    {
        if let Some(ch) = chars.next() {
            value.push(ch);
        }
    }
    Ok(value)
}

fn read_string(chars: &mut std::iter::Peekable<std::str::Chars<'_>>) -> Result<String> {
    chars.next();
    let mut value = String::new();
    while let Some(ch) = chars.next() {
        if ch == '"' {
            return Ok(value);
        }
        if ch == '\\' {
            value.push(read_escape(chars)?);
        } else {
            value.push(ch);
        }
    }
    Err(Error::input("SYNTAX_STRING", "unterminated string"))
}

fn read_escape(chars: &mut std::iter::Peekable<std::str::Chars<'_>>) -> Result<char> {
    match chars.next() {
        Some('n') => Ok('\n'),
        Some('t') => Ok('\t'),
        Some('"') => Ok('"'),
        Some('\\') => Ok('\\'),
        Some(value) => Ok(value),
        None => Err(Error::input("SYNTAX_ESCAPE", "unfinished escape")),
    }
}

fn skip_space(chars: &mut std::iter::Peekable<std::str::Chars<'_>>) {
    while chars.peek().is_some_and(|ch| ch.is_whitespace()) {
        chars.next();
    }
}

pub fn strip_comment(line: &str) -> &str {
    line.split_once('#').map_or(line, |item| item.0)
}

pub fn at_line(error: Error, line: usize) -> Error {
    Error::input(error.code, format!("line {line}: {}", error.message)).hint(
        error
            .hint
            .unwrap_or_else(|| "check the .dtui syntax".into()),
    )
}

pub fn finish(diagram: Diagram) -> Result<Diagram> {
    if diagram.title.is_empty() {
        Err(Error::input("SYNTAX_DIAGRAM", "missing diagram header"))
    } else {
        Ok(diagram)
    }
}

pub fn require(tokens: &[String], count: usize, usage: &str) -> Result<()> {
    if tokens.len() >= count {
        Ok(())
    } else {
        Err(Error::input("SYNTAX_ARITY", usage))
    }
}

pub fn option_value(tokens: &[String], key: &str) -> Option<String> {
    position(tokens, key)
        .and_then(|i| tokens.get(i + 1))
        .cloned()
}

pub fn position(tokens: &[String], key: &str) -> Option<usize> {
    tokens.iter().position(|item| item == key)
}

pub fn point_value(tokens: &[String], key: &str) -> Result<Option<Point>> {
    option_value(tokens, key)
        .map(|v| parse_point(&v))
        .transpose()
}

pub fn size_value(tokens: &[String], key: &str) -> Result<Option<Size>> {
    option_value(tokens, key)
        .map(|v| parse_size(&v))
        .transpose()
}

pub fn port_value(tokens: &[String], key: &str) -> Result<Option<Port>> {
    option_value(tokens, key).map(|v| port(&v)).transpose()
}

pub fn parse_point(value: &str) -> Result<Point> {
    let (x, y) = pair(value, ',')?;
    Ok(Point { x, y })
}

pub fn parse_size(value: &str) -> Result<Size> {
    let (width, height) = pair(value, 'x')?;
    Ok(Size { width, height })
}

fn pair(value: &str, separator: char) -> Result<(u16, u16)> {
    let (a, b) = value
        .split_once(separator)
        .ok_or_else(|| Error::input("SYNTAX_PAIR", value))?;
    Ok((number(a)?, number(b)?))
}

fn number(value: &str) -> Result<u16> {
    value
        .parse()
        .map_err(|_| Error::input("SYNTAX_NUMBER", value))
}

pub fn points_after(tokens: &[String], key: &str) -> Result<Vec<Point>> {
    let Some(start) = position(tokens, key) else {
        return Ok(vec![]);
    };
    tokens[start + 1..]
        .iter()
        .take_while(|v| v.contains(','))
        .map(|v| parse_point(v))
        .collect()
}

pub fn enum_value<T: Default>(
    tokens: &[String],
    key: &str,
    parse: fn(&str) -> Result<T>,
) -> Result<T> {
    option_value(tokens, key)
        .map(|v| parse(&v))
        .transpose()
        .map(|v| v.unwrap_or_default())
}

pub fn enum_value_or<T>(
    tokens: &[String],
    key: &str,
    parse: fn(&str) -> Result<T>,
    fallback: T,
) -> Result<T> {
    option_value(tokens, key)
        .map(|v| parse(&v))
        .transpose()
        .map(|v| v.unwrap_or(fallback))
}

pub fn node_kind(value: &str) -> Result<NodeKind> {
    serde_json::from_str(&format!("\"{value}\"")).map_err(Into::into)
}

pub fn edge_kind(value: &str) -> Result<EdgeKind> {
    serde_json::from_str(&format!("\"{value}\"")).map_err(Into::into)
}

pub fn group_kind(value: &str) -> Result<GroupKind> {
    serde_json::from_str(&format!("\"{value}\"")).map_err(Into::into)
}

pub fn text_kind(value: &str) -> Result<TextKind> {
    serde_json::from_str(&format!("\"{value}\"")).map_err(Into::into)
}

pub fn port(value: &str) -> Result<Port> {
    serde_json::from_str(&format!("\"{value}\"")).map_err(Into::into)
}

pub fn relation(value: &str) -> Result<Relation> {
    serde_json::from_str(&format!("\"{value}\"")).map_err(Into::into)
}
