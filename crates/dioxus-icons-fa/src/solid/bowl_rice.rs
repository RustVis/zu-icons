// Copyright (c) 2026 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by Apache-2.0 License
// that can be found in the LICENSE file.

// Auto Generated! DO NOT EDIT!

use dioxus::prelude::*;

#[derive(Debug, Clone, PartialEq, Props)]
pub struct BowlRiceProps {
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

pub fn BowlRice(props: BowlRiceProps) -> Element {
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
        d: "M176 40c0-13.3 10.7-24 24-24l16 0c13.3 0 24 10.7 24 24s-10.7 24-24 24l-16 0c-13.3 0-24-10.7-24-24zm24 48l16 0c13.3 0 24 10.7 24 24s-10.7 24-24 24l-16 0c-13.3 0-24-10.7-24-24s10.7-24 24-24zM56 160l16 0c13.3 0 24 10.7 24 24s-10.7 24-24 24l-16 0c-13.3 0-24-10.7-24-24s10.7-24 24-24zM0 283.4C0 268.3 12.3 256 27.4 256l457.1 0c15.1 0 27.4 12.3 27.4 27.4 0 70.5-44.4 130.7-106.7 154.1L403.5 452c-2 16-15.6 28-31.8 28l-231.5 0c-16.1 0-29.8-12-31.8-28l-1.8-14.4C44.4 414.1 0 353.9 0 283.4zM224 184c0-13.3 10.7-24 24-24l16 0c13.3 0 24 10.7 24 24s-10.7 24-24 24l-16 0c-13.3 0-24-10.7-24-24zm-96 0c0-13.3 10.7-24 24-24l16 0c13.3 0 24 10.7 24 24s-10.7 24-24 24l-16 0c-13.3 0-24-10.7-24-24zM104 88l16 0c13.3 0 24 10.7 24 24s-10.7 24-24 24l-16 0c-13.3 0-24-10.7-24-24s10.7-24 24-24zm216 96c0-13.3 10.7-24 24-24l16 0c13.3 0 24 10.7 24 24s-10.7 24-24 24l-16 0c-13.3 0-24-10.7-24-24zM296 88l16 0c13.3 0 24 10.7 24 24s-10.7 24-24 24l-16 0c-13.3 0-24-10.7-24-24s10.7-24 24-24zm120 96c0-13.3 10.7-24 24-24l16 0c13.3 0 24 10.7 24 24s-10.7 24-24 24l-16 0c-13.3 0-24-10.7-24-24zM392 88l16 0c13.3 0 24 10.7 24 24s-10.7 24-24 24l-16 0c-13.3 0-24-10.7-24-24s10.7-24 24-24zM296 16l16 0c13.3 0 24 10.7 24 24s-10.7 24-24 24l-16 0c-13.3 0-24-10.7-24-24s10.7-24 24-24z",
            }
        }
    }
}
