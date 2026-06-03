// Copyright (c) 2024 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by Lesser General Public License
// that can be found in the LICENSE file.

use roxmltree::{Document, Error, Node, NodeType};

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

/// Get inner html of an svg file, without `<svg>` root tag.
#[must_use]
pub fn get_svg_inner(s: &str) -> Option<&str> {
    let start_index = s.find("<svg")?;
    let end_index = s.find("</svg>")?;

    let mut start_index_end = start_index;
    for (index, c) in s[start_index..].chars().enumerate() {
        if c == '>' {
            start_index_end = start_index + index + 1;
            break;
        }
    }
    if start_index == start_index_end || start_index_end >= end_index {
        return None;
    }
    Some(s[start_index_end..end_index].trim())
}

/// Get all `<path />` nodes in an svg file.
///
/// # Errors
/// Returns error if s is not a valid svg file.
pub fn get_svg_path_data(s: &str) -> Result<String, Error> {
    let doc = Document::parse(s)?;
    let nodes: Vec<Node> = doc
        .descendants()
        .filter(|n| n.node_type() == NodeType::Element && n.has_tag_name("path"))
        .collect();
    let mut s = Vec::new();
    for node in nodes {
        let mut parts = vec!["<path".to_owned()];
        for attr in node.attributes() {
            parts.push(format!(" {}=\"{}\"", attr.name(), attr.value()));
        }
        parts.push("/>".to_owned());
        s.push(parts.join(""));
    }
    Ok(s.join(""))
}
