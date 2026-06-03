// Copyright (c) 2026 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by Apache-2.0 License
// that can be found in the LICENSE file.

// Auto Generated! DO NOT EDIT!

use dioxus::prelude::*;

#[derive(Debug, Clone, PartialEq, Props)]
pub struct ProceduresProps {
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

    #[props(default = Some("0 0 640 512"))]
    pub view_box: Option<&'static str>,

    #[props(default = None)]
    pub xmlns: Option<&'static str>,
}

pub fn Procedures(props: ProceduresProps) -> Element {
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
        d: "M531.2-22.4L572 32 616 32c13.3 0 24 10.7 24 24s-10.7 24-24 24l-56 0c-7.6 0-14.7-3.6-19.2-9.6l-24.1-32.1-47 99.9c-3.7 7.8-11.3 13.1-19.9 13.7s-16.9-3.4-21.7-10.6L387.2 80 344 80c-13.3 0-24-10.7-24-24s10.7-24 24-24l56 0c8 0 15.5 4 20 10.7l24.4 36.6 45.9-97.5c3.6-7.6 10.9-12.8 19.3-13.7s16.6 2.7 21.6 9.5zM320 160c0-17.7 14.3-32 32-32l9.5 0 26.6 39.9c14.4 21.6 39.3 33.8 65.2 31.9s48.8-17.6 59.8-41.1L527 129.2c45.9 7.2 81 46.9 81 94.8l0 224c0 17.7-14.3 32-32 32s-32-14.3-32-32l0-64-448 0 0 64c0 17.7-14.3 32-32 32s-32-14.3-32-32L32 64c0-17.7 14.3-32 32-32S96 46.3 96 64l0 224 224 0 0-128zM144 192a64 64 0 1 1 128 0 64 64 0 1 1 -128 0z",
            }
        }
    }
}
