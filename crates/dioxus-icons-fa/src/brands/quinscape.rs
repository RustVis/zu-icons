// Copyright (c) 2026 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by Apache-2.0 License
// that can be found in the LICENSE file.

// Auto Generated! DO NOT EDIT!

use dioxus::prelude::*;

#[derive(Debug, Clone, PartialEq, Props)]
pub struct QuinscapeProps {
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

pub fn Quinscape(props: QuinscapeProps) -> Element {
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
        d: "M313.7 474.6l-1 0c-41.9 0-82.1-16.7-111.8-46.3s-46.3-69.9-46.3-111.8 16.7-82.1 46.3-111.8 69.9-46.3 111.8-46.3c94.9 0 168.2 83.1 157 176.6 4 5.1 8.2 9.6 11.2 15.3 13.4-30.3 20.3-62.4 20.3-97.7 0-135.1-109.5-244.6-244.6-244.6S12.1 117.5 12.1 252.6 121.6 497.2 256.6 497.2c23.8 .2 47.6-3.2 70.4-10.1-5.2-3.5-8.9-8.1-13.3-12.5zm78.3-169a99.2 99.2 0 1 0 0 198.4 99.2 99.2 0 1 0 0-198.4z",
            }
        }
    }
}
