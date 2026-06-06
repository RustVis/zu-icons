use zu_icons_util::svg::{generate_svg_component, parse_svg_content};

#[test]
fn test_parse_svg() {
    let svg_content = include_str!("devicons/snyk.svg");
    let svg_obj = parse_svg_content(svg_content);
    assert!(svg_obj.is_some());
    let Some(svg_obj) = svg_obj else {
        return;
    };

    // SVG element attributes
    assert_eq!(svg_obj.view_box.as_deref(), Some("0 0 600 600"));
    assert_eq!(svg_obj.fill.as_deref(), Some("none"));

    assert!(svg_obj.xmlns.is_none());
    assert!(svg_obj.width.is_none());
    assert!(svg_obj.height.is_none());
    assert!(svg_obj.stroke.is_none());

    // Children should contain path elements
    assert!(
        svg_obj.children.contains("path {"),
        "children should contain path elements"
    );

    // The `fill` attribute on child <path> elements is present in children
    // (not filtered out due to QualName namespace comparison in the parser)
    assert!(
        svg_obj.children.contains("fill:"),
        "children should contain the `fill` attribute on child path elements"
    );

    // The `d` attribute (path data) should be present
    assert!(
        svg_obj.children.contains("d:"),
        "children should contain d attribute"
    );

    // Multiple path elements
    assert!(
        svg_obj.children.matches("path {").count() > 1,
        "should have multiple path elements"
    );
}

#[test]
fn test_generate_component() {
    let svg_content = include_str!("devicons/snyk.svg");
    #[allow(clippy::expect_used)]
    let svg_obj = parse_svg_content(svg_content).expect("valid svg");
    let component = generate_svg_component("Snyk", None, &svg_obj);

    // Struct definition (unit struct with derive)
    assert!(component.contains("pub struct Snyk;"));
    assert!(component.contains("#[derive(Default, Copy, Clone, PartialEq, Eq)]"));
    assert!(component.contains("impl IconShape for Snyk"));

    // Children embedded in the rsx! macro
    assert!(component.contains("rsx!("));
    assert!(component.contains("path {"));
    assert!(component.contains("d:"));

    // The `fill` attribute is present on child elements (not filtered out)
    assert!(component.contains("fill:"));

    // Extracted SVG attributes
    assert!(component.contains("const FILL: Option<&'static str> = Some(\"none\");"));
    assert!(component.contains("const VIEW_BOX: Option<&'static str> = Some(\"0 0 600 600\");"));

    // No width, height, stroke, or title on the root <svg>
    assert!(!component.contains("const WIDTH:"));
    assert!(!component.contains("const HEIGHT:"));
    assert!(!component.contains("const STROKE:"));
    assert!(!component.contains("const TITLE:"));
}
