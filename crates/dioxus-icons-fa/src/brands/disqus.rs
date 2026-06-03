// Copyright (c) 2026 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by Apache-2.0 License
// that can be found in the LICENSE file.

// Auto Generated! DO NOT EDIT!

use dioxus::prelude::*;

#[derive(Debug, Clone, PartialEq, Props)]
pub struct DisqusProps {
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

pub fn Disqus(props: DisqusProps) -> Element {
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
        d: "M290.2 512c-62.6 0-119.9-22.9-164.2-60.8L17 466.1 59.1 362.2c-14.7-32.4-22.9-68.3-22.9-106.2 0-141.4 113.7-256 254-256 140.3 0 254 114.6 254 256 0 141.4-113.7 256-254 256zM428.9 255.3l0-.7C428.9 180.7 376.8 128 287 128l-97 0 0 256 95.6 0c90.5 0 143.4-54.9 143.4-128.7zM288 321.1l-28.4 0 0-130.2 28.4 0c41.7 0 69.3 23.8 69.3 64.7l0 .7c0 41.3-27.7 64.7-69.3 64.7z",
            }
        }
    }
}
