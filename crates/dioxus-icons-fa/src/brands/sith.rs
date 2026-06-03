// Copyright (c) 2026 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by Apache-2.0 License
// that can be found in the LICENSE file.

// Auto Generated! DO NOT EDIT!

use dioxus::prelude::*;

#[derive(Debug, Clone, PartialEq, Props)]
pub struct SithProps {
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

pub fn Sith(props: SithProps) -> Element {
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
        d: "M0 32l69.7 118.8-58.9-11.5 69.8 91c-3 17-3 34.4 0 51.4l-69.8 91 58.9-11.5-69.7 118.8 118.8-69.7-11.5 58.9 91-69.8c17 3 34.5 3 51.5 0l91 69.8-11.5-58.9 118.7 69.7-69.7-118.8 58.9 11.5-69.8-91c3-17 3-34.4 0-51.4l69.8-91-58.9 11.5 69.7-118.8-118.7 69.7 11.5-58.9-91.1 69.9c-8.5-1.5-17.1-2.3-25.7-2.3s-17.2 .8-25.7 2.3L107.2 42.8 118.8 101.7 0 32zM224 380.2a124.2 124.2 0 1 1 0-248.4 124.2 124.2 0 1 1 0 248.4zm0-211.8a87.6 87.6 0 1 0 0 175.1 87.6 87.6 0 1 0 0-175.1z",
            }
        }
    }
}
