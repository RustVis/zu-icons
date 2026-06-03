// Copyright (c) 2026 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by Apache-2.0 License
// that can be found in the LICENSE file.

// Auto Generated! DO NOT EDIT!

use dioxus::prelude::*;

#[derive(Debug, Clone, PartialEq, Props)]
pub struct DollyProps {
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

pub fn Dolly(props: DollyProps) -> Element {
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
        d: "M32 0C14.3 0 0 14.3 0 32S14.3 64 32 64l72.9 0 92.1 276.2c-22.5 17.6-37 45-37 75.8 0 53 43 96 96 96 52.4 0 95.1-42 96-94.3l202.1-67.4c16.8-5.6 25.8-23.7 20.2-40.5s-23.7-25.8-40.5-20.2L331.8 357c-17.2-22.1-43.9-36.5-74-37L165.7 43.8C156.9 17.6 132.5 0 104.9 0L32 0zM208 416a48 48 0 1 1 96 0 48 48 0 1 1 -96 0zM280.5 89.3c-25.2 8.2-39 35.3-30.8 60.5l39.6 121.7c8.2 25.2 35.3 39 60.5 30.8l121.7-39.6c25.2-8.2 39-35.3 30.8-60.5L462.8 80.5c-8.2-25.2-35.3-39-60.5-30.8L280.5 89.3z",
            }
        }
    }
}
