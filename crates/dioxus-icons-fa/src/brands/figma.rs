// Copyright (c) 2026 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by Apache-2.0 License
// that can be found in the LICENSE file.

// Auto Generated! DO NOT EDIT!

use dioxus::prelude::*;

#[derive(Debug, Clone, PartialEq, Props)]
pub struct FigmaProps {
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

    #[props(default = Some("0 0 384 512"))]
    pub view_box: Option<&'static str>,

    #[props(default = None)]
    pub xmlns: Option<&'static str>,
}

pub fn Figma(props: FigmaProps) -> Element {
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
        d: "M14 95.8C14 42.9 56.9 0 109.8 0L274.2 0c52.9 0 95.8 42.9 95.8 95.8 0 33.5-17.2 63-43.2 80.1 26 17.1 43.2 46.6 43.2 80.1 0 52.9-42.9 95.8-95.8 95.8l-2.1 0c-24.8 0-47.4-9.4-64.4-24.9l0 88.3c0 53.6-44 96.8-97.4 96.8-52.8 0-96.3-42.8-96.3-95.8 0-33.5 17.2-63 43.2-80.1-26-17.1-43.2-46.6-43.2-80.1s17.2-63 43.2-80.1C31.2 158.8 14 129.3 14 95.8zm162.3 95.8l-66.5 0c-35.6 0-64.4 28.8-64.4 64.4 0 35.4 28.6 64.2 64 64.4l66.9 0 0-128.8zM207.7 256c0 35.6 28.8 64.4 64.4 64.4l2.1 0c35.6 0 64.4-28.8 64.4-64.4s-28.8-64.4-64.4-64.4l-2.1 0c-35.6 0-64.4 28.8-64.4 64.4zm-97.9 95.8l-.4 0c-35.4 .2-64 29-64 64.4s29.2 64.4 64.9 64.4c36.3 0 66-29.4 66-65.5l0-63.4-66.5 0zm0-320.4c-35.6 0-64.4 28.8-64.4 64.4s28.8 64.4 64.4 64.4l66.5 0 0-128.8-66.5 0zm97.9 128.8l66.5 0c35.6 0 64.4-28.8 64.4-64.4s-28.8-64.4-64.4-64.4l-66.5 0 0 128.8z",
            }
        }
    }
}
