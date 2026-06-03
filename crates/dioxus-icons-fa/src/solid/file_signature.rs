// Copyright (c) 2026 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by Apache-2.0 License
// that can be found in the LICENSE file.

// Auto Generated! DO NOT EDIT!

use dioxus::prelude::*;

#[derive(Debug, Clone, PartialEq, Props)]
pub struct FileSignatureProps {
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

    #[props(default = Some("0 0 640 512"))]
    pub view_box: Option<&'static str>,

    #[props(default = None)]
    pub xmlns: Option<&'static str>,
}

pub fn FileSignature(props: FileSignatureProps) -> Element {
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
        d: "M64.1 64c0-35.3 28.7-64 64-64L277.6 0c17 0 33.3 6.7 45.3 18.7L429.3 125.3c12 12 18.7 28.3 18.7 45.3l0 97.5-132 132-42.1 0-16.1-53.6c-4.7-15.7-19.1-26.4-35.5-26.4-11.3 0-21.9 5.1-28.9 13.9L133.3 409c-8.3 10.3-6.6 25.5 3.7 33.7s25.5 6.6 33.7-3.8l47.1-58.8 15.2 50.7c3 10.2 12.4 17.1 23 17.1l31.5 0c-.9 3.1-1.7 6.3-2.3 9.5l-10.9 54.5-146.2 0c-35.3 0-64-28.7-64-64l0-384zm208-5.5l0 93.5c0 13.3 10.7 24 24 24l93.5 0-117.5-117.5zm60.2 408.4c2.5-12.4 8.6-23.8 17.5-32.7l118.9-118.9 80 80-118.9 118.9c-8.9 8.9-20.3 15-32.7 17.5l-59.6 11.9c-.9 .2-1.9 .3-2.9 .3-8 0-14.6-6.5-14.6-14.6 0-1 .1-1.9 .3-2.9l11.9-59.6zm267.8-123l-28.8 28.8-80-80 28.8-28.8c22.1-22.1 57.9-22.1 80 0s22.1 57.9 0 80z",
            }
        }
    }
}
