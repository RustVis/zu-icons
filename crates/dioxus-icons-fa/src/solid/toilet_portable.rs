// Copyright (c) 2026 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by Apache-2.0 License
// that can be found in the LICENSE file.

// Auto Generated! DO NOT EDIT!

use dioxus::prelude::{Element, dioxus_core, dioxus_elements, rsx};

use crate::IconShape;

#[derive(Clone, PartialEq)]
pub struct ToiletPortable {}

impl IconShape for ToiletPortable {
    fn child_elements(&self) -> Element {
        rsx!(path {
        d: "M0 64l0 16 320 0 0-16c0-35.3-28.7-64-64-64L64 0C28.7 0 0 28.7 0 64zm24 64L0 128 0 488c0 13.3 10.7 24 24 24s24-10.7 24-24l0-8 224 0 0 8c0 13.3 10.7 24 24 24s24-10.7 24-24l0-360-296 0zm224 96l24 0 0 96-24 0c-13.3 0-24-10.7-24-24l0-48c0-13.3 10.7-24 24-24z",
            })
    }

    const VIEW_BOX: Option<&'static str> = Some("0 0 320 512");

}
