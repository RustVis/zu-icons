// Copyright (c) 2026 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by Apache-2.0 License
// that can be found in the LICENSE file.

// Auto Generated! DO NOT EDIT!

use dioxus::prelude::*;

#[derive(Debug, Clone, PartialEq, Props)]
pub struct UltralyticsHubProps {
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

pub fn UltralyticsHub(props: UltralyticsHubProps) -> Element {
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
        d: "M130.4 16c60.9 0 110.4 49.5 110.4 110.4l0 24.9c4.8-.7 9.7-1 14.6-1 88.1-.2 165.1 49 205.4 121.2-22.8-16.2-50.7-25.4-80.4-25.3-77.1 .2-139.5 62.6-139.6 139.4-.1 60.9-49.5 110.3-110.7 110.5-60.6 .2-110.2-49.5-110.2-110.5 .2-2.6 0 0 .1-3.2l0-256C20.1 65.5 69.5 16 130.4 16zM380.3 496a110.4 110.4 0 1 1 0-220.9 110.4 110.4 0 1 1 0 220.9z",
            }
        }
    }
}
