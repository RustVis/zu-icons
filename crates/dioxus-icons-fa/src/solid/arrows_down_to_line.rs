// Copyright (c) 2026 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by Apache-2.0 License
// that can be found in the LICENSE file.

// Auto Generated! DO NOT EDIT!

use dioxus::prelude::*;

#[derive(Debug, Clone, PartialEq, Props)]
pub struct ArrowsDownToLineProps {
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

pub fn ArrowsDownToLine(props: ArrowsDownToLineProps) -> Element {
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
        d: "M214.6 310.6l-64 64c-12.5 12.5-32.8 12.5-45.3 0l-64-64c-12.5-12.5-12.5-32.8 0-45.3s32.8-12.5 45.3 0L96 274.7 96 32c0-17.7 14.3-32 32-32s32 14.3 32 32l0 242.7 9.4-9.4c12.5-12.5 32.8-12.5 45.3 0s12.5 32.8 0 45.3zm256 0l-64 64c-12.5 12.5-32.8 12.5-45.3 0l-64-64c-12.5-12.5-12.5-32.8 0-45.3s32.8-12.5 45.3 0l9.4 9.4 0-242.7c0-17.7 14.3-32 32-32s32 14.3 32 32l0 242.7 9.4-9.4c12.5-12.5 32.8-12.5 45.3 0s12.5 32.8 0 45.3zM32 512c-17.7 0-32-14.3-32-32s14.3-32 32-32l448 0c17.7 0 32 14.3 32 32s-14.3 32-32 32L32 512z",
            }
        }
    }
}
