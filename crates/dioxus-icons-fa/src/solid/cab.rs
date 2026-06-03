// Copyright (c) 2026 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by Apache-2.0 License
// that can be found in the LICENSE file.

// Auto Generated! DO NOT EDIT!

use dioxus::prelude::*;

#[derive(Debug, Clone, PartialEq, Props)]
pub struct CabProps {
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

pub fn Cab(props: CabProps) -> Element {
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
        d: "M192 0c-17.7 0-32 14.3-32 32l0 32-8.9 0c-42 0-79.1 27.3-91.6 67.4l-23 73.5C14.5 219.1 0 243.9 0 272L0 448c0 17.7 14.3 32 32 32l32 0c17.7 0 32-14.3 32-32l0-32 320 0 0 32c0 17.7 14.3 32 32 32l32 0c17.7 0 32-14.3 32-32l0-176c0-28.1-14.5-52.9-36.4-67.1l-23-73.5C440.1 91.3 402.9 64 360.9 64l-8.9 0 0-32c0-17.7-14.3-32-32-32L192 0zM151.1 128l209.9 0c14 0 26.4 9.1 30.5 22.5l13 41.5-296.9 0 13-41.5c4.2-13.4 16.5-22.5 30.5-22.5zM96 272a32 32 0 1 1 0 64 32 32 0 1 1 0-64zm288 32a32 32 0 1 1 64 0 32 32 0 1 1 -64 0z",
            }
        }
    }
}
