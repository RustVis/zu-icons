// Copyright (c) 2026 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by Apache-2.0 License
// that can be found in the LICENSE file.

// Auto Generated! DO NOT EDIT!

use dioxus::prelude::*;

#[derive(Debug, Clone, PartialEq, Props)]
pub struct TransgenderAltProps {
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

pub fn TransgenderAlt(props: TransgenderAltProps) -> Element {
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
        d: "M128-32c17.7 0 32 14.3 32 32s-14.3 32-32 32L97.9 32 136 70.1 151 55c9.4-9.4 24.6-9.4 33.9 0s9.4 24.6 0 33.9l-15 15 14.2 14.2c27.9-23.8 64.2-38.2 103.8-38.2 36.7 0 70.6 12.4 97.6 33.2L466.7 32 448 32c-17.7 0-32-14.3-32-32s14.3-32 32-32l96 0c17.7 0 32 14.3 32 32l0 96c0 17.7-14.3 32-32 32s-32-14.3-32-32l0-18.7-84.4 84.4c13 23.1 20.4 49.9 20.4 78.3 0 77.4-55 142-128 156.8l0 35.2 32 0c17.7 0 32 14.3 32 32s-14.3 32-32 32l-32 0 0 16c0 17.7-14.3 32-32 32s-32-14.3-32-32l0-16-32 0c-17.7 0-32-14.3-32-32s14.3-32 32-32l32 0 0-35.2c-73-14.8-128-79.4-128-156.8 0-31.4 9-60.7 24.7-85.4l-16.7-16.7-15 15c-9.4 9.4-24.6 9.4-33.9 0s-9.4-24.6 0-33.9l15-15-38.1-38.1 0 30.1c0 17.7-14.3 32-32 32S0 113.7 0 96L0 0C0-17.7 14.3-32 32-32l96 0zM288 336a96 96 0 1 0 0-192 96 96 0 1 0 0 192z",
            }
        }
    }
}
