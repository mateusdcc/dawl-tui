use dawl_tui::canvas::{ArrowDirection, Grid, Rect};
use dawl_tui::model::Point;
use dawl_tui::theme::Style;

#[test]
fn wide_glyph_consumes_two_cells() {
    let mut grid = Grid::new(5, 1);
    grid.write(Point { x: 0, y: 0 }, "界x", Style::Plain);
    assert_eq!(grid.visible_char(0, 0), '界');
    assert!(grid.cell(1, 0).unwrap().continuation);
    assert_eq!(grid.visible_char(2, 0), 'x');
}

#[test]
fn box_junctions_are_resolved() {
    let mut grid = Grid::new(6, 4);
    grid.draw_box(
        Rect {
            x: 0,
            y: 0,
            width: 6,
            height: 4,
        },
        Style::Group,
        false,
    );
    assert_eq!(grid.visible_char(0, 0), '┌');
    assert_eq!(grid.visible_char(5, 3), '┘');
}

#[test]
fn crossing_lines_render_a_cross() {
    let mut grid = Grid::new(5, 5);
    grid.draw_path(&[Point { x: 0, y: 2 }, Point { x: 4, y: 2 }], Style::Edge);
    grid.draw_path(&[Point { x: 2, y: 0 }, Point { x: 2, y: 4 }], Style::Edge);
    assert_eq!(grid.visible_char(2, 2), '┼');
}

#[test]
fn right_angle_path_renders_a_corner_instead_of_a_cross() {
    let mut grid = Grid::new(5, 5);
    grid.draw_path(
        &[
            Point { x: 0, y: 1 },
            Point { x: 2, y: 1 },
            Point { x: 2, y: 3 },
        ],
        Style::Edge,
    );
    assert_eq!(grid.visible_char(2, 1), '┐');
    assert_eq!(grid.visible_char(0, 1), '─');
    assert_eq!(grid.visible_char(2, 3), '│');
}

#[test]
fn route_crosses_a_group_border_without_creating_a_false_junction() {
    let mut grid = Grid::new(9, 7);
    grid.draw_box(
        Rect {
            x: 2,
            y: 1,
            width: 5,
            height: 5,
        },
        Style::Group,
        false,
    );
    grid.draw_path(&[Point { x: 0, y: 3 }, Point { x: 8, y: 3 }], Style::Edge);
    assert_eq!(grid.visible_char(2, 3), '─');
    assert_eq!(grid.visible_char(6, 3), '─');
}

#[test]
fn route_entering_a_node_border_renders_a_tee_port() {
    let mut grid = Grid::new(7, 5);
    grid.draw_path(&[Point { x: 0, y: 2 }, Point { x: 2, y: 2 }], Style::Edge);
    grid.draw_box(
        Rect {
            x: 2,
            y: 1,
            width: 4,
            height: 3,
        },
        Style::Agent,
        false,
    );
    assert_eq!(grid.visible_char(2, 2), '┤');
}

#[test]
fn decision_diamond_has_diagonal_terminal_geometry() {
    let mut grid = Grid::new(13, 7);
    grid.draw_diamond(
        Rect {
            x: 2,
            y: 1,
            width: 9,
            height: 5,
        },
        Style::Decision,
    );
    assert_eq!(grid.visible_char(4, 2), '╱');
    assert_eq!(grid.visible_char(8, 2), '╲');
    assert_eq!(grid.visible_char(6, 1), '╳');
}

#[test]
fn arrow_marker_preserves_the_underlying_route() {
    let mut grid = Grid::new(5, 1);
    grid.draw_path(&[Point { x: 0, y: 0 }, Point { x: 4, y: 0 }], Style::Edge);
    grid.arrow(Point { x: 2, y: 0 }, ArrowDirection::East, Style::Edge);
    assert_eq!(grid.visible_char(2, 0), '▶');
    assert_ne!(grid.cell(2, 0).unwrap().line, 0);
}
