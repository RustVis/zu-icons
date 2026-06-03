// Copyright (c) 2026 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by Apache-2.0 License
// that can be found in the LICENSE file.

// Auto Generated! DO NOT EDIT!

use dioxus::prelude::*;

#[derive(Debug, Clone, PartialEq, Props)]
pub struct TentArrowDownToLineProps {
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

pub fn TentArrowDownToLine(props: TentArrowDownToLineProps) -> Element {
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
        d: "M185 121c9.4-9.4 9.4-24.6 0-33.9s-24.6-9.4-33.9 0l-31 31 0-102.1C120 2.7 109.3-8 96-8S72 2.7 72 16l0 102.1-31-31C31.6 77.7 16.4 77.7 7 87S-2.3 111.6 7 121l72 72c9.4 9.4 24.6 9.4 33.9 0l72-72zM155.8 260.6L132.1 448 32 448c-17.7 0-32 14.3-32 32s14.3 32 32 32l512 0c17.7 0 32-14.3 32-32 0-2.8-.4-5.5-1-8L548.2 260.7c-1.2-9.7-6.9-18.4-15.3-23.4L367.4 137.9c-10.2-6.1-22.9-6.1-33.1 .1L170.9 237.3c-8.3 5.1-13.9 13.7-15.1 23.3zM448 448l-97.1 0 0-149.4c0-5.9 4.7-10.6 10.6-10.6 4 0 7.7 2.3 9.5 5.9L448 448z",
            }
        }
    }
}
