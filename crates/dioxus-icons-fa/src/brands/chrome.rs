// Copyright (c) 2026 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by Apache-2.0 License
// that can be found in the LICENSE file.

// Auto Generated! DO NOT EDIT!

use dioxus::prelude::*;

#[derive(Debug, Clone, PartialEq, Props)]
pub struct ChromeProps {
    #[props(default = None)]
    pub title: Option<&'static str>,

    #[props(default = None)]
    pub class: Option<&'static str>,

    #[props(default = None)]
    pub style: Option<&'static str>,

    #[props(default = None)]
    pub width: Option<&'static str>,

    #[props(default = None)]
    pub height: Option<&'static str>,

    #[props(default = None)]
    pub fill: Option<&'static str>,

    #[props(default = None)]
    pub stroke: Option<&'static str>,

    #[props(default = Some("0 0 512 512"))]
    pub view_box: Option<&'static str>,

    #[props(default = None)]
    pub xmlns: Option<&'static str>,
}

pub fn Chrome(props: ChromeProps) -> Element {
    rsx! {
        svg {
            class: props.class,
            style: props.style,
            height: props.height,
            width: props.width,
            view_box: props.view_box.unwrap_or("0 0 16 16"),
            xmlns: props.xmlns.unwrap_or("http://www.w3.org/2000/svg"),
            fill: props.fill.unwrap_or("currentColor"),
            stroke: props.stroke,

            if let Some(title_text) = props.title {
                title {{ title_text }}
            }

                        path {
        d: "M0 256c0-46.6 12.5-90.4 34.3-128.9L144.1 318.3c21.9 39.2 63.8 65.7 111.9 65.7 14.3 0 27.1-2.3 40.8-6.6L220.5 509.6C95.9 492.3 0 385.3 0 256zm365.1 65.6c12.3-19.2 18.9-42.5 18.9-65.6 0-38.2-16.8-72.5-43.3-96l152.7 0c12 29.6 18.6 62.1 18.6 96 0 141.4-114.6 255.1-256 256L365.1 321.6zM477.8 128L256 128c-62.9 0-113.7 44.1-125.5 102.7L54.2 98.5C101 38.5 174 0 256 0 350.8 0 433.5 51.5 477.8 128zM344 256a88 88 0 1 1 -176 0 88 88 0 1 1 176 0z",
            }
        }
    }
}
