// Copyright (c) 2026 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by Apache-2.0 License
// that can be found in the LICENSE file.

// Auto Generated! DO NOT EDIT!

use dioxus::prelude::*;

#[derive(Debug, Clone, PartialEq, Props)]
pub struct StumbleuponProps {
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

pub fn Stumbleupon(props: StumbleuponProps) -> Element {
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
        d: "M502.9 266l0 69.7c0 62.1-50.3 112.4-112.4 112.4-61.8 0-112.4-49.8-112.4-111.3l0-70.2 34.3 16 51.1-15.2 0 70.6c0 14.7 12 26.5 26.7 26.5S417 352.7 417 338l0-72 85.9 0zM278.2 207.8l34.3 16 51.1-15.2 0-35.6c0-60.5-51.1-109-112.1-109-60.8 0-112.1 48.2-112.1 108.2l0 162.4c0 14.9-12 26.7-26.7 26.7S86 349.5 86 334.6l0-68.6-86 0 0 69.7c0 62 50.3 112.3 112.4 112.3 61.6 0 112.4-49.5 112.4-110.8l0-160.3c0-14.7 12-26.7 26.7-26.7s26.7 12 26.7 26.7l0 30.9z",
            }
        }
    }
}
