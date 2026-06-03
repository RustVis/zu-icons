// Copyright (c) 2026 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by Apache-2.0 License
// that can be found in the LICENSE file.

// Auto Generated! DO NOT EDIT!

use dioxus::prelude::*;

#[derive(Debug, Clone, PartialEq, Props)]
pub struct CancerProps {
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

pub fn Cancer(props: CancerProps) -> Element {
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
        d: "M408 152c57.4 0 104 46.6 104 104 0 141.4-114.6 256-256 256-54.3 0-104.8-17-146.3-45.9-14.5-10.1-18-30.1-7.9-44.6s30.1-18 44.6-7.9c31.1 21.7 68.9 34.4 109.7 34.4 67.9 0 127.5-35.3 161.7-88.5-3.2 .3-6.4 .5-9.7 .5-57.4 0-104-46.6-104-104s46.6-104 104-104zM256 0c54.3 0 104.8 17 146.3 45.9 14.5 10.1 18 30.1 7.9 44.6s-30.1 18-44.6 7.9c-31.1-21.7-68.9-34.4-109.7-34.4-67.9 0-127.5 35.2-161.7 88.4 3.2-.3 6.4-.4 9.7-.4 57.4 0 104 46.6 104 104S161.4 360 104 360 0 313.4 0 256C0 254.1 0 252.2 .1 250.4 3.1 111.6 116.5 0 256 0zM104 216a40 40 0 1 0 0 80 40 40 0 1 0 0-80zm304 0a40 40 0 1 0 0 80 40 40 0 1 0 0-80z",
            }
        }
    }
}
