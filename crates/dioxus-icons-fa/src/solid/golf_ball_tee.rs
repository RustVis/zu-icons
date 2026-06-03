// Copyright (c) 2026 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by Apache-2.0 License
// that can be found in the LICENSE file.

// Auto Generated! DO NOT EDIT!

use dioxus::prelude::*;

#[derive(Debug, Clone, PartialEq, Props)]
pub struct GolfBallTeeProps {
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

pub fn GolfBallTee(props: GolfBallTeeProps) -> Element {
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
        d: "M298.5 384.1c12.1 1.2 21.5 11.5 21.5 23.9s-9.4 22.6-21.5 23.9l-2.5 .1-80 0 0 88c0 13.3-10.7 24-24 24s-24-10.7-24-24l0-88-80 0c-13.3 0-24-10.7-24-24 0-12.4 9.4-22.6 21.5-23.9l2.5-.1 208 0 2.5 .1zM192 0c106 0 192 86 192 192 0 57.4-25.2 108.8-65.1 144L65.1 336C25.2 300.8 0 249.4 0 192 0 86 86 0 192 0zm32 240c-7.7 0-14.2 5.5-15.7 12.8l-.6 6.4c-1.3 6.3-6.2 11.2-12.5 12.5l-6.4 .6c-7.3 1.5-12.8 8-12.8 15.7 0 8.8 7.2 16 16 16 26.5 0 48-21.5 48-48 0-8.8-7.2-16-16-16zm64-64c-7.7 0-14.2 5.5-15.7 12.8l-.6 6.4c-1.3 6.3-6.2 11.2-12.5 12.5l-6.4 .6c-7.3 1.5-12.8 8-12.8 15.7 0 8.8 7.2 16 16 16 26.5 0 48-21.5 48-48 0-8.8-7.2-16-16-16zm-80-16c-7.7 0-14.2 5.5-15.7 12.8l-.6 6.4c-1.3 6.3-6.2 11.2-12.5 12.5l-6.4 .6c-7.3 1.5-12.8 8-12.8 15.7 0 8.8 7.2 16 16 16 26.5 0 48-21.5 48-48 0-8.8-7.2-16-16-16z",
            }
        }
    }
}
