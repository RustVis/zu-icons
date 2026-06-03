// Copyright (c) 2026 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by Apache-2.0 License
// that can be found in the LICENSE file.

// Auto Generated! DO NOT EDIT!

use dioxus::prelude::*;

#[derive(Debug, Clone, PartialEq, Props)]
pub struct BitcoinSignProps {
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

    #[props(default = Some("0 0 320 512"))]
    pub view_box: Option<&'static str>,

    #[props(default = None)]
    pub xmlns: Option<&'static str>,
}

pub fn BitcoinSign(props: BitcoinSignProps) -> Element {
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
        d: "M64 24C64 10.7 74.7 0 88 0s24 10.7 24 24l0 40 32 0 0-40c0-13.3 10.7-24 24-24s24 10.7 24 24l0 41.1c54.3 7.8 96 54.4 96 110.9 0 24.2-7.7 46.6-20.7 64.9 31.7 19.8 52.7 55 52.7 95.1 0 61.9-50.1 112-112 112l-16 0 0 40c0 13.3-10.7 24-24 24s-24-10.7-24-24l0-40-32 0 0 40c0 13.3-10.7 24-24 24s-24-10.7-24-24l0-40-22.3 0C18.7 448 0 429.3 0 406.3L0 101.6C0 80.8 16.8 64 37.6 64L64 64 64 24zm0 200l112 0c26.5 0 48-21.5 48-48s-21.5-48-48-48l-112 0 0 96zm112 64l-112 0 0 96 144 0c26.5 0 48-21.5 48-48s-21.5-48-48-48l-32 0z",
            }
        }
    }
}
