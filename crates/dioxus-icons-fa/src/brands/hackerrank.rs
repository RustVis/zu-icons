// Copyright (c) 2026 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by Apache-2.0 License
// that can be found in the LICENSE file.

// Auto Generated! DO NOT EDIT!

use dioxus::prelude::*;

#[derive(Debug, Clone, PartialEq, Props)]
pub struct HackerrankProps {
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

pub fn Hackerrank(props: HackerrankProps) -> Element {
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
        d: "M477.9 128C463.4 103 285.5 0 256.5 0s-206.9 102.8-221.3 128-14.5 230.8 0 256 192.4 128 221.3 128 206.8-102.9 221.3-128 14.5-231 0-256zM316.5 414.2c-4 0-40.9-35.8-38-38.7 .9-.9 6.3-1.5 17.5-1.8 0-26.2 .6-68.6 .9-86.3 0-2-.4-3.4-.4-5.8l-79.9 0c0 7.1-.5 36.2 1.4 72.9 .2 4.5-1.6 6-5.7 5.9-10.1 0-20.3-.1-30.4-.1-4.1 0-5.9-1.5-5.7-6.1 .9-33.4 3-84-.2-212.7l0-3.2c-9.7-.4-16.4-1-17.3-1.8-2.9-2.9 34.5-38.7 38.5-38.7s41.2 35.8 38.3 38.7c-.9 .9-7.9 1.5-16.8 1.8l0 3.2c-2.4 25.8-2 79.6-2.6 105.4l80.3 0c0-4.6 .4-34.7-1.2-83.6-.1-3.4 1-5.2 4.2-5.2 11.1-.1 22.2-.1 33.2-.1 3.5 0 4.6 1.7 4.5 5.4-3.7 191.3-.7 177.9-.7 210.3 8.9 .4 16.8 1 17.7 1.8 2.9 2.9-33.6 38.7-37.6 38.7l0 0z",
            }
        }
    }
}
