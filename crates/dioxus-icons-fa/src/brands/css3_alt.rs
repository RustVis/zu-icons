// Copyright (c) 2026 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by Apache-2.0 License
// that can be found in the LICENSE file.

// Auto Generated! DO NOT EDIT!

use dioxus::prelude::*;

#[derive(Debug, Clone, PartialEq, Props)]
pub struct Css3AltProps {
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

    #[props(default = Some("0 0 384 512"))]
    pub view_box: Option<&'static str>,

    #[props(default = None)]
    pub xmlns: Option<&'static str>,
}

pub fn Css3Alt(props: Css3AltProps) -> Element {
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
        d: "M0 32L34.9 427.8 192 480 349.1 427.8 384 32 0 32zm313.1 80l-4.8 47.3-115.3 49.3-.3 .1 111.5 0-12.8 146.6-98.2 28.7-98.8-29.2-6.4-73.9 48.9 0 3.2 38.3 52.6 13.3 54.7-15.4 3.7-61.6-166.3-.5 0-.1-.2 .1-3.6-46.3 112.1-46.7 6.5-2.7-122.9 0-5.8-47.3 242.2 0z",
            }
        }
    }
}
