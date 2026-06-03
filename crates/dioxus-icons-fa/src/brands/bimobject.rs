// Copyright (c) 2026 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by Apache-2.0 License
// that can be found in the LICENSE file.

// Auto Generated! DO NOT EDIT!

use dioxus::prelude::*;

#[derive(Debug, Clone, PartialEq, Props)]
pub struct BimobjectProps {
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

    #[props(default = Some("0 0 448 512"))]
    pub view_box: Option<&'static str>,

    #[props(default = None)]
    pub xmlns: Option<&'static str>,
}

pub fn Bimobject(props: BimobjectProps) -> Element {
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
        d: "M416 32L32 32C14.4 32 0 46.4 0 64L0 448c0 17.6 14.4 32 32 32l384 0c17.6 0 32-14.4 32-32l0-384c0-17.6-14.4-32-32-32zM352 289.4c0 49.4-11.4 82.6-103.8 82.6l-16.9 0c-44.1 0-62.4-14.9-70.4-38.8l-.9 0 0 34.8-64 0 0-232 64 0 0 74.7 1.1 0c4.6-30.5 39.7-38.8 69.7-38.8l17.3 0c92.4 0 103.8 33.1 103.8 82.5l0 35 .1 0zm-64-28.9l0 22.9c0 21.7-3.4 33.8-38.4 33.8l-45.3 0c-28.9 0-44.1-6.5-44.1-35.7l0-19c0-29.3 15.2-35.7 44.1-35.7l45.3 0c35-.2 38.4 12 38.4 33.7z",
            }
        }
    }
}
