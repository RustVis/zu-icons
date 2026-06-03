// Copyright (c) 2026 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by Apache-2.0 License
// that can be found in the LICENSE file.

// Auto Generated! DO NOT EDIT!

use dioxus::prelude::*;

#[derive(Debug, Clone, PartialEq, Props)]
pub struct PersonDressBurstProps {
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

pub fn PersonDressBurst(props: PersonDressBurstProps) -> Element {
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
        d: "M208 40.1c13.3 0 24-10.7 24-24l0-48c0-13.3-10.7-24-24-24s-24 10.7-24 24l0 48c0 13.3 10.7 24 24 24zM8 144.1c0 13.3 10.7 24 24 24l48 0c13.3 0 24-10.7 24-24s-10.7-24-24-24l-48 0c-13.3 0-24 10.7-24 24zm92.5 141.4l33.9-33.9c9.4-9.4 9.4-24.6 0-33.9s-24.6-9.4-33.9 0L66.6 251.6c-9.4 9.4-9.4 24.6 0 33.9s24.6 9.4 33.9 0zM66.6 2.7c-9.4 9.4-9.4 24.6 0 33.9l33.9 33.9c9.4 9.4 24.6 9.4 33.9 0s9.4-24.6 0-33.9L100.5 2.7C91.1-6.7 76-6.7 66.6 2.7zM352 80a56 56 0 1 0 0-112 56 56 0 1 0 0 112zM246.2 384l25.8 0 0 128c0 17.7 14.3 32 32 32s32-14.3 32-32l0-128 32 0 0 128c0 17.7 14.3 32 32 32s32-14.3 32-32l0-128 25.8 0c10.9 0 18.6-10.7 15.2-21.1l-43-129 48.3 65.1c10.5 14.2 30.6 17.2 44.8 6.6s17.2-30.6 6.6-44.8l-70.5-95C434 132 394.3 112 352 112s-82 20-107.2 53.9l-70.5 95c-10.5 14.2-7.6 34.2 6.6 44.8s34.2 7.6 44.8-6.6l48.3-65.1-43 129c-3.5 10.4 4.3 21.1 15.2 21.1z",
            }
        }
    }
}
