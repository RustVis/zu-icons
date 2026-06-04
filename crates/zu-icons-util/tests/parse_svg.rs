use zu_icons_util::svg::{generate_svg_component, parse_svg_content};

#[test]
fn test_parse_whats_app_svg() {
    let svg_content = include_str!("ant-design/whats-app.svg");
    let result = parse_svg_content(svg_content);
    assert!(
        result.is_some(),
        "parse_svg_content should return Some for valid SVG"
    );

    let svg_obj = result.unwrap();

    // SVG element attributes
    assert_eq!(svg_obj.view_box.as_deref(), Some("0 0 1024 1024"));
    // xmlns is stripped by the HTML5 parser (scraper)
    assert!(svg_obj.xmlns.is_none());
    assert_eq!(svg_obj.width.as_deref(), Some("200"));
    assert_eq!(svg_obj.height.as_deref(), Some("200"));

    // No fill or stroke on the root <svg> element
    assert!(svg_obj.fill.is_none());
    assert!(svg_obj.stroke.is_none());

    // Children should contain the two <path> elements with their attributes
    assert!(
        svg_obj.children.contains("path {"),
        "children should contain path elements"
    );

    // The `d` attribute (path data) should be present
    assert!(
        svg_obj.children.contains("d:"),
        "children should contain d attribute"
    );

    // The `p-id` attribute should have been converted to `p_id`
    assert!(
        svg_obj.children.contains("p_id:"),
        "children should contain p_id (converted from p-id)"
    );

    // Both child paths should be present
    assert_eq!(
        svg_obj.children.matches("path {").count(),
        2,
        "should have exactly two path elements"
    );

    // The `data-*` and `fill` attributes are filtered out
    assert!(
        !svg_obj.children.contains("t:"),
        "children should NOT contain the `t` attribute (data-like prefix matches)"
    );
    // The `data-*` and `fill` attributes are filtered out
    assert!(
        !svg_obj.children.contains("class:"),
        "children should NOT contain the `class` attribute (present on svg only)"
    );
    // The `data-*` and `fill` attributes are filtered out
    assert!(
        !svg_obj.children.contains("version:"),
        "children should NOT contain the `version` attribute (present on svg only)"
    );
}

#[test]
fn test_generate_whats_app_component() {
    let svg_content = include_str!("ant-design/whats-app.svg");
    let svg_obj = parse_svg_content(svg_content).expect("valid svg");
    let component = generate_svg_component("WhatsApp", None, &svg_obj);

    // Struct definition
    assert!(component.contains("pub struct WhatsApp {}"));
    assert!(component.contains("#[derive(Copy, Clone, PartialEq)]"));

    // Impl block
    assert!(component.contains("impl IconShape for WhatsApp"));

    // Icon path children are embedded in the rsx! macro
    assert!(component.contains("rsx!("));
    assert!(component.contains("path {"));
    assert!(component.contains("p_id:"));

    // Extracted SVG attributes
    assert!(component.contains("const WIDTH: Option<u32> = Some(200);"));
    assert!(component.contains("const HEIGHT: Option<u32> = Some(200);"));
    assert!(component.contains("const VIEW_BOX: Option<&'static str> = Some(\"0 0 1024 1024\");"));

    // No title, no fill, no stroke
    assert!(!component.contains("const TITLE:"));
    assert!(!component.contains("const FILL:"));
    assert!(!component.contains("const STROKE:"));
}
