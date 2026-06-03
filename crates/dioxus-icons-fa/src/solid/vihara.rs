// Copyright (c) 2026 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by Apache-2.0 License
// that can be found in the LICENSE file.

// Auto Generated! DO NOT EDIT!

use dioxus::prelude::*;

#[derive(Debug, Clone, PartialEq, Props)]
pub struct ViharaProps {
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

pub fn Vihara(props: ViharaProps) -> Element {
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
        d: "M273.6-3.2c8.5-6.4 20.3-6.4 28.8 0L443.2 102.4c8.3 6.2 18.4 9.6 28.8 9.6 13.3 0 24 10.7 24 24s-10.7 24-24 24l-24 0 0 48 58.7 64 13.3 0c13.3 0 24 10.7 24 24s-10.7 24-24 24l-8 0 0 48 24.4 32 15.6 0c13.3 0 24 10.7 24 24s-10.7 24-24 24l-40 0 0 32c0 17.7-14.3 32-32 32s-32-14.3-32-32l0-32-128 0 0 32c0 17.7-14.3 32-32 32s-32-14.3-32-32l0-32-128 0 0 32c0 17.7-14.3 32-32 32s-32-14.3-32-32l0-32-40 0c-13.3 0-24-10.7-24-24s10.7-24 24-24l22.1 0 17.9-32 0-48-8 0c-13.3 0-24-10.7-24-24s10.7-24 24-24l13.3 0 58.7-64 0-48-24 0c-13.3 0-24-10.7-24-24s10.7-24 24-24c10.4 0 20.5-3.4 28.8-9.6L273.6-3.2zM128 368l320 0 0-48-320 0 0 48zM384 160l-192 0 0 48 192 0 0-48z",
            }
        }
    }
}
