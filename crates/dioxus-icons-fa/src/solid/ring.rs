// Copyright (c) 2026 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by Apache-2.0 License
// that can be found in the LICENSE file.

// Auto Generated! DO NOT EDIT!

use dioxus::prelude::*;

#[derive(Debug, Clone, PartialEq, Props)]
pub struct RingProps {
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

pub fn Ring(props: RingProps) -> Element {
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
        d: "M64 208c0 7.8 4.4 18.7 17.1 30.3 45.4-24.2 107.7-38.3 174.9-38.3s129.5 14.1 174.9 38.3c12.8-11.6 17.1-22.5 17.1-30.3 0-12.3-10.8-32-47.9-50.6-35.2-17.6-86.1-29.4-144.1-29.4s-108.9 11.8-144.1 29.4C74.8 176 64 195.7 64 208zm192 40c-47 0-89.3 7.6-122.9 19.7 33.2 12.4 75.7 20.3 122.9 20.3s89.7-7.8 122.9-20.3C345.3 255.6 303 248 256 248zM0 208C0 158.4 39.4 122.2 83.3 100.2 129.1 77.3 190.3 64 256 64s126.9 13.3 172.7 36.2c43.9 22 83.3 58.2 83.3 107.8l0 96c0 49.6-39.4 85.8-83.3 107.8-45.9 22.9-107 36.2-172.7 36.2S129.1 434.7 83.3 411.8C39.4 389.8 0 353.6 0 304l0-96z",
            }
        }
    }
}
