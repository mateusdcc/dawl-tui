use std::io::Read;
use std::path::Path;

use serde_json::Value;

use crate::error::{Error, Result};
use crate::model::Diagram;

pub fn load_diagram(path: &Path) -> Result<Diagram> {
    let source = read_source(path)?;
    let diagram = parse_source(path, &source)?;
    diagram.validate()?;
    Ok(diagram)
}

fn read_source(path: &Path) -> Result<String> {
    if path == Path::new("-") {
        return read_stdin();
    }
    Ok(std::fs::read_to_string(path)?)
}

fn read_stdin() -> Result<String> {
    let mut source = String::new();
    std::io::stdin().read_to_string(&mut source)?;
    Ok(source)
}

pub fn parse_source(path: &Path, source: &str) -> Result<Diagram> {
    if is_json(path, source) {
        return parse_json(source);
    }
    crate::parser::parse(source)
}

fn is_json(path: &Path, source: &str) -> bool {
    let extension = path.extension().and_then(|item| item.to_str());
    matches!(extension, Some("json")) || source.trim_start().starts_with('{')
}

fn parse_json(source: &str) -> Result<Diagram> {
    let mut value: Value = serde_json::from_str(source)?;
    normalize(&mut value)?;
    let diagram: Diagram = serde_json::from_value(value)?;
    Ok(diagram)
}

fn normalize(value: &mut Value) -> Result<()> {
    let object = value.as_object_mut().ok_or_else(json_object_error)?;
    object
        .entry("schema")
        .or_insert(Value::String("dawl.diagram/v1".into()));
    object
        .entry("id")
        .or_insert(Value::String("diagram".into()));
    object
        .entry("theme")
        .or_insert(Value::String("midnight".into()));
    normalize_nodes(object.get_mut("nodes"));
    normalize_groups(object.get_mut("groups"));
    Ok(())
}

fn normalize_nodes(value: Option<&mut Value>) {
    for item in array_items(value) {
        rename(item, "groupId", "group");
    }
}

fn normalize_groups(value: Option<&mut Value>) {
    for item in array_items(value) {
        rename(item, "parentId", "parent");
    }
}

fn array_items(value: Option<&mut Value>) -> Vec<&mut serde_json::Map<String, Value>> {
    value
        .and_then(Value::as_array_mut)
        .into_iter()
        .flatten()
        .filter_map(Value::as_object_mut)
        .collect()
}

fn rename(object: &mut serde_json::Map<String, Value>, old: &str, new: &str) {
    if object.contains_key(new) {
        return;
    }
    if let Some(value) = object.remove(old) {
        object.insert(new.into(), value);
    }
}

fn json_object_error() -> Error {
    Error::input("JSON_ROOT", "diagram JSON root must be an object")
}
