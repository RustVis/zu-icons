// Copyright (c) 2026 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by Apache-2.0 License
// that can be found in the LICENSE file.

// Auto Generated! DO NOT EDIT!

use dioxus::prelude::*;

#[derive(Debug, Clone, PartialEq, Props)]
pub struct RulerProps {
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

    #[props(default = Some("0 0 576 512"))]
    pub view_box: Option<&'static str>,

    #[props(default = None)]
    pub xmlns: Option<&'static str>,
}

pub fn Ruler(props: RulerProps) -> Element {
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
        d: "M209.1 516.2c-18.7 18.7-49.1 18.7-67.9 0L28.1 403.1c-18.7-18.7-18.7-49.1 0-67.9l17-17 73.5 73.5c9.4 9.4 24.6 9.4 33.9 0s9.4-24.6 0-33.9l-73.5-73.5 33.9-33.9 50.9 50.9c9.4 9.4 24.6 9.4 33.9 0s9.4-24.6 0-33.9l-50.9-50.9 33.9-33.9 73.5 73.5c9.4 9.4 24.6 9.4 33.9 0s9.4-24.6 0-33.9l-73.5-73.5 33.9-33.9 50.9 50.9c9.4 9.4 24.6 9.4 33.9 0s9.4-24.6 0-33.9l-50.9-50.9 33.9-33.9 73.5 73.5c9.4 9.4 24.6 9.4 33.9 0s9.4-24.6 0-33.9l-73.5-73.5 17-17c18.7-18.7 49.1-18.7 67.9 0L548.5 108.9c18.7 18.7 18.7 49.1 0 67.9L209.1 516.2z",
            }
        }
    }
}
