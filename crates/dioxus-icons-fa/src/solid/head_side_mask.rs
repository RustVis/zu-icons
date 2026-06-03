// Copyright (c) 2026 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by Apache-2.0 License
// that can be found in the LICENSE file.

// Auto Generated! DO NOT EDIT!

use dioxus::prelude::*;

#[derive(Debug, Clone, PartialEq, Props)]
pub struct HeadSideMaskProps {
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

pub fn HeadSideMask(props: HeadSideMaskProps) -> Element {
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
        d: "M445.9 193c-.1-.4-.2-.9-.2-1.3-15.7-108.4-108.9-191.7-221.7-191.7-85 0-159 47.4-196.9 117.2L283.3 288 456 288 442.9 336 336 336c-8.8 0-16 7.2-16 16s7.2 16 16 16l98.2 0-8.7 32-89.5 0c-8.8 0-16 7.2-16 16s7.2 16 16 16l80.6 0c-6.7 19.1-24.8 32-45.3 32L288 464c-17.7 0-32-14.3-32-32l0-104.5-247.5-165C3 182.1 0 202.7 0 224 0 278 19.1 327.5 50.9 366.2 58.9 375.9 64 387.8 64 400.4L64 464c0 26.5 21.5 48 48 48l127.3 0 .7 0 131.3 0c44.9 0 83.8-31.1 93.6-74.9l33.9-150.3c3.3-14.4-.3-29.5-9.7-41L445.9 193zM288 192a32 32 0 1 1 64 0 32 32 0 1 1 -64 0z",
            }
        }
    }
}
