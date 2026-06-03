// Copyright (c) 2026 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by Apache-2.0 License
// that can be found in the LICENSE file.

// Auto Generated! DO NOT EDIT!

use dioxus::prelude::*;

#[derive(Debug, Clone, PartialEq, Props)]
pub struct SleighProps {
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

pub fn Sleigh(props: SleighProps) -> Element {
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
        d: "M64 32C46.3 32 32 46.3 32 64S46.3 96 64 96l0 160c0 41.8 26.7 77.4 64 90.5l0 69.5-72 0c-13.3 0-24 10.7-24 24s10.7 24 24 24l504 0c44.2 0 80-35.8 80-80l0-8c0-13.3-10.7-24-24-24s-24 10.7-24 24l0 8c0 17.7-14.3 32-32 32l-80 0 0-64c53 0 96-43 96-96l0-96c17.7 0 32-14.3 32-32s-14.3-32-32-32l-32 0c-17.7 0-32 14.3-32 32l0 32c0 35.3-28.7 64-64 64l-48.9 0c-48.5 0-92.8-27.4-114.5-70.8l-25.2-50.5C237.7 59.4 193.4 32 144.9 32L64 32zM432 416l-256 0 0-64 256 0 0 64z",
            }
        }
    }
}
