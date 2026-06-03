// Copyright (c) 2026 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by Apache-2.0 License
// that can be found in the LICENSE file.

// Auto Generated! DO NOT EDIT!

use dioxus::prelude::*;

#[derive(Debug, Clone, PartialEq, Props)]
pub struct EnvelopeSquareProps {
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

pub fn EnvelopeSquare(props: EnvelopeSquareProps) -> Element {
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
        d: "M64 32C28.7 32 0 60.7 0 96L0 416c0 35.3 28.7 64 64 64l320 0c35.3 0 64-28.7 64-64l0-320c0-35.3-28.7-64-64-64L64 32zM209.1 267.9L108.4 207.4c-7.7-4.6-12.4-12.9-12.4-21.9 0-14.1 11.4-25.5 25.5-25.5l204.9 0c14.1 0 25.5 11.4 25.5 25.5 0 9-4.7 17.3-12.4 21.9L238.9 267.9c-4.5 2.7-9.6 4.1-14.9 4.1s-10.4-1.4-14.9-4.1zM352 237.3l0 82.7c0 17.7-14.3 32-32 32l-192 0c-17.7 0-32-14.3-32-32l0-82.7 96.7 58C202.1 301 213 304 224 304s21.9-3 31.3-8.7l96.7-58z",
            }
        }
    }
}
