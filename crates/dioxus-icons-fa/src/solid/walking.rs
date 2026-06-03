// Copyright (c) 2026 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by Apache-2.0 License
// that can be found in the LICENSE file.

// Auto Generated! DO NOT EDIT!

use dioxus::prelude::*;

#[derive(Debug, Clone, PartialEq, Props)]
pub struct WalkingProps {
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

    #[props(default = Some("0 0 384 512"))]
    pub view_box: Option<&'static str>,

    #[props(default = None)]
    pub xmlns: Option<&'static str>,
}

pub fn Walking(props: WalkingProps) -> Element {
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
        d: "M192 80a56 56 0 1 0 0-112 56 56 0 1 0 0 112zM105.4 227.9l22.6-22.6 0 69.3c0 28 12.2 54.7 33.5 72.9l71.4 61.2c5.9 5.1 9.8 12.1 10.9 19.8l12.6 88.1c2.5 17.5 18.7 29.7 36.2 27.2s29.7-18.7 27.2-36.2l-12.6-88.1c-3.3-23.1-14.9-44.1-32.6-59.3l-34.5-29.6 0-115.2 3.8 4.7c18.2 22.8 45.8 36 75 36l33.2 0c17.7 0 32-14.3 32-32s-14.3-32-32-32l-33.2 0c-9.7 0-18.9-4.4-25-12l-17.9-22.4c-23-28.8-57.9-45.6-94.8-45.6-32.2 0-63.1 12.8-85.8 35.6L60.1 182.6C42.1 200.6 32 225 32 250.5L32 288c0 17.7 14.3 32 32 32s32-14.3 32-32l0-37.5c0-8.5 3.4-16.6 9.4-22.6zm12.4 179.4c-1.5 5.2-4.3 10-8.1 13.8L41.4 489.4c-12.5 12.5-12.5 32.8 0 45.3s32.8 12.5 45.3 0l68.3-68.3c11.5-11.5 19.9-25.8 24.4-41.5l2.2-7.6-46-39.4c-2.5-2.2-5-4.4-7.4-6.8l-10.4 36.2z",
            }
        }
    }
}
