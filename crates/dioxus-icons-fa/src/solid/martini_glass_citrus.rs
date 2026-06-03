// Copyright (c) 2026 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by Apache-2.0 License
// that can be found in the LICENSE file.

// Auto Generated! DO NOT EDIT!

use dioxus::prelude::*;

#[derive(Debug, Clone, PartialEq, Props)]
pub struct MartiniGlassCitrusProps {
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

pub fn MartiniGlassCitrus(props: MartiniGlassCitrusProps) -> Element {
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
        d: "M576 80c0-44.2-35.8-80-80-80-18 0-34.6 6-48 16l-81 0c23.6-47.4 72.5-80 129-80 79.5 0 144 64.5 144 144S575.5 224 496 224c-6.5 0-13-.4-19.3-1.3l64-74.7c1.1-1.3 2.2-2.7 3.3-4.1 19.4-14.6 32-37.8 32-64zM66.9 82.6C72.2 71.3 83.5 64 96 64l384 0c12.5 0 23.8 7.3 29.1 18.6s3.4 24.7-4.8 34.2l-184.3 215 0 116.2 64 0c17.7 0 32 14.3 32 32s-14.3 32-32 32l-192 0c-17.7 0-32-14.3-32-32s14.3-32 32-32l64 0 0-116.2-184.3-215c-8.1-9.5-10-22.8-4.8-34.2zM165.6 128L288 270.8 410.4 128 165.6 128z",
            }
        }
    }
}
