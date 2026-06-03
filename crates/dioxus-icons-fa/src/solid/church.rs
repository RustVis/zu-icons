// Copyright (c) 2026 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by Apache-2.0 License
// that can be found in the LICENSE file.

// Auto Generated! DO NOT EDIT!

use dioxus::prelude::*;

#[derive(Debug, Clone, PartialEq, Props)]
pub struct ChurchProps {
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

pub fn Church(props: ChurchProps) -> Element {
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
        d: "M280-8c0-13.3-10.7-24-24-24S232-21.3 232-8l0 24-32 0c-13.3 0-24 10.7-24 24s10.7 24 24 24l32 0 0 48-98.6 65.8C120 186.7 112 201.6 112 217.7l0 38.3-79.8 45.6C12.3 313 0 334.2 0 357.1L0 448c0 35.3 28.7 64 64 64 74.7 0 85.4 0 320 0l64 0c35.3 0 64-28.7 64-64l0-90.9c0-23-12.3-44.2-32.2-55.6L400 256 400 217.7c0-16-8-31-21.4-39.9l-98.6-65.8 0-48 32 0c13.3 0 24-10.7 24-24s-10.7-24-24-24l-32 0 0-24zM256 320c35.3 0 64 28.7 64 64l0 80-128 0 0-80c0-35.3 28.7-64 64-64z",
            }
        }
    }
}
