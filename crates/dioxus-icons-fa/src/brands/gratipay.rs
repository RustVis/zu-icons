// Copyright (c) 2026 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by Apache-2.0 License
// that can be found in the LICENSE file.

// Auto Generated! DO NOT EDIT!

use dioxus::prelude::{Element, dioxus_core, dioxus_elements, rsx};

use crate::IconShape;

#[derive(Clone, PartialEq)]
pub struct Gratipay {}

impl IconShape for Gratipay {
    fn child_elements(&self) -> Element {
        rsx!(path {
        d: "M256 0a256 256 0 1 0 0 512 256 256 0 1 0 0-512zM374.3 233.7L257.7 391.3 141.3 233.7c-9-12.3-19.7-52 14-74.3 29-18.7 56.4-4.3 70.7 12.3 16.4 18.5 48.1 17.4 63.7 0 14.3-16.6 41.7-31 70.3-12.3 34 22.3 23.3 61.9 14.2 74.3z",
            })
    }

    const VIEW_BOX: Option<&'static str> = Some("0 0 512 512");

}
