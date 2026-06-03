// Copyright (c) 2026 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by Apache-2.0 License
// that can be found in the LICENSE file.

// Auto Generated! DO NOT EDIT!

use dioxus::prelude::*;

#[derive(Debug, Clone, PartialEq, Props)]
pub struct FeatherProps {
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

pub fn Feather(props: FeatherProps) -> Element {
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
        d: "M352 0c41 0 80.3 16.3 109.2 45.2l5.5 5.5c29 29 45.3 68.3 45.3 109.2 0 24.1-5.7 47.6-16.2 68.8-1.9 3.7-5.3 6.5-9.3 7.7L374.5 270c-3.9 1.2-6.5 4.7-6.5 8.8 0 5.1 4.1 9.2 9.2 9.2l32.2 0c14.3 0 21.4 17.2 11.3 27.3l-22.4 22.4c-1.9 1.9-4.2 3.2-6.7 4l-81 24.3c-3.9 1.2-6.5 4.7-6.5 8.8 0 5.1 4.1 9.2 9.2 9.2 13.2 0 18.9 15.7 7.8 22.9-41.1 26.6-89.3 41.1-139 41.1l-86 0-48 48c-8.8 8.8-23.2 8.8-32 0s-8.8-23.2 0-32L256 224c8.8-8.8 8.8-23.2 0-32s-23.2-8.8-32 0L79.5 336.5c-5.7 5.7-15.5 1.7-15.5-6.4 0-67.9 27-133 75-181L242.8 45.2C271.7 16.3 311 0 352 0z",
            }
        }
    }
}
