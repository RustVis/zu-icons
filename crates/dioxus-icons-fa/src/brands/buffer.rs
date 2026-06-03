// Copyright (c) 2026 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by Apache-2.0 License
// that can be found in the LICENSE file.

// Auto Generated! DO NOT EDIT!

use dioxus::prelude::*;

#[derive(Debug, Clone, PartialEq, Props)]
pub struct BufferProps {
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

    #[props(default = Some("0 0 448 512"))]
    pub view_box: Option<&'static str>,

    #[props(default = None)]
    pub xmlns: Option<&'static str>,
}

pub fn Buffer(props: BufferProps) -> Element {
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
        d: "M428.2 380.7L231.7 478.5c-2.3 1-4.8 1.5-7.3 1.5s-5-.5-7.3-1.5L20.5 380.7c-4-2-4-5.3 0-7.3L67.6 350c2.3-1 4.8-1.5 7.3-1.5s5 .5 7.3 1.5l134.8 67c2.3 1 4.8 1.5 7.3 1.5s5-.5 7.3-1.5l134.8-67c2.3-1 4.8-1.5 7.3-1.5s5 .5 7.3 1.5l47.1 23.4c4 2 4 5.2 0 7.2zm0-136.5l-47.1-23.4c-2.3-1-4.8-1.5-7.3-1.5s-5 .5-7.3 1.5L231.7 287.8c-2.3 1-4.8 1.5-7.3 1.5s-5-.5-7.3-1.5L82.3 220.7c-2.3-1-4.8-1.5-7.3-1.5s-5 .5-7.3 1.5L20.5 244.1c-4 2-4 5.3 0 7.3l196.5 97.8c2.3 1 4.8 1.5 7.3 1.5s5-.5 7.3-1.5l196.5-97.8c4-2 4-5.3 0-7.3zM20.5 130.4L217 220.7c4.7 1.9 10 1.9 14.7 0l196.5-90.3c4-1.9 4-4.9 0-6.7L231.7 33.4c-4.7-1.9-10-1.9-14.7 0L20.5 123.7c-4 1.8-4 4.9 0 6.7z",
            }
        }
    }
}
