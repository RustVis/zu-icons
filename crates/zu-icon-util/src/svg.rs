use inflections::Inflect;
use regex::Regex;
use scraper::{ElementRef, Html, node::Element};

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

    let children = extract_svg_child_elements(&child_elements);

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

fn extract_svg_child_elements(elements: &[&Element]) -> String {
    elements
        .iter()
        .map(|element| {
            let tag_name = element.name();
            let mut element_attrs = element
                .attrs()
                .filter_map(|(name, value)| {
                    if let Ok(re) = Regex::new(r"^data-.*$") {
                        if !re.is_match(name) && name != "fill" {
                            return Some(format!("        {}: \"{value}\",", name.to_snake_case()));
                        }
                    }
                    None
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

pub fn generate_svg_component(node_name: &str, title: Option<&str>, svg_obj: &SvgObject) -> String {
    let title = if let Some(title) = title {
        &format!("    const TITLE: Option<&'static str> = Some(\"{title}\");\n")
    } else {
        ""
    };
    let width = if let Some(width) = &svg_obj.width {
        &format!("    const WIDTH: Option<u32> = Some({width});\n")
    } else {
        ""
    };
    let height = if let Some(height) = &svg_obj.height {
        &format!("    const HEIGHT: Option<u32> = Some({height});\n")
    } else {
        ""
    };
    let fill = if let Some(fill) = &svg_obj.fill {
        &format!("    const FILL: Option<&'static str> = Some(\"{fill}\");\n")
    } else {
        ""
    };
    let stroke = if let Some(stroke) = &svg_obj.stroke {
        &format!("    const STROKE: Option<&'static str> = Some(\"{stroke}\");\n")
    } else {
        ""
    };
    let view_box = if let Some(view_box) = &svg_obj.view_box {
        &format!("    const VIEW_BOX: Option<&'static str> = Some(\"{view_box}\");\n")
    } else {
        ""
    };
    let other_props = [title, width, height, fill, stroke, view_box].join("");
    TEMPLATE_FILE
        .replace("{ICON_NAME}", node_name)
        .replace("{ICON_PATH}", &svg_obj.children)
        .replace("{OTHER_PROPS}", &other_props)
}
