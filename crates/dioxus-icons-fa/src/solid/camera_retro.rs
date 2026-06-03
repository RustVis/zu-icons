// Copyright (c) 2026 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by Apache-2.0 License
// that can be found in the LICENSE file.

// Auto Generated! DO NOT EDIT!

use dioxus::prelude::*;

#[derive(Debug, Clone, PartialEq, Props)]
pub struct CameraRetroProps {
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

pub fn CameraRetro(props: CameraRetroProps) -> Element {
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
        d: "M0 416l0-208 136.2 0c13.5-20.2 32-36.8 53.7-48L0 160 0 125.7c0-35.3 28.7-64 64-64l.1 0C65.3 45.1 79.1 32 96 32l32 0c16.9 0 30.7 13.1 31.9 29.7l32.1 0 51.2-23.8c8.4-3.9 17.6-6 26.9-6L448 32c35.3 0 64 28.7 64 64l0 64-190 0c21.7 11.2 40.2 27.8 53.7 48l136.2 0 0 208c0 35.3-28.7 64-64 64L64 480c-35.3 0-64-28.7-64-64zM256 192a96.1 96.1 0 1 0 0 192.1 96.1 96.1 0 1 0 0-192.1z",
            }
        }
    }
}
