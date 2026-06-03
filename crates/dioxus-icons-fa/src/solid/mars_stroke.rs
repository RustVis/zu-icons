// Copyright (c) 2026 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by Apache-2.0 License
// that can be found in the LICENSE file.

// Auto Generated! DO NOT EDIT!

use dioxus::prelude::*;

#[derive(Debug, Clone, PartialEq, Props)]
pub struct MarsStrokeProps {
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

pub fn MarsStroke(props: MarsStrokeProps) -> Element {
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
        d: "M416-32c-17.7 0-32 14.3-32 32s14.3 32 32 32l50.7 0-58.7 58.7-17.4-17.4c-12.5-12.5-32.8-12.5-45.3 0s-12.5 32.8 0 45.3l17.4 17.4-23 23c-28.4-19.5-62.7-31-99.8-31-97.2 0-176 78.8-176 176s78.8 176 176 176 176-78.8 176-176c0-37-11.4-71.4-31-99.8l23-23 17.4 17.4c12.5 12.5 32.8 12.5 45.3 0s12.5-32.8 0-45.3L453.3 136 512 77.3 512 128c0 17.7 14.3 32 32 32s32-14.3 32-32L576 0c0-17.7-14.3-32-32-32L416-32zM128 304a112 112 0 1 1 224 0 112 112 0 1 1 -224 0z",
            }
        }
    }
}
