// Copyright (c) 2026 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by Apache-2.0 License
// that can be found in the LICENSE file.

// Auto Generated! DO NOT EDIT!

use dioxus::prelude::*;

#[derive(Debug, Clone, PartialEq, Props)]
pub struct PiedPiperProps {
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

pub fn PiedPiper(props: PiedPiperProps) -> Element {
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
        d: "M440.2 23.2c-26.7 6.8-68.1 28.5-114.6 67.5-30.9-17.5-65.8-26.7-101.4-26.7-114.9 0-208 93.1-208 208s93.1 208 208 208 208-93.1 208-208c.1-54.1-21-106.1-58.7-144.8-6.6 8.5-12.3 17.7-17 27.4 28.9 32.3 44.8 74.1 44.9 117.4 0 97.7-79.4 177.1-177.1 177.1-30.8 0-61-8.1-87.6-23.4 82.9-107.3 150.8-37.8 184.3-226.6 5.8-32.6 28-94.3 126.2-160.2 8.1-5.4 2.4-18.1-7-15.7zM109.3 406.4C89.8 389.8 74.2 369.2 63.5 346s-16.3-48.5-16.3-74c0-97.7 79.4-177.1 177.1-177.1 26.6 0 52.8 6.1 76.6 17.8-66 62.1-126.9 152.9-191.6 293.8z",
            }
        }
    }
}
