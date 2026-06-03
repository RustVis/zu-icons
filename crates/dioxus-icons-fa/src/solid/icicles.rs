// Copyright (c) 2026 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by Apache-2.0 License
// that can be found in the LICENSE file.

// Auto Generated! DO NOT EDIT!

use dioxus::prelude::*;

#[derive(Debug, Clone, PartialEq, Props)]
pub struct IciclesProps {
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

pub fn Icicles(props: IciclesProps) -> Element {
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
        d: "M75.8 304.8L1 35.7C.3 33.2 0 30.7 0 28.2 0 12.6 12.6 0 28.2 0L482.4 0c16.3 0 29.6 13.2 29.6 29.6 0 1.6-.1 3.3-.4 4.9L434.6 496.1c-1.5 9.2-9.5 15.9-18.8 15.9-9.2 0-17.1-6.6-18.7-15.6L336 160 307.2 303.9c-1.9 9.3-10.1 16.1-19.6 16.1-9.2 0-17.2-6.2-19.4-15.1L240 192 210.6 368.2c-1.5 9.1-9.4 15.8-18.6 15.8s-17.1-6.7-18.6-15.8L144 192 115.9 304.3c-2.3 9.2-10.6 15.7-20.1 15.7-9.3 0-17.5-6.2-20-15.2z",
            }
        }
    }
}
