// Copyright (c) 2026 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by Apache-2.0 License
// that can be found in the LICENSE file.

// Auto Generated! DO NOT EDIT!

use dioxus::prelude::{Element, dioxus_core, dioxus_elements, rsx};

use crate::IconShape;

#[derive(Clone, PartialEq)]
pub struct Adn {}

impl IconShape for Adn {
    fn child_elements(&self) -> Element {
        rsx!(path {
        d: "M256 167.5l64.9 98.8-129.8 0 64.9-98.8zM8 256a248 248 0 1 1 496 0 248 248 0 1 1 -496 0zm396.2 82.7l-148.2-223.2-148.2 223.2 30.4 0 33.6-51.7 168.6 0 33.6 51.7 30.2 0z",
            })
    }

    const VIEW_BOX: Option<&'static str> = Some("0 0 512 512");

}
