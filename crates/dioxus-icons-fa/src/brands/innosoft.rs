// Copyright (c) 2026 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by Apache-2.0 License
// that can be found in the LICENSE file.

// Auto Generated! DO NOT EDIT!

use dioxus::prelude::*;

#[derive(Debug, Clone, PartialEq, Props)]
pub struct InnosoftProps {
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

pub fn Innosoft(props: InnosoftProps) -> Element {
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
        d: "M320 96l0 320c21 0 41.8-4.1 61.2-12.2s37-19.8 51.9-34.7 26.6-32.5 34.7-51.9 12.2-40.2 12.2-61.2-4.1-41.8-12.2-61.2-19.8-37.1-34.7-51.9-32.5-26.6-51.9-34.7-40.2-12.2-61.2-12.2zM0 256L160 416 320 256 160 96 0 256zm480 0c0 21 4.1 41.8 12.2 61.2s19.8 37 34.7 51.9 32.5 26.6 51.9 34.7 40.2 12.2 61.2 12.2l0-320c-42.4 0-83.1 16.9-113.1 46.9S480 213.6 480 256z",
            }
        }
    }
}
