// Copyright (c) 2026 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by Apache-2.0 License
// that can be found in the LICENSE file.

// Auto Generated! DO NOT EDIT!

use dioxus::prelude::*;

#[derive(Debug, Clone, PartialEq, Props)]
pub struct HandsHoldingProps {
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

    #[props(default = Some("0 0 640 512"))]
    pub view_box: Option<&'static str>,

    #[props(default = None)]
    pub xmlns: Option<&'static str>,
}

pub fn HandsHolding(props: HandsHoldingProps) -> Element {
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
        d: "M80 104c0-22.1-17.9-40-40-40S0 81.9 0 104L0 325.5c0 25.5 10.1 49.9 28.1 67.9L128 493.3c12 12 28.3 18.7 45.3 18.7l66.7 0c26.5 0 48-21.5 48-48l0-78.9c0-29.7-11.8-58.2-32.8-79.2l-25.3-25.3 0 0c-7.3-7.3-23.1-23.1-47.2-47.2-12.5-12.5-32.8-12.5-45.3 0s-12.5 32.8 0 45.3c24.1 24.1 39.9 39.9 47.2 47.2 11 11 9.2 29.2-3.7 37.8-9.7 6.5-22.7 5.2-31-3.1L98.7 309.5c-12-12-18.7-28.3-18.7-45.3L80 104zm480 0l0 160.2c0 17-6.7 33.3-18.7 45.3l-51.1 51.1c-8.3 8.3-21.3 9.6-31 3.1-12.9-8.6-14.7-26.9-3.7-37.8 7.3-7.3 23.1-23.1 47.2-47.2 12.5-12.5 12.5-32.8 0-45.3s-32.8-12.5-45.3 0c-24.1 24.1-39.9 39.9-47.2 47.2l0 0-25.3 25.3c-21 21-32.8 49.5-32.8 79.2l0 78.9c0 26.5 21.5 48 48 48l66.7 0c17 0 33.3-6.7 45.3-18.7l99.9-99.9c18-18 28.1-42.4 28.1-67.9L640 104c0-22.1-17.9-40-40-40s-40 17.9-40 40z",
            }
        }
    }
}
