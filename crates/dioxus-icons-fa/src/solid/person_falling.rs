// Copyright (c) 2026 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by Apache-2.0 License
// that can be found in the LICENSE file.

// Auto Generated! DO NOT EDIT!

use dioxus::prelude::*;

#[derive(Debug, Clone, PartialEq, Props)]
pub struct PersonFallingProps {
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

pub fn PersonFalling(props: PersonFallingProps) -> Element {
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
        d: "M320 32c0-17.7-14.3-32-32-32s-32 14.3-32 32l0 5.4c0 45-23.6 86.6-62.1 109.8l-4.6 2.8C131.4 184.7 96 247.1 96 314.6L96 384c0 17.7 14.3 32 32 32s32-14.3 32-32l0-69.4c0-16.7 3.3-33 9.4-48L359.2 500.2c11.1 13.7 31.3 15.8 45 4.7s15.8-31.3 4.7-45L295.2 320 400 320 438.4 371.2c10.6 14.1 30.7 17 44.8 6.4s17-30.7 6.4-44.8l-43.2-57.6C437.3 263.1 423.1 256 408 256l-89 0-62.9-75.5c40.3-36 63.9-87.9 63.9-143.1l0-5.4zM104 144a56 56 0 1 0 0-112 56 56 0 1 0 0 112z",
            }
        }
    }
}
