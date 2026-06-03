// Copyright (c) 2026 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by Apache-2.0 License
// that can be found in the LICENSE file.

// Auto Generated! DO NOT EDIT!

use dioxus::prelude::{Element, dioxus_core, dioxus_elements, rsx};

use crate::IconShape;

#[derive(Clone, PartialEq)]
pub struct Gitter {}

impl IconShape for Gitter {
    fn child_elements(&self) -> Element {
        rsx!(path {
        d: "M66.4 322.5l-50.4 0 0-322.5 50.4 0 0 322.5zM166.9 76.1l-50.4 0 0 435.9 50.4 0 0-435.9zm100.6 0l-50.4 0 0 435.9 50.4 0 0-435.9zM368 76l-50.4 0 0 247 50.4 0 0-247z",
            })
    }

    const VIEW_BOX: Option<&'static str> = Some("0 0 384 512");

}
