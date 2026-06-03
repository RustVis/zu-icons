// Copyright (c) 2026 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by Apache-2.0 License
// that can be found in the LICENSE file.

// Auto Generated! DO NOT EDIT!

use dioxus::prelude::*;

#[derive(Debug, Clone, PartialEq, Props)]
pub struct BoxesPackingProps {
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

pub fn BoxesPacking(props: BoxesPackingProps) -> Element {
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
        d: "M208 0l80 0 0 56c0 13.3 10.7 24 24 24l80 0c13.3 0 24-10.7 24-24l0-56 80 0c26.5 0 48 21.5 48 48l0 416c0 26.5-21.5 48-48 48l-108.8 0c8.2-14.1 12.8-30.5 12.8-48l0-160c10-13.4 16-30 16-48l0-32c0-44.2-35.8-80-80-80l-176 0 0-96c0-26.5 21.5-48 48-48zM32 336l320 0 0 128c0 26.5-21.5 48-48 48L80 512c-26.5 0-48-21.5-48-48l0-128zM48 192l288 0c17.7 0 32 14.3 32 32l0 32c0 17.7-14.3 32-32 32L48 288c-17.7 0-32-14.3-32-32l0-32c0-17.7 14.3-32 32-32z",
            }
        }
    }
}
