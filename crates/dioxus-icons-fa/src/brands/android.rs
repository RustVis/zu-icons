// Copyright (c) 2026 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by Apache-2.0 License
// that can be found in the LICENSE file.

// Auto Generated! DO NOT EDIT!

use dioxus::prelude::*;

#[derive(Debug, Clone, PartialEq, Props)]
pub struct AndroidProps {
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

pub fn Android(props: AndroidProps) -> Element {
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
        d: "M420.5 253.9a24 24 0 1 1 0 48 24 24 0 1 1 0-48zm-265.1 0a24 24 0 1 1 0 48 24 24 0 1 1 0-48zm273.7-96.5l47.9-83c.8-1.1 1.3-2.4 1.5-3.8s.2-2.7-.1-4.1-.9-2.6-1.7-3.7-1.8-2-3-2.7-2.5-1.1-3.9-1.3-2.7 0-4 .4-2.5 1.1-3.6 1.9-1.9 2-2.5 3.2l-48.5 84.1c-38.8-17.4-80.8-26.4-123.3-26.4s-84.5 9-123.3 26.4L116.2 64.4c-.6-1.2-1.5-2.3-2.5-3.2s-2.3-1.5-3.6-1.9-2.7-.5-4-.4-2.7 .6-3.9 1.3-2.2 1.6-3 2.7-1.4 2.4-1.7 3.7-.3 2.7-.1 4.1 .8 2.6 1.5 3.8l47.9 83C64.5 202.2 8.2 285.5 0 384l576 0c-8.2-98.5-64.5-181.8-146.9-226.6z",
            }
        }
    }
}
