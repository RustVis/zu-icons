// Copyright (c) 2026 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by Apache-2.0 License
// that can be found in the LICENSE file.

// Auto Generated! DO NOT EDIT!

use dioxus::prelude::*;

#[derive(Debug, Clone, PartialEq, Props)]
pub struct ThreemaProps {
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

pub fn Threema(props: ThreemaProps) -> Element {
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
        d: "M87.4 445.1c18.5 0 33.5 15 33.5 33.4S105.9 512 87.4 512 54 497 54 478.5 69 445.1 87.4 445.1zm109.2 0c18.5 0 33.5 15 33.5 33.4s-15 33.5-33.5 33.5-33.4-15-33.4-33.5 15-33.4 33.4-33.4zm109.3 0c18.5 0 33.5 15 33.5 33.4s-15 33.5-33.5 33.5-33.5-15-33.5-33.5 15-33.4 33.5-33.4zM192.3 .3c109.8 0 199.2 89.3 199.2 199.1S302.2 398.5 192.4 398.5c-40.5 0-78.1-12.1-109.6-32.9l-76.5 24.5 24.8-74C7.3 283.4-6.8 243-6.8 199.4-6.8 89.7 82.5 .3 192.3 .3zm.1 93.7c-31.6 0-57.3 25.8-57.3 57.3l0 26.7-1.8 0c-10 0-18 8-18 18l0 72.3c0 10 8 18 18 18l118.1 0c10 0 18-8 18-18l.1 0 0-72.3c0-10-8.1-18-18-18l-1.8 0 0-26.7c0-31.6-25.8-57.3-57.3-57.3zm-.1 23.5c18.6 0 33.8 15.1 33.8 33.8l0 26.7-67.6 0 0-26.7c0-18.6 15.1-33.8 33.8-33.8z",
            }
        }
    }
}
