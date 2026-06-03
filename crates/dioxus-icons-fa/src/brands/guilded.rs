// Copyright (c) 2026 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by Apache-2.0 License
// that can be found in the LICENSE file.

// Auto Generated! DO NOT EDIT!

use dioxus::prelude::*;

#[derive(Debug, Clone, PartialEq, Props)]
pub struct GuildedProps {
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

pub fn Guilded(props: GuildedProps) -> Element {
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
        d: "M443.9 64L5 64c0 103.3 22.2 180.1 43.4 222.4 64.1 127.8 176 161.6 177.3 161.6 55.7-20.5 104.5-56.3 140.6-103.5 25.9-33.9 53.1-87.2 65.9-145.8l-259.9 0c4.1 36.4 22.2 67.9 45.1 86.9l88.6 0c-17 28.2-48.2 54.4-80.5 69.5-31.2-13.3-69.1-46.5-96.5-98.4-26.7-53.8-27.1-105.9-27.1-105.9l336.1 0c4-28.8 5.9-57.9 5.9-86.9z",
            }
        }
    }
}
