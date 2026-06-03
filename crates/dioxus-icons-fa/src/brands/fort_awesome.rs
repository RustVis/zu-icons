// Copyright (c) 2026 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by Apache-2.0 License
// that can be found in the LICENSE file.

// Auto Generated! DO NOT EDIT!

use dioxus::prelude::*;

#[derive(Debug, Clone, PartialEq, Props)]
pub struct FortAwesomeProps {
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

pub fn FortAwesome(props: FortAwesomeProps) -> Element {
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
        d: "M489.5 287.9l-27.4 0c-2.6 0-4.6 2-4.6 4.6l0 32-36.6 0 0-178.3c0-2.6-2-4.6-4.6-4.6l-27.4 0c-2.6 0-4.6 2-4.6 4.6l0 32-36.6 0 0-32c0-2.6-2-4.6-4.6-4.6l-27.4 0c-2.6 0-4.6 2-4.6 4.6l0 32-36.6 0 0-32c0-6-8-4.6-11.7-4.6l0-38c8.3-2 17.1-3.4 25.7-3.4 10.9 0 20.9 4.3 31.4 4.3 4.6 0 27.7-1.1 27.7-8l0-60c0-2.6-2-4.6-4.6-4.6-5.1 0-15.1 4.3-24 4.3-9.7 0-20.9-4.3-32.6-4.3-8 0-16 1.1-23.7 2.9l0-4.9c5.4-2.6 9.1-8.3 9.1-14.3 0-20.7-31.4-20.8-31.4 0 0 6 3.7 11.7 9.1 14.3l0 111.7c-3.7 0-11.7-1.4-11.7 4.6l0 32-36.6 0 0-32c0-2.6-2-4.6-4.6-4.6l-27.4 0c-2.6 0-4.6 2-4.6 4.6l0 32-36.3 0 0-32c0-2.6-2-4.6-4.6-4.6l-27.4 0c-2.6 0-4.6 2-4.6 4.6l0 178.3-36.6 0 0-32c0-2.6-2-4.6-4.6-4.6l-27.4 0c-2.6 0-4.6 2-4.6 4.6l0 219.5 182.9 0 0-96c0-72.6 109.7-72.6 109.7 0l0 96 182.9 0 0-219.5c.1-2.6-1.9-4.6-4.5-4.6zm-288.1-4.5c0 2.6-2 4.6-4.6 4.6l-27.4 0c-2.6 0-4.6-2-4.6-4.6l0-64c0-2.6 2-4.6 4.6-4.6l27.4 0c2.6 0 4.6 2 4.6 4.6l0 64zm146.4 0c0 2.6-2 4.6-4.6 4.6l-27.4 0c-2.6 0-4.6-2-4.6-4.6l0-64c0-2.6 2-4.6 4.6-4.6l27.4 0c2.6 0 4.6 2 4.6 4.6l0 64z",
            }
        }
    }
}
