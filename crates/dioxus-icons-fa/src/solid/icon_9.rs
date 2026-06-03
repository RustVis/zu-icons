// Copyright (c) 2026 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by Apache-2.0 License
// that can be found in the LICENSE file.

// Auto Generated! DO NOT EDIT!

use dioxus::prelude::*;

#[derive(Debug, Clone, PartialEq, Props)]
pub struct Icon9Props {
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

pub fn Icon9(props: Icon9Props) -> Element {
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
        d: "M208 320c29.6 0 57.1-8.9 80-24.2l0 32.2c0 48.6-39.4 88-88 88L96 416c-17.7 0-32 14.3-32 32s14.3 32 32 32l104 0c83.9 0 152-68.1 152-152l0-152.1C351.9 96.4 287.5 32 208 32l-48 0C80.5 32 16 96.5 16 176S80.5 320 160 320l48 0zm80-144c0 44.2-35.8 80-80 80l-48 0c-44.2 0-80-35.8-80-80s35.8-80 80-80l48 0c44.2 0 80 35.8 80 80z",
            }
        }
    }
}
