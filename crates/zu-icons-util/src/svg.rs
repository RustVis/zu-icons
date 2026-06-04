use inflections::Inflect;
use scraper::{ElementRef, Html, node::Element};

const TEMPLATE_FILE: &str = include_str!("template.rs");

/// Parsed representation of an SVG file.
///
/// Extracts the root `<svg>` element's attributes (`viewBox`, `xmlns`, `width`,
/// `height`, `fill`, `stroke`) and the rendered child elements as a string.
///
/// The `children` field contains the SVG child elements (paths, groups, filters,
/// etc.) formatted for use inside a Dioxus `rsx!()` macro.
#[derive(Debug)]
pub struct SvgObject {
    pub view_box: Option<String>,
    pub xmlns: Option<String>,
    pub width: Option<String>,
    pub height: Option<String>,
    pub fill: Option<String>,
    pub stroke: Option<String>,
    pub children: String,
}

/// Parse SVG content and extract its structure.
///
/// Parses the SVG string using an HTML5 parser, extracts root `<svg>` attributes
/// and all descendant elements with attributes. Elements that have no attributes
/// (e.g. `<defs>`, `<feMerge>`) are excluded from the child list.
///
/// Returns `None` if the SVG contains no elements with attributes.
pub fn parse_svg_content(svg_content: &str) -> Option<SvgObject> {
    let fragment = Html::parse_fragment(svg_content);
    let elements: Vec<_> = fragment
        .tree
        .nodes()
        .filter_map(|node| {
            if node.value().is_element() {
                if let Some(element) = ElementRef::wrap(node) {
                    let element = element.value();
                    if !element.attrs.is_empty() {
                        return Some(element);
                    }
                }
            }
            None
        })
        .collect();
    if elements.is_empty() {
        println!("svg is empty");
        return None;
    }
    let svg_element = &elements[0];
    let child_elements = &elements[1..];
    let view_box = svg_element.attr("viewBox").map(String::from);
    let xmlns = svg_element.attr("xmlns").map(String::from);
    let width = svg_element.attr("width").map(String::from);
    let height = svg_element.attr("height").map(String::from);
    let fill = svg_element.attr("fill").map(String::from);
    let stroke = svg_element.attr("stroke").map(String::from);

    let children = extract_svg_child_elements(child_elements);

    Some(SvgObject {
        view_box,
        xmlns,
        width,
        height,
        fill,
        stroke,
        children,
    })
}

/// Converts an SVG attribute name to its Rust-friendly form for use in `rsx!()`.
///
/// SVG attributes on child elements are mapped to matching Dioxus attribute names
/// inside the generated `rsx!()` macro. This function handles special cases:
///
/// - `data-*` attributes are dropped — they are metadata, not rendered.
/// - `p-id` is dropped — it is stripped by the HTML5 parser anyway and serves
///   no purpose in the generated component.
/// - `in` is prefixed with an underscore (`_in`) to avoid collision with Rust's
///   reserved keyword.
/// - `filterUnits` is kept as-is because `to_snake_case()` from the `inflections`
///   crate does not reliably convert it (camelCase boundary detection differs).
/// - All other attributes are converted to `snake_case` via `inflections`.
fn convert_svg_child_name(name: &str) -> Option<String> {
    // Drop data-* attributes (metadata, not rendered)
    if name.starts_with("data-") {
        return None;
    }
    // Drop specific attributes that should not appear in the generated component
    let skipped_names = &["p-id"];
    if skipped_names.contains(&name) {
        return None;
    }
    // Prefix `in` with underscore to avoid Rust keyword conflict
    if name == "in" {
        return Some("_in".to_owned());
    }
    // Keep `filterUnits` as-is (inflections does not reliably camelCase it)
    if name == "filterUnits" {
        return Some(name.to_owned());
    }
    // Default: convert to snake_case
    Some(name.to_snake_case())
}

/// Format a list of child elements into an `rsx!()`-compatible string.
///
/// Each element is rendered as:
/// ```text
/// TAG_NAME {
///         attr1: "value1",
///         attr2: "value2",
///             }
/// ```
/// Attributes are sorted alphabetically for deterministic output.
fn extract_svg_child_elements(elements: &[&Element]) -> String {
    elements
        .iter()
        .map(|element| {
            let tag_name = element.name();
            let mut element_attrs = element
                .attrs()
                .filter_map(|(name, value)| {
                    convert_svg_child_name(name).map(|name| format!("        {name}: \"{value}\","))
                })
                .collect::<Vec<_>>();
            element_attrs.sort();
            let attrs_str = element_attrs.join("\n");
            "{TAG_NAME} {\n{ATTRS}\n            }"
                .replace("{TAG_NAME}", tag_name)
                .replace("{ATTRS}", &attrs_str)
        })
        .collect::<Vec<_>>()
        .join("\n")
}

/// Generate a complete Dioxus icon component source from an `SvgObject`.
///
/// The output is a Rust source string containing a unit struct with an
/// `IconShape` implementation, filled from the template file.
///
/// # Parameters
///
/// * `node_name` — `PascalCase` name for the generated struct (e.g. `"WhatsApp"`).
/// * `title` — Optional default title text rendered as a `<title>` element.
/// * `svg_obj` — The parsed SVG data.
#[must_use]
pub fn generate_svg_component(node_name: &str, title: Option<&str>, svg_obj: &SvgObject) -> String {
    let title = title.map_or(String::new(), |t| {
        format!("    const TITLE: Option<&'static str> = Some(\"{t}\");\n")
    });
    let width = svg_obj.width.as_ref().map_or(String::new(), |w| {
        format!("    const WIDTH: Option<u32> = Some({w});\n")
    });
    let height = svg_obj.height.as_ref().map_or(String::new(), |h| {
        format!("    const HEIGHT: Option<u32> = Some({h});\n")
    });
    let fill = svg_obj.fill.as_ref().map_or(String::new(), |f| {
        format!("    const FILL: Option<&'static str> = Some(\"{f}\");\n")
    });
    let stroke = svg_obj.stroke.as_ref().map_or(String::new(), |s| {
        format!("    const STROKE: Option<&'static str> = Some(\"{s}\");\n")
    });
    let view_box = svg_obj.view_box.as_ref().map_or(String::new(), |v| {
        format!("    const VIEW_BOX: Option<&'static str> = Some(\"{v}\");\n")
    });
    let other_props = [title, width, height, fill, stroke, view_box].join("");
    TEMPLATE_FILE
        .replace("{ICON_NAME}", node_name)
        .replace("{ICON_PATH}", &svg_obj.children)
        .replace("{OTHER_PROPS}", &other_props)
}
