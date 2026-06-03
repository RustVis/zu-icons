// Copyright (c) 2026 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by Apache-2.0 License
// that can be found in the LICENSE file.

// Auto Generated! DO NOT EDIT!

use dioxus::prelude::*;

#[derive(Debug, Clone, PartialEq, Props)]
pub struct StethoscopeProps {
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

    #[props(default = Some("0 0 576 512"))]
    pub view_box: Option<&'static str>,

    #[props(default = None)]
    pub xmlns: Option<&'static str>,
}

pub fn Stethoscope(props: StethoscopeProps) -> Element {
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
        d: "M32 48C32 21.5 53.5 0 80 0l48 0c17.7 0 32 14.3 32 32s-14.3 32-32 32l-32 0 0 128c0 53 43 96 96 96s96-43 96-96l0-128-32 0c-17.7 0-32-14.3-32-32S238.3 0 256 0l48 0c26.5 0 48 21.5 48 48l0 144c0 77.4-55 142-128 156.8l0 19.2c0 61.9 50.1 112 112 112s112-50.1 112-112l0-85.5c-37.3-13.2-64-48.7-64-90.5 0-53 43-96 96-96s96 43 96 96c0 41.8-26.7 77.4-64 90.5l0 85.5c0 97.2-78.8 176-176 176S160 465.2 160 368l0-19.2C87 334 32 269.4 32 192L32 48zM480 224a32 32 0 1 0 0-64 32 32 0 1 0 0 64z",
            }
        }
    }
}
