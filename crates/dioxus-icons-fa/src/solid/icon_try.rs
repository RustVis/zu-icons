// Copyright (c) 2026 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by Apache-2.0 License
// that can be found in the LICENSE file.

// Auto Generated! DO NOT EDIT!

use dioxus::prelude::*;

#[derive(Debug, Clone, PartialEq, Props)]
pub struct IconTryProps {
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

    #[props(default = Some("0 0 448 512"))]
    pub view_box: Option<&'static str>,

    #[props(default = None)]
    pub xmlns: Option<&'static str>,
}

pub fn IconTry(props: IconTryProps) -> Element {
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
        d: "M160 32c17.7 0 32 14.3 32 32l0 43.6 121.4-34.7c12.7-3.6 26 3.7 29.7 16.5s-3.7 26-16.5 29.7l-134.6 38.5 0 46.1 121.4-34.7c12.7-3.6 26 3.7 29.7 16.5s-3.7 26-16.5 29.7l-134.6 38.5 0 162.5 72 0c53 0 96-43 96-96 0-17.7 14.3-32 32-32s32 14.3 32 32c0 88.4-71.6 160-160 160l-104 0c-17.7 0-32-14.3-32-32l0-176.2-25.4 7.3c-12.7 3.6-26-3.7-29.7-16.5s3.7-26 16.5-29.7l38.6-11 0-46.1-25.4 7.3c-12.7 3.6-26-3.7-29.7-16.5s3.7-26 16.5-29.7l38.6-11 0-61.9c0-17.7 14.3-32 32-32z",
            }
        }
    }
}
