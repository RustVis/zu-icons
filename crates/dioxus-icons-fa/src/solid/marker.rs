// Copyright (c) 2026 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by Apache-2.0 License
// that can be found in the LICENSE file.

// Auto Generated! DO NOT EDIT!

use dioxus::prelude::*;

#[derive(Debug, Clone, PartialEq, Props)]
pub struct MarkerProps {
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

pub fn Marker(props: MarkerProps) -> Element {
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
        d: "M408.8 0c-27.4 0-53.6 10.9-73 30.2L318.1 48 305 34.9c-28.1-28.1-73.7-28.1-101.8 0L103 135c-9.4 9.4-9.4 24.6 0 33.9s24.6 9.4 33.9 0L237.1 68.9c9.4-9.4 24.6-9.4 33.9 0L284.1 81.9 184 182.1 329.9 328 481.8 176.2c19.4-19.4 30.2-45.6 30.2-73 0-57-46.2-103.2-103.2-103.2zM102.4 263.7c-49.9 49.9-83.3 114-95.5 183.5L.4 483.8C-1 491.6 1.5 499.4 7 505s13.4 8 21.1 6.7l36.7-6.5c69.5-12.3 133.6-45.6 183.5-95.5L296 361.9 150.1 216 102.4 263.7z",
            }
        }
    }
}
