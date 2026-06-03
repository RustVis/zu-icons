// Copyright (c) 2026 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by Apache-2.0 License
// that can be found in the LICENSE file.

// Auto Generated! DO NOT EDIT!

use dioxus::prelude::*;

#[derive(Debug, Clone, PartialEq, Props)]
pub struct MillSignProps {
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

pub fn MillSign(props: MillSignProps) -> Element {
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
        d: "M297-22.2c12.3 5 18.2 19 13.2 31.3l-29 71.3C338.8 85.1 384 133.3 384 192l0 208c0 17.7-14.3 32-32 32s-32-14.3-32-32l0-208c0-26.5-21.5-48-48-48-6.4 0-12.5 1.2-18 3.5l-30 73.8 0 178.7c0 17.7-14.3 32-32 32s-32-14.3-32-32l0-21.2-57.8 142.2c-5 12.3-19 18.2-31.3 13.2s-18.2-19-13.2-31.3L160 251.3 160 192c0-26.5-21.5-48-48-48s-48 21.5-48 48l0 208c0 17.7-14.3 32-32 32S0 417.7 0 400L0 112c0-17.7 14.3-32 32-32 10.9 0 20.5 5.4 26.3 13.7 16-8.7 34.3-13.7 53.7-13.7 31.3 0 59.7 12.9 80 33.6 9.6-9.8 20.9-17.8 33.5-23.5L265.8-9c5-12.3 19-18.2 31.3-13.2z",
            }
        }
    }
}
