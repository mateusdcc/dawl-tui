use dawl_tui::theme::{Palette, Rgb, Style};

#[test]
fn midnight_theme_maps_semantic_roles() {
    let palette = Palette::midnight();
    assert_eq!(palette.foreground(Style::Structure), Rgb::new(68, 211, 242));
    assert_eq!(palette.foreground(Style::Success), Rgb::new(116, 226, 145));
    assert_eq!(palette.background(), Rgb::new(13, 27, 47));
}

#[test]
fn runtime_styles_remain_visually_distinct() {
    let palette = Palette::midnight();
    assert_ne!(palette.foreground(Style::Running), palette.foreground(Style::Failure));
    assert_ne!(palette.foreground(Style::Agent), palette.foreground(Style::Reviewer));
}
