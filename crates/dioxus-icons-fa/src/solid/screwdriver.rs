// Copyright (c) 2026 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by Apache-2.0 License
// that can be found in the LICENSE file.

// Auto Generated! DO NOT EDIT!

use dioxus::prelude::*;

#[derive(Debug, Clone, PartialEq, Props)]
pub struct ScrewdriverProps {
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

pub fn Screwdriver(props: ScrewdriverProps) -> Element {
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
        d: "M352.1 146.7l0-49.6c0-10.7 5.3-20.7 14.2-26.6L485.2-8.7c6.3-4.2 14.8-3.4 20.2 2l45.4 45.5c5.4 5.4 6.2 13.8 2 20.2L473.6 177.8c-5.9 8.9-15.9 14.2-26.6 14.2l-49.6 0-90.7 90.7c15 33.3 8.9 73.9-18.5 101.3L162.1 510.1c-18.7 18.7-49.1 18.7-67.9 0L34.1 449.9c-18.7-18.7-18.7-49.1 0-67.9L160.1 256c27.4-27.4 67.9-33.6 101.3-18.5l90.7-90.7z",
            }
        }
    }
}
