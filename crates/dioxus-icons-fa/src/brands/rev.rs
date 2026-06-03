// Copyright (c) 2026 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by Apache-2.0 License
// that can be found in the LICENSE file.

// Auto Generated! DO NOT EDIT!

use dioxus::prelude::*;

#[derive(Debug, Clone, PartialEq, Props)]
pub struct RevProps {
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

pub fn Rev(props: RevProps) -> Element {
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
        d: "M158.7 274.9a65.6 65.6 0 1 1 131.2 0 65.6 65.6 0 1 1 -131.2 0zm270.7-5.1l-.1 0c-.7-29.3-7.7-58.2-20.5-84.6s-31.2-49.7-53.8-68.4L309.6 143c22 14.2 40.1 33.8 52.7 56.8s19.1 48.8 19.1 75.1c0 86.6-70.5 157.1-157.1 157.1S67.2 361.5 67.2 274.9c0-81.9 63-149.3 143-156.4l0 39.1 108.8-62.8-108.8-62.8 0 38.3c-106.7 7.2-191 96-191 204.6 0 111.6 89.1 202.3 200.1 205l0 .1 210.2 0 0-210.2z",
            }
        }
    }
}
