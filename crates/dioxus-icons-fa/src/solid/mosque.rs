// Copyright (c) 2026 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by Apache-2.0 License
// that can be found in the LICENSE file.

// Auto Generated! DO NOT EDIT!

use dioxus::prelude::*;

#[derive(Debug, Clone, PartialEq, Props)]
pub struct MosqueProps {
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

pub fn Mosque(props: MosqueProps) -> Element {
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
        d: "M174.8 224l226.4 0c43.5 0 78.8-35.3 78.8-78.8 0-25.5-12.3-49.4-33.1-64.2L297.3-25.4c-5.6-3.9-13-3.9-18.5 0L129.1 81C108.3 95.8 96 119.7 96 145.2 96 188.7 131.3 224 174.8 224zM512 512c35.3 0 64-28.7 64-64l0-224c0-17.7-14.3-32-32-32s-32 14.3-32 32l0 48-448 0 0-48c0-17.7-14.3-32-32-32S0 206.3 0 224L0 448c0 35.3 28.7 64 64 64l448 0zM240 384c0-26.5 21.5-48 48-48s48 21.5 48 48l0 80-96 0 0-80z",
            }
        }
    }
}
