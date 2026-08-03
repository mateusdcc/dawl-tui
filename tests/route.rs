use dawl_tui::canvas::Rect;
use dawl_tui::layout::{layout_diagram, LayoutOptions};
use dawl_tui::model::Point;
use dawl_tui::parser::parse;
use dawl_tui::route::route_diagram;
use std::fs;

fn route_source(source: &str) -> (dawl_tui::layout::Layout, Vec<dawl_tui::route::RoutedEdge>) {
    let diagram = parse(source).unwrap();
    let layout = layout_diagram(&diagram, &LayoutOptions::default()).unwrap();
    let routes = route_diagram(&diagram, &layout).unwrap();
    (layout, routes)
}

#[test]
fn routes_around_node_interiors_with_orthogonal_segments() {
    let source = "diagram routed \"Routed\"\nviewport 60x20\nnode a \"A\" at 2,7 size 9x3\nnode obstacle \"Obstacle\" at 20,5 size 12x7\nnode b \"B\" at 45,7 size 9x3\nedge ab a -> b\n";
    let (layout, routes) = route_source(source);
    let route = &routes[0];
    assert!(route
        .points
        .windows(2)
        .all(|pair| orthogonal(pair[0], pair[1])));
    assert_eq!(route.points.first().copied(), Some(Point { x: 10, y: 8 }));
    assert_eq!(route.points.last().copied(), Some(Point { x: 45, y: 8 }));
    assert!(route
        .points
        .iter()
        .all(|p| !inside(layout.nodes["obstacle"], *p)));
}

#[test]
fn honors_explicit_ports_and_route_points() {
    let source = "diagram hinted \"Hinted\"\nviewport 50x16\nnode a \"A\" at 5,2 size 9x3\nnode b \"B\" at 30,2 size 9x3\nedge ab a -> b from_port south to_port south via 9,9 34,9\n";
    let (_, routes) = route_source(source);
    let expected = vec![
        Point { x: 9, y: 4 },
        Point { x: 9, y: 9 },
        Point { x: 34, y: 9 },
        Point { x: 34, y: 4 },
    ];
    assert_eq!(routes[0].points, expected);
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

#[test]
fn approval_fan_out_and_fan_in_use_single_integer_tracks() {
    let source = fs::read_to_string("examples/approval.dtui").unwrap();
    let (_, routes) = route_source(&source);
    for id in ["fork65", "fork66", "forkN"] {
        let route = routes.iter().find(|route| route.id == id).unwrap();
        assert!(route_covers(route, Point { x: 20, y: 36 }));
    }
    for id in ["yes65", "yes66", "yesN"] {
        let route = routes.iter().find(|route| route.id == id).unwrap();
        assert!(route_covers(route, Point { x: 108, y: 31 }));
    }
    let merge_retry = routes.iter().find(|route| route.id == "mretry").unwrap();
    assert!(route_covers(merge_retry, Point { x: 142, y: 13 }));
}

#[test]
fn approval_arrows_finish_in_the_declared_port_direction() {
    let source = fs::read_to_string("examples/approval.dtui").unwrap();
    let (_, routes) = route_source(&source);
    assert_final_direction(&routes, "results_skip", (0, 1));
    assert_final_direction(&routes, "summary_agent", (0, 1));
    assert_final_direction(&routes, "summary_output", (0, 1));
    assert_final_direction(&routes, "retry65", (0, -1));
    assert_final_direction(&routes, "phase_mdev", (1, 0));
    assert_final_direction(&routes, "cleanup_summary", (1, 0));
}

fn assert_final_direction(routes: &[dawl_tui::route::RoutedEdge], id: &str, expected: (i8, i8)) {
    let route = routes.iter().find(|route| route.id == id).unwrap();
    let pair = route.points.windows(2).last().unwrap();
    let dx = pair[1].x.cmp(&pair[0].x) as i8;
    let dy = pair[1].y.cmp(&pair[0].y) as i8;
    assert_eq!((dx, dy), expected, "wrong final approach for {id}");
}

fn route_covers(route: &dawl_tui::route::RoutedEdge, point: Point) -> bool {
    route.points.windows(2).any(|pair| {
        let [from, to] = pair else {
            return false;
        };
        if from.x == to.x && point.x == from.x {
            return (from.y.min(to.y)..=from.y.max(to.y)).contains(&point.y);
        }
        from.y == to.y
            && point.y == from.y
            && (from.x.min(to.x)..=from.x.max(to.x)).contains(&point.x)
    })
}

fn orthogonal(a: Point, b: Point) -> bool {
    a.x == b.x || a.y == b.y
}

fn inside(rect: Rect, point: Point) -> bool {
    point.x > rect.x && point.x < rect.right() && point.y > rect.y && point.y < rect.bottom()
}
