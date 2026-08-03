use dawl_tui::canvas::Rect;
use dawl_tui::layout::{layout_diagram, LayoutOptions};
use dawl_tui::model::Point;
use dawl_tui::parser::parse;
use dawl_tui::route::route_diagram;

#[test]
fn routes_around_node_interiors_with_orthogonal_segments() {
    let source = r#"
    diagram routed "Routed"
    viewport 60x20
    node a "A" at 2,7 size 9x3
    node obstacle "Obstacle" at 20,5 size 12x7
    node b "B" at 45,7 size 9x3
    edge ab a -> b
    "#;
    let diagram = parse(source).unwrap();
    let layout = layout_diagram(&diagram, &LayoutOptions::default()).unwrap();
    let routes = route_diagram(&diagram, &layout).unwrap();
    let route = &routes[0];
    assert!(route.points.windows(2).all(|pair| orthogonal(pair[0], pair[1])));
    assert_eq!(route.points.first().copied(), Some(Point { x: 10, y: 8 }));
    assert_eq!(route.points.last().copied(), Some(Point { x: 45, y: 8 }));
    assert!(route.points.iter().all(|point| !inside(layout.nodes["obstacle"], *point)));
}

#[test]
fn honors_explicit_ports_and_route_points() {
    let source = r#"
    diagram hinted "Hinted"
    viewport 50x16
    node a "A" at 5,2 size 9x3
    node b "B" at 30,2 size 9x3
    edge ab a -> b from_port south to_port south via 9,9 34,9
    "#;
    let diagram = parse(source).unwrap();
    let layout = layout_diagram(&diagram, &Default::default()).unwrap();
    let routes = route_diagram(&diagram, &layout).unwrap();
    assert_eq!(routes[0].points, vec![
        Point { x: 9, y: 4 }, Point { x: 9, y: 9 },
        Point { x: 34, y: 9 }, Point { x: 34, y: 4 },
    ]);
}

#[test]
fn back_edges_leave_below_the_loop() {
    let source = r#"
    diagram retry "Retry"
    viewport 50x20
    node developer "Developer" at 4,4 size 13x3
    node failed "failed review" at 27,10 size 15x3 kind failure
    edge retry failed -> developer kind back
    "#;
    let diagram = parse(source).unwrap();
    let layout = layout_diagram(&diagram, &Default::default()).unwrap();
    let routes = route_diagram(&diagram, &layout).unwrap();
    assert!(routes[0].points.iter().any(|point| point.y > 12));
}

fn orthogonal(a: Point, b: Point) -> bool {
    a.x == b.x || a.y == b.y
}

fn inside(rect: Rect, point: Point) -> bool {
    point.x > rect.x && point.x < rect.right() && point.y > rect.y && point.y < rect.bottom()
}
