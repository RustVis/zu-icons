// Copyright (c) 2026 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by Apache-2.0 License
// that can be found in the LICENSE file.

// Auto Generated! DO NOT EDIT!

use dioxus::prelude::*;

#[derive(Debug, Clone, PartialEq, Props)]
pub struct CentSignProps {
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

pub fn CentSign(props: CentSignProps) -> Element {
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
        d: "M208 0c17.7 0 32 14.3 32 32l0 25.4c43.6 5.2 83 24.5 113.3 53.1 12.9 12.1 13.4 32.4 1.3 45.2s-32.4 13.4-45.2 1.3c-24.4-23-57.2-37.1-93.3-37.1-75.1 0-136 60.9-136 136s60.9 136 136 136c36.2 0 69-14.1 93.3-37.1 12.9-12.1 33.1-11.5 45.2 1.3s11.5 33.1-1.3 45.2C323 430.1 283.6 449.4 240 454.6l0 25.4c0 17.7-14.3 32-32 32s-32-14.3-32-32l0-28C84.7 433.5 16 352.8 16 256S84.7 78.5 176 60l0-28c0-17.7 14.3-32 32-32z",
            }
        }
    }
}
