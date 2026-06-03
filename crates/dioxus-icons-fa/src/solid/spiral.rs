// Copyright (c) 2026 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by Apache-2.0 License
// that can be found in the LICENSE file.

// Auto Generated! DO NOT EDIT!

use dioxus::prelude::*;

#[derive(Debug, Clone, PartialEq, Props)]
pub struct SpiralProps {
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

pub fn Spiral(props: SpiralProps) -> Element {
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
        d: "M115.5 7.4c13.6-11.3 33.8-9.5 45.1 4.1s9.5 33.8-4.1 45.1C100.2 103.5 64 175.2 64 256 64 362 150 448 256 448s192-86 192-192c0-75.1-60.9-136-136-136S176 180.9 176 256c0 44.2 35.8 80 80 80s80-35.8 80-80c0-13.3-10.7-24-24-24s-24 10.7-24 24c0 17.7-14.3 32-32 32s-32-14.3-32-32c0-48.6 39.4-88 88-88s88 39.4 88 88c0 79.5-64.5 144-144 144S112 335.5 112 256c0-110.5 89.5-200 200-200s200 89.5 200 200c0 141.4-114.6 256-256 256S0 397.4 0 256C0 155.8 45 66.1 115.5 7.4z",
            }
        }
    }
}
