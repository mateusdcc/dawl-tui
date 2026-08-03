use dawl_tui::canvas::{Grid, Rect};
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
    grid.draw_box(Rect { x: 0, y: 0, width: 6, height: 4 }, Style::Group, false);
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
