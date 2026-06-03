// Copyright (c) 2026 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by Apache-2.0 License
// that can be found in the LICENSE file.

// Auto Generated! DO NOT EDIT!

use dioxus::prelude::*;

#[derive(Debug, Clone, PartialEq, Props)]
pub struct Icon3Props {
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

    #[props(default = Some("0 0 320 512"))]
    pub view_box: Option<&'static str>,

    #[props(default = None)]
    pub xmlns: Option<&'static str>,
}

pub fn Icon3(props: Icon3Props) -> Element {
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
        d: "M80 288c-17.7 0-32-14.3-32-32s14.3-32 32-32l112 0c35.3 0 64-28.7 64-64s-28.7-64-64-64L32 96C14.3 96 0 81.7 0 64S14.3 32 32 32l160 0c70.7 0 128 57.3 128 128 0 38.2-16.8 72.5-43.3 96 26.6 23.5 43.3 57.8 43.3 96 0 70.7-57.3 128-128 128L32 480c-17.7 0-32-14.3-32-32s14.3-32 32-32l160 0c35.3 0 64-28.7 64-64s-28.7-64-64-64L80 288z",
            }
        }
    }
}
