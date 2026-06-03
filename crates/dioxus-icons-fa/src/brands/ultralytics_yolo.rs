// Copyright (c) 2026 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by Apache-2.0 License
// that can be found in the LICENSE file.

// Auto Generated! DO NOT EDIT!

use dioxus::prelude::*;

#[derive(Debug, Clone, PartialEq, Props)]
pub struct UltralyticsYoloProps {
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

pub fn UltralyticsYolo(props: UltralyticsYoloProps) -> Element {
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
        d: "M383.5 10.8c61.8-.2 112.3 50.6 112.3 112.9-.2 2.7 0 0-.1 3.3l.3 .3c-1.5 89.9-53 168.4-127.4 208.5l0 52.8c0 62.6-51.1 113.5-113.7 112.9-62-.6-111.4-52.2-111.4-114.3l0-51.2c-40.7-21.9-74.5-55.2-97-95.6 23.2 16.5 51.7 26 82 25.9 78.6-.2 142.2-64 142.3-142.5 .1-62.3 50.5-112.7 112.9-112.9zm-255 225.8a112.9 112.9 0 1 1 0-225.8 112.9 112.9 0 1 1 0 225.8z",
            }
        }
    }
}
