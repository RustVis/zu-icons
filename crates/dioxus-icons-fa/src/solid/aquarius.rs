// Copyright (c) 2026 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by Apache-2.0 License
// that can be found in the LICENSE file.

// Auto Generated! DO NOT EDIT!

use dioxus::prelude::*;

#[derive(Debug, Clone, PartialEq, Props)]
pub struct AquariusProps {
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

    #[props(default = Some("0 0 576 512"))]
    pub view_box: Option<&'static str>,

    #[props(default = None)]
    pub xmlns: Option<&'static str>,
}

pub fn Aquarius(props: AquariusProps) -> Element {
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
        d: "M401.7 291.4c9-4.5 19.6-4.5 28.6 0l128 64c15.8 7.9 22.2 27.1 14.3 42.9s-27.1 22.2-42.9 14.3L416 355.8 302.3 412.6c-9 4.5-19.6 4.5-28.6 0L160 355.8 46.3 412.6c-15.8 7.9-35 1.5-42.9-14.3s-1.5-35 14.3-42.9l128-64c9-4.5 19.6-4.5 28.6 0L288 348.2 401.7 291.4zm3.4-193.5c8.2-3 17.3-2.5 25.2 1.5l128 64c15.8 7.9 22.2 27.1 14.3 42.9s-27.1 22.2-42.9 14.3L416 163.8 302.3 220.6c-9 4.5-19.6 4.5-28.6 0L160 163.8 46.3 220.6c-15.8 7.9-35 1.5-42.9-14.3s-1.5-35 14.3-42.9l128-64 3.4-1.5c8.2-3 17.3-2.5 25.2 1.5l113.7 56.8 113.7-56.8 3.4-1.5z",
            }
        }
    }
}
