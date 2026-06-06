use zu_icons_util::svg::{generate_svg_component, parse_svg_content};

#[test]
fn test_parse_svg() {
    let svg_content = include_str!("ant-design/twitch.svg");
    let svg_obj = parse_svg_content(svg_content);
    assert!(svg_obj.is_some());
    let Some(svg_obj) = svg_obj else {
        return;
    };

    // SVG element attributes
    assert_eq!(svg_obj.view_box.as_deref(), Some("0 0 1042 1042"));
    // fill attribute is present on the root <svg> element
    assert_eq!(svg_obj.fill.as_deref(), Some("currentColor"));

    // Not present on the root <svg> element
    assert!(svg_obj.xmlns.is_none());
    assert!(svg_obj.width.is_none());
    assert!(svg_obj.height.is_none());
    assert!(svg_obj.stroke.is_none());

    // Children contain the nested structure (elements without attributes are skipped)
    assert!(
        !svg_obj.children.contains("defs {"),
        "defs has no attributes, so it is filtered out"
    );
    assert!(
        svg_obj.children.contains("filter {"),
        "children should contain filter"
    );
    assert!(
        svg_obj.children.contains("feOffset {"),
        "children should contain feOffset"
    );
    assert!(
        svg_obj.children.contains("feGaussianBlur {"),
        "children should contain feGaussianBlur"
    );
    assert!(
        svg_obj.children.contains("feColorMatrix {"),
        "children should contain feColorMatrix"
    );
    assert!(
        !svg_obj.children.contains("feMerge {"),
        "feMerge has no attributes, so it is filtered out"
    );
    assert!(
        svg_obj.children.contains("feMergeNode {"),
        "children should contain feMergeNode"
    );
    assert!(
        svg_obj.children.contains("g {"),
        "children should contain g"
    );
    assert!(
        svg_obj.children.contains("path {"),
        "children should contain path"
    );

    // Path attribute `d` should be included
    assert!(
        svg_obj.children.contains("d:"),
        "children should contain d attribute"
    );

    // The `filterUnits` attribute stays as-is (not converted to snake_case)
    assert!(
        svg_obj.children.contains("filterUnits:"),
        "children should contain filterUnits (not converted to snake_case)"
    );

    // The `fill` attribute on child elements is filtered out
    assert!(
        !svg_obj.children.contains("fill:"),
        "children should NOT contain fill attribute"
    );
}

#[test]
fn test_generate_component() {
    let svg_content = include_str!("ant-design/twitch.svg");
    #[allow(clippy::expect_used)]
    let svg_obj = parse_svg_content(svg_content).expect("valid svg");
    let component = generate_svg_component("Twitch", None, &svg_obj);

    // Struct definition (unit struct with derive)
    assert!(component.contains("pub struct Twitch;"));
    assert!(component.contains("#[derive(Default, Copy, Clone, PartialEq, Eq)]"));
    assert!(component.contains("impl IconShape for Twitch"));

    // All child elements (only those with attributes)
    assert!(!component.contains("defs {"));
    assert!(component.contains("filter {"));
    assert!(component.contains("feOffset {"));
    assert!(!component.contains("feMerge {"));
    assert!(component.contains("feMergeNode {"));
    assert!(component.contains("path {"));
    assert!(component.contains("g {"));

    // Child element attributes are properly converted
    assert!(component.contains("filterUnits:"));
    assert!(component.contains("std_deviation:"));
    assert!(component.contains("d:"));

    // The `fill` attribute is filtered out from children
    assert!(!component.contains("fill:"));

    // Both feMergeNode instances are present
    assert_eq!(component.matches("feMergeNode {").count(), 2);

    // Extracted SVG attributes
    assert!(component.contains("const FILL: Option<&'static str> = Some(\"currentColor\");"));
    assert!(component.contains("const VIEW_BOX: Option<&'static str> = Some(\"0 0 1042 1042\");"));

    // No width, height, stroke, or title on the root <svg>
    assert!(!component.contains("const WIDTH:"));
    assert!(!component.contains("const HEIGHT:"));
    assert!(!component.contains("const STROKE:"));
    assert!(!component.contains("const TITLE:"));
}
