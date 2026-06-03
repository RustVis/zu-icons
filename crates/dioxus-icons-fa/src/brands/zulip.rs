// Copyright (c) 2026 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by Apache-2.0 License
// that can be found in the LICENSE file.

// Auto Generated! DO NOT EDIT!

use dioxus::prelude::*;

#[derive(Debug, Clone, PartialEq, Props)]
pub struct ZulipProps {
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

pub fn Zulip(props: ZulipProps) -> Element {
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
        d: "M424 99c0 22.6-10.1 42.6-25.5 54.8L249.3 287.7c-2.8 2.4-6.4-1.5-4.4-4.7l54.7-110.1c1.5-3.1-.5-6.9-3.6-6.9L83.8 166C50.9 166 24 135.9 24 99 24 62.1 50.9 32 83.8 32l280.4 0C397.1 32 424 62.1 424 99zM83.8 480l280.4 0c32.9 0 59.8-30.2 59.8-67s-26.9-67-59.8-67l-212.3 0c-3.1 0-5.1-3.8-3.6-6.9L203.1 229c2-3.2-1.6-7.1-4.4-4.7L49.5 358.2C34.1 370.4 24 390.4 24 413 24 449.8 50.9 480 83.8 480z",
            }
        }
    }
}
