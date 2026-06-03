// Copyright (c) 2026 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by Apache-2.0 License
// that can be found in the LICENSE file.

// Auto Generated! DO NOT EDIT!

use dioxus::prelude::*;

#[derive(Debug, Clone, PartialEq, Props)]
pub struct SimplybuiltProps {
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

pub fn Simplybuilt(props: SimplybuiltProps) -> Element {
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
        d: "M481.6 64l-106 0C361.1 64 349 75.8 349 90.3l0 39.6-185.3 0 0-39.6c0-14.5-12-26.3-26.6-26.3l-106 0C16.5 64 4.7 75.8 4.7 90.3l0 331.4c0 14.5 11.8 26.3 26.6 26.3l450.4 0c14.8 0 26.6-11.8 26.6-26.3l0-331.4c-.2-14.5-12-26.3-26.7-26.3zM150.5 222.7a66.5 66.5 0 1 1 -.3 133.1 66.5 66.5 0 1 1 .3-133.1zm211.9 .1a66.5 66.5 0 1 1 .3 132.9 66.5 66.5 0 1 1 -.3-132.9z",
            }
        }
    }
}
