// Copyright (c) 2024 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by Lesser General Public License
// that can be found in the LICENSE file.

use std::{fs, io};

use inflections::Inflect;
use regex::Regex;
use scraper::{ElementRef, Html, node::Element};

pub const UPDATE_KEY: &str = "UPDATE_DIOXUS_ICONS";
pub const TEMPLATE_FILE: &str = include_str!("template.rs");

/// Check whether icon crate shall be refreshed.
///
/// Set `UPDATE_DIOXUS_ICONS=1` environment to force update all icons.
#[inline]
#[must_use]
pub fn need_update() -> bool {
    std::env::var_os(UPDATE_KEY).is_some_and(|val| val != "0" && val != "false")
}

/// Remove all files in `src/` directory of crate
pub fn reset_crate_source() -> Result<(), io::Error> {
    fs::remove_dir_all("src")?;
    fs::create_dir("src")?;
    fs::write("src/lib.rs", "")
}

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
            "            {TAG_NAME} {\n{ATTRS}\n            }"
                .replace("{TAG_NAME}", tag_name)
                .replace("{ATTRS}", &attrs_str)
        })
        .collect::<Vec<_>>()
        .join("\n")
}

pub fn generate_svg_component(node_name: &str, svg_obj: &SvgObject) -> String {
    let width = if let Some(width) = &svg_obj.width {
        &format!("Some(\"{width}\")")
    } else {
        "None"
    };
    let height = if let Some(height) = &svg_obj.height {
        &format!("Some(\"{height}\")")
    } else {
        "None"
    };
    let fill = if let Some(fill) = &svg_obj.fill {
        &format!("Some(\"{fill}\"")
    } else {
        "None"
    };
    let stroke = if let Some(stroke) = &svg_obj.stroke {
        &format!("Some(\"{stroke}\"")
    } else {
        "None"
    };
    let view_box = if let Some(view_box) = &svg_obj.view_box {
        &format!("Some(\"{view_box}\"")
    } else {
        "None"
    };
    let xmlns = if let Some(xmlns) = &svg_obj.xmlns {
        &format!("Some(\"{xmlns}\"")
    } else {
        "None"
    };
    TEMPLATE_FILE
        .replace("{ICON_NAME}", node_name)
        .replace("{WIDTH}", &width)
        .replace("{HEIGHT}", &height)
        .replace("{FILL}", &fill)
        .replace("{stroke}", &stroke)
        .replace("{view_box}", &view_box)
        .replace("{xmlns}", &xmlns)
        .replace("{ICON_PATH}", &svg_obj.children)
}
