// Copyright (c) 2026 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by Apache-2.0 License
// that can be found in the LICENSE file.

// Auto Generated! DO NOT EDIT!

use dioxus::prelude::*;

#[derive(Debug, Clone, PartialEq, Props)]
pub struct PenFancyProps {
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

pub fn PenFancy(props: PenFancyProps) -> Element {
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
        d: "M373.5 27.1c15-17.2 36.7-27.1 59.6-27.1 43.6 0 79 35.4 79 79 0 22.8-9.9 44.6-27.1 59.6L283.7 313.8 273 303 209 239 198.2 228.3 373.5 27.1zM161.1 259C162 260 188 286 239 337l13.9 13.9-17.1 74.2c-3.9 17.1-16.9 30.7-33.8 35.4l-169.8 47.5 92.3-92.3c1.2 .1 2.3 .2 3.5 .2 17.7 0 32-14.3 32-32s-14.3-32-32-32-32 14.3-32 32c0 1.2 .1 2.4 .2 3.5L3.9 479.8 51.5 310c4.7-16.9 18.3-29.9 35.4-33.8L161.1 259z",
            }
        }
    }
}
