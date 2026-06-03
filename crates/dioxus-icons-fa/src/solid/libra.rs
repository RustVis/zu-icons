// Copyright (c) 2026 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by Apache-2.0 License
// that can be found in the LICENSE file.

// Auto Generated! DO NOT EDIT!

use dioxus::prelude::*;

#[derive(Debug, Clone, PartialEq, Props)]
pub struct LibraProps {
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

pub fn Libra(props: LibraProps) -> Element {
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
        d: "M480 384c17.7 0 32 14.3 32 32s-14.3 32-32 32L32 448c-17.7 0-32-14.3-32-32s14.3-32 32-32l448 0zM256 32c97.2 0 176 78.8 176 176 0 16.6-2.4 32.7-6.7 48l54.7 0c17.7 0 32 14.3 32 32s-14.3 32-32 32l-104.2 0c-11.8 0-22.7-6.5-28.2-16.9s-4.9-23.1 1.6-32.9c11.9-17.8 18.8-39.1 18.8-62.2 0-61.9-50.1-112-112-112S144 146.1 144 208c0 23.1 6.9 44.4 18.8 62.2 6.6 9.8 7.2 22.5 1.6 32.9S148.1 320 136.2 320L32 320c-17.7 0-32-14.3-32-32s14.3-32 32-32l54.8 0c-4.3-15.3-6.8-31.4-6.8-48 0-97.2 78.8-176 176-176z",
            }
        }
    }
}
