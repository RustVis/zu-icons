// Copyright (c) 2026 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by Apache-2.0 License
// that can be found in the LICENSE file.

// Auto Generated! DO NOT EDIT!

use dioxus::prelude::*;

#[derive(Debug, Clone, PartialEq, Props)]
pub struct TailwindCssProps {
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

pub fn TailwindCss(props: TailwindCssProps) -> Element {
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
        d: "M320 64c-85.3 0-138.7 42.7-160 128 32-42.7 69.3-58.7 112-48 24.3 6.1 41.7 23.8 61 43.3 31.4 31.8 67.7 68.7 147 68.7 85.3 0 138.7-42.7 160-128-32 42.7-69.3 58.7-112 48-24.3-6.1-41.7-23.8-61-43.3-31.4-31.8-67.7-68.7-147-68.7zM160 256c-85.3 0-138.7 42.7-160 128 32-42.7 69.3-58.7 112-48 24.3 6.1 41.7 23.8 61 43.3 31.4 31.8 67.7 68.7 147 68.7 85.3 0 138.7-42.7 160-128-32 42.7-69.3 58.7-112 48-24.3-6.1-41.7-23.8-61-43.3-31.4-31.8-67.7-68.7-147-68.7z",
            }
        }
    }
}
