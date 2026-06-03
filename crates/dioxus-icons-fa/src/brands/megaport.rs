// Copyright (c) 2026 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by Apache-2.0 License
// that can be found in the LICENSE file.

// Auto Generated! DO NOT EDIT!

use dioxus::prelude::*;

#[derive(Debug, Clone, PartialEq, Props)]
pub struct MegaportProps {
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

pub fn Megaport(props: MegaportProps) -> Element {
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
        d: "M222.5 209.6l0 66.2 33.5 33.5 33.3-33.3 0-66.4-33.4-33.4-33.4 33.4zM256 8a248 248 0 1 0 0 496 248 248 0 1 0 0-496zM401.1 422.4l-26.1 19.2-26-19.2 0-65.5-33.4-33.4-33.4 33.4 0 65.5-26.2 19.2-26.1-19.2 0-65.5-33.4-33.4-33.5 33.4 0 65.5-26.1 19.2-26.1-19.2 0-87 59.5-59.5 0-87.9 59.5-59.5 0-75.6 26.1-19.2 26.1 19.2 0 75.6 59.5 59.5 0 87.6 59.7 59.7 0 87.1-.1 0z",
            }
        }
    }
}
