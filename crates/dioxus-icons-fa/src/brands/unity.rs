// Copyright (c) 2026 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by Apache-2.0 License
// that can be found in the LICENSE file.

// Auto Generated! DO NOT EDIT!

use dioxus::prelude::*;

#[derive(Debug, Clone, PartialEq, Props)]
pub struct UnityProps {
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

pub fn Unity(props: UnityProps) -> Element {
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
        d: "M243.6 91.6l80.1 46.8c2.9 1.6 3 6.2 0 7.8l-95.2 55.6c-2.9 1.7-6.3 1.6-9 0l-95.2-55.6c-2.9-1.6-3-6.3 0-7.8l80.1-46.8 0-91.6-204.4 119.4 0 238.8 78.4-45.8 0-93.6c-.1-3.3 3.8-5.7 6.7-3.9l95.2 55.6c2.9 1.7 4.5 4.7 4.5 7.8l0 111.2c.1 3.3-3.8 5.7-6.7 3.9L98 346.8 19.6 392.6 224 512 428.4 392.6 350 346.8 269.9 393.6c-2.8 1.7-6.8-.5-6.7-3.9l0-111.2c0-3.3 1.8-6.3 4.5-7.8L362.9 215c2.8-1.7 6.8 .5 6.7 3.9l0 93.6 78.4 45.8 0-238.8-204.4-119.4 0 91.6z",
            }
        }
    }
}
