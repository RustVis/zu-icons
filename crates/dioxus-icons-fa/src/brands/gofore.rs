// Copyright (c) 2026 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by Apache-2.0 License
// that can be found in the LICENSE file.

// Auto Generated! DO NOT EDIT!

use dioxus::prelude::*;

#[derive(Debug, Clone, PartialEq, Props)]
pub struct GoforeProps {
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

pub fn Gofore(props: GoforeProps) -> Element {
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
        d: "M348 319.8l-13.2 0 0 34.7c-24.5 23.1-56.3 35.8-89.9 35.8-73.2 0-132.4-60.2-132.4-134.4 0-74.1 59.2-134.4 132.4-134.4 35.3 0 68.6 14 93.6 39.4l62.3-63.3c-41.8-42.3-97.1-65.6-156.1-65.6-122.7 0-220.7 100.6-220.7 224 0 122.5 97 224 220.7 224 63.2 0 124.5-26.2 171-82.5-2-27.6-13.4-77.7-67.7-77.7zM335.9 207.3l-106.3 0 0 89 118.4 0c33.5 0 60.5 15.1 76 41.8l0-30.6c0-65.2-40.4-100.2-88.1-100.2z",
            }
        }
    }
}
