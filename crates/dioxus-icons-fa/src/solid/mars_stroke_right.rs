// Copyright (c) 2026 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by Apache-2.0 License
// that can be found in the LICENSE file.

// Auto Generated! DO NOT EDIT!

use dioxus::prelude::*;

#[derive(Debug, Clone, PartialEq, Props)]
pub struct MarsStrokeRightProps {
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

    #[props(default = Some("0 0 640 512"))]
    pub view_box: Option<&'static str>,

    #[props(default = None)]
    pub xmlns: Option<&'static str>,
}

pub fn MarsStrokeRight(props: MarsStrokeRightProps) -> Element {
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
        d: "M320.5 256a112 112 0 1 0 -224 0 112 112 0 1 0 224 0zM208.5 80c86.3 0 158.1 62.1 173.1 144.1 1-.1 1.9-.1 2.9-.1l16 0 0-32c0-17.7 14.3-32 32-32s32 14.3 32 32l0 32 61.4 0-22.4-28c-11-13.8-8.8-33.9 5-45s33.9-8.8 45 5l64 80c9.3 11.7 9.3 28.3 0 40l-64 80c-11 13.8-31.2 16-45 5s-16-31.2-5-45l22.4-28-61.4 0 0 32c0 17.7-14.3 32-32 32s-32-14.3-32-32l0-32-16 0c-1 0-1.9 0-2.9-.1-15 82-86.8 144.1-173.1 144.1-97.2 0-176-78.8-176-176s78.8-176 176-176z",
            }
        }
    }
}
