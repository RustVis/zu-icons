// Copyright (c) 2026 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by Apache-2.0 License
// that can be found in the LICENSE file.

// Auto Generated! DO NOT EDIT!

use dioxus::prelude::*;

#[derive(Debug, Clone, PartialEq, Props)]
pub struct LungsProps {
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

pub fn Lungs(props: LungsProps) -> Element {
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
        d: "M320 32c0-17.7-14.3-32-32-32s-32 14.3-32 32l0 141.9-32 19.2 0-116.7c0-24.5-19.9-44.4-44.4-44.4-12.5 0-24.4 5.3-32.8 14.5l-26.4 29C42.9 160.8 0 271.8 0 387l0 30.5c0 52.2 42.3 94.5 94.5 94.5 22 0 43.7-5.1 63.4-15l5-2.5c37.4-18.7 61-56.9 61-98.8l0-128 64-38.4 64 38.4 0 128c0 41.8 23.6 80.1 61 98.8l5 2.5c19.7 9.8 41.4 15 63.4 15 52.2 0 94.5-42.3 94.5-94.5l0-6.2c0-111.1-36.7-219-104.4-307L428.9 48.7c-8.1-10.6-20.7-16.7-34-16.7-23.7 0-42.9 19.2-42.9 42.9l0 118.2-32-19.2 0-141.9z",
            }
        }
    }
}
