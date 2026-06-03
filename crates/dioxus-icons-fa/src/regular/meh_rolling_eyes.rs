// Copyright (c) 2026 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by Apache-2.0 License
// that can be found in the LICENSE file.

// Auto Generated! DO NOT EDIT!

use dioxus::prelude::*;

#[derive(Debug, Clone, PartialEq, Props)]
pub struct MehRollingEyesProps {
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

pub fn MehRollingEyes(props: MehRollingEyesProps) -> Element {
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
        d: "M256 48a208 208 0 1 1 0 416 208 208 0 1 1 0-416zm0 464a256 256 0 1 0 0-512 256 256 0 1 0 0 512zM176 376c0 13.3 10.7 24 24 24l112 0c13.3 0 24-10.7 24-24s-10.7-24-24-24l-112 0c-13.3 0-24 10.7-24 24zM160 264c-22.1 0-40-17.9-40-40 0-9.5 3.3-18.1 8.8-25 3.2 14.3 16 25 31.2 25s28-10.7 31.2-25c5.5 6.8 8.8 15.5 8.8 25 0 22.1-17.9 40-40 40zm0 40a80 80 0 1 0 0-160 80 80 0 1 0 0 160zm192-40c-22.1 0-40-17.9-40-40 0-9.5 3.3-18.1 8.8-25 3.2 14.3 16 25 31.2 25s28-10.7 31.2-25c5.5 6.8 8.8 15.5 8.8 25 0 22.1-17.9 40-40 40zm0 40a80 80 0 1 0 0-160 80 80 0 1 0 0 160z",
            }
        }
    }
}
