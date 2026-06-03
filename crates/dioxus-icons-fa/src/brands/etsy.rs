// Copyright (c) 2026 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by Apache-2.0 License
// that can be found in the LICENSE file.

// Auto Generated! DO NOT EDIT!

use dioxus::prelude::*;

#[derive(Debug, Clone, PartialEq, Props)]
pub struct EtsyProps {
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

pub fn Etsy(props: EtsyProps) -> Element {
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
        d: "M384 348c-1.8 10.7-13.8 110-15.5 132-117.9-4.3-219.9-4.7-368.5 0l0-25.5c45.5-8.9 60.6-8 61-35.2 1.8-72.3 3.5-244.1 0-322-1-28.5-12.1-26.8-61-36L0 35.8c73.9 2.4 255.9 8.6 363-3.8-3.5 38.2-7.8 126.5-7.8 126.5l-23.2 0C320.9 115.7 313.2 68 277.3 68l-137 0c-10.2 0-10.7 3.5-10.7 9.8l0 163.8c58 .5 88.5-2.5 88.5-2.5 29.8-1 27.6-8.5 40.7-65.3l25.8 0c-4.4 101.4-3.9 61.8-1.8 160.3L257 334c-9.2-40.1-9.1-61-39.5-61.5 0 0-21.5-2-88-2l0 139c0 26 14.3 38.3 44.3 38.3l89.3 0c63.6 0 66.6-25 98.7-99.8l22.2 0z",
            }
        }
    }
}
