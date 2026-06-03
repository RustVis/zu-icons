// Copyright (c) 2026 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by Apache-2.0 License
// that can be found in the LICENSE file.

// Auto Generated! DO NOT EDIT!

use dioxus::prelude::*;

#[derive(Debug, Clone, PartialEq, Props)]
pub struct NimblrProps {
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

pub fn Nimblr(props: NimblrProps) -> Element {
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
        d: "M246.6 353.3a27 27 0 1 1 0-54 27 27 0 1 1 0 54zm-79.4-27a27.1 27.1 0 1 1 -54.3 0 27.1 27.1 0 1 1 54.3 0zM191.8 159C157 159 89.4 178.8 59.2 227L14 0 14 335.5C14 433.1 93.6 512 191.8 512S369.5 433 369.5 335.5 290.1 159 191.8 159zm0 308.1c-73.3 0-132.5-58.9-132.5-131.6s59.2-131.6 132.5-131.6 132.5 58.9 132.5 131.5-59.3 131.6-132.5 131.6l0 .1z",
            }
        }
    }
}
