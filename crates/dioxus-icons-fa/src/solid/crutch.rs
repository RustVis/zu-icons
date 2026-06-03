// Copyright (c) 2026 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by Apache-2.0 License
// that can be found in the LICENSE file.

// Auto Generated! DO NOT EDIT!

use dioxus::prelude::*;

#[derive(Debug, Clone, PartialEq, Props)]
pub struct CrutchProps {
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

pub fn Crutch(props: CrutchProps) -> Element {
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
        d: "M297.4 9.4c12.5-12.5 32.8-12.5 45.3 0l160 160c12.5 12.5 12.5 32.8 0 45.3s-32.8 12.5-45.3 0l-1.4-1.4-158.6 158.6c-18 18-42.4 28.1-67.9 28.1l-59 0c-8.5 0-16.6 3.4-22.6 9.4-61.7 61.7-92.8 92.8-93.3 93.3-12.5 12.5-32.8 12.5-45.3 0s-12.5-32.8 0-45.3l16-16 0 0 77.3-77.3c6-6 9.4-14.1 9.4-22.6l0-59c0-25.5 10.1-49.9 28.1-67.9L298.7 56 297.4 54.6c-12.5-12.5-12.5-32.8 0-45.3zM344 101.3L261.3 184 328 250.7 410.7 168 344 101.3zm-128 128l-30.6 30.6c-6 6-9.4 14.1-9.4 22.6l0 53.5 53.5 0c8.5 0 16.6-3.4 22.6-9.4L282.7 296 216 229.3z",
            }
        }
    }
}
