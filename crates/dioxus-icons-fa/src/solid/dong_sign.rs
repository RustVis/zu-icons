// Copyright (c) 2026 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by Apache-2.0 License
// that can be found in the LICENSE file.

// Auto Generated! DO NOT EDIT!

use dioxus::prelude::*;

#[derive(Debug, Clone, PartialEq, Props)]
pub struct DongSignProps {
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

    #[props(default = Some("0 0 384 512"))]
    pub view_box: Option<&'static str>,

    #[props(default = None)]
    pub xmlns: Option<&'static str>,
}

pub fn DongSign(props: DongSignProps) -> Element {
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
        d: "M288-16c-17.7 0-32 14.3-32 32l0 32-104 0c-13.3 0-24 10.7-24 24s10.7 24 24 24l104 0 0 72.2c-22.9-15.3-50.4-24.2-80-24.2-79.5 0-144 64.5-144 144S96.5 432 176 432c30 0 57.8-9.1 80.8-24.8 3.3 14.2 16 24.8 31.2 24.8 17.7 0 32-14.3 32-32l0-304 40 0c13.3 0 24-10.7 24-24s-10.7-24-24-24l-40 0 0-32c0-17.7-14.3-32-32-32zM96 288a80 80 0 1 1 160 0 80 80 0 1 1 -160 0zM24 464c-13.3 0-24 10.7-24 24s10.7 24 24 24l336 0c13.3 0 24-10.7 24-24s-10.7-24-24-24L24 464z",
            }
        }
    }
}
