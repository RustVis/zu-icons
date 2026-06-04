use inflections::Inflect;
use scraper::{node::Element, ElementRef, Html};

const TEMPLATE_FILE: &str = include_str!("template.rs");

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

fn convert_svg_child_name(name: &str) -> Option<String> {
    // Ignores data- attributes
    if name.starts_with("data-") {
        return None;
    }
    // Ignores specific attributes
    let skipped_names = &["p-id"];
    if skipped_names.contains(&name) {
        return None;
    }
    // Special attributes
    if name == "in" {
        return Some("_in".to_owned());
    }
    if name == "filterUnits" {
        return Some(name.to_owned());
    }
    Some(name.to_snake_case())
}

fn extract_svg_child_elements(elements: &[&Element]) -> String {
    elements
        .iter()
        .map(|element| {
            let tag_name = element.name();
            let mut element_attrs = element
                .attrs()
                .filter_map(|(name, value)| {
                    if let Some(name) = convert_svg_child_name(name) {
                        Some(format!("        {name}: \"{value}\","))
                    } else {
                        None
                    }
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
