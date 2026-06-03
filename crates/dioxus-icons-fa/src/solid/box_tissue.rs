// Copyright (c) 2026 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by Apache-2.0 License
// that can be found in the LICENSE file.

// Auto Generated! DO NOT EDIT!

use dioxus::prelude::*;

#[derive(Debug, Clone, PartialEq, Props)]
pub struct BoxTissueProps {
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

pub fn BoxTissue(props: BoxTissueProps) -> Element {
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
        d: "M103.9 32l161 0c13.8 0 26 8.8 30.4 21.9l17.4 52.2c4.4 13.1 16.6 21.9 30.4 21.9l60.5 0c21.8 0 37.3 21.4 30.4 42.1L384 320 128 320 72.7 70.9C68.2 51 83.4 32 103.9 32zM48 256l16.6 0 16.5 74.4C86 352.4 105.5 368 128 368l256 0c20.7 0 39-13.2 45.5-32.8l26.4-79.2 8.1 0c26.5 0 48 21.5 48 48l0 128c0 26.5-21.5 48-48 48L48 480c-26.5 0-48-21.5-48-48L0 304c0-26.5 21.5-48 48-48z",
            }
        }
    }
}
