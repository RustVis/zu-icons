// Copyright (c) 2026 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by Apache-2.0 License
// that can be found in the LICENSE file.

// Auto Generated! DO NOT EDIT!

use dioxus::prelude::*;

#[derive(Debug, Clone, PartialEq, Props)]
pub struct RugProps {
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

pub fn Rug(props: RugProps) -> Element {
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
        d: "M24 64l56 0 0 384-56 0c-13.3 0-24-10.7-24-24s10.7-24 24-24l8 0 0-40-8 0c-13.3 0-24-10.7-24-24s10.7-24 24-24l8 0 0-32-8 0c-13.3 0-24-10.7-24-24s10.7-24 24-24l8 0 0-32-8 0c-13.3 0-24-10.7-24-24s10.7-24 24-24l8 0 0-40-8 0C10.7 112 0 101.3 0 88S10.7 64 24 64zm104 0l320 0 0 384-320 0 0-384zM576 88c0 13.3-10.7 24-24 24l-8 0 0 40 8 0c13.3 0 24 10.7 24 24s-10.7 24-24 24l-8 0 0 32 8 0c13.3 0 24 10.7 24 24s-10.7 24-24 24l-8 0 0 32 8 0c13.3 0 24 10.7 24 24s-10.7 24-24 24l-8 0 0 40 8 0c13.3 0 24 10.7 24 24s-10.7 24-24 24l-56 0 0-384 56 0c13.3 0 24 10.7 24 24z",
            }
        }
    }
}
