// Copyright (c) 2026 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by Apache-2.0 License
// that can be found in the LICENSE file.

// Auto Generated! DO NOT EDIT!

use dioxus::prelude::*;

#[derive(Debug, Clone, PartialEq, Props)]
pub struct StumbleuponCircleProps {
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

pub fn StumbleuponCircle(props: StumbleuponCircleProps) -> Element {
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
        d: "M264 8a248 248 0 1 0 0 496 248 248 0 1 0 0-496zm0 177.5c-9.8 0-17.8 8-17.8 17.8l0 106.9c0 40.9-33.9 73.9-74.9 73.9-41.4 0-74.9-33.5-74.9-74.9l0-46.5 57.3 0 0 45.8c0 10 8 17.8 17.8 17.8s17.8-7.9 17.8-17.8l0-108.4c0-40 34.2-72.1 74.7-72.1 40.7 0 74.7 32.3 74.7 72.6l0 23.7-34.1 10.1-22.9-10.7 0-20.6c.1-9.6-7.9-17.6-17.7-17.6zM431.6 309.1c0 41.4-33.5 74.9-74.9 74.9-41.2 0-74.9-33.2-74.9-74.2l0-46.8 22.9 10.7 34.1-10.1 0 47.1c0 9.8 8 17.6 17.8 17.6s17.8-7.9 17.8-17.6l0-48 57.3 0c-.1 45.9-.1 46.4-.1 46.4z",
            }
        }
    }
}
