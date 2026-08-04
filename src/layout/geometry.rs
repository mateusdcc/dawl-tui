use indexmap::IndexMap;
use unicode_width::UnicodeWidthStr;

use super::LayoutOptions;
use crate::canvas::Rect;
use crate::model::{Diagram, Point, Size, TextItem};

pub const CANVAS_PADDING_X: u16 = 2;
pub const CANVAS_PADDING_Y: u16 = 2;
pub const GROUP_PADDING_X: u16 = 2;
pub const GROUP_PADDING_Y: u16 = 2;

pub fn measure(label: &str) -> Size {
    let content = text_size(label);
    Size {
        width: content.width.saturating_add(4),
        height: content.height.saturating_add(2),
    }
}

pub fn text_size(text: &str) -> Size {
    let width = text.lines().map(UnicodeWidthStr::width).max().unwrap_or(1);
    Size {
        width: cell(width),
        height: cell(text.lines().count().max(1)),
    }
}

pub fn rect(point: Point, size: Size) -> Rect {
    Rect {
        x: point.x,
        y: point.y,
        width: size.width,
        height: size.height,
    }
}

pub fn bounds(items: &[Rect]) -> Option<Rect> {
    let first = items.first()?;
    let (mut left, mut top) = (first.x, first.y);
    let (mut right, mut bottom) = (first.right(), first.bottom());
    for item in items.iter().skip(1) {
        left = left.min(item.x);
        top = top.min(item.y);
        right = right.max(item.right());
        bottom = bottom.max(item.bottom());
    }
    Some(Rect {
        x: left,
        y: top,
        width: right - left + 1,
        height: bottom - top + 1,
    })
}

pub fn group_box(children: &[Rect], label: &str) -> Option<Rect> {
    let mut result = pad(bounds(children)?, GROUP_PADDING_X, GROUP_PADDING_Y);
    result.width = result.width.max(measure(label).width);
    Some(result)
}

pub fn canvas_size(
    diagram: &Diagram,
    options: &LayoutOptions,
    nodes: &IndexMap<String, Rect>,
    groups: &IndexMap<String, Rect>,
) -> Size {
    let extent = bounds(&content_rects(diagram, nodes, groups)).unwrap_or_default();
    let width = intrinsic_width(diagram, extent);
    let height = intrinsic_height(diagram, extent);
    Size {
        width: options
            .width
            .unwrap_or(width.max(diagram.viewport.width))
            .max(20),
        height: options
            .height
            .unwrap_or(height.max(diagram.viewport.height))
            .max(8),
    }
}

fn intrinsic_width(diagram: &Diagram, extent: Rect) -> u16 {
    let padding = if diagram.viewport.width == 0 {
        CANVAS_PADDING_X + 1
    } else {
        CANVAS_PADDING_X
    };
    extent.right().saturating_add(padding)
}

fn intrinsic_height(diagram: &Diagram, extent: Rect) -> u16 {
    let padding = if diagram.viewport.height == 0 {
        CANVAS_PADDING_Y + 1
    } else {
        CANVAS_PADDING_Y
    };
    extent.bottom().saturating_add(padding)
}

fn content_rects(
    diagram: &Diagram,
    nodes: &IndexMap<String, Rect>,
    groups: &IndexMap<String, Rect>,
) -> Vec<Rect> {
    let mut items = nodes
        .values()
        .chain(groups.values())
        .copied()
        .collect::<Vec<_>>();
    items.extend(diagram.texts.iter().map(text_rect));
    items.push(rect(Point { x: 0, y: 0 }, text_size(&diagram.title)));
    items
}

fn text_rect(item: &TextItem) -> Rect {
    rect(item.at, text_size(&item.text))
}

fn pad(value: Rect, x: u16, y: u16) -> Rect {
    Rect {
        x: value.x.saturating_sub(x),
        y: value.y.saturating_sub(y),
        width: value.width.saturating_add(x * 2),
        height: value.height.saturating_add(y * 2),
    }
}

fn cell(value: usize) -> u16 {
    u16::try_from(value).unwrap_or(u16::MAX)
}
