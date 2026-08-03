use crate::model::{EdgeKind, GroupKind, NodeKind, TextKind};
use crate::state::Status;
use crate::theme::Style;

pub fn group_style(kind: GroupKind) -> Style {
    match kind {
        GroupKind::Panel | GroupKind::Lane => Style::Structure,
        _ => Style::Group,
    }
}

pub fn node_style(kind: NodeKind, status: Status) -> Style {
    match status {
        Status::Running => Style::Running,
        Status::Succeeded => Style::Success,
        Status::Failed => Style::Failure,
        Status::Idle => kind_style(kind),
    }
}

pub fn kind_style(kind: NodeKind) -> Style {
    match kind {
        NodeKind::Agent => Style::Agent,
        NodeKind::Reviewer => Style::Reviewer,
        NodeKind::Decision => Style::Decision,
        NodeKind::Phase => Style::Phase,
        NodeKind::Input => Style::Input,
        NodeKind::Output => Style::Output,
        NodeKind::Success => Style::Success,
        NodeKind::Failure => Style::Failure,
        NodeKind::Shell => Style::Shell,
        NodeKind::Join | NodeKind::Fork => Style::Join,
        NodeKind::Return | NodeKind::Value | NodeKind::Activity => Style::Plain,
    }
}

pub fn edge_style(kind: EdgeKind, status: Status) -> Style {
    match status {
        Status::Running => Style::Running,
        Status::Succeeded => Style::Success,
        Status::Failed => Style::Failure,
        Status::Idle => edge_kind_style(kind),
    }
}

pub fn edge_kind_style(kind: EdgeKind) -> Style {
    match kind {
        EdgeKind::Success => Style::Success,
        EdgeKind::Failure | EdgeKind::Back => Style::Failure,
        EdgeKind::Muted => Style::Muted,
        EdgeKind::Forward => Style::Edge,
    }
}

pub fn text_style(kind: TextKind) -> Style {
    match kind {
        TextKind::Title => Style::Title,
        TextKind::Metric => Style::Metric,
        TextKind::Dim => Style::Dim,
        TextKind::Plain => Style::Plain,
    }
}
